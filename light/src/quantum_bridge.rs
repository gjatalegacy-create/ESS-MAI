// quantum_bridge.rs — Light Platform → Quantum Communication
// GJATA LEGACY™ — Light Platform 
//
// Entry point i Light-it për komunikim me Quantum Platform.
// Pasqyron saktësisht formatin e quantum_platform::light_bridge.
//
// FLUKSI:
//   1. send_to_quantum(response, text)
//      → serialize QuantumInput payload
//      → light_buss_send("light_coordinator", "quantum_platform", payload, HIGH)
//
//   2. receive_from_quantum(timeout_secs)
//      → light_buss_recv("light_coordinator", msg_out) me retry loop
//      → deserialize QuantumOutput
//      → return QuantumResult
//
// FORMAT payload dërguar (saktë si quantum::types::QuantumInput::deserialize):
//   "trace_id={};domain={};contract_id={};lgc_seal={};lang_code={};evolve={};input_sha256={};ts={};text={}"
//
// FORMAT payload marrë (saktë si quantum::types::QuantumOutput::serialize):
//   "trace_id={};verdict={};pot={:.4};axis={:.1};seal={};sfx={};state={};territory={};depth={};ts={};content={}"

use std::ffi::CString;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use std::thread;

// ============================================================
// §1  KONSTANTET — identike me light_buss.h
// ============================================================

const LB_MAX_MODULE:  usize = 64;
const LB_MAX_PAYLOAD: usize = 2048;

const LB_OK:           i32 = 0;
const LB_ERR_NOT_INIT: i32 = -1;
const LB_ERR_FULL:     i32 = -2;
const LB_ERR_EMPTY:    i32 = -3;

const LB_PRIO_HIGH: u8 = 2;

// Timeout default: 5 sekonda pritje për Quantum response
const QUANTUM_TIMEOUT_SECS: u64 = 5;
// Poll interval: 50ms ndërmjet tentativave
const QUANTUM_POLL_MS: u64 = 50;

// Emrat e moduleve në bus — saktë si Quantum i pret
pub const MODULE_LIGHT:   &str = "light_coordinator";
pub const MODULE_QUANTUM: &str = "quantum_platform";

// ============================================================
// §2  LightBussMsg — repr(C) identik me light_buss.h
//
// Layout:
//   from[64] + to[64] + payload[2048] + payload_len(u32)
//   + priority(u8) + _pad[3] + ts_ns(u64)
//   + crc32(u32) + _reserved(u32) = 2200 bytes
// ============================================================

#[repr(C)]
pub struct LightBussMsg {
    pub from:        [u8; LB_MAX_MODULE],
    pub to:          [u8; LB_MAX_MODULE],
    pub payload:     [u8; LB_MAX_PAYLOAD],
    pub payload_len: u32,
    pub priority:    u8,
    pub _pad:        [u8; 3],
    pub ts_ns:       u64,
    pub crc32:       u32,
    pub _reserved:   u32,
}

impl LightBussMsg {
    pub fn zeroed() -> Self {
        // SAFETY: all-zero valid per repr(C) struct
        unsafe { std::mem::zeroed() }
    }

    pub fn set_str(buf: &mut [u8; LB_MAX_MODULE], s: &str) {
        buf.fill(0);
        let b = s.as_bytes();
        let n = b.len().min(LB_MAX_MODULE - 1);
        buf[..n].copy_from_slice(&b[..n]);
    }

    /// PËRDORIM I BRENDSHËM: vetëm mbi buffer-a të shkruar nga write_str
    /// (UTF-8 i garantuar). Për bytes që vijnë nga bus-i C i jashtëm përdor
    /// GJITHMONË variantet *_checked — korrupsioni refuzohet, s'maskohet.
    pub fn read_str(buf: &[u8; LB_MAX_MODULE]) -> &str {
        let end = buf.iter().position(|&b| b == 0).unwrap_or(LB_MAX_MODULE);
        std::str::from_utf8(&buf[..end]).unwrap_or("")
    }

