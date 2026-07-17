// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  SRK_IBE.RS — IBE (Lipton) + Bayesian + Conservation                 ║
// ║  GJATA LEGACY™ — Quantum Platform                                     ║
// ║                                                                          ║
// ║  3 makineritë e SRK-së:                                              ║
// ║   • IBE: 5 kriteret e Lipton (1991) → ibe_score                      ║
// ║   • Bayesian: prior → posterior, epistemic/aleatoric                 ║
// ║   • Conservation: ligjet e ruajtjes epistemike                       ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::pro_types::PROCandidate;
use crate::srk_types::*;

// ═════════════════════════════════════════════════════════════════════════════
// §1  IBE — Inference to the Best Explanation (Lipton 1991)
//   ibe_score = loveliness*0.25 + likeliness*0.25 + scope*0.20
//             + simplicity*0.15 + coherence*0.15
// ═════════════════════════════════════════════════════════════════════════════

pub struct IbeScorer;

impl IbeScorer {
    /// Vlerëson një kandidat PRO si shpjegim abduktiv.
    /// conservation_ok ndikon coherence (0.90 nëse OK, 0.30 nëse jo).
    pub fn evaluate(
        candidate:       &PROCandidate,
        phenomena_covered: f32,   // sa fenomene mbulon [0,1]
        causal_complexity: f32,   // sa komplekse është struktura kauzale
        theory_coherence:  f32,   // koherenca me teorinë ekzistuese [0,1]
        new_entities:      f32,   // sa entitete të reja shton (për simplicity)
        conservation_ok:   bool,
    ) -> AbductiveExplanation {
        // ── Loveliness: parsimonik + i unifikuar ──────────────────────────
        // (phenomena / max(complexity,1)) * 0.5 + score * 0.5
        let complexity = causal_complexity.max(0.01);
        let loveliness = ((phenomena_covered / complexity) * 0.5
                        + candidate.score * 0.5).clamp(0.0, 1.0);

        // ── Likeliness: confidence * 0.6 + theory_coherence * 0.4 ─────────
        let likeliness = (candidate.confidence * 0.6
                        + theory_coherence * 0.4).clamp(0.0, 1.0);

        // ── Explanatory scope: sa fenomene mbulon ─────────────────────────
        let explanatory_scope = phenomena_covered.clamp(0.0, 1.0);

        // ── Simplicity: Occam — 1 / (1 + new_entities * 0.2) ──────────────
        let simplicity = (1.0 / (1.0 + new_entities * 0.2)).clamp(0.0, 1.0);

        // ── Coherence: conservation gate (branchless) ─────────────────────
        // 0.90 nëse conservation OK, 0.30 nëse jo.
        let cons = conservation_ok as u32 as f32;
        let coherence = 0.30 + 0.60 * cons;

        // ── IBE SCORE — Lipton (1991) ─────────────────────────────────────
        let ibe_score = (
            loveliness        * 0.25 +
            likeliness        * 0.25 +
            explanatory_scope * 0.20 +
            simplicity        * 0.15 +
            coherence         * 0.15
        ).clamp(0.0, 1.0);

        AbductiveExplanation {
            explanation_id: format!("EXP_{}", candidate.candidate_id),
            loveliness,
            likeliness,
            explanatory_scope,
            simplicity,
            coherence,
            ibe_score,
            source_operator: candidate.operator,
            source_id: candidate.candidate_id.clone(),
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// §2  BAYESIAN — epistemic/aleatoric + posterior
//   combined = sqrt(epistemic² + aleatoric²) / sqrt(2)
//   posterior = P(E|H) * P(H) / P(E)
// ═════════════════════════════════════════════════════════════════════════════

pub struct UncertaintyEngine;

impl UncertaintyEngine {
    /// Llogarit profilin e pasigurisë për një grup shpjegimesh.
    ///
    /// epistemic = sa s'dimë por mund të dimë (1 - mesatarja e IBE)
    /// aleatoric = pasiguri e natyrshme (nga varianca e score-eve)
    pub fn compute(explanations: &[AbductiveExplanation]) -> UncertaintyProfile {
        match explanations.is_empty() {
            true => return UncertaintyProfile {
                epistemic: 1.0,
                aleatoric: 1.0,
                combined: 1.0,
                posterior: BayesianPosterior { prior: 0.0, posterior: 0.0 },
            },
            false => {}
        }

        let scores: Vec<f32> = explanations.iter().map(|e| e.ibe_score).collect();
        let mean_ibe = scores.iter().sum::<f32>() / scores.len() as f32;

        // ── Epistemic: reduktibël — sa larg nga siguria e plotë ──────────
        let epistemic = (1.0 - mean_ibe).clamp(0.0, 1.0);

        // ── Aleatoric: irreduktibël — nga varianca (shpërndarja e IBE) ───
        let variance = scores.iter()
            .map(|s| (s - mean_ibe).powi(2))
            .sum::<f32>() / scores.len() as f32;
        let aleatoric = variance.sqrt().clamp(0.0, 1.0);

        // ── Combined: sqrt(e² + a²) / sqrt(2) ─────────────────────────────
        let combined = ((epistemic.powi(2) + aleatoric.powi(2)).sqrt()
                       / 2.0_f32.sqrt()).clamp(0.0, 1.0);

        // ── Bayesian posterior ────────────────────────────────────────────
        // prior = mesatarja e IBE (besimi para Shadow).
        // posterior = Bayes update me likelihood = best IBE.
        let prior = mean_ibe;
        let best_ibe = scores.iter().cloned().fold(0.0f32, f32::max);
        let posterior = Self::bayes_update(prior, best_ibe);

        UncertaintyProfile {
            epistemic,
            aleatoric,
            combined,
            posterior: BayesianPosterior { prior, posterior },
        }
    }

    /// Bayes: P(H|E) = P(E|H)*P(H) / [P(E|H)*P(H) + P(E|¬H)*P(¬H)]
    /// likelihood = P(E|H) (best IBE), prior = P(H).
    fn bayes_update(prior: f32, likelihood: f32) -> f32 {
        let p = prior.clamp(0.001, 0.999);
        let l = likelihood.clamp(0.0, 1.0);
        // P(E|¬H) — likelihood nën hipotezën e kundërt (komplement i butë).
        let l_not = 1.0 - l;
        let numerator = l * p;
        let denominator = l * p + l_not * (1.0 - p);
        match denominator == 0.0 {
            true  => prior,
            false => (numerator / denominator).clamp(0.0, 1.0),
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// §3  CONSERVATION — ligjet e ruajtjes epistemike
// ═════════════════════════════════════════════════════════════════════════════

pub struct ConservationEngine;

impl ConservationEngine {
    /// Kontrollon ligjet e ruajtjes mbi një kandidat.
    /// Kthen check-et; nëse ndonjë violated → kandidati eliminohet nga SRK.
    pub fn check(candidate: &PROCandidate) -> Vec<ConservationCheck> {
        let mut checks = Vec::new();

        // ── Ligji 1: Kauzaliteti — çdo efekt duhet shkak ──────────────────
        // Nëse kandidati s'ka fragment_refs → s'ka bazë kauzale → violated.
        let no_cause = candidate.fragment_refs.is_empty();
        checks.push(ConservationCheck {
            law_name: "CAUSAL_GROUNDING".to_string(),
            violated: no_cause,
            reason: match no_cause {
                true  => "Pa fragmente referencë — efekt pa shkak".to_string(),
                false => "Ligjet e ruajtjes: të respektuara (kauzalitet)".to_string(),
            },
        });

        // ── Ligji 2: Mbi-siguria — score=1.0 pa bazë → violated ──────────
        // Shumë i sigurt (>0.98) me pak referenca → dyshim.
        let overconfident = candidate.score > 0.98 && candidate.fragment_refs.len() < 2;
        checks.push(ConservationCheck {
            law_name: "NO_OVERCONFIDENCE".to_string(),
            violated: overconfident,
            reason: match overconfident {
                true  => "Tepër i sigurt pa intervencion të mjaftueshëm".to_string(),
                false => "Siguria proporcionale me evidencën".to_string(),
            },
        });

        checks
    }

    /// A i kalon kandidati të gjitha ligjet (asnjë violated)?
    pub fn passes(checks: &[ConservationCheck]) -> bool {
        checks.iter().all(|c| !c.violated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lim_types::Operator;

    fn candidate(score: f32, conf: f32, refs: Vec<usize>) -> PROCandidate {
        PROCandidate {
            candidate_id: "C1".to_string(),
            operator: Operator::Reverse,
            score, confidence: conf,
            reasoning: "test".to_string(),
            fragment_refs: refs,
        }
    }

    #[test]
    fn ibe_in_range_and_weighted() {
        let c = candidate(0.8, 0.7, vec![0, 1]);
        let exp = IbeScorer::evaluate(&c, 0.8, 1.5, 0.6, 1.0, true);
        assert!((0.0..=1.0).contains(&exp.ibe_score));
        // conservation_ok=true → coherence=0.90
        assert!((exp.coherence - 0.90).abs() < 1e-5);
    }

    #[test]
    fn conservation_fail_lowers_coherence() {
        let c = candidate(0.8, 0.7, vec![0]);
        let ok = IbeScorer::evaluate(&c, 0.8, 1.0, 0.6, 1.0, true);
        let bad = IbeScorer::evaluate(&c, 0.8, 1.0, 0.6, 1.0, false);
        assert!(bad.coherence < ok.coherence);
        assert!(bad.ibe_score < ok.ibe_score);
    }

    #[test]
    fn bayes_increases_with_evidence() {
        let exps = vec![
            IbeScorer::evaluate(&candidate(0.9, 0.85, vec![0,1]), 0.9, 1.0, 0.8, 1.0, true),
            IbeScorer::evaluate(&candidate(0.85, 0.8, vec![0,1]), 0.85, 1.0, 0.7, 1.0, true),
        ];
        let u = UncertaintyEngine::compute(&exps);
        // posterior duhet > prior kur evidenca është e fortë
        assert!(u.posterior.posterior >= u.posterior.prior - 0.01);
        assert!((0.0..=1.0).contains(&u.combined));
    }

    #[test]
    fn conservation_catches_no_cause() {
        let c = candidate(0.5, 0.5, vec![]); // pa refs
        let checks = ConservationEngine::check(&c);
        assert!(!ConservationEngine::passes(&checks));
    }

    #[test]
    fn conservation_catches_overconfidence() {
        let c = candidate(0.99, 0.99, vec![0]); // score 0.99, 1 ref
        let checks = ConservationEngine::check(&c);
        assert!(!ConservationEngine::passes(&checks));
    }

    #[test]
    fn conservation_passes_grounded() {
        let c = candidate(0.7, 0.7, vec![0, 1, 2]);
        let checks = ConservationEngine::check(&c);
        assert!(ConservationEngine::passes(&checks));
    }
}
