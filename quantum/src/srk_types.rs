// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  SRK_TYPES.RS — Scientific Reasoning Kernel: tipet                    ║
// ║  GJATA LEGACY™ — Quantum Platform                                     ║
// ║                                                                          ║
// ║  POZICIONI: MES PRO dhe PIM. SRK filtron + arsyeton shkencërisht.     ║
// ║  SRK NUK vendos (verdict = None). Prodhon ReasoningPackage.           ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::lim_types::Operator;

// ─────────────────────────────────────────────────────────────────────────────
// ABDUCTIVE EXPLANATION — IBE Lipton (1991)
// ─────────────────────────────────────────────────────────────────────────────

/// Shpjegimi abduktiv me 5 kriteret e Lipton.
#[derive(Debug, Clone)]
pub struct AbductiveExplanation {
    pub explanation_id: String,
    /// Lipton (1991) — Inference to the Best Explanation
    pub loveliness:        f32,  // sa parsimonik/i unifikuar
    pub likeliness:        f32,  // sa probable a priori
    pub explanatory_scope: f32,  // sa fenomene mbulon
    pub simplicity:        f32,  // Occam's Razor
    pub coherence:         f32,  // koherenca me teorinë
    pub ibe_score:         f32,  // agregat i 5 kritereve
    /// Kandidati nga PRO që e prodhoi.
    pub source_operator: Operator,
    pub source_id:       String,
}

// ─────────────────────────────────────────────────────────────────────────────
// BAYESIAN POSTERIOR — uncertainty
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct BayesianPosterior {
    pub prior:     f32,   // para verdict Shadow
    pub posterior: f32,   // pas Bayes update
}

/// Profili i pasigurisë: epistemic (reduktibël) + aleatoric (irreduktibël).
#[derive(Debug, Clone)]
pub struct UncertaintyProfile {
    pub epistemic: f32,   // çfarë s'dimë por mund të dimë
    pub aleatoric: f32,   // pasiguri e natyrshme (irreduktibël)
    pub combined:  f32,   // sqrt(e² + a²) / sqrt(2)
    pub posterior: BayesianPosterior,
}

// ─────────────────────────────────────────────────────────────────────────────
// CONSERVATION CHECK — ligjet e ruajtjes epistemike
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ConservationCheck {
    pub law_name: String,
    pub violated: bool,
    pub reason:   String,
}

// ─────────────────────────────────────────────────────────────────────────────
// EVIDENCE — proof-carrying
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Evidence {
    pub source:     String,   // PRO / SRK / LIM
    pub claim:      String,
    pub verifiable: bool,
}

// ─────────────────────────────────────────────────────────────────────────────
// REASONING PACKAGE — output i SRK tek PIM/Shadow
// ─────────────────────────────────────────────────────────────────────────────

/// Pakoja që SRK prodhon. verdict = None (SRK NUK vendos).
#[derive(Debug, Clone)]
pub struct ReasoningPackage {
    pub package_id: String,
    /// Shpjegimet që mbijetuan filtrimin, renditur sipas IBE.
    pub explanations: Vec<AbductiveExplanation>,
    /// Shpjegimi më i mirë (IBE max).
    pub best_explanation: Option<AbductiveExplanation>,
    pub uncertainty: UncertaintyProfile,
    pub evidence_chain: Vec<Evidence>,
    pub conservation_checks: Vec<ConservationCheck>,
    /// Sa kandidatë u eliminuan nga conservation.
    pub conservation_eliminated: usize,
    /// Orientim për Shadow (JO vendim).
    pub shadow_note: String,
    pub origin: String,  // "SRK"
}

impl ReasoningPackage {
    pub fn report(&self) -> String {
        let mut out = format!(
            "ReasoningPackage [{}]: {} shpjegime | conservation_eliminated={}\n",
            self.package_id, self.explanations.len(), self.conservation_eliminated,
        );
        match &self.best_explanation {
            Some(b) => out += &format!(
                "  BEST: [{}] IBE={:.3} (lovel={:.2} likel={:.2} scope={:.2} simpl={:.2} coher={:.2})\n",
                b.source_operator.name(), b.ibe_score,
                b.loveliness, b.likeliness, b.explanatory_scope, b.simplicity, b.coherence,
            ),
            None => {}
        }
        out += &format!(
            "  uncertainty: epistemic={:.3} aleatoric={:.3} combined={:.3} posterior={:.3}\n",
            self.uncertainty.epistemic, self.uncertainty.aleatoric,
            self.uncertainty.combined, self.uncertainty.posterior.posterior,
        );
        out += &format!("  → {}\n", self.shadow_note);
        out
    }
}
