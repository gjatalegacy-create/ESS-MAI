// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  SRK.RS — SCIENTIFIC REASONING KERNEL — ORCHESTRATORI                 ║
// ║  GJATA LEGACY™ — Quantum Platform                                     ║
// ║                                                                          ║
// ║  POZICIONI: MES PRO dhe PIM.                                          ║
// ║   PRO → kandidatë                                                     ║
// ║   SRK → filtro (conservation) → IBE score → Bayesian →               ║
// ║         ReasoningPackage                                              ║
// ║   PIM → paketon vetëm ç'kaloi SRK                                     ║
// ║                                                                          ║
// ║  SRK NUK vendos (verdict=None). NUK shkruan memory.                  ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::pro_types::{ProResult, PROCandidate};
use crate::srk_types::*;
use crate::srk_ibe::{IbeScorer, UncertaintyEngine, ConservationEngine};

pub struct ScientificReasoningKernel {
    session_id: String,
    territory:  String,
}

impl ScientificReasoningKernel {
    pub fn new(session_id: &str, territory: &str) -> Self {
        Self {
            session_id: session_id.to_string(),
            territory: territory.to_string(),
        }
    }

    /// PROCESS — merr ProResult → prodhon ReasoningPackage.
    ///
    /// Hapat (besnik ndaj srk_core.pseudo):
    ///   1. filter_conservation_violators (eliminon ata që shkelin ligjet)
    ///   2. IBE evaluate për secilin që mbijetoi
    ///   3. rendit sipas ibe_score
    ///   4. uncertainty (Bayesian) mbi shpjegimet
    ///   5. ReasoningPackage me shadow_note
    pub fn process(&self, pro_result: &ProResult) -> ReasoningPackage {
        let candidates = &pro_result.candidates;

        // ── 1. CONSERVATION FILTER ─────────────────────────────────────────
        // Ndaj kandidatët: ata që kalojnë vs ata që shkelin ligjet.
        let mut surviving: Vec<&PROCandidate> = Vec::new();
        let mut all_checks: Vec<ConservationCheck> = Vec::new();
        let mut eliminated = 0usize;

        for c in candidates {
            let checks = ConservationEngine::check(c);
            let passes = ConservationEngine::passes(&checks);
            // Mblidh check-et (për proof chain).
            for ch in &checks {
                all_checks.push(ch.clone());
            }
            // push kushtëzuar përmes match (jo if/else).
            match passes {
                true => surviving.push(c),
                false => eliminated += 1,
            }
        }

        // ── 2. IBE për secilin që mbijetoi ─────────────────────────────────
        let mut explanations: Vec<AbductiveExplanation> = Vec::new();
        for c in &surviving {
            // Parametrat e IBE nga vetitë e kandidatit + konteksti.
            let phenomena_covered = (c.fragment_refs.len() as f32 / 5.0).clamp(0.1, 1.0);
            let causal_complexity = 1.0 + c.fragment_refs.len() as f32 * 0.3;
            let theory_coherence = c.confidence;  // proxy
            let new_entities = (c.fragment_refs.len() as f32 * 0.5).max(1.0);
            // conservation_ok për këtë kandidat (e rikontrollojmë lokalisht).
            let conservation_ok = ConservationEngine::passes(&ConservationEngine::check(c));

            let exp = IbeScorer::evaluate(
                c, phenomena_covered, causal_complexity,
                theory_coherence, new_entities, conservation_ok,
            );
            explanations.push(exp);
        }

        // ── 3. Rendit sipas ibe_score (zbritës) ────────────────────────────
        explanations.sort_by(|a, b| {
            b.ibe_score.partial_cmp(&a.ibe_score).unwrap_or(std::cmp::Ordering::Equal)
        });

        let best_explanation = explanations.first().cloned();

        // ── 4. UNCERTAINTY (Bayesian) ──────────────────────────────────────
        let uncertainty = UncertaintyEngine::compute(&explanations);

        // ── 5. Evidence chain (proof-carrying) ─────────────────────────────
        let evidence_chain = Self::build_evidence(&explanations, pro_result);

        // ── 6. Shadow note (orientim, JO vendim) ───────────────────────────
        let shadow_note = Self::build_shadow_note(&best_explanation, &uncertainty, eliminated);

        ReasoningPackage {
            package_id: format!("SRK_{}_{}", self.territory, self.session_id),
            explanations,
            best_explanation,
            uncertainty,
            evidence_chain,
            conservation_checks: all_checks,
            conservation_eliminated: eliminated,
            shadow_note,
            origin: "SRK".to_string(),
        }
    }

