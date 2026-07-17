// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAYER2/LAB_FLOW.RS — Orkestruesi i Rrjedhës Lab-to-Lab (Shadow)     ║
// ║  GJATA LEGACY™ — ESS-MAI Shadow                          (LAYER 2)    ║
// ║                                                                          ║
// ║  Hook i PASTËR që lidh Layer 2 me rrjedhën e plotë, PA prekur bazën     ║
// ║  operative të platformave. Demonstron rrjedhën e sinkronizuar:         ║
// ║                                                                          ║
// ║    INPUT → LIGHT (TrustContext) → QUANTUM (PressureReport+TrlEvidence   ║
// ║    +EvidencePackage) → SHADOW (VerificationState+LabVerdict+Ledger+     ║
// ║    NegativeKnowledge) → LIGHT (Trust update + Output)                   ║
// ║                                                                          ║
// ║  Ky modul vetëm KOORDINON mesazhet (LabMessage) dhe thërret gjykatat;   ║
// ║  nuk shton logjikë të re vendimmarrjeje. Zero if/else.                ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::lab_contracts::evidence::EvidencePackage;
use crate::lab_contracts::message::{LabMessage, LabNode, LabPayloadKind};
use crate::layer2::shadow_courts::{FinalVerdictCourt, SupremeVerification};

// ─────────────────────────────────────────────────────────────────────────────
// FLOW STAGE — faza e rrjedhës
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlowStage {
    LightTrust,        // Light prodhon TrustContext
    QuantumPressure,   // Quantum prodhon PressureReport+EvidencePackage
    ShadowVerdict,     // Shadow gjykon
    LightUpdate,       // Light përditëson besimin
}

