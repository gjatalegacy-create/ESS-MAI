// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  KNOWLEDGE_VAULT.RS — ESS-MAI SHADOW PLATFORM                    ║
// ║  Gjata Legacy™  |  Arkitekt: Bledar Gjata                             ║
// ║                                                                          ║
// ║  7 magazinat e dijes — ARKIVI SOVRAN.                                 ║
// ║                                                                          ║
// ║  RREGULLA ABSOLUTE (LAW_3 — pastërti arsyetimi):                     ║
// ║    LEXON:   çdo modul Shadow + Quantum                                 ║
// ║    SHKRUAN: shadow_gj_legacy.lgc VETËM (write API = pub(crate))       ║
// ║                                                                          ║
// ║  — I PASURUAR:                                                    ║
// ║    • Negative Knowledge me kontekst (shkak/proces/masë/frekuencë)     ║
// ║    • access_count gjurmohet për primitive + negative                  ║
// ║    • Maturimi Legacy ushqehet realisht (vector_points/temporal_score) ║
// ║    • check_known_negative() — Negative si KUFI aktiv                   ║
// ║    • VaultBackend — abstraksion për persistencë në disk (RAM tani)    ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::knowledge_lineage::cross_domain_legacy_ready;
use crate::types::{KnowledgeBand, NegativeContext, PrimitiveTrace, ShadowError, ShadowPath};
use std::sync::{Arc, RwLock};

// ════════════════════════════════════════════════════════════════════════════
// §0  VAULT BACKEND — abstraksion për persistencë (RAM tani, disk më vonë)
// ════════════════════════════════════════════════════════════════════════════

/// Cila magazinë po shkruhet (për backend-in e persistencës).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StoreKind {
    Scientific,
    Negative,
    Hypothesis,
    Fact,
    Primitive,
    Sovereign,
    Legacy,
}

/// Backend i persistencës (grepi i durabilitetit). Burimi i LEXIMIT mbetet
/// gjithmonë RAM (Vec); backend-i regjistron NGJARJET logjike që rikrijojnë
/// gjendjen në startup. Metodat kanë trup bosh si parazgjedhje → backend-et
/// jo-persistente (RAM) s'implementojnë asgjë.
///
/// Regjistrohen VETËM ngjarjet thelbësore të dijes:
///   `on_primitive` / `on_verified` / `on_negative` / `on_confirm`.
/// `access_count` (popullariteti runtime) është metadata e butë dhe NUK
/// persiston veçmas — rindërtohet nga ripërsëritja e `Confirm`.
///
/// LIGJI 0: të gjitha metodat marrin payload-in me referencë (`&[u8]`) —
/// asnjë klon; serializimi në disk është kufiri i vetëm i durabilitetit.
pub trait VaultBackend: std::fmt::Debug + Send + Sync {
    #[allow(clippy::too_many_arguments)]
    fn on_primitive(&self, _input_id: u64, _vector_points: f32, _temporal_score: f32, _mass: f32, _seal: u32, _path: u8, _domain: &str, _bytes: &[u8], _ts: u64) {}
    fn on_verified(&self, _input_id: u64, _band: u8, _mass: f32, _bytes: &[u8], _ts: u64) {}
    fn on_negative(&self, _input_id: u64, _reason: u32, _process: &str, _mass: f32, _frequency: u32, _bytes: &[u8], _ts: u64) {}
    fn on_confirm(&self, _input_id: u64, _hits: u32) {}
}

/// Backend në RAM — pa durabilitet (default i; përdoret nga Shadow::new).
#[derive(Debug, Default)]
pub struct InMemoryBackend;

impl VaultBackend for InMemoryBackend {}

// ════════════════════════════════════════════════════════════════════════════
// §1  ENTRY TYPES PER STORE
// ════════════════════════════════════════════════════════════════════════════

/// Tolerancë e masës për të njohur të njëjtin model negativ.
const NEG_MASS_TOLERANCE: f32 = 0.05;

/// FNV-1a 64-bit — identifikues përmbajtjeje deterministik (content_hash).
fn fnv64(bytes: &[u8]) -> u64 {
    let mut h: u64 = 14695981039346656037;
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(1099511628211);
    }
    h
}

#[derive(Debug, Clone)]
pub struct ScientificEntry {
    pub domain:     String,
    pub content:    Vec<u8>,
    pub source:     String,
    pub added_at:   u64,
}

#[derive(Debug, Clone)]
pub struct NegativeEntry {
    pub input_id:            u64,
    /// Kodi i arsyes së dështimit — bitmask (qeverisje | vula | etj.)
    pub failure_reason_code: u32,
    /// Procesi / territori ku ndodhi
    pub process:             String,
    /// Epistemic mass në çastin e dështimit
    pub mass_at_fail:        f32,
    /// Sa herë u përsërit ky model negativ
    pub frequency:           u32,
    pub raw_bytes:           Vec<u8>,
    /// Sa herë u konsultua si kufi për procese të reja
    pub access_count:        u32,
    pub added_at:            u64,
}

impl NegativeEntry {
    pub fn context(&self) -> NegativeContext {
        NegativeContext {
            failure_reason_code: self.failure_reason_code,
            process:             self.process.clone(),
            mass_at_fail:        self.mass_at_fail,
            frequency:           self.frequency,
        }
    }

