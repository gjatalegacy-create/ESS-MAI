// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LIM_CLASSIFIER.RS — Klasifikimi + Operator Gates + Depth Config      ║
// ║  GJATA LEGACY™ — Quantum Platform / Linear Info Metric                ║
// ║                                                                          ║
// ║  Besnik ndaj lim_core.pseudo & lim_analyzers.pseudo:                  ║
// ║   • classify_epistemic_state: thresholds 0.30/0.35/0.98               ║
// ║   • compute_operator_gates: REVERSE/MIRROR/CONTRAST/TRACE             ║
// ║   • compute_depth_config: operator_depth, turbo, reconstruction       ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::lim_types::*;

// ─────────────────────────────────────────────────────────────────────────────
// KLASIFIKIMI — epistemic_mass → EpistemicState
// Nga: FUNCTION classify_epistemic_state (lim_core.pseudo)
// ─────────────────────────────────────────────────────────────────────────────

pub fn classify_epistemic_state(lim: &LinearInfoMetric) -> EpistemicState {
    let mass = lim.epistemic_mass;

    // 0.30-0.35: Hypothesis (TRL0)
    match mass >= 0.30 && mass <= 0.35 {
        true => return EpistemicState::Hypothesis {
            confidence_range: (0.30, 0.35),
            reason: build_hypothesis_reason(lim),
            pro_mode: "reconstruction_mode".to_string(),
        },
        false => {}
    }

    // 0.36-0.98: PotentialHypothesis (TRL1/2/3 nga thellësia)
    match mass > 0.35 && mass <= 0.98 {
        true => { let trl = match (mass < 0.60, mass < 0.80) {
            (true, _)  => 1,
            (_, true)  => 2,
            _          => 3,
        };
        return EpistemicState::PotentialHypothesis {
            confidence_range: (0.36, 0.98),
            stability: lim.semantic_integrity,
            trl_candidate: trl,
            pro_mode: "deep_exploration_mode".to_string(),
        }; }
        false => {}
    }

    // > 0.98: FactCandidate (kandidat për validim, JO fakt absolut)
    match mass > 0.98 {
        true => return EpistemicState::FactCandidate {
            confidence_range: (0.99, 1.00),
            validation_path: "DigitalLab → TRL_progression → sovereign_validation".to_string(),
            pro_mode: "turbo_mode".to_string(),
        },
        false => {}
    }

    // < 0.30: Hypothesis minimal (reconstruction)
    EpistemicState::Hypothesis {
        confidence_range: (0.00, 0.29),
        reason: "Epistemic mass nën threshold — reconstruction i nevojshëm".to_string(),
        pro_mode: "minimal_reconstruction".to_string(),
    }
}

fn build_hypothesis_reason(lim: &LinearInfoMetric) -> String {
    format!(
        "mass={:.2} density={:.2} continuity={:.2} — strukturë embrionale",
        lim.epistemic_mass, lim.information_density, lim.continuity_score,
    )
}

// ─────────────────────────────────────────────────────────────────────────────
// OPERATOR GATES — çfarë operatori PRO aktivizohet
// Nga: FUNCTION compute_operator_gates (lim_analyzers.pseudo)
// Vendimet janë kushtëzime (gating) — error-handling i pranuar.
// ─────────────────────────────────────────────────────────────────────────────

