// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAYER3/HCP_PRO_L3.RS — HCP_PRO Aktiv Paralel (Layer 3)              ║
// ║  GJATA LEGACY™ — ESS-MAI Quantum                          (LAYER 3)    ║
// ║                                                                          ║
// ║  Në Layer 3, HCP_PRO ngre TË GJITHË operatorët NJËHERESH në reasoning   ║
// ║  eliminues paralel. Ky është funksioni i RI (s'ekziston në L1/L2):     ║
// ║                                                                          ║
// ║  Quantum bëhet AKTIV PARALEL:                                         ║
// ║   • Të gjithë operatorët (PRO/NPRO/HPRO/APRO) arsyetojnë njëkohësisht   ║
// ║   • Gjenden optimal → paketohen → nisen te Shadow                      ║
// ║   • NDËRKOHË që vihen kufijtë hardware (HPRO mban harduerin)           ║
// ║                                                                          ║
// ║  HCP_PRO orientohet nga HPRO (që mban harduerin) dhe ngre/ul çdo        ║
// ║  operator — aktiv, jo pasiv.                                          ║
// ║                                                                          ║
// ║  KUFIRI: HCP_PRO orkestron paralelizmin; s'jep verdikt. Çdo cikël       ║
// ║  paralel vuloset me token. Zero if/else klasik.                       ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::hcp_pro::{HcpApplicationResult, HcpPro, HardwareDirective, OperatorOrder};
use crate::hw_real::territories::PressureEnvelope;
use crate::hw_real::thermal::{ThermalReading, SensorMathHarduer};
use crate::sovereign::lgc_gate::SovereignGate;
use crate::layer3::hpro::HproResult;

// ─────────────────────────────────────────────────────────────────────────────
// PRO FAMILY — familjet e operatorëve që ngrihen paralelisht
// ─────────────────────────────────────────────────────────────────────────────

/// Familjet e PRO-ve që HCP_PRO ngre njëkohësisht në Layer 3.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProFamily {
    Pro,    // operatorët bazë
    Npro,   // operatorët negativë
    Hpro,   // operatorët e thellësisë (mban harduerin)
    Apro,   // operatorët abstraktë
}

impl ProFamily {
    pub fn label(self) -> &'static str {
        match self {
            ProFamily::Pro  => "PRO",
            ProFamily::Npro => "NPRO",
            ProFamily::Hpro => "HPRO",
            ProFamily::Apro => "APRO",
        }
    }
}

/// Sinjali i një familjeje (familja + 4 sinjalet e operatorëve të saj).
#[derive(Debug, Clone)]
pub struct FamilySignals {
    pub family:  ProFamily,
    /// 4 sinjalet optimal të operatorëve [0,1].
    pub signals: [f32; 4],
}

impl FamilySignals {
    pub fn new(family: ProFamily, signals: [f32; 4]) -> Self {
        Self { family, signals }
    }

