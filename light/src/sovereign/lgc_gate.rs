// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  SOVEREIGN/LGC_GATE.RS — Capability Token Një-Përdorimësh               ║
// ║  GJATA LEGACY™ — Light Platform                                   ║
// ║                                                                          ║
// ║  Adaptim i SOVEREIGN FFI GATE + LgcGate (nga ligji 0-copy).           ║
// ║                                                                          ║
// ║  TEOREMA: C sheh vetëm CapHandle(gen, nonce). CapSlot{AtomicBool}      ║
// ║  jeton i paprekur në heap Rust. validate() bën CAS(true→false) —      ║
// ║  hardware-atomik. Nga kopjet identike vetëm përdorimi i parë kalon; replay refuzohet.       ║
// ║                                                                          ║
// ║  FILOZOFIA: nuk e DETYRON C-në — ELIMINON mundësinë. LAW_3 mbijeton.   ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use std::marker::PhantomData;
use std::sync::atomic::{AtomicBool, Ordering};

// ─────────────────────────────────────────────────────────────────────────────
// LGC TOKEN — !Send !Clone !Copy — kurrë s'del nga funksioni
// ─────────────────────────────────────────────────────────────────────────────

/// Dëshmi që një operacion u autorizua. Prodhohet brenda gate-it,
/// konsumohet, dhe nuk del kurrë jashtë. PhantomData<*const ()> → !Send.
pub struct LgcToken(PhantomData<*const ()>);