pub fn compute_operator_gates(lim: &LinearInfoMetric) -> Vec<OperatorGate> {
    let depth = lim.reasoning_depth_allowance;

    let reverse = OperatorGate {
        operator: Operator::Reverse,
        // aktiv nëse ka info por kontradiktë e lartë
        active: lim.information_density > 0.25 && lim.contradiction_presence > 0.30,
        // Zero if/else: cond=true→1, false→3 (formulë: 3 - 2*cond)
        priority: 3 - 2 * ((lim.contradiction_presence > 0.50) as u8),
        reason: "Kontradiktë e lartë → causal traversal prapa".to_string(),
        max_depth: (depth * 5.0) as u8 + 1,
    };

    let mirror = OperatorGate {
        operator: Operator::Mirror,
        // aktiv nëse continuity e ulët por density e mirë
        active: lim.continuity_score < 0.40 && lim.information_density > 0.30,
        // Zero if/else: cond=true→2, false→4 (formulë: 4 - 2*cond)
        priority: 4 - 2 * ((lim.continuity_score < 0.25) as u8),
        reason: "Continuity e ulët → analogji nga domain tjetër".to_string(),
        max_depth: (depth * 4.0) as u8 + 1,
    };

    let contrast = OperatorGate {
        operator: Operator::Contrast,
        // aktiv nëse linearitet i ulët
        active: lim.contextual_linearity < 0.60,
        // Zero if/else: cond=true→2, false→3 (formulë: 3 - cond)
        priority: 3 - ((lim.contextual_linearity < 0.30) as u8),
        reason: "Linearitet i ulët → diferenca → kuptimi".to_string(),
        max_depth: (depth * 3.0) as u8 + 1,
    };

    let trace = OperatorGate {
        operator: Operator::Trace,
        // fallback: aktiv gjithmonë nëse mass mbi 0.15
        active: lim.epistemic_mass > 0.15,
        // Zero if/else: cond=true→1, false→2 (formulë: 2 - cond)
        priority: 2 - ((lim.information_density < 0.35) as u8),
        reason: "TRACE: operatori bazë — rrugë minimale nga fragmentet".to_string(),
        max_depth: (depth * 6.0) as u8 + 1,
    };

    vec![reverse, mirror, contrast, trace]
}

// ─────────────────────────────────────────────────────────────────────────────
// DEPTH CONFIG — sa thellë shkon PRO
// Nga: FUNCTION compute_depth_config (lim_analyzers.pseudo)
// ─────────────────────────────────────────────────────────────────────────────

pub fn compute_depth_config(lim: &LinearInfoMetric) -> DepthConfig {
    let mass = lim.epistemic_mass;

    let operator_depth = ((lim.reasoning_depth_allowance * 7.0) as u32).clamp(1, 7);
    let cross_territory = lim.continuity_score > 0.50 && mass > 0.40;
    let reconstruction_mode = lim.information_density < 0.35;
    let turbo_allowed = mass > 0.70 && lim.continuity_score > 0.60;

    DepthConfig {
        operator_depth,
        cross_territory_allowed: cross_territory,
        reconstruction_mode,
        turbo_allowed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lim_with_mass(mass: f32) -> LinearInfoMetric {
        LinearInfoMetric {
            information_density: 0.5, continuity_score: 0.5,
            contradiction_presence: 0.1, semantic_integrity: 0.5,
            hypothesis_support_score: 0.5, contextual_linearity: 0.5,
            exploration_readiness: 0.5, reasoning_depth_allowance: 0.5,
            epistemic_mass: mass, epistemic_state: None,
            session_id: "s".into(), territory: "t".into(),
            computed_at: 0, phase: LimPhase::PrePro,
        }
    }

    #[test]
    fn classify_hypothesis() {
        let s = classify_epistemic_state(&lim_with_mass(0.32));
        assert_eq!(s.kind_str(), "HYPOTHESIS");
    }

    #[test]
    fn classify_potential() {
        let s = classify_epistemic_state(&lim_with_mass(0.55));
        assert_eq!(s.kind_str(), "POTENTIAL_HYPOTHESIS");
        // Zero if/else: match në vend të if let.
        match s {
            EpistemicState::PotentialHypothesis { trl_candidate, .. } => {
                assert_eq!(trl_candidate, 1); // mass < 0.60 → TRL1
            }
            _ => {}
        }
    }

    #[test]
    fn classify_fact_candidate() {
        let s = classify_epistemic_state(&lim_with_mass(0.99));
        assert_eq!(s.kind_str(), "FACT_CANDIDATE");
    }

    #[test]
    fn trace_always_active_above_015() {
        let gates = compute_operator_gates(&lim_with_mass(0.5));
        let trace = gates.iter().find(|g| g.operator == Operator::Trace).unwrap();
        assert!(trace.active);
    }

    #[test]
    fn turbo_only_high_mass() {
        let mut lim = lim_with_mass(0.75);
        lim.continuity_score = 0.65;
        let cfg = compute_depth_config(&lim);
        assert!(cfg.turbo_allowed);

        let lim_low = lim_with_mass(0.50);
        let cfg_low = compute_depth_config(&lim_low);
        assert!(!cfg_low.turbo_allowed);
    }
}
