// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  SHADOW_SNB.RS — shadow_snb (Shadow Negative-Bug memory)              ║
// ║  Gjata Legacy™ | Bledar Gjata                                        ║
// ║                                                                          ║
// ║  Hyrje nga Light (snb_algorithm → ShadowBugReport).            ║
// ║  Filozofia: LIGHT RAPORTON BUG-UN — SHADOW VENDOS DHE E RUAN.          ║
// ║                                                                          ║
// ║  Ligjet supreme:                                                        ║
// ║    • Shadow është i vetmi shkrues i kësaj kujtese sovrane.            ║
// ║    • Gjurmueshmëri PARA ruajtjes: pa flow_trace + përshkrim →          ║
// ║      REFUZOHET (bug pa gjurmë rrjedhe nuk është raport i vlefshëm).   ║
// ║    • Zero-copy (Ligji 0): përshkrimi/gjurma ZHVENDOSEN.               ║
// ║    • Zero if/else në verdikt: pranueshmëri & klasifikim me & + tabela. ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::sovereign_log::{RecReader, RecWriter, SovereignLog};
use std::io;
use std::path::Path;
use std::sync::{OnceLock, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

/// MAGIC/VERSION të dedikuara (që një skedar i huaj të mos lexohet gabimisht).
const SNB_MAGIC:   u64 = 0x474A_534E_4230_0501; // "GJSNB" + 0 05 01
const SNB_VERSION: u32 = 1;

fn now_ns() -> u64 {
    // FAIL-LOUD + MONOTONIK (ULTIMATUM): ora e murit para 1970 → NUK 0 i heshtur.
    // Fallback monotonik jo-zero që ruan lineage-in, me alarm audit. Zero unwrap.
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d)  => d.as_nanos() as u64,
        Err(_) => {
            eprintln!("[APUPK/SNB_TIME] ALARM: ora para UNIX_EPOCH — fallback monotonik (lineage i ruajtur)");
            crate::shadow_pipeline::mark_time_degraded(); // VULA E AUDITIMIT
            static START: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();
            let start = START.get_or_init(std::time::Instant::now);
            1_000_000_000u64.saturating_add(start.elapsed().as_nanos() as u64)
        }
    }
}

/// Pasqyrë e `ShadowBugReport` (raporti që Light dërgon te Shadow).
#[derive(Debug, Clone)]
pub struct BugInbound {
    pub timestamp_ns: u64,
    pub module_name:  String,
    pub description:  String,
    pub flow_trace:   Vec<String>,
    pub severity:     u8,
}

/// Niveli i ashpërsisë — POLITIKË SOVRANE e Shadow-it (Light jep vetëm `severity:u8`).
/// Kufijtë e bucket-imit i vendos Shadow (1/5/9) — Shadow vendos eskalimin.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SnbSeverity {
    Low      = 0,
    Medium   = 1,
    High     = 2,
    Critical = 3,
}

impl SnbSeverity {
    /// Klasifikim BRANCHLESS: numëron sa kufij kalon `severity` → indeks tabele.
    pub fn classify(severity: u8) -> Self {
        let level = (severity >= 1) as usize + (severity >= 5) as usize + (severity >= 9) as usize;
        const TABLE: [SnbSeverity; 4] = [
            SnbSeverity::Low,
            SnbSeverity::Medium,
            SnbSeverity::High,
            SnbSeverity::Critical,
        ];
        TABLE[level]
    }

    pub fn label(self) -> &'static str {
        match self {
            SnbSeverity::Low      => "LOW",
            SnbSeverity::Medium   => "MEDIUM",
            SnbSeverity::High     => "HIGH",
            SnbSeverity::Critical => "CRITICAL",
        }
    }
}

/// Arsyeja e refuzimit (Shadow vendos).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SnbReject {
    /// Pa gjurmë rrjedhe — s'ka si të lokalizohet bug-u.
    NoFlowTrace,
    /// Përshkrim bosh.
    EmptyDescription,
}

/// Dëftesa që Shadow kthen pas pranimit (vendimi sovran).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SnbReceipt {
    pub level:     SnbSeverity,
    pub frequency: u32, // sa herë u pa i njëjti bug
}

/// Bug i ruajtur (Shadow e zotëron).
#[derive(Debug, Clone)]
pub struct BugEntry {
    pub timestamp_ns:  u64,
    pub module_name:   String,
    pub description:   String,
    pub flow_trace:    Vec<String>,
    pub severity:      u8,
    pub level:         SnbSeverity,
    pub frequency:     u32,
    pub recorded_at_ns: u64,
}

/// shadow_snb — kujtesa sovrane e raporteve të bug-eve.
#[derive(Debug)]
pub struct ShadowSnb {
    entries: RwLock<Vec<BugEntry>>,
    log:     RwLock<Option<SovereignLog>>,
}

impl ShadowSnb {
    const fn new() -> Self {
        Self {
            entries: RwLock::new(Vec::new()),
            log:     RwLock::new(None),
        }
    }