impl FlowStage {
    pub fn label(self) -> &'static str {
        match self {
            FlowStage::LightTrust      => "LIGHT_TRUST",
            FlowStage::QuantumPressure => "QUANTUM_PRESSURE",
            FlowStage::ShadowVerdict   => "SHADOW_VERDICT",
            FlowStage::LightUpdate     => "LIGHT_UPDATE",
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// LAB FLOW — orkestron rrjedhën në Shadow
// ─────────────────────────────────────────────────────────────────────────────

/// Rezultati i rrjedhës së plotë në Shadow.
#[derive(Debug, Clone)]
pub struct FlowResult {
    pub verification:    SupremeVerification,
    pub inbound_valid:   bool,   // a ishte mesazhi hyrës i vlefshëm
    pub stage:           FlowStage,
}

pub struct LabFlow;

impl LabFlow {
    /// receive_from_quantum — Shadow pranon EvidencePackage nga Quantum me LabMessage.
    ///
    /// Verifikon zarfin (vula 500 + rruga Quantum→Shadow), pastaj thërret
    /// gjykatat supreme. Zero if — match mbi vlefshmërinë.
    pub fn receive_from_quantum(
        msg: &LabMessage,
        pkg: &EvidencePackage,
        recorded_at: u64,
    ) -> FlowResult {
        // Verifiko zarfin: vula 500 + rruga e saktë + lloji i ngarkesës.
        let seal_ok = msg.seal_ok();
        let route_ok = Self::is_quantum_to_shadow(msg);
        let payload_ok = (msg.payload_kind as u8) == (LabPayloadKind::EvidencePackage as u8);
        let inbound_valid = seal_ok & route_ok & payload_ok;

        // Gjykimi suprem (gjithmonë kryhet; vlefshmëria regjistrohet).
        let verification = FinalVerdictCourt::adjudicate(pkg, recorded_at);

        FlowResult {
            verification,
            inbound_valid,
            stage: FlowStage::ShadowVerdict,
        }
    }

    /// is_quantum_to_shadow — a është rruga Quantum→Shadow? Zero if.
    fn is_quantum_to_shadow(msg: &LabMessage) -> bool {
        let from_quantum = (msg.from as u8) == (LabNode::Quantum as u8);
        let to_shadow    = (msg.to as u8) == (LabNode::Shadow as u8);
        from_quantum & to_shadow
    }

    /// build_response_to_light — ndërton LabMessage me verdiktin për Light.
    /// Shadow→Light: dërgon verdiktin që Light të përditësojë besimin. Zero if.
    pub fn build_response_to_light(claim_id: u64, sequence: u64, timestamp: u64) -> LabMessage {
        LabMessage::new(
            claim_id,
            LabNode::Shadow,
            LabNode::Light,
            LabPayloadKind::Verdict,
            sequence,
            timestamp,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lab_contracts::trl::TrlEvidence;
    use crate::lab_contracts::trust::{TrustContext, TrustGuidance};
    use crate::lab_contracts::pressure::{PressureReport, PressureSource};
    use crate::lab_contracts::evidence::EvidenceKind;

    fn valid_package() -> EvidencePackage {
        let mut e = TrlEvidence::pending(1);
        e.trl_level = 3; e.trl_passed = true; e.confidence = 0.8;
        e.cross_domain_tested = true; e.timestamp = 100;
        let mut p = PressureReport::none(1, PressureSource::QuantumNpro);
        p.pressure_path = "NReverse".to_string();
        p.survived_level = 3;
        EvidencePackage {
            claim_id: 1, claim: "X".to_string(), reasoning: "Y".to_string(),
            trl_evidence: e,
            trust_context: TrustContext {
                trace_id: 1, domain: "d".to_string(),
                trust_score: 0.7, reliability_score: 0.6, verification_score: 0.5,
                history_refs: vec![1], guidance: TrustGuidance::Recommended,
            },
            pressure_report: p,
            evidence_chain: vec![],
            cross_domain_checks: vec!["d2".to_string()],
            kind: EvidenceKind::Reasoned,
        }
    }

    #[test]
    fn valid_inbound_from_quantum() {
        let msg = LabMessage::new(1, LabNode::Quantum, LabNode::Shadow, LabPayloadKind::EvidencePackage, 1, 100);
        let result = LabFlow::receive_from_quantum(&msg, &valid_package(), 1000);
        assert!(result.inbound_valid);
        assert_eq!(result.stage, FlowStage::ShadowVerdict);
    }

    #[test]
    fn wrong_route_invalid() {
        // Light→Shadow (jo Quantum→Shadow) → rrugë e pavlefshme.
        let msg = LabMessage::new(1, LabNode::Light, LabNode::Shadow, LabPayloadKind::EvidencePackage, 1, 100);
        let result = LabFlow::receive_from_quantum(&msg, &valid_package(), 1000);
        assert!(!result.inbound_valid);
    }

    #[test]
    fn wrong_payload_invalid() {
        // Ngarkesë e gabuar (TrustContext jo EvidencePackage).
        let msg = LabMessage::new(1, LabNode::Quantum, LabNode::Shadow, LabPayloadKind::TrustContext, 1, 100);
        let result = LabFlow::receive_from_quantum(&msg, &valid_package(), 1000);
        assert!(!result.inbound_valid);
    }

    #[test]
    fn verdict_produced_regardless() {
        // Edhe me mesazh të pavlefshëm, gjykimi kryhet (vlefshmëria regjistrohet).
        let msg = LabMessage::new(1, LabNode::Light, LabNode::Shadow, LabPayloadKind::TrustContext, 1, 100);
        let result = LabFlow::receive_from_quantum(&msg, &valid_package(), 1000);
        assert_eq!(result.verification.ledger_entry.claim_id, 1);
    }

    #[test]
    fn response_to_light_is_valid() {
        let resp = LabFlow::build_response_to_light(1, 2, 200);
        assert!(resp.seal_ok());
        assert!(resp.is_valid_route());
        assert_eq!(resp.from, LabNode::Shadow);
        assert_eq!(resp.to, LabNode::Light);
    }
}
