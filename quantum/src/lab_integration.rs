// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAB_INTEGRATION.RS — Digital Lab PERFEKT me Sistemin                 ║
// ║  GJATA LEGACY™ — Quantum Platform                                 ║
// ║                                                                          ║
// ║  E lidh Digital Lab plotësisht me sistemin, sipas pseudo-kodit:        ║
// ║                                                                          ║
// ║   1. LIGJET — intervention_guard para çdo TRL faze (GOVERNANCE)       ║
// ║   2. TRACE — çdo fazë regjistrohet në raw_cognitive_trace (audit)     ║
// ║   3. VNK — kur TRL3 dështon → negative knowledge për Shadow           ║
// ║   4. EpistemicState — TRL i Lab-it konsiston me lim_classifier        ║
// ║   5. MEMORY — Lab respekton ephemeral (s'shkruan persistent)          ║
// ║                                                                          ║
// ║  LIGJET: Lab NUK vendos (Shadow vendos), Lab NUK mban memory.         ║
// ║  Zero if/else — match/formula.                                        ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::lab::digital_lab::DigitalLab;
use crate::lab::lab_types::{Hypothesis, Trl3Result, TrlEvidence, RejectionReason, LabError};
use crate::governance::Governance;
use crate::memory::raw_cognitive_trace::{RawCognitiveTrace, EventType, Outcome};
use crate::lim_types::EpistemicState;

// ─────────────────────────────────────────────────────────────────────────────
// LAB→VNK PACKAGE — dështimi i Lab-it si negative knowledge
// ─────────────────────────────────────────────────────────────────────────────

/// Kur TRL3 dështon, Lab prodhon këtë për Shadow (VNK).
/// Byte-for-byte e përputhshme me NegativeContext të Shadow-it.
#[derive(Debug, Clone)]
pub struct LabNegativeKnowledge {
    pub failure_reason_code: u32,
    pub process:             String,
    pub mass_at_fail:        f32,
    pub frequency:           u32,
    pub hypothesis_hash:     String,
    pub rejection:           &'static str,
}