impl LgcToken {
    fn new() -> Self {
        LgcToken(PhantomData)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// SOVEREIGN ERROR
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SovErr {
    SealMismatch(u64),
    NonceMismatch { expected: u64, actual: u64 },
    AlreadyConsumed,
    NotFound,
}

// ─────────────────────────────────────────────────────────────────────────────
// LGC GATE — verifikon seal-in para se të hapë token
// ─────────────────────────────────────────────────────────────────────────────

/// Porta që verifikon seal-in e modulit thirrës.
/// Nëse runtime_seal == expected → hap LgcToken; ndryshe → SealMismatch.
pub struct LgcGate {
    expected: u64,
}

impl LgcGate {
    pub const fn new(expected: u64) -> Self {
        Self { expected }
    }

    /// Hap token nëse seal-i përputhet. Branchless delta check.
    pub fn open(&self, runtime_seal: u64) -> Result<LgcToken, SovErr> {
        let delta = self.expected ^ runtime_seal;
        // delta == 0 → seal i saktë. Pa if: përdorim match mbi bool.
        match delta {
            0 => Ok(LgcToken::new()),
            d => Err(SovErr::SealMismatch(d)),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// CAP SLOT — qeliza sovrane me AtomicBool single-use
// ─────────────────────────────────────────────────────────────────────────────

/// Qeliza një-përdorimësh. AtomicBool: CAS(true→false) konsumon.
/// Jeton në heap Rust; C-ja nuk e prek dot.
struct CapSlot {
    live: AtomicBool,
    expected_nonce: u64,
}

impl CapSlot {
    fn new(expected_nonce: u64) -> Self {
        Self {
            live: AtomicBool::new(true),
            expected_nonce,
        }
    }

    /// Konsumon qelizën. CAS(true→false): vetëm hera e parë kalon.
    /// Çdo thirrje tjetër → false (already consumed). Hardware-atomik.
    fn consume(&self) -> bool {
        self.live
            .compare_exchange(true, false, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// CAP HANDLE — e vetmja gjë që del jashtë (numra)
// ─────────────────────────────────────────────────────────────────────────────

/// Çelësi që del jashtë gate-it — vetëm dy numra.
/// C mund ta kopjojë me memcpy, por kopjet ndajnë të njëjtin slot një-përdorimësh:
/// vetëm njëra mund të fitojë konsumimin; replay refuzohet nga CAS-i në heap.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapHandle {
    pub gen:   u64,
    pub nonce: u64,
}

// ─────────────────────────────────────────────────────────────────────────────
// SOVEREIGN GATE — lëshon capabilities, validon një herë
// ─────────────────────────────────────────────────────────────────────────────

/// Porta sovrane: lëshon CapHandle + ruan CapSlot, validon një herë.
pub struct SovereignGate {
    slots: Vec<CapSlot>,
    gen_counter: u64,
}

impl SovereignGate {
    pub fn new() -> Self {
        Self { slots: Vec::new(), gen_counter: 0 }
    }

    /// Lëshon një capability të re. Kthen handle (numra) për thirrësin.
    pub fn issue(&mut self, module_seal: u64) -> CapHandle {
        let gen = self.gen_counter;
        self.gen_counter += 1;
        self.slots.push(CapSlot::new(module_seal));
        CapHandle { gen, nonce: module_seal }
    }

    /// Validon dhe konsumon. Një herë.
    /// Klon i handle-it → CAS(false→false) → AlreadyConsumed.
    pub fn validate(&self, handle: CapHandle) -> Result<LgcToken, SovErr> {
        let idx = usize::try_from(handle.gen).map_err(|_| SovErr::NotFound)?;
        let slot = self.slots.get(idx).ok_or(SovErr::NotFound)?;

        // Nonce-i është pjesë e capability contract. Kontrollohet PARA CAS-it:
        // një handle i manipuluar refuzohet pa djegur capability-n legjitime.
        match slot.expected_nonce == handle.nonce {
            true => {}
            false => return Err(SovErr::NonceMismatch {
                expected: slot.expected_nonce,
                actual: handle.nonce,
            }),
        }

        // CAS atomik — vetëm hera e parë kalon.
        match slot.consume() {
            true => Ok(LgcToken::new()),
            false => Err(SovErr::AlreadyConsumed),
        }
    }
}

impl Default for SovereignGate {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sovereign::laws::SEAL_COORDINATOR;

    #[test]
    fn cap_handle_layout_is_ffi_stable() {
        assert_eq!(
            std::mem::size_of::<CapHandle>(),
            2 * std::mem::size_of::<u64>(),
        );
    }

    #[test]
    fn gate_opens_on_correct_seal() {
        let gate = LgcGate::new(SEAL_COORDINATOR);
        assert!(gate.open(SEAL_COORDINATOR).is_ok());
        assert_eq!(
            gate.open(0xDEAD).err(),
            Some(SovErr::SealMismatch(SEAL_COORDINATOR ^ 0xDEAD)),
        );
    }

    #[test]
    fn capability_single_use() {
        let mut sov = SovereignGate::new();
        let handle = sov.issue(SEAL_COORDINATOR);
        // Hera e parë: OK
        assert!(sov.validate(handle).is_ok());
        // Hera e dytë (klon i handle-it): AlreadyConsumed
        assert_eq!(sov.validate(handle).err(), Some(SovErr::AlreadyConsumed));
    }

    #[test]
    fn tampered_nonce_is_rejected_without_burning_capability() {
        let mut sov = SovereignGate::new();
        let original = sov.issue(SEAL_COORDINATOR);
        let forged = CapHandle {
            gen: original.gen,
            nonce: original.nonce ^ 1,
        };

        assert_eq!(
            sov.validate(forged).err(),
            Some(SovErr::NonceMismatch {
                expected: original.nonce,
                actual: original.nonce ^ 1,
            }),
        );

        // Refuzimi i nonce-it të manipuluar nuk konsumon slot-in legjitim.
        assert!(sov.validate(original).is_ok());
    }

    #[test]
    fn copied_handle_cannot_replay_after_consumption() {
        let mut sov = SovereignGate::new();
        let original = sov.issue(SEAL_COORDINATOR);
        // Kopje byte-identike e capability handle-it (Copy).
        let stolen = original;
        assert_eq!(stolen.gen, original.gen);
        assert_eq!(stolen.nonce, original.nonce);
        // Hera e parë me original: OK (konsumon AtomicBool)
        assert!(sov.validate(original).is_ok());
        // Kopja pas konsumimit → CAS(false→false) → replay i refuzuar.
        assert_eq!(sov.validate(stolen).err(), Some(SovErr::AlreadyConsumed));
    }
}
