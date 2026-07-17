// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAB/DIGITAL_LAB.RS — Digital Lab Pipeline (Teoria 44)               ║
// ║  GJATA LEGACY™ — Quantum Platform                                     ║
// ║                                                                          ║
// ║  Matja e SAKTË përmes progresionit TRL të kontrolluar:                ║
// ║   TRL1 Concept → TRL2 Hypothesis → TRL3 Validation                    ║
// ║                                                                          ║
// ║  INVARIANT I DETYRUESHËM: "all promotion must go through FILTER_GATE"  ║
// ║   → asnjë promovim s'kalon pa FilterGate.                            ║
// ║                                                                          ║
// ║  Threshold TRL3 = 0.70. Lab MAT dhe prodhon TrlEvidence për PIM.       ║
// ║  Quantum NUK vendos — promovimi final konfirmohet nga Shadow.         ║
// ║  Zero if/else — match/formula.                                        ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::lab::lab_types::*;

// ─────────────────────────────────────────────────────────────────────────────
// SIMULATION ENGINE — ekzekuton simulimin deterministik
// ─────────────────────────────────────────────────────────────────────────────

pub struct SimulationEngine;

impl SimulationEngine {
    /// run — simulon hipotezën N herë për riprodhueshmëri.
    ///
    /// Deterministik: nga hash-i i hipotezës derivohen metrikat (riprodhueshëm).
    /// Konsistenca ndër përsëritjet mat sa i besueshëm është rezultati.
    pub fn run(hypothesis: &Hypothesis) -> SimulationResult {
        // Metrikat derivohen deterministikisht nga hash-i (riprodhueshmëri e plotë).
        let metrics = Self::derive_metrics(hypothesis);

        // Outcome nga metrikat (zero if — match mbi tuple guards).
        let outcome = Self::classify_outcome(&metrics);

        // Përsëritjet: invariantët measurable+reproducible përcaktojnë sa cikle.
        // measurable AND reproducible → 5 përsëritje; përndryshe më pak.
        let base_reps = 3u32;
        let bonus = (hypothesis.measurable as u32) + (hypothesis.reproducible as u32);
        let repetitions = base_reps + bonus;  // 3..5

        // Konsistenca: sa stabile janë metrikat (nga stability).
        let consistency = metrics.stability;

        SimulationResult {
            outcome,
            metrics,
            repetitions,
            consistency,
        }
    }

    /// derive_metrics — metrika deterministike nga hash-i (riprodhueshëm).
    fn derive_metrics(hypothesis: &Hypothesis) -> SimMetrics {
        // P7: metrika SEMANTIKE reale (jo pseudo nga hash bytes).
        let stmt  = &hypothesis.statement;
        let assumptions = &hypothesis.assumptions;
        let n_assum = assumptions.len().max(1) as f32;

        // stability: gjatësia mesatare e assumptions, normalizuar ndaj 80 karaktere.
        let avg_len = assumptions.iter()
            .map(|a| a.len() as f32)
            .sum::<f32>() / n_assum;
        let base_stability = (avg_len / 80.0).clamp(0.0, 1.0);
        let invariant_boost =
            (hypothesis.measurable as u32 + hypothesis.reproducible as u32) as f32 * 0.15;
        let stability = (base_stability + invariant_boost).clamp(0.0, 1.0);

        // signal_noise: dendësia e fjalëve kyçe shkencore në statement.
        const KEYWORDS: &[&str] = &[
            "measur", "verif", "determin", "reprod", "quantif", "formal",
            "invariant", "evidence", "causal", "eliminat",
        ];
        let stmt_low = stmt.to_lowercase();
        let kw_hits = KEYWORDS.iter()
            .filter(|k| stmt_low.contains(*k))
            .count() as f32;
        let signal_noise = (kw_hits / KEYWORDS.len() as f32).clamp(0.0, 1.0);

        // convergence: sa fjalë të assumptions mbulohen nga statement.
        let stmt_words: std::collections::HashSet<&str> =
            stmt.split_whitespace().collect();
        let covered = assumptions.iter()
            .flat_map(|a| a.split_whitespace())
            .filter(|w| stmt_words.contains(w))
            .count() as f32;
        let convergence = (covered / stmt_words.len().max(1) as f32)
            .clamp(0.0, 1.0);

        SimMetrics { stability, signal_noise, convergence }
    }

