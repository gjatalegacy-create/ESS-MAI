// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  BRIDGE/SHADOW_OUT.RS — RRUGA E KTHIMIT Shadow → Quantum/Light        ║
// ║  Gjata Legacy™ | Bledar Gjata                                        ║
// ║                                                                          ║
// ║  Shadow vendos; këtu përgjigja sovrane shndërrohet në FORMËN që        ║
// ║  Quantum/Light presin. Strukturat janë MIRROR EKZAKT i kontratave të   ║
// ║  verifikuara në burim:                                                 ║
// ║    Quantum: quantum_shadow_bridge.rs  → ShadowVerdictMirror,           ║
// ║                                          ShadowResponseMirror          ║
// ║    Light:   shadow_seal_bridge.rs     → ShadowVerdict (1/0)            ║
// ║                                                                          ║
// ║  Hyrja e kthimit: `LightShadowBridge::receive_from_light` (emri që     ║
// ║  Quantum referon në OPSIONIN A) ruan LIGJIN SUPREM të DY    ║
// ║  origjinave: merr PassPackage + LightEnvelope (jo një envelope të      ║
// ║  vetëm), pra rruga është pa humbje.                                    ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::bridge::{LightInbound, QuantumInbound};
use crate::shadow_gateway::{Shadow, ShadowResponse};
use crate::types::SupremeVerdict;

/// Rezultati 3-gjendjesh — MIRROR i `ShadowVerdictMirror` të Quantum-it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupremeOutcome {
    /// 1 — vula 500 mbijetoi: primitive_knowledge.
    PrimitiveKnowledge,
    /// 0 — u fragmentua: i verifikueshëm, jo primitiv.
    VerifiableNotPrimitive,
    /// 0 — zinxhiri ra (pa gjurmë → purge).
    ChainRejected,
}

impl SupremeOutcome {
    /// Klasifikim BRANCHLESS nga `SupremeVerdict` (vendimi tashmë i llogaritur).
    /// Përparësia: purge (0x20) > primitive > i-verifikueshëm.
    pub fn from_verdict(v: &SupremeVerdict) -> Self {
        let purged = ((v.lgc_law & 0x20) != 0) as usize; // bit 0x20 = vetëm purge
        let prim   = (v.primitive == 1) as usize;
        // purged→2 ; jo-purge & prim→0 ; jo-purge & jo-prim→1
        let idx = purged * 2 + (1 - purged) * (1 - prim);
        const TABLE: [SupremeOutcome; 3] = [
            SupremeOutcome::PrimitiveKnowledge,
            SupremeOutcome::VerifiableNotPrimitive,
            SupremeOutcome::ChainRejected,
        ];
        TABLE[idx]
    }

    /// Biti suprem (si `ShadowVerdict::bit` te Light): 1 vetëm për primitiv.
    pub fn bit(self) -> u8 {
        (self == SupremeOutcome::PrimitiveKnowledge) as u8
    }

    pub fn label(self) -> &'static str {
        match self {
            SupremeOutcome::PrimitiveKnowledge     => "1:PRIMITIVE_KNOWLEDGE",
            SupremeOutcome::VerifiableNotPrimitive => "0:VERIFIABLE_NOT_PRIMITIVE",
            SupremeOutcome::ChainRejected          => "0:CHAIN_REJECTED",
        }
    }
}

/// Përgjigja e Shadow-it drejt Quantum/Light — MIRROR i `ShadowResponseMirror`
/// (Quantum) / `ShadowLightResponse` (kontrata e Shadow-it). Formë IDENTIKE.
#[derive(Debug, Clone)]
pub struct ShadowLightResponse {
    pub session_id:   String,
    pub verdict:      SupremeOutcome,
    pub shadow_note:  String,
    pub legacy_score: f32,
    /// Dëftesa e output-it të mbyllur nga Shadow (token-i mbetet privat).
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
    /// FIX #12/#13: kodi i arsyes (0 = sukses; jo-zero = shkaku specifik).
    /// bit-i (0/1) mbetet i thjeshtë; diagnoza e plotë ruhet këtu.
    pub reason_code:  u32,
    /// Faza ku ndodhi dështimi (parser/seal/auth/exchange/sovereign/none).
    pub failure_stage: &'static str,
    /// RUNTIME HARDENING (      ): moduli burim i dështimit (error surfacing i plotë).
    /// "none" në sukses; emri i modulit/urës në dështim.
    pub source_module: &'static str,
}

impl ShadowLightResponse {
    /// Maps përgjigjen e brendshme → përgjigjen MIRROR (ZHVENDOS — Ligji 0).
    pub fn from_response(r: ShadowResponse) -> Self {
        let verdict      = SupremeOutcome::from_verdict(&r.verdict);
        let legacy_score = r.verdict.legacy_score;
        let receipt      = r.verification_receipt;
        ShadowLightResponse {
            session_id:   r.session_id,
            verdict,
            shadow_note:  r.verdict.lgc_dsl,
            legacy_score,
            y_verdict: receipt.y_verdict,
            x_verdict: receipt.x_verdict,
            verification_seal: receipt.seal,
            verification_generation: receipt.generation,
            verification_receipt_id: receipt.receipt_id,
            verification_parent_i0: receipt.parent_i0,
            verification_primitive_anchor: receipt.primitive_anchor,
            verification_xy_digest: receipt.xy_digest,
            verification_pd_binding_digest: receipt.pd_binding_digest,
            verification_pd_continuum_activation_digest:
                receipt.pd_continuum_activation_digest,
            verification_living_trust_digest: receipt.living_trust_digest,
            reason_code:   0,
            failure_stage: "none",
            source_module: "none",
        }
    }
}

