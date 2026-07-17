// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  PIM.RS — PASS INFO METRIC — Potentiality 5D                          ║
// ║  GJATA LEGACY™ — Quantum Platform                                     ║
// ║                                                                          ║
// ║  POZICIONI: pas SRK, para Shadow.                                     ║
// ║  PIM NUK ri-klasifikon — mat POTENCIALITETIN në 5 dimensione.        ║
// ║  PIM sugjeron — KURRË nuk vendos (is_directive=false).               ║
// ║                                                                          ║
// ║  5D:                                                                   ║
// ║   evidence_density, logical_coherence, causal_integrity,             ║
// ║   convergence_strength, reproducibility                              ║
// ║  → potentiality_score (peshat sipas kategorisë)                      ║
// ║  → ShadowSuggestion: CONSIDER_OPTIMAL | HOLD_FOR_TESTING | EXPLORE   ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::srk_types::ReasoningPackage;
use crate::lim_types::EpistemicState;
use crate::lab::lab_types::TrlEvidence;

// ─────────────────────────────────────────────────────────────────────────────
// POTENTIALITY PROFILE — 5 dimensionet
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct PotentialityProfile {
    pub evidence_density:     f32,
    pub logical_coherence:    f32,
    pub causal_integrity:     f32,
    pub convergence_strength: f32,
    pub reproducibility:      f32,
    pub potentiality_score:   f32,
}

// ─────────────────────────────────────────────────────────────────────────────
// PROOF LINK — proof-carrying chain
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ProofLink {
    pub source:     String,   // PRO/SRK/LIM/LAB/LINEAGE
    pub claim:      String,
    pub verifiable: bool,
}

// ─────────────────────────────────────────────────────────────────────────────
// SHADOW SUGGESTION — sugjerim, JO vendim (is_directive=false)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum SuggestedVerdict {
    ConsiderOptimal,
    HoldForTesting,
    ExploreFurther,
}