    /// KUFI I JASHTËM (fail-closed): UTF-8 i prishur → None (LB_ERR_UTF8).
    pub fn read_str_checked(buf: &[u8; LB_MAX_MODULE]) -> Option<&str> {
        let end = buf.iter().position(|&b| b == 0).unwrap_or(LB_MAX_MODULE);
        match std::str::from_utf8(&buf[..end]) {
            Ok(s)  => Some(s),
            Err(_) => None,
        }
    }

    /// PËRDORIM I BRENDSHËM — shih shënimin te read_str.
    pub fn payload_str(&self) -> &str {
        let len = (self.payload_len as usize).min(LB_MAX_PAYLOAD);
        std::str::from_utf8(&self.payload[..len])
            .unwrap_or("")
            .trim_end_matches('\0')
    }

    /// KUFI I JASHTËM (fail-closed): UTF-8 i prishur → None (LB_ERR_UTF8).
    pub fn payload_str_checked(&self) -> Option<&str> {
        let len = (self.payload_len as usize).min(LB_MAX_PAYLOAD);
        match std::str::from_utf8(&self.payload[..len]) {
            Ok(s)  => Some(s.trim_end_matches('\0')),
            Err(_) => None,
        }
    }
}

// ============================================================
// §3  extern "C" — FFI declarations (light_buss.c)
//     Aktiv vetëm me feature = "c_kernel"
// ============================================================

#[cfg(feature = "c_kernel")]
extern "C" {
    fn light_buss_init() -> i32;
    fn light_buss_send(
        from:     *const std::ffi::c_char,
        to:       *const std::ffi::c_char,
        payload:  *const u8,
        len:      u32,
        priority: u8,
    ) -> i32;
    fn light_buss_recv(
        target:  *const std::ffi::c_char,
        msg_out: *mut LightBussMsg,
    ) -> i32;
    fn light_buss_destroy();
}

// ============================================================
// §4  QBridgeError
// ============================================================

#[derive(Debug, Clone)]
pub enum QBridgeError {
    /// Bus jo i inicializuar
    BussNotInit,
    /// Bus plot — nuk mund të dërgojë
    BussFull,
    /// Timeout — Quantum nuk u përgjigj
    Timeout(u64),
    /// Payload shumë i madh (> 2048 bytes)
    PayloadTooLarge(usize),
    /// Deserializim i QuantumOutput dështoi
    DeserializeFail(String),
    /// CString konvertim dështoi
    StringConvert,
    /// Gabim tjetër i bus
    BussError(i32),
    /// FAIL-CLOSED (#8): bus real mungon dhe dev_simulation s'është aktiv.
    /// Në prodhim, mungesa e c_kernel = gabim kritik, JO no-op i heshtur.
    BusUnavailable,
}

impl std::fmt::Display for QBridgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BussNotInit           => write!(f, "QBRIDGE:NOT_INIT"),
            Self::BussFull              => write!(f, "QBRIDGE:FULL"),
            Self::Timeout(ms)           => write!(f, "QBRIDGE:TIMEOUT:{}ms", ms),
            Self::PayloadTooLarge(n)    => write!(f, "QBRIDGE:PAYLOAD_TOO_LARGE:{}", n),
            Self::DeserializeFail(s)    => write!(f, "QBRIDGE:DESER:{}", s),
            Self::StringConvert         => write!(f, "QBRIDGE:CSTRING_FAIL"),
            Self::BussError(rc)         => write!(f, "QBRIDGE:BUSS_RC:{}", rc),
            Self::BusUnavailable        => write!(f, "QBRIDGE:BUS_UNAVAILABLE"),
        }
    }
}

// ============================================================
// §5  QuantumResult — output nga Quantum Platform
//
// Deserialized from Quantum's QuantumOutput::serialize() format:
//   "trace_id={};verdict={};pot={:.4};axis={:.1};seal={};
//    sfx={};state={};territory={};depth={};ts={};content={}"
// ============================================================

// ─────────────────────────────────────────────────────────────────────────────
// PATCH 7.2 — AlgorithmStateTag: tipizim i 'algorithm_state' (shablloni 7.3)
// ─────────────────────────────────────────────────────────────────────────────

