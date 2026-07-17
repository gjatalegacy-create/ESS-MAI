// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  SOVEREIGN FFI GATE  —  GJATA LEGACY™        (integruar në Shadow)   ║
// ║  Përgjigje matematikore: "Si e pengon C-ja të klonojë LgcToken?"     ║
// ║                                                                          ║
// ║  PROOF THEOREM:                                                         ║
// ║    Premise 1: C sheh vetëm  CapHandle(gen: u64, nonce: u64)          ║
// ║    Premise 2: CapSlot{AtomicBool} jeton i paprekur ∈ heap Rust       ║
// ║    Premise 3: validate() bën CAS(true→false) — hardware-atomik       ║
// ║                                                                          ║
// ║    C sulmon:  stolen = memcpy(handle)  → gen, nonce identikë         ║
// ║    C dështon: validate(stolen) → CAS(false→false) → Err → -1         ║
// ║    Sepse: C klonoi NUMRAT. Jo AtomicBool-in. ∎                       ║
// ║                                                                          ║
// ║  INTEGRIM (FAZA: gjurmueshmëri): vula atomike = AUTORIZIM shkrimi.    ║
// ║    Token i lindur ⇒ shkrim i ligjshëm ⇒ gjurmë kapaciteti regjistrohet║
// ║    (knowledge_lineage). Sulm klonimi ⇒ shënohet VERIFIED_NEGATIVE.    ║
// ║                                                                          ║
// ║  FILOZOFIA: Nuk e DETYROI C-në. ELIMINOI mundësinë. LAW_3 mbijeton FFI.║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::knowledge_lineage::{ledger, Lineage};
use crate::knowledge_vault::{get_global_vault, KnowledgeVault};
use crate::types::{KnowledgeBand, NegativeContext, SupremeVerdict};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};

// ════════════════════════════════════════════════════════════════════════════
// §0  MATEMATIKA BAZË — FNV-1a (e njëjtë me projektin)
// ════════════════════════════════════════════════════════════════════════════

const fn fnv(s: &[u8]) -> u64 {
    let mut h: u64 = 14695981039346656037;
    let mut i = 0;
    while i < s.len() {
        h ^= s[i] as u64;
        h = h.wrapping_mul(1099511628211);
        i += 1;
    }
    h
}
const fn seal(m: &[u8]) -> u64 { fnv(m) ^ fnv(b"GJATA_LEGACY") }

// Compile-time nonce-ët — të prera, jo të negociueshme
pub const S_SHADOW_WRITE:  u64 = seal(b"SHADOW_WRITE_KERNEL");
pub const S_SHADOW_VERIFY: u64 = seal(b"SHADOW_VERIFICATION_OUTPUT");
pub const S_LAB_TRL1:     u64 = seal(b"LAB_TRL1_FFI");
pub const S_LAB_TRL2:     u64 = seal(b"LAB_TRL2_FFI");

fn now_ns() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_nanos() as u64).unwrap_or(0)
}

// ════════════════════════════════════════════════════════════════════════════
// §1  LGC TOKEN — PATHYESHËM, KURRË nuk kalon FFI
//     PhantomData<*const ()> → auto: !Send !Clone !Copy
// ════════════════════════════════════════════════════════════════════════════

pub struct LgcToken(PhantomData<*const ()>);

// ════════════════════════════════════════════════════════════════════════════
// §2  CAP HANDLE — E VETMJA gjë që C sheh dhe prek
//     repr(C) → C e lexon si dy u64. Copy → C bën memcpy lirisht (KURTHI).
// ════════════════════════════════════════════════════════════════════════════

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CapHandle {
    pub gen:   u64, // ID gjenerimi — identifikon CapSlot në Rust heap
    pub nonce: u64, // FNV seal e modulit — verifikim matematik
}
impl CapHandle { pub const INVALID: Self = Self { gen: 0, nonce: 0 }; }


/// Dëftesa publike e mbylljes së output-it aktual. LgcToken nuk del kurrë nga
/// Shadow; kjo strukturë është prova e transportueshme që token-i u lind dhe u
/// konsumua për seal-in e verifikimit të XY.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationReceipt {
    pub session_id:        String,
    pub parent_i0:         String,
    pub primitive_anchor:  u64,
    pub xy_digest:         u64,
    pub pd_binding_digest: u64,
    pub pd_continuum_activation_digest: u64,
    pub living_trust_digest: u64,
    pub living_trust_sha256: [u8; 32],
    pub y_verdict:         u8,
    pub x_verdict:         u8,
    pub seal:              u64,
    pub generation:        u64,
    pub receipt_id:        String,
}

