// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  SHADOW_APUPK.RS — shadow_APUPK_memory (Awaken Project User Personal   ║
// ║  Knowledge)   ·   Gjata Legacy™ | Bledar Gjata                       ║
// ║                                                                          ║
// ║  Hyrje nga Light (apupk_coordinator → ShadowApupkPackage).     ║
// ║  Filozofia: LIGHT PËRGATIT DHE ORIENTON — SHADOW VENDOS DHE RUAN.      ║
// ║                                                                          ║
// ║  Ligjet supreme:                                                        ║
// ║    • Shadow është i vetmi shkrues i kësaj kujtese sovrane.            ║
// ║    • Gjurmueshmëri PARA ruajtjes: pa ApupkTrace të vlefshme +          ║
// ║      përmbajtje → REFUZOHET (s'bëhet njohuri).                        ║
// ║    • Zero-copy (Ligji 0): përmbajtja ZHVENDOSET, nuk klonohet.        ║
// ║    • Zero if/else në verdikt: pranueshmëria llogaritet me & boolean.   ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::sovereign_log::{RecReader, RecWriter, SovereignLog};
use std::io;
use std::path::Path;
use std::sync::{OnceLock, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

/// MAGIC/VERSION të dedikuara (që një skedar i huaj të mos lexohet gabimisht).
const APUPK_MAGIC:   u64 = 0x474A_4150_554B_0501; // "GJAPUK" + 05 01
const APUPK_VERSION: u32 = 2;

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

// ── KONTRATA E LIGHT-it (pasqyrë e tipave kufitarë) ─────────────────────────

/// Pasqyrë e `ApupkTrace` të Light-it.
#[derive(Debug, Clone)]
pub struct ApupkTrace {
    pub trace_id:     u64,
    pub project_id:   u64,
    pub user_id:      u64,
    pub initial_trace: String,
    pub timestamp_ns: u64,
}

/// Pasqyrë e `ProjectProgress` të Light-it.
#[derive(Debug, Clone)]
pub struct ProjectProgress {
    pub project_id:          u64,
    pub progress_percentage: f32,
    pub last_update_ns:      u64,
    pub notes:               String,
}

/// Pasqyrë e `ShadowApupkPackage` (paketa që Light dërgon te Shadow).
#[derive(Debug, Clone)]
pub struct ApupkInbound {
    pub trace:                  ApupkTrace,
    pub initial_progress:       ProjectProgress,
    pub project_title:          String,
    pub light_sovereign_flags:  u32,
    pub project_content:        String,
}

/// Arsyeja e refuzimit nga Shadow (Shadow vendos).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ApupkReject {
    /// Pa gjurmë algoritmike (trace_id==0 ose initial_trace bosh).
    NoTrace,
    /// Përmbajtje boshe — s'ka çfarë të ruhet si njohuri.
    EmptyContent,
    /// Titulli është pjesë e identitetit APUPK dhe s'mund të jetë bosh.
    InvalidProjectTitle,
    /// Vula e origjinës Light duhet të dekodohet saktë në 500.
    InvalidSovereignSeal,
    /// `trace_id` duhet të lindë nga project_id + user_id + titulli i Light-it.
    InvalidTraceIdentity,
    /// Një project_id ekzistues nuk mund të kalojë te një përdorues tjetër.
    ProjectOwnershipMismatch,
    /// Progresi NaN/Infinity nuk lejohet në WAL-in sovran.
    InvalidProgress,
    /// Regjistrimi sovran kërkoi WAL, por kujtesa është vetëm në RAM.
    PersistenceUnavailable,
    /// WAL ekziston, por write/flush/fsync dështoi; witness nuk lejohet.
    PersistenceFailed,
}

// ── NJOHURIA E RUAJTUR (Shadow e zotëron) ───────────────────────────────────

#[derive(Debug, Clone)]
pub struct ApupkEntry {
    pub trace_id:            u64,
    pub project_id:          u64,
    pub user_id:             u64,
    pub initial_trace:       String,
    pub project_title:       String,
    pub light_sovereign_flags: u32,
    pub content:             String,
    pub progress_percentage: f32,
    pub last_update_ns:      u64,
    pub stored_at_ns:        u64,
    pub notes:               String,
    /// Sa herë u ringarkua i njëjti projekt (rifreskim sovran).
    pub revisions:           u32,
}

