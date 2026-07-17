// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAYER2/HCP_PRO_L2.RS — HCP_PRO Orkestrim me Presion (Layer 2)       ║
// ║  GJATA LEGACY™ — ESS-MAI Quantum                          (LAYER 2)    ║
// ║                                                                          ║
// ║  Në Layer 2, HCP_PRO lidh vendimin e orkestrimit (L1) me presionin       ║
// ║  e sondës abisale (abyssal_probe). Controller-i përshtat urdhrat sipas   ║
// ║  rezultatit të presionit: kandidatët që mbijetojnë presionin marrin      ║
// ║  "thellohu", ata që thyhen marrin "stop".                              ║
// ║                                                                          ║
// ║  KUFIRI: HCP_PRO orkestron midis shtresave; s'jep verdikt. Çdo vendim    ║
// ║  i përshtatur vuloset. Zero if/else klasik.                           ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::hcp_pro::{HcpApplicationResult, HcpPro, OrchestrationDecision, OperatorOrder};
use crate::hw_real::territories::PressureEnvelope;
use crate::lim_types::Operator;
use crate::sovereign::lgc_gate::SovereignGate;
use crate::layer2::abyssal_probe::{ProbeResult, TruthVerdict};

// ─────────────────────────────────────────────────────────────────────────────
// PRESSURE-ADJUSTED DECISION — vendimi i përshtatur me presion
// ─────────────────────────────────────────────────────────────────────────────

/// Vendimi i orkestrimit i përshtatur sipas rezultatit të presionit.
#[derive(Debug, Clone)]
pub struct PressureAdjustedDecision {
    /// Vendimi bazë i orkestrimit (nga L1).
    pub base:           OrchestrationDecision,
    /// A mbijetoi kandidati presionin.
    pub survived:       bool,
    /// Urdhri final pas përshtatjes me presion.
    pub final_order:    OperatorOrder,
}

// ─────────────────────────────────────────────────────────────────────────────
// HCP_PRO LAYER 2 — orkestrim me presion
// ─────────────────────────────────────────────────────────────────────────────

pub struct HcpProL2;

impl HcpProL2 {
    /// orchestrate_with_pressure — orkestrim i përshtatur me rezultatin e presionit.
    ///
    /// Lidh vendimin bazë (L1) me sondën abisale (L2). Kandidatët që mbijetojnë
    /// presionin marrin "thellohu"; ata që thyhen marrin "stop". Zero if — match.
    pub fn orchestrate_with_pressure(
        gate: &mut SovereignGate,
        envelope: &PressureEnvelope,
        operator_signals: &[(Operator, f32)],
        probe: &ProbeResult,
    ) -> PressureAdjustedDecision {
        // Vendimi bazë nga L1 (hardware + operatorët).
        let base = HcpPro::orchestrate(gate, envelope, operator_signals);

        // A mbijetoi kandidati presionin? (nga abyssal_probe).
        let survived = match probe.verdict {
            TruthVerdict::Holds  => true,
            TruthVerdict::Breaks => false,
        };

        // Urdhri final: edhe nëse L1 tha GoDeeper, presioni i thyer → Stop.
        // Zero if — match mbi (survived, base_says_deeper).
        let base_deeper = Self::base_says_deeper(&base);
        let final_order = match (survived, base_deeper) {
            (true, true)  => OperatorOrder::GoDeeper,  // mbijetoi + L1 thotë thellohu
            _             => OperatorOrder::Stop,       // ndryshe ndalo
        };

        PressureAdjustedDecision {
            base,
            survived,
            final_order,
        }
    }

    /// Production path: konsumon vendimin HCP tashmë të vulosur/aplikuar.
    /// Nuk mint-on token tjetër dhe nuk ndryshon lease-in; vetëm lidh rezultatin
    /// e Abyssal Probe me admission-in e operatorëve para PRO-së.
    pub fn adjust_applied_with_pressure(
        application: &HcpApplicationResult,
        probe: &ProbeResult,
    ) -> PressureAdjustedDecision {
        let mut base = application.decision.clone();
        base.hw_directive = application.applied_directive;
        let survived = match probe.verdict {
            TruthVerdict::Holds => true,
            TruthVerdict::Breaks => false,
        };
        let has_admitted_operator = application.receipt.operator_mask_after != 0;
        let has_runtime_capacity = application.lease_after.active
            & (application.lease_after.granted_bytes > 0);
        let final_order = match (
            survived,
            has_admitted_operator,
            has_runtime_capacity,
            base.sealed,
        ) {
            (true, true, true, true) => OperatorOrder::GoDeeper,
            _ => OperatorOrder::Stop,
        };
        PressureAdjustedDecision {
            base,
            survived,
            final_order,
        }
    }