// ════════════════════════════════════════════════════════════════════════════
// §3  CAP SLOT — ZEMRA e garancisë; KURRË nuk del nga Rust
//     AtomicBool: true→i freskët (1× konsum me CAS), false→i djegur (kurrë rikthim)
// ════════════════════════════════════════════════════════════════════════════

struct CapSlot {
    valid:          AtomicBool, // burned me CAS — JO me store()
    gen:            u64,
    expected_nonce: u64,
}

#[derive(Debug, PartialEq)]
pub enum FfiErr {
    HandleNotFound,
    NonceMismatch,
    AlreadyConsumed, // ← KURTHI I SULMIT C: CAS i rënë në burned slot
}

// ════════════════════════════════════════════════════════════════════════════
// §4  SOVEREIGN GATE — Registry i fshehur: HashMap<gen → Box<CapSlot>>
//     C ka gen (numri). C nuk ka *Box<CapSlot> (adresën e realitetit).
// ════════════════════════════════════════════════════════════════════════════

struct SovereignGate {
    slots:   HashMap<u64, Box<CapSlot>>,
    counter: u64,
}

impl SovereignGate {
    fn new() -> Self { Self { slots: HashMap::new(), counter: 1 } }

    /// Krijon CapSlot në heap Rust, kthen vetëm CapHandle (dy numra) tek C.
    fn issue(&mut self, module_nonce: u64) -> CapHandle {
        let gen = self.counter;
        self.counter += 1;
        self.slots.insert(gen, Box::new(CapSlot {
            valid: AtomicBool::new(true),
            gen,
            expected_nonce: module_nonce,
        }));
        CapHandle { gen, nonce: module_nonce }
    }

    // ═══ MURI MATEMATIK ═══════════════════════════════════════════════════
    //   compare_exchange(true→false, AcqRel, Acquire)
    //     valid==true  → atomikisht false → Ok  → LgcToken (1× konsum)
    //     valid==false → s'ndryshon       → Err → AlreadyConsumed (burned)
    fn validate_and_burn(&self, h: CapHandle) -> Result<LgcToken, FfiErr> {
        let slot = self.slots.get(&h.gen).ok_or(FfiErr::HandleNotFound)?;
        // Gjenerata në handle duhet të jetë po ajo që mban slot-i real.
        match slot.gen != h.gen { true => return Err(FfiErr::HandleNotFound), false => {} }
        // LAW_5: NK-check (nonce matematik) PARA operacionit.
        match slot.expected_nonce != h.nonce { true => return Err(FfiErr::NonceMismatch), false => {} }
        slot.valid
            .compare_exchange(true, false, Ordering::AcqRel, Ordering::Acquire)
            .map(|_| LgcToken(PhantomData))
            .map_err(|_| FfiErr::AlreadyConsumed)
    }
}

// ════════════════════════════════════════════════════════════════════════════
// §5  SINGLETON REGISTRY — C nuk sheh tipin kurrë
// ════════════════════════════════════════════════════════════════════════════

static GATE: OnceLock<Mutex<SovereignGate>> = OnceLock::new();
fn gate() -> &'static Mutex<SovereignGate> {
    GATE.get_or_init(|| Mutex::new(SovereignGate::new()))
}


// ════════════════════════════════════════════════════════════════════════════
// §5.1  VERIFICATION OUTPUT SEAL — Shadow mbyll XY me token autoritativ
// ════════════════════════════════════════════════════════════════════════════

pub fn seal_verified_output(
    session_id:       &str,
    parent_i0:        &str,
    primitive_anchor: u64,
    xy_digest:        u64,
    pd_binding_digest: u64,
    pd_continuum_activation_digest: u64,
    verdict:          &SupremeVerdict,
) -> Result<VerificationReceipt, FfiErr> {
    let mut guard = gate().lock().unwrap_or_else(|p| {
        eprintln!("[SOVEREIGN] ANOMALY: gate lock poisoned te verification seal — recovering");
        p.into_inner()
    });
    let handle = guard.issue(S_SHADOW_VERIFY);
    let token = guard.validate_and_burn(handle)?;
    drop(guard);
    Ok(verification_receipt_from_token(
        token, handle, session_id, parent_i0, primitive_anchor, xy_digest,
        pd_binding_digest, pd_continuum_activation_digest, verdict,
    ))
}