/// shadow_APUPK_memory — kujtesa sovrane e njohurive të projekteve.
#[derive(Debug)]
pub struct ShadowApupkMemory {
    entries: RwLock<Vec<ApupkEntry>>,
    log:     RwLock<Option<SovereignLog>>,
}

impl ShadowApupkMemory {
    const fn new() -> Self {
        Self {
            entries: RwLock::new(Vec::new()),
            log:     RwLock::new(None),
        }
    }

    /// Hap kujtesën me durabilitet WAL: riprodhon entry-t nga log-u, pastaj e
    /// lidh log-un për shkrime të ardhshme (log-first).
    pub fn with_disk(path: &Path) -> io::Result<Self> {
        let (log, records) = SovereignLog::open(path, APUPK_MAGIC, APUPK_VERSION)?;
        let store = Self {
            entries: RwLock::new(Vec::new()),
            log:     RwLock::new(Some(log)),
        };
        // Ripërsëritje: apliko çdo ngjarje store në rend → dedupe+revisions riprodhohen.
        for rec in records {
            let (pkg, stored_at) = decode_apupk(&rec).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "APUPK WAL përmban rekord semantikisht të padeshifrueshëm",
                )
            })?;
            store.apply(pkg, stored_at);
        }
        Ok(store)
    }

    /// SHADOW VENDOS: pranon vetëm me gjurmë + përmbajtje (ZERO if/else mbi
    /// verdiktin), e RUAN në log (log-first), pastaj e aplikon duke ZHVENDOSUR
    /// përmbajtjen (Ligji 0). Dedupe me `project_id`: ringarkim → revisions++.
    pub fn store(&self, pkg: ApupkInbound) -> Result<u64, ApupkReject> {
        // ── Verdikt branchless i pranueshmërisë ────────────────────────────
        let has_trace = (pkg.trace.trace_id != 0) & !pkg.trace.initial_trace.is_empty();
        let has_content = !pkg.project_content.is_empty();
        let title_ok = !pkg.project_title.trim().is_empty();
        let seal_ok = crate::gcl_project_contract::seal_is_500(pkg.light_sovereign_flags);
        let trace_identity_ok = pkg.trace.trace_id == crate::gcl_project_contract::project_trace_id(
            pkg.trace.project_id,
            pkg.trace.user_id,
            &pkg.project_title,
        );
        let progress_ok = pkg.initial_progress.progress_percentage.is_finite();
        let ownership_ok = {
            let store = self.entries.read().unwrap_or_else(|p| p.into_inner());
            store.iter()
                .find(|entry| entry.project_id == pkg.trace.project_id)
                .map(|entry| entry.user_id == pkg.trace.user_id)
                .unwrap_or(true)
        };
        let admissible = has_trace & has_content & title_ok & seal_ok
            & trace_identity_ok & progress_ok & ownership_ok;

        // Guard i vetëm (dispatch veprimi, jo verdikt); arsyeja zgjidhet me tabelë.
        match admissible {
            false => {
                return Err(match (
                    has_trace,
                    has_content,
                    title_ok,
                    seal_ok,
                    trace_identity_ok,
                    progress_ok,
                    ownership_ok,
                ) {
                    (false, _, _, _, _, _, _) => ApupkReject::NoTrace,
                    (_, false, _, _, _, _, _) => ApupkReject::EmptyContent,
                    (_, _, false, _, _, _, _) => ApupkReject::InvalidProjectTitle,
                    (_, _, _, false, _, _, _) => ApupkReject::InvalidSovereignSeal,
                    (_, _, _, _, false, _, _) => ApupkReject::InvalidTraceIdentity,
                    (_, _, _, _, _, false, _) => ApupkReject::InvalidProgress,
                    (_, _, _, _, _, _, false) => ApupkReject::ProjectOwnershipMismatch,
                    _ => ApupkReject::EmptyContent,
                });
            }
            true => {}
        }

        let tid = pkg.trace.trace_id;
        let stored_at = now_ns();

        // ── 1) WAL log-first (best-effort); huazon pkg pa e zhvendosur ─────
        {
            let guard = self.log.read().unwrap_or_else(|p| p.into_inner());
            match guard.as_ref() {
                Some(log) => log.append(&encode_apupk(&pkg, stored_at)),
                None => eprintln!("[APUPK] ALARM: WAL i padisponueshëm gjatë store(tid={tid}) — persistencë e degraduar (RAM aktiv)"),
            }
        }

        // ── 2) Aplikim në RAM (ZHVENDOS pkg) ──────────────────────────────
        self.apply(pkg, stored_at);
        Ok(tid)
    }

    /// Regjistrim fail-closed për kufirin Light→Shadow→Quantum.
    /// Shadow lëshon ProjectContextWitness vetëm pasi rekordi është shkruar,
    /// flush-uar dhe fsync-uar. Pa WAL ose në dështim I/O, RAM nuk ndryshohet.
    pub fn store_durable(&self, pkg: ApupkInbound) -> Result<u64, ApupkReject> {
        let has_trace = (pkg.trace.trace_id != 0) & !pkg.trace.initial_trace.is_empty();
        let has_content = !pkg.project_content.is_empty();
        let title_ok = !pkg.project_title.trim().is_empty();
        let seal_ok = crate::gcl_project_contract::seal_is_500(pkg.light_sovereign_flags);
        let trace_identity_ok = pkg.trace.trace_id == crate::gcl_project_contract::project_trace_id(
            pkg.trace.project_id,
            pkg.trace.user_id,
            &pkg.project_title,
        );
        let progress_ok = pkg.initial_progress.progress_percentage.is_finite();
        let ownership_ok = {
            let store = self.entries.read().unwrap_or_else(|p| p.into_inner());
            store.iter()
                .find(|entry| entry.project_id == pkg.trace.project_id)
                .map(|entry| entry.user_id == pkg.trace.user_id)
                .unwrap_or(true)
        };
        let admissible = has_trace & has_content & title_ok & seal_ok
            & trace_identity_ok & progress_ok & ownership_ok;

        match admissible {
            false => {
                return Err(match (
                    has_trace,
                    has_content,
                    title_ok,
                    seal_ok,
                    trace_identity_ok,
                    progress_ok,
                    ownership_ok,
                ) {
                    (false, _, _, _, _, _, _) => ApupkReject::NoTrace,
                    (_, false, _, _, _, _, _) => ApupkReject::EmptyContent,
                    (_, _, false, _, _, _, _) => ApupkReject::InvalidProjectTitle,
                    (_, _, _, false, _, _, _) => ApupkReject::InvalidSovereignSeal,
                    (_, _, _, _, false, _, _) => ApupkReject::InvalidTraceIdentity,
                    (_, _, _, _, _, false, _) => ApupkReject::InvalidProgress,
                    (_, _, _, _, _, _, false) => ApupkReject::ProjectOwnershipMismatch,
                    _ => ApupkReject::EmptyContent,
                });
            }
            true => {}
        }

        let tid = pkg.trace.trace_id;
        let stored_at = now_ns();
        let payload = encode_apupk(&pkg, stored_at);
        {
            let guard = self.log.read().unwrap_or_else(|p| p.into_inner());
            let log = guard.as_ref().ok_or(ApupkReject::PersistenceUnavailable)?;
            log.append_checked(&payload)
                .map_err(|_| ApupkReject::PersistenceFailed)?;
        }
        self.apply(pkg, stored_at);
        Ok(tid)
    }

    /// Aplikim në RAM (pa rilogim) — i përdorur edhe nga ripërsëritja.
    /// `stored_at` jepet shtjellazi që ripërsëritja të jetë deterministike.
    fn apply(&self, pkg: ApupkInbound, stored_at: u64) {
        let mut store = self.entries.write().unwrap_or_else(|p| p.into_inner());

        // Dedupe me project_id (gjej indeksin pa zhvendosur ende pkg-në).
        let pos = store.iter().position(|e| e.project_id == pkg.trace.project_id);
        match pos {
            Some(i) => {
                // Rifreskim sovran — ZHVENDOS fushat (pa klon).
                let e = &mut store[i];
                e.trace_id            = pkg.trace.trace_id;
                e.initial_trace       = pkg.trace.initial_trace;            // MOVE
                e.project_title       = pkg.project_title;                  // MOVE
                e.light_sovereign_flags = pkg.light_sovereign_flags;
                e.content             = pkg.project_content;                // MOVE
                e.progress_percentage = pkg.initial_progress.progress_percentage.clamp(0.0, 100.0);
                e.last_update_ns      = pkg.initial_progress.last_update_ns;
                e.stored_at_ns        = stored_at;
                e.notes               = pkg.initial_progress.notes;         // MOVE
                e.revisions           = e.revisions.saturating_add(1);
            }
            None => {
                store.push(ApupkEntry {
                    trace_id:            pkg.trace.trace_id,
                    project_id:          pkg.trace.project_id,
                    user_id:             pkg.trace.user_id,
                    initial_trace:       pkg.trace.initial_trace,           // MOVE
                    project_title:       pkg.project_title,                  // MOVE
                    light_sovereign_flags: pkg.light_sovereign_flags,
                    content:             pkg.project_content,               // MOVE
                    progress_percentage: pkg.initial_progress.progress_percentage.clamp(0.0, 100.0),
                    last_update_ns:      pkg.initial_progress.last_update_ns,
                    stored_at_ns:        stored_at,
                    notes:               pkg.initial_progress.notes,        // MOVE
                    revisions:           0,
                });
            }
        }
    }

    /// Shadow përditëson progresin e një projekti (orientim → vendim sovran).
    /// `false` nëse projekti s'ekziston.
    pub fn update_progress(&self, project_id: u64, pct: f32, notes: String, ts_ns: u64) -> bool {
        let mut store = self.entries.write().unwrap_or_else(|p| p.into_inner());
        let pos = store.iter().position(|e| e.project_id == project_id);
        match pos {
            Some(i) => {
                let e = &mut store[i];
                e.progress_percentage = pct.clamp(0.0, 100.0);
                e.notes               = notes;        // MOVE
                e.last_update_ns      = ts_ns;
                true
            }
            None => false,
        }
    }

    /// Lexim: kopje e entry-t të një projekti (ose None).
    pub fn get(&self, project_id: u64) -> Option<ApupkEntry> {
        let store = self.entries.read().unwrap_or_else(|p| p.into_inner());
        store.iter().find(|e| e.project_id == project_id).cloned()
    }

    /// Numri i projekteve të ruajtura.
    pub fn count(&self) -> usize {
        self.entries.read().unwrap_or_else(|p| p.into_inner()).len()
    }

    /// Numri i projekteve të një përdoruesi.
    pub fn count_for_user(&self, user_id: u64) -> usize {
        self.entries.read().unwrap_or_else(|p| p.into_inner())
            .iter().filter(|e| e.user_id == user_id).count()
    }
}

