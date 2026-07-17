// thermal_thread.rs - ESS-MAI Parallel Thermal Monitor
// GJATA LEGACY - Quantum Platform     
//
// Spawns a background thread that runs continuously,
// independent from the main reasoning pipeline.
// Uses ThermalGuardState (AtomicBool) for zero-lock
// communication with the pipeline.
//
// Thread lifecycle:
//   start_thermal_monitor() -> Arc<ThermalGuardState>
//       |
//       +-> thread loop:
//             hw_snapshot() -> run_thermal_cycle()
//               -> if overheat: set thermal_stop=true
//               -> if warning:  set thermal_warning=true
//             sleep(THERMAL_POLL_MS)
//   stop_thermal_monitor(state) -> signals thread to exit

use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::hw_real::ffi::{
    kernel_hw_snapshot,
    S_RESOURCE_TERRITORY, hw_lgc_open,
};
use crate::hw_real::thermal::{
    ThermalGuardState, ColdDown,
};

/// Poll interval for thermal monitoring (milliseconds)
const THERMAL_POLL_MS: u64 = 2000;

/// Starts the parallel thermal monitor thread.
/// Returns Arc<ThermalGuardState> shared with the pipeline.
/// Thread runs until thermal_stop is set externally OR overheat.
pub fn start_thermal_monitor() -> Arc<ThermalGuardState> {
    let state = Arc::new(ThermalGuardState::new());
    let state_thread = Arc::clone(&state);

    // FAIL-CLOSED, JO PANIK (audit): spawn i thread-it mund të dështojë (OS
    // limit). Para: .expect → rrëzonte të gjithë procesin Quantum. Tani:
    // degradim i kontrolluar — monitori shënohet i padisponueshëm dhe gjendja
    // vendoset THROTTLE si postuim i sigurt (asnjë supozim ftohtësie pa monitor).
    match thread::Builder::new()
        .name("hw_thermal_monitor".to_string())
        .spawn(move || {
            thermal_monitor_loop(state_thread);
        }) {
        Ok(_handle) => {}
        Err(e) => {
            eprintln!("[HW_THERMAL] DEGRADIM: spawn i monitorit dështoi ({e}) → \
                       gjendja THROTTLE (postuim i sigurt, s'ka panik)");
            state.mark_monitor_unavailable();
        }
    }

    state
}

/// The thermal monitor loop — runs until stop signal.
fn thermal_monitor_loop(state: Arc<ThermalGuardState>) {
    loop {
        // Check if pipeline requested stop. Zero if — match me break.
        match state.is_stop() {
            true => break,
            false => {}
        }

        // Get hardware snapshot for thermal calculation
        let hw_result = {
            match hw_lgc_open(S_RESOURCE_TERRITORY, S_RESOURCE_TERRITORY) {
                Ok(tok) => kernel_hw_snapshot(&tok),
                Err(_)  => {
                    thread::sleep(Duration::from_millis(THERMAL_POLL_MS));
                    continue;
                }
            }
        };

        let hw = match hw_result {
            Ok(snap) => snap,
            Err(_)   => {
                thread::sleep(Duration::from_millis(THERMAL_POLL_MS));
                continue;
            }
        };

        // Run thermal cycle: Sensor -> Heat -> Down
        match ColdDown::run_thermal_cycle(&hw) {
            Ok(Some(result)) => {
                // Action was taken. Prioritet: throttle > fan/cooling.
                // Zero if — match mbi tuple (throttle, fan||cooling).
                match (result.throttle_set, result.fan_set || result.cooling_set) {
                    (true, _) => {
                        // Thermal emergency: signal pipeline to stop
                        state.set_stop(true);
                        state.set_warning(true);
                    }
                    (false, true) => {
                        // Warning level: signal pipeline to reduce load
                        state.set_warning(true);
                    }
                    (false, false) => {}
                }
            }
            Ok(None) => {
                // Normal temperature - clear warning if it was set
                state.set_warning(false);
            }
            Err(_) => {
                // Read error - do not panic, just continue
            }
        }

        thread::sleep(Duration::from_millis(THERMAL_POLL_MS));
    }
}

/// Stops the thermal monitor gracefully.
/// Sets thermal_stop = true, thread exits on next poll.
pub fn stop_thermal_monitor(state: &Arc<ThermalGuardState>) {
    state.set_stop(true);
}

/// Checks thermal state from pipeline side (zero-lock).
/// Called from Orchestrator before each reasoning cycle.
pub fn thermal_pipeline_check(state: &Arc<ThermalGuardState>)
    -> ThermalPipelineStatus
{
    match state.is_stop() {
        true => return ThermalPipelineStatus::EmergencyStop,
        false => {}
    }
    match state.is_warning() {
        true => return ThermalPipelineStatus::ReduceLoad,
        false => {}
    }
    ThermalPipelineStatus::Normal
}

/// Status returned to pipeline from thermal check
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ThermalPipelineStatus {
    /// Normal temperature - proceed
    Normal,
    /// Warning temperature - reduce reasoning depth
    ReduceLoad,
    /// Emergency - stop reasoning immediately
    EmergencyStop,
}

impl ThermalPipelineStatus {
    pub fn label(self) -> &'static str {
        match self {
            Self::Normal        => "THERMAL_NORMAL",
            Self::ReduceLoad    => "THERMAL_WARN_REDUCE",
            Self::EmergencyStop => "THERMAL_EMERGENCY_STOP",
        }
    }

    pub fn blocks_pipeline(self) -> bool {
        self == Self::EmergencyStop
    }
}