/// Gjendja e algoritmit, E TIPIZUAR (jo String i papërpunuar).
///
/// Para patch-it, 'algorithm_state' pranohej si tekst i lirë nga Quantum.
/// Tani tipizohet në enum — i njëjti standard si bridge_shadow (7.3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlgorithmStateTag {
    /// Primitivi u vulos (SEALED).
    Sealed,
    /// U refuzua (REJECTED).
    Rejected,
    /// Kaloi te Shadow për verifikim (SHADOW_VERIFY).
    ShadowVerify,
    /// Gjendje e panjohur (fallback).
    Unknown,
}

impl AlgorithmStateTag {
    /// from_wire — parson string-un në enum të tipizuar. Zero if — match.
    pub fn from_wire(s: &str) -> Self {
        match s {
            "SEALED"        => AlgorithmStateTag::Sealed,
            "REJECTED"      => AlgorithmStateTag::Rejected,
            "SHADOW_VERIFY" => AlgorithmStateTag::ShadowVerify,
            _               => AlgorithmStateTag::Unknown,
        }
    }

    /// as_str — kthen formën string (për serializim). Zero if — match.
    pub fn as_str(self) -> &'static str {
        match self {
            AlgorithmStateTag::Sealed       => "SEALED",
            AlgorithmStateTag::Rejected     => "REJECTED",
            AlgorithmStateTag::ShadowVerify => "SHADOW_VERIFY",
            AlgorithmStateTag::Unknown      => "UNKNOWN",
        }
    }

    /// goes_to_shadow — a duhet të kalojë te Shadow? Zero if — match.
    pub fn goes_to_shadow(self) -> bool {
        match self {
            AlgorithmStateTag::ShadowVerify => true,
            _                               => false,
        }
    }
}

#[derive(Debug)]
pub struct QuantumResult {
    /// Trace ID — i njëjtë me input (chain continuity)
    pub trace_id:        String,
    /// OPTIMAL | NEGATIVE | HOLD | DISMISSED
    pub verdict:         String,
    /// Potentiality [0,1] from PIM
    pub potentiality:    f32,
    /// Axis score [-100,+100] from PRO
    pub axis_score:      f32,
    /// KODUNIK seal nga Quantum (QNT:...)
    pub seal:            String,
    /// DSL suffix: VFYKNG (verified) ose NEGKNG (negative)
    pub evolve_sfx:      String,
    /// AlgorithmState: SEALED | REJECTED | SHADOW_VERIFY
    pub algorithm_state: String,
    /// Territory: INDUSTRIAL | SEMANTIC | PHILOSOPHICAL | ...
    pub territory:       String,
    /// Reasoning depth: SHALLOW | MEDIUM | DEEP | VERY_DEEP
    pub depth_hint:      String,
    /// Content — reasoning summary
    pub content:         String,
    /// true = shadow bit 1 (Optimal)
    pub is_verified:     bool,
}

// FIX #1 — unescape për fusha free-text (simetrik me quantum::escape_field).
/// escape_field — zëvendëson delimiterët e protokollit. Simetrik me unescape. Zero if.
fn escape_field(s: &str) -> String {
    s.replace('\\', "\\\\")
     .replace(';', "\\s")
     .replace('=', "\\e")
     .replace('|', "\\p")
     .replace(':', "\\c")
}

/// Kthen sekuencat \s \e \p \c në ; = | : . Zero if — fold state machine.
fn unescape_field(s: &str) -> String {
    let (out, _) = s.chars().fold(
        (String::new(), false),
        |(mut acc, esc), ch| match esc {
            true => {
                let decoded = match ch {
                    's'   => ';',
                    'e'   => '=',
                    'p'   => '|',
                    'c'   => ':',
                    '\\'  => '\\',
                    other => other,
                };
                acc.push(decoded);
                (acc, false)
            }
            false => match ch {
                '\\' => (acc, true),
                _    => { acc.push(ch); (acc, false) }
            },
        },
    );
    out
}