fn verification_receipt_from_token(
    _token: LgcToken,
    handle: CapHandle,
    session_id: &str,
    parent_i0: &str,
    primitive_anchor: u64,
    xy_digest: u64,
    pd_binding_digest: u64,
    pd_continuum_activation_digest: u64,
    verdict: &SupremeVerdict,
) -> VerificationReceipt {
    let living_trust_digest = crate::living_trust_contract::LivingTrustSeal {
        identity_sha256: verdict.living_trust_sha256,
        intensity: verdict.living_trust_intensity,
        kind: verdict.living_trust_kind,
        sovereign_value: verdict.living_trust_sovereign_value,
    }.identity_digest();
    let receipt_id = crate::lab_contracts::verification_receipt::receipt_id(
        session_id,
        parent_i0,
        primitive_anchor,
        xy_digest,
        pd_binding_digest,
        pd_continuum_activation_digest,
        &verdict.living_trust_sha256,
        verdict.verified,
        verdict.primitive,
        handle.gen,
        handle.nonce,
    );
    VerificationReceipt {
        session_id: session_id.to_string(),
        parent_i0: parent_i0.to_string(),
        primitive_anchor,
        xy_digest,
        pd_binding_digest,
        pd_continuum_activation_digest,
        living_trust_digest,
        living_trust_sha256: verdict.living_trust_sha256,
        y_verdict: verdict.verified,
        x_verdict: verdict.primitive,
        seal: handle.nonce,
        generation: handle.gen,
        receipt_id,
    }
}

// ════════════════════════════════════════════════════════════════════════════
// §6  FFI BOUNDARY — KUFIRI SOVRAN
//     ▲ Rust:  LgcToken  CapSlot  SovereignGate  AtomicBool
//     ▼ C:     CapHandle(u64,u64) + int return.  LgcToken KURRË nuk kalon.
// ════════════════════════════════════════════════════════════════════════════

/// C thërret: merr CapHandle(gen, nonce).
#[no_mangle]
pub extern "C" fn sovereign_issue_capability(module_seal: u64) -> CapHandle {
    // PJESA C: lock-u SOVRAN (autoriteti i token-ave) — nëse helmohet, njofto PARA
    // rikuperimit (gjendja duhet audituar). Rikuperimi mbetet (fail-safe), por jo i heshtur.
    gate().lock().unwrap_or_else(|p| {
        eprintln!("[SOVEREIGN] ANOMALY: gate lock poisoned te issue() — recovering, gjendja sovrane duhet audituar");
        p.into_inner()
    }).issue(module_seal)
}

/// C thërret: provon shkrimin persistent me handle.
///
/// LIDHJA ME GJURMUESHMËRINË: kur tokeni lind (vula atomike kalon), shkrimi
/// është i AUTORIZUAR; ndërtohet një gjurmë kapaciteti (proof = [gen, nonce]),
/// kontrollohet me ligjin e gjurmueshmërisë, regjistrohet, dhe dija shkruhet.
///
/// Kthim:  0 = OK (token: born→used→dropped ∈ Rust)
///        -1 = AlreadyConsumed (sulm klonimi → shënohet VERIFIED_NEGATIVE)
///        -2 = HandleNotFound
///        -3 = NonceMismatch
///        -4 = NoTrace (autorizim OK por pa gjurmë → inputi fshihet)
///        -5 = EmptyPayload (autorizim OK por pa të dhëna — fail-closed, #11)
///        -6 = VaultUnavailable (autoriteti persistent nuk është inicializuar)
///        -7 = VaultWriteFailed (backend-i refuzoi shkrimin)
///
/// # Safety
/// Kur `data_len > 0`, `data_ptr` duhet të tregojë te të paktën `data_len` byte
/// të lexueshëm dhe të mbetet valid gjatë gjithë thirrjes. Pointeri mund të
/// lexohet edhe në rrugën `AlreadyConsumed` për gjurmën e sulmit të klonimit.
#[no_mangle]
pub unsafe extern "C" fn sovereign_validate_and_write(
    handle:   CapHandle,
    data_ptr: *const u8,
    data_len: usize,
) -> i32 {
    match gate().lock().unwrap_or_else(|p| {
        eprintln!("[SOVEREIGN] ANOMALY: gate lock poisoned te validate_and_burn() — recovering, gjendja sovrane duhet audituar");
        p.into_inner()
    }).validate_and_burn(handle) {
        Ok(token) => {
            // SAFETY: përcillet kontrata e pointerit nga kufiri publik unsafe.
            unsafe { sovereign_commit(token, handle, data_ptr, data_len) }
        }
        Err(FfiErr::AlreadyConsumed) => {
            // ANOMALI: handle i klonuar/ri-përdorur nga C — sulm i dokumentuar.
            eprintln!("[SOVEREIGN] ANOMALY: gen={} AlreadyConsumed — clone attack?", handle.gen);
            // SAFETY: përcillet kontrata e pointerit nga kufiri publik unsafe.
            unsafe { record_clone_attack(handle, data_ptr, data_len) };
            -1
        }
        Err(FfiErr::HandleNotFound) => -2,
        Err(FfiErr::NonceMismatch)  => -3,
    }
}

