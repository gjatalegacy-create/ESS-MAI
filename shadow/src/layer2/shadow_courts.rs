// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAYER2/SHADOW_COURTS.RS — Gjykatat e Verifikimit Suprem (Shadow)    ║
// ║  GJATA LEGACY™ — ESS-MAI Shadow                          (LAYER 2)    ║
// ║                                                                          ║
// ║  Shadow është AUTORITETI SUPREM I VERIFIKIMIT — jo thjesht vendimmarrës.║
// ║  Layer 2 e bën më të rreptë, më të saktë, më të pasur në ledger.        ║
// ║                                                                          ║
// ║  GJASHTË GJYKATA (Verification First):                                ║
// ║    • Adversarial Court    → sulmon çdo pretendim (avokat i djallit)    ║
// ║    • Evidence Court       → verifikon provat (rreptë për TRL 4)        ║
// ║    • TRL Court            → vendos nivelin përfundimtar                ║
// ║    • Cross-Domain Court   → kontrollon universalitetin                ║
// ║    • Negative Knowledge   → ruan dështimin si ASET                     ║
// ║    • Final Verdict        → orkestron verdiktin suprem                 ║
// ║                                                                          ║
// ║  Merr: EvidencePackage (TrlEvidence + TrustContext + PressureReport).   ║
// ║  Prodhon: VerificationState + LabVerdict + Ledger + NegativeKnowledge.  ║
// ║  KUFIRI: Shadow vendos — nuk ndërton besim (Light), nuk bën presion të  ║
// ║  hapur (Quantum). Presioni i tij është SOVRAN. Zero if/else.          ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::lab_contracts::evidence::{EvidencePackage, EvidenceKind};
use crate::lab_contracts::verdict::{VerificationState, LabVerdict};
use crate::lab_contracts::memory::{VerificationLedgerEntry, NegativeKnowledgeRecord};
use crate::lab_contracts::pressure::PressureReport;

// ─────────────────────────────────────────────────────────────────────────────
// KONSTANTET — pragjet sovrane (Shadow është më strikt se të tjerët)
// ─────────────────────────────────────────────────────────────────────────────

/// Pragu i besueshmërisë për pranim (Shadow kërkon të lartë).
pub const SHADOW_ACCEPT_CONFIDENCE: f32 = 0.70;
/// Pragu i presionit të mbijetuar për të kaluar Adversarial Court.
pub const SHADOW_SURVIVED_MIN: u8 = 2;
/// Numri minimal i kontrolleve cross-domain për universalitet.
pub const SHADOW_CROSS_DOMAIN_MIN: usize = 1;
/// Niveli TRL që kërkon provë fizike (jo vetëm arsyetim).
pub const SHADOW_PHYSICAL_EVIDENCE_TRL: u8 = 4;

// ─────────────────────────────────────────────────────────────────────────────
// 1. ADVERSARIAL COURT — sulmon pretendimin
// ─────────────────────────────────────────────────────────────────────────────

pub struct AdversarialCourt;

impl AdversarialCourt {
    /// challenge — a mbijeton pretendimi sulmin adversarial? Zero if.
    ///
    /// Mbijeton = presioni u aplikua AND ideja mbijetoi mjaftueshëm AND
    /// nuk ka mënyra dështimi të hapura.
    pub fn challenge(pressure: &PressureReport) -> bool {
        let pressure_applied = !pressure.pressure_path.is_empty();
        let survived_enough  = pressure.survived_level >= SHADOW_SURVIVED_MIN;
        let no_open_failures = pressure.failure_modes.is_empty();
        pressure_applied & survived_enough & no_open_failures
    }

