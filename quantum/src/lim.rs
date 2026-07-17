// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LIM.RS — LINEAR INFO METRIC — ORCHESTRATORI                          ║
// ║  GJATA LEGACY™ — Quantum Platform                                     ║
// ║                                                                          ║
// ║  Lidh Grupin 1 (Collector) + Grupin 2 (Measurer).                     ║
// ║  Ekspozon entry-points e pseudokodit:                                 ║
// ║   • pre_pro_scan()         → EpistemicScaffold (faza 1, për PRO)      ║
// ║   • post_quantum_classify()→ EpistemicClassification (faza 2, Shadow) ║
// ║                                                                          ║
// ║  RRJEDHA E DY GRUPEVE:                                                 ║
// ║   collect() × N turne  →  has_sufficient_material()  →  measure()     ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::lim_types::*;
use crate::lim_collector::LimCollector;
use crate::lim_measurer::LimMeasurer;
use crate::lim_classifier::{classify_epistemic_state, compute_operator_gates, compute_depth_config};

// ─────────────────────────────────────────────────────────────────────────────
// LINEAR INFO METRIC — moduli i plotë (dy grupet bashkë)
// ─────────────────────────────────────────────────────────────────────────────

pub struct LinearInfoMetricEngine {
    collector: LimCollector,
}

impl LinearInfoMetricEngine {
    pub fn new(session_id: &str, territory: &str) -> Self {
        Self {
            collector: LimCollector::new(session_id, territory),
        }
    }

    // ═════════════════════════════════════════════════════════════════════
    // GRUPI 1 — akumulim ndër-turn
    // ═════════════════════════════════════════════════════════════════════

    /// Mbledh një fragment të ri (një turn). NUK mat.
    pub fn collect(&mut self, fragment: &str) {
        self.collector.collect(fragment);
    }

    /// A ka material të mjaftueshëm për të matur?
    pub fn ready_to_measure(&self) -> bool {
        self.collector.has_sufficient_material()
    }

    // ═════════════════════════════════════════════════════════════════════
    // FAZA 1 — pre_pro_scan() (Grupi 2 mat → scaffold për PRO)
    // Nga: FUNCTION pre_pro_scan (lim_core.pseudo)
    // ═════════════════════════════════════════════════════════════════════

    /// Mat grumbullimin → EpistemicScaffold (hyrja për PRO).
    ///
    /// Error-handling: nëse s'ka material të mjaftueshëm → Err.
    /// (Në prod: LGC seal verify do shtohej këtu para çdo gjëje.)
    pub fn pre_pro_scan(&self) -> Result<EpistemicScaffold, LimError> {
        match self.collector.is_empty() {
            true => return Err(LimError::EmptyInput),
            false => {}
        }
        match self.collector.has_sufficient_material() {
            false => return Err(LimError::InsufficientMaterial),
            true => {}
        }

        // ── Grupi 2 mat 8 akset mbi GRUMBULLIMIN ───────────────────────────
        let lim = LimMeasurer::measure(
            self.collector.fragments(),
            self.collector.territory(),
            self.collector.session_id(),
        );

        // ── Gates + depth për PRO ──────────────────────────────────────────
        let operator_gates = compute_operator_gates(&lim);
        let depth_config = compute_depth_config(&lim);

        let ready_for_pro = lim.epistemic_mass >= 0.20;
        let reconstruction_needed = lim.information_density < 0.35;

        Ok(EpistemicScaffold {
            lim,
            operator_gates,
            depth_config,
            ready_for_pro,
            reconstruction_needed,
        })
    }

    // ═════════════════════════════════════════════════════════════════════
    // FAZA 2 — post_quantum_classify()
    // Nga: FUNCTION post_quantum_classify (lim_core.pseudo)
    //
    // Merr output-in e Quantum (PRO+SRK), ri-mat, klasifikon EpistemicState.
    // Këtu thjeshtësuar: pranon fragmentet e arsyetimit të Quantum si tekst.
    // (Integrimi i plotë me QuantumOutput vjen kur lidhim Light+Quantum+Shadow.)
    // ═════════════════════════════════════════════════════════════════════