/// LAW_3: VETËM ky bllok (Shadow-controlled, me token të gjallë) shkruan
/// persistent. `_token` konsumohet këtu dhe bie në fund — C s'mund ta riprodhojë.
unsafe fn sovereign_commit(_token: LgcToken, handle: CapHandle, data_ptr: *const u8, data_len: usize) -> i32 {
    // FAIL-CLOSED (#11): payload bosh NUK është sukses — kthe error të veçantë.
    // Para: kthente 0 (OK) → "s'ka payload" maskohej si "shkrim i suksesshëm".
    // Tani: -5 EmptyPayload → thirrësi e di që asgjë s'u shkrua. Zero if — match.
    match data_ptr.is_null() || data_len == 0 {
        true  => return -5, // EmptyPayload — autorizim OK por pa të dhëna për t'u shkruar
        false => {}
    }
    // SAFETY: kontrata e caller-it unsafe garanton pointer të lexueshëm për `data_len`.
    let data = unsafe { std::slice::from_raw_parts(data_ptr, data_len) };

    // input_id = hash i përmbajtjes; proof = lineage i kapacitetit (gen, nonce).
    let input_id = fnv(data);
    let proof = [handle.gen, handle.nonce];
    let lineage = Lineage::derive_capability(input_id, &proof);

    // LIGJI I GJURMUESHMËRISË: pa gjurmë → fshije (mos shkruaj).
    match lineage.is_traceable() {
        false => {
            ledger().note_purge();
            return -4;
        }
        true => {}
    }

    // Gjurma regjistrohet VETËM pasi shkrimi persistent ka përfunduar.
    // 0 nuk lejohet kur vault-i mungon ose backend-i refuzon shkrimin.
    persist_verified(get_global_vault(), input_id, data.to_vec(), lineage)
    // _token bie këtu: rishkrim me të njëjtin handle = AlreadyConsumed.
}

/// Njësia e persistimit ndahet nga pointer boundary që të testohet pa mutuar
/// singleton-in global. Kthen 0 vetëm pasi fact-i dhe lineage janë regjistruar.
fn persist_verified(
    vault: Option<std::sync::Arc<KnowledgeVault>>,
    input_id: u64,
    data: Vec<u8>,
    lineage: Lineage,
) -> i32 {
    match vault {
        Some(vault) => match vault.write_verified(
            input_id, data, KnowledgeBand::Fact, 1.0, now_ns(),
        ) {
            Ok(()) => {
                ledger().record(lineage);
                0
            }
            Err(e) => {
                eprintln!("[SOVEREIGN] ALARM: persistent write dështoi ({e:?})");
                -7
            }
        },
        None => {
            eprintln!("[SOVEREIGN] ALARM: global vault mungon — shkrimi refuzohet");
            -6
        }
    }
}