    /// avg_signal — sinjali mesatar i familjes. Zero if.
    pub fn avg_signal(&self) -> f32 {
        let sum: f32 = self.signals.iter().sum();
        sum / 4.0
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// PARALLEL ACTIVATION — rezultati i ngritjes paralele
// ─────────────────────────────────────────────────────────────────────────────

/// Vendimi për një familje në ekzekutim paralel.
#[derive(Debug, Clone)]
pub struct FamilyActivation {
    pub family:    ProFamily,
    /// Urdhri për familjen (i bazuar te sinjali mesatar).
    pub order:     OperatorOrder,
    /// Sinjali mesatar që çoi te vendimi.
    pub avg:       f32,
}

/// Rezultati i plotë i aktivizimit paralel.
#[derive(Debug, Clone)]
pub struct ParallelActivation {
    /// Urdhri global i harduerit.
    pub hw_directive:  HardwareDirective,
    /// Aktivizimet për secilën familje.
    pub families:      Vec<FamilyActivation>,
    /// Sa familje u ngritën (GoDeeper).
    pub active_count:  usize,
    /// A u vulos cikli paralel.
    pub sealed:        bool,
    /// Gjenerimi i token-it.
    pub token_gen:     u64,
}

// ─────────────────────────────────────────────────────────────────────────────
// HCP_PRO LAYER 3 — aktivizimi paralel i të gjithë operatorëve
// ─────────────────────────────────────────────────────────────────────────────

/// Pragu i aktivizimit të familjes në paralel.
pub const HCP_L3_FAMILY_THRESHOLD: f32 = 0.50;

pub struct HcpProL3;

impl HcpProL3 {
    /// activate_parallel — ngre TË GJITHA familjet njëheresh (aktiv paralel).
    ///
    /// HPRO mban harduerin (hpro_result); HCP_PRO lexon presionin dhe ngre të
    /// gjitha familjet që kanë optimal, ndërsa vihen kufijtë. Zero if — map/match.
    pub fn activate_parallel(
        gate: &mut SovereignGate,
        envelope: &PressureEnvelope,
        thermal: &ThermalReading,
        family_signals: &[FamilySignals],
        hpro_result: &HproResult,
    ) -> ParallelActivation {
        // Urdhri global i harduerit nga presioni RAM/CPU + termik (      ).
        // thermal_hot nga needs_action → bëhet pjesë e të njëjtit vendim të gjurmuar.
        let thermal_hot = SensorMathHarduer::needs_action(thermal);
        let hw_directive = HcpPro::decide_hardware(envelope, thermal_hot);

        // Ngri çdo familje njëheresh (zero if — map mbi familjet).
        let families: Vec<FamilyActivation> = family_signals.iter()
            .map(|fs| Self::activate_family(fs, hw_directive))
            .collect();

        // Sa familje u ngritën (GoDeeper). Zero if — filter.
        let active_count = families.iter()
            .filter(|f| f.order.bit() == 1)
            .count();

        // Vulos ciklin paralel me token (mint+burn).
        let handle = gate.issue(Self::cycle_seal(hpro_result));
        let token_gen = handle.gen;
        let sealed = match gate.validate(handle) {
            Ok(_)  => true,
            Err(_) => false,
        };

        ParallelActivation {
            hw_directive,
            families,
            active_count,
            sealed,
            token_gen,
        }
    }

    /// Production path: konsumon vendimin HCP tashmë të aplikuar dhe kufizon
    /// familjet që lejohen të hyjnë në MPRO. Nuk mint-on token të dytë.
    pub fn activate_from_application(
        application: &HcpApplicationResult,
        family_signals: &[FamilySignals],
        hpro_result: &HproResult,
        l2_allows_progress: bool,
        laws_passed: bool,
    ) -> ParallelActivation {
        let hw_directive = application.applied_directive;
        let lease_valid = hpro_result.lease.active
            & (hpro_result.lease.granted_bytes > 0)
            & application.lease_after.active
            & (application.lease_after.granted_bytes > 0);
        // Layer 3 respekton të njëjtin vendim të aplikuar DHE përfundimin e
        // Layer 2. Një probe që tha Stop nuk mund të rihapet nga familjet L3.
        let sealed = application.decision.sealed
            & lease_valid
            & l2_allows_progress
            & laws_passed;
        let limit = usize::try_from(application.lease_after.granted_parallel)
            .unwrap_or(usize::MAX);
        let mut admitted = 0usize;
        let families: Vec<FamilyActivation> = family_signals
            .iter()
            .map(|signals| {
                let mut activation = Self::activate_family(signals, hw_directive);
                let can_admit = (activation.order == OperatorOrder::GoDeeper)
                    & (admitted < limit)
                    & sealed;
                match can_admit {
                    true => admitted = admitted.saturating_add(1),
                    false => activation.order = OperatorOrder::Stop,
                }
                activation
            })
            .collect();

        ParallelActivation {
            hw_directive,
            families,
            active_count: admitted,
            sealed,
            token_gen: application.decision.token_gen,
        }
    }

    /// activate_family — vendos urdhër për një familje nga sinjali mesatar.
    ///
    /// Hardware PullBack → edhe familjet e forta ulin intensitetin. Zero if — match.
    fn activate_family(fs: &FamilySignals, hw: HardwareDirective) -> FamilyActivation {
        let avg = fs.avg_signal();
        let above_threshold = avg >= HCP_L3_FAMILY_THRESHOLD;

        // Nën PullBack, vetëm familjet shumë të forta vazhdojnë. Zero if — match.
        let hw_allows = Self::hw_permits_depth(hw, avg);

        // Familja ngrihet vetëm nëse kalon pragun DHE hardware lejon. Zero if — match.
        let order = match (above_threshold, hw_allows) {
            (true, true) => OperatorOrder::GoDeeper,
            _            => OperatorOrder::Stop,
        };

        FamilyActivation {
            family: fs.family,
            order,
            avg,
        }
    }

    /// hw_permits_depth — a lejon hardware thellim për këtë sinjal. Zero if — match.
    fn hw_permits_depth(hw: HardwareDirective, signal: f32) -> bool {
        // PullBack kërkon sinjal shumë të lartë (≥0.80); të tjerat lejojnë normal.
        let very_strong = signal >= 0.80;
        match hw {
            HardwareDirective::PullBack    => very_strong,  // vetëm shumë të fortët
            HardwareDirective::Hold        => true,
            HardwareDirective::PushDeeper  => true,
        }
    }

    /// cycle_seal — prodhon seal-in e ciklit nga rezultati HPRO. Zero if.
    fn cycle_seal(hpro_result: &HproResult) -> u64 {
        // Seal-i pranon lease aktiv gjatë workload-it ose receipt release pas tij.
        let base = 0x4C33_0001u64; // "L3" + version
        let lease_valid = hpro_result.lease.active | hpro_result.hardware_released;
        base.wrapping_add(lease_valid as u64)
    }

    /// ready_to_package — a janë gati familjet aktive për paketim te Shadow? Zero if.
    pub fn ready_to_package(activation: &ParallelActivation) -> bool {
        // Gati nëse të paktën një familje u ngrit + cikli u vulos.
        let has_active = activation.active_count > 0;
        has_active & activation.sealed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lab_contracts_v11::hardware::DeviceSnapshot;
    use crate::lim_types::Operator;
    use crate::layer3::hpro::HproEngine;

    fn envelope(under_pressure: bool, depth: crate::hw_real::territories::DepthHint) -> PressureEnvelope {
        PressureEnvelope {
            locked_bytes: 1_000_000, released_bytes: 500_000, total_reserved: 1_500_000,
            demand_bytes: 800_000, under_pressure, depth_hint: depth, ts_ns: 100,
        }
    }

    fn hpro_ok() -> HproResult {
        HproEngine::run(&DeviceSnapshot {
            ram_free_bytes: 4_000_000_000,
            ram_total_bytes: 8_000_000_000,
            cpu_load: 0.3,
            cores_active: 4,
            cores_total: 8,
            energy_margin: 0.9,
            on_battery: false,
            temp_c: 40.0,
            temp_limit_c: 90.0,
            timestamp: 100,
        })
    }

    fn all_families() -> Vec<FamilySignals> {
        vec![
            FamilySignals::new(ProFamily::Pro,  [0.8, 0.7, 0.9, 0.6]),  // avg 0.75
            FamilySignals::new(ProFamily::Npro, [0.3, 0.2, 0.4, 0.3]),  // avg 0.30
            FamilySignals::new(ProFamily::Hpro, [0.6, 0.5, 0.7, 0.6]),  // avg 0.60
            FamilySignals::new(ProFamily::Apro, [0.4, 0.5, 0.5, 0.4]),  // avg 0.45
        ]
    }

    fn applied_runtime(
        requested_parallel: u32,
        operator_mask: u8,
    ) -> (HcpApplicationResult, HproResult) {
        use crate::hw_real::{
            HardwareRuntimeContext, HwManager, MeasurementState,
        };
        use crate::hw_real::territories::DepthHint;

        let mut runtime = HardwareRuntimeContext::capture(0xA454, DepthHint::Shallow);
        runtime.measurement_state = MeasurementState::Measured;
        runtime.raw_snapshot.ram_total_bytes = 8_000_000_000;
        runtime.raw_snapshot.ram_free_bytes = 4_000_000_000;
        runtime.raw_snapshot.cores_active = requested_parallel.saturating_add(1).max(2);
        runtime.set_operator_admission_mask(operator_mask);
        let environment = runtime.environment();
        let lease = HwManager::govern(
            environment.ram_total_bytes,
            environment.ram_free_bytes,
            512 * 1024,
            environment.cores_active,
            requested_parallel,
        );
        runtime.install_initial_lease(lease);
        let signals = [
            (Operator::Reverse, 0.95),
            (Operator::Mirror, 0.90),
            (Operator::Contrast, 0.85),
            (Operator::Trace, 0.80),
        ];
        let pressure = runtime.pressure_envelope();
        let mut gate = SovereignGate::new();
        let application = HcpPro::orchestrate_and_apply(
            &mut gate,
            &mut runtime,
            &pressure,
            &signals,
            false,
        );
        let mut hpro = HproEngine::acquire(
            &DeviceSnapshot {
                ram_free_bytes: environment.ram_free_bytes,
                ram_total_bytes: environment.ram_total_bytes,
                cpu_load: 0.10,
                cores_active: environment.cores_active,
                cores_total: environment.cores_active,
                energy_margin: 0.90,
                on_battery: false,
                temp_c: 35.0,
                temp_limit_c: 90.0,
                timestamp: 0xA454,
            },
            MeasurementState::Measured,
        );
        hpro.apply_lease(application.lease_after);
        (application, hpro)
    }

    #[test]
    fn all_families_activated_parallel() {
        let mut gate = SovereignGate::new();
        let env = envelope(false, crate::hw_real::territories::DepthHint::Deep);
        let activation = HcpProL3::activate_parallel(&mut gate, &env, &ThermalReading::for_test(false), &all_families(), &hpro_ok());
        // Të 4 familjet u vlerësuan njëheresh.
        assert_eq!(activation.families.len(), 4);
        // PRO (0.75) dhe HPRO (0.60) ngrihen; NPRO (0.30), APRO (0.45) jo.
        assert_eq!(activation.active_count, 2);
        // Cikli u vulos.
        assert!(activation.sealed);
    }

    #[test]
    fn pullback_limits_weak_families() {
        let mut gate = SovereignGate::new();
        // Nën presion → PullBack.
        let env = envelope(true, crate::hw_real::territories::DepthHint::Deep);
        let activation = HcpProL3::activate_parallel(&mut gate, &env, &ThermalReading::for_test(false), &all_families(), &hpro_ok());
        assert_eq!(activation.hw_directive, HardwareDirective::PullBack);
        // Nën PullBack, asnjë familje s'ka sinjal ≥0.80 → asnjë ngrihet.
        assert_eq!(activation.active_count, 0);
    }

    #[test]
    fn strong_family_survives_pullback() {
        let mut gate = SovereignGate::new();
        let env = envelope(true, crate::hw_real::territories::DepthHint::Deep);
        let strong = vec![
            FamilySignals::new(ProFamily::Pro, [0.9, 0.85, 0.9, 0.88]),  // avg ~0.88
        ];
        let activation = HcpProL3::activate_parallel(&mut gate, &env, &ThermalReading::for_test(false), &strong, &hpro_ok());
        // Familje shumë e fortë (≥0.80) mbijeton edhe nën PullBack.
        assert_eq!(activation.active_count, 1);
    }

    #[test]
    fn ready_to_package_when_active() {
        let mut gate = SovereignGate::new();
        let env = envelope(false, crate::hw_real::territories::DepthHint::Deep);
        let activation = HcpProL3::activate_parallel(&mut gate, &env, &ThermalReading::for_test(false), &all_families(), &hpro_ok());
        // Ka familje aktive + vulosur → gati për Shadow.
        assert!(HcpProL3::ready_to_package(&activation));
    }

    #[test]
    fn family_avg_correct() {
        let fs = FamilySignals::new(ProFamily::Pro, [0.8, 0.6, 0.8, 0.8]);
        assert!((fs.avg_signal() - 0.75).abs() < 0.01);
    }

    // ── TEST INTEGRUES (end-to-end): activate_parallel → bridge → mesazh real ──
    //
    // RREGULLIM     : vërteton që vendimi REAL i HCP_PRO (jo hardcoded) kalon
    // përmes bridge-it të tipizuar drejt Shadow-it. Vlerat e mesazhit duhet të
    // dalin nga vendimi real (token_gen nga gate), JO nga literale.
    #[test]
    fn integration_hcp_decision_reaches_bridge() {
        use crate::bridge_shadow::{HcpShadowBridge, HcpDecisionInbound};

        let mut gate = SovereignGate::new();
        let env = envelope(false, crate::hw_real::territories::DepthHint::Deep);
        // Vendim REAL i orkestrimit paralel.
        let activation = HcpProL3::activate_parallel(&mut gate, &env, &ThermalReading::for_test(false), &all_families(), &hpro_ok());
        assert!(activation.sealed); // token u mint+burn

        // Kalo vendimin REAL përmes bridge-it të tipizuar (jo hardcoded).
        let input_id = 7777u64;
        let token_nonce = 0x4843_5050_524F_0001u64; // S_HCP_PRO
        let inbound: HcpDecisionInbound =
            HcpShadowBridge::from_parallel(input_id, &activation, token_nonce);

        // Mesazhi mban vlerat REALE nga vendimi (jo literale).
        assert_eq!(inbound.input_id, input_id);
        assert_eq!(inbound.token_gen, activation.token_gen); // nga gate, JO hardcoded
        assert_eq!(inbound.token_nonce, token_nonce);
        // PushDeeper (thellë, pa presion) → directive_bit=1.
        assert_eq!(inbound.directive_bit, 1);

        // Parametrat për Shadow heart_byte (as_record_args).
        let (id, gen, nonce, bit) = inbound.as_record_args();
        assert_eq!(id, input_id);
        assert_eq!(gen, activation.token_gen);
        assert_eq!(nonce, token_nonce);
        assert_eq!(bit, 1);
    }

    #[test]
    fn integration_pullback_directive_zero() {
        use crate::bridge_shadow::HcpShadowBridge;
        let mut gate = SovereignGate::new();
        // Nën presion → PullBack → directive_bit=0.
        let env = envelope(true, crate::hw_real::territories::DepthHint::Deep);
        let activation = HcpProL3::activate_parallel(&mut gate, &env, &ThermalReading::for_test(false), &all_families(), &hpro_ok());
        let inbound = HcpShadowBridge::from_parallel(1, &activation, 0xABCD);
        assert_eq!(inbound.directive_bit, 0); // PullBack
    }

    #[test]
    fn applied_runtime_lease_caps_family_admission_without_second_token() {
        use crate::hw_real::{
            HardwareRuntimeContext, HwManager, MeasurementState,
        };
        use crate::hw_real::territories::DepthHint;

        let mut runtime = HardwareRuntimeContext::capture(0xA453, DepthHint::Shallow);
        let env = runtime.environment();
        let lease = HwManager::govern(
            env.ram_total_bytes,
            env.ram_free_bytes,
            512 * 1024,
            env.cores_active,
            2,
        );
        runtime.install_initial_lease(lease);

        let operator_signals = [
            (Operator::Reverse, 0.90),
            (Operator::Mirror, 0.85),
            (Operator::Contrast, 0.80),
            (Operator::Trace, 0.75),
        ];
        let pressure = runtime.pressure_envelope();
        let mut gate = SovereignGate::new();
        let application = HcpPro::orchestrate_and_apply(
            &mut gate,
            &mut runtime,
            &pressure,
            &operator_signals,
            false,
        );

        let mut hpro = HproEngine::acquire(&DeviceSnapshot {
            ram_free_bytes: env.ram_free_bytes,
            ram_total_bytes: env.ram_total_bytes,
            cpu_load: 0.10,
            cores_active: env.cores_active,
            cores_total: env.cores_active,
            energy_margin: 0.90,
            on_battery: false,
            temp_c: 35.0,
            temp_limit_c: 90.0,
            timestamp: 0xA453,
        }, MeasurementState::Measured);
        hpro.apply_lease(application.lease_after);

        let strong_families = vec![
            FamilySignals::new(ProFamily::Pro, [0.9, 0.9, 0.9, 0.9]),
            FamilySignals::new(ProFamily::Npro, [0.9, 0.9, 0.9, 0.9]),
            FamilySignals::new(ProFamily::Hpro, [0.9, 0.9, 0.9, 0.9]),
            FamilySignals::new(ProFamily::Apro, [0.9, 0.9, 0.9, 0.9]),
        ];
        let activation = HcpProL3::activate_from_application(
            &application,
            &strong_families,
            &hpro,
            true,
            true,
        );

        assert!(activation.sealed);
        assert_eq!(activation.token_gen, application.decision.token_gen);
        assert!(activation.active_count <= application.lease_after.granted_parallel as usize);
        assert_eq!(
            activation.families.iter()
                .filter(|family| family.order == OperatorOrder::GoDeeper)
                .count(),
            activation.active_count,
        );
    }

    #[test]
    fn zero_byte_applied_lease_closes_all_family_admission() {
        use crate::hcp_pro::{
            HcpApplicationReceipt, OrchestrationDecision,
        };
        use crate::hw_real::{HwLease, MeasurementState};

        let zero_lease = HwLease {
            lease_id: 0x0,
            granted_bytes: 0,
            granted_parallel: 1,
            floor_bytes: 0,
            reserved_cores: 1,
            flexed: true,
            active: true,
        };
        let decision = OrchestrationDecision {
            hw_directive: HardwareDirective::PullBack,
            operator_orders: Vec::new(),
            sealed: true,
            token_gen: 9,
        };
        let application = HcpApplicationResult {
            decision,
            lease_before: zero_lease,
            lease_after: zero_lease,
            applied_directive: HardwareDirective::PullBack,
            receipt: HcpApplicationReceipt {
                cycle_id: 1,
                requested: HardwareDirective::PullBack,
                applied: HardwareDirective::PullBack,
                lease_before_id: 0,
                lease_after_id: 0,
                bytes_before: 0,
                bytes_after: 0,
                parallel_before: 1,
                parallel_after: 1,
                operator_mask_before: 0x0F,
                operator_mask_after: 0,
                sealed: true,
                changed: true,
                flexed: true,
            },
        };
        let mut hpro = HproEngine::acquire(
            &DeviceSnapshot {
                ram_free_bytes: 4_000_000_000,
                ram_total_bytes: 8_000_000_000,
                cpu_load: 0.1,
                cores_active: 4,
                cores_total: 8,
                energy_margin: 0.9,
                on_battery: false,
                temp_c: 35.0,
                temp_limit_c: 90.0,
                timestamp: 1,
            },
            MeasurementState::Measured,
        );
        hpro.apply_lease(zero_lease);

        let activation = HcpProL3::activate_from_application(
            &application,
            &all_families(),
            &hpro,
            true,
            true,
        );
        assert!(!activation.sealed);
        assert_eq!(activation.active_count, 0);
        assert!(activation
            .families
            .iter()
            .all(|family| family.order == OperatorOrder::Stop));
    }


    #[test]
    fn l2_stop_cannot_be_reopened_by_layer3() {
        let (application, hpro) = applied_runtime(4, 0x0F);
        let signals = vec![
            FamilySignals::new(ProFamily::Pro, [1.0; 4]),
            FamilySignals::new(ProFamily::Hpro, [1.0; 4]),
        ];
        let activation = HcpProL3::activate_from_application(
            &application,
            &signals,
            &hpro,
            false,
            true,
        );
        assert!(!activation.sealed);
        assert_eq!(activation.active_count, 0);
        assert!(activation
            .families
            .iter()
            .all(|family| family.order == OperatorOrder::Stop));
    }

}
