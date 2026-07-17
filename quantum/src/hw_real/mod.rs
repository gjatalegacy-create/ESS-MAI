// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  HW_REAL/MOD.RS — Hardware REAL (nga      Quantum Platform)           ║
// ║  GJATA LEGACY™                                                        ║
// ║                                                                          ║
// ║  Hardware-i i vërtetë: lexon RAM/CPU/termik real përmes sysinfo() +    ║
// ║  /proc, me fallback të simuluar kur hw_kernel feature është off.       ║
// ║                                                                          ║
// ║  feature "hw_kernel" ON  → kernelët C (RAM real via sysinfo)          ║
// ║  feature "hw_kernel" OFF → fallback nominal (hardware NUK matet)      ║
// ╚══════════════════════════════════════════════════════════════════════════╝

pub mod ffi;
pub mod hw_manager;   // MENAXHERI: 10% floor, fleksibilitet, R E A L control
pub mod runtime;      // v1.6.8: një matje + lease aktiv + release receipt
pub mod territories;
pub mod hw_core;
pub mod thermal;
pub mod thermal_thread;

// Re-eksporte kryesore
pub use ffi::{
    HwLgcError, HwLgcToken, hw_lgc_open,
    CHwSnapshot, CThermalSnapshot,
    kernel_hw_snapshot, kernel_thermal_read,
    kernel_hw_available, kernel_cooling_available,
};

pub use territories::{
    DepthHint, ResourceSnapshot, ResourceTerritory,
    ReserveSnapshot, ReserveTerritory,
    PressureEnvelope, PressureTerritory,
};

pub use hw_core::{
    HwBudget, ScaleHarduer,
    ProOp, ParallelDecision, ActParallel,
    ContractSignal, ResourceContract, QuantumSofthard,
};

pub use thermal::{
    SensorMode, ThermalReading, SensorMathHarduer,
    ThermalAction, ThermalDecision, ColdHeatManager,
    ColdDownResult, ColdDown, ThermalGuardState,
};

// ─────────────────────────────────────────────────────────────────────────────
// HardwareGateResult + hardware_pre_gate — pika kryesore e integrimit
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum HardwareGateResult {
    Contract(ResourceContract),
    Hold,
    Bypass,
}

/// hardware_pre_gate() — funksioni kryesor i integrimit.
/// Thirret nga pipeline-i PARA reasoning-ut të thellë.
///
/// Rrjedha: snapshot → reserve → envelope → budget → parallel → contract.
///
/// FAIL-CLOSED (audit): DALLIM I PRERË ndërmjet dy rrugëve —
///   • Bypass LEGJITIM: thellësia s'e kërkon portën (Shallow/Medium) →
///     kalim i deklaruar, JO anomali.
///   • Bypass i GABIMIT (para): çdo hap që DËSHTONTE kthente Bypass, pra
///     reasoning-u i thellë vazhdonte pa e detyruar rrugën fizike (fail-open).
///     Tani: dështimi i një hapi hardware = HOLD me arsye të emërtuar —
///     porta fizike bëhet DETYRUESE, s'anashkalohet në heshtje.
pub fn hardware_pre_gate(depth_hint: DepthHint) -> HardwareGateResult {
    match depth_hint.requires_hw_gate() {
        false => return HardwareGateResult::Bypass, // legjitim: pa gate për këtë thellësi
        true => {}
    }

    let snap = match ResourceTerritory::snapshot(depth_hint) {
        Ok(s)  => s,
        Err(_) => {
            eprintln!("[HW_GATE] HOLD: HWG_ERR_SNAPSHOT — matja fizike dështoi (fail-closed)");
            return HardwareGateResult::Hold;
        }
    };

    let reserve = match ReserveTerritory::apply(&snap) {
        Ok(r)  => r,
        Err(_) => {
            eprintln!("[HW_GATE] HOLD: HWG_ERR_RESERVE — rezervimi i burimeve dështoi");
            return HardwareGateResult::Hold;
        }
    };

    let envelope = match PressureTerritory::prepare(&reserve) {
        Ok(e)  => e,
        Err(_) => {
            eprintln!("[HW_GATE] HOLD: HWG_ERR_ENVELOPE — zarfi i presionit dështoi");
            return HardwareGateResult::Hold;
        }
    };

    let budget = match ScaleHarduer::allocate(&envelope, depth_hint) {
        Ok(b)  => b,
        Err(_) => {
            eprintln!("[HW_GATE] HOLD: HWG_ERR_BUDGET — alokimi i buxhetit dështoi");
            return HardwareGateResult::Hold;
        }
    };

    let parallel = match ActParallel::decide(&budget) {
        Ok(p)  => p,
        Err(_) => {
            eprintln!("[HW_GATE] HOLD: HWG_ERR_PARALLEL — vendimi paralel dështoi");
            return HardwareGateResult::Hold;
        }
    };

    let energy  = snap.energy_margin();
    let hw_real = kernel_hw_available();

    match QuantumSofthard::evaluate(budget, parallel, &envelope, energy, hw_real) {
        Ok(contract) => {
            // Hold → Hold; ndryshe → Contract. Zero if — match mbi sinjalin.
            match contract.signal == ContractSignal::Hold {
                true  => HardwareGateResult::Hold,
                false => HardwareGateResult::Contract(contract),
            }
        }
        Err(_) => {
            eprintln!("[HW_GATE] HOLD: HWG_ERR_EVALUATE — kontrata s'u vlerësua dot");
            HardwareGateResult::Hold
        }
    }
}

pub use hw_manager::{HwEnvironment, HwManager, HwLease, HW_FREE_FLOOR_PCT};
pub use runtime::{
    HardwareReleaseReceipt, HardwareRuntimeContext, MeasurementSource, MeasurementState,
};
