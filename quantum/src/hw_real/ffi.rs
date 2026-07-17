// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  ffi.rs — ESS-MAI Hardware FFI Boundary                               ║
// ║  GJATA LEGACY™ — Quantum Platform                                     ║
// ║                                                                          ║
// ║  RREGULL ABSOLUT: I VETËM FILE me `unsafe` në hardware layer.         ║
// ║  Çdo extern "C" deklaratë është këtu.                                ║
// ║  Safe wrappers validojnë output C para kthimit në Rust.               ║
// ║                                                                          ║
// ║  LGC SEALS: const fn compile-time për të 9 modulet hardware.          ║
// ║  Njësoj si modulet ekzistuese      — zero runtime computation.        ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use std::marker::PhantomData;

// ════════════════════════════════════════════════════════════════════════════
// §1  FNV-1a + LGC SEAL — compile-time (identikë me      ekzistues)
// ════════════════════════════════════════════════════════════════════════════

#[inline(always)]
pub const fn fnv1a_64(s: &[u8]) -> u64 {
    let mut h: u64 = 14695981039346656037;
    let mut i = 0usize;
    while i < s.len() {
        h ^= s[i] as u64;
        h = h.wrapping_mul(1099511628211);
        i += 1;
    }
    h
}

pub const fn hw_seal(m: &[u8]) -> u64 {
    fnv1a_64(m) ^ fnv1a_64(b"GJATA_LEGACY")
}

/// LGC Seals compile-time për të 9 modulet hardware
pub const S_RESOURCE_TERRITORY:  u64 = hw_seal(b"RESOURCE_TERRITORY");
pub const S_RESERVE_TERRITORY:   u64 = hw_seal(b"RESERVE_TERRITORY");
pub const S_PRESSURE_TERRITORY:  u64 = hw_seal(b"PRESSURE_TERRITORY");
pub const S_SCALE_HARDUER:       u64 = hw_seal(b"SCALE_HARDUER");
pub const S_ACT_PARALLEL:        u64 = hw_seal(b"ACT_PARALLEL_ARGS");
pub const S_QUANTUM_SOFTHARD:    u64 = hw_seal(b"QUANTUM_SOFTHARD");
pub const S_SENSOR_MATH:         u64 = hw_seal(b"SENSOR_MATH_HARDUER");
pub const S_COLD_HEAT:           u64 = hw_seal(b"COLD_HEAT_MANAGER");
pub const S_COLD_DOWN:           u64 = hw_seal(b"COLD_DOWN");

// ════════════════════════════════════════════════════════════════════════════
// §2  LGC TOKEN — identik me      (PhantomData !Send !Clone !Copy)
// ════════════════════════════════════════════════════════════════════════════

pub struct HwLgcToken(PhantomData<*const ()>);

