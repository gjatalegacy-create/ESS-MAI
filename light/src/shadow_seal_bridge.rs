// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  SHADOW_SEAL_BRIDGE.RS — PORTA E FUNDIT E LIGHT DREJT SHADOW          ║
// ║  Gjata Legacy™  |  Arkitekt: Bledar Gjata                             ║
// ║                                                                          ║
// ║  Mbart vulën 500 nga EvolveTrace → buss_legacy (i VERBËR) →           ║
// ║  shadow_gj_legacy.lgc, i cili vendos 0/1.                            ║
// ║                                                                          ║
// ║  FILOZOFIA:                                                             ║
// ║    Light NUK vendos. Light llogarit vulën dhe e TRANSPORTON.          ║
// ║    buss_legacy nuk e di ç'është 500 — e kopjon të verbër.            ║
// ║    shadow_gj_legacy është i VETMI që e kupton dhe vendos.            ║
// ║                                                                          ║
// ║  c_kernel ON  → FFI real te kernelët C (buss_legacy + shadow).        ║
// ║  c_kernel OFF → fallback pure-Rust me TË NJËJTËN logjikë.             ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::evolve_trace::EvolveTrace;

// ─────────────────────────────────────────────────────────────────────────────
// KONSTANTET E VULËS — identike me shadow_gj_legacy.c
// ─────────────────────────────────────────────────────────────────────────────

const SGL_SEAL_XOR:       u32 = 0xA5A5;
const SGL_SEAL_MASK:      u32 = 0x0000_FFFF;
const SGL_SEAL_PRIMITIVE: u32 = 500;

// ─────────────────────────────────────────────────────────────────────────────
// VERDIKTI I SHADOW-it
// ─────────────────────────────────────────────────────────────────────────────

/// Vendimi suprem i shadow_gj_legacy.lgc mbi një input.
#[derive(Debug, Clone, PartialEq)]
pub enum ShadowVerdict {
    /// 1 — vula 500 mbijetoi: input mbeti primitiv i fortë.
    PrimitiveKnowledge,
    /// 0 — vula u hoq: input u fragmentua; i verifikueshëm, jo primitiv.
    VerifiableNotPrimitive,
}

impl ShadowVerdict {
    pub fn bit(&self) -> u8 {
        match self {
            Self::PrimitiveKnowledge      => 1,
            Self::VerifiableNotPrimitive  => 0,
        }
    }

    pub fn label(&self) -> &str {
        match self {
            Self::PrimitiveKnowledge =>
                "1:PRIMITIVE_KNOWLEDGE",
            Self::VerifiableNotPrimitive =>
                "0:VERIFIABLE_NOT_PRIMITIVE",
        }
    }

    /// Shpjegimi i plotë siç do ta jepte shadow_gj_legacy.lgc.
    pub fn declaration(&self) -> &str {
        match self {
            Self::PrimitiveKnowledge =>
                "Ky input kaloi nëpër filtrat dhe procesimin e sistemit dhe \
                 mbijetoi. E mbart vulën 500 dhe shpallet 1 si primitive_knowledge \
                 — dije e verifikueshme me potencial për legacy_knowledge.",
            Self::VerifiableNotPrimitive =>
                "Ky input u sfidua nga forca e sistemit dhe filtrat e tij dhe si \
                 primitiv nuk qëndroi i fortë por u nda në disa arsyetime/knowledge/gjendje. \
                 Vendimi është 0: i verifikueshëm për sistemin, por jo si primitive_knowledge.",
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// FFI — buss_legacy (i verbër) + shadow_gj_legacy (autoriteti)
// Vetëm me feature c_kernel.
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "c_kernel")]
mod ffi {
    // BussLegacyMsg — ABI identike me kernel/buss_legacy.h
    pub const BL_LGC_SEALED_LEN: usize = 512;
    pub const BL_MAX_PAYLOAD:    usize = 2048;

    #[repr(C)]
    pub struct BussLegacyMsg {
        pub trace_id:    u64,
        pub lgc_sealed:  [u8; BL_LGC_SEALED_LEN],
        pub flags:       u32,
        pub payload:     [u8; BL_MAX_PAYLOAD],
        pub payload_len: u32,
        pub priority:    u8,
    }

