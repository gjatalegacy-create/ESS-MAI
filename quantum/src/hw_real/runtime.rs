// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  HW_REAL/RUNTIME.RS — Konteksti kanonik hardware për një cikël Quantum ║
// ║  ESS-MAI v1.6.8                                                        ║
// ║                                                                          ║
// ║  Një matje → një DeviceSnapshot → një lease aktiv → HPRO/PRO/HCP_PRO.   ║
// ║  Ky modul nuk jep verdict dhe nuk rezervon RAM në nivel OS; kontrollon  ║
// ║  workload-in real të ESS-MAI (operatorë, worker-a, buffers dhe depth).  ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::hw_real::ffi::{kernel_cooling_available, kernel_hw_available, CHwSnapshot};
use crate::hw_real::hw_manager::{HwEnvironment, HwLease};
use crate::hw_real::territories::{DepthHint, PressureEnvelope, ResourceTerritory};
use crate::hw_real::thermal::{SensorMathHarduer, SensorMode, ThermalState};
use crate::lab_contracts_v11::hardware::DeviceSnapshot;

/// Cilësia e matjes së përdorur në cikël.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeasurementState {
    /// Hardware + sensor termik real.
    Measured,
    /// Hardware real, por termika ose një fushë është e vlerësuar/paplotë.
    Degraded,
    /// `hw_kernel` joaktiv: vlera nominale të deklaruara.
    NominalFallback,
    /// Matja nuk u mor; vetëm kufij konservativë.
    Unavailable,
}

impl MeasurementState {
    pub fn label(self) -> &'static str {
        match self {
            MeasurementState::Measured => "MEASURED",
            MeasurementState::Degraded => "DEGRADED",
            MeasurementState::NominalFallback => "NOMINAL_FALLBACK",
            MeasurementState::Unavailable => "UNAVAILABLE",
        }
    }

    pub fn has_real_measurement(self) -> bool {
        matches!(self, MeasurementState::Measured | MeasurementState::Degraded)
    }
}

/// Burimi i matjes për trace runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeasurementSource {
    KernelSensor,
    KernelMathematicalThermal,
    Nominal,
    Unavailable,
}

impl MeasurementSource {
    pub fn label(self) -> &'static str {
        match self {
            MeasurementSource::KernelSensor => "KERNEL_SENSOR",
            MeasurementSource::KernelMathematicalThermal => "KERNEL_MATHEMATICAL_THERMAL",
            MeasurementSource::Nominal => "NOMINAL",
            MeasurementSource::Unavailable => "UNAVAILABLE",
        }
    }
}

/// Prova e lëshimit të workload-it të ciklit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HardwareReleaseReceipt {
    pub cycle_id: u64,
    pub lease_id: u64,
    pub workers_joined: u32,
    pub buffers_released: u32,
    pub buffer_bytes_released: u64,
    pub lease_released: bool,
    pub completed: bool,
    pub reason: &'static str,
}

/// State-i kanonik që kalon në të gjithë rrjedhën Quantum.
#[derive(Debug)]
pub struct HardwareRuntimeContext {
    pub cycle_id: u64,
    pub depth_hint: DepthHint,
    pub measurement_state: MeasurementState,
    pub measurement_source: MeasurementSource,
    pub measured_at: u64,
    pub raw_snapshot: CHwSnapshot,
    pub device_snapshot: DeviceSnapshot,
    pub thermal_state: ThermalState,
    pub cooling_control_available: bool,
    pub initial_lease: Option<HwLease>,
    pub current_lease: Option<HwLease>,
    pub last_release: Option<HardwareReleaseReceipt>,
    /// Bitmask kanonik: Reverse=1, Mirror=2, Contrast=4, Trace=8.
    pub operator_admission_mask: u8,
    workers_joined: u32,
    buffers_tracked: u32,
    buffer_bytes_tracked: u64,
}

