// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  HCP_PRO.RS — Push Hardware Controller Primitive Reasoning Origin    ║
// ║  GJATA LEGACY™ — ESS-MAI Quantum                          (LAYER 1)    ║
// ║                                                                          ║
// ║  HCP_PRO është CONTROLLER AKTIV i orkestrimit. Orientohet nga HPRO       ║
// ║  (që mban harduerin) dhe i flet HPRO + operatorëve të PRO-ve:           ║
// ║   • "Jep më shumë territor RAM" / "ule territorin"                      ║
// ║   • "Ngri procesorin" / "ule procesorin"                              ║
// ║   • Operatorit me OPTIMAL → "vazhdo, thellohu"                         ║
// ║   • Operatorit me MINIMAL → "ndalo, stop"                             ║
// ║                                                                          ║
// ║  RREGULL #0 (PËRFORCO, MOS RINDËRTO):                                 ║
// ║  Territori ËSHTË tashmë i tokenizuar te hw_real/territories.rs          ║
// ║  (ResourceTerritory/ReserveTerritory/PressureTerritory). HCP_PRO NUK     ║
// ║  i rishkruan — është SHTRESA SIPËR tyre që merr VENDIME orkestrimi.     ║
// ║                                                                          ║
// ║  ÇFARË ËSHTË REALISHT E RE: logjika "kujt operatori çfarë t'i thuash,    ║
// ║  bazuar te presioni" + vulosja e çdo vendimi si rresht i përhershëm     ║
// ║  auditimi (mint+burn token nga SovereignGate lokal).                   ║
// ║                                                                          ║
// ║  KUFIRI: HCP_PRO ORKESTRON, s'arsyeton vetë, s'jep verdikt. Token       ║
// ║  mint/burn për çdo vendim (anti-replay). Zero if/else klasik.         ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::hw_real::territories::PressureEnvelope;
use crate::hw_real::{HardwareRuntimeContext, HwLease, HwManager};
use crate::lim_types::Operator;
use crate::sovereign::lgc_gate::{SovereignGate, CapHandle};

// Token seal për vendimet e HCP_PRO (FNV i modulit).
const S_HCP_PRO: u64 = 0x4843_5050_524F_0001; // "HCPPRO" + version

// ─────────────────────────────────────────────────────────────────────────────
// HARDWARE DIRECTIVE — urdhri që HCP_PRO i jep HPRO-s për harduerin
// ─────────────────────────────────────────────────────────────────────────────

/// Urdhri i harduerit (HCP_PRO → HPRO).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HardwareDirective {
    /// Jep më shumë territor RAM + ngri procesorin (kandidatë premtues).
    PushDeeper,
    /// Mbaj territorin aktual (gjendje stabile).
    Hold,
    /// Ule territorin + ule procesorin (nën presion ose pak premtues).
    PullBack,
}

