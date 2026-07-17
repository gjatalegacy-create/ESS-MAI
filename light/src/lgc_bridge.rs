// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LGC_BRIDGE.RS — URA RUST → C KERNEL                                  ║
// ║  GJATA LEGACY™  — ESS MAI                                              ║
// ║                                                                          ║
// ║  Rust zëvendëson Python/ctypes plotësisht.                            ║
// ║  extern "C" = zero-cost, zero-marshal, zero-overhead.                  ║
// ║                                                                          ║
// ║  HIERARKIA:                                                              ║
// ║  ALLAH → BLEDAR GJATA → GJATA LEGACY →                                ║
// ║  SHADOW_GJ_LEGACY → NURA → ALISAH → AMMY                             ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use std::ffi::{CStr, CString};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

// ─────────────────────────────────────────────────────────────────────────────
// FFI — Pasqyrë e shadow_gj_legacy.h (zero-cost, direkt)
// ─────────────────────────────────────────────────────────────────────────────

const LGC_ID_LEN: usize = 48;
const LGC_MOD_NAME: usize = 64;
const LGC_REASON_LEN: usize = 128;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum LgcDecision {
    Block  = 0,
    Pass   = 1,
    Refine = 2,
    Wait   = 3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum LgcState {
    Light     = 0,
    Pressured = 1,
    Absorbing = 2,
    Submodule = 3,
    FastPath  = 4,
}

#[repr(C)]
pub struct LgcRequest {
    pub req_id:    [u8; LGC_ID_LEN],
    pub caller:    [u8; LGC_MOD_NAME],
    pub sha256:    [u8; 65],
    pub ram_usage: f32,
    pub flags:     u32,
    pub ts:        u64,
}

#[repr(C)]
pub struct LgcResult {
    pub req_id:   [u8; LGC_ID_LEN],
    pub decision: LgcDecision,
    pub state:    LgcState,
    pub sha256:   [u8; 65],
    pub reason:   [u8; LGC_REASON_LEN],
    pub ts:       u64,
}

#[repr(C)]
pub struct LgcStats {
    pub total:   u64,
    pub passed:  u64,
    pub blocked: u64,
    pub state:   i32,
    pub frozen:  i32,
}

// ─────────────────────────────────────────────────────────────────────────────
// EXTERN "C" — Lidhja direkte me shadow_gj_legacy.c
// Kompilo: gcc -shared -fPIC -o libnura_kernel.so shadow_gj_legacy.c buss_legacy.c
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "c_kernel")]
#[link(name = "light_kernel")]
extern "C" {
    pub fn lgc_init() -> i32;
    pub fn lgc_check(req: *const LgcRequest, res: *mut LgcResult) -> i32;
    pub fn lgc_sha256(data: *const u8, len: usize, out: *mut u8);
    pub fn lgc_get_stats(stats: *mut LgcStats);
    pub fn lgc_freeze(stage: i32) -> i32;
    pub fn lgc_unfreeze() -> i32;
}

// ─────────────────────────────────────────────────────────────────────────────
// FALLBACK — Pure Rust SHA256 kur C kernel nuk është i ngarkuar
// (identike me lgc_sha256 në C — garanton rezultat identik)
// ─────────────────────────────────────────────────────────────────────────────

pub fn sha256_internal(data: &[u8]) -> [u8; 32] {
    use sha2::{Sha256, Digest};
    let mut h = Sha256::new();
    h.update(data);
    h.finalize().into()
}

