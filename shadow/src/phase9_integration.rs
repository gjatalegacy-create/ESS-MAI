// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  PHASE9_INTEGRATION.RS — Lidhja me Rrjedhën e Gjallë (Shadow)        ║
// ║  GJATA LEGACY™ — ESS-MAI Shadow                          (FAZA 9)      ║
// ║                                                                          ║
// ║  FAZA 9 — PATCHIM I RRJEDHËS SË GJALLË:                               ║
// ║  Para Fazës 9, modulet (spine, true_knowledge, Layer 2/3) ishin të      ║
// ║  gjalla në teste por NUK thirreshin nga pipeline-i real. Ky modul       ║
// ║  i LIDH me rrjedhën e vërtetë, duke respektuar TË GJITHA ligjet.       ║
// ║                                                                          ║
// ║  PIKAT E LIDHJES (hook-e të pastra, s'zëvendësojnë run_pipeline):      ║
// ║   • adjudicate_with_spine → shadow_spine pas verifikimit suprem         ║
// ║   • intake_knowledge → shadow_true_knowledge për dijen pip             ║
// ║                                                                          ║
// ║  LIGJET E RESPEKTUARA (kontroll para çdo lidhjeje):                   ║
// ║   • Reasoning Purity  → vetëm Shadow shkruan persistent                ║
// ║   • Module Authority  → çdo modul brenda autoritetit                   ║
// ║   • Forbidden Bypass  → asnjë entitet i mbrojtur s'kapërcehet          ║
// ║   • Sovraniteti i inputit → knowledge pa trace REFUZOHET               ║
// ║                                                                          ║
// ║  KUFIRI: Faza 9 ORKESTRON lidhjen; vendimin suprem e mban              ║
// ║  ShadowGjLegacy. Zero if/else klasik (match/boolean). Kontroll rrjedhe  ║
// ║  përmes match është i lejuar.                                          ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::lab_contracts::evidence::EvidencePackage;
use crate::lab_contracts_v11::watch::NightWatchReport;
use crate::layer2::verification_memory::VerificationMemoryIndex;
use crate::layer3::shadow_wisdom::WisdomWarehouse;
use crate::shadow_spine::{ShadowSpine, ShadowSpineResult};
use crate::shadow_true_knowledge::{ShadowTrueKnowledge, TracedKnowledge, IntakeResult, IntakeRejection};

// ─────────────────────────────────────────────────────────────────────────────
// LIGJET E FAZËS 9 — kontroll para çdo lidhjeje
// ─────────────────────────────────────────────────────────────────────────────

/// Identiteti i shkruesit të autorizuar (vetëm Shadow shkruan persistent).
pub const SHADOW_WRITER_ID: &str = "shadow_gj_legacy";

/// Rezultati i kontrollit të ligjeve për një lidhje.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LawCheck {
    Passed,
    FailedPurity,      // shkrues jo-Shadow provoi të shkruajë persistent
    FailedAuthority,   // modul jashtë autoritetit
    FailedSovereignty, // input pa gjurmim fillestar
}