impl QuantumResult {
    pub fn from_payload(s: &str) -> Result<Self, QBridgeError> {
        let mut trace_id        = String::new();
        let mut verdict         = String::from("DISMISSED");
        let mut potentiality    = 0.0f32;
        let mut axis_score      = 0.0f32;
        let mut seal            = String::new();
        let mut evolve_sfx      = String::from("NEGKNG");
        let mut algorithm_state = String::from("REJECTED");
        let mut territory       = String::from("INDUSTRIAL");
        let mut depth_hint      = String::from("SHALLOW");
        let mut content         = String::new();

        for part in s.split(';') {
            match part.split_once('=') {
                Some((k, v)) => {
                match k {
                    "trace_id"  => trace_id        = unescape_field(v),
                    "verdict"   => verdict         = unescape_field(v),
                    // FAIL-CLOSED: vlerë numerike e prishur = payload i refuzuar
                    // me reason — kurrë 0.0 e butë (korrupsioni s'maskohet).
                    "pot"       => potentiality = match v.parse() {
                        Ok(x)  => x,
                        Err(_) => return Err(QBridgeError::DeserializeFail(
                            format!("LB_ERR_NUM: pot i prishur '{}'", v))),
                    },
                    "axis"      => axis_score = match v.parse() {
                        Ok(x)  => x,
                        Err(_) => return Err(QBridgeError::DeserializeFail(
                            format!("LB_ERR_NUM: axis i prishur '{}'", v))),
                    },
                    "seal"      => seal            = unescape_field(v),
                    "sfx"       => evolve_sfx      = unescape_field(v),
                    "state"     => algorithm_state = unescape_field(v),
                    "territory" => territory       = unescape_field(v),
                    "depth"     => depth_hint      = unescape_field(v),
                    "content"   => content         = unescape_field(v),
                    _           => {}
                }
                }
                None => {}
            }
        }

        // FAIL-CLOSED (#3): fushat kritike duhet të jenë të pranishme.
        // Para: vetëm trace_id; payload i prishur kthente Ok me DISMISSED/0.0 (sukses i rremë).
        // Tani: trace_id, verdict, seal, state, content të detyrueshme. Zero if — match.
        let missing = match trace_id.is_empty() {
            true  => Some("trace_id"),
            false => match verdict.is_empty() {
                true  => Some("verdict"),
                false => match seal.is_empty() {
                    true  => Some("seal"),
                    false => match algorithm_state.is_empty() {
                        true  => Some("state"),
                        false => match content.is_empty() {
                            true  => Some("content"),
                            false => None,
                        },
                    },
                },
            },
        };

        match missing {
            Some(field) => return Err(QBridgeError::DeserializeFail(
                format!("missing critical field: {field}")
            )),
            None => {}
        }

        // RUNTIME HARDENING (      , nga GPT runtime_patch): validim i vlerave.
        // verdict duhet të jetë një nga vlerat e njohura — fail-closed. Zero if — match.
        let verdict_ok = match verdict.as_str() {
            "OPTIMAL" | "NEGATIVE" | "HOLD" | "DISMISSED" => true,
            _ => false,
        };
        match verdict_ok {
            true => {}
            false => return Err(QBridgeError::DeserializeFail(
                format!("invalid verdict: {verdict}")
            )),
        }

        // algorithm_state duhet të parsohet në tag të njohur (jo Unknown). Zero if — match.
        let state_tag = AlgorithmStateTag::from_wire(&algorithm_state);
        match state_tag == AlgorithmStateTag::Unknown {
            true => return Err(QBridgeError::DeserializeFail(
                format!("invalid algorithm_state: {algorithm_state}")
            )),
            false => {}
        }

        let is_verified = verdict == "OPTIMAL";

        Ok(QuantumResult {
            trace_id, verdict, potentiality, axis_score,
            seal, evolve_sfx, algorithm_state,
            territory, depth_hint, content, is_verified,
        })
    }

