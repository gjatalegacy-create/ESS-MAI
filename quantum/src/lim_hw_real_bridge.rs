// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LIM_HW_REAL_BRIDGE.RS — Lidhja LIM ↔ HardwareRuntimeContext           ║
// ║  ESS-MAI v1.6.8 — një snapshot dhe një lease për çdo cikël             ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::hw_real::{
    DepthHint, HardwareRuntimeContext, HwLease, MeasurementState,
};
use crate::layer3::hpro::HproEngine;
use crate::lim_classifier::{compute_depth_config, compute_operator_gates};
use crate::lim_types::EpistemicScaffold;

/// Scaffold epistemik i lidhur me lease-in real të ciklit.
#[derive(Debug, Clone)]
pub struct RealHardwareScaffold {
    pub scaffold: EpistemicScaffold,
    pub depth_hint: DepthHint,
    pub allocated_bytes: u64,
    pub n_parallel: u8,
    pub gate_active: bool,
    pub hw_real: bool,
    pub measurement_state: MeasurementState,
    pub depth_epistemic: f32,
    pub depth_hardware: f32,
    pub depth_final: f32,
    pub ram_free_bytes: u64,
    pub ram_total_bytes: u64,
    pub cpu_load: f32,
    pub cores_active: u32,
    pub cores_total: u32,
    pub energy_margin: f32,
    pub on_battery: bool,
    pub lease: HwLease,
    pub operator_admission_mask: u8,
}

impl RealHardwareScaffold {
    pub fn hardware_permits(&self) -> bool {
        self.lease.active & (self.allocated_bytes > 0)
    }
}

impl crate::pro::HwScaffoldView for RealHardwareScaffold {
    fn epistemic(&self) -> &EpistemicScaffold {
        &self.scaffold
    }

    fn parallel_count(&self) -> u8 {
        self.n_parallel
    }

    fn runtime_lease(&self) -> Option<HwLease> {
        Some(self.lease)
    }

    fn operator_allowed(&self, operator: crate::lim_types::Operator) -> bool {
        let bit = match operator {
            crate::lim_types::Operator::Reverse => 0x01,
            crate::lim_types::Operator::Mirror => 0x02,
            crate::lim_types::Operator::Contrast => 0x04,
            crate::lim_types::Operator::Trace => 0x08,
        };
        (self.operator_admission_mask & bit) != 0
    }

    fn work_permitted(&self) -> bool {
        self.hardware_permits()
    }
}

pub struct LimHwRealBridge;

impl LimHwRealBridge {
    /// Production path: konsumon kontekstin kanonik; nuk mat hardware për herë të dytë.
    pub fn finalize_with_runtime(
        mut scaffold: EpistemicScaffold,
        runtime: &HardwareRuntimeContext,
    ) -> RealHardwareScaffold {
        let device = runtime.device_snapshot;
        let lease = runtime.lease().unwrap_or_else(|| {
            crate::hw_real::HwManager::govern(
                device.ram_total_bytes,
                device.ram_free_bytes,
                0,
                device.cores_active.max(1),
                1,
            )
        });

        let depth_epistemic = scaffold.lim.reasoning_depth_allowance;
        let depth_hardware = match device.ram_free_bytes {
            0 => 0.0,
            free => {
                (lease.granted_bytes as f32 / free as f32).clamp(0.0, 1.0)
            }
        };
        let depth_final = (depth_epistemic * depth_hardware).clamp(0.0, 1.0);

        scaffold.lim.reasoning_depth_allowance = depth_final;
        scaffold.operator_gates = compute_operator_gates(&scaffold.lim);
        scaffold.depth_config = compute_depth_config(&scaffold.lim);

        let gate_active = runtime.depth_hint.requires_hw_gate()
            & runtime.measurement_state.has_real_measurement()
            & lease.active;

        RealHardwareScaffold {
            scaffold,
            depth_hint: runtime.depth_hint,
            allocated_bytes: lease.granted_bytes,
            n_parallel: u8::try_from(lease.granted_parallel.min(4)).unwrap_or(4),
            gate_active,
            hw_real: runtime.measurement_state.has_real_measurement(),
            measurement_state: runtime.measurement_state,
            depth_epistemic,
            depth_hardware,
            depth_final,
            ram_free_bytes: device.ram_free_bytes,
            ram_total_bytes: device.ram_total_bytes,
            cpu_load: device.cpu_load,
            cores_active: device.cores_active,
            cores_total: device.cores_total,
            energy_margin: device.energy_margin,
            on_battery: device.on_battery,
            lease,
            operator_admission_mask: runtime.operator_admission_mask,
        }
    }

    /// Compatibility path: krijon kontekst dhe HPRO lease, pastaj thërret rrugën
    /// production. `main.rs` nuk duhet ta përdorë këtë sepse mban vetë kontekstin.
    pub fn finalize(scaffold: EpistemicScaffold) -> RealHardwareScaffold {
        let depth_hint = DepthHint::from_mass(scaffold.lim.epistemic_mass);
        let mut runtime = HardwareRuntimeContext::capture(0, depth_hint);
        let hpro = HproEngine::acquire(
            &runtime.device_snapshot,
            runtime.measurement_state,
        );
        runtime.install_initial_lease(hpro.lease);
        Self::finalize_with_runtime(scaffold, &runtime)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lim::LinearInfoMetricEngine;

    fn build_scaffold(turns: &[&str]) -> EpistemicScaffold {
        let mut lim = LinearInfoMetricEngine::new("s", "science");
        for t in turns {
            lim.collect(t);
        }
        lim.pre_pro_scan().expect("scaffold")
    }

    #[test]
    fn real_hw_depth_is_product() {
        let scaffold = build_scaffold(&[
            "deterministic elimination replaces probabilistic generation in reasoning systems",
            "knowledge that survives the negative filters becomes verifiable primitive knowledge",
        ]);
        let epistemic = scaffold.lim.reasoning_depth_allowance;
        let hw = LimHwRealBridge::finalize(scaffold);
        let expected = (epistemic * hw.depth_hardware).clamp(0.0, 1.0);
        assert!((hw.depth_final - expected).abs() < 1e-5);
    }

    #[test]
    fn scaffold_carries_active_runtime_lease() {
        let scaffold = build_scaffold(&[
            "quantum reasoning eliminates wrong paths through deterministic filters here",
            "the shadow authority verifies each output with a binary zero or one",
        ]);
        let hw = LimHwRealBridge::finalize(scaffold);
        assert!(hw.lease.active);
        assert_eq!(u32::from(hw.n_parallel), hw.lease.granted_parallel.min(4));
    }

    #[test]
    fn default_measurement_is_explicit() {
        let scaffold = build_scaffold(&[
            "help quick it opens fine",
            "help quick it will not open",
        ]);
        let hw = LimHwRealBridge::finalize(scaffold);
        match crate::hw_real::kernel_hw_available() {
            true => assert!(hw.measurement_state.has_real_measurement()),
            false => assert_eq!(
                hw.measurement_state,
                MeasurementState::NominalFallback
            ),
        }
    }
}