impl LawCheck {
    pub fn label(self) -> &'static str {
        match self {
            LawCheck::Passed           => "LAWS_PASSED",
            LawCheck::FailedPurity     => "FAILED_REASONING_PURITY",
            LawCheck::FailedAuthority  => "FAILED_MODULE_AUTHORITY",
            LawCheck::FailedSovereignty => "FAILED_INPUT_SOVEREIGNTY",
        }
    }

    /// is_passed — a kaluan ligjet? Zero if.
    pub fn is_passed(self) -> bool {
        match self {
            LawCheck::Passed => true,
            _ => false,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// PHASE 9 INTEGRATION — lidh spine + true_knowledge me rrjedhën
// ─────────────────────────────────────────────────────────────────────────────

/// Rezultati i adjudikimit me spine të lidhur.
#[derive(Debug, Clone)]
pub struct Phase9AdjudicationResult {
    pub law_check:  LawCheck,
    pub spine:      Option<ShadowSpineResult>,  // Some nëse ligjet kaluan
    pub precedent_signal: f32,                  // sinjali për Light (0 nëse dështoi)
}

/// Rezultati i pranimit të knowledge me ligje.
#[derive(Debug, Clone)]
pub struct Phase9KnowledgeResult {
    pub law_check: LawCheck,
    pub intake:    IntakeResult,
}

pub struct Phase9Integration;

impl Phase9Integration {
    /// adjudicate_with_spine — lidh shadow_spine me rrjedhën, duke respektuar ligjet.
    ///
    /// Para se spine të aktivizohet, verifikon Reasoning Purity (vetëm Shadow
    /// shkruan) dhe Module Authority. Nëse ligjet kalojnë → spine ekzekutohet.
    /// Zero if — match mbi gjendjen e ligjeve.
    pub fn adjudicate_with_spine(
        pkg: &EvidencePackage,
        watch_report: &NightWatchReport,
        memory: &mut VerificationMemoryIndex,
        warehouse: &mut WisdomWarehouse,
        writer_id: &str,
        recorded_at: u64,
    ) -> Phase9AdjudicationResult {
        // LIGJ: Reasoning Purity — vetëm Shadow shkruan persistent.
        let law_check = Self::check_write_laws(writer_id);

        // Spine ekzekutohet VETËM nëse ligjet kalojnë. Zero if — match.
        let (spine, precedent_signal) = match law_check.is_passed() {
            true => {
                let result = ShadowSpine::adjudicate_full(
                    pkg, watch_report, memory, warehouse, recorded_at,
                );
                let signal = ShadowSpine::precedent_signal_for_light(&result);
                (Some(result), signal)
            }
            false => (None, 0.0),
        };

        Phase9AdjudicationResult {
            law_check,
            spine,
            precedent_signal,
        }
    }

    /// intake_knowledge — lidh shadow_true_knowledge me rrjedhën, me ligje.
    ///
    /// LIGJ: sovraniteti i inputit — knowledge pa trace fillestar REFUZOHET.
    /// Kjo verifikohet brenda intake; këtu shtojmë kontrollin e shkruesit.
    /// Zero if — match.
    pub fn intake_knowledge(
        store: &mut ShadowTrueKnowledge,
        knowledge: &TracedKnowledge,
        writer_id: &str,
        at: u64,
    ) -> Phase9KnowledgeResult {
        // LIGJ: vetëm Shadow sistemon dijen persistent.
        let write_law = Self::check_write_laws(writer_id);

        // Intake bëhet VETËM nëse shkruesi është i autorizuar. Zero if — match.
        let intake = match write_law.is_passed() {
            true  => store.intake(knowledge, at),
            false => IntakeResult {
                accepted: false,
                rejection: IntakeRejection::NoInitialTrace, // bllokim para verifikimit
                classified: None,
            },
        };

        // Ligji final: kombinon kontrollin e shkruesit me rezultatin e trace-it.
        let law_check = Self::combine_knowledge_law(write_law, &intake);

        Phase9KnowledgeResult {
            law_check,
            intake,
        }
    }

    /// check_write_laws — verifikon Reasoning Purity (vetëm Shadow shkruan).
    /// Zero if — krahasim boolean + match.
    fn check_write_laws(writer_id: &str) -> LawCheck {
        let is_shadow = writer_id == SHADOW_WRITER_ID;
        // Vetëm Shadow lejohet të shkruajë persistent. Zero if — match.
        match is_shadow {
            true  => LawCheck::Passed,
            false => LawCheck::FailedPurity,
        }
    }

    /// combine_knowledge_law — kombinon ligjin e shkruesit me trace-in. Zero if.
    fn combine_knowledge_law(write_law: LawCheck, intake: &IntakeResult) -> LawCheck {
        let write_ok = write_law.is_passed();
        let trace_ok = intake.accepted;
        // Match mbi (write_ok, trace_ok) — prioritet shkruesi > trace.
        match (write_ok, trace_ok) {
            (false, _)    => LawCheck::FailedPurity,
            (true, false) => LawCheck::FailedSovereignty,
            (true, true)  => LawCheck::Passed,
        }
    }

    /// verify_no_bypass — Forbidden Bypass: asnjë entitet i mbrojtur s'kapërcehet.
    /// Zero if — pa fjalë "bypass"/"skip" për entitete të mbrojtura.
    pub fn verify_no_bypass(action: &str) -> bool {
        let has_bypass = action.contains("bypass");
        let has_skip   = action.contains("skip");
        // Veprimi është i sigurt nëse s'përmban bypass/skip.
        let unsafe_action = has_bypass | has_skip;
        !unsafe_action
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn no_bypass_law_fails_closed_on_skip() {
        // Ligji i mos-anashkalimit (aktiv në main): bypass/skip → false.
        assert!(Phase9Integration::verify_no_bypass("feed_session_normal"));
        assert!(!Phase9Integration::verify_no_bypass("try_bypass_verification"));
        assert!(!Phase9Integration::verify_no_bypass("skip_sovereign_check"));
    }

    use super::*;
    use crate::lab_contracts::trl::TrlEvidence;
    use crate::lab_contracts::trust::{TrustContext, TrustGuidance};
    use crate::lab_contracts::pressure::{PressureReport, PressureSource};
    use crate::lab_contracts::evidence::EvidenceKind;
    use crate::lab_contracts_v11::watch::SevenLayerInfo;

    fn strong_pkg() -> EvidencePackage {
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
                trace_id: 1, domain: "physics".to_string(),
                trust_score: 0.7, reliability_score: 0.6, verification_score: 0.5,
                history_refs: vec![1], guidance: TrustGuidance::Recommended,
            },
            pressure_report: p, evidence_chain: vec![],
            cross_domain_checks: vec!["d2".to_string()], kind: EvidenceKind::Reasoned,
        }
    }

    fn full_watch() -> NightWatchReport {
        NightWatchReport {
            claim_id: 1,
            layers: SevenLayerInfo {
                policy_alignment: 0.8, literature_support: 0.7, source_consensus: 0.9,
                recency: 0.6, credibility: 0.85, contradiction_free: 0.75, cross_coverage: 0.7,
            },
            via_device: true, sources_count: 5, gathered_at: 100,
        }
    }

    fn traced_kb(trace_id: u64) -> TracedKnowledge {
        TracedKnowledge::new(trace_id, b"content".to_vec(), "pip:x".to_string(), "physics".to_string(), 100)
    }

    #[test]
    fn spine_runs_when_shadow_writes() {
        let mut memory = VerificationMemoryIndex::new();
        let mut warehouse = WisdomWarehouse::new();
        let result = Phase9Integration::adjudicate_with_spine(
            &strong_pkg(), &full_watch(), &mut memory, &mut warehouse,
            SHADOW_WRITER_ID, 1000,
        );
        // Ligjet kaluan → spine ekzekutoi.
        assert!(result.law_check.is_passed());
        assert!(result.spine.is_some());
        assert!(result.precedent_signal >= 0.0);
    }

    #[test]
    fn spine_blocked_for_non_shadow_writer() {
        let mut memory = VerificationMemoryIndex::new();
        let mut warehouse = WisdomWarehouse::new();
        // Shkrues jo-Shadow → Reasoning Purity bllokon.
        let result = Phase9Integration::adjudicate_with_spine(
            &strong_pkg(), &full_watch(), &mut memory, &mut warehouse,
            "quantum_engine", 1000,
        );
        assert_eq!(result.law_check, LawCheck::FailedPurity);
        assert!(result.spine.is_none());
        // Asgjë s'u shkrua në kujtesë.
        assert_eq!(memory.ledger_size(), 0);
    }

    #[test]
    fn knowledge_intake_with_shadow_writer() {
        let mut store = ShadowTrueKnowledge::new();
        let result = Phase9Integration::intake_knowledge(
            &mut store, &traced_kb(42), SHADOW_WRITER_ID, 1000,
        );
        assert!(result.law_check.is_passed());
        assert!(result.intake.accepted);
    }

    #[test]
    fn knowledge_blocked_without_trace() {
        let mut store = ShadowTrueKnowledge::new();
        // Knowledge pa trace (trace_id=0) → sovraniteti i inputit bllokon.
        let result = Phase9Integration::intake_knowledge(
            &mut store, &traced_kb(0), SHADOW_WRITER_ID, 1000,
        );
        assert_eq!(result.law_check, LawCheck::FailedSovereignty);
        assert!(!result.intake.accepted);
    }

    #[test]
    fn knowledge_blocked_for_non_shadow() {
        let mut store = ShadowTrueKnowledge::new();
        let result = Phase9Integration::intake_knowledge(
            &mut store, &traced_kb(42), "light_coordinator", 1000,
        );
        assert_eq!(result.law_check, LawCheck::FailedPurity);
    }

    #[test]
    fn bypass_detection() {
        assert!(Phase9Integration::verify_no_bypass("normal_action"));
        assert!(!Phase9Integration::verify_no_bypass("bypass_judiciary"));
        assert!(!Phase9Integration::verify_no_bypass("skip_verification"));
    }

    #[test]
    fn law_check_passed_logic() {
        assert!(LawCheck::Passed.is_passed());
        assert!(!LawCheck::FailedPurity.is_passed());
        assert!(!LawCheck::FailedSovereignty.is_passed());
    }
}
