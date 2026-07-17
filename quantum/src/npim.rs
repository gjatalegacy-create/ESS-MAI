// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  NPIM.RS — Negative PIM (Paketuesi i Negative Knowledge)             ║
// ║  GJATA LEGACY™ — Quantum Platform                                     ║
// ║                                                                          ║
// ║  NPIM është paralelja negative e PIM. Ashtu si PIM mat potencialitetin ║
// ║  5D dhe paketon për Shadow, NPIM mat NEGATIVITETIN dhe paketon         ║
// ║  Negative Knowledge ME ARGUMENTA.                                     ║
// ║                                                                          ║
// ║  3 dimensionet e kundërta (sipas vizionit të arkitektit):            ║
// ║   failure_density       — sa dendur janë dështimet                   ║
// ║   contradiction_strength — sa të forta janë kontradiktat              ║
// ║   elimination_certainty  — sa e sigurt është eliminimi                ║
// ║  → negativity_score → NegativeKnowledgePackage për Shadow            ║
// ║                                                                          ║
// ║  NPIM SUGJERON (jo vendos): CONFIRM_NEGATIVE | HOLD_NEGATIVE | WEAK.   ║
// ║  Shadow vendos nëse negative knowledge është e konfirmuar.            ║
// ║  Zero if/else — formula matematike.                                   ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::npro::{NproResult, NegativeElimination};
use crate::sovereign::laws::gate;

// ─────────────────────────────────────────────────────────────────────────────
// NEGATIVITY PROFILE — dimensionet e kundërta të PIM
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
pub struct NegativityProfile {
    pub failure_density:        f32,  // ↔ evidence_density
    pub contradiction_strength: f32,  // ↔ logical_coherence (e kundërt)
    pub elimination_certainty:  f32,  // ↔ convergence_strength
    pub negativity_score:       f32,  // ↔ potentiality_score
}

// ─────────────────────────────────────────────────────────────────────────────
// NEGATIVE SUGGESTION — NPIM sugjeron, Shadow vendos
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NegativeSuggestion {
    ConfirmNegative,  // negativitet i fortë → konfirmo si NK
    HoldNegative,     // i moderuar → mban për vlerësim
    WeakElimination,  // i dobët → ndoshta s'është NK e vërtetë
}

