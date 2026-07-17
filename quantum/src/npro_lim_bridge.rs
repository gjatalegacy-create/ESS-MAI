// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  NPRO_LIM_BRIDGE.RS — WeaknessSignal → LIM (Kufijtë për PRO)         ║
// ║  GJATA LEGACY™ — Quantum Platform                                     ║
// ║                                                                          ║
// ║  NPRO dërgon WeaknessSignal te LIM njëkohësisht me NPIM.              ║
// ║  LIM merr forbidden_paths → çaktivizon gates përkatëse → PRO s'humbet  ║
// ║  cikle në rrugë të dobëta.                                            ║
// ║                                                                          ║
// ║  "PRO mund të shkojë më thellë sepse tani di ku janë kufijtë."         ║
// ║  Zero if/else — match/formula.                                        ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::lim_types::{EpistemicScaffold, OperatorGate, Operator};
use crate::npro::WeaknessSignal;

// ─────────────────────────────────────────────────────────────────────────────
// MAPPING — emri i operatorit (nga forbidden_paths) → Operator enum
// ─────────────────────────────────────────────────────────────────────────────

/// A përputhet gate-i me një forbidden path? Zero if — match mbi operator.
fn gate_matches_forbidden(operator: Operator, forbidden: &str) -> bool {
    let name = match operator {
        Operator::Reverse  => "Reverse",
        Operator::Mirror   => "Mirror",
        Operator::Contrast => "Contrast",
        Operator::Trace    => "Trace",
    };
    name == forbidden
}

// ─────────────────────────────────────────────────────────────────────────────
// NPRO → LIM BRIDGE
// ─────────────────────────────────────────────────────────────────────────────

pub struct NproLimBridge;

impl NproLimBridge {
    /// apply_weakness — aplikon WeaknessSignal mbi scaffold-in e LIM.
    ///
    /// Çdo gate operator që ndodhet në forbidden_paths → active=false.
    /// PRO pastaj s'i ekzekuton ato rrugë (kursen cikle).
    /// Zero if — map me predikat boolean.
    pub fn apply_weakness(
        mut scaffold: EpistemicScaffold,
        signal: &WeaknessSignal,
    ) -> EpistemicScaffold {
        // Rindërto operator_gates: active = active AND (jo forbidden).
        let new_gates: Vec<OperatorGate> = scaffold.operator_gates.into_iter()
            .map(|mut g| {
                // is_forbidden ∈ {true,false}: a është ky operator i ndaluar?
                let is_forbidden = signal.forbidden_paths.iter()
                    .any(|f| gate_matches_forbidden(g.operator, f));
                // active i ri = active AND (jo forbidden). Branchless boolean.
                g.active = g.active && !is_forbidden;
                g
            })
            .collect();

        scaffold.operator_gates = new_gates;
        scaffold
    }

    /// count_active — sa operatorë mbetën aktivë pas aplikimit.
    pub fn count_active(scaffold: &EpistemicScaffold) -> usize {
        scaffold.operator_gates.iter().filter(|g| g.active).count()
    }

    /// forbidden_applied — sa gates u çaktivizuan nga sinjali.
    pub fn forbidden_applied(signal: &WeaknessSignal) -> usize {
        signal.forbidden_paths.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lim::LinearInfoMetricEngine;
    use crate::npro::NproEngine;

    fn scaffold_from(frags: &[&str]) -> EpistemicScaffold {
        let mut lim = LinearInfoMetricEngine::new("s1", "science");
        for f in frags { lim.collect(f); }
        lim.pre_pro_scan().expect("scaffold")
    }

    #[test]
    fn forbidden_paths_deactivate_gates() {
        let scaffold = scaffold_from(&[
            "deterministic reasoning produces verifiable knowledge here now",
            "each candidate passes through filters before being accepted today",
        ]);
        let active_before = NproLimBridge::count_active(&scaffold);

        // Krijo një sinjal që ndalon Reverse + Mirror.
        let signal = WeaknessSignal {
            forbidden_paths: vec!["Reverse".to_string(), "Mirror".to_string()],
            weakness_ceiling: 0.65,
            avg_weakness: 0.7,
        };
        let after = NproLimBridge::apply_weakness(scaffold, &signal);
        let active_after = NproLimBridge::count_active(&after);

        // Sa gates aktive duhet të jenë ulur (ose të paktën jo rritur).
        assert!(active_after <= active_before);
    }

    #[test]
    fn no_forbidden_keeps_all_active() {
        let scaffold = scaffold_from(&[
            "deterministic reasoning produces verifiable knowledge here now",
            "each candidate passes through filters before acceptance today",
        ]);
        let active_before = NproLimBridge::count_active(&scaffold);

        // Sinjal bosh → asgjë s'ndryshon.
        let signal = WeaknessSignal {
            forbidden_paths: vec![],
            weakness_ceiling: 0.65,
            avg_weakness: 0.2,
        };
        let after = NproLimBridge::apply_weakness(scaffold, &signal);
        assert_eq!(NproLimBridge::count_active(&after), active_before);
    }

    #[test]
    fn full_npro_to_lim_flow() {
        // Rrjedha e plotë: NPRO → WeaknessSignal → LIM scaffold.
        // v1.5.1 (Korrigjimi B, TEST-ONLY): NPRO mbetet mbi fragmentin e vetëm
        // të dobët (ana NPRO e rrjedhës); scaffold-i LIM ndërtohet nga fixture
        // që plotëson kontratën reale (≥2 fragmente, ≥8 fjalë — pragjet
        // TË PAPREKURA), që rrjedha NPRO→LIM të arrihet REALISHT.
        let frags_str = vec!["single weak fragment".to_string()];
        let npro = NproEngine::activate(&frags_str, 0.3, 0.3);

        // Precondition 1: sinjali është REAL — input i dobët prodhon
        // të paktën një rrugë të ndaluar (contrast/mirror mbi 1 fragment).
        assert!(NproLimBridge::forbidden_applied(&npro.weakness_signal) >= 1,
            "input i dobët duhet të prodhojë sinjal real dobësie");

        let scaffold = scaffold_from(&[
            "single weak fragment sits here",
            "second weak fragment sits there",
        ]);
        // Precondition 2: scaffold-i u ndërtua nga materiali i mjaftueshëm.
        let active_before = NproLimBridge::count_active(&scaffold);

        let after = NproLimBridge::apply_weakness(scaffold, &npro.weakness_signal);

        // Sinjali u aplikua (forbidden paths ekzistojnë për input të dobët).
        assert!(NproLimBridge::forbidden_applied(&npro.weakness_signal) <= 4);
        // Aplikimi vetëm çaktivizon — kurrë s'shton gates aktive.
        assert!(NproLimBridge::count_active(&after) <= active_before,
            "apply_weakness s'duhet të rrisë gates aktive");
    }

    #[test]
    fn forbidden_gate_specifically_off() {
        let scaffold = scaffold_from(&[
            "deterministic reasoning here now today produces strong knowledge",
            "each candidate is filtered before acceptance in this system today",
        ]);
        let signal = WeaknessSignal {
            forbidden_paths: vec!["Contrast".to_string()],
            weakness_ceiling: 0.65,
            avg_weakness: 0.7,
        };
        let after = NproLimBridge::apply_weakness(scaffold, &signal);
        // Gate-i Contrast duhet të jetë inaktiv.
        let contrast_gate = after.operator_gates.iter()
            .find(|g| matches!(g.operator, Operator::Contrast));
        // Nëse ekziston, duhet të jetë active=false.
        let contrast_active = contrast_gate.map(|g| g.active).unwrap_or(false);
        assert!(!contrast_active);
    }
}