    /// Hap kujtesën me durabilitet WAL: riprodhon bug-et nga log-u, pastaj lidh
    /// log-un për shkrime të ardhshme (log-first).
    pub fn with_disk(path: &Path) -> io::Result<Self> {
        let (log, records) = SovereignLog::open(path, SNB_MAGIC, SNB_VERSION)?;
        let store = Self {
            entries: RwLock::new(Vec::new()),
            log:     RwLock::new(Some(log)),
        };
        for rec in records {
            match decode_snb(&rec) {
                Some((bug, recorded_at)) => { let _ = store.apply(bug, recorded_at); }
                None => eprintln!("[SNB_REPLAY] ALARM: rekord SNB i korruptuar gjatë replay — u kapërcye (WAL i dyshimtë)"),
            }
        }
        Ok(store)
    }

    /// SHADOW VENDOS: pranon vetëm me gjurmë rrjedhe + përshkrim (ZERO if/else
    /// mbi verdiktin), e RUAN në log (log-first), pastaj e aplikon (Ligji 0).
    /// Dedupe me (module_name + description): i njëjti bug → frequency++.
    pub fn report(&self, bug: BugInbound) -> Result<SnbReceipt, SnbReject> {
        // ── Verdikt branchless i pranueshmërisë ────────────────────────────
        let has_flow = !bug.flow_trace.is_empty();
        let has_desc = !bug.description.is_empty();
        let admissible = has_flow & has_desc;

        match admissible {
            false => {
                const REASON: [SnbReject; 2] = [SnbReject::EmptyDescription, SnbReject::NoFlowTrace];
                return Err(REASON[(!has_flow) as usize]); // pa-flow ka përparësi
            }
            true => {}
        }

        let recorded_at = now_ns();

        // ── 1) WAL log-first (best-effort); huazon bug pa e zhvendosur ─────
        {
            let guard = self.log.read().unwrap_or_else(|p| p.into_inner());
            match guard.as_ref() {
                Some(log) => log.append(&encode_snb(&bug, recorded_at)),
                None => eprintln!("[SNB] ALARM: WAL i padisponueshëm gjatë report — persistencë e degraduar (RAM aktiv)"),
            }
        }

        // ── 2) Aplikim në RAM (ZHVENDOS bug) ──────────────────────────────
        Ok(self.apply(bug, recorded_at))
    }

    /// Aplikim në RAM (pa rilogim) — i përdorur edhe nga ripërsëritja.
    /// `recorded_at` jepet shtjellazi që ripërsëritja të jetë deterministike.
    fn apply(&self, bug: BugInbound, recorded_at: u64) -> SnbReceipt {
        let level = SnbSeverity::classify(bug.severity);
        let mut store = self.entries.write().unwrap_or_else(|p| p.into_inner());

        // Dedupe me (module + description) — gjej indeksin para zhvendosjes.
        let pos = store
            .iter()
            .position(|e| e.module_name == bug.module_name && e.description == bug.description);
        match pos {
            Some(i) => {
                let e = &mut store[i];
                e.frequency      = e.frequency.saturating_add(1);
                // Përditëso te ndodhia më e fundit (ZHVENDOS gjurmën e re).
                e.timestamp_ns   = bug.timestamp_ns;
                e.flow_trace     = bug.flow_trace;     // MOVE
                e.severity       = bug.severity;
                e.level          = level;
                e.recorded_at_ns = recorded_at;
                SnbReceipt { level, frequency: e.frequency }
            }
            None => {
                store.push(BugEntry {
                    timestamp_ns:  bug.timestamp_ns,
                    module_name:   bug.module_name,    // MOVE
                    description:   bug.description,     // MOVE
                    flow_trace:    bug.flow_trace,      // MOVE
                    severity:      bug.severity,
                    level,
                    frequency:     1,
                    recorded_at_ns: recorded_at,
                });
                SnbReceipt { level, frequency: 1 }
            }
        }
    }

    /// Numri i bug-eve unikë të ruajtur.
    pub fn count(&self) -> usize {
        self.entries.read().unwrap_or_else(|p| p.into_inner()).len()
    }

    /// Numri i bug-eve të një niveli (p.sh. Critical).
    pub fn count_at_level(&self, level: SnbSeverity) -> usize {
        self.entries.read().unwrap_or_else(|p| p.into_inner())
            .iter().filter(|e| e.level == level).count()
    }

    /// Lexim: kopje e bug-eve të një moduli.
    pub fn by_module(&self, module_name: &str) -> Vec<BugEntry> {
        self.entries.read().unwrap_or_else(|p| p.into_inner())
            .iter().filter(|e| e.module_name == module_name).cloned().collect()
    }
}

// ── Kodimi i rekordit WAL (ngjarje report) ──────────────────────────────────

fn encode_snb(bug: &BugInbound, recorded_at: u64) -> Vec<u8> {
    let mut w = RecWriter::new();
    w.u64(bug.timestamp_ns)
        .str(&bug.module_name)
        .str(&bug.description)
        .str_list(&bug.flow_trace)
        .u8(bug.severity)
        .u64(recorded_at);
    w.finish()
}