pub fn sha256_hex(data: &[u8]) -> String {
    sha256_internal(data)
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

// ─────────────────────────────────────────────────────────────────────────────
// LGC BRIDGE — Wrapper Rust i sigurt
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct BridgeDecision {
    pub pass:    bool,
    pub state:   String,
    pub reason:  String,
    pub sha256:  String,
}

pub struct LgcBridge {
    initialized: bool,
    c_kernel:    bool,  // true = C kernel aktiv, false = Rust fallback
}

impl LgcBridge {
    /// Singleton init — thirret vetëm 1 herë nga boot
    pub fn new() -> Self {
        #[cfg(feature = "c_kernel")]
        {
            let ok = unsafe { lgc_init() };
            match ok == 0 {
                true => {
                tracing::info!("LGC_BRIDGE: C kernel AKTIV — libnura_kernel.so");
                return Self { initialized: true, c_kernel: true };
                }
                false => {}
            }
        }
        tracing::warn!("LGC_BRIDGE: Rust fallback (C kernel jo i ngarkuar)");
        Self { initialized: true, c_kernel: false }
    }

    /// Verifikim kryesor: çdo request kalon këtu
    /// 0 = BLLOKO, 1 = KALO
    pub fn check(&self, caller: &str, sha256: &str, ram_usage: f32) -> BridgeDecision {
        match self.initialized {
            false => return BridgeDecision {
                pass: false, state: "UNINITIALIZED".into(),
                reason: "Kernel jo i inicializuar".into(), sha256: sha256.into(),
            },
            true => {}
        }

        #[cfg(feature = "c_kernel")]
        match self.c_kernel {
            true => return self._check_c(caller, sha256, ram_usage),
            false => {}
        }

        // Rust fallback — logjika identike me C
        self._check_rust(caller, sha256, ram_usage)
    }

    #[cfg(feature = "c_kernel")]
    fn _check_c(&self, caller: &str, sha256: &str, ram_usage: f32) -> BridgeDecision {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let mut req = LgcRequest {
            req_id:    [0u8; 48],
            caller:    [0u8; 64],
            sha256:    [0u8; 65],
            ram_usage,
            flags: 0,
            ts,
        };

        // Copy strings in fixed buffers
        let cal = caller.as_bytes();
        req.caller[..cal.len().min(63)].copy_from_slice(&cal[..cal.len().min(63)]);
        let sha = sha256.as_bytes();
        req.sha256[..sha.len().min(64)].copy_from_slice(&sha[..sha.len().min(64)]);

        let mut res = LgcResult {
            req_id:   [0u8; 48],
            decision: LgcDecision::Block,
            state:    LgcState::Light,
            sha256:   [0u8; 65],
            reason:   [0u8; 128],
            ts: 0,
        };

        unsafe { lgc_check(&req, &mut res) };

        let pass = matches!(res.decision, LgcDecision::Pass);
        let reason = String::from_utf8_lossy(&res.reason)
            .trim_end_matches('\0')
            .to_string();

        BridgeDecision {
            pass,
            state: format!("{:?}", res.state),
            reason,
            sha256: sha256.into(),
        }
    }

    fn _check_rust(&self, caller: &str, sha256: &str, ram_usage: f32) -> BridgeDecision {
        // Rust fallback — rule: SHA256 duhet të jetë jo bosh + 64 hex chars
        let valid_sha = sha256.len() == 64
            && sha256.chars().all(|c| c.is_ascii_hexdigit());

        let (pass, reason) = match (sha256.is_empty(), !valid_sha, ram_usage > 0.90) {
            (true, _, _) => (false, "SHA256 mungon — BLLOKO".into()),
            (_, true, _) => (false, "SHA256 invalid — BLLOKO".into()),
            (_, _, true) => (false, "RAM kritike — BLLOKO".into()),
            _            => (true, "Rust fallback — KALO".into()),
        };

        // State machine (thjeshtuar pa C)
        let state = match ram_usage > 0.50 { true => "PRESSURED", false => "LIGHT" };

        BridgeDecision {
            pass,
            state: state.into(),
            reason,
            sha256: sha256.into(),
        }
    }

    pub fn sha256_of(&self, data: &[u8]) -> String {
        // FIX #9: C lgc_sha256() është PLACEHOLDER (mbush me zero) — KURRË s'përdoret
        // për hash real. sha256_hex (Rust) është implementimi i vërtetë dhe i vetmi
        // autoritet kriptografik deri sa C kernel të ketë SHA-256 real. Zero if.
        // (Thirrja C u hoq qëllimisht: integriteti > simetria me kernelin placeholder.)
        sha256_hex(data)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// CHECK WITH FLAGS — lidhja: lgc_bridge → legalgocrypt → C kernel
//
// Rruga e thirrjes (call & callback):
//   1. Thirret nga light_coordinator ose pipeline me flags të vulosur
//   2. Nëse c_kernel aktiv → kalon tek _check_c_with_flags (C vendos)
//   3. Nëse Rust fallback → legalgocrypt::unseal_from_flags(flags) verifikon
//   4. Callback: BridgeDecision kthehet tek thirruesi
//
// Vlera 500 = seal valid (5*100, XOR 0xA5A5 në C kernel)
// ─────────────────────────────────────────────────────────────────────────────

impl LgcBridge {
    pub fn check_with_flags(
        &self,
        caller:    &str,
        sha256:    &str,
        ram_usage: f32,
        flags:     u32,   // seal i fshehur brenda flags
    ) -> BridgeDecision {
        // C kernel aktiv → delegohet direkt, C vendos 0/1
        #[cfg(feature = "c_kernel")]
        match self.c_kernel {
            true => return self._check_c_with_flags(caller, sha256, ram_usage, flags),
            false => {}
        }

        // Rust fallback → legalgocrypt::unseal_from_flags callback
        // (flags & 0x0000FFFF) ^ 0xA5A5 → duhet 500
        let seal = crate::legalgocrypt::unseal_from_flags(flags);
        let pass = seal == 500
            && sha256.len() == 64
            && sha256.chars().all(|c| c.is_ascii_hexdigit());

        BridgeDecision {
            pass,
            state:  match pass { true => "SHADOW_SEALED".into(), false => "BLOCK".into() },
            reason: String::new(),
            sha256: sha256.into(),
        }
    }

    #[cfg(feature = "c_kernel")]
    fn _check_c_with_flags(
        &self, caller: &str, sha256: &str, ram_usage: f32, flags: u32,
    ) -> BridgeDecision {
        use std::time::{SystemTime, UNIX_EPOCH};
        let ts = SystemTime::now().duration_since(UNIX_EPOCH)
            .unwrap_or_default().as_secs();

        let mut req = LgcRequest {
            req_id:    [0u8; 48],
            caller:    [0u8; 64],
            sha256:    [0u8; 65],
            ram_usage,
            flags,      // ← 500 fshehur brenda, bus kalon të paprekur
            ts,
        };

        let cal = caller.as_bytes();
        req.caller[..cal.len().min(63)].copy_from_slice(&cal[..cal.len().min(63)]);
        let sha = sha256.as_bytes();
        req.sha256[..sha.len().min(64)].copy_from_slice(&sha[..sha.len().min(64)]);

        let mut res = LgcResult {
            req_id: [0u8; 48], decision: LgcDecision::Block,
            state: LgcState::Light, sha256: [0u8; 65],
            reason: [0u8; 128], ts: 0,
        };

        // shadow_gj_legacy.lgc merr flags → vendos 0/1
        unsafe { lgc_check(&req, &mut res) };

        BridgeDecision {
            pass:   matches!(res.decision, LgcDecision::Pass),
            state:  "SHADOW_DECIDED".into(),
            reason: String::new(),
            sha256: sha256.into(),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// LIGHT BUSS — FFI structs
// Layout identik me light_buss.h (padding explicit — ABI e saktë)
// ─────────────────────────────────────────────────────────────────────────────

pub const LB_MAX_MODULE:  usize = 64;
pub const LB_MAX_PAYLOAD: usize = 2048;
pub const LB_PRIO_RINGS:  usize = 4;

pub const LB_PRIO_LOW:      u8 = 0;
pub const LB_PRIO_NORMAL:   u8 = 1;
pub const LB_PRIO_HIGH:     u8 = 2;
pub const LB_PRIO_CRITICAL: u8 = 3;

pub const LB_OK:            i32 = 0;
pub const LB_ERR_NOT_INIT:  i32 = -1;
pub const LB_ERR_FULL:      i32 = -2;
pub const LB_ERR_EMPTY:     i32 = -3;
pub const LB_ERR_CRC:       i32 = -4;
pub const LB_ERR_PARAM:     i32 = -6;

/// Pasqyrë e LightBussMsg nga light_buss.h
/// Totale: 64+64+2048+4+1+3+8+4+4 = 2200 bytes
#[repr(C)]
#[derive(Clone)]
pub struct LightBussMsgFfi {
    pub from:        [u8; 64],    //  64
    pub to:          [u8; 64],    //  64
    pub payload:     [u8; 2048],  // 2048
    pub payload_len: u32,         //   4
    pub priority:    u8,          //   1
    pub _pad:        [u8; 3],     //   3
    pub ts_ns:       u64,         //   8
    pub crc32:       u32,         //   4
    pub _reserved:   u32,         //   4
}

impl LightBussMsgFfi {
    pub fn zeroed() -> Self {
        Self {
            from:        [0u8; 64],
            to:          [0u8; 64],
            payload:     [0u8; 2048],
            payload_len: 0,
            priority:    0,
            _pad:        [0u8; 3],
            ts_ns:       0,
            crc32:       0,
            _reserved:   0,
        }
    }

    /// Kthim: emri i modulit dërgues si &str
    pub fn from_str(&self) -> &str {
        let end = self.from.iter().position(|&b| b == 0).unwrap_or(64);
        // KUFI: UTF-8 i prishur → shënues korrupsioni EKSPLICIT (jo "?" i errët).
        match std::str::from_utf8(&self.from[..end]) {
            Ok(s)  => s,
            Err(_) => "\u{2620}LGC_ERR_UTF8_FROM", // ☠ — dallohet, s'ngatërrohet me emër real
        }
    }

    /// Kthim: emri i modulit marrës si &str
    pub fn to_str(&self) -> &str {
        let end = self.to.iter().position(|&b| b == 0).unwrap_or(64);
        match std::str::from_utf8(&self.to[..end]) {
            Ok(s)  => s,
            Err(_) => "\u{2620}LGC_ERR_UTF8_TO",
        }
    }

    /// Kthim: slice i payload-it aktiv
    pub fn payload_slice(&self) -> &[u8] {
        &self.payload[..self.payload_len.min(LB_MAX_PAYLOAD as u32) as usize]
    }
}

/// Pasqyrë e LightBussStats nga light_buss.h
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct LightBussStatsFfi {
    pub total_sent:       u64,
    pub total_recv:       u64,
    pub total_dropped:    u64,
    pub crc_errors:       u64,
    pub seal_errors:      u64,
    pub ring_pending:     [u64; 4],
}

// ─────────────────────────────────────────────────────────────────────────────
// LIGHT BUSS — extern "C" FFI
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "c_kernel")]
#[link(name = "light_kernel")]
extern "C" {
    fn light_buss_init() -> i32;

    fn light_buss_send(
        from:     *const std::os::raw::c_char,
        to:       *const std::os::raw::c_char,
        payload:  *const u8,
        len:      u32,
        priority: u8,
    ) -> i32;

    fn light_buss_recv(
        target:  *const std::os::raw::c_char,
        msg_out: *mut LightBussMsgFfi,
    ) -> i32;

    fn light_buss_stats(stats: *mut LightBussStatsFfi);

    fn light_buss_destroy();

    fn light_buss_verify_seal(flags: u32) -> i32;
}

// ─────────────────────────────────────────────────────────────────────────────
// LIGHT BUSS — wrapper Rust i sigurt
// Roli: BUS — kalon mesazhet, nuk interpreton, nuk vendos
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum BussError {
    NotInitialized,
    QueueFull,
    QueueEmpty,
    CrcFailed,
    InvalidParam,
    SealInvalid,
    Unknown(i32),
}

impl BussError {
    fn from_code(code: i32) -> Self {
        match code {
            LB_ERR_NOT_INIT => Self::NotInitialized,
            LB_ERR_FULL     => Self::QueueFull,
            LB_ERR_EMPTY    => Self::QueueEmpty,
            LB_ERR_CRC      => Self::CrcFailed,
            LB_ERR_PARAM    => Self::InvalidParam,
            other           => Self::Unknown(other),
        }
    }
}

impl std::fmt::Display for BussError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotInitialized => write!(f, "BUSS_ERR:NOT_INIT"),
            Self::QueueFull      => write!(f, "BUSS_ERR:FULL"),
            Self::QueueEmpty     => write!(f, "BUSS_ERR:EMPTY"),
            Self::CrcFailed      => write!(f, "BUSS_ERR:CRC"),
            Self::InvalidParam   => write!(f, "BUSS_ERR:PARAM"),
            Self::SealInvalid    => write!(f, "BUSS_ERR:SEAL"),
            Self::Unknown(c)     => write!(f, "BUSS_ERR:UNKNOWN({})", c),
        }
    }
}

pub struct LightBuss;

impl LightBuss {
    /// Inicializo bus-in — thirrur 1 herë nga main boot
    pub fn init() -> Result<(), BussError> {
        #[cfg(feature = "c_kernel")]
        {
            let r = unsafe { light_buss_init() };
            match r != LB_OK { true => return Err(BussError::from_code(r)), false => {} }
        }
        Ok(())
    }

    /// Dërgo mesazh në bus
    ///
    /// from     — emri i modulit dërgues
    /// to       — emri i modulit marrës
    /// payload  — bytes të dhënave
    /// priority — LB_PRIO_LOW..LB_PRIO_CRITICAL
    pub fn send(
        from:     &str,
        to:       &str,
        payload:  &[u8],
        priority: u8,
    ) -> Result<(), BussError> {
        #[cfg(feature = "c_kernel")]
        {
            let from_c = std::ffi::CString::new(from)
                .map_err(|_| BussError::InvalidParam)?;
            let to_c = std::ffi::CString::new(to)
                .map_err(|_| BussError::InvalidParam)?;

            let r = unsafe {
                light_buss_send(
                    from_c.as_ptr(),
                    to_c.as_ptr(),
                    payload.as_ptr(),
                    payload.len() as u32,
                    priority,
                )
            };

            match r != LB_OK { true => return Err(BussError::from_code(r)), false => {} }
        }
        Ok(())
    }

    /// Send Critical — shortcut për komunikim Light→Shadow
    pub fn send_critical(from: &str, to: &str, payload: &[u8]) -> Result<(), BussError> {
        Self::send(from, to, payload, LB_PRIO_CRITICAL)
    }

    /// Send High — shortcut për komunikim Light→Quantum
    pub fn send_high(from: &str, to: &str, payload: &[u8]) -> Result<(), BussError> {
        Self::send(from, to, payload, LB_PRIO_HIGH)
    }

    /// Merr mesazhin e radhës për modulin 'target'
    /// Prioritet: CRITICAL → HIGH → NORMAL → LOW
    pub fn recv(target: &str) -> Result<LightBussMsgFfi, BussError> {
        #[cfg(feature = "c_kernel")]
        {
            let target_c = std::ffi::CString::new(target)
                .map_err(|_| BussError::InvalidParam)?;
            let mut msg = LightBussMsgFfi::zeroed();

            let r = unsafe {
                light_buss_recv(target_c.as_ptr(), &mut msg)
            };

            match r == LB_OK { true => return Ok(msg), false => {} }
            return Err(BussError::from_code(r));
        }
        #[cfg(not(feature = "c_kernel"))]
        Err(BussError::NotInitialized)
    }

    /// Statistikat e bus-it
    pub fn stats() -> LightBussStatsFfi {
        let mut s = LightBussStatsFfi::default();
        #[cfg(feature = "c_kernel")]
        {
            unsafe { light_buss_stats(&mut s) };
        }
        s
    }

    /// Pastro resurset e bus-it — thirrur nga shutdown
    pub fn destroy() {
        #[cfg(feature = "c_kernel")]
        {
            unsafe { light_buss_destroy() };
        }
    }

    /// Verifiko LGC seal
    pub fn verify_seal(flags: u32) -> bool {
        #[cfg(feature = "c_kernel")]
        {
            return unsafe { light_buss_verify_seal(flags) } == 1;
        }
        #[cfg(not(feature = "c_kernel"))]
        {
            // Rust fallback — logjika identike me C
            match flags == 0 { true => return false, false => {} }
            let v = (flags & 0x0000_FFFF) ^ 0xA5A5;
            v == 500
        }
    }
}