impl SuggestedVerdict {
    pub fn label(&self) -> &'static str {
        match self {
            Self::ConsiderOptimal => "CONSIDER_OPTIMAL",
            Self::HoldForTesting  => "HOLD_FOR_TESTING",
            Self::ExploreFurther  => "EXPLORE_FURTHER",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ShadowSuggestion {
    pub suggested_verdict: SuggestedVerdict,
    pub reasoning:         String,
    /// GJITHMONË false — PIM sugjeron, Shadow vendos.
    pub is_directive:      bool,
}

// ─────────────────────────────────────────────────────────────────────────────
// PASS PACKAGE — output i PIM tek Shadow
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct PassPackage {
    pub package_id:        String,
    pub potentiality:      PotentialityProfile,
    pub proof_chain:       Vec<ProofLink>,
    pub shadow_suggestion: ShadowSuggestion,
    /// Banda e dijes (nga kategoria epistemike).
    pub knowledge_band:    String,
    pub origin:            String,  // "PIM"
}

impl PassPackage {
    pub fn report(&self) -> String {
        let p = &self.potentiality;
        format!(
            "PassPackage [{}]\n  5D: ed={:.2} lc={:.2} ci={:.2} cs={:.2} rp={:.2}\n  \
             potentiality_score={:.3}\n  knowledge_band={}\n  \
             SHADOW_SUGGESTION: {} (directive={})\n    {}\n",
            self.package_id,
            p.evidence_density, p.logical_coherence, p.causal_integrity,
            p.convergence_strength, p.reproducibility, p.potentiality_score,
            self.knowledge_band,
            self.shadow_suggestion.suggested_verdict.label(),
            self.shadow_suggestion.is_directive,
            self.shadow_suggestion.reasoning,
        )
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// PIM ENGINE
// ─────────────────────────────────────────────────────────────────────────────

pub struct PassInfoMetric;

impl PassInfoMetric {
    /// Mat potencialitetin e ReasoningPackage → PassPackage.
    ///
    /// state = EpistemicState nga LIM faza 2 (përcakton peshat).
    pub fn measure(
        srk_package: &ReasoningPackage,
        state:       &EpistemicState,
        session_id:  &str,
    ) -> PassPackage {
        // ── 5 DIMENSIONET ──────────────────────────────────────────────────
        let potentiality = Self::compute_potentiality(srk_package, state);

        // ── PROOF CHAIN (proof-carrying) ───────────────────────────────────
        let proof_chain = Self::build_proof_chain(srk_package);

        // ── SHADOW SUGGESTION (JO vendim) ──────────────────────────────────
        let shadow_suggestion = Self::build_suggestion(&potentiality, state);

        // ── KNOWLEDGE BAND ─────────────────────────────────────────────────
        let knowledge_band = state.kind_str().to_string();

        PassPackage {
            package_id: format!("PIM_{}", session_id),
            potentiality,
            proof_chain,
            shadow_suggestion,
            knowledge_band,
            origin: "PIM".to_string(),
        }
    }

    /// measure_with_lab — matje E SAKTË duke përdorur evidencën e Digital Lab.
    ///
    /// Lidhja kritike: Digital Lab mat reproducibility-n përmes simulimit me
    /// përsëritje (më saktë se vlerësimi nga evidence_chain). Kjo metodë e
    /// integron atë evidencë në potentiality.
    ///
    /// pending_lab=true → përdor matjen standarde (Lab s'u konsultua).
    /// pending_lab=false → reproducibility nga Lab + confidence boost.
    /// Zero if — formula me faktorë boolean.
    pub fn measure_with_lab(
        srk_package: &ReasoningPackage,
        state:       &EpistemicState,
        session_id:  &str,
        lab_evidence: &TrlEvidence,
    ) -> PassPackage {
        // Matja bazë.
        let mut potentiality = Self::compute_potentiality(srk_package, state);

        // lab_active ∈ {0,1}: a u konsultua Lab-i?
        let lab_active = (!lab_evidence.pending_lab) as u32 as f32;

        // Reproducibility i përmirësuar: kur Lab aktiv, përziej me matjen e Lab-it.
        // Lab mat më saktë (simulim me përsëritje) → peshë 0.6 për Lab kur aktiv.
        let lab_repro = lab_evidence.reproducibility;
        let base_repro = potentiality.reproducibility;
        let merged_repro = lab_active * (lab_repro * 0.6 + base_repro * 0.4)
                         + (1.0 - lab_active) * base_repro;
        potentiality.reproducibility = merged_repro.clamp(0.0, 1.0);

        // Causal integrity boost kur TRL3 kaloi (evidencë e fortë eksperimentale).
        let trl_passed = (lab_evidence.trl_passed) as u32 as f32;
        let causal_boost = lab_active * trl_passed * 0.10;
        potentiality.causal_integrity = (potentiality.causal_integrity + causal_boost).clamp(0.0, 1.0);

        // Ri-llogarit potentiality_score me dimensionet e përditësuara.
        let (w_ed, w_lc, w_ci, w_cs, w_rp) = Self::weights_for(state);
        potentiality.potentiality_score = (
            potentiality.evidence_density     * w_ed +
            potentiality.logical_coherence    * w_lc +
            potentiality.causal_integrity     * w_ci +
            potentiality.convergence_strength * w_cs +
            potentiality.reproducibility      * w_rp
        ).clamp(0.0, 1.0);

        let proof_chain = Self::build_proof_chain(srk_package);
        let shadow_suggestion = Self::build_suggestion(&potentiality, state);
        let knowledge_band = state.kind_str().to_string();

        PassPackage {
            package_id: format!("PIM_{}_LAB", session_id),
            potentiality,
            proof_chain,
            shadow_suggestion,
            knowledge_band,
            origin: "PIM+LAB".to_string(),
        }
    }

    /// weights_for — peshat 5D sipas kategorisë (i përbashkët).
    fn weights_for(state: &EpistemicState) -> (f32, f32, f32, f32, f32) {
        match state {
            EpistemicState::Hypothesis { .. } =>
                (0.30, 0.30, 0.20, 0.10, 0.10),
            EpistemicState::PotentialHypothesis { .. } =>
                (0.25, 0.25, 0.25, 0.15, 0.10),
            EpistemicState::FactCandidate { .. } =>
                (0.20, 0.20, 0.25, 0.15, 0.20),
        }
    }

    /// 5D potentiality me peshat sipas kategorisë (besnik ndaj pim_core.pseudo).
    fn compute_potentiality(
        pkg:   &ReasoningPackage,
        state: &EpistemicState,
    ) -> PotentialityProfile {
        let best_ibe = pkg.best_explanation.as_ref()
            .map(|e| e.ibe_score).unwrap_or(0.0);
        let best_coherence = pkg.best_explanation.as_ref()
            .map(|e| e.coherence).unwrap_or(0.0);

        // evidence_density: nga IBE + gjatësia e proof chain.
        let evidence_density = (best_ibe * 0.6
            + (pkg.evidence_chain.len() as f32 / 6.0).clamp(0.0, 1.0) * 0.4)
            .clamp(0.0, 1.0);

        // logical_coherence: nga coherence i shpjegimit + (1 - epistemic uncert).
        let logical_coherence = (best_coherence * 0.6
            + (1.0 - pkg.uncertainty.epistemic) * 0.4).clamp(0.0, 1.0);

        // causal_integrity: conservation passed → integ i lartë.
        let conservation_ok = pkg.conservation_checks.iter().all(|c| !c.violated);
        let causal_integrity = (conservation_ok as u32 as f32) * 0.5
            + (1.0 - pkg.uncertainty.combined) * 0.5;

        // convergence_strength: (1 - combined uncertainty) + posterior.
        let convergence_strength = ((1.0 - pkg.uncertainty.combined) * 0.6
            + pkg.uncertainty.posterior.posterior * 0.4).clamp(0.0, 1.0);

        // reproducibility: sa evidencë e verifikueshme.
        let verifiable = pkg.evidence_chain.iter().filter(|e| e.verifiable).count();
        let total = pkg.evidence_chain.len().max(1);
        let reproducibility = (verifiable as f32 / total as f32).clamp(0.0, 1.0);

        // ── Peshat sipas kategorisë ────────────────────────────────────────
        let (w_ed, w_lc, w_ci, w_cs, w_rp) = match state {
            EpistemicState::Hypothesis { .. } =>
                (0.30, 0.30, 0.20, 0.10, 0.10),
            EpistemicState::PotentialHypothesis { .. } =>
                (0.25, 0.25, 0.25, 0.15, 0.10),
            EpistemicState::FactCandidate { .. } =>
                (0.20, 0.20, 0.25, 0.15, 0.20),
        };

        let potentiality_score = (
            evidence_density     * w_ed +
            logical_coherence    * w_lc +
            causal_integrity     * w_ci +
            convergence_strength * w_cs +
            reproducibility      * w_rp
        ).clamp(0.0, 1.0);

        PotentialityProfile {
            evidence_density,
            logical_coherence,
            causal_integrity,
            convergence_strength,
            reproducibility,
            potentiality_score,
        }
    }

    fn build_proof_chain(pkg: &ReasoningPackage) -> Vec<ProofLink> {
        let mut chain = Vec::new();
        for e in &pkg.evidence_chain {
            chain.push(ProofLink {
                source: e.source.clone(),
                claim: e.claim.clone(),
                verifiable: e.verifiable,
            });
        }
        chain
    }

    /// Shadow suggestion — KURRË VERIFIED_*; vetëm sugjerim.
    fn build_suggestion(
        p:     &PotentialityProfile,
        state: &EpistemicState,
    ) -> ShadowSuggestion {
        let score = p.potentiality_score;

        // Vendos sugjerimin sipas kategori + potencialitet.
        let (verdict, reasoning) = match state {
            EpistemicState::FactCandidate { .. } => {
                match score > 0.70 {
                    true => (SuggestedVerdict::ConsiderOptimal,
                     format!("FactCandidate me potencialitet {:.0}% — Shadow mund ta konsiderojë optimal", score * 100.0)),
                    false => (SuggestedVerdict::HoldForTesting,
                     format!("FactCandidate por potencialitet {:.0}% — kërkon validim", score * 100.0)),
                }
            }
            EpistemicState::PotentialHypothesis { .. } => {
                match score > 0.50 {
                    true => (SuggestedVerdict::ExploreFurther,
                     format!("Hipotezë potenciale {:.0}% — eksplorim premtues", score * 100.0)),
                    false => (SuggestedVerdict::HoldForTesting,
                     format!("Hipotezë potenciale {:.0}% — pezull për evidencë", score * 100.0)),
                }
            }
            EpistemicState::Hypothesis { .. } => {
                (SuggestedVerdict::HoldForTesting,
                 "Hipotezë embrionale — materiali ka nevojë për elaborim".to_string())
            }
        };

        ShadowSuggestion {
            suggested_verdict: verdict,
            reasoning,
            is_directive: false,  // GJITHMONË false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::srk::ScientificReasoningKernel;
    use crate::pro_types::{ProResult, PROCandidate};
    use crate::lim_types::Operator;

    fn make_state() -> EpistemicState {
        EpistemicState::PotentialHypothesis {
            confidence_range: (0.36, 0.98),
            stability: 0.6,
            trl_candidate: 2,
            pro_mode: "deep".to_string(),
        }
    }

    fn make_package() -> ReasoningPackage {
        let srk = ScientificReasoningKernel::new("s1", "science");
        let pr = ProResult {
            candidates: vec![
                PROCandidate {
                    candidate_id: "C1".to_string(), operator: Operator::Reverse,
                    score: 0.8, confidence: 0.78,
                    reasoning: "r".to_string(), fragment_refs: vec![0, 1, 2],
                },
            ],
            operators_run: 4, nk_blocked: 0, source_mass: 0.6, hw_parallel: 4,
            workers_joined: 4, fragments_admitted: 3, working_set_bytes: 128,
        };
        srk.process(&pr)
    }

    #[test]
    fn pim_produces_passpackage() {
        let pkg = make_package();
        let pass = PassInfoMetric::measure(&pkg, &make_state(), "s1");
        assert_eq!(pass.origin, "PIM");
        assert!((0.0..=1.0).contains(&pass.potentiality.potentiality_score));
    }

    #[test]
    fn pim_never_directive() {
        let pkg = make_package();
        let pass = PassInfoMetric::measure(&pkg, &make_state(), "s1");
        // is_directive GJITHMONË false — PIM sugjeron, s'vendos.
        assert!(!pass.shadow_suggestion.is_directive);
        // Kurrë VERIFIED_* — vetëm CONSIDER/HOLD/EXPLORE.
        let v = pass.shadow_suggestion.suggested_verdict.label();
        assert!(v == "CONSIDER_OPTIMAL" || v == "HOLD_FOR_TESTING" || v == "EXPLORE_FURTHER");
    }

    #[test]
    fn pim_proof_chain_carries() {
        let pkg = make_package();
        let pass = PassInfoMetric::measure(&pkg, &make_state(), "s1");
        assert!(!pass.proof_chain.is_empty());
        // Duhet të ketë të paktën një link nga LIM.
        assert!(pass.proof_chain.iter().any(|l| l.source == "LIM"));
    }
}