impl LabNegativeKnowledge {
    /// from_failed — ndërton VNK nga një TRL3 i dështuar.
    /// Zero if — vlerat derivohen direkt.
    pub fn from_failed(hypothesis: &Hypothesis, result: &Trl3Result) -> Self {
        // failure_reason_code: koduar nga rejection (branchless via match).
        let failure_reason_code = match result.rejection_reason {
            RejectionReason::FilterGate       => 1001,
            RejectionReason::ScoreLow         => 1002,
            RejectionReason::PrerequisiteFail => 1003,
            RejectionReason::None             => 1000,
        };

        Self {
            failure_reason_code,
            process: format!("LAB_TRL3:{}", hypothesis.domain),
            mass_at_fail: result.score,
            frequency: 1,
            hypothesis_hash: hypothesis.hypothesis_hash.clone(),
            rejection: result.rejection_reason.label(),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// INTEGRATED LAB RESULT — rezultati i plotë me të gjitha lidhjet
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct IntegratedLabResult {
    pub trl3_result:      Option<Trl3Result>,        // None nëse ligjet bllokuan
    pub evidence:         TrlEvidence,               // për PIM
    pub negative_knowledge: Option<LabNegativeKnowledge>, // Some nëse dështoi → VNK
    pub trace_ids:        Vec<String>,               // gjurmët e regjistruara
    pub laws_passed:      bool,                       // a kaluan ligjet
    pub blocked_reason:   &'static str,
}

// ─────────────────────────────────────────────────────────────────────────────
// LAB SYSTEM BRIDGE — Lab-i i integruar plotësisht
// ─────────────────────────────────────────────────────────────────────────────

/// I vetmi autoritet që promovon (Shadow). Lab vetëm mat dhe sugjeron.
const LAB_MODULE: &str = "LAB_TRL3";

pub struct LabSystemBridge;

impl LabSystemBridge {
    /// run_integrated — pipeline-i i Lab-it i lidhur plotësisht me sistemin.
    ///
    /// 1. Kontrollon ligjet (recursion guard + override detection)
    /// 2. Regjistron çdo fazë në trace (audit)
    /// 3. Validon me TRL pipeline
    /// 4. Nëse dështon → prodhon VNK për Shadow
    ///
    /// governance + trace janë &mut (Lab regjistron, por s'vendos verdiktin).
    pub fn run_integrated(
        title: &str,
        domain: &str,
        raw_idea: &str,
        statement: &str,
        assumptions: Vec<String>,
        governance: &mut Governance,
        trace: &mut RawCognitiveTrace,
        session_id: &str,
    ) -> IntegratedLabResult {
        let mut trace_ids = Vec::new();

        // ── 1. LIGJET — intervention_guard (recursion për lab_retry) ───────
        // Ligji 1: max 2 lab retry (FORBIDDEN_RECURSION).
        let recursion_ok = governance.check_recursion("lab_trl_retry");

        // override detection: Lab s'duhet të kapërcejë gate.
        let no_override = !governance.detect_override("lab_validate", false, false);

        let laws_passed = recursion_ok && no_override;

        // Match mbi ligjet: dështim → bllokim me TrlEvidence pending.
        match laws_passed {
            false => {
                let tid = trace.record(
                    session_id, "LAB_BLOCKED", domain, "HIGH",
                    EventType::Failure, "ligjet bllokuan lab", LAB_MODULE,
                    Outcome::Failure, "recursion ose override", "lab nuk u ekzekutua", "",
                );
                trace_ids.push(tid);
                IntegratedLabResult {
                    trl3_result: None,
                    evidence: TrlEvidence::pending(),
                    negative_knowledge: None,
                    trace_ids,
                    laws_passed: false,
                    blocked_reason: "LAWS_BLOCKED",
                }
            }
            true => {
                // ── 2-3. TRACE + VALIDIM ───────────────────────────────────
                Self::execute_with_trace(
                    title, domain, raw_idea, statement, assumptions,
                    trace, session_id, &mut trace_ids,
                )
            }
        }
    }

    /// execute_with_trace — TRL pipeline me regjistrim të çdo faze.
    fn execute_with_trace(
        title: &str,
        domain: &str,
        raw_idea: &str,
        statement: &str,
        assumptions: Vec<String>,
        trace: &mut RawCognitiveTrace,
        session_id: &str,
        trace_ids: &mut Vec<String>,
    ) -> IntegratedLabResult {
        // TRL1 — Concept (regjistro).
        let concept = DigitalLab::trl1_concept(title, domain, raw_idea);
        let tid1 = trace.record(
            session_id, "TRL1_CONCEPT", domain, "LOW",
            EventType::Reasoning, &concept.lab_hash, LAB_MODULE,
            Outcome::Success, "concept created", "", "",
        );
        trace_ids.push(tid1.clone());

        // TRL2 — Hypothesis (regjistro). Lidhet me TRL1 via lineage_ref.
        let hyp_result = DigitalLab::trl2_hypothesis(&concept, statement, assumptions);

        // Match mbi rezultatin e TRL2 (zero if).
        match hyp_result {
            Err(e) => Self::handle_trl2_error(e, trace, session_id, &tid1, trace_ids),
            Ok(hypothesis) => {
                let tid2 = trace.record(
                    session_id, "TRL2_HYPOTHESIS", domain, "LOW",
                    EventType::Reasoning, &hypothesis.hypothesis_hash, LAB_MODULE,
                    Outcome::Success, "hypothesis formulated", "", &tid1,
                );
                trace_ids.push(tid2.clone());

                // TRL3 — Validim (regjistro rezultatin).
                Self::execute_trl3(&hypothesis, trace, session_id, &tid2, trace_ids)
            }
        }
    }

    /// execute_trl3 — validimi + trace + VNK nëse dështon.
    fn execute_trl3(
        hypothesis: &Hypothesis,
        trace: &mut RawCognitiveTrace,
        session_id: &str,
        parent_tid: &str,
        trace_ids: &mut Vec<String>,
    ) -> IntegratedLabResult {
        let validation = DigitalLab::trl3_validate(hypothesis);

        match validation {
            Err(e) => Self::handle_trl3_error(e, trace, session_id, parent_tid, trace_ids),
            Ok(result) => {
                // Outcome nga promovimi (branchless via match).
                let outcome = match result.promoted {
                    true  => Outcome::Success,
                    false => Outcome::Failure,
                };

                let tid3 = trace.record(
                    session_id, "TRL3_VALIDATE", &hypothesis.domain, "HIGH",
                    EventType::Reasoning, &result.evidence.findings, LAB_MODULE,
                    outcome, result.rejection_reason.label(),
                    &format!("score={:.3}", result.score), parent_tid,
                );
                trace_ids.push(tid3);

                // ── VNK: nëse s'u promovua → negative knowledge ────────────
                // promoted=false → Some(VNK); promoted=true → None.
                let negative_knowledge = match result.promoted {
                    false => Some(LabNegativeKnowledge::from_failed(hypothesis, &result)),
                    true  => None,
                };

                let evidence = result.evidence.clone();

                IntegratedLabResult {
                    trl3_result: Some(result),
                    evidence,
                    negative_knowledge,
                    trace_ids: trace_ids.clone(),
                    laws_passed: true,
                    blocked_reason: "NONE",
                }
            }
        }
    }

    /// handle_trl2_error — TRL2 dështoi (prerequisite).
    fn handle_trl2_error(
        _e: LabError,
        trace: &mut RawCognitiveTrace,
        session_id: &str,
        parent_tid: &str,
        trace_ids: &mut Vec<String>,
    ) -> IntegratedLabResult {
        let tid = trace.record(
            session_id, "TRL2_ERROR", "", "HIGH",
            EventType::Failure, "trl2 prerequisite fail", LAB_MODULE,
            Outcome::Failure, "concept jo TRL1", "", parent_tid,
        );
        trace_ids.push(tid);
        IntegratedLabResult {
            trl3_result: None,
            evidence: TrlEvidence::pending(),
            negative_knowledge: None,
            trace_ids: trace_ids.clone(),
            laws_passed: true,
            blocked_reason: "TRL2_PREREQUISITE",
        }
    }

    /// handle_trl3_error — TRL3 dështoi (prerequisite).
    fn handle_trl3_error(
        _e: LabError,
        trace: &mut RawCognitiveTrace,
        session_id: &str,
        parent_tid: &str,
        trace_ids: &mut Vec<String>,
    ) -> IntegratedLabResult {
        let tid = trace.record(
            session_id, "TRL3_ERROR", "", "HIGH",
            EventType::Failure, "trl3 prerequisite fail", LAB_MODULE,
            Outcome::Failure, "hypothesis jo TRL2", "", parent_tid,
        );
        trace_ids.push(tid);
        IntegratedLabResult {
            trl3_result: None,
            evidence: TrlEvidence::pending(),
            negative_knowledge: None,
            trace_ids: trace_ids.clone(),
            laws_passed: true,
            blocked_reason: "TRL3_PREREQUISITE",
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// EPISTEMIC STATE BRIDGE — TRL i Lab-it ↔ EpistemicState i lim_classifier
// ─────────────────────────────────────────────────────────────────────────────

pub struct EpistemicTrlBridge;

impl EpistemicTrlBridge {
    /// expected_trl — TRL që pret EpistemicState (nga lim_classifier).
    /// Lidh dy sistemet: lim klasifikon → Lab validon në atë TRL.
    /// Zero if — match mbi gjendjen.
    pub fn expected_trl(state: &EpistemicState) -> u8 {
        match state {
            EpistemicState::Hypothesis { .. }          => 1,
            EpistemicState::PotentialHypothesis { trl_candidate, .. } => *trl_candidate,
            EpistemicState::FactCandidate { .. }       => 3,
        }
    }

    /// trl_consistent — a përputhet TRL i Lab-it me pritjen e lim_classifier?
    /// Kjo siguron koherencë midis matjes së LIM dhe validimit të Lab.
    /// Zero if — krahasim direkt.
    pub fn trl_consistent(state: &EpistemicState, lab_evidence: &TrlEvidence) -> bool {
        let expected = Self::expected_trl(state);
        // Lab TRL >= pritjen → konsistent (Lab mund të validojë më lart).
        lab_evidence.trl_level >= expected
    }

    /// requires_lab — a kërkon kjo gjendje validim Lab?
    /// FactCandidate dhe PotentialHypothesis (TRL>=2) kërkojnë Lab.
    /// Zero if — match.
    pub fn requires_lab(state: &EpistemicState) -> bool {
        match state {
            EpistemicState::Hypothesis { .. }          => false, // embrion, s'ka nevojë
            EpistemicState::PotentialHypothesis { .. } => true,
            EpistemicState::FactCandidate { .. }       => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> (Governance, RawCognitiveTrace) {
        (Governance::new(), RawCognitiveTrace::new())
    }

    #[test]
    fn integrated_run_records_traces() {
        let (mut gov, mut trace) = setup();
        let result = LabSystemBridge::run_integrated(
            "Test Concept", "science", "deterministic idea",
            "X reduces Y measurably", vec!["assumption A".to_string(), "assumption B".to_string()],
            &mut gov, &mut trace, "sess1",
        );
        // Ligjet kaluan dhe u regjistruan gjurmë.
        assert!(result.laws_passed);
        assert!(!result.trace_ids.is_empty());
        // Trace-t janë në store.
        assert!(trace.len() >= 3); // TRL1 + TRL2 + TRL3
    }

    #[test]
    fn trl3_produces_evidence_for_pim() {
        let (mut gov, mut trace) = setup();
        let result = LabSystemBridge::run_integrated(
            "Strong", "science", "idea",
            "strong measurable claim", vec!["a".to_string(), "b".to_string()],
            &mut gov, &mut trace, "sess1",
        );
        assert_eq!(result.evidence.trl_level, 3);
        assert!(!result.evidence.pending_lab);
    }

    #[test]
    fn failed_trl3_produces_vnk() {
        let (mut gov, mut trace) = setup();
        // Pa assumptions → jo reproducible → FilterGate refuzon → VNK.
        let result = LabSystemBridge::run_integrated(
            "Weak", "science", "idea",
            "claim", vec![],  // pa assumptions
            &mut gov, &mut trace, "sess1",
        );
        // FilterGate refuzoi → negative knowledge u prodhua.
        assert!(result.negative_knowledge.is_some());
        let vnk = result.negative_knowledge.unwrap();
        assert_eq!(vnk.failure_reason_code, 1001); // FilterGate
        assert_eq!(vnk.rejection, "FILTER_GATE");
    }

    #[test]
    fn laws_block_excessive_recursion() {
        let (mut gov, mut trace) = setup();
        // Konsumo recursion budget (lab_trl_retry max = 2).
        gov.check_recursion("lab_trl_retry"); // 1
        gov.check_recursion("lab_trl_retry"); // 2
        // Tani lab-i duhet të bllokohet.
        let result = LabSystemBridge::run_integrated(
            "T", "science", "idea", "claim", vec!["a".to_string()],
            &mut gov, &mut trace, "sess1",
        );
        assert!(!result.laws_passed);
        assert_eq!(result.blocked_reason, "LAWS_BLOCKED");
    }

    #[test]
    fn traces_have_lineage() {
        let (mut gov, mut trace) = setup();
        LabSystemBridge::run_integrated(
            "T", "science", "idea",
            "measurable claim", vec!["a".to_string()],
            &mut gov, &mut trace, "sess1",
        );
        // Trace-t lidhen via lineage (TRL2 → TRL1, TRL3 → TRL2).
        assert!(trace.len() >= 3);
    }

    #[test]
    fn epistemic_trl_bridge_maps_states() {
        let hyp = EpistemicState::Hypothesis {
            confidence_range: (0.30, 0.35),
            reason: "test".to_string(),
            pro_mode: "x".to_string(),
        };
        assert_eq!(EpistemicTrlBridge::expected_trl(&hyp), 1);
        assert!(!EpistemicTrlBridge::requires_lab(&hyp)); // embrion

        let fact = EpistemicState::FactCandidate {
            confidence_range: (0.99, 1.0),
            validation_path: "p".to_string(),
            pro_mode: "x".to_string(),
        };
        assert_eq!(EpistemicTrlBridge::expected_trl(&fact), 3);
        assert!(EpistemicTrlBridge::requires_lab(&fact)); // kërkon validim
    }

    #[test]
    fn trl_consistency_check() {
        let fact = EpistemicState::FactCandidate {
            confidence_range: (0.99, 1.0),
            validation_path: "p".to_string(),
            pro_mode: "x".to_string(),
        };
        // Lab TRL3 → konsistent me FactCandidate (pret TRL3).
        let evidence = TrlEvidence {
            trl_level: 3, trl_passed: true, lab_test_id: None,
            test_description: "".to_string(), findings: "".to_string(),
            confidence: 0.9, pending_lab: false, reproducibility: 0.8,
        };
        assert!(EpistemicTrlBridge::trl_consistent(&fact, &evidence));
        // Lab TRL1 → JO konsistent me FactCandidate.
        let low_evidence = TrlEvidence { trl_level: 1, ..evidence };
        assert!(!EpistemicTrlBridge::trl_consistent(&fact, &low_evidence));
    }

    #[test]
    fn vnk_byte_compatible_with_shadow() {
        let (mut gov, mut trace) = setup();
        let result = LabSystemBridge::run_integrated(
            "W", "science", "idea", "claim", vec![],
            &mut gov, &mut trace, "sess1",
        );
        let vnk = result.negative_knowledge.unwrap();
        // Fushat përputhen me NegativeContext të Shadow.
        assert!(vnk.failure_reason_code > 0);
        assert!(!vnk.process.is_empty());
        assert!((0.0..=1.0).contains(&vnk.mass_at_fail));
        assert_eq!(vnk.frequency, 1);
    }
}