// ── Kodimi i rekordit WAL (ngjarje store) ───────────────────────────────────

fn encode_apupk(pkg: &ApupkInbound, stored_at: u64) -> Vec<u8> {
    let mut w = RecWriter::new();
    w.u64(pkg.trace.trace_id)
        .u64(pkg.trace.project_id)
        .u64(pkg.trace.user_id)
        .str(&pkg.trace.initial_trace)
        .str(&pkg.project_title)
        .u32(pkg.light_sovereign_flags)
        .str(&pkg.project_content)
        .f32(pkg.initial_progress.progress_percentage)
        .u64(pkg.initial_progress.last_update_ns)
        .u64(stored_at)
        .str(&pkg.initial_progress.notes);
    w.finish()
}

fn decode_apupk(rec: &[u8]) -> Option<(ApupkInbound, u64)> {
    let mut r = RecReader::new(rec);
    let trace_id       = r.u64()?;
    let project_id     = r.u64()?;
    let user_id        = r.u64()?;
    let initial_trace  = r.string()?;
    let project_title  = r.string()?;
    let light_sovereign_flags = r.u32()?;
    let content        = r.string()?;
    let progress       = r.f32()?;
    let last_update_ns = r.u64()?;
    let stored_at      = r.u64()?;
    let notes          = r.string()?;
    let pkg = ApupkInbound {
        trace: ApupkTrace { trace_id, project_id, user_id, initial_trace, timestamp_ns: 0 },
        initial_progress: ProjectProgress { project_id, progress_percentage: progress, last_update_ns, notes },
        project_title,
        light_sovereign_flags,
        project_content: content,
    };
    Some((pkg, stored_at))
}