    // LgcRequest / LgcResult — ABI identike me kernel/shadow_gj_legacy.h
    #[repr(C)]
    pub struct LgcRequest {
        pub req_id:    [u8; 48],
        pub caller:    [u8; 64],
        pub sha256:    [u8; 65],
        pub ram_usage: f32,
        pub flags:     u32,
        pub ts:        u64,
    }

    #[repr(C)]
    pub struct LgcResult {
        pub req_id:   [u8; 48],
        pub decision: i32,   // LgcDecision: 0=BLOCK,1=PASS
        pub state:    i32,
        pub sha256:   [u8; 65],
        pub reason:   [u8; 128],
        pub ts:       u64,
    }

    #[link(name = "light_kernel")]
    extern "C" {
        pub fn buss_legacy_init() -> i32;
        pub fn buss_legacy_send(msg: *const BussLegacyMsg) -> i32;
        pub fn buss_legacy_recv(msg_out: *mut BussLegacyMsg) -> i32;

        pub fn lgc_check(req: *const LgcRequest, res: *mut LgcResult) -> i32;
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// LOGJIKA E AUTORITETIT (pure-Rust mirror — identike me shadow_gj_legacy.c)
// Përdoret si fallback kur c_kernel OFF, dhe si referencë e vërtetës.
// ─────────────────────────────────────────────────────────────────────────────

/// (flags & 0xFFFF) ^ 0xA5A5 == 500  →  true
fn shadow_seal_check(flags: u32) -> bool {
    let cleared = flags != 0;
    let value   = (flags & SGL_SEAL_MASK) ^ SGL_SEAL_XOR;
    cleared && value == SGL_SEAL_PRIMITIVE
}

// ─────────────────────────────────────────────────────────────────────────────
// PORTA — dispaton vulën përmes bus-it të verbër te autoriteti
// ─────────────────────────────────────────────────────────────────────────────

pub struct ShadowSealBridge;

impl ShadowSealBridge {
    /// Inicializon bus-in e verbër. FAIL-CLOSED (#8): pa c_kernel, init i vërtetë
    /// kërkon dev_simulation; në prodhim mungesa e bus-it real = false (kritik).
    pub fn init() -> bool {
        #[cfg(feature = "c_kernel")]
        unsafe {
            return ffi::buss_legacy_init() == 0;
        }
        // dev_simulation: lejo init simbolik (testim lokal).
        #[cfg(all(not(feature = "c_kernel"), feature = "dev_simulation"))]
        {
            true
        }
        // PRODHIM: bus legacy real mungon → init dështon (boot e kap si kritike).
        #[cfg(all(not(feature = "c_kernel"), not(feature = "dev_simulation")))]
        {
            false
        }
    }

    /// DISPATCH — merr vulën nga EvolveTrace, e mbart përmes buss_legacy
    /// (i verbër), dhe e dorëzon te shadow_gj_legacy.lgc për vendimin 0/1.
    ///
    /// Hapat (fizikë me c_kernel, logjikë e njëjtë pa të):
    ///   1. flags = evolve.primitive_flags()   (0xA451 ose 0x0000)
    ///   2. buss_legacy_send(msg{flags})        bus s'e di ç'është 500
    ///   3. buss_legacy_recv(msg)               del i paprekur
    ///   4. lgc_check(req{flags}) → decision    shadow vendos
    ///   5. → ShadowVerdict
    pub fn dispatch_seal(
        &self,
        evolve:    &EvolveTrace,
        lgc_sealed: &str,
        ram_usage:  f32,
    ) -> ShadowVerdict {
        let flags = evolve.primitive_flags();

        #[cfg(feature = "c_kernel")]
        {
            return self.dispatch_via_c(evolve.origin_id, flags, lgc_sealed, ram_usage);
        }

        #[cfg(not(feature = "c_kernel"))]
        {
            // Fallback pure-Rust — bus "i verbër" simbolik: vlera kalon e paprekur.
            let _ = (lgc_sealed, ram_usage);
            let carried = flags;                 // bus s'e ndryshon
            match shadow_seal_check(carried) {
                true  => ShadowVerdict::PrimitiveKnowledge,
                false => ShadowVerdict::VerifiableNotPrimitive,
            }
        }
    }

    #[cfg(feature = "c_kernel")]
    fn dispatch_via_c(
        &self,
        trace_id:   u64,
        flags:      u32,
        lgc_sealed: &str,
        ram_usage:  f32,
    ) -> ShadowVerdict {
        use std::time::{SystemTime, UNIX_EPOCH};

        // ── 1+2. Ndërto mesazhin dhe dërgoje te bus-i i verbër ─────────────
        let mut msg = ffi::BussLegacyMsg {
            trace_id,
            lgc_sealed:  [0u8; ffi::BL_LGC_SEALED_LEN],
            flags,
            payload:     [0u8; ffi::BL_MAX_PAYLOAD],
            payload_len: 0,
            priority:    1,
        };
        let lb = lgc_sealed.as_bytes();
        let n  = lb.len().min(ffi::BL_LGC_SEALED_LEN - 1);
        msg.lgc_sealed[..n].copy_from_slice(&lb[..n]);

        unsafe { ffi::buss_legacy_send(&msg); }

        // ── 3. Merre mbrapsht (bus FIFO, i paprekur) ───────────────────────
        let mut recv = ffi::BussLegacyMsg {
            trace_id: 0,
            lgc_sealed:  [0u8; ffi::BL_LGC_SEALED_LEN],
            flags: 0,
            payload:     [0u8; ffi::BL_MAX_PAYLOAD],
            payload_len: 0,
            priority: 0,
        };
        let got = unsafe { ffi::buss_legacy_recv(&mut recv) };
        // Nëse bus bosh (s'duhet ndodhë), përdor flags origjinale.
        let carried_flags = match got == 0 {
            true  => recv.flags,
            false => flags,
        };

        // ── 4. Autoriteti vendos ───────────────────────────────────────────
        let ts = SystemTime::now().duration_since(UNIX_EPOCH)
            .unwrap_or_default().as_secs();

        let req = ffi::LgcRequest {
            req_id:    [0u8; 48],
            caller:    [0u8; 64],
            sha256:    [b'0'; 65],
            ram_usage,
            flags:     carried_flags,
            ts,
        };
        let mut res = ffi::LgcResult {
            req_id: [0u8; 48], decision: 0, state: 0,
            sha256: [0u8; 65], reason: [0u8; 128], ts: 0,
        };

        unsafe { ffi::lgc_check(&req, &mut res); }

        // ── 5. Verdikti ────────────────────────────────────────────────────
        match res.decision == 1 {
            true  => ShadowVerdict::PrimitiveKnowledge,
            false => ShadowVerdict::VerifiableNotPrimitive,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// TESTE
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trace_info::TraceInfo;
    use crate::ess_mai_system::dsl;

    #[test]
    fn primitive_survives_when_coherent() {
        let info = TraceInfo::new(b"input i forte primitiv");
        let mut ev = EvolveTrace::from_trace(&info);
        // Degë koherente (drift i ulët) → vula mbahet
        let _ = ev.branch_scored(dsl::SFX_INPUT, 1.0);
        let _ = ev.branch_scored(dsl::SFX_HISTORY, 0.95);
        assert!(ev.is_primitive_sealed());

        let bridge = ShadowSealBridge;
        let v = bridge.dispatch_seal(&ev, "LGC::test", 0.1);
        assert_eq!(v, ShadowVerdict::PrimitiveKnowledge);
        assert_eq!(v.bit(), 1);
    }

    #[test]
    fn primitive_stripped_when_fragmented() {
        let info = TraceInfo::new(b"input qe fragmentohet");
        let mut ev = EvolveTrace::from_trace(&info);
        // Degë divergjente (drift > 0.15) → vula humbet
        let _ = ev.branch_scored(dsl::SFX_INPUT, 1.0);
        let _ = ev.branch_scored(dsl::SFX_MATHEMATICS, 0.40);
        assert!(!ev.is_primitive_sealed());

        let bridge = ShadowSealBridge;
        let v = bridge.dispatch_seal(&ev, "LGC::test", 0.1);
        assert_eq!(v, ShadowVerdict::VerifiableNotPrimitive);
        assert_eq!(v.bit(), 0);
    }

    #[test]
    fn seal_check_matches_c_kernel() {
        // 0xA451 = (500 & 0xFFFF) ^ 0xA5A5
        assert!(shadow_seal_check(0xA451));
        assert!(!shadow_seal_check(0x0000));
        assert!(!shadow_seal_check(500));    // pa XOR — jo valid
    }
}
