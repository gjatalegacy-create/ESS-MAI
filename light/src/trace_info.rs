// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  TRACE_INFO.RS — ESS-MAI LIGHT PLATFORM                               ║
// ║  Gjata Legacy™  |  Arkitekt: Bledar Gjata                             ║
// ║                                                                          ║
// ║  Gjurma e parë e inputit — stamp fillestar sapo hyn.                  ║
// ║  SHA256 NUK ndodh këtu — ai është kompetencë e Shadow.               ║
// ║  Këtu: FNV64(input) ^ ts_ns → ID unik i lehtë, O(1).                ║
// ║                                                                          ║
// ║  Roli: TRACER — vë vulë, nuk vendos, nuk verifikon.                  ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

// ════════════════════════════════════════════════════════════════════════════
// §1  FNV-1a 64-bit — i njëjtë me quantum_v14_core.rs
//     Inline: zero dipendencë, zero allokacion
// ════════════════════════════════════════════════════════════════════════════

#[inline(always)]
pub fn fnv64(data: &[u8]) -> u64 {
    let mut h: u64 = 14695981039346656037;
    for &b in data {
        h ^= b as u64;
        h = h.wrapping_mul(1099511628211);
    }
    h
}

// ════════════════════════════════════════════════════════════════════════════
// §2  DOMAIN HINT — indikacion i lehtë domeni
//     Koordinohet me Domain në nura_core.rs
//     light_coordinator bën konversionin midis të dyjave
// ════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, PartialEq)]
pub enum DomainHint {
    Work,     // NURA: biznes, industri, governance
    Home,     // ALISAH: familje, shtëpi, IoT/MATTER
    Outside,  // AMMY: mobile, ephemeral web
    Unknown,  // Pa detekim — light_coordinator do vendosë
}

impl DomainHint {
    pub fn label(&self) -> &str {
        match self {
            DomainHint::Work    => "WORK",
            DomainHint::Home    => "HOME",
            DomainHint::Outside => "OUTSIDE",
            DomainHint::Unknown => "UNKNOWN",
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// §3  TRACE INFO — struct kryesor
// ════════════════════════════════════════════════════════════════════════════

/// Counter sekuencial global — atomik, thread-safe, zero lock
static TRACE_SEQ: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone)]
pub struct TraceInfo {
    /// ID unik: fnv64(input) XOR timestamp_ns
    /// SHA-256 real i inputit lind në Light (evolve_trace::input_sha256)
    pub id:          u64,

    /// Timestamp nanoseconds since UNIX_EPOCH
    pub ts_ns:       u64,

    /// Numëruesi sekuencial i sesionit — rrit me 1 për çdo input
    pub seq:         u64,

    /// Gjatësia e inputit të papërpunuar
    pub raw_len:     usize,

    /// Hint i domenit — Unknown deri sa light_coordinator vendos
    pub domain_hint: DomainHint,
}

impl TraceInfo {
    /// Krijon stamp të ri për inputin — O(1), zero allokacion shtesë
    pub fn new(raw_input: &[u8]) -> Self {
        let ts = Self::ts_now();
        let id = fnv64(raw_input) ^ ts;
        let seq = TRACE_SEQ.fetch_add(1, Ordering::SeqCst);

        Self {
            id,
            ts_ns: ts,
            seq,
            raw_len: raw_input.len(),
            domain_hint: DomainHint::Unknown,
        }
    }

    /// Vendos domain hint — merret nga light_coordinator
    pub fn with_domain(mut self, hint: DomainHint) -> Self {
        self.domain_hint = hint;
        self
    }

    /// Timestamp nanoseconds — platformë e pavarur
    fn ts_now() -> u64 {
        let d = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        d.as_secs()
            .wrapping_mul(1_000_000_000)
            .wrapping_add(d.subsec_nanos() as u64)
    }

    /// ID si hex 16 karaktere — për logging dhe evolve_trace
    pub fn id_hex(&self) -> String {
        format!("{:016x}", self.id)
    }

    /// Label kompakt: T{seq}_{8 hex chars}
    /// Shembull: T3_a3f4b2c1
    pub fn label(&self) -> String {
        format!("T{}_{}", self.seq, &self.id_hex()[..8])
    }

    /// Short ID: vetëm 4 hex — për display
    pub fn short_id(&self) -> String {
        format!("{:04x}", self.id & 0xFFFF)
    }

    /// Verifikim i bazë: ID nuk mund të jetë zero
    pub fn is_valid(&self) -> bool {
        self.id != 0 && self.raw_len > 0
    }
}

// ════════════════════════════════════════════════════════════════════════════
// §4  TRACE SESSION — mbledh trace-t e një sesioni
// ════════════════════════════════════════════════════════════════════════════

/// Session trace — ndjek të gjitha trace-t e një konversacioni
#[derive(Debug)]
pub struct TraceSession {
    pub session_id: u64,
    pub traces:     Vec<TraceInfo>,
    pub started_at: u64,
}

impl TraceSession {
    pub fn new() -> Self {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let session_id = fnv64(&ts.to_le_bytes()) ^ TRACE_SEQ.load(Ordering::Relaxed);
        Self {
            session_id,
            traces: Vec::new(),
            started_at: ts,
        }
    }

    /// Shton trace të ri në sesion
    pub fn add(&mut self, trace: TraceInfo) {
        self.traces.push(trace);
    }

    /// Numri i trace-ve aktuale
    pub fn count(&self) -> usize {
        self.traces.len()
    }

    /// Session label
    pub fn label(&self) -> String {
        format!("SESSION_{:08x}", self.session_id & 0xFFFFFFFF)
    }
}

impl Default for TraceSession {
    fn default() -> Self {
        Self::new()
    }
}