    /// to_bytes — serializon entry-n si bytes (P3: eksport sinkron për PRO Vault).
    /// Format: input_id(8) | reason(4) | mass(4) | freq(4) | access(4) | added(8)
    ///         | proc_len(4) | proc_bytes | raw_len(4) | raw_bytes. Zero if.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&self.input_id.to_le_bytes());
        out.extend_from_slice(&self.failure_reason_code.to_le_bytes());
        out.extend_from_slice(&self.mass_at_fail.to_le_bytes());
        out.extend_from_slice(&self.frequency.to_le_bytes());
        out.extend_from_slice(&self.access_count.to_le_bytes());
        out.extend_from_slice(&self.added_at.to_le_bytes());
        let proc_bytes = self.process.as_bytes();
        out.extend_from_slice(&(proc_bytes.len() as u32).to_le_bytes());
        out.extend_from_slice(proc_bytes);
        out.extend_from_slice(&(self.raw_bytes.len() as u32).to_le_bytes());
        out.extend_from_slice(&self.raw_bytes);
        out
    }

    /// from_bytes — deserializon entry-n nga bytes. Zero if — match mbi gjatësi.
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        // Minimumi: 8+4+4+4+4+8+4 = 36 byte header. Zero if — match.
        let enough_header = bytes.len() >= 36;
        match enough_header {
            false => None,
            true => {
                let input_id = u64::from_le_bytes(bytes[0..8].try_into().ok()?);
                let failure_reason_code = u32::from_le_bytes(bytes[8..12].try_into().ok()?);
                let mass_at_fail = f32::from_le_bytes(bytes[12..16].try_into().ok()?);
                let frequency = u32::from_le_bytes(bytes[16..20].try_into().ok()?);
                let access_count = u32::from_le_bytes(bytes[20..24].try_into().ok()?);
                let added_at = u64::from_le_bytes(bytes[24..32].try_into().ok()?);
                let proc_len = u32::from_le_bytes(bytes[32..36].try_into().ok()?) as usize;
                // Lexo process (fail-closed: truncation = korrupsion → refuzo).
                let proc_end = 36 + proc_len;
                let process = match bytes.len() >= proc_end {
                    true  => String::from_utf8_lossy(&bytes[36..proc_end]).to_string(),
                    false => {
                        eprintln!("[VAULT_DECODE] ALARM: rekord i cunguar (process: pritej {proc_end}B, ka {}B) — REFUZUAR (korrupsion)", bytes.len());
                        return None;
                    }
                };
                // Lexo raw_bytes (fail-closed: truncation = korrupsion → refuzo).
                let raw_bytes = match bytes.len() >= proc_end + 4 {
                    true => {
                        let raw_len = u32::from_le_bytes(
                            bytes[proc_end..proc_end+4].try_into().ok()?
                        ) as usize;
                        let raw_start = proc_end + 4;
                        let raw_end = raw_start + raw_len;
                        match bytes.len() >= raw_end {
                            true  => bytes[raw_start..raw_end].to_vec(),
                            false => {
                                eprintln!("[VAULT_DECODE] ALARM: rekord i cunguar (raw: pritej {raw_end}B, ka {}B) — REFUZUAR (korrupsion)", bytes.len());
                                return None;
                            }
                        }
                    }
                    false => {
                        eprintln!("[VAULT_DECODE] ALARM: rekord i cunguar (raw_len header mungon) — REFUZUAR (korrupsion)");
                        return None;
                    }
                };
                Some(NegativeEntry {
                    input_id, failure_reason_code, process, mass_at_fail,
                    frequency, raw_bytes, access_count, added_at,
                })
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct HypothesisEntry {
    pub input_id:     u64,
    pub mass:         f32,
    pub raw_bytes:    Vec<u8>,
    pub added_at:     u64,
}

#[derive(Debug, Clone)]
pub struct FactEntry {
    pub input_id:     u64,
    pub mass:         f32,
    pub raw_bytes:    Vec<u8>,
    pub added_at:     u64,
}

#[derive(Debug, Clone)]
pub struct PrimitiveEntry {
    pub input_id:       u64,
    pub content_hash:   u64,     /* identifikon inputin (FNV i raw_bytes)  */
    pub raw_bytes:      Vec<u8>,
    pub vector_points:  f32,     /* pikë të akumuluara drejt Legacy       */
    pub real_hits:      u32,     /* sa herë u konfirmua nga bota reale    */
    pub temporal_score: f32,     /* qëndrueshmëria kohore                 */
    pub access_count:   u32,     /* sa herë u konsultua si bazë vendimi   */
    pub added_at:       u64,
    /// GJURMUESHMËRIA: historiku i përforcimit ndër-domain (ku/si/kur/masë/vulë).
    pub primitive_chain: Vec<PrimitiveTrace>,
    /// I gatshëm për Legacy kur primitive_chain ka ≥5 domain-e unike e konsistente.
    pub legacy_ready:    bool,
}

impl PrimitiveEntry {
    /// Skor cilësie i kristalizimit (zero if/else) — ruhet te LegacyEntry kur
    /// primitivi promovohet. Promovimi VENDOSET nga rregulla ndër-domain (≥5),
    /// jo nga ky skor; ky skor është thjesht metrikë e cilësisë së dijes.
    pub fn legacy_score(&self) -> f32 {
        (self.vector_points      * 0.40
       + self.real_hits as f32   * 0.35 / 100.0
       + self.temporal_score     * 0.25)
        .clamp(0.0, 1.0)
    }

    /// LIGJI Primitive→Legacy (i PËRFORCUAR, FAZA 3): gatishmëria kërkon
    /// BOTH përhapje ndër-domain (`legacy_ready`) DHE maturim real nga bota
    /// (real_hits + temporal_score + access_count mbi prag).
    ///
    /// Një legacy lind me KOHË: jo mjafton të prekë 5 domaine — duhet të ketë
    /// "mbijetuar" përdorimin real. `legacy_ready` mbahet nga `apply_primitive`;
    /// maturimi vlerësohet nga MaturationGate mbi sinjalet e këtij entry.
    /// Zero if/else — produkt boolean.
    pub fn ready_for_legacy(&self) -> bool {
        let signals = crate::legacy_maturation::MaturationSignals {
            real_hits:      self.real_hits,
            temporal_score: self.temporal_score,
            access_count:   self.access_count,
            domain_count:   crate::knowledge_lineage::unique_domains(&self.primitive_chain),
            chain_strength: crate::primitive_chain::ChainStrength::compute(&self.primitive_chain).strength,
        };
        // BOTH: rregulla ndër-domain (legacy_ready) AND maturim real.
        crate::legacy_maturation::MaturationGate::legacy_qualified(&signals, self.legacy_ready)
    }

    /// legacy_reason — prejardhja e plotë: PSE/SI ky primitiv (do të) bëhet
    /// legacy. Hook për vëzhguesin e primitivëve (plani #7) — çdo legacy ka
    /// arsyen e regjistruar se si erdhi.
    pub fn legacy_reason(&self) -> crate::legacy_maturation::LegacyReason {
        let signals = crate::legacy_maturation::MaturationSignals {
            real_hits:      self.real_hits,
            temporal_score: self.temporal_score,
            access_count:   self.access_count,
            domain_count:   crate::knowledge_lineage::unique_domains(&self.primitive_chain),
            chain_strength: crate::primitive_chain::ChainStrength::compute(&self.primitive_chain).strength,
        };
        crate::legacy_maturation::LegacyReason::derive(&signals, self.legacy_ready)
    }
}

#[derive(Debug, Clone)]
pub struct SovereignEntry {
    pub law_id:      u8,         /* 1-5                                   */
    pub decision:    u8,         /* 0/1                                   */
    pub input_id:    u64,
    pub raw_bytes:   Vec<u8>,
    pub added_at:    u64,
}

#[derive(Debug, Clone)]
pub struct LegacyEntry {
    pub origin_primitive_id: u64,
    pub raw_bytes:           Vec<u8>,
    pub crystallized_at:     u64,
    pub legacy_score:        f32,
    /// I paprekshëm — Shadow nuk e modifikon kurrë (LAW_4)
    pub immutable_seal:      u32,
}

// ════════════════════════════════════════════════════════════════════════════
// §2  KNOWLEDGE VAULT — 7 stores + backend
// ════════════════════════════════════════════════════════════════════════════

#[derive(Debug)]
pub struct KnowledgeVault {
    pub scientific: Arc<RwLock<Vec<ScientificEntry>>>,
    pub negative:   Arc<RwLock<Vec<NegativeEntry>>>,
    pub hypothesis: Arc<RwLock<Vec<HypothesisEntry>>>,
    pub fact:       Arc<RwLock<Vec<FactEntry>>>,
    pub primitive:  Arc<RwLock<Vec<PrimitiveEntry>>>,
    pub sovereign:  Arc<RwLock<Vec<SovereignEntry>>>,
    pub legacy:     Arc<RwLock<Vec<LegacyEntry>>>,
    /// FAZA 7 — vëzhguesi i përhershëm: regjistron prejardhjen e çdo legacy.
    pub observer:   Arc<crate::legacy_observer::LegacyObserver>,
    backend:        Box<dyn VaultBackend>,
}

impl Default for KnowledgeVault {
    fn default() -> Self {
        Self::with_backend(Box::new(InMemoryBackend))
    }
}

impl KnowledgeVault {
    pub fn new() -> Self {
        Self::default()
    }

    /// Krijon vault me një backend persistence të zgjedhur (RAM ose disk).
    pub fn with_backend(backend: Box<dyn VaultBackend>) -> Self {
        Self {
            scientific: Arc::new(RwLock::new(Vec::new())),
            negative:   Arc::new(RwLock::new(Vec::new())),
            hypothesis: Arc::new(RwLock::new(Vec::new())),
            fact:       Arc::new(RwLock::new(Vec::new())),
            primitive:  Arc::new(RwLock::new(Vec::new())),
            sovereign:  Arc::new(RwLock::new(Vec::new())),
            legacy:     Arc::new(RwLock::new(Vec::new())),
            observer:   Arc::new(crate::legacy_observer::LegacyObserver::new()),
            backend,
        }
    }

    /// FAZA 2 — Hap vault me DURABILITET në disk; ripërsërit log-un ekzistues.
    /// Pas restart-it, gjendja sovrane rikrijohet EKZAKT nga WAL-i.
    pub fn open_disk(path: &std::path::Path) -> Result<Self, ShadowError> {
        let (backend, ops) = crate::vault_disk::DiskBackend::open(path)
            .map_err(|e| ShadowError::VaultWriteError(format!("disk open: {e}")))?;
        let vault = Self::with_backend(Box::new(backend));
        vault.replay(ops);
        Ok(vault)
    }

    /// Ripërsërit ngjarjet e regjistruara përmes TË NJËJTËS logjikë aplikimi
    /// (pa rilogim). Dedup-i negativ dhe promovimi Legacy riprodhohen besnikërisht.
    fn replay(&self, ops: Vec<crate::vault_disk::ReplayOp>) {
        use crate::vault_disk::ReplayOp;
        for op in ops {
            match op {
                ReplayOp::Primitive { input_id, vector_points, temporal_score, mass, seal, path, ts, domain, bytes } => {
                    let trace = PrimitiveTrace {
                        domain, timestamp: ts, mass,
                        path_taken: ShadowPath::from_u8(path), lgc_seal: seal,
                    };
                    // FAIL-LOUD (ULTIMATUM): dështim semantik gjatë replay = gjendje
                    // e persistuar problematike → raportohet, s'kalon në heshtje.
                    match self.apply_primitive(input_id, bytes, vector_points, temporal_score, trace) {
                        Ok(())  => {}
                        Err(e)  => eprintln!("[VAULT_REPLAY] ALARM: apply_primitive(id={input_id}) dështoi ({e:?}) — WAL i dyshimtë"),
                    }
                    // Riprodho promovimin ndër-domain (5-të domain → Legacy). Count, jo Result.
                    let _ = self.try_promote_to_legacy();
                }
                ReplayOp::Verified { input_id, band, mass, ts, bytes } => {
                    let kb = match band {
                        1 => KnowledgeBand::Hypothesis,
                        2 => KnowledgeBand::Fact,
                        3 => KnowledgeBand::Scientific,
                        _ => KnowledgeBand::Negative,
                    };
                    match self.apply_verified(input_id, bytes, kb, mass, ts) {
                        Ok(())  => {}
                        Err(e)  => eprintln!("[VAULT_REPLAY] ALARM: apply_verified(id={input_id}) dështoi ({e:?}) — WAL i dyshimtë"),
                    }
                }
                ReplayOp::Negative { input_id, reason, process, mass, frequency, ts, bytes } => {
                    match self.apply_negative(input_id, reason, process, mass, frequency, bytes, ts) {
                        Ok(())  => {}
                        Err(e)  => eprintln!("[VAULT_REPLAY] ALARM: apply_negative(id={input_id}) dështoi ({e:?}) — WAL i dyshimtë"),
                    }
                }
                ReplayOp::Confirm { input_id, hits } => {
                    let _ = self.apply_confirm(input_id, hits); // count, jo Result
                }
            };
        }
    }

    // ── READ API — çdo modul mund të lexojë ─────────────────────────────

    pub fn query_scientific_similarity(&self, candidate_scores: &[f32]) -> f32 {
        let store = self.scientific.read().unwrap_or_else(|p| p.into_inner());
        match store.is_empty() { true => return 0.0, false => {} }
        let sum: f32 = candidate_scores.iter().copied().sum();
        (sum / candidate_scores.len().max(1) as f32).clamp(0.0, 1.0)
    }

    pub fn check_sovereign_conflict(&self, reasoning_strength: f32) -> f32 {
        // PJESA C: store-i SOVRAN (vendimet supreme) — nëse lock-u helmohet, njofto
        // PARA rikuperimit. Rikuperimi mbetet fail-safe, por gjendja duhet audituar.
        let store = self.sovereign.read().unwrap_or_else(|p| {
            eprintln!("[SOVEREIGN] ANOMALY: sovereign store lock poisoned — recovering, vendimet sovrane duhet audituar");
            p.into_inner()
        });
        match store.is_empty() { true => return 0.0, false => {} }
        // Vendimi sovran është binar (0/1): numëro refuzimet si sasi diskrete,
        // pastaj normalizo vetëm në kufirin e score-it. Kjo shmang aritmetikën
        // float mbi një counter dhe nuk lejon vlera negative për decision jo-kanonik.
        let rejected: usize = store.iter()
            .filter(|e| e.decision == 0)
            .count();
        let conflict = rejected as f32 / store.len() as f32;
        (conflict * (1.0 - reasoning_strength * 0.5)).clamp(0.0, 1.0)
    }

    pub fn query_negative_pattern(&self, epistemic_mass: f32) -> f32 {
        let store = self.negative.read().unwrap_or_else(|p| p.into_inner());
        match store.is_empty() { true => return 0.0, false => {} }
        // Dendësia e modeleve negative pranë kësaj mase — read-only.
        let similar: f32 = store.iter()
            .map(|e| {
                let close = (NEG_MASS_TOLERANCE - (e.mass_at_fail - epistemic_mass).abs())
                    .max(0.0) / NEG_MASS_TOLERANCE;
                close * (e.frequency as f32).min(10.0) / 10.0
            })
            .sum::<f32>()
            / store.len() as f32;
        similar.clamp(0.0, 1.0)
    }

    /// NEGATIVE SI KUFI AKTIV — a është kjo rrugë tashmë e njohur si negative?
    /// Kthen kontekstin (shkak/proces/masë/frekuencë) dhe rrit access_count.
    pub fn check_known_negative(&self, epistemic_mass: f32, territory: &str) -> Option<NegativeContext> {
        let mut store = self.negative.write().unwrap_or_else(|p| p.into_inner());
        for e in store.iter_mut() {
            let same_proc  = e.process == territory;
            let close_mass = (e.mass_at_fail - epistemic_mass).abs() <= NEG_MASS_TOLERANCE;
            match same_proc && close_mass {
                true => {
                    e.access_count = e.access_count.saturating_add(1);
                    return Some(e.context());
                }
                false => {}
            }
        }
        None
    }

    pub fn primitive_count(&self)  -> usize { self.primitive.read().unwrap_or_else(|p| p.into_inner()).len() }

    /// GCL LIVE (vetëm lexim): real_hits e një ankore PA sipas input_id —
    /// 0 nëse primitivi s'ekziston. Ushqen auditimin e lakut Y→X në runtime.
    pub fn primitive_real_hits(&self, input_id: u64) -> u32 {
        self.primitive
            .read()
            .unwrap_or_else(|p| p.into_inner())
            .iter()
            .filter(|e| e.input_id == input_id)
            .map(|e| e.real_hits)
            .next()
            .unwrap_or(0)
    }
    pub fn legacy_count(&self)     -> usize { self.legacy.read().unwrap_or_else(|p| p.into_inner()).len() }
    pub fn negative_count(&self)   -> usize { self.negative.read().unwrap_or_else(|p| p.into_inner()).len() }
    pub fn hypothesis_count(&self) -> usize { self.hypothesis.read().unwrap_or_else(|p| p.into_inner()).len() }

    /// export_negative — kthen klon të të gjitha NegativeEntry-ve (P3: për PRO Vault).
    /// Vetëm lexim — Shadow mban autoritetin; Quantum merr kopje sinkronizimi. Zero if.
    pub fn export_negative(&self) -> Vec<NegativeEntry> {
        self.negative.read().unwrap_or_else(|p| p.into_inner()).clone()
    }
    pub fn fact_count(&self)       -> usize { self.fact.read().unwrap_or_else(|p| p.into_inner()).len() }

    // ── WRITE API — VETËM shadow_gj_legacy.lgc (pub(crate)) ─────────────

    /// Shkruan në PrimitiveStore — 500 seal i konfirmuar.
    /// Çdo shkrim regjistron një GJURMË (domain/kohë/masë/path/vulë). Përsëritja
    /// e të njëjtës përmbajtje në domain të ndryshëm e afron drejt Legacy.
    /// FAZA 2: WAL-first — regjistron ngjarjen (borrow), pastaj apliko (move).
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn write_primitive(
        &self,
        input_id:       u64,
        vector_points:  f32,
        temporal_score: f32,
        mass:           f32,
        seal:           u32,
        path:           ShadowPath,
        domain:         String,
        raw_bytes:      Vec<u8>,
        ts_ns:          u64,
    ) -> Result<(), ShadowError> {
        self.backend.on_primitive(
            input_id, vector_points, temporal_score, mass, seal,
            path.as_u8(), &domain, &raw_bytes, ts_ns,
        );
        let trace = PrimitiveTrace { domain, timestamp: ts_ns, mass, path_taken: path, lgc_seal: seal };
        self.apply_primitive(input_id, raw_bytes, vector_points, temporal_score, trace)
    }

    /// Variant raw (callback C): gjurmë me domain "FFI", path Fast, vulë 0xA451.
    pub(crate) fn write_primitive_raw(
        &self,
        input_id:  u64,
        raw_bytes: Vec<u8>,
        ts_ns:     u64,
    ) -> Result<(), ShadowError> {
        let domain = String::from("FFI");
        self.backend.on_primitive(
            input_id, 0.0, 0.5, 0.0, 0xA451,
            ShadowPath::Fast.as_u8(), &domain, &raw_bytes, ts_ns,
        );
        let trace = PrimitiveTrace {
            domain, timestamp: ts_ns, mass: 0.0,
            path_taken: ShadowPath::Fast, lgc_seal: 0xA451,
        };
        self.apply_primitive(input_id, raw_bytes, 0.0, 0.5, trace)
    }

    /// Aplikim në RAM (pa rilogim) — i përdorur edhe nga ripërsëritja.
    /// Dedupe ndër-domain me `content_hash`: i njëjti input → shton gjurmë;
    /// i ri → entry e re. `legacy_ready` rillogaritet sa herë (ZERO if/else mbi vulën).
    fn apply_primitive(
        &self,
        input_id:       u64,
        raw_bytes:      Vec<u8>,
        vector_points:  f32,
        temporal_score: f32,
        trace:          PrimitiveTrace,
    ) -> Result<(), ShadowError> {
        let content_hash = fnv64(&raw_bytes);
        let ts = trace.timestamp;

        let mut store = self.primitive.write()
            .map_err(|e| ShadowError::VaultWriteError(e.to_string()))?;

        // Përforcim: i njëjti content_hash → shto gjurmën (jo entry e re).
        for e in store.iter_mut() {
            match e.content_hash == content_hash {
                true => {
                e.primitive_chain.push(trace);
                e.legacy_ready  = cross_domain_legacy_ready(&e.primitive_chain);
                e.access_count  = e.access_count.saturating_add(1);
                return Ok(()); // raw_bytes hidhet — përmbajtja ekziston tashmë
            }
                false => {}
            }
        }

        // Shfaqje e parë → entry e re me gjurmën e parë.
        let chain = vec![trace];
        let legacy_ready = cross_domain_legacy_ready(&chain); // 1 domain < 5 → false
        store.push(PrimitiveEntry {
            input_id,
            content_hash,
            raw_bytes,
            vector_points,
            real_hits:       0,
            temporal_score,
            access_count:    0,
            added_at:        ts,
            primitive_chain: chain,
            legacy_ready,
        });
        Ok(())
    }

    /// Shkruan sipas knowledge_band — verified por jo primitive.
    /// FAZA 2: regjistron NJË ngjarje `on_verified`; ripërsëritja riprodhon
    /// të njëjtin rrugëzim (përfshirë rastin Negative).
    pub(crate) fn write_verified(
        &self,
        input_id:  u64,
        raw_bytes: Vec<u8>,
        band:      KnowledgeBand,
        mass:      f32,
        ts_ns:     u64,
    ) -> Result<(), ShadowError> {
        self.backend.on_verified(input_id, band.as_u8(), mass, &raw_bytes, ts_ns);
        self.apply_verified(input_id, raw_bytes, band, mass, ts_ns)
    }

    fn apply_verified(
        &self,
        input_id:  u64,
        raw_bytes: Vec<u8>,
        band:      KnowledgeBand,
        mass:      f32,
        ts_ns:     u64,
    ) -> Result<(), ShadowError> {
        match band {
            KnowledgeBand::Hypothesis => {
                let mut s = self.hypothesis.write()
                    .map_err(|e| ShadowError::VaultWriteError(e.to_string()))?;
                s.push(HypothesisEntry { input_id, mass, raw_bytes, added_at: ts_ns });
            }
            KnowledgeBand::Fact | KnowledgeBand::Scientific => {
                let mut s = self.fact.write()
                    .map_err(|e| ShadowError::VaultWriteError(e.to_string()))?;
                s.push(FactEntry { input_id, mass, raw_bytes, added_at: ts_ns });
            }
            KnowledgeBand::Negative => {
                // verified me bandë negative → trajto si negative pa kontekst të pasur.
                self.apply_negative(input_id, 0, String::new(), mass, 1, raw_bytes, ts_ns)?;
            }
        }
        Ok(())
    }

    /// Shkruan në NegativeStore me KONTEKST të plotë (LAW_5).
    /// FAZA 2: regjistron INPUT-in; dedup-i (frekuenca) riprodhohet besnikërisht
    /// nga ripërsëritja e të njëjtave ngjarje në renditje.
    pub(crate) fn write_negative(
        &self,
        input_id:  u64,
        raw_bytes: Vec<u8>,
        ctx:       NegativeContext,
        ts_ns:     u64,
    ) -> Result<(), ShadowError> {
        self.backend.on_negative(
            input_id, ctx.failure_reason_code, &ctx.process,
            ctx.mass_at_fail, ctx.frequency, &raw_bytes, ts_ns,
        );
        self.apply_negative(
            input_id, ctx.failure_reason_code, ctx.process,
            ctx.mass_at_fail, ctx.frequency, raw_bytes, ts_ns,
        )
    }

    /// Aplikim në RAM me dedupe (pa rilogim) — i përdorur edhe nga ripërsëritja.
    /// Nëse modeli ekziston (arsye + proces + masë e ngjashme) → frekuencë++.
    #[allow(clippy::too_many_arguments)]
    pub fn apply_negative(
        &self,
        input_id:  u64,
        reason:    u32,
        process:   String,
        mass:      f32,
        frequency: u32,
        raw_bytes: Vec<u8>,
        ts_ns:     u64,
    ) -> Result<(), ShadowError> {
        let mut store = self.negative.write()
            .map_err(|e| ShadowError::VaultWriteError(e.to_string()))?;

        for e in store.iter_mut() {
            let same_reason = e.failure_reason_code == reason;
            let same_proc   = e.process == process;
            let close_mass  = (e.mass_at_fail - mass).abs() <= NEG_MASS_TOLERANCE;
            match same_reason && same_proc && close_mass {
                true => {
                e.frequency    = e.frequency.saturating_add(1);
                e.access_count = e.access_count.saturating_add(1);
                e.added_at     = ts_ns;
                return Ok(());
            }
                false => {}
            }
        }

        store.push(NegativeEntry {
            input_id,
            failure_reason_code: reason,
            process,
            mass_at_fail:        mass,
            frequency:           frequency.max(1),
            raw_bytes,
            access_count:        0,
            added_at:            ts_ns,
        });
        Ok(())
    }

    // ── MATURIMI — Primitive → Legacy (mbijetesa e zgjatur) ─────────────

    /// Konfirmim nga bota reale: rrit real_hits dhe access_count, pastaj
    /// provon promovimin në Legacy. Kjo zbaton "mbijetesën e zgjatur".
    /// Konfirmim nga bota reale: rrit real_hits dhe access_count, pastaj
    /// provon promovimin në Legacy. Kjo zbaton "mbijetesën e zgjatur".
    /// FAZA 2: regjistron `on_confirm`; ripërsëritja riprodhon EKZAKT
    /// rritjen e real_hits dhe promovimin (kaskadë RAM, pa rilogim).
    pub fn confirm_primitive(&self, input_id: u64, hits: u32) -> u32 {
        self.backend.on_confirm(input_id, hits);
        self.apply_confirm(input_id, hits)
    }

    fn apply_confirm(&self, input_id: u64, hits: u32) -> u32 {
        {
            let mut store = self.primitive.write().unwrap_or_else(|p| p.into_inner());
            for e in store.iter_mut() {
                match e.input_id == input_id {
                    true => {
                        e.real_hits    = e.real_hits.saturating_add(hits);
                        e.access_count = e.access_count.saturating_add(1);
                    }
                    false => {}
                }
            }
        }
        self.try_promote_to_legacy()
    }

    /// Rrit access_count për një primitiv kur konsultohet si bazë vendimi.
    pub fn note_primitive_access(&self, input_id: u64) {
        let mut store = self.primitive.write().unwrap_or_else(|p| p.into_inner());
        for e in store.iter_mut() {
            match e.input_id == input_id {
                true => e.access_count = e.access_count.saturating_add(1),
                false => {}
            }
        }
    }

    /// Promovon Primitive → Legacy kur përmbushet rregulla NDËR-DOMAIN
    /// (≥5 domain-e unike me vulë konsistente). Hook publik për maturim manual.
    pub fn run_maturation(&self) -> u32 {
        self.try_promote_to_legacy()
    }

    pub(crate) fn try_promote_to_legacy(&self) -> u32 {
        let mut promoted = 0u32;

        // Identifiko kandidatët e gatshëm (lexim).
        let ready_ids: Vec<u64> = {
            let store = self.primitive.read().unwrap_or_else(|p| p.into_inner());
            store.iter()
                .filter(|e| e.ready_for_legacy())
                .map(|e| e.input_id)
                .collect()
        };

        for id in ready_ids {
            // Hiq nga primitive (kristalizohet — nuk qëndron i dyfishtë).
            let entry = {
                let mut store = self.primitive.write().unwrap_or_else(|p| p.into_inner());
                match store.iter().position(|e| e.input_id == id) {
                    Some(pos) => Some(store.remove(pos)),
                    None => None,
                }
            };

            match entry {
                Some(c) => {
                let immutable = (c.input_id as u32).wrapping_mul(0xA4B1);
                // FAZA 7 — kap prejardhjen PARA zhvendosjes së raw_bytes.
                let birth_id     = c.input_id;
                let birth_at     = c.added_at;
                let birth_score  = c.legacy_score();
                let birth_reason = c.legacy_reason();
                let mut legacy = self.legacy.write().unwrap_or_else(|p| p.into_inner());
                legacy.push(LegacyEntry {
                    origin_primitive_id: c.input_id,
                    raw_bytes:           c.raw_bytes,
                    crystallized_at:     c.added_at,
                    legacy_score:        birth_score,
                    immutable_seal:      immutable,
                });
                promoted = promoted.wrapping_add(1);
                // Regjistro lindjen me arsyen e plotë (vëzhguesi i përhershëm).
                self.observer.observe_birth(birth_id, birth_at, birth_score, birth_reason);
                }
                None => {}
            }
        }
        promoted
    }
}

// ════════════════════════════════════════════════════════════════════════════
// §3  CALLBACKS nga C kernel → Rust (vetëm ABI; kerneli dublikatë jashtë build)
//     shadow_gj_legacy_kernel.c NUK është në build.rs; këto simbole mbeten
//     për pajtueshmëri ABI. Shkrimi real bëhet nga shadow_gj_legacy.rs.
// ════════════════════════════════════════════════════════════════════════════

use std::sync::OnceLock;

static GLOBAL_VAULT: OnceLock<Arc<KnowledgeVault>> = OnceLock::new();

pub fn init_global_vault(vault: Arc<KnowledgeVault>) {
    // FAIL-LOUD: set() dështon vetëm nëse vault-i global është VENDOSUR TASHMË
    // (dyfishim inicializimi = anomali). Raportohet, s'fshihet me `let _`.
    match GLOBAL_VAULT.set(vault) {
        Ok(())  => {}
        Err(_)  => eprintln!("[GLOBAL_VAULT] ALARM: init i dyfishtë — vault-i global u vendos tashmë (injorohet i dyti)"),
    }
}

pub fn get_global_vault() -> Option<Arc<KnowledgeVault>> {
    GLOBAL_VAULT.get().cloned()
}

/// Callback ABI nga kerneli C për shkrim primitive.
///
/// # Safety
/// Kur `pkg_len > 0`, `pkg_bytes` duhet të tregojë te të paktën `pkg_len` byte
/// të lexueshëm dhe të mbetet valid gjatë thirrjes. ABI/simboli C nuk ndryshon;
/// `unsafe` formalizon kontratën që C duhet të garantojë.
#[no_mangle]
pub unsafe extern "C" fn vault_write_primitive(
    pkg_bytes: *const u8,
    pkg_len:   u32,
    ts_ns:     u64,
) {
    match pkg_bytes.is_null() || pkg_len == 0 { true => return, false => {} }
    // SAFETY: kontrata e funksionit kërkon pointer të lexueshëm për `pkg_len` byte.
    let bytes = unsafe { std::slice::from_raw_parts(pkg_bytes, pkg_len as usize) }.to_vec();
    match get_global_vault() {
        Some(vault) => match vault.write_primitive_raw(ts_ns, bytes, ts_ns) {
            Ok(_)  => {}
            Err(e) => eprintln!("[VAULT_FFI] ALARM: write_primitive dështoi ({e:?}) — shkrim i humbur"),
        },
        None => eprintln!("[VAULT_FFI] ALARM: vault global s'është gati — write_primitive HUMBI (shkrim i humbur)"),
    }
}

/// Callback ABI nga kerneli C për dije të verifikuar.
///
/// # Safety
/// Kur `pkg_len > 0`, `pkg_bytes` duhet të tregojë te të paktën `pkg_len` byte
/// të lexueshëm dhe të mbetet valid gjatë thirrjes.
#[no_mangle]
pub unsafe extern "C" fn vault_write_verified(
    pkg_bytes:      *const u8,
    pkg_len:        u32,
    knowledge_band: u8,
    ts_ns:          u64,
) {
    match pkg_bytes.is_null() || pkg_len == 0 { true => return, false => {} }
    // SAFETY: kontrata e funksionit kërkon pointer të lexueshëm për `pkg_len` byte.
    let bytes = unsafe { std::slice::from_raw_parts(pkg_bytes, pkg_len as usize) }.to_vec();
    let band = match knowledge_band {
        1 => KnowledgeBand::Hypothesis,
        2 => KnowledgeBand::Fact,
        3 => KnowledgeBand::Scientific,
        _ => KnowledgeBand::Negative,
    };
    match get_global_vault() {
        Some(vault) => match vault.write_verified(ts_ns, bytes, band, 0.5, ts_ns) {
            Ok(_)  => {}
            Err(e) => eprintln!("[VAULT_FFI] ALARM: write_verified dështoi ({e:?}) — shkrim i humbur"),
        },
        None => eprintln!("[VAULT_FFI] ALARM: vault global s'është gati — write_verified HUMBI"),
    }
}

/// Callback ABI nga kerneli C për dije negative.
///
/// # Safety
/// Kur `pkg_len > 0`, `pkg_bytes` duhet të tregojë te të paktën `pkg_len` byte
/// të lexueshëm dhe të mbetet valid gjatë thirrjes.
#[no_mangle]
pub unsafe extern "C" fn vault_write_negative(
    pkg_bytes: *const u8,
    pkg_len:   u32,
    ts_ns:     u64,
) {
    match pkg_bytes.is_null() || pkg_len == 0 { true => return, false => {} }
    // SAFETY: kontrata e funksionit kërkon pointer të lexueshëm për `pkg_len` byte.
    let bytes = unsafe { std::slice::from_raw_parts(pkg_bytes, pkg_len as usize) }.to_vec();
    match get_global_vault() {
        Some(vault) => {
            let ctx = NegativeContext { failure_reason_code: 0, process: String::new(), mass_at_fail: 0.0, frequency: 1 };
            match vault.write_negative(ts_ns, bytes, ctx, ts_ns) {
                Ok(_)  => {}
                Err(e) => eprintln!("[VAULT_FFI] ALARM: write_negative dështoi ({e:?}) — dije negative e humbur"),
            }
        }
        None => eprintln!("[VAULT_FFI] ALARM: vault global s'është gati — write_negative HUMBI"),
    }
}

// ════════════════════════════════════════════════════════════════════════════
// §4  TESTE NË-KREJT (akses te write API pub(crate))
// ════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: shkruan një primitiv në një domain të dhënë (nënshkrimi i ri).
    fn wp(v: &KnowledgeVault, id: u64, domain: &str, bytes: Vec<u8>, ts: u64) {
        v.write_primitive(id, 0.9, 0.9, 0.9, 0xA451, ShadowPath::Fast, domain.to_string(), bytes, ts)
            .expect("wp: rezultat i papritur në test");
    }

    /// 5 domain-e standarde për të kristalizuar një primitiv → Legacy.
    const FIVE_DOMAINS: [&str; 5] = ["science", "industrial", "medical", "security", "research"];

    #[test]
    fn maturation_primitive_to_legacy() {
        let v = KnowledgeVault::new();
        // I njëjti input, i përforcuar në 5 DOMAIN të ndryshme → kristalizim.
        let content = b"reinforced-fact".to_vec();
        for (i, dom) in FIVE_DOMAINS.iter().enumerate() {
            wp(&v, 100 + i as u64, dom, content.clone(), 1000 + i as u64);
        }
        // Dedupe me content_hash → NJË entry me zinxhir 5-gjurmësh (input_id=100).
        assert_eq!(v.primitive_count(), 1);
        assert_eq!(v.legacy_count(), 0);

        // LIGJI I PËRFORCUAR (FAZA 3): 5 domaine janë të nevojshme por JO të
        // mjaftueshme — legacy lind me KOHË. real_hits=0 → ende s'maturohet.
        let promoted_before = v.try_promote_to_legacy();
        assert_eq!(promoted_before, 0);
        assert_eq!(v.legacy_count(), 0);

        // Konfirmim nga bota reale (≥3 hits) → maturim → promovohet tani.
        // (input_id=100 mbahet nga shfaqja e parë pas dedupe.)
        let promoted = v.confirm_primitive(100, 3);
        assert_eq!(promoted, 1);
        assert_eq!(v.primitive_count(), 0);
        assert_eq!(v.legacy_count(), 1);

        // FAZA 7 — vëzhguesi regjistroi lindjen me arsyen e plotë.
        assert_eq!(v.observer.birth_count(), 1);
        assert!(v.observer.was_observed(100));
        let reason = v.observer.reason_for(100);
        assert!(reason.is_some());
        let reason = reason.expect("maturation_primitive_to_legacy: rezultat i papritur në test");
        assert!(reason.cross_domain_ok);
        assert!(reason.maturation_ok);
        assert_eq!(reason.domain_count, 5);
    }

    #[test]
    fn four_domains_do_not_promote() {
        let v = KnowledgeVault::new();
        // Vetëm 4 domain-e unike (< 5) → NUK kristalizohet.
        let content = b"not-yet".to_vec();
        for (i, dom) in FIVE_DOMAINS.iter().take(4).enumerate() {
            wp(&v, 200 + i as u64, dom, content.clone(), 1 + i as u64);
        }
        assert_eq!(v.primitive_count(), 1);
        assert_eq!(v.try_promote_to_legacy(), 0);
        assert_eq!(v.primitive_count(), 1);
        assert_eq!(v.legacy_count(), 0);
    }

    #[test]
    fn same_domain_five_times_does_not_promote() {
        let v = KnowledgeVault::new();
        // 5 përforcime por NË TË NJËJTIN domain → 1 domain unik → s'promovon.
        let content = b"echo".to_vec();
        for i in 0..5u64 {
            wp(&v, 300 + i, "science", content.clone(), 1 + i);
        }
        assert_eq!(v.primitive_count(), 1);
        assert_eq!(v.try_promote_to_legacy(), 0); // unique_domains == 1
        assert_eq!(v.legacy_count(), 0);
    }

    #[test]
    fn fresh_primitive_does_not_promote() {
        let v = KnowledgeVault::new();
        // Një domain i vetëm → unique_domains = 1 < 5 → s'promovon.
        wp(&v, 7, "science", vec![9], 1);
        let promoted = v.run_maturation();
        assert_eq!(promoted, 0);
        assert_eq!(v.primitive_count(), 1);
        assert_eq!(v.legacy_count(), 0);
    }

    #[test]
    fn negative_dedup_increments_frequency() {
        let v = KnowledgeVault::new();
        let ctx = NegativeContext {
            failure_reason_code: 0x101,
            process: "INDUSTRIAL".to_string(),
            mass_at_fail: 0.50,
            frequency: 1,
        };
        v.write_negative(1, vec![0], ctx.clone(), 10).expect("negative_dedup_increments_frequency: rezultat i papritur në test");
        v.write_negative(2, vec![0], ctx.clone(), 20).expect("negative_dedup_increments_frequency: rezultat i papritur në test");
        assert_eq!(v.negative_count(), 1); // dedupe — i njëjti model
        let store = v.negative.read().expect("negative_dedup_increments_frequency: rezultat i papritur në test");
        assert_eq!(store[0].frequency, 2);
    }

    #[test]
    fn known_negative_boundary_lookup() {
        let v = KnowledgeVault::new();
        let ctx = NegativeContext {
            failure_reason_code: 0x1,
            process: "TERR".to_string(),
            mass_at_fail: 0.50,
            frequency: 1,
        };
        v.write_negative(1, vec![0], ctx, 10).expect("known_negative_boundary_lookup: rezultat i papritur në test");
        assert!(v.check_known_negative(0.50, "TERR").is_some());
        assert!(v.check_known_negative(0.50, "OTHER").is_none());
        assert!(v.check_known_negative(0.90, "TERR").is_none()); // jashtë tolerancës
    }

    #[test]
    fn verified_routing_by_band() {
        let v = KnowledgeVault::new();
        v.write_verified(1, vec![0], KnowledgeBand::Hypothesis, 0.80, 1).expect("verified_routing_by_band: rezultat i papritur në test");
        v.write_verified(2, vec![0], KnowledgeBand::Fact, 1.00, 2).expect("verified_routing_by_band: rezultat i papritur në test");
        v.write_verified(3, vec![0], KnowledgeBand::Scientific, 1.00, 3).expect("verified_routing_by_band: rezultat i papritur në test");
        assert_eq!(v.hypothesis_count(), 1);
        assert_eq!(v.fact_count(), 2); // Fact + Scientific → fact store
    }

    // ── FAZA 2 — durabiliteti (round-trip në disk) ──────────────────────

    fn temp_wal(tag: &str) -> std::path::PathBuf {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).expect("temp_wal: rezultat i papritur në test").as_nanos();
        let mut p = std::env::temp_dir();
        p.push(format!("shadow_wal_{}_{}_{}.log", tag, std::process::id(), nanos));
        p
    }

    #[test]
    fn durability_roundtrip() {
        let path = temp_wal("roundtrip");

        // Sesioni 1 — shkruaj dije, pastaj "rrëzo" (drop).
        {
            let v = KnowledgeVault::open_disk(&path).expect("durability_roundtrip: rezultat i papritur në test");
            wp(&v, 100, "science", b"prim".to_vec(), 1);
            v.write_verified(200, b"hyp".to_vec(), KnowledgeBand::Hypothesis, 0.80, 2).expect("durability_roundtrip: rezultat i papritur në test");
            v.write_verified(300, b"fact".to_vec(), KnowledgeBand::Fact, 1.00, 3).expect("durability_roundtrip: rezultat i papritur në test");
            let ctx = NegativeContext {
                failure_reason_code: 0x1, process: "TERR".to_string(),
                mass_at_fail: 0.5, frequency: 1,
            };
            v.write_negative(400, b"neg".to_vec(), ctx.clone(), 4).expect("durability_roundtrip: rezultat i papritur në test");
            v.write_negative(401, b"neg2".to_vec(), ctx, 5).expect("durability_roundtrip: rezultat i papritur në test"); // dedupe → frekuencë 2
            assert_eq!(v.primitive_count(), 1);
            assert_eq!(v.negative_count(), 1);
        }

        // Sesioni 2 — rihap; gjendja rikrijohet EKZAKT nga WAL-i.
        {
            let v = KnowledgeVault::open_disk(&path).expect("durability_roundtrip: rezultat i papritur në test");
            assert_eq!(v.primitive_count(),  1);
            assert_eq!(v.hypothesis_count(), 1);
            assert_eq!(v.fact_count(),       1);
            assert_eq!(v.negative_count(),   1);
            let neg = v.check_known_negative(0.5, "TERR").expect("durability_roundtrip: rezultat i papritur në test");
            assert_eq!(neg.frequency, 2); // dedup-i mbijetoi restart-in
        }

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn promotion_survives_restart() {
        let path = temp_wal("promote");
        let content = b"p".to_vec();

        {
            let v = KnowledgeVault::open_disk(&path).expect("promotion_survives_restart: rezultat i papritur në test");
            // I njëjti input në 5 domain → kristalizim Legacy.
            for (i, dom) in FIVE_DOMAINS.iter().enumerate() {
                wp(&v, 500 + i as u64, dom, content.clone(), 1 + i as u64);
            }
            assert_eq!(v.primitive_count(), 1);
            let promoted = v.try_promote_to_legacy();
            assert_eq!(promoted, 1);
            assert_eq!(v.primitive_count(), 0);
            assert_eq!(v.legacy_count(),    1);
        }

        {
            // Promovimi riprodhohet nga ripërsëritja: 5 ngjarje Primitive
            // (secila me domain-in e vet) → apply + try_promote → Legacy.
            let v = KnowledgeVault::open_disk(&path).expect("promotion_survives_restart: rezultat i papritur në test");
            assert_eq!(v.primitive_count(), 0);
            assert_eq!(v.legacy_count(),    1);
        }

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn corrupt_tail_is_ignored() {
        use std::io::Write;
        let path = temp_wal("corrupt");

        {
            let v = KnowledgeVault::open_disk(&path).expect("corrupt_tail_is_ignored: rezultat i papritur në test");
            wp(&v, 1, "science", b"ok".to_vec(), 1);
        }
        // Shto bytes mbeturina (rekord i paplotë/i dëmtuar) në fund.
        {
            let mut f = std::fs::OpenOptions::new().append(true).open(&path).expect("corrupt_tail_is_ignored: rezultat i papritur në test");
            f.write_all(&[0xFF, 0xFF, 0xFF, 0xFF, 0x00]).expect("corrupt_tail_is_ignored: rezultat i papritur në test");
        }
        // Rihapja shpërfill bishtin e dëmtuar dhe ruan rekordin e mirë.
        {
            let v = KnowledgeVault::open_disk(&path).expect("corrupt_tail_is_ignored: rezultat i papritur në test");
            assert_eq!(v.primitive_count(), 1);
        }

        let _ = std::fs::remove_file(&path);
    }
}