/// Kontrata e hyrjes së kthimit që Quantum/Light referojnë.
/// (Emri `receive_from_light` përkon me OPSIONIN A të quantum_shadow_bridge.rs.)
pub trait LightShadowBridge {
    /// Dy origjina (Ligji suprem): propozimi (PassPackage) + transporti (Light).
    fn receive_from_light(&self, pkg: QuantumInbound, light: LightInbound) -> ShadowLightResponse;
}

impl LightShadowBridge for Shadow {
    fn receive_from_light(&self, pkg: QuantumInbound, light: LightInbound) -> ShadowLightResponse {
        let session = light.session_id.clone(); // ruaj për rrugën e gabimit
        match self.ingest_bridged(pkg, light) {
            Ok(r) => ShadowLightResponse::from_response(r),
            // DISTRUST: çdo dështim i brendshëm → refuzim (s'pranohet me gabim).
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
                source_module: "shadow_out::receive_from_light",
            },
        }
    }
}

/// Adaptori i emërtuar siç e referon Quantum (`DefaultLightShadowBridge`).
/// Mban referencë te Shadow (: brid-i ka nevojë për arkivin sovran).
pub struct DefaultLightShadowBridge<'a> {
    shadow: &'a Shadow,
}

impl<'a> DefaultLightShadowBridge<'a> {
    pub fn new(shadow: &'a Shadow) -> Self {
        Self { shadow }
    }
}

impl LightShadowBridge for DefaultLightShadowBridge<'_> {
    fn receive_from_light(&self, pkg: QuantumInbound, light: LightInbound) -> ShadowLightResponse {
        // Delegim te implementimi i Shadow-it (metodë trait, pa paqartësi).
        LightShadowBridge::receive_from_light(self.shadow, pkg, light)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{KnowledgeBand, SupremeVerdict};

    fn verdict(primitive: u8, verified: u8, lgc_law: u32) -> SupremeVerdict {
        SupremeVerdict {
            verified,
            primitive,
            knowledge_band: KnowledgeBand::Fact,
            lgc_law,
            legacy_score: 0.5,
            lgc_dsl: "LGC::SHADOW|test".to_string(),
            living_trust_sha256: [0u8; 32],
            living_trust_intensity: 0,
            living_trust_kind: 0,
            living_trust_action_state: 0,
            living_trust_action_mask: 0,
            living_trust_required_action_mask: 0,
            living_trust_system_laws_seal: 0,
            living_trust_sovereign_flags: 0,
            living_trust_sovereign_value: 0,
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
        }
    }

    #[test]
    fn primitive_maps_to_primitive_knowledge() {
        let o = SupremeOutcome::from_verdict(&verdict(1, 1, 0x03));
        assert_eq!(o, SupremeOutcome::PrimitiveKnowledge);
        assert_eq!(o.bit(), 1);
    }

    #[test]
    fn verified_nonprimitive_maps_to_verifiable() {
        let o = SupremeOutcome::from_verdict(&verdict(0, 1, 0x02));
        assert_eq!(o, SupremeOutcome::VerifiableNotPrimitive);
        assert_eq!(o.bit(), 0);
    }

    #[test]
    fn purge_maps_to_chain_rejected() {
        let o = SupremeOutcome::from_verdict(&verdict(0, 0, 0x20));
        assert_eq!(o, SupremeOutcome::ChainRejected);
        assert_eq!(o.bit(), 0);
    }

    #[test]
    fn negative_maps_to_verifiable_not_primitive() {
        // verified=0, jo purge (0x10 NEGATIVE) → jo primitiv, jo i refuzuar nga zinxhiri.
        let o = SupremeOutcome::from_verdict(&verdict(0, 0, 0x10));
        assert_eq!(o, SupremeOutcome::VerifiableNotPrimitive);
    }

    #[test]
    fn from_response_moves_session_and_dsl() {
        let v = verdict(1, 1, 0x03);
        let receipt = crate::sovereign_ffi_gate::seal_verified_output(
            "sess-Z", "sess-Z", 0xAA, 0xBB, 0xCC, 0xDD, &v,
        ).expect("verification receipt");
        let r = ShadowResponse {
            session_id: "sess-Z".to_string(),
            verdict: v,
            known_negative: None,
            verification_receipt: receipt,
        };
        let out = ShadowLightResponse::from_response(r);
        assert_eq!(out.session_id, "sess-Z");
        assert_eq!(out.verdict, SupremeOutcome::PrimitiveKnowledge);
        assert!((out.legacy_score - 0.5).abs() < 1e-6);
        assert!(out.shadow_note.contains("LGC::SHADOW"));
        assert_eq!((out.y_verdict, out.x_verdict), (1, 1));
        assert_ne!(out.verification_seal, 0);
        assert!(!out.verification_receipt_id.is_empty());
    }
}