    /// PATCH 7.2 — algorithm_state_typed: kthen gjendjen e TIPIZUAR.
    ///
    /// Konsumatorët marrin enum (AlgorithmStateTag) në vend të String-ut të lirë.
    /// Zero if.
    pub fn algorithm_state_typed(&self) -> AlgorithmStateTag {
        AlgorithmStateTag::from_wire(&self.algorithm_state)
    }
}

// ============================================================
// §6  Bus safe wrappers — thin layer mbi extern "C"
// ============================================================

/// Initialize light_buss — thirret 1 herë në startup
pub fn buss_init() -> Result<(), QBridgeError> {
    #[cfg(feature = "c_kernel")]
    {
        let rc = unsafe { light_buss_init() };
        return match rc {
            LB_OK           => Ok(()),
            LB_ERR_NOT_INIT => Err(QBridgeError::BussNotInit),
            other           => Err(QBridgeError::BussError(other)),
        };
    }
    // RUNTIME DEFAULT = EKZEKUTIV: prodhimi ndërtohet me `c_kernel` → përdoret
    // ura REALE C më sipër (light_buss_*). Simulimi no-op kompilohet VETËM me
    // `dev_simulation` eksplicit (testim lokal) dhe NUK ekziston në binarin e
    // prodhimit. Pa c_kernel dhe pa dev_simulation → fail-closed (Err kritik).
    #[cfg(all(not(feature = "c_kernel"), feature = "dev_simulation"))]
    {
        Ok(()) // dev_simulation — no-op i lejuar VETËM në testim lokal
    }
    #[cfg(all(not(feature = "c_kernel"), not(feature = "dev_simulation")))]
    {
        Err(QBridgeError::BusUnavailable) // RUNTIME/PRODHIM: bus real mungon → kritik
    }
}

/// Shutdown light_buss
pub fn buss_destroy() {
    #[cfg(feature = "c_kernel")]
    unsafe { light_buss_destroy(); }
}

fn buss_send_raw(
    from:     &str,
    to:       &str,
    payload:  &[u8],
    priority: u8,
) -> Result<(), QBridgeError> {
    match payload.len() > LB_MAX_PAYLOAD {
        true => return Err(QBridgeError::PayloadTooLarge(payload.len())),
        false => {}
    }

    #[cfg(feature = "c_kernel")]
    {
        let from_c = CString::new(from).map_err(|_| QBridgeError::StringConvert)?;
        let to_c   = CString::new(to).map_err(|_| QBridgeError::StringConvert)?;
        let len    = payload.len() as u32;
        let rc = unsafe {
            light_buss_send(from_c.as_ptr(), to_c.as_ptr(),
                            payload.as_ptr(), len, priority)
        };
        return match rc {
            LB_OK        => Ok(()),
            LB_ERR_FULL  => Err(QBridgeError::BussFull),
            other        => Err(QBridgeError::BussError(other)),
        };
    }

    #[cfg(all(not(feature = "c_kernel"), feature = "dev_simulation"))]
    {
        let _ = (from, to, payload, priority);
        Ok(()) // dev_simulation — log only (lejuar vetëm në testim)
    }
    #[cfg(all(not(feature = "c_kernel"), not(feature = "dev_simulation")))]
    {
        let _ = (from, to, payload, priority);
        Err(QBridgeError::BusUnavailable) // PRODHIM: bus real mungon → kritik
    }
}

fn buss_recv_raw(target: &str) -> Result<Option<LightBussMsg>, QBridgeError> {
    #[cfg(feature = "c_kernel")]
    {
        let target_c = CString::new(target).map_err(|_| QBridgeError::StringConvert)?;
        let mut msg  = LightBussMsg::zeroed();
        let rc = unsafe { light_buss_recv(target_c.as_ptr(), &mut msg) };
        return match rc {
            LB_OK         => Ok(Some(msg)),
            LB_ERR_EMPTY  => Ok(None),
            other         => Err(QBridgeError::BussError(other)),
        };
    }

    #[cfg(all(not(feature = "c_kernel"), feature = "dev_simulation"))]
    {
        let _ = target;
        Ok(None) // dev_simulation — no messages (lejuar vetëm në testim)
    }
    #[cfg(all(not(feature = "c_kernel"), not(feature = "dev_simulation")))]
    {
        let _ = target;
        Err(QBridgeError::BusUnavailable) // PRODHIM: bus real mungon → kritik
    }
}