/// Sulm klonimi i detektuar → Shadow Judiciary e shënon si VERIFIED_NEGATIVE
/// (kufi aktiv: kjo rrugë FFI është provuar e pavlefshme).
unsafe fn record_clone_attack(handle: CapHandle, data_ptr: *const u8, data_len: usize) {
    let input_id = match !data_ptr.is_null() && data_len > 0 {
        true => {
            // SAFETY: kontrata e caller-it unsafe garanton pointer të lexueshëm.
            fnv(unsafe { std::slice::from_raw_parts(data_ptr, data_len) })
        },
        false => handle.gen,
    };
    match get_global_vault() {
        Some(vault) => {
        let ctx = NegativeContext {
            failure_reason_code: 0xC10E, // "CLONE"
            process:             "FFI_CLONE_ATTACK".to_string(),
            mass_at_fail:        0.0,
            frequency:           1,
        };
        // RUNTIME HARDENING (      ): shkrimi i NK (clone attack) bëhet eksplicit.
        match vault.write_negative(input_id, Vec::new(), ctx, now_ns()) {
            Ok(())  => {}
            Err(e) => eprintln!("[SOVEREIGN] ALARM: regjistrimi i clone attack dështoi ({e:?})"),
        }
    }
        None => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_payload_is_rejected_after_capability_validation() {
        let h = sovereign_issue_capability(S_SHADOW_WRITE);
        assert_ne!(h, CapHandle::INVALID);
        // SAFETY: len=0 lejon pointer null sipas kontratës FFI.
        let r = unsafe { sovereign_validate_and_write(h, std::ptr::null(), 0) };
        assert_eq!(r, -5); // token konsumohet, por payload bosh nuk maskohet si sukses
    }

    #[test]
    fn clone_attack_second_use_is_refused() {
        let h_orig = sovereign_issue_capability(S_SHADOW_WRITE);
        let h_copy = h_orig; // memcpy perfekt — çdo bit identik
        assert_eq!(h_orig, h_copy);

        // SAFETY: len=0 lejon pointer null; përdorimi i parë djeg token-in edhe pse kthen -5.
        let r1 = unsafe { sovereign_validate_and_write(h_orig, std::ptr::null(), 0) };
        assert_eq!(r1, -5); // AtomicBool: true → false; payload bosh refuzohet

        // SAFETY: len=0 lejon pointer null; handle-i i kopjuar duhet të refuzohet.
        let r2 = unsafe { sovereign_validate_and_write(h_copy, std::ptr::null(), 0) };
        assert_eq!(r2, -1); // CAS(false→false) = refuzim hardware
    }

    #[test]
    fn forged_nonce_is_rejected() {
        let h_real = sovereign_issue_capability(S_SHADOW_WRITE);
        let h_fake = CapHandle { gen: h_real.gen, nonce: h_real.nonce ^ 0xDEAD_BEEF };
        // SAFETY: len=0 lejon pointer null.
        assert_eq!(unsafe { sovereign_validate_and_write(h_fake, std::ptr::null(), 0) }, -3);
    }

    #[test]
    fn unknown_handle_is_not_found() {
        let h = CapHandle { gen: 9_999_999, nonce: S_SHADOW_WRITE };
        // SAFETY: len=0 lejon pointer null.
        assert_eq!(unsafe { sovereign_validate_and_write(h, std::ptr::null(), 0) }, -2);
    }

    #[test]
    fn verification_output_is_sealed_by_single_use_token() {
        let proof = crate::living_trust_contract::LivingTrustProof {
            action_state: 0x1122_3344_5566_7788,
            action_mask: crate::living_trust_contract::REQUIRED_ACTION_MASK,
            required_action_mask: crate::living_trust_contract::REQUIRED_ACTION_MASK,
            verified: 1,
            primitive: 1,
            knowledge_band: KnowledgeBand::Fact.as_u8(),
            lgc_law: 0x03,
            system_laws_seal:
                crate::lab_contracts::gjata_collapse_law::system_laws_seal(),
            scientific_project_sha256: [0u8; 32],
            scientific_project_verdict_sha256: [0u8; 32],
            sovereign_flags: 0xA451,
        };
        let trust = crate::living_trust_contract::compute(proof, 1.0)
            .expect("Vula e Gjallë");
        let verdict = SupremeVerdict {
            verified: 1,
            primitive: 1,
            knowledge_band: KnowledgeBand::Fact,
            lgc_law: 0x03,
            legacy_score: 1.0,
            lgc_dsl: "LGC::SHADOW".to_string(),
            living_trust_sha256: trust.identity_sha256,
            living_trust_intensity: trust.intensity,
            living_trust_kind: trust.kind,
            living_trust_action_state: proof.action_state,
            living_trust_action_mask: proof.action_mask,
            living_trust_required_action_mask: proof.required_action_mask,
            living_trust_system_laws_seal: proof.system_laws_seal,
            living_trust_sovereign_flags: proof.sovereign_flags,
            living_trust_sovereign_value: trust.sovereign_value,
        
            project_id: 0,
            project_status: shadow_contracts::PROJECT_STATUS_NONE,
            project_context_digest: 0,
            project_evidence_digest: 0,
            project_context_sha256: [0u8; 32],
            project_evidence_sha256: [0u8; 32],
            novel_factualized: false,
            novel_trl_level: 0,
            novel_proof_score: 0,
            novel_rejection: 0,
        };
        let receipt = seal_verified_output(
            "sess-v", "i0-v", 0xAA, 0xBB, 0xCC, 0xDD, &verdict,
        ).expect("verification seal duhet të hapet");
        assert_eq!(receipt.session_id, "sess-v");
        assert_eq!(receipt.parent_i0, "i0-v");
        assert_eq!(receipt.primitive_anchor, 0xAA);
        assert_eq!(receipt.xy_digest, 0xBB);
        assert_eq!(receipt.pd_binding_digest, 0xCC);
        assert_eq!(receipt.pd_continuum_activation_digest, 0xDD);
        assert_eq!(receipt.living_trust_digest, trust.identity_digest());
        assert_eq!((receipt.y_verdict, receipt.x_verdict), (1, 1));
        assert_eq!(receipt.seal, S_SHADOW_VERIFY);
        assert_eq!(
            S_SHADOW_VERIFY,
            crate::seal_registry::module_seal(b"SHADOW_VERIFICATION_OUTPUT"),
        );
        let expected_receipt = crate::lab_contracts::verification_receipt::receipt_id(
            &receipt.session_id,
            &receipt.parent_i0,
            receipt.primitive_anchor,
            receipt.xy_digest,
            receipt.pd_binding_digest,
            receipt.pd_continuum_activation_digest,
            &receipt.living_trust_sha256,
            receipt.y_verdict,
            receipt.x_verdict,
            receipt.generation,
            receipt.seal,
        );
        assert_ne!(receipt.generation, 0);
        assert_eq!(receipt.receipt_id, expected_receipt);
    }

    #[test]
    fn replay_third_time_still_burned() {
        let h = sovereign_issue_capability(S_SHADOW_WRITE);
        // SAFETY: len=0 lejon pointer null. Përdorimi i parë djeg token-in dhe kthen -5.
        assert_eq!(unsafe { sovereign_validate_and_write(h, std::ptr::null(), 0) }, -5);
        // SAFETY: i njëjti kusht pointeri; replay duhet të mbetet i djegur.
        assert_eq!(unsafe { sovereign_validate_and_write(h, std::ptr::null(), 0) }, -1);
        // SAFETY: i njëjti kusht pointeri; përdorimi i tretë nuk ringjall token-in.
        assert_eq!(unsafe { sovereign_validate_and_write(h, std::ptr::null(), 0) }, -1);
    }

    #[test]
    fn authorized_persist_records_fact_without_global_state() {
        let vault = std::sync::Arc::new(KnowledgeVault::new());
        let before = vault.fact_count();
        let data = b"verified-optimal-knowledge-v151".to_vec();
        let input_id = fnv(&data);
        let lineage = Lineage::derive_capability(input_id, &[1, S_LAB_TRL1]);

        let r = persist_verified(Some(vault.clone()), input_id, data, lineage);
        assert_eq!(r, 0);
        assert_eq!(vault.fact_count(), before + 1, "0 duhet të nënkuptojë shkrim real");
    }

    #[test]
    fn missing_vault_is_never_reported_as_success() {
        let data = b"verified-optimal-knowledge-v151".to_vec();
        let input_id = fnv(&data);
        let lineage = Lineage::derive_capability(input_id, &[2, S_LAB_TRL2]);
        assert_eq!(persist_verified(None, input_id, data, lineage), -6);
    }
}
