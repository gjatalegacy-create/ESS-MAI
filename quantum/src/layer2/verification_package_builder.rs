// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAYER2/VERIFICATION_PACKAGE_BUILDER.RS — Ndërtuesi i Dosjes (Quantum) ║
// ║  GJATA LEGACY™ — ESS-MAI Quantum                          (LAYER 2)    ║
// ║                                                                          ║
// ║  Quantum nuk dërgon një SUPOZIM te Shadow — dërgon një DOSJE PROVE.     ║
// ║  Ky modul mbledh produktet e ndara në një EvidencePackage të vetëm:    ║
// ║                                                                          ║
// ║    • TrlEvidence    (matja — nga digital_lab/PIM)                      ║
// ║    • TrustContext   (besimi — nga Light)                               ║
// ║    • PressureReport (presioni — nga Pressure Engine/NPRO)              ║
// ║    → EvidencePackage (dosja e plotë për Shadow)                        ║
// ║                                                                          ║
// ║  KUFIRI: ky modul vetëm ASAMBLON — nuk gjykon (Shadow gjykon), nuk      ║
// ║  mat (digital_lab mat), nuk ndërton besim (Light ndërton). Mbledh      ║
// ║  gjuhën e përbashkët në një dosje. Zero if/else.                      ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::lab_contracts::trl::TrlEvidence;
use crate::lab_contracts::trust::TrustContext;
use crate::lab_contracts::pressure::PressureReport;
use crate::lab_contracts::evidence::{EvidencePackage, EvidenceKind};

// ─────────────────────────────────────────────────────────────────────────────
// PACKAGE BUILDER — asamblon EvidencePackage
// ─────────────────────────────────────────────────────────────────────────────

pub struct VerificationPackageBuilder;

impl VerificationPackageBuilder {
    /// build — mbledh matjen + besimin + presionin në një dosje prove.
    ///
    /// kind përcaktohet automatikisht nga prova: nëse ka zinxhir prove fizik
    /// → Documented; ndryshe → Reasoned. Zero if — match.
    pub fn build(
        claim_id: u64,
        claim: String,
        reasoning: String,
        trl_evidence: TrlEvidence,
        trust_context: TrustContext,
        pressure_report: PressureReport,
        evidence_chain: Vec<String>,
        cross_domain_checks: Vec<String>,
    ) -> EvidencePackage {
        // Lloji i provës: a ka zinxhir prove fizik?
        let has_physical = !evidence_chain.is_empty();
        let kind = match has_physical {
            true  => EvidenceKind::Documented,
            false => EvidenceKind::Reasoned,
        };

        EvidencePackage {
            claim_id,
            claim,
            reasoning,
            trl_evidence,
            trust_context,
            pressure_report,
            evidence_chain,
            cross_domain_checks,
            kind,
        }
    }

    /// build_reasoned — dosje vetëm me arsyetim (pa provë fizike, TRL 1-3).
    /// Zero if.
    pub fn build_reasoned(
        claim_id: u64,
        claim: String,
        reasoning: String,
        trl_evidence: TrlEvidence,
        trust_context: TrustContext,
        pressure_report: PressureReport,
        cross_domain_checks: Vec<String>,
    ) -> EvidencePackage {
        Self::build(
            claim_id, claim, reasoning, trl_evidence, trust_context,
            pressure_report, Vec::new(), cross_domain_checks,
        )
    }

    /// is_ready_for_shadow — a është dosja gati për gjykim suprem? Zero if.
    ///
    /// Gati = ka pretendim + arsyetim + TRL jo-pending + presion i aplikuar.
    /// (Shadow vendos finalisht; ky është vetëm kontroll plotësie minimale.)
    /// Presioni u aplikua nëse pressure_path s'është bosh.
    pub fn is_ready_for_shadow(pkg: &EvidencePackage) -> bool {
        let has_claim     = !pkg.claim.is_empty();
        let has_reasoning = !pkg.reasoning.is_empty();
        let trl_ready     = !pkg.trl_evidence.is_pending();
        let pressure_done = !pkg.pressure_report.pressure_path.is_empty();
        has_claim & has_reasoning & trl_ready & pressure_done
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lab_contracts::pressure::PressureSource;
    use crate::lab_contracts::trust::TrustGuidance;

    fn ready_trl() -> TrlEvidence {
        let mut e = TrlEvidence::pending(1);
        e.trl_level = 3;
        e.trl_passed = true;
        e.confidence = 0.8;
        e.timestamp = 100;
        e
    }

    fn sample_trust() -> TrustContext {
        TrustContext {
            trace_id: 1, domain: "physics".to_string(),
            trust_score: 0.7, reliability_score: 0.6, verification_score: 0.5,
            history_refs: vec![1], guidance: TrustGuidance::Recommended,
        }
    }

    fn sample_pressure() -> PressureReport {
        let mut p = PressureReport::none(1, PressureSource::QuantumNpro);
        p.pressure_path = "NReverse->NMirror".to_string();
        p
    }

    #[test]
    fn build_with_evidence_is_documented() {
        let pkg = VerificationPackageBuilder::build(
            1, "X works".to_string(), "because Y".to_string(),
            ready_trl(), sample_trust(), sample_pressure(),
            vec!["hash1".to_string()], vec!["domain2".to_string()],
        );
        assert_eq!(pkg.kind, EvidenceKind::Documented);
        assert_eq!(pkg.claim_id, 1);
        assert!(pkg.has_physical_evidence());
    }

    #[test]
    fn build_reasoned_is_reasoned() {
        let pkg = VerificationPackageBuilder::build_reasoned(
            1, "X".to_string(), "Y".to_string(),
            ready_trl(), sample_trust(), sample_pressure(),
            vec!["domain2".to_string()],
        );
        assert_eq!(pkg.kind, EvidenceKind::Reasoned);
        assert!(!pkg.has_physical_evidence());
    }

    #[test]
    fn completeness_reflects_content() {
        let pkg = VerificationPackageBuilder::build(
            1, "X".to_string(), "Y".to_string(),
            ready_trl(), sample_trust(), sample_pressure(),
            vec!["hash".to_string()], vec!["d2".to_string()],
        );
        // Të 5 komponentët → completeness 1.0.
        assert!((pkg.completeness() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn cross_domain_preserved() {
        let pkg = VerificationPackageBuilder::build_reasoned(
            1, "X".to_string(), "Y".to_string(),
            ready_trl(), sample_trust(), sample_pressure(),
            vec!["d2".to_string(), "d3".to_string()],
        );
        assert_eq!(pkg.cross_domain_count(), 2);
    }
}