impl HardwareDirective {
    pub fn label(self) -> &'static str {
        match self {
            HardwareDirective::PushDeeper => "PUSH_DEEPER",
            HardwareDirective::Hold       => "HOLD",
            HardwareDirective::PullBack    => "PULL_BACK",
        }
    }

    /// ram_multiplier — sa territor RAM (relativ). Zero if — match.
    pub fn ram_multiplier(self) -> f32 {
        match self {
            HardwareDirective::PushDeeper => 1.25,
            HardwareDirective::Hold       => 1.00,
            HardwareDirective::PullBack    => 0.75,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// OPERATOR ORDER — urdhri që HCP_PRO i jep një operatori
// ─────────────────────────────────────────────────────────────────────────────

/// Urdhri për një operator (HCP_PRO → PRO/NPRO/HPRO/APRO operator).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorOrder {
    /// Vazhdo më thellë (ka optimal).
    GoDeeper,
    /// Ndalo (ka minimal).
    Stop,
}

impl OperatorOrder {
    pub fn label(self) -> &'static str {
        match self {
            OperatorOrder::GoDeeper => "GO_DEEPER",
            OperatorOrder::Stop     => "STOP",
        }
    }

    /// bit — 1=thellohu, 0=stop. Zero if — match.
    pub fn bit(self) -> u8 {
        match self {
            OperatorOrder::GoDeeper => 1,
            OperatorOrder::Stop     => 0,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ORCHESTRATION DECISION — vendimi i plotë i HCP_PRO (i vulosur)
// ─────────────────────────────────────────────────────────────────────────────

/// Vendimi i një operatori, i lidhur me operatorin e tij.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OperatorDecision {
    pub operator: Operator,
    pub order:    OperatorOrder,
    /// Sinjali optimal që çoi te vendimi [0,1].
    pub signal:   f32,
}

/// Vendimi i plotë i orkestrimit (hardware + operatorët), i vulosur me token.
#[derive(Debug, Clone)]
pub struct OrchestrationDecision {
    /// Urdhri për harduerin (drejt HPRO).
    pub hw_directive:   HardwareDirective,
    /// Urdhrat për secilin operator.
    pub operator_orders: Vec<OperatorDecision>,
    /// A u vulos vendimi (token mint+burn i suksesshëm).
    pub sealed:         bool,
    /// Gjenerimi i token-it (për auditim).
    pub token_gen:      u64,
}

/// Receipt i aplikimit real të direktivës mbi lease-in e workload-it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HcpApplicationReceipt {
    pub cycle_id: u64,
    pub requested: HardwareDirective,
    pub applied: HardwareDirective,
    pub lease_before_id: u64,
    pub lease_after_id: u64,
    pub bytes_before: u64,
    pub bytes_after: u64,
    pub parallel_before: u32,
    pub parallel_after: u32,
    pub operator_mask_before: u8,
    pub operator_mask_after: u8,
    pub sealed: bool,
    pub changed: bool,
    pub flexed: bool,
}

/// Vendimi i vulosur + efekti real mbi lease.
#[derive(Debug, Clone)]
pub struct HcpApplicationResult {
    pub decision: OrchestrationDecision,
    pub lease_before: HwLease,
    pub lease_after: HwLease,
    pub applied_directive: HardwareDirective,
    pub receipt: HcpApplicationReceipt,
}

// ─────────────────────────────────────────────────────────────────────────────
// HCP_PRO — controller aktiv i orkestrimit
// ─────────────────────────────────────────────────────────────────────────────

/// Pragu i optimal-it: mbi këtë → GoDeeper.
pub const HCP_OPTIMAL_THRESHOLD: f32 = 0.55;

pub struct HcpPro;

impl HcpPro {
    /// decide_hardware — vendos urdhrin e harduerit nga presioni + thellësia.
    ///
    /// Lexon PressureEnvelope EKZISTUES (s'e rishkruan). Nën presion → PullBack;
    /// thellë pa presion → PushDeeper. Zero if — match mbi (under_pressure, deep).
    /// decide_hardware — vendos urdhrin hardware nga presioni RAM/CPU + termik.
    ///
    /// INTEGRIM TERMIK (      ): para lexonte vetëm PressureEnvelope (RAM/CPU);
    /// tani edhe gjendjen termike. thermal_hot vjen nga SensorMathHarduer::needs_action.
    /// under_pressure = presion RAM/CPU OSE termik → tërheqje graduale (jo vetëm
    /// "frenë urgjence" e Cold Heat Manager). Zero if — match ekzistues i zgjeruar.
    pub fn decide_hardware(envelope: &PressureEnvelope, thermal_hot: bool) -> HardwareDirective {
        // Presion efektiv = RAM/CPU OSE termik (warning para Emergency → graduale).
        let under_pressure = envelope.under_pressure || thermal_hot;
        let is_deep = envelope.depth_hint.is_deep();

        // Match mbi (nën_presion, thellë) — zero if.
        match (under_pressure, is_deep) {
            // Nën presion (RAM/CPU ose termik) → tërhiqu, pavarësisht thellësisë.
            (true, _)      => HardwareDirective::PullBack,
            // Pa presion + thellë → shtyj më thellë.
            (false, true)  => HardwareDirective::PushDeeper,
            // Pa presion + jo thellë → mbaj.
            (false, false) => HardwareDirective::Hold,
        }
    }

    /// decide_operator — vendos urdhrin për një operator nga sinjali optimal.
    ///
    /// Optimal mbi prag → GoDeeper; nën prag → Stop. Zero if — match mbi krahasimin.
    pub fn decide_operator(operator: Operator, optimal_signal: f32) -> OperatorDecision {
        let above = optimal_signal >= HCP_OPTIMAL_THRESHOLD;
        // Match mbi above — zero if.
        let order = match above {
            true  => OperatorOrder::GoDeeper,
            false => OperatorOrder::Stop,
        };
        OperatorDecision {
            operator,
            order,
            signal: optimal_signal.clamp(0.0, 1.0),
        }
    }

    /// orchestrate — vendimi i plotë: hardware + operatorët, i vulosur me token.
    ///
    /// 1. Vendos urdhrin e harduerit nga presioni.
    /// 2. Vendos urdhër për secilin operator nga sinjalet.
    /// 3. MINT+BURN token nga SovereignGate (vendim i vulosur, anti-replay).
    /// Zero if — match.
    pub fn orchestrate(
        gate: &mut SovereignGate,
        envelope: &PressureEnvelope,
        operator_signals: &[(Operator, f32)],
    ) -> OrchestrationDecision {
        // Rruga historike (thermal_hot=false) — mbahet për pas-pajtueshmëri.
        Self::orchestrate_thermal(gate, envelope, operator_signals, false)
    }

    /// orchestrate_thermal (v1.4.2) — I NJËJTI vendim, POR me nxehtësinë REALE.
    /// GJETJA e Gardianit: pipeline-i thërriste `orchestrate` që kalonte gjithnjë
    /// `thermal_hot=false` — menaxhimi i harduerit s'reagonte ndaj mbinxehjes.
    /// Direktiva e Arkitektit ("harduer real, 10% i lirë, pa mbinxehje") kërkon
    /// që porta të TËRHIQET kur sensori është i nxehtë, edhe pa presion RAM-i.
    /// `decide_hardware(env, thermal_hot)` tashmë e pranonte parametrin — tani
    /// ushqehet realisht. Zero if — match.
    pub fn orchestrate_thermal(
        gate: &mut SovereignGate,
        envelope: &PressureEnvelope,
        operator_signals: &[(Operator, f32)],
        thermal_hot: bool,
    ) -> OrchestrationDecision {
        // 1. Urdhri i harduerit — nga presioni RAM OSE nxehtësia reale.
        let hw_directive = Self::decide_hardware(envelope, thermal_hot);

        // 2. Urdhër për secilin operator (zero if — map).
        let operator_orders: Vec<OperatorDecision> = operator_signals.iter()
            .map(|(op, sig)| Self::decide_operator(*op, *sig))
            .collect();

        // 3. MINT token për këtë vendim.
        let handle: CapHandle = gate.issue(S_HCP_PRO);
        let token_gen = handle.gen;

        // BURN token (CAS single-use) — vulos vendimin. Zero if — match.
        let sealed = match gate.validate(handle) {
            Ok(_token)  => true,   // token konsumuar → vendim i vulosur
            Err(_)      => false,  // dështoi (s'duhet ndodhë për handle të ri)
        };

        OrchestrationDecision {
            hw_directive,
            operator_orders,
            sealed,
            token_gen,
        }
    }

    fn operator_bit(operator: Operator) -> u8 {
        match operator {
            Operator::Reverse => 0x01,
            Operator::Mirror => 0x02,
            Operator::Contrast => 0x04,
            Operator::Trace => 0x08,
        }
    }

    /// Përkthen urdhrat e vulosur në operator admission real. PullBack mban
    /// vetëm sinjale shumë të forta; lease-i vendos edhe kufirin numerik.
    fn admission_mask(
        decision: &OrchestrationDecision,
        directive: HardwareDirective,
        parallel_limit: u32,
    ) -> u8 {
        let threshold = match directive {
            HardwareDirective::PullBack => 0.80,
            HardwareDirective::Hold | HardwareDirective::PushDeeper => HCP_OPTIMAL_THRESHOLD,
        };
        let limit = usize::try_from(parallel_limit.max(1)).unwrap_or(usize::MAX);
        decision
            .operator_orders
            .iter()
            .filter(|order| {
                (order.order == OperatorOrder::GoDeeper) & (order.signal >= threshold)
            })
            .take(limit)
            .fold(0u8, |mask, order| mask | Self::operator_bit(order.operator))
    }

    /// orchestrate_and_apply — vendimi HCP_PRO bëhet state transition real.
    ///
    /// `ram_multiplier()` dhe cores reale ushqejnë HwManager::adjust; runtime-i
    /// ruan lease-in e ri. Vendimi i pavulosur nuk ndryshon state-in.
    pub fn orchestrate_and_apply(
        gate: &mut SovereignGate,
        runtime: &mut HardwareRuntimeContext,
        envelope: &PressureEnvelope,
        operator_signals: &[(Operator, f32)],
        thermal_hot: bool,
    ) -> HcpApplicationResult {
        let blind_deep = (!runtime.measurement_state.has_real_measurement())
            & runtime.depth_hint.requires_hw_gate();
        let effective_thermal_pressure = thermal_hot | blind_deep;
        let decision = Self::orchestrate_thermal(
            gate,
            envelope,
            operator_signals,
            effective_thermal_pressure,
        );
        let lease_before = runtime.lease().unwrap_or_else(|| {
            HwManager::govern(
                runtime.raw_snapshot.ram_total_bytes,
                runtime.raw_snapshot.ram_free_bytes,
                0,
                runtime.raw_snapshot.cores_active.max(1),
                1,
            )
        });

        let multiplier = decision.hw_directive.ram_multiplier();
        let requested_bytes =
            (lease_before.granted_bytes as f64 * multiplier as f64) as u64;
        let requested_parallel = match decision.hw_directive {
            HardwareDirective::PushDeeper => {
                lease_before.granted_parallel.saturating_add(1)
            }
            HardwareDirective::Hold => lease_before.granted_parallel,
            HardwareDirective::PullBack => {
                lease_before.granted_parallel.saturating_sub(1).max(1)
            }
        };

        let lease_after = match decision.sealed {
            true => HwManager::adjust(
                lease_before,
                runtime.environment(),
                requested_bytes,
                requested_parallel,
            ),
            false => lease_before,
        };

        let operator_mask_before = runtime.operator_admission_mask;
        let requested_mask = match (lease_after.active, lease_after.granted_bytes > 0) {
            (true, true) => Self::admission_mask(
                &decision,
                decision.hw_directive,
                lease_after.granted_parallel,
            ),
            _ => 0,
        };
        // Vendim i pavulosur = fail-closed: lease-i nuk ndryshon, por asnjë
        // operator i ri nuk lejohet të procedojë nën një urdhër pa autoritet.
        let operator_mask_after = match decision.sealed {
            true => requested_mask,
            false => 0,
        };
        let mask_narrowed = (operator_mask_after & operator_mask_before)
            != operator_mask_before;

        let applied_directive = match (
            lease_after.granted_bytes.cmp(&lease_before.granted_bytes),
            lease_after
                .granted_parallel
                .cmp(&lease_before.granted_parallel),
            mask_narrowed,
        ) {
            // Çdo ngushtim i burimeve ose admission-it ka përparësi: sistemi
            // nuk raporton PushDeeper kur puna reale u ngushtua.
            (std::cmp::Ordering::Less, _, _)
            | (_, std::cmp::Ordering::Less, _)
            | (_, _, true) => HardwareDirective::PullBack,
            (std::cmp::Ordering::Greater, _, false)
            | (_, std::cmp::Ordering::Greater, false) => HardwareDirective::PushDeeper,
            _ => HardwareDirective::Hold,
        };

        let changed = lease_after.granted_bytes != lease_before.granted_bytes
            || lease_after.granted_parallel != lease_before.granted_parallel
            || operator_mask_after != operator_mask_before;
        let flexed = lease_after.flexed
            || applied_directive != decision.hw_directive;

        runtime.replace_lease(lease_after);
        runtime.set_operator_admission_mask(operator_mask_after);

        let receipt = HcpApplicationReceipt {
            cycle_id: runtime.cycle_id,
            requested: decision.hw_directive,
            applied: applied_directive,
            lease_before_id: lease_before.lease_id,
            lease_after_id: lease_after.lease_id,
            bytes_before: lease_before.granted_bytes,
            bytes_after: lease_after.granted_bytes,
            parallel_before: lease_before.granted_parallel,
            parallel_after: lease_after.granted_parallel,
            operator_mask_before,
            operator_mask_after,
            sealed: decision.sealed,
            changed,
            flexed,
        };

        HcpApplicationResult {
            decision,
            lease_before,
            lease_after,
            applied_directive,
            receipt,
        }
    }

    /// count_go_deeper — sa operatorë morën urdhër GoDeeper. Zero if — filter.
    pub fn count_go_deeper(decision: &OrchestrationDecision) -> usize {
        decision.operator_orders.iter()
            .filter(|d| d.order.bit() == 1)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hw_real::territories::DepthHint;

    fn envelope(under_pressure: bool, depth: DepthHint) -> PressureEnvelope {
        PressureEnvelope {
            locked_bytes: 1_000_000, released_bytes: 500_000, total_reserved: 1_500_000,
            demand_bytes: 800_000, under_pressure, depth_hint: depth, ts_ns: 100,
        }
    }

    #[test]
    fn hardware_pullback_under_pressure() {
        let env = envelope(true, DepthHint::Deep);
        // Nën presion → tërhiqu (pavarësisht thellësisë). thermal_hot=false.
        assert_eq!(HcpPro::decide_hardware(&env, false), HardwareDirective::PullBack);
    }

    #[test]
    fn hardware_push_when_deep_no_pressure() {
        let env = envelope(false, DepthHint::VeryDeep);
        assert_eq!(HcpPro::decide_hardware(&env, false), HardwareDirective::PushDeeper);
    }

    #[test]
    fn hardware_hold_when_shallow() {
        let env = envelope(false, DepthHint::Shallow);
        assert_eq!(HcpPro::decide_hardware(&env, false), HardwareDirective::Hold);
    }

    #[test]
    fn orchestrate_thermal_hot_true_causes_pullback() {
        // Sjellja e kërkuar nga auditi (p.10 test 1): rruga E PLOTË runtime —
        // thermal_hot=true → PullBack, edhe pa presion RAM, edhe Shallow.
        let mut gate = SovereignGate::new();
        let env = PressureEnvelope {
            locked_bytes: 0, released_bytes: 1024, total_reserved: 1024,
            demand_bytes: 0, under_pressure: false,
            depth_hint: crate::hw_real::territories::DepthHint::Shallow,
            ts_ns: 0,
        };
        let d = HcpPro::orchestrate_thermal(&mut gate, &env, &[], true);
        assert!(matches!(d.hw_directive, HardwareDirective::PullBack),
            "nxehtësia reale duhet të tërheqë harduerin, mori {:?}", d.hw_directive);
        assert!(d.sealed, "vendimi mbetet i vulosur me token edhe në tërheqje");
        // Dhe rruga e ftohtë s'e falsifikon tërheqjen:
        let mut gate2 = SovereignGate::new();
        let d2 = HcpPro::orchestrate_thermal(&mut gate2, &env, &[], false);
        assert!(!matches!(d2.hw_directive, HardwareDirective::PullBack),
            "pa nxehtësi e pa presion s'ka tërheqje të rreme");
    }

    #[test]
    fn hardware_pullback_when_thermal_hot_even_without_ram_pressure() {
        // INTEGRIM TERMIK (      ): pa presion RAM/CPU, por termik nxehtë →
        // tërhiqu (warning graduale, jo vetëm frenë urgjence). Zero if.
        let env = envelope(false, DepthHint::VeryDeep);
        // thermal_hot=true mbizotëron: edhe deep+no-pressure → PullBack.
        assert_eq!(HcpPro::decide_hardware(&env, true), HardwareDirective::PullBack);
    }

    #[test]
    fn operator_go_deeper_when_optimal() {
        let d = HcpPro::decide_operator(Operator::Reverse, 0.8);
        assert_eq!(d.order, OperatorOrder::GoDeeper);
    }

    #[test]
    fn operator_stop_when_minimal() {
        let d = HcpPro::decide_operator(Operator::Mirror, 0.3);
        assert_eq!(d.order, OperatorOrder::Stop);
    }

    #[test]
    fn orchestrate_seals_decision() {
        let mut gate = SovereignGate::new();
        let env = envelope(false, DepthHint::Deep);
        let signals = vec![
            (Operator::Reverse, 0.8),
            (Operator::Mirror, 0.3),
            (Operator::Contrast, 0.9),
            (Operator::Trace, 0.4),
        ];
        let decision = HcpPro::orchestrate(&mut gate, &env, &signals);
        // Vendimi u vulos (token mint+burn).
        assert!(decision.sealed);
        // Hardware push (thellë, pa presion).
        assert_eq!(decision.hw_directive, HardwareDirective::PushDeeper);
        // 2 operatorë GoDeeper (Reverse 0.8, Contrast 0.9).
        assert_eq!(HcpPro::count_go_deeper(&decision), 2);
    }

    #[test]
    fn each_decision_gets_unique_token() {
        let mut gate = SovereignGate::new();
        let env = envelope(false, DepthHint::Medium);
        let signals = vec![(Operator::Reverse, 0.7)];
        let d1 = HcpPro::orchestrate(&mut gate, &env, &signals);
        let d2 = HcpPro::orchestrate(&mut gate, &env, &signals);
        // Çdo vendim merr token gen unik (anti-replay).
        assert_ne!(d1.token_gen, d2.token_gen);
        assert!(d1.sealed && d2.sealed);
    }

    #[test]
    fn directive_multipliers() {
        assert_eq!(HardwareDirective::PushDeeper.ram_multiplier(), 1.25);
        assert_eq!(HardwareDirective::PullBack.ram_multiplier(), 0.75);
    }
    #[test]
    fn orchestrate_and_apply_changes_runtime_lease() {
        let mut runtime = HardwareRuntimeContext::capture(0xA451, DepthHint::Deep);
        let initial = HwManager::govern(
            8 * 1024 * 1024 * 1024,
            6 * 1024 * 1024 * 1024,
            512 * 1024 * 1024,
            8,
            2,
        );
        // Testi kontrollon PushDeeper me matje të deklaruar reale; përndryshe
        // Deep + NominalFallback duhet me ligj të detyrojë PullBack.
        runtime.measurement_state = crate::hw_real::MeasurementState::Measured;
        runtime.raw_snapshot.ram_total_bytes = 8 * 1024 * 1024 * 1024;
        runtime.raw_snapshot.ram_free_bytes = 6 * 1024 * 1024 * 1024;
        runtime.raw_snapshot.cores_active = 8;
        runtime.install_initial_lease(initial);

        let mut gate = SovereignGate::new();
        let env = envelope(false, DepthHint::Deep);
        let result = HcpPro::orchestrate_and_apply(
            &mut gate,
            &mut runtime,
            &env,
            &[(Operator::Reverse, 0.9)],
            false,
        );
        assert!(result.receipt.sealed);
        assert!(result.lease_after.granted_parallel >= result.lease_before.granted_parallel);
        assert_eq!(runtime.lease(), Some(result.lease_after));
    }


    fn runtime_for_test(
        state: crate::hw_real::MeasurementState,
        depth: DepthHint,
        bytes: u64,
        parallel: u32,
    ) -> HardwareRuntimeContext {
        let mut runtime = HardwareRuntimeContext::capture(0xA452, depth);
        runtime.measurement_state = state;
        runtime.depth_hint = depth;
        runtime.raw_snapshot.ram_total_bytes = 8 * 1024 * 1024 * 1024;
        runtime.raw_snapshot.ram_free_bytes = 6 * 1024 * 1024 * 1024;
        runtime.raw_snapshot.cores_active = 8;
        runtime.install_initial_lease(HwManager::govern(
            8 * 1024 * 1024 * 1024,
            6 * 1024 * 1024 * 1024,
            bytes,
            8,
            parallel,
        ));
        runtime
    }

    #[test]
    fn nominal_deep_runtime_forces_conservative_pullback() {
        let mut runtime = runtime_for_test(
            crate::hw_real::MeasurementState::NominalFallback,
            DepthHint::Deep,
            512 * 1024 * 1024,
            3,
        );
        let mut gate = SovereignGate::new();
        let result = HcpPro::orchestrate_and_apply(
            &mut gate,
            &mut runtime,
            &envelope(false, DepthHint::Deep),
            &[(Operator::Reverse, 0.9), (Operator::Contrast, 0.9)],
            false,
        );
        assert_eq!(result.decision.hw_directive, HardwareDirective::PullBack);
        assert!(result.lease_after.granted_bytes < result.lease_before.granted_bytes);
        assert!(result.lease_after.granted_parallel <= result.lease_before.granted_parallel);
    }

    #[test]
    fn pullback_changes_lease_and_operator_admission() {
        let mut runtime = runtime_for_test(
            crate::hw_real::MeasurementState::Measured,
            DepthHint::Deep,
            512 * 1024 * 1024,
            4,
        );
        let mut gate = SovereignGate::new();
        let signals = [
            (Operator::Reverse, 0.95),
            (Operator::Mirror, 0.60),
            (Operator::Contrast, 0.90),
            (Operator::Trace, 0.40),
        ];
        let result = HcpPro::orchestrate_and_apply(
            &mut gate,
            &mut runtime,
            &envelope(true, DepthHint::Deep),
            &signals,
            false,
        );
        assert_eq!(result.applied_directive, HardwareDirective::PullBack);
        assert!(result.receipt.changed);
        assert!(result.receipt.bytes_after < result.receipt.bytes_before);
        assert!(result.receipt.parallel_after < result.receipt.parallel_before);
        assert_eq!(result.receipt.operator_mask_after, 0x05);
    }

    #[test]
    fn hold_revalidates_without_changing_grant() {
        let mut runtime = runtime_for_test(
            crate::hw_real::MeasurementState::Measured,
            DepthHint::Shallow,
            256 * 1024 * 1024,
            2,
        );
        let mut gate = SovereignGate::new();
        let result = HcpPro::orchestrate_and_apply(
            &mut gate,
            &mut runtime,
            &envelope(false, DepthHint::Shallow),
            &[(Operator::Reverse, 0.9), (Operator::Mirror, 0.9)],
            false,
        );
        assert_eq!(result.decision.hw_directive, HardwareDirective::Hold);
        assert_eq!(result.receipt.bytes_before, result.receipt.bytes_after);
        assert_eq!(result.receipt.parallel_before, result.receipt.parallel_after);
    }

    #[test]
    fn zero_capacity_closes_operator_admission_even_when_signals_are_strong() {
        let mut runtime = runtime_for_test(
            crate::hw_real::MeasurementState::Measured,
            DepthHint::Shallow,
            0,
            2,
        );
        // Simulo mjedis pa usable RAM për riverifikimin e lease-it.
        runtime.raw_snapshot.ram_free_bytes = 0;
        let mut gate = SovereignGate::new();
        let result = HcpPro::orchestrate_and_apply(
            &mut gate,
            &mut runtime,
            &envelope(false, DepthHint::Shallow),
            &[(Operator::Reverse, 0.99), (Operator::Mirror, 0.99)],
            false,
        );
        assert_eq!(result.lease_after.granted_bytes, 0);
        assert_eq!(result.receipt.operator_mask_after, 0);
        assert_eq!(result.applied_directive, HardwareDirective::PullBack);
    }

}