impl HardwareRuntimeContext {
    /// Merr vetëm një snapshot për ciklin. Default build etiketohet nominal;
    /// `hw_kernel` dallohet si measured/degraded.
    pub fn capture(cycle_id: u64, depth_hint: DepthHint) -> Self {
        let kernel_real = kernel_hw_available();
        match ResourceTerritory::snapshot(depth_hint) {
            Ok(resource) => {
                let raw = resource.inner;
                let thermal = SensorMathHarduer::read(&raw);
                let (thermal_state, measurement_state, measurement_source, temp_c, temp_limit_c) =
                    match (kernel_real, thermal) {
                        (false, Ok(reading)) => {
                            let hot = SensorMathHarduer::needs_action(&reading);
                            (
                                match hot {
                                    true => ThermalState::Hot,
                                    false => ThermalState::Normal,
                                },
                                MeasurementState::NominalFallback,
                                MeasurementSource::Nominal,
                                reading.temp_c(),
                                reading.t_limit(),
                            )
                        }
                        (false, Err(_)) => (
                            ThermalState::Unknown,
                            MeasurementState::NominalFallback,
                            MeasurementSource::Nominal,
                            1.0,
                            1.0,
                        ),
                        (true, Ok(reading)) => {
                            let hot = SensorMathHarduer::needs_action(&reading);
                            let (state, source) = match reading.mode {
                                SensorMode::IotDirect => (
                                    MeasurementState::Measured,
                                    MeasurementSource::KernelSensor,
                                ),
                                SensorMode::Mathematical => (
                                    MeasurementState::Degraded,
                                    MeasurementSource::KernelMathematicalThermal,
                                ),
                            };
                            (
                                match hot {
                                    true => ThermalState::Hot,
                                    false => ThermalState::Normal,
                                },
                                state,
                                source,
                                reading.temp_c(),
                                reading.t_limit(),
                            )
                        }
                        (true, Err(_)) => (
                            ThermalState::Unknown,
                            MeasurementState::Degraded,
                            MeasurementSource::KernelMathematicalThermal,
                            1.0,
                            1.0,
                        ),
                    };

                let device_snapshot = DeviceSnapshot {
                    ram_free_bytes: raw.ram_free_bytes,
                    ram_total_bytes: raw.ram_total_bytes,
                    cpu_load: (raw.cpu_load_pct / 100.0).clamp(0.0, 1.0),
                    cores_active: raw.cores_active.max(1),
                    cores_total: raw.cores_total.max(raw.cores_active).max(1),
                    energy_margin: raw.energy_margin_pct.clamp(0.0, 1.0),
                    on_battery: raw.on_battery,
                    temp_c,
                    temp_limit_c: temp_limit_c.max(1.0),
                    timestamp: raw.ts_ns,
                };

                Self {
                    cycle_id,
                    depth_hint,
                    measurement_state,
                    measurement_source,
                    measured_at: raw.ts_ns,
                    raw_snapshot: raw,
                    device_snapshot,
                    thermal_state,
                    cooling_control_available: kernel_cooling_available(),
                    initial_lease: None,
                    current_lease: None,
                    last_release: None,
                    operator_admission_mask: 0x0F,
                    workers_joined: 0,
                    buffers_tracked: 0,
                    buffer_bytes_tracked: 0,
                }
            }
            Err(_) => {
                let raw = CHwSnapshot {
                    ram_free_bytes: 0,
                    ram_total_bytes: 0,
                    cpu_load_pct: 100.0,
                    cores_active: 1,
                    cores_total: 1,
                    energy_margin_pct: 0.0,
                    on_battery: false,
                    _pad: [0u8; 7],
                    ts_ns: 0,
                };
                Self {
                    cycle_id,
                    depth_hint,
                    measurement_state: MeasurementState::Unavailable,
                    measurement_source: MeasurementSource::Unavailable,
                    measured_at: 0,
                    raw_snapshot: raw,
                    device_snapshot: DeviceSnapshot {
                        ram_free_bytes: 0,
                        ram_total_bytes: 0,
                        cpu_load: 1.0,
                        cores_active: 1,
                        cores_total: 1,
                        energy_margin: 0.0,
                        on_battery: false,
                        temp_c: 1.0,
                        temp_limit_c: 1.0,
                        timestamp: 0,
                    },
                    thermal_state: ThermalState::Unknown,
                    cooling_control_available: false,
                    initial_lease: None,
                    current_lease: None,
                    last_release: None,
                    operator_admission_mask: 0x0F,
                    workers_joined: 0,
                    buffers_tracked: 0,
                    buffer_bytes_tracked: 0,
                }
            }
        }
    }

    pub fn environment(&self) -> HwEnvironment {
        HwEnvironment {
            ram_total_bytes: self.raw_snapshot.ram_total_bytes,
            ram_free_bytes: self.raw_snapshot.ram_free_bytes,
            cores_active: self.raw_snapshot.cores_active.max(1),
        }
    }

    pub fn install_initial_lease(&mut self, lease: HwLease) {
        self.initial_lease = Some(lease);
        self.current_lease = Some(lease);
    }

    pub fn replace_lease(&mut self, lease: HwLease) {
        self.current_lease = Some(lease);
    }

    pub fn lease(&self) -> Option<HwLease> {
        self.current_lease
    }

    pub fn set_operator_admission_mask(&mut self, mask: u8) {
        self.operator_admission_mask = mask & 0x0F;
    }

    pub fn note_workers_joined(&mut self, count: usize) {
        self.workers_joined = self
            .workers_joined
            .saturating_add(u32::try_from(count).unwrap_or(u32::MAX));
    }

