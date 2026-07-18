// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  BRIDGE/SHADOW_CALLABLE.RS — Modulet që THIRREN nga Quantum/Light      ║
// ║  Gjata Legacy™ | Arkitekt: Bledar Gjata                    (FAZA 6)    ║
// ║                                                                          ║
// ║  Shadow ekspozon endpoint-e të QARTA që Quantum dhe Light i thërrasin.  ║
// ║  Të gatshme për lidhje finale me përsosmëri (Fazat 4-5 finalizohen pas).║
// ║                                                                          ║
// ║  PROBLEMI REAL (verifikuar te kodi, jo hamendësim):                    ║
// ║   • Quantum pret SupremeOutcome — 3 gjendje (PrimitiveKnowledge /       ║
// ║     VerifiableNotPrimitive / ChainRejected). bit(): 1 vetëm për prim.  ║
// ║   • Light pret ShadowVerdict — 2 gjendje (PrimitiveKnowledge /          ║
// ║     VerifiableNotPrimitive). bit(): 1 vetëm për primitiv.              ║
// ║                                                                          ║
// ║  Asimetria: Shadow ka 3 gjendje, Light njeh 2. ChainRejected (purge)    ║
// ║  → te Light bie në bit 0 (jo-primitiv), por RUHET shkaku si shënim.    ║
// ║                                                                          ║
// ║  LIGJET E RUAJTURA:                                                    ║
// ║   • Ligji suprem i DY origjinave: çdo endpoint merr PassPackage +       ║
// ║     LightEnvelope (jo një envelope të vetëm) — pa humbje.              ║
// ║   • bit() suprem: 1 VETËM për primitive_knowledge (identik ndër platforma).
// ║   • DISTRUST: çdo dështim i brendshëm → refuzim (jo pranim me gabim).   ║
// ║  Zero if/else — match/formula.                                        ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::bridge::{LightInbound, QuantumInbound};
use crate::bridge::shadow_out::{ShadowLightResponse, SupremeOutcome};
use crate::shadow_gateway::Shadow;

// ─────────────────────────────────────────────────────────────────────────────
// LIGHT VERDICT — përgjigja në formatin 2-gjendjesh që LIGHT pret
// ─────────────────────────────────────────────────────────────────────────────

/// Verdikti siç e pret Light (mirror EKZAKT i `ShadowVerdict` te
/// light/shadow_seal_bridge.rs — 2 gjendje, bit() identik).
///
/// Shadow ka 3 gjendje (SupremeOutcome); Light njeh 2. Konvertimi është
/// pa humbje në nivel BITI (që është ç'i intereson Light: 1/0), dhe shkaku
/// i ChainRejected ruhet veçmas te `shadow_note`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightVerdict {
    /// 1 — vula 500 mbijetoi: primitive_knowledge.
    PrimitiveKnowledge,
    /// 0 — u fragmentua ose u refuzua: i verifikueshëm, jo primitiv.
    VerifiableNotPrimitive,
}

impl LightVerdict {
    /// bit() — biti suprem, IDENTIK me Light: 1 vetëm për primitiv. Zero if.
    pub fn bit(self) -> u8 {
        (self == LightVerdict::PrimitiveKnowledge) as u8
    }