#[derive(Debug)]
pub enum HwLgcError {
    SealMismatch(u64),
    CKernelNull,
    CKernelInvalid(i32),
    InvariantViolation(&'static str),
}

/// Hap LGC gate për modul hardware
/// Branchless delta check — identik me lgc_open()     
pub fn hw_lgc_open(expected: u64, runtime: u64) -> Result<HwLgcToken, HwLgcError> {
    let delta   = expected ^ runtime;
    let invalid = (delta | delta.wrapping_neg()) >> 63;
    match invalid {
        0 => Ok(HwLgcToken(PhantomData)),
        _ => Err(HwLgcError::SealMismatch(delta)),
    }
}

// ════════════════════════════════════════════════════════════════════════════
// §3  C STRUCTS — repr(C) për FFI
// ════════════════════════════════════════════════════════════════════════════

/// Pasqyron HwSnapshot nga hw_resource.h
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct CHwSnapshot {
    pub ram_free_bytes:    u64,
    pub ram_total_bytes:   u64,
    pub cpu_load_pct:      f32,
    pub cores_active:      u32,
    pub cores_total:       u32,
    pub energy_margin_pct: f32,
    pub on_battery:        bool,
    pub _pad:              [u8; 7],
    pub ts_ns:             u64,
}

/// Pasqyron ThermalSnapshot nga hw_thermal.h
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct CThermalSnapshot {
    pub t_estimated_c:    f32,
    pub t_limit_c:        f32,
    pub t_warn_c:         f32,
    pub t_junction_max_c: f32,
    pub p_cpu_w:          f32,
    pub p_mem_w:          f32,
    pub p_total_w:        f32,
    pub sensor_used:      bool,
    pub overheat:         bool,
    pub warning:          bool,
    pub _pad:             [u8; 5],
    pub ts_ns:            u64,
}

/// Pasqyron ColdDownAction nga hw_colddown.h
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CColdDownAction {
    Normal    = 0,
    Warn      = 1,
    Cool      = 2,
    Throttle  = 3,
    Emergency = 4,
}

/// Pasqyron ColdDownResult nga hw_colddown.h
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct CColdDownResult {
    pub rc:               i32,
    pub fan_set:          bool,
    pub cooling_set:      bool,
    pub throttle_set:     bool,
    pub fan_pct_applied:  u8,
}

// ════════════════════════════════════════════════════════════════════════════
// §4  EXTERN "C" — TË GJITHA DEKLARATAT UNSAFE JANË KËTU
//     Asgjë unsafe jashtë këtij seksioni.
// ════════════════════════════════════════════════════════════════════════════

#[cfg(feature = "hw_kernel")]
extern "C" {
    // hw_resource.h
    fn hw_resource_snapshot(out: *mut CHwSnapshot) -> i32;
    fn hw_resource_validate(snap: *const CHwSnapshot) -> i32;

    // hw_thermal.h
    fn hw_thermal_read(
        hw:  *const CHwSnapshot,
        cfg: *const u8,        // NULL = use default ThermalConfig
        out: *mut CThermalSnapshot,
    ) -> i32;
    fn hw_thermal_iot_available() -> i32;

    // hw_colddown.h
    fn hw_colddown_execute(
        action: CColdDownAction,
        result: *mut CColdDownResult,
    ) -> i32;
    fn hw_colddown_restore_normal() -> i32;
    fn hw_colddown_sysfs_available() -> i32;
}

// ════════════════════════════════════════════════════════════════════════════
// §5  SAFE WRAPPERS — validojnë output C para kthimit
//     Jashtë këtyre, kodi është 100% safe Rust.
// ════════════════════════════════════════════════════════════════════════════

/// Merr snapshot hardware nga C kernel
/// Validon të gjitha invariantet para kthimit
pub fn kernel_hw_snapshot(_tok: &HwLgcToken) -> Result<CHwSnapshot, HwLgcError> {
    #[cfg(feature = "hw_kernel")]
    {
        let mut snap = CHwSnapshot::default();
        let rc = unsafe { hw_resource_snapshot(&mut snap) };
        match rc != 0 {
            true => return Err(HwLgcError::CKernelInvalid(rc)),
            false => {}
        }
        // Validim invariantesh
        match snap.ram_total_bytes == 0 {
            true => return Err(HwLgcError::InvariantViolation("ram_total=0")),
            false => {}
        }
        match snap.ram_free_bytes > snap.ram_total_bytes {
            true => return Err(HwLgcError::InvariantViolation("ram_free > ram_total")),
            false => {}
        }
        match snap.cpu_load_pct < 0.0 || snap.cpu_load_pct > 100.0 {
            true => return Err(HwLgcError::InvariantViolation("cpu_load_pct ∉ [0,100]")),
            false => {}
        }
        match snap.energy_margin_pct < 0.0 || snap.energy_margin_pct > 1.0 {
            true => return Err(HwLgcError::InvariantViolation("energy_margin ∉ [0,1]")),
            false => {}
        }
        match snap.cores_active == 0 || snap.cores_active > snap.cores_total {
            true => return Err(HwLgcError::InvariantViolation("cores_active invalid")),
            false => {}
        }
        Ok(snap)
    }

    // ── FALLBACK NOMINAL (hw_kernel OFF — hardware NUK matet) ──────────────
    // Kompilohet VETËM pa feature-n hw_kernel. Prodhimi (full/hw_kernel)
    // përdor kernelin C REAL më lart. Këto s'janë "simulim që pretendon":
    // janë vlera nominale të DEKLARUARA si të pamatura. Sinjali i vërtetë
    // është kernel_hw_available()=false → hw_real=false rrjedh në gate dhe
    // asnjë fazë s'i trajton këto si matje reale. ts_ns=0 = "pa stampë reale".
    #[cfg(not(feature = "hw_kernel"))]
    {
        let _ = _tok;
        Ok(CHwSnapshot {
            ram_free_bytes:    2u64 * 1024 * 1024 * 1024,   // nominal (i pamatur)
            ram_total_bytes:   8u64 * 1024 * 1024 * 1024,   // nominal (i pamatur)
            cpu_load_pct:      30.0,                          // nominal (i pamatur)
            cores_active:      4,
            cores_total:       4,
            energy_margin_pct: 0.85,
            on_battery:        false,
            _pad:              [0u8; 7],
            ts_ns:             0,                             // 0 = pa stampë reale
        })
    }
}

/// Lexon gjendjen termike
pub fn kernel_thermal_read(
    _tok: &HwLgcToken,
    hw:   &CHwSnapshot,
) -> Result<CThermalSnapshot, HwLgcError> {
    #[cfg(feature = "hw_kernel")]
    {
        let mut snap = CThermalSnapshot::default();
        let rc = unsafe {
            hw_thermal_read(hw, core::ptr::null(), &mut snap)
        };
        match rc != 0 {
            true => return Err(HwLgcError::CKernelInvalid(rc)),
            false => {}
        }
        // Validim
        match snap.t_limit_c <= 0.0 {
            true => return Err(HwLgcError::InvariantViolation("t_limit_c ≤ 0")),
            false => {}
        }
        match snap.p_total_w < 0.0 {
            true => return Err(HwLgcError::InvariantViolation("p_total_w < 0")),
            false => {}
        }
        Ok(snap)
    }

    #[cfg(not(feature = "hw_kernel"))]
    {
        let _ = (_tok, hw);
        // FALLBACK NOMINAL (hw_kernel OFF): termika nga formula matematike mbi
        // snapshot-in nominal. sensor_used=false = DEKLARIM se s'ka sensor real.
        let p_cpu = 65.0f32 * (hw.cpu_load_pct / 100.0);
        let ram_used_gb = (hw.ram_total_bytes.saturating_sub(hw.ram_free_bytes))
            as f32 / (1024.0 * 1024.0 * 1024.0);
        let p_mem   = ram_used_gb * 3.0;
        let p_total = p_cpu + p_mem;
        let t_est   = 25.0 + p_total * 0.5;
        let t_limit = 100.0 * 0.85;
        Ok(CThermalSnapshot {
            t_estimated_c:    t_est,
            t_limit_c:        t_limit,
            t_warn_c:         100.0 * 0.75,
            t_junction_max_c: 100.0,
            p_cpu_w:          p_cpu,
            p_mem_w:          p_mem,
            p_total_w:        p_total,
            sensor_used:      false,
            overheat:         t_est >= t_limit,
            warning:          t_est >= (100.0 * 0.75) && t_est < t_limit,
            _pad:             [0u8; 5],
            ts_ns:            0,
        })
    }
}

/// Ekzekuton veprimin e ftohjes
pub fn kernel_colddown_execute(
    _tok:   &HwLgcToken,
    action: CColdDownAction,
) -> Result<CColdDownResult, HwLgcError> {
    #[cfg(feature = "hw_kernel")]
    {
        let mut result = CColdDownResult::default();
        let rc = unsafe { hw_colddown_execute(action, &mut result) };
        result.rc = rc;
        Ok(result)
    }

    #[cfg(not(feature = "hw_kernel"))]
    {
        let _ = _tok;
        // FALLBACK NOMINAL (hw_kernel OFF): asnjë veprim fizik nuk u aplikua.
        // Mos prodho boolean-e/fan percentage që duken si kontroll real.
        let _ = action;
        Ok(CColdDownResult {
            rc:              -2, // provider fizik i padisponueshëm
            fan_set:         false,
            cooling_set:     false,
            throttle_set:    false,
            fan_pct_applied: 0,
        })
    }
}

/// Kthen sistemin në gjendje normale
pub fn kernel_restore_normal(_tok: &HwLgcToken) -> Result<(), HwLgcError> {
    #[cfg(feature = "hw_kernel")]
    {
        let rc = unsafe { hw_colddown_restore_normal() };
        match rc != 0 {
            true => return Err(HwLgcError::CKernelInvalid(rc)),
            false => {}
        }
        Ok(())
    }
    #[cfg(not(feature = "hw_kernel"))]
    {
        let _ = _tok;
        Ok(())
    }
}

/// Tregon nëse build-i përdor provider-in REAL të snapshot-it hardware.
/// Kjo nuk është e njëjtë me disponueshmërinë e sysfs/fan control: në Windows
/// snapshot-i është real edhe kur sysfs cooling nuk ekziston.
pub fn kernel_hw_available() -> bool {
    #[cfg(feature = "hw_kernel")]
    {
        true
    }
    #[cfg(not(feature = "hw_kernel"))]
    {
        false
    }
}

/// Aftësi e veçantë: a mund të aplikohet cooling/throttle përmes kernelit C.
/// `false` nuk e zhvlerëson snapshot-in real (p.sh. Windows pa sysfs).
pub fn kernel_cooling_available() -> bool {
    #[cfg(feature = "hw_kernel")]
    {
        unsafe { hw_colddown_sysfs_available() > 0 }
    }
    #[cfg(not(feature = "hw_kernel"))]
    {
        false
    }
}

#[cfg(all(test, not(feature = "hw_kernel")))]
mod fallback_tests {
    use super::*;

    #[test]
    fn nominal_fallback_never_claims_physical_cooling() {
        let token = hw_lgc_open(S_COLD_DOWN, S_COLD_DOWN).expect("token");
        let result = kernel_colddown_execute(&token, CColdDownAction::Emergency)
            .expect("typed fallback result");
        assert_ne!(result.rc, 0);
        assert!(!result.fan_set);
        assert!(!result.cooling_set);
        assert!(!result.throttle_set);
        assert_eq!(result.fan_pct_applied, 0);
        assert!(!kernel_hw_available());
        assert!(!kernel_cooling_available());
    }
}