// ============================================================
// §7  send_to_quantum() — ENTRY POINT 1
//
// Builds QuantumInput payload and sends via light_buss HIGH.
// Called from LightCoordinator::dispatch_to_quantum().
//
// Payload format (saktë si Quantum QuantumInput::deserialize):
//   trace_id=...;domain=...;contract_id=...;lgc_seal=...;
//   lang_code=...;evolve=...;input_sha256=...;ts=...;text=...
// ============================================================

pub fn send_to_quantum(
    trace_id:       &str,
    domain:         &str,
    contract_id:    &str,
    lgc_seal:       &str,
    lang_code:      &str,
    evolve_summary: &str,
    input_sha256:   &str,
    text:           &str,
) -> Result<(), QBridgeError> {

    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    // Saktë si Quantum::QuantumInput::deserialize() pret.
    // RUNTIME HARDENING (      ): escape i të gjitha fushave (delimiter injection).
    let payload = format!(
        "trace_id={};domain={};contract_id={};lgc_seal={};lang_code={};evolve={};input_sha256={};ts={};text={}",
        escape_field(trace_id),
        escape_field(domain),
        escape_field(contract_id),
        escape_field(lgc_seal),
        escape_field(lang_code),
        escape_field(evolve_summary),
        escape_field(input_sha256),
        ts,
        escape_field(text),
    );

    let bytes = payload.as_bytes();
    buss_send_raw(MODULE_LIGHT, MODULE_QUANTUM, bytes, LB_PRIO_HIGH)
}


/// send_project_to_quantum — e njëjta rrjedhë Light→Quantum, por me
/// ProjectContextWitness të lëshuar nga Shadow main. Evidenca shkencore nuk
/// krijon kanal paralel: ajo udhëton si pjesë opsionale e inputit që më pas
/// paketohet në FinalEvidenceWire nën GCL.
pub fn build_project_payload(
    trace_id: &str,
    domain: &str,
    contract_id: &str,
    lgc_seal: &str,
    lang_code: &str,
    evolve_summary: &str,
    input_sha256: &str,
    text: &str,
    witness: &shadow_contracts::ProjectContextWitnessWire,
    title: &str,
    hypothesis: &str,
    assumptions: &[String],
    description: &str,
    files: &[(u8, Vec<u8>)],
) -> String {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let assumptions_wire = assumptions.join("\n");
    let files_wire = crate::gcl_project_contract::encode_files_hex(files);
    format!(
        concat!(
            "trace_id={};domain={};contract_id={};lgc_seal={};lang_code={};",
            "evolve={};input_sha256={};ts={};text={};",
            "project_id={};project_user_id={};project_trace_id={};",
            "project_revision={};project_content_sha256={};",
            "project_sovereign_flags={};project_context_sha256={};",
            "project_title={};project_hypothesis={};project_assumptions={};",
            "project_description={};project_files={}"
        ),
        escape_field(trace_id), escape_field(domain), escape_field(contract_id),
        escape_field(lgc_seal), escape_field(lang_code), escape_field(evolve_summary),
        escape_field(input_sha256), ts, escape_field(text),
        witness.project_id, witness.user_id, witness.trace_id, witness.revision,
        escape_field(&witness.content_sha256), witness.light_sovereign_flags,
        escape_field(&witness.context_sha256), escape_field(title),
        escape_field(hypothesis), escape_field(&assumptions_wire),
        escape_field(description), escape_field(&files_wire),
    )
}