impl NegativeSuggestion {
    pub fn label(self) -> &'static str {
        match self {
            NegativeSuggestion::ConfirmNegative => "CONFIRM_NEGATIVE",
            NegativeSuggestion::HoldNegative    => "HOLD_NEGATIVE",
            NegativeSuggestion::WeakElimination => "WEAK_ELIMINATION",
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ARGUMENT — argumenti i paketuar për një eliminim
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct NegativeArgument {
    pub operator:       String,
    pub cause:          String,       // pse u eliminua
    pub consequence:    String,       // pasoja nëse pranohej
    pub weakness_score: f32,
}

// ─────────────────────────────────────────────────────────────────────────────
// NEGATIVE KNOWLEDGE PACKAGE — për Shadow
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct NegativeKnowledgePackage {
    pub package_id:     String,
    pub negativity:     NegativityProfile,
    pub arguments:      Vec<NegativeArgument>,   // paketim me argumenta
    pub suggestion:     NegativeSuggestion,      // JO vendim
    pub origin:         String,
}

impl NegativeKnowledgePackage {
    pub fn report(&self) -> String {
        format!(
            "NPIM[{}] negativity={:.3} args={} → {}",
            self.package_id, self.negativity.negativity_score,
            self.arguments.len(), self.suggestion.label()
        )
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// NPIM ENGINE
// ─────────────────────────────────────────────────────────────────────────────

/// Pragjet për suggestion (jo vendim — Shadow vendos).
pub const CONFIRM_THRESHOLD: f32 = 0.70;
pub const HOLD_THRESHOLD:     f32 = 0.40;

pub struct NegativePassMetric;

impl NegativePassMetric {
    /// measure — pasqyrim simetrik i PIM::measure për negativen.
    ///
    /// Merr NproResult (eliminimet nga NPRO) → NegativeKnowledgePackage.
    pub fn measure(npro: &NproResult, session_id: &str) -> NegativeKnowledgePackage {
        // ── 3 DIMENSIONET E KUNDËRTA ──────────────────────────────────────
        let negativity = Self::compute_negativity(&npro.eliminations);

        // ── ARGUMENTA (paketim) ────────────────────────────────────────────
        let arguments = Self::build_arguments(&npro.eliminations);

        // ── SUGGESTION (JO vendim) ─────────────────────────────────────────
        let suggestion = Self::build_suggestion(&negativity);

        NegativeKnowledgePackage {
            package_id: format!("NPIM_{}", session_id),
            negativity,
            arguments,
            suggestion,
            origin: "NPIM".to_string(),
        }
    }

    /// compute_negativity — 3 dimensionet nga eliminimet.
    fn compute_negativity(eliminations: &[NegativeElimination]) -> NegativityProfile {
        let n = eliminations.len().max(1) as f32;

        // failure_density: sa dendur janë eliminimet me weakness të lartë.
        let high_weakness = eliminations.iter()
            .filter(|e| e.weakness_score > 0.5)
            .count() as f32;
        let failure_density = (high_weakness / n).clamp(0.0, 1.0);

        // contradiction_strength: weakness mesatar (sa të forta dështimet).
        let sum_weakness: f32 = eliminations.iter().map(|e| e.weakness_score).sum();
        let contradiction_strength = (sum_weakness / n).clamp(0.0, 1.0);

        // elimination_certainty: weakness maksimal (sa i sigurt eliminimi më i fortë).
        let max_weakness = eliminations.iter()
            .map(|e| e.weakness_score)
            .fold(0.0f32, f32::max);
        let elimination_certainty = max_weakness.clamp(0.0, 1.0);

        // negativity_score: peshim i 3 dimensioneve.
        let negativity_score = gate(
            failure_density * 0.35
            + contradiction_strength * 0.35
            + elimination_certainty * 0.30,
            0.0, 1.0,
        );

        NegativityProfile {
            failure_density,
            contradiction_strength,
            elimination_certainty,
            negativity_score,
        }
    }

    /// build_arguments — paketon çdo eliminim me cause + consequence.
    fn build_arguments(eliminations: &[NegativeElimination]) -> Vec<NegativeArgument> {
        eliminations.iter()
            .map(|e| NegativeArgument {
                operator: e.operator.clone(),
                cause: e.cause.clone(),
                consequence: Self::infer_consequence(e.weakness_score),
                weakness_score: e.weakness_score,
            })
            .collect()
    }

    /// infer_consequence — pasoja nga niveli i dobësisë (zero if — match guards).
    fn infer_consequence(weakness: f32) -> String {
        let severe = weakness > 0.80;
        let moderate = weakness > 0.50;
        match (severe, moderate) {
            (true, _)  => "destabilizim i rëndë nëse pranohej".to_string(),
            (_, true)  => "humbje koherence nëse pranohej".to_string(),
            _          => "rrezik i ulët, por jo optimal".to_string(),
        }
    }

    /// build_suggestion — sugjerim nga negativity (JO vendim).
    /// Zero if — match mbi tuple guards.
    fn build_suggestion(neg: &NegativityProfile) -> NegativeSuggestion {
        let confirm = neg.negativity_score >= CONFIRM_THRESHOLD;
        let hold = neg.negativity_score >= HOLD_THRESHOLD;
        match (confirm, hold) {
            (true, _)  => NegativeSuggestion::ConfirmNegative,
            (_, true)  => NegativeSuggestion::HoldNegative,
            _          => NegativeSuggestion::WeakElimination,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::npro::NproEngine;

    fn npro_from(frags: &[&str], signal: f32, causal: f32) -> NproResult {
        let owned: Vec<String> = frags.iter().map(|s| s.to_string()).collect();
        NproEngine::activate(&owned, signal, causal)
    }

    #[test]
    fn npim_packages_eliminations() {
        let npro = npro_from(&["a", "b"], 0.5, 0.5);
        let pkg = NegativePassMetric::measure(&npro, "s1");
        // 4 eliminime → 4 argumenta.
        assert_eq!(pkg.arguments.len(), 4);
        assert_eq!(pkg.origin, "NPIM");
    }

    #[test]
    fn high_weakness_confirms_negative() {
        // Fragment i vetëm → weakness shumë i lartë → CONFIRM.
        let npro = npro_from(&["single"], 0.1, 0.1);
        let pkg = NegativePassMetric::measure(&npro, "s1");
        // Negativity i lartë → ConfirmNegative ose HoldNegative.
        assert!(pkg.negativity.negativity_score > 0.4);
    }

    #[test]
    fn strong_input_can_produce_decisive_negative_evidence() {
        // v1.5.1 (Korrigjimi C, TEST-ONLY): semantika AUTORITATIVE —
        // negativity_score është BESUESHMËRIA/intensiteti se një rrugë
        // është negative dhe duhet eliminuar. Input i fortë MUND të japë
        // eliminime negative shumë të qarta: score i lartë është i SAKTË
        // kur rruga e gabuar identifikohet me besueshmëri të lartë.
        // Formula (0.35/0.35/0.30), pragjet (0.70/0.40) dhe NproEngine
        // janë TË PAPREKURA — ndryshohet vetëm testi i vjetruar.
        let npro = npro_from(&[
            "deterministic elimination produces verifiable knowledge here",
            "each candidate passes through negative filters before acceptance",
            "surviving knowledge becomes primitive knowledge over time",
        ], 0.9, 0.9);
        let pkg = NegativePassMetric::measure(&npro, "s1");

        // 1. Score gjithmonë në [0.0, 1.0].
        assert!((0.0..=1.0).contains(&pkg.negativity.negativity_score));
        // 2. Për këtë fixture, prova negative reale është VENDIMTARE:
        //    score >= CONFIRM_THRESHOLD → sugjerimi ConfirmNegative.
        //    (Lidhur me pragun dhe enum-in real, JO me decimal arbitrar.)
        assert!(pkg.negativity.negativity_score >= CONFIRM_THRESHOLD,
            "prova vendimtare negative: score={} < CONFIRM_THRESHOLD",
            pkg.negativity.negativity_score);
        assert_eq!(pkg.suggestion, NegativeSuggestion::ConfirmNegative);
        // 3. Paketa mbetet SUGJERIM — kurrë verdikt Shadow.
        assert!(!pkg.suggestion.label().contains("VERIFIED"));
        // 4. Argumentet ruajnë operator, cause, consequence, weakness_score.
        for arg in &pkg.arguments {
            assert!(!arg.operator.is_empty());
            assert!(!arg.cause.is_empty());
            assert!(!arg.consequence.is_empty());
            assert!((0.0..=1.0).contains(&arg.weakness_score));
        }
        // 5. Numri i argumenteve = eliminimet REALE (jo hardcode arbitrar).
        assert_eq!(pkg.arguments.len(), npro.eliminations.len());
    }

    #[test]
    fn negativity_dimensions_in_range() {
        let npro = npro_from(&["a", "b"], 0.5, 0.5);
        let pkg = NegativePassMetric::measure(&npro, "s1");
        assert!((0.0..=1.0).contains(&pkg.negativity.failure_density));
        assert!((0.0..=1.0).contains(&pkg.negativity.contradiction_strength));
        assert!((0.0..=1.0).contains(&pkg.negativity.elimination_certainty));
        assert!((0.0..=1.0).contains(&pkg.negativity.negativity_score));
    }

    #[test]
    fn arguments_have_cause_and_consequence() {
        let npro = npro_from(&["single fragment"], 0.5, 0.5);
        let pkg = NegativePassMetric::measure(&npro, "s1");
        for arg in &pkg.arguments {
            assert!(!arg.cause.is_empty());
            assert!(!arg.consequence.is_empty());
        }
    }

    #[test]
    fn suggestion_never_decides_verified() {
        // NPIM sugjeron, kurrë s'thotë VERIFIED_* (vetëm Shadow).
        let npro = npro_from(&["x"], 0.5, 0.5);
        let pkg = NegativePassMetric::measure(&npro, "s1");
        let label = pkg.suggestion.label();
        assert!(!label.contains("VERIFIED"));
        assert!(label == "CONFIRM_NEGATIVE" || label == "HOLD_NEGATIVE" || label == "WEAK_ELIMINATION");
    }

    #[test]
    fn elimination_certainty_is_max() {
        let npro = npro_from(&["single"], 0.5, 0.5);
        let pkg = NegativePassMetric::measure(&npro, "s1");
        // elimination_certainty = max weakness ndër eliminimet.
        let max_w = npro.eliminations.iter().map(|e| e.weakness_score).fold(0.0f32, f32::max);
        assert!((pkg.negativity.elimination_certainty - max_w).abs() < 1e-5);
    }
}