    /// Klasifikon output-in e Quantum → EpistemicClassification (për Shadow).
    pub fn post_quantum_classify(
        &self,
        pre: &EpistemicScaffold,
        quantum_reasoning: &[String],
    ) -> Result<EpistemicClassification, LimError> {
        match quantum_reasoning.is_empty() {
            true => return Err(LimError::EmptyInput),
            false => {}
        }

        // Ri-mat mbi output-in e Quantum-it.
        let mut lim_post = LimMeasurer::measure(
            quantum_reasoning,
            &pre.lim.territory,
            &pre.lim.session_id,
        );
        lim_post.phase = LimPhase::PostQuantum;

        // Klasifiko gjendjen epistemike.
        let state = classify_epistemic_state(&lim_post);
        lim_post.epistemic_state = Some(state.clone());

        let delta = (lim_post.epistemic_mass - pre.lim.epistemic_mass).abs();
        let shadow_note = build_shadow_note(&state, &lim_post);

        Ok(EpistemicClassification {
            lim_pre: pre.lim.clone(),
            lim_post: lim_post.clone(),
            state,
            delta,
            session_id: lim_post.session_id.clone(),
            territory: lim_post.territory.clone(),
            ready_for_shadow: true,
            shadow_note,
        })
    }

    /// Akses te collector (për test/raport).
    pub fn collector(&self) -> &LimCollector {
        &self.collector
    }

    /// Reset (fund sesioni — ephemeral).
    pub fn reset(&mut self) {
        self.collector.reset();
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// SHADOW NOTE — shënimi që shoqëron klasifikimin drejt Shadow
// Nga: FUNCTION build_shadow_note (lim_analyzers.pseudo)
// ─────────────────────────────────────────────────────────────────────────────

fn build_shadow_note(state: &EpistemicState, lim: &LinearInfoMetric) -> String {
    format!(
        "LIM→Shadow: state={} mass={:.3} integrity={:.2} pro_mode={}",
        state.kind_str(),
        lim.epistemic_mass,
        lim.semantic_integrity,
        state.pro_mode(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_flow_collect_then_scan() {
        let mut lim = LinearInfoMetricEngine::new("s1", "science");
        // Grupi 1: akumulon ndër-turn
        lim.collect("The system uses deterministic elimination instead of generation");
        lim.collect("It scores epistemic mass across eight axes");
        lim.collect("Knowledge that survives becomes primitive knowledge");

        assert!(lim.ready_to_measure());

        // Grupi 2: mat
        let scaffold = lim.pre_pro_scan().expect("should measure");
        assert!(scaffold.lim.epistemic_mass > 0.0);
        // Duhet të ketë të paktën TRACE aktiv
        assert!(scaffold.active_operators().iter()
            .any(|g| g.operator == Operator::Trace));
    }

    #[test]
    fn insufficient_material_errors() {
        let mut lim = LinearInfoMetricEngine::new("s1", "x");
        lim.collect("short"); // 1 fragment, 1 fjalë
        assert_eq!(lim.pre_pro_scan().unwrap_err(), LimError::InsufficientMaterial);
    }

    #[test]
    fn empty_errors() {
        let lim = LinearInfoMetricEngine::new("s1", "x");
        assert_eq!(lim.pre_pro_scan().unwrap_err(), LimError::EmptyInput);
    }

    #[test]
    fn post_quantum_classifies() {
        let mut lim = LinearInfoMetricEngine::new("s1", "science");
        lim.collect("quantum reasoning eliminates wrong paths deterministically");
        lim.collect("the shadow authority verifies with binary zero or one");
        let pre = lim.pre_pro_scan().unwrap();

        let quantum_out = vec![
            "REVERSE found the causal origin of the instability".to_string(),
            "TRACE filled the gap between premise and conclusion".to_string(),
        ];
        let cls = lim.post_quantum_classify(&pre, &quantum_out).unwrap();
        assert!(cls.ready_for_shadow);
        assert_eq!(cls.lim_post.phase, LimPhase::PostQuantum);
        assert!(cls.lim_post.epistemic_state.is_some());
    }
}