    fn build_evidence(
        explanations: &[AbductiveExplanation],
        pro_result: &ProResult,
    ) -> Vec<Evidence> {
        let mut chain = Vec::new();
        // Evidencë nga LIM (masa).
        chain.push(Evidence {
            source: "LIM".to_string(),
            claim: format!("epistemic_mass={:.3}", pro_result.source_mass),
            verifiable: true,
        });
        // Evidencë nga secili shpjegim (IBE).
        for e in explanations {
            chain.push(Evidence {
                source: "SRK".to_string(),
                claim: format!("{} IBE={:.3}", e.source_operator.name(), e.ibe_score),
                verifiable: true,
            });
        }
        chain
    }

    fn build_shadow_note(
        best: &Option<AbductiveExplanation>,
        uncertainty: &UncertaintyProfile,
        eliminated: usize,
    ) -> String {
        match best {
            Some(b) => format!(
                "SRK→Shadow: best={} IBE={:.3} | combined_uncertainty={:.3} | conservation_eliminated={}",
                b.source_operator.name(), b.ibe_score,
                uncertainty.combined, eliminated,
            ),
            None => format!(
                "SRK→Shadow: asnjë shpjegim mbijetoi (conservation_eliminated={})",
                eliminated,
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lim_types::Operator;

    fn pro_result(candidates: Vec<PROCandidate>) -> ProResult {
        ProResult {
            candidates,
            operators_run: 4,
            nk_blocked: 0,
            source_mass: 0.5,
            hw_parallel: 4,
            workers_joined: 4,
            fragments_admitted: 1,
            working_set_bytes: 64,
        }
    }

    fn cand(id: &str, op: Operator, score: f32, conf: f32, refs: Vec<usize>) -> PROCandidate {
        PROCandidate {
            candidate_id: id.to_string(),
            operator: op, score, confidence: conf,
            reasoning: "r".to_string(), fragment_refs: refs,
        }
    }

    #[test]
    fn srk_produces_package() {
        let srk = ScientificReasoningKernel::new("s1", "science");
        let pr = pro_result(vec![
            cand("C1", Operator::Reverse, 0.8, 0.75, vec![0, 1]),
            cand("C2", Operator::Trace, 0.6, 0.6, vec![0, 1, 2]),
        ]);
        let pkg = srk.process(&pr);
        assert!(pkg.best_explanation.is_some());
        assert_eq!(pkg.origin, "SRK");
        // Renditur sipas IBE
        for w in pkg.explanations.windows(2) {
            assert!(w[0].ibe_score >= w[1].ibe_score);
        }
    }

    #[test]
    fn srk_eliminates_conservation_violators() {
        let srk = ScientificReasoningKernel::new("s1", "science");
        let pr = pro_result(vec![
            cand("C1", Operator::Reverse, 0.8, 0.75, vec![0, 1]),  // OK
            cand("C2", Operator::Mirror, 0.5, 0.5, vec![]),         // pa refs → violated
            cand("C3", Operator::Trace, 0.99, 0.99, vec![0]),       // overconfident → violated
        ]);
        let pkg = srk.process(&pr);
        assert_eq!(pkg.conservation_eliminated, 2);
        assert_eq!(pkg.explanations.len(), 1);
    }

    #[test]
    fn srk_never_decides() {
        // verdict s'ekziston në ReasoningPackage — SRK NUK vendos.
        let srk = ScientificReasoningKernel::new("s1", "x");
        let pkg = srk.process(&pro_result(vec![
            cand("C1", Operator::Reverse, 0.7, 0.7, vec![0, 1]),
        ]));
        assert_eq!(pkg.origin, "SRK");
        assert!(pkg.shadow_note.contains("SRK→Shadow"));
    }
}