    /// Regjistron buffer-at e zotëruar nga cikli. Meqë context-i deklarohet
    /// para buffer-ave në main, ata dalin nga scope-i para Drop/release-it.
    pub fn register_buffers(&mut self, count: usize, estimated_bytes: u64) {
        self.buffers_tracked = self
            .buffers_tracked
            .saturating_add(u32::try_from(count).unwrap_or(u32::MAX));
        self.buffer_bytes_tracked = self
            .buffer_bytes_tracked
            .saturating_add(estimated_bytes);
    }

    /// Envelope nga lease-i aktual; HCP_PRO e përdor pa krijuar snapshot të dytë.
    pub fn pressure_envelope(&self) -> PressureEnvelope {
        let lease = self.current_lease.unwrap_or(HwLease {
            lease_id: 0,
            granted_bytes: 0,
            granted_parallel: 1,
            floor_bytes: 0,
            reserved_cores: 0,
            flexed: true,
            active: false,
        });
        let reserved = self.raw_snapshot.ram_free_bytes;
        let cpu_pressure = self.raw_snapshot.cpu_load_pct >= 85.0;
        let energy_pressure = self.raw_snapshot.energy_margin_pct <= 0.25;
        let usable_after_floor = reserved.saturating_sub(lease.floor_bytes);
        let ram_pressure = (usable_after_floor == 0)
            | (lease.granted_bytes >= usable_after_floor);
        // CPU reserve normale nuk quhet presion. Presion është vetëm kur RAM-i
        // ka prekur kufirin fizik, CPU është e ngarkuar ose energjia është e ulët.
        let under_pressure = ram_pressure | cpu_pressure | energy_pressure;
        PressureEnvelope {
            locked_bytes: lease.granted_bytes,
            released_bytes: reserved.saturating_sub(lease.granted_bytes),
            total_reserved: reserved,
            demand_bytes: lease.granted_bytes,
            under_pressure,
            depth_hint: self.depth_hint,
            ts_ns: self.measured_at,
        }
    }

    pub fn release(&mut self, reason: &'static str) -> HardwareReleaseReceipt {
        match self.last_release {
            Some(receipt) => receipt,
            None => {
                let (lease_id, lease_released) = match self.current_lease.as_mut() {
                    Some(lease) => (lease.lease_id, lease.release()),
                    None => (0, true),
                };
                let receipt = HardwareReleaseReceipt {
                    cycle_id: self.cycle_id,
                    lease_id,
                    workers_joined: self.workers_joined,
                    buffers_released: self.buffers_tracked,
                    buffer_bytes_released: self.buffer_bytes_tracked,
                    lease_released,
                    completed: lease_released,
                    reason,
                };
                self.last_release = Some(receipt);
                receipt
            }
        }
    }
}

impl Drop for HardwareRuntimeContext {
    fn drop(&mut self) {
        match self.last_release {
            Some(_) => {}
            None => {
                let receipt = self.release("scope_drop");
                eprintln!(
                    "  [HW_RELEASE] cycle=0x{:X} lease=0x{:X} workers={} buffers={} buffer_bytes={} completed={} reason={}",
                    receipt.cycle_id,
                    receipt.lease_id,
                    receipt.workers_joined,
                    receipt.buffers_released,
                    receipt.buffer_bytes_released,
                    receipt.completed,
                    receipt.reason,
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_build_never_claims_measured_without_kernel() {
        let ctx = HardwareRuntimeContext::capture(1, DepthHint::Shallow);
        match kernel_hw_available() {
            true => assert!(matches!(
                ctx.measurement_state,
                MeasurementState::Measured | MeasurementState::Degraded
            )),
            false => assert_eq!(ctx.measurement_state, MeasurementState::NominalFallback),
        }
    }

    #[test]
    fn release_is_derived_from_active_lease() {
        let mut ctx = HardwareRuntimeContext::capture(2, DepthHint::Medium);
        let lease = crate::hw_real::HwManager::govern(
            8 * 1024 * 1024 * 1024,
            4 * 1024 * 1024 * 1024,
            512 * 1024 * 1024,
            4,
            2,
        );
        ctx.install_initial_lease(lease);
        ctx.note_workers_joined(2);
        let receipt = ctx.release("test");
        assert!(receipt.lease_released);
        assert!(receipt.completed);
        assert_eq!(receipt.workers_joined, 2);
    }

    #[test]
    fn release_receipt_is_idempotent_and_tracks_buffers() {
        let mut ctx = HardwareRuntimeContext::capture(3, DepthHint::Shallow);
        let lease = crate::hw_real::HwManager::govern(
            8 * 1024 * 1024 * 1024,
            4 * 1024 * 1024 * 1024,
            128 * 1024 * 1024,
            4,
            2,
        );
        ctx.install_initial_lease(lease);
        ctx.register_buffers(2, 4096);
        ctx.note_workers_joined(2);
        let first = ctx.release("first");
        let second = ctx.release("second");
        assert_eq!(first, second);
        assert_eq!(first.buffers_released, 2);
        assert_eq!(first.buffer_bytes_released, 4096);
        assert!(first.completed);
    }

}