    /// from_supreme — projekton SupremeOutcome (3) → LightVerdict (2).
    ///
    /// PrimitiveKnowledge → PrimitiveKnowledge (1).
    /// {VerifiableNotPrimitive, ChainRejected} → VerifiableNotPrimitive (0).
    /// Zero if — match shterues (purge dhe verifiable bien të dyja te bit 0).
    pub fn from_supreme(o: SupremeOutcome) -> Self {
        match o {
            SupremeOutcome::PrimitiveKnowledge => LightVerdict::PrimitiveKnowledge,
            SupremeOutcome::VerifiableNotPrimitive => LightVerdict::VerifiableNotPrimitive,
            SupremeOutcome::ChainRejected => LightVerdict::VerifiableNotPrimitive,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            LightVerdict::PrimitiveKnowledge     => "1:PRIMITIVE_KNOWLEDGE",
            LightVerdict::VerifiableNotPrimitive => "0:VERIFIABLE_NOT_PRIMITIVE",
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// LIGHT RESPONSE — paketa e plotë e përgjigjes drejt Light
// ─────────────────────────────────────────────────────────────────────────────

/// Përgjigja e plotë drejt Light: verdikti 2-gjendjesh + shënimi sovran.
/// Ruan session_id (që Light të lidhë përgjigjen me kërkesën) + DSL-in.
#[derive(Debug, Clone)]
pub struct LightResponse {
    pub session_id:   String,
    pub verdict:      LightVerdict,
    pub bit:          u8,        // 0/1 — biti suprem (kopje e drejtpërdrejtë)
    pub shadow_note:  String,    // DSL-i sovran (LGC::SHADOW|...)
    pub legacy_score: f32,
    pub y_verdict: u8,
    pub x_verdict: u8,
    pub verification_seal: u64,
    pub verification_generation: u64,
    pub verification_receipt_id: String,
    pub verification_parent_i0: String,
    pub verification_primitive_anchor: u64,
    pub verification_xy_digest: u64,
    pub verification_pd_binding_digest: u64,
    pub verification_pd_continuum_activation_digest: u64,
    pub verification_living_trust_digest: u64,
}

impl LightResponse {
    /// from_shared — nga përgjigja e përbashkët (ShadowLightResponse) → forma Light.
    /// Projekton 3 gjendje → 2 dhe ruan shënimin/skorin. Zero if.
    pub fn from_shared(r: ShadowLightResponse) -> Self {
        let verdict = LightVerdict::from_supreme(r.verdict);
        Self {
            session_id:   r.session_id,
            verdict,
            bit:          verdict.bit(),
            shadow_note:  r.shadow_note,
            legacy_score: r.legacy_score,
            y_verdict: r.y_verdict,
            x_verdict: r.x_verdict,
            verification_seal: r.verification_seal,
            verification_generation: r.verification_generation,
            verification_receipt_id: r.verification_receipt_id,
            verification_parent_i0: r.verification_parent_i0,
            verification_primitive_anchor: r.verification_primitive_anchor,
            verification_xy_digest: r.verification_xy_digest,
            verification_pd_binding_digest: r.verification_pd_binding_digest,
            verification_pd_continuum_activation_digest:
                r.verification_pd_continuum_activation_digest,
            verification_living_trust_digest:
                r.verification_living_trust_digest,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// SHADOW CALLABLE — endpoint-i i unifikuar që Quantum/Light thërrasin
// ─────────────────────────────────────────────────────────────────────────────

/// Endpoint-et sovrane që Shadow ekspozon për t'u thirrur nga jashtë.
///
/// Të DY platformat thërrasin të njëjtin entry, por marrin formën që presin:
///   • Quantum → `call_for_quantum` → ShadowLightResponse (3 gjendje).
///   • Light   → `call_for_light`   → LightResponse (2 gjendje + bit).
///
/// Të dyja kalojnë nga i njëjti pipeline suprem (ingest_bridged) — asnjë
/// rrugë e dytë vendimi (Ligji: vetëm Shadow vendos, një pipeline).
pub trait ShadowCallable {
    /// Thirrje nga Quantum: merr dy origjinat, kthen përgjigjen 3-gjendjesh.
    fn call_for_quantum(&self, pkg: QuantumInbound, light: LightInbound) -> ShadowLightResponse;

    /// Thirrje nga Light: merr dy origjinat, kthen përgjigjen 2-gjendjesh + bit.
    fn call_for_light(&self, pkg: QuantumInbound, light: LightInbound) -> LightResponse;
}

impl ShadowCallable for Shadow {
    fn call_for_quantum(&self, pkg: QuantumInbound, light: LightInbound) -> ShadowLightResponse {
        let session = light.session_id.clone(); // ruaj për rrugën e gabimit
        // I njëjti pipeline suprem; DISTRUST: dështim → ChainRejected.
        match self.ingest_bridged(pkg, light) {
            Ok(r) => ShadowLightResponse::from_response(r),
            Err(e) => ShadowLightResponse {
                session_id:   session.clone(),
                verdict:      SupremeOutcome::ChainRejected,
                shadow_note:  format!("shadow_error: {e}"),
                legacy_score: 0.0,
                y_verdict: 0,
                x_verdict: 0,
                verification_seal: 0,
                verification_generation: 0,
                verification_receipt_id: String::new(),
                verification_parent_i0: session,
                verification_primitive_anchor: 0,
                verification_xy_digest: 0,
                verification_pd_binding_digest: 0,
                verification_pd_continuum_activation_digest: 0,
                verification_living_trust_digest: 0,
                reason_code:   0x5040_0001,  // SHADOW_INGEST_FAIL (#12)
                failure_stage: "sovereign",
                source_module: "shadow_callable::call_for_quantum",
            },
        }
    }

    fn call_for_light(&self, pkg: QuantumInbound, light: LightInbound) -> LightResponse {
        // Ripërdor rrugën e Quantum-it (i njëjti pipeline) → projekto te forma Light.
        let shared = self.call_for_quantum(pkg, light);
        LightResponse::from_shared(shared)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ADAPTOR — referencë te Shadow (për thirrësit që mbajnë &Shadow)
// ─────────────────────────────────────────────────────────────────────────────

/// Adaptor i emërtuar që mban &Shadow — për Quantum/Light që e thërrasin
/// Shadow-un përmes një reference (jo pronësi).
pub struct ShadowEndpoint<'a> {
    shadow: &'a Shadow,
}

impl<'a> ShadowEndpoint<'a> {
    pub fn new(shadow: &'a Shadow) -> Self {
        Self { shadow }
    }
}

impl ShadowCallable for ShadowEndpoint<'_> {
    fn call_for_quantum(&self, pkg: QuantumInbound, light: LightInbound) -> ShadowLightResponse {
        ShadowCallable::call_for_quantum(self.shadow, pkg, light)
    }

    fn call_for_light(&self, pkg: QuantumInbound, light: LightInbound) -> LightResponse {
        ShadowCallable::call_for_light(self.shadow, pkg, light)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ndihmës: QuantumInbound primitiv i fortë (vula 500, masë e lartë).
    fn strong_inbound() -> (QuantumInbound, LightInbound) {
        let q = QuantumInbound {
            package_id:           "pkg-6".to_string(),
            session_id:           "sess-6".to_string(),
            territory:            "INDUSTRIAL".to_string(),
            epistemic_mass:       0.95,
            evidence_density:     0.9,
            logical_coherence:    0.9,
            causal_integrity:     0.9,
            convergence_strength: 0.9,
            reproducibility:      0.9,
            potentiality_score:   0.9,
            ibe_score:            0.9,
            chain_coherence:      0.9,
            conservation_ok:      true,
            candidate_scores:     vec![0.9, 0.85],
            suggested_verdict:    1,
            primitive_flags:      0xA451,   // vula 500 e vlefshme
            raw_bytes:            b"strong-state".to_vec(),
            xy_x: None, xy_y: None, xy_mass: None, pd_binding_digest: None, pd_continuum_activation_digest: None,
            pd_parent_i0: None,
            collapse_refused: None, negative_paths: Vec::new(),
            hcp_input_id:         None,
            hcp_token_gen:        None,
            hcp_token_nonce:      None,
            hcp_directive_bit:    None,
            final_evidence_digest: 0,
            quantum_action_state: 0x1122_3344_5566_7788,
            quantum_action_mask: shadow_contracts::REQUIRED_ACTION_MASK,
            quantum_required_action_mask: shadow_contracts::REQUIRED_ACTION_MASK,
            pd_gcl_process_digest: 0,
            spine_completion_digest: 0,
            scientific_project: None,
        };
        let l = q.derive_light();
        (q, l)
    }

    #[test]
    fn light_verdict_bit_identical() {
        // bit() suprem: 1 vetëm për primitiv (identik me Light).
        assert_eq!(LightVerdict::PrimitiveKnowledge.bit(), 1);
        assert_eq!(LightVerdict::VerifiableNotPrimitive.bit(), 0);
    }

    #[test]
    fn supreme_3_projects_to_light_2() {
        // PrimitiveKnowledge → PrimitiveKnowledge (1).
        assert_eq!(
            LightVerdict::from_supreme(SupremeOutcome::PrimitiveKnowledge),
            LightVerdict::PrimitiveKnowledge
        );
        // VerifiableNotPrimitive → VerifiableNotPrimitive (0).
        assert_eq!(
            LightVerdict::from_supreme(SupremeOutcome::VerifiableNotPrimitive),
            LightVerdict::VerifiableNotPrimitive
        );
        // ChainRejected → VerifiableNotPrimitive (0) — purge bie te bit 0.
        assert_eq!(
            LightVerdict::from_supreme(SupremeOutcome::ChainRejected),
            LightVerdict::VerifiableNotPrimitive
        );
    }

    #[test]
    fn chain_rejected_keeps_bit_zero_for_light() {
        // Asimetria 3→2: ChainRejected → bit 0 (Light njeh vetëm 1/0).
        let lv = LightVerdict::from_supreme(SupremeOutcome::ChainRejected);
        assert_eq!(lv.bit(), 0);
    }

    #[test]
    fn light_response_from_shared_preserves_session() {
        let shared = ShadowLightResponse {
            session_id:   "sess-Z".to_string(),
            verdict:      SupremeOutcome::PrimitiveKnowledge,
            shadow_note:  "LGC::SHADOW|PRIMITIVE".to_string(),
            legacy_score: 0.7,
            y_verdict: 1,
            x_verdict: 1,
            verification_seal: 0xA451,
            verification_generation: 7,
            verification_receipt_id: "r7".to_string(),
            verification_parent_i0: "sess-Z".to_string(),
            verification_primitive_anchor: 0xAA,
            verification_xy_digest: 0xBB,
            verification_pd_binding_digest: 0xCC,
            verification_pd_continuum_activation_digest: 0xDD,
            verification_living_trust_digest: 0xEE,
            reason_code:   0,
            failure_stage: "none",
            source_module: "none",
        };
        let lr = LightResponse::from_shared(shared);
        assert_eq!(lr.session_id, "sess-Z");
        assert_eq!(lr.verdict, LightVerdict::PrimitiveKnowledge);
        assert_eq!(lr.bit, 1);
        assert!(lr.shadow_note.contains("LGC::SHADOW"));
        assert!((lr.legacy_score - 0.7).abs() < 1e-6);
    }

    #[test]
    fn callable_quantum_returns_3state() {
        let shadow = Shadow::new().unwrap();
        let (q, l) = strong_inbound();
        let resp = shadow.call_for_quantum(q, l);
        // Përgjigja ka session-in dhe një verdikt të vlefshëm.
        assert_eq!(resp.session_id, "sess-6");
    }

    #[test]
    fn callable_light_returns_2state_with_bit() {
        let shadow = Shadow::new().unwrap();
        let (q, l) = strong_inbound();
        let resp = shadow.call_for_light(q, l);
        // Forma Light: session + bit ∈ {0,1}.
        assert_eq!(resp.session_id, "sess-6");
        let bit_valid = (resp.bit == 0) | (resp.bit == 1);
        assert!(bit_valid);
        // bit përputhet me verdiktin.
        assert_eq!(resp.bit, resp.verdict.bit());
    }

    #[test]
    fn endpoint_delegates_to_shadow() {
        let shadow = Shadow::new().unwrap();
        let endpoint = ShadowEndpoint::new(&shadow);
        let (q, l) = strong_inbound();
        let resp = endpoint.call_for_light(q, l);
        assert_eq!(resp.session_id, "sess-6");
    }

    #[test]
    fn both_paths_use_same_pipeline() {
        // Quantum dhe Light kalojnë nga i njëjti pipeline → biti përputhet.
        let shadow = Shadow::new().unwrap();
        let (q1, l1) = strong_inbound();
        let (q2, l2) = strong_inbound();
        let q_resp = shadow.call_for_quantum(q1, l1);
        let l_resp = shadow.call_for_light(q2, l2);
        // Biti suprem i Quantum (përmes SupremeOutcome) == biti i Light.
        assert_eq!(q_resp.verdict.bit(), l_resp.bit);
    }
}