    /// weakness_exposed — sa dobësi ekspozoi sulmi [0,1]. Zero if.
    pub fn weakness_exposed(pressure: &PressureReport) -> f32 {
        pressure.stress_index()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 2. EVIDENCE COURT — verifikon provat
// ─────────────────────────────────────────────────────────────────────────────

pub struct EvidenceCourt;

impl EvidenceCourt {
    /// validate — a janë provat e mjaftueshme për nivelin e pretenduar? Zero if.
    ///
    /// Për TRL < 4: arsyetim mjafton.
    /// Për TRL >= 4: KËRKOHET provë fizike (Documented/Operational + zinxhir).
    pub fn validate(pkg: &EvidencePackage) -> bool {
        let trl = pkg.trl_evidence.trl_level;
        let needs_physical = trl >= SHADOW_PHYSICAL_EVIDENCE_TRL;
        let has_physical   = pkg.has_physical_evidence();

        // Zero if — match tuple: nëse kërkon fizike, duhet ta ketë.
        match (needs_physical, has_physical) {
            (true, false) => false,  // TRL 4 pa provë fizike → refuzohet
            (true, true)  => true,   // TRL 4 me provë → OK
            (false, _)    => true,   // TRL < 4 → arsyetim mjafton
        }
    }

    /// evidence_strength — forca e provës [0,1]. Zero if.
    pub fn evidence_strength(pkg: &EvidencePackage) -> f32 {
        pkg.completeness()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 3. CROSS-DOMAIN COURT — kontrollon universalitetin
// ─────────────────────────────────────────────────────────────────────────────

pub struct CrossDomainCourt;

impl CrossDomainCourt {
    /// validate — a u testua ndër-domenesh mjaftueshëm? Zero if.
    pub fn validate(pkg: &EvidencePackage) -> bool {
        let count = pkg.cross_domain_count();
        let tested = pkg.trl_evidence.cross_domain_tested;
        let enough = count >= SHADOW_CROSS_DOMAIN_MIN;
        tested & enough
    }

    /// cross_domain_score — sa i fortë është kontrolli universal [0,1]. Zero if.
    pub fn cross_domain_score(pkg: &EvidencePackage) -> f32 {
        let count = pkg.cross_domain_count() as f32;
        (count / 3.0).clamp(0.0, 1.0)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 4. TRL COURT — vendos gjendjen e verifikimit
// ─────────────────────────────────────────────────────────────────────────────

pub struct TrlCourt;

impl TrlCourt {
    /// determine_state — vendos VerificationState nga gjykimet e gjykatave.
    ///
    /// Shkalla progresive (Verification First):
    ///   pa arsyetim → Unverified
    ///   arsyetim → Reasoned
    ///   + mbijetoi presion → PressureTested
    ///   + cross-domain → CrossDomainVerified
    ///   + provë fizike → EvidenceVerified
    ///   + operacional → OperationallyVerified
    /// Zero if — match mbi tuplin e shkallëve.
    pub fn determine_state(
        pkg: &EvidencePackage,
        adversarial_passed: bool,
        evidence_passed: bool,
        cross_domain_passed: bool,
    ) -> VerificationState {
        let has_reasoning = !pkg.reasoning.is_empty();
        let is_operational = (pkg.kind as u8) == (EvidenceKind::Operational as u8);
        let has_physical = pkg.has_physical_evidence();

        // Shkallët kumulative (secila kërkon të mëparshmen).
        // Match mbi tuplin e kushteve, nga më e larta te më e ulëta.
        match (has_reasoning, adversarial_passed, cross_domain_passed, evidence_passed, has_physical, is_operational) {
            (true, true, true, true, true, true)  => VerificationState::OperationallyVerified,
            (true, true, true, true, true, false) => VerificationState::EvidenceVerified,
            (true, true, true, true, false, _)    => VerificationState::CrossDomainVerified,
            (true, true, true, false, _, _)       => VerificationState::PressureTested,
            (true, true, false, _, _, _)          => VerificationState::PressureTested,
            (true, false, _, _, _, _)             => VerificationState::Reasoned,
            (false, _, _, _, _, _)                => VerificationState::Unverified,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 5. NEGATIVE KNOWLEDGE COURT — ruan dështimin si aset
// ─────────────────────────────────────────────────────────────────────────────

pub struct NegativeKnowledgeCourt;

impl NegativeKnowledgeCourt {
    /// record_failure — krijon NegativeKnowledgeRecord nga një dështim i verifikuar.
    /// Dështimi është ASET — ruhet me arsyen dhe rrugën e presionit. Zero if.
    pub fn record_failure(
        pkg: &EvidencePackage,
        failure_reason: String,
        recorded_at: u64,
    ) -> NegativeKnowledgeRecord {
        NegativeKnowledgeRecord::new(
            pkg.claim_id,
            failure_reason,
            pkg.pressure_report.pressure_path.clone(),
            true,  // verifikuar nga Shadow
            recorded_at,
        )
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 6. FINAL VERDICT ORCHESTRATION — verdikti suprem
// ─────────────────────────────────────────────────────────────────────────────

/// Rezultati i plotë i gjykimit suprem të Shadow.
#[derive(Debug, Clone)]
pub struct SupremeVerification {
    pub state:           VerificationState,
    pub verdict:         LabVerdict,
    pub ledger_entry:    VerificationLedgerEntry,
    pub negative_record: Option<NegativeKnowledgeRecord>,  // Some nëse u refuzua
}

pub struct FinalVerdictCourt;

impl FinalVerdictCourt {
    /// adjudicate — orkestron të 6 gjykatat dhe prodhon verdiktin suprem.
    ///
    /// Rrjedha (Verification First): çdo gjykatë kontribuon; pranimi kërkon
    /// adversarial + evidence + besueshmëri. Refuzimi prodhon negative knowledge.
    /// Zero if — match mbi rezultatin e gjykatave.
    pub fn adjudicate(pkg: &EvidencePackage, recorded_at: u64) -> SupremeVerification {
        // Të 6 gjykatat.
        let adversarial = AdversarialCourt::challenge(&pkg.pressure_report);
        let evidence    = EvidenceCourt::validate(pkg);
        let cross_domain = CrossDomainCourt::validate(pkg);
        let confidence_ok = pkg.trl_evidence.confidence >= SHADOW_ACCEPT_CONFIDENCE;

        // Gjendja e verifikimit.
        let state = TrlCourt::determine_state(pkg, adversarial, evidence, cross_domain);

        // Skoret për ledger.
        let pressure_score = 1.0 - AdversarialCourt::weakness_exposed(&pkg.pressure_report);
        let cross_score    = CrossDomainCourt::cross_domain_score(pkg);

        // Vendimi suprem: pranohet vetëm me adversarial + evidence + besueshmëri.
        let accepted = adversarial & evidence & confidence_ok;

        // Verdikti + negative record. Zero if — match.
        let (verdict, negative_record, final_bit) = match accepted {
            true => {
                let v = LabVerdict::Accepted {
                    trl: pkg.trl_evidence.trl_level,
                    state,
                    reason: "Mbijetoi gjykimin suprem: adversarial + evidence + besueshmëri".to_string(),
                };
                (v, None, 1u8)
            }
            false => {
                let weaknesses = pkg.pressure_report.failure_modes.clone();
                let reason = Self::rejection_reason(adversarial, evidence, confidence_ok);
                let v = LabVerdict::Rejected {
                    reason: reason.clone(),
                    weaknesses,
                };
                let nk = NegativeKnowledgeCourt::record_failure(pkg, reason, recorded_at);
                (v, Some(nk), 0u8)
            }
        };

        // Ledger entry — gjurma e plotë e verifikimit.
        let ledger_entry = VerificationLedgerEntry {
            claim_id:          pkg.claim_id,
            state,
            evidence_hash:     Self::evidence_hash(pkg),
            pressure_score:    pressure_score.clamp(0.0, 1.0),
            cross_domain_score: cross_score,
            final_verdict:     final_bit,
            recorded_at,
        };

        SupremeVerification {
            state,
            verdict,
            ledger_entry,
            negative_record,
        }
    }

    /// rejection_reason — arsyeja specifike e refuzimit (prioritet). Zero if — match.
    fn rejection_reason(adversarial: bool, evidence: bool, confidence: bool) -> String {
        match (adversarial, evidence, confidence) {
            (false, _, _) => "Refuzuar nga Adversarial Court: nuk mbijetoi presionin".to_string(),
            (true, false, _) => "Refuzuar nga Evidence Court: provë e pamjaftueshme".to_string(),
            (true, true, false) => "Refuzuar: besueshmëri nën pragun suprem".to_string(),
            (true, true, true) => "Pranuar".to_string(),  // s'arrihet (accepted=true)
        }
    }

    /// evidence_hash — SHA-256 kanonik i dëshmisë së marrë nga runtime-i.
    /// Nuk ndërton pseudo-hash nga etiketa tekstuale.
    fn evidence_hash(pkg: &EvidencePackage) -> String {
        pkg.trl_evidence.evidence_hash.iter()
            .map(|byte| format!("{byte:02x}"))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lab_contracts::trl::TrlEvidence;
    use crate::lab_contracts::trust::{TrustContext, TrustGuidance};
    use crate::lab_contracts::pressure::{PressureReport, PressureSource};

    fn strong_pressure() -> PressureReport {
        let mut p = PressureReport::none(1, PressureSource::QuantumNpro);
        p.pressure_path = "NReverse->NMirror".to_string();
        p.survived_level = 3;  // mbijetoi ekstremin
        p.contradiction_score = 0.1;
        p.weakness_score = 0.1;
        p
    }

    fn weak_pressure() -> PressureReport {
        let mut p = PressureReport::none(1, PressureSource::QuantumNpro);
        p.pressure_path = "NReverse".to_string();
        p.survived_level = 0;  // u thye
        p.failure_modes.push("breaks under load".to_string());
        p
    }

    fn pkg_with(trl: u8, conf: f32, pressure: PressureReport, physical: bool, cross: usize) -> EvidencePackage {
        let mut e = TrlEvidence::pending(1);
        e.trl_level = trl;
        e.trl_passed = true;
        e.confidence = conf;
        e.cross_domain_tested = cross > 0;
        e.timestamp = 100;
        let kind = match physical {
            true => EvidenceKind::Documented,
            false => EvidenceKind::Reasoned,
        };
        let chain = match physical {
            true => vec!["hash1".to_string()],
            false => Vec::new(),
        };
        let cross_checks: Vec<String> = (0..cross).map(|i| format!("domain{}", i)).collect();
        EvidencePackage {
            claim_id: 1,
            claim: "X works".to_string(),
            reasoning: "because Y".to_string(),
            trl_evidence: e,
            trust_context: TrustContext {
                trace_id: 1, domain: "d".to_string(),
                trust_score: 0.7, reliability_score: 0.6, verification_score: 0.5,
                history_refs: vec![1], guidance: TrustGuidance::Recommended,
            },
            pressure_report: pressure,
            evidence_chain: chain,
            cross_domain_checks: cross_checks,
            kind,
        }
    }

    #[test]
    fn adversarial_passes_strong_pressure() {
        assert!(AdversarialCourt::challenge(&strong_pressure()));
    }

    #[test]
    fn adversarial_rejects_weak_pressure() {
        assert!(!AdversarialCourt::challenge(&weak_pressure()));
    }

    #[test]
    fn evidence_court_trl4_needs_physical() {
        // TRL 4 pa provë fizike → refuzohet.
        let pkg_no_phys = pkg_with(4, 0.9, strong_pressure(), false, 1);
        assert!(!EvidenceCourt::validate(&pkg_no_phys));
        // TRL 4 me provë → OK.
        let pkg_phys = pkg_with(4, 0.9, strong_pressure(), true, 1);
        assert!(EvidenceCourt::validate(&pkg_phys));
    }

    #[test]
    fn evidence_court_trl3_reasoning_enough() {
        // TRL 3 pa provë fizike → OK (arsyetim mjafton).
        let pkg = pkg_with(3, 0.8, strong_pressure(), false, 1);
        assert!(EvidenceCourt::validate(&pkg));
    }

    #[test]
    fn accepted_when_all_pass() {
        let pkg = pkg_with(3, 0.8, strong_pressure(), false, 1);
        let result = FinalVerdictCourt::adjudicate(&pkg, 1000);
        assert!(result.verdict.is_accepted());
        assert_eq!(result.verdict.bit(), 1);
        assert!(result.negative_record.is_none());
        assert_eq!(result.ledger_entry.final_verdict, 1);
    }

    #[test]
    fn rejected_produces_negative_knowledge() {
        // Presion i dobët → adversarial dështon → refuzohet + negative record.
        let pkg = pkg_with(3, 0.8, weak_pressure(), false, 1);
        let result = FinalVerdictCourt::adjudicate(&pkg, 1000);
        assert!(!result.verdict.is_accepted());
        assert_eq!(result.verdict.bit(), 0);
        assert!(result.negative_record.is_some());
        // Negative record është verifikuar nga Shadow.
        let nk = result.negative_record.unwrap();
        assert!(nk.verified_by_shadow);
        assert_eq!(nk.claim_id, 1);
    }

    #[test]
    fn low_confidence_rejected() {
        // Besueshmëri nën prag → refuzohet edhe me presion të fortë.
        let pkg = pkg_with(3, 0.5, strong_pressure(), false, 1);
        let result = FinalVerdictCourt::adjudicate(&pkg, 1000);
        assert!(!result.verdict.is_accepted());
    }

    #[test]
    fn ledger_always_recorded() {
        // Edhe sukses, edhe dështim → ledger gjithmonë shkruhet.
        let pkg_ok = pkg_with(3, 0.8, strong_pressure(), false, 1);
        let r_ok = FinalVerdictCourt::adjudicate(&pkg_ok, 1000);
        assert_eq!(r_ok.ledger_entry.claim_id, 1);

        let pkg_fail = pkg_with(3, 0.8, weak_pressure(), false, 1);
        let r_fail = FinalVerdictCourt::adjudicate(&pkg_fail, 1000);
        assert_eq!(r_fail.ledger_entry.claim_id, 1);
        assert_eq!(r_fail.ledger_entry.final_verdict, 0);
    }

    #[test]
    fn verification_state_progressive() {
        // Dosje e plotë e fortë → gjendje e lartë.
        let pkg = pkg_with(3, 0.8, strong_pressure(), false, 1);
        let result = FinalVerdictCourt::adjudicate(&pkg, 1000);
        // Të paktën PressureTested (mbijetoi adversarial).
        assert!(result.state.at_least(VerificationState::Reasoned));
    }
}