fn decode_snb(rec: &[u8]) -> Option<(BugInbound, u64)> {
    let mut r = RecReader::new(rec);
    let timestamp_ns = r.u64()?;
    let module_name  = r.string()?;
    let description  = r.string()?;
    let flow_trace   = r.str_list()?;
    let severity     = r.u8()?;
    let recorded_at  = r.u64()?;
    Some((
        BugInbound { timestamp_ns, module_name, description, flow_trace, severity },
        recorded_at,
    ))
}

// ── Singleton global (një kujtesë SNB sovrane për procesin) ─────────────────
static SNB: OnceLock<ShadowSnb> = OnceLock::new();

pub fn snb_store() -> &'static ShadowSnb {
    SNB.get_or_init(ShadowSnb::new)
}

/// Nis kujtesën globale SNB me durabilitet WAL. Duhet thirrur PARA çdo
/// `snb_store()` (ndryshe singleton-i në-RAM është nisur tashmë).
pub fn init_snb_disk(path: &Path) -> io::Result<()> {
    let store = ShadowSnb::with_disk(path)?;
    SNB
        .set(store)
        .map_err(|_| io::Error::new(io::ErrorKind::AlreadyExists, "snb tashmë e nisur"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bug(module: &str, desc: &str, severity: u8) -> BugInbound {
        BugInbound {
            timestamp_ns: 1000,
            module_name:  module.to_string(),
            description:  desc.to_string(),
            flow_trace:   vec!["router".to_string(), module.to_string()],
            severity,
        }
    }

    #[test]
    fn severity_classification_branchless() {
        assert_eq!(SnbSeverity::classify(0), SnbSeverity::Low);
        assert_eq!(SnbSeverity::classify(3), SnbSeverity::Medium);
        assert_eq!(SnbSeverity::classify(5), SnbSeverity::High);
        assert_eq!(SnbSeverity::classify(8), SnbSeverity::High);
        assert_eq!(SnbSeverity::classify(9), SnbSeverity::Critical);
        assert_eq!(SnbSeverity::classify(255), SnbSeverity::Critical);
    }

    #[test]
    fn stores_traceable_bug() {
        let s = ShadowSnb::new();
        let r = s.report(bug("matrix", "panic në indeks", 7)).unwrap();
        assert_eq!(r.level, SnbSeverity::High);
        assert_eq!(r.frequency, 1);
        assert_eq!(s.count(), 1);
    }

    #[test]
    fn same_bug_bumps_frequency() {
        let s = ShadowSnb::new();
        s.report(bug("matrix", "panic", 3)).unwrap();
        let r = s.report(bug("matrix", "panic", 3)).unwrap();
        assert_eq!(r.frequency, 2);
        assert_eq!(s.count(), 1);               // dedupe me (module+desc)
    }

    #[test]
    fn rejects_without_flow_trace() {
        let s = ShadowSnb::new();
        let mut b = bug("matrix", "x", 5);
        b.flow_trace.clear();
        assert_eq!(s.report(b), Err(SnbReject::NoFlowTrace));
        assert_eq!(s.count(), 0);
    }

    #[test]
    fn rejects_empty_description() {
        let s = ShadowSnb::new();
        let b = bug("matrix", "", 5);
        assert_eq!(s.report(b), Err(SnbReject::EmptyDescription));
        assert_eq!(s.count(), 0);
    }

    #[test]
    fn counts_critical_and_by_module() {
        let s = ShadowSnb::new();
        s.report(bug("matrix", "a", 9)).unwrap();   // Critical
        s.report(bug("router", "b", 2)).unwrap();   // Medium
        s.report(bug("matrix", "c", 10)).unwrap();  // Critical
        assert_eq!(s.count_at_level(SnbSeverity::Critical), 2);
        assert_eq!(s.by_module("matrix").len(), 2);
    }

    fn temp(tag: &str) -> std::path::PathBuf {
        let mut p = std::env::temp_dir();
        let ns = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        p.push(format!("snb_{tag}_{ns}.log"));
        p
    }

    #[test]
    fn survives_restart_with_disk() {
        let path = temp("restart");
        {
            let s = ShadowSnb::with_disk(&path).unwrap();
            s.report(bug("matrix", "panic", 9)).unwrap();
            s.report(bug("router", "overflow", 2)).unwrap();
            let r = s.report(bug("matrix", "panic", 9)).unwrap(); // i njëjti bug
            assert_eq!(r.frequency, 2);
            assert_eq!(s.count(), 2);
        }
        {
            // Ripërsëritja riprodhon gjendjen (dedupe + frequency + level).
            let s = ShadowSnb::with_disk(&path).unwrap();
            assert_eq!(s.count(), 2);
            assert_eq!(s.count_at_level(SnbSeverity::Critical), 1);
            let m = s.by_module("matrix");
            assert_eq!(m.len(), 1);
            assert_eq!(m[0].frequency, 2);
        }
        let _ = std::fs::remove_file(&path);
    }
}