    /// base_says_deeper — a thotë vendimi bazë "thellohu" (ndonjë operator)? Zero if.
    fn base_says_deeper(base: &OrchestrationDecision) -> bool {
        // Të paktën një operator mori GoDeeper.
        let go_count = HcpPro::count_go_deeper(base);
        go_count > 0
    }

    /// directs_deeper — a drejton vendimi final drejt thellimit? Zero if — match.
    pub fn directs_deeper(decision: &PressureAdjustedDecision) -> bool {
        match decision.final_order {
            OperatorOrder::GoDeeper => true,
            OperatorOrder::Stop     => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hw_real::territories::DepthHint;
    use crate::layer2::abyssal_probe::ProbeDepth;
    use crate::lab_contracts::pressure::{PressureReport, PressureSource};

    fn envelope() -> PressureEnvelope {
        PressureEnvelope {
            locked_bytes: 1_000_000, released_bytes: 500_000, total_reserved: 1_500_000,
            demand_bytes: 800_000, under_pressure: false, depth_hint: DepthHint::Deep, ts_ns: 100,
        }
    }

    fn probe(holds: bool) -> ProbeResult {
        let verdict = match holds {
            true  => TruthVerdict::Holds,
            false => TruthVerdict::Breaks,
        };
        ProbeResult {
            claim_id: 1,
            depth_reached: ProbeDepth::Abyssal,
            verdict,
            pressure_report: PressureReport::none(1, PressureSource::QuantumNpro),
            survived_to_limit: holds,
        }
    }

    #[test]
    fn survived_and_optimal_goes_deeper() {
        let mut gate = SovereignGate::new();
        let signals = vec![(Operator::Reverse, 0.8), (Operator::Contrast, 0.9)];
        let decision = HcpProL2::orchestrate_with_pressure(&mut gate, &envelope(), &signals, &probe(true));
        // Mbijetoi presionin + optimal → thellohu.
        assert!(decision.survived);
        assert_eq!(decision.final_order, OperatorOrder::GoDeeper);
    }

    #[test]
    fn broke_pressure_stops_despite_optimal() {
        let mut gate = SovereignGate::new();
        let signals = vec![(Operator::Reverse, 0.9)];
        // Edhe me sinjal të lartë, presioni i thyer → Stop.
        let decision = HcpProL2::orchestrate_with_pressure(&mut gate, &envelope(), &signals, &probe(false));
        assert!(!decision.survived);
        assert_eq!(decision.final_order, OperatorOrder::Stop);
    }

    #[test]
    fn minimal_signal_stops_even_if_survived() {
        let mut gate = SovereignGate::new();
        let signals = vec![(Operator::Mirror, 0.2)]; // nën prag
        let decision = HcpProL2::orchestrate_with_pressure(&mut gate, &envelope(), &signals, &probe(true));
        // Mbijetoi por s'ka optimal → Stop.
        assert_eq!(decision.final_order, OperatorOrder::Stop);
    }

    #[test]
    fn base_decision_sealed() {
        let mut gate = SovereignGate::new();
        let signals = vec![(Operator::Reverse, 0.8)];
        let decision = HcpProL2::orchestrate_with_pressure(&mut gate, &envelope(), &signals, &probe(true));
        // Vendimi bazë u vulos me token.
        assert!(decision.base.sealed);
    }

    fn applied_decision() -> HcpApplicationResult {
        use crate::hw_real::{HardwareRuntimeContext, HwManager};

        let mut runtime = HardwareRuntimeContext::capture(0xA451, DepthHint::Shallow);
        let env = runtime.environment();
        let lease = HwManager::govern(
            env.ram_total_bytes,
            env.ram_free_bytes,
            256 * 1024,
            env.cores_active,
            2,
        );
        runtime.install_initial_lease(lease);
        let signals = [
            (Operator::Reverse, 0.90),
            (Operator::Mirror, 0.85),
        ];
        let pressure = runtime.pressure_envelope();
        let mut gate = SovereignGate::new();
        HcpPro::orchestrate_and_apply(
            &mut gate,
            &mut runtime,
            &pressure,
            &signals,
            false,
        )
    }

    #[test]
    fn applied_and_survived_keeps_pro_admission() {
        let application = applied_decision();
        let decision = HcpProL2::adjust_applied_with_pressure(&application, &probe(true));
        assert!(decision.base.sealed);
        assert!(decision.survived);
        assert!(HcpProL2::directs_deeper(&decision));
    }

    #[test]
    fn applied_but_broken_probe_closes_pro_admission() {
        let application = applied_decision();
        let decision = HcpProL2::adjust_applied_with_pressure(&application, &probe(false));
        assert!(decision.base.sealed);
        assert!(!decision.survived);
        assert!(!HcpProL2::directs_deeper(&decision));
    }
}