/// Variant legacy mbi bus-in 2048-byte. Projektet reale të v1.6.4 përdorin
/// `project_process_bridge::execute_project_with_quantum_main`, sepse materialet
/// shkencore/dokumentet nuk duhet të priten nga kufiri i bus-it të vjetër.
pub fn send_project_to_quantum(
    trace_id: &str, domain: &str, contract_id: &str, lgc_seal: &str,
    lang_code: &str, evolve_summary: &str, input_sha256: &str, text: &str,
    witness: &shadow_contracts::ProjectContextWitnessWire, title: &str,
    hypothesis: &str, assumptions: &[String], description: &str,
    files: &[(u8, Vec<u8>)],
) -> Result<(), QBridgeError> {
    let payload = build_project_payload(
        trace_id, domain, contract_id, lgc_seal, lang_code, evolve_summary,
        input_sha256, text, witness, title, hypothesis, assumptions, description, files,
    );
    buss_send_raw(MODULE_LIGHT, MODULE_QUANTUM, payload.as_bytes(), LB_PRIO_HIGH)
}

// ============================================================
// §8  receive_from_quantum() — ENTRY POINT 2 (callback)
//
// Polls light_buss for response from quantum_platform.
// Non-blocking per tentativë, timeout me retry loop.
// Called from LightCoordinator::wait_quantum_response().
// ============================================================

pub fn receive_from_quantum(
    timeout_secs: u64,
) -> Result<QuantumResult, QBridgeError> {

    let timeout  = Duration::from_secs(timeout_secs);
    let poll_int = Duration::from_millis(QUANTUM_POLL_MS);
    let start    = Instant::now();

    loop {
        match buss_recv_raw(MODULE_LIGHT)? {
            Some(msg) => {
                // KUFI I JASHTËM: sender-i vjen nga bus-i C — checked.
                // UTF-8 i prishur = mesazh i refuzuar ME EMËR (jo "" e heshtur).
                let from = match LightBussMsg::read_str_checked(&msg.from) {
                    Some(f) => f,
                    None => {
                        eprintln!("[QBRIDGE] MESAZH I REFUZUAR: sender UTF8 i prishur (LB_ERR_UTF8)");
                        match start.elapsed() >= timeout { true => break, false => {} }
                        thread::sleep(poll_int);
                        continue;
                    }
                };
                match from != MODULE_QUANTUM {
                    true => {
                    // Mesazh nga modul tjetër — injoro, vazhdo polling
                    match start.elapsed() >= timeout { true => break, false => {} }
                    thread::sleep(poll_int);
                    continue;
                    }
                    false => {}
                }
                // KUFI I JASHTËM: payload nga bus-i C — checked, fail-closed.
                let payload_str = match msg.payload_str_checked() {
                    Some(p) => p,
                    None => return Err(QBridgeError::DeserializeFail(
                        "LB_ERR_UTF8: payload i prishur nga bus-i".to_string())),
                };
                return QuantumResult::from_payload(payload_str);
            }
            None => {
                // Bus bosh — kontrollo timeout
                match start.elapsed() >= timeout {
                    true => break,
                    false => {}
                }
                thread::sleep(poll_int);
            }
        }
    }

    let elapsed_ms = start.elapsed().as_millis() as u64;
    Err(QBridgeError::Timeout(elapsed_ms))
}

// ============================================================
// §9  full_quantum_cycle() — cikli i plotë Light→Quantum→Light
//
// Kombinon send + receive në 1 thirrje.
// Timeout default = QUANTUM_TIMEOUT_SECS (5s).
// Called from LightCoordinator.
// ============================================================

pub fn full_quantum_cycle(
    trace_id:       &str,
    domain:         &str,
    contract_id:    &str,
    lgc_seal:       &str,
    lang_code:      &str,
    evolve_summary: &str,
    input_sha256:   &str,
    text:           &str,
    timeout_secs:   Option<u64>,
) -> Result<QuantumResult, QBridgeError> {

    let timeout = timeout_secs.unwrap_or(QUANTUM_TIMEOUT_SECS);

    // HAPI 1: Dërgo te Quantum
    send_to_quantum(
        trace_id, domain, contract_id, lgc_seal,
        lang_code, evolve_summary, input_sha256, text,
    )?;

    // HAPI 2: Prit përgjigjen
    receive_from_quantum(timeout)
}