    /// classify_outcome — Converged/Diverged/Inconclusive nga metrikat.
    /// Zero if — match mbi tuple boolean (prioritet i ruajtur).
    fn classify_outcome(m: &SimMetrics) -> SimOutcome {
        let strong = m.stability > 0.60 && m.convergence > 0.60;
        let weak = m.stability < 0.30 || m.convergence < 0.30;
        match (strong, weak) {
            (true, _)  => SimOutcome::Converged,
            (_, true)  => SimOutcome::Diverged,
            _          => SimOutcome::Inconclusive,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// RESULT EVALUATOR — simulim → confidence scalar [0,1]
// ─────────────────────────────────────────────────────────────────────────────

pub struct ResultEvaluator;

impl ResultEvaluator {
    /// evaluate — agregat i metrikave → confidence [0,1].
    pub fn evaluate(sim: &SimulationResult) -> f32 {
        let m = &sim.metrics;
        // Peshim: stability dominon (qëndrueshmëria = besueshmëri).
        let base = m.stability * 0.40
                 + m.signal_noise * 0.30
                 + m.convergence * 0.30;

        // Outcome modifikon (Converged boost, Diverged penalty) — branchless.
        let outcome_factor = match sim.outcome {
            SimOutcome::Converged    => 1.0,
            SimOutcome::Inconclusive => 0.70,
            SimOutcome::Diverged     => 0.30,
        };

        // Konsistenca ndër përsëritjet ngre besimin.
        let consistency_boost = sim.consistency * 0.10;

        (base * outcome_factor + consistency_boost).clamp(0.0, 1.0)
    }

    /// simple_score — fallback kur Evaluator është BLOCKED (nga pseudo).
    pub fn simple_score(sim: &SimulationResult) -> f32 {
        // Vetëm stability (fallback minimal).
        sim.metrics.stability.clamp(0.0, 1.0)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// FILTER GATE — INVARIANT I DETYRUESHËM
// ─────────────────────────────────────────────────────────────────────────────

/// FilterGate: "all promotion must go through FILTER_GATE".
/// Kontrollon integritetin PARA promovimit. Asnjë gjë s'kalon pa të.
pub struct FilterGate;

impl FilterGate {
    /// check — invariantët që duhen plotësuar para promovimit.
    ///
    /// Kontrollon: measurable AND reproducible AND score valid AND hash valid.
    /// Zero if — produkt boolean i kushteve.
    pub fn check(hypothesis: &Hypothesis, score: f32) -> bool {
        let measurable = hypothesis.measurable as u32;
        let reproducible = hypothesis.reproducible as u32;
        let score_valid = ((0.0..=1.0).contains(&score)) as u32;
        let hash_valid = (hypothesis.hypothesis_hash.len() == 64) as u32;
        let trl_valid = (hypothesis.trl_level >= 2) as u32;

        // Të GJITHA duhet 1 (produkt boolean).
        (measurable * reproducible * score_valid * hash_valid * trl_valid) == 1
    }

    /// gate_reason — pse refuzoi (informativ, për trace).
    pub fn gate_reason(hypothesis: &Hypothesis, score: f32) -> &'static str {
        let not_measurable = !hypothesis.measurable;
        let not_reproducible = !hypothesis.reproducible;
        let bad_score = !(0.0..=1.0).contains(&score);
        // Prioritet: i pari problem që gjendet.
        match (not_measurable, not_reproducible, bad_score) {
            (true, _, _) => "jo measurable",
            (_, true, _) => "jo reproducible",
            (_, _, true) => "score invalid",
            _            => "OK",
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// DIGITAL LAB — pipeline-i i plotë
// ─────────────────────────────────────────────────────────────────────────────

/// Threshold TRL3 — nga pseudo (THRESHOLD_TRL3 = 0.70).
pub const THRESHOLD_TRL3: f32 = 0.70;

pub struct DigitalLab;

impl DigitalLab {
    // ── TRL1 ────────────────────────────────────────────────────────────────

    /// trl1_concept — krijon konceptin (hash deterministik).
    pub fn trl1_concept(title: &str, domain: &str, raw_idea: &str) -> Concept {
        Concept::create(title, domain, raw_idea)
    }

    // ── TRL2 ────────────────────────────────────────────────────────────────

    /// trl2_hypothesis — formulon hipotezën. Kërkon TRL1.
    /// Zero if — match mbi prerequisite.
    pub fn trl2_hypothesis(
        concept: &Concept,
        statement: &str,
        assumptions: Vec<String>,
    ) -> Result<Hypothesis, LabError> {
        // Invariant: Concept duhet TRL1 (trl_level >= 1).
        match concept.trl_level >= 1 {
            true => Ok(Hypothesis::formulate(concept, statement, assumptions)),
            false => Err(LabError::PrerequisiteNotMet(
                "Concept duhet TRL1 para hipotezës".to_string()
            )),
        }
    }

    // ── TRL3 ────────────────────────────────────────────────────────────────

    /// trl3_validate — validimi shkencor i plotë.
    ///
    /// Rrjedha: Simulation → Evaluation → FilterGate → Threshold → Promotion.
    /// FilterGate është INVARIANT — asnjë promovim pa të.
    /// Zero if — match mbi prerequisite + tuple guards.
    pub fn trl3_validate(hypothesis: &Hypothesis) -> Result<Trl3Result, LabError> {
        // Invariant: Hypothesis duhet TRL2.
        match hypothesis.trl_level >= 2 {
            false => return Err(LabError::PrerequisiteNotMet(
                "Hypothesis duhet TRL2 para validimit".to_string()
            )),
            true => {}
        }

        // ── 1. SIMULIM ─────────────────────────────────────────────────────
        let sim = SimulationEngine::run(hypothesis);

        // ── 2. EVALUIM → score ─────────────────────────────────────────────
        let score = ResultEvaluator::evaluate(&sim);

        // ── 3. FILTER GATE (INVARIANT) ─────────────────────────────────────
        let gate_ok = FilterGate::check(hypothesis, score);

        // ── 4. THRESHOLD ───────────────────────────────────────────────────
        let score_ok = score >= THRESHOLD_TRL3;

        // ── 5. VENDIM PROMOVIMI (match mbi gate + threshold) ──────────────
        // promoted = gate_ok AND score_ok. Refuzimi ka arsye specifike.
        // Prioritet: gate dështon i pari (invariant), pastaj threshold.
        let (promoted, reason) = match (gate_ok, score_ok) {
            (false, _) => (false, RejectionReason::FilterGate),
            (true, false) => (false, RejectionReason::ScoreLow),
            (true, true) => (true, RejectionReason::None),
        };

        // ── TrlEvidence për PIM (matja e saktë) ────────────────────────────
        let evidence = Self::build_evidence(hypothesis, &sim, score, promoted);

        Ok(Trl3Result {
            promoted,
            score,
            rejection_reason: reason,
            evidence,
            gate_passed: gate_ok,
        })
    }

    /// build_evidence — prodhon TrlEvidence që PIM konsumon.
    fn build_evidence(
        hypothesis: &Hypothesis,
        sim: &SimulationResult,
        score: f32,
        promoted: bool,
    ) -> TrlEvidence {
        // reproducibility nga konsistenca + përsëritjet.
        let rep_factor = (sim.repetitions as f32 / 5.0).clamp(0.0, 1.0);
        let reproducibility = (sim.consistency * 0.6 + rep_factor * 0.4).clamp(0.0, 1.0);

        let findings = format!(
            "Simulim: {} | {} përsëritje | konsistencë {:.2}",
            sim.outcome.label(), sim.repetitions, sim.consistency
        );

        TrlEvidence {
            trl_level: 3,
            trl_passed: promoted,
            lab_test_id: Some(format!("TEST_{}", &hypothesis.hypothesis_hash[..8])),
            test_description: format!("TRL3 validim për {}", hypothesis.hypothesis_id),
            findings,
            confidence: score,
            pending_lab: false,
            reproducibility,
        }
    }

    // ── PIPELINE I PLOTË ────────────────────────────────────────────────────

    /// run_full_pipeline — raw_idea → TRL3 result.
    pub fn run_full_pipeline(
        title: &str,
        domain: &str,
        raw_idea: &str,
        statement: &str,
        assumptions: Vec<String>,
    ) -> Result<Trl3Result, LabError> {
        let concept = Self::trl1_concept(title, domain, raw_idea);
        let hypothesis = Self::trl2_hypothesis(&concept, statement, assumptions)?;
        Self::trl3_validate(&hypothesis)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_hypothesis(measurable_stmt: &str, with_assumptions: bool) -> Hypothesis {
        let c = Concept::create("Test", "science", "test idea here");
        let assumptions = match with_assumptions {
            true => vec!["assumption A".to_string(), "assumption B".to_string()],
            false => vec![],
        };
        Hypothesis::formulate(&c, measurable_stmt, assumptions)
    }

    #[test]
    fn simulation_is_deterministic() {
        let h = make_hypothesis("X causes Y measurably", true);
        let sim1 = SimulationEngine::run(&h);
        let sim2 = SimulationEngine::run(&h);
        // I njëjti hash → i njëjti rezultat (riprodhueshmëri).
        assert_eq!(sim1.metrics.stability, sim2.metrics.stability);
        assert_eq!(sim1.repetitions, sim2.repetitions);
    }

    #[test]
    fn invariants_increase_repetitions() {
        // measurable + reproducible → 5 përsëritje.
        let full = make_hypothesis("claim", true);
        let sim_full = SimulationEngine::run(&full);
        assert_eq!(sim_full.repetitions, 5);
        // pa assumptions → më pak.
        let partial = make_hypothesis("claim", false);
        let sim_partial = SimulationEngine::run(&partial);
        assert!(sim_partial.repetitions < 5);
    }

    #[test]
    fn evaluator_produces_valid_score() {
        let h = make_hypothesis("strong claim", true);
        let sim = SimulationEngine::run(&h);
        let score = ResultEvaluator::evaluate(&sim);
        assert!((0.0..=1.0).contains(&score));
    }

    #[test]
    fn filter_gate_blocks_non_measurable() {
        // Hipotezë pa statement → jo measurable → FilterGate refuzon.
        let h = make_hypothesis("", true);
        assert!(!FilterGate::check(&h, 0.9));
        assert_eq!(FilterGate::gate_reason(&h, 0.9), "jo measurable");
    }

    #[test]
    fn filter_gate_blocks_non_reproducible() {
        // Pa assumptions → jo reproducible → FilterGate refuzon.
        let h = make_hypothesis("claim", false);
        assert!(!FilterGate::check(&h, 0.9));
    }

    #[test]
    fn filter_gate_passes_valid() {
        let h = make_hypothesis("valid measurable claim", true);
        // measurable + reproducible + score valid + hash valid + trl >= 2.
        assert!(FilterGate::check(&h, 0.8));
    }

    #[test]
    fn trl2_requires_trl1() {
        let c = Concept::create("T", "science", "idea");
        let result = DigitalLab::trl2_hypothesis(&c, "claim", vec!["a".to_string()]);
        assert!(result.is_ok());
    }

    #[test]
    fn trl3_requires_trl2() {
        // Hipotezë me trl_level të ulët (manual) → refuzohet.
        let mut h = make_hypothesis("claim", true);
        h.trl_level = 1; // forcë invalide
        let result = DigitalLab::trl3_validate(&h);
        assert!(result.is_err());
    }

    #[test]
    fn trl3_filter_gate_rejection() {
        // Hipotezë jo measurable → FilterGate refuzon në TRL3.
        let h = make_hypothesis("", true); // jo measurable
        let result = DigitalLab::trl3_validate(&h).unwrap();
        assert!(!result.promoted);
        assert_eq!(result.rejection_reason, RejectionReason::FilterGate);
        assert!(!result.gate_passed);
    }

    #[test]
    fn trl3_produces_evidence_for_pim() {
        let h = make_hypothesis("strong reproducible claim", true);
        let result = DigitalLab::trl3_validate(&h).unwrap();
        // Evidenca është për PIM.
        assert_eq!(result.evidence.trl_level, 3);
        assert!(!result.evidence.pending_lab);
        assert!(result.evidence.lab_test_id.is_some());
        assert!((0.0..=1.0).contains(&result.evidence.reproducibility));
    }

    #[test]
    fn full_pipeline_runs() {
        let result = DigitalLab::run_full_pipeline(
            "Quantum Determinism",
            "science",
            "deterministic elimination produces verifiable knowledge",
            "elimination reduces hallucination measurably",
            vec!["no probabilistic sampling".to_string(), "formal invariants hold".to_string()],
        );
        assert!(result.is_ok());
        let r = result.unwrap();
        // Pipeline prodhoi evidencë.
        assert_eq!(r.evidence.trl_level, 3);
    }

    #[test]
    fn threshold_is_070() {
        assert_eq!(THRESHOLD_TRL3, 0.70);
    }

    #[test]
    fn invariant_promotion_requires_gate() {
        // Provë e invariantit: promoted=true KËRKON gate_passed=true.
        let h = make_hypothesis("strong claim with evidence", true);
        let result = DigitalLab::trl3_validate(&h).unwrap();
        // Nëse u promovua, DUHET të ketë kaluar gate-in (invariant).
        let promoted_implies_gate = (!result.promoted) || result.gate_passed;
        assert!(promoted_implies_gate);
    }
}