// ── Singleton global (një kujtesë APUPK sovrane për procesin) ───────────────
static APUPK: OnceLock<ShadowApupkMemory> = OnceLock::new();

pub fn apupk_memory() -> &'static ShadowApupkMemory {
    APUPK.get_or_init(ShadowApupkMemory::new)
}

/// Nis kujtesën globale APUPK me durabilitet WAL. Duhet thirrur PARA çdo
/// `apupk_memory()` (ndryshe singleton-i në-RAM është nisur tashmë).
pub fn init_apupk_disk(path: &Path) -> io::Result<()> {
    let store = ShadowApupkMemory::with_disk(path)?;
    APUPK
        .set(store)
        .map_err(|_| io::Error::new(io::ErrorKind::AlreadyExists, "apupk tashmë e nisur"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pkg(project_id: u64, user_id: u64, content: &str) -> ApupkInbound {
        let project_title = format!("project-{project_id}");
        ApupkInbound {
            trace: ApupkTrace {
                trace_id: crate::gcl_project_contract::project_trace_id(
                    project_id,
                    user_id,
                    &project_title,
                ),
                project_id,
                user_id,
                initial_trace: format!("APUPK-TRACE-{project_id}"),
                timestamp_ns: 1000,
            },
            initial_progress: ProjectProgress {
                project_id,
                progress_percentage: 0.0,
                last_update_ns: 1000,
                notes: "fillim".to_string(),
            },
            project_title,
            project_content: content.to_string(),
            light_sovereign_flags: 0xA451,
        }
    }

    #[test]
    fn stores_traceable_project() {
        let m = ShadowApupkMemory::new();
        let tid = m.store(pkg(10, 1, "kodi i projektit")).expect("stores_traceable_project: rezultat i papritur në test");
        assert_ne!(tid, 0);
        assert_eq!(m.count(), 1);
        assert_eq!(m.get(10).expect("stores_traceable_project: rezultat i papritur në test").content, "kodi i projektit");
    }

    #[test]
    fn reupload_same_project_refreshes_not_duplicates() {
        let m = ShadowApupkMemory::new();
        m.store(pkg(10, 1, "v1")).expect("reupload_same_project_refreshes_not_duplicates: rezultat i papritur në test");
        m.store(pkg(10, 1, "v2")).expect("reupload_same_project_refreshes_not_duplicates: rezultat i papritur në test");
        assert_eq!(m.count(), 1);               // dedupe me project_id
        let e = m.get(10).expect("reupload_same_project_refreshes_not_duplicates: rezultat i papritur në test");
        assert_eq!(e.content, "v2");
        assert_eq!(e.revisions, 1);
    }

    #[test]
    fn rejects_without_trace() {
        let m = ShadowApupkMemory::new();
        let mut p = pkg(10, 1, "x");
        p.trace.trace_id = 0;
        p.trace.initial_trace = String::new();
        assert_eq!(m.store(p), Err(ApupkReject::NoTrace));
        assert_eq!(m.count(), 0);
    }

    #[test]
    fn rejects_empty_content() {
        let m = ShadowApupkMemory::new();
        let p = pkg(10, 1, "");
        assert_eq!(m.store(p), Err(ApupkReject::EmptyContent));
        assert_eq!(m.count(), 0);
    }

    #[test]
    fn update_progress_moves_project_forward() {
        let m = ShadowApupkMemory::new();
        m.store(pkg(10, 1, "x")).expect("update_progress_moves_project_forward: rezultat i papritur në test");
        assert!(m.update_progress(10, 42.5, "gjysmë".to_string(), 2000));
        let e = m.get(10).expect("update_progress_moves_project_forward: rezultat i papritur në test");
        assert!((e.progress_percentage - 42.5).abs() < 1e-6);
        assert_eq!(e.notes, "gjysmë");
        assert!(!m.update_progress(999, 10.0, "s'ka".to_string(), 3000));
    }

    #[test]
    fn rejects_cross_user_revision_before_wal_or_ram_mutation() {
        let m = ShadowApupkMemory::new();
        m.store(pkg(10, 1, "owner-v1")).expect("owner store");
        let foreign = pkg(10, 2, "foreign-v2");
        assert_eq!(m.store(foreign), Err(ApupkReject::ProjectOwnershipMismatch));
        let entry = m.get(10).expect("owner remains");
        assert_eq!(entry.user_id, 1);
        assert_eq!(entry.content, "owner-v1");
        assert_eq!(entry.revisions, 0);
    }

    #[test]
    fn rejects_trace_not_derived_from_light_project_identity() {
        let m = ShadowApupkMemory::new();
        let mut p = pkg(10, 1, "v1");
        p.trace.trace_id ^= 1;
        assert_eq!(m.store(p), Err(ApupkReject::InvalidTraceIdentity));
        assert_eq!(m.count(), 0);
    }

    #[test]
    fn rejects_non_finite_project_progress() {
        let m = ShadowApupkMemory::new();
        let mut p = pkg(10, 1, "v1");
        p.initial_progress.progress_percentage = f32::NAN;
        assert_eq!(m.store(p), Err(ApupkReject::InvalidProgress));
        assert_eq!(m.count(), 0);
    }

    #[test]
    fn counts_per_user() {
        let m = ShadowApupkMemory::new();
        m.store(pkg(1, 7, "a")).expect("counts_per_user: rezultat i papritur në test");
        m.store(pkg(2, 7, "b")).expect("counts_per_user: rezultat i papritur në test");
        m.store(pkg(3, 9, "c")).expect("counts_per_user: rezultat i papritur në test");
        assert_eq!(m.count_for_user(7), 2);
        assert_eq!(m.count_for_user(9), 1);
    }

    fn temp(tag: &str) -> std::path::PathBuf {
        let mut p = std::env::temp_dir();
        let ns = SystemTime::now().duration_since(UNIX_EPOCH).expect("temp: rezultat i papritur në test").as_nanos();
        p.push(format!("apupk_{tag}_{ns}.log"));
        p
    }

    #[test]
    fn durable_store_rejects_ram_only_memory() {
        let m = ShadowApupkMemory::new();
        assert_eq!(
            m.store_durable(pkg(10, 1, "v1")),
            Err(ApupkReject::PersistenceUnavailable),
        );
        assert_eq!(m.count(), 0);
    }

    #[test]
    fn durable_store_survives_restart_before_witness() {
        let path = temp("durable");
        {
            let m = ShadowApupkMemory::with_disk(&path)
                .expect("durable_store_survives_restart_before_witness: open");
            m.store_durable(pkg(10, 1, "v1"))
                .expect("durable_store_survives_restart_before_witness: store");
            assert_eq!(m.count(), 1);
        }
        {
            let m = ShadowApupkMemory::with_disk(&path)
                .expect("durable_store_survives_restart_before_witness: reopen");
            assert_eq!(m.get(10).map(|e| e.content), Some("v1".to_string()));
        }
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn survives_restart_with_disk() {
        let path = temp("restart");
        {
            let m = ShadowApupkMemory::with_disk(&path).expect("survives_restart_with_disk: rezultat i papritur në test");
            m.store(pkg(10, 1, "v1")).expect("survives_restart_with_disk: rezultat i papritur në test");
            m.store(pkg(20, 1, "other")).expect("survives_restart_with_disk: rezultat i papritur në test");
            m.store(pkg(10, 1, "v2")).expect("survives_restart_with_disk: rezultat i papritur në test"); // rifreskim i të njëjtit projekt
            assert_eq!(m.count(), 2);
        }
        {
            // Ripërsëritja riprodhon gjendjen (dedupe + revisions).
            let m = ShadowApupkMemory::with_disk(&path).expect("survives_restart_with_disk: rezultat i papritur në test");
            assert_eq!(m.count(), 2);
            let e = m.get(10).expect("survives_restart_with_disk: rezultat i papritur në test");
            assert_eq!(e.content, "v2");
            assert_eq!(e.revisions, 1);
        }
        let _ = std::fs::remove_file(&path);
    }
}
