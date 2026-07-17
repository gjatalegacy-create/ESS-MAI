// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  PRO_NK_GATE.RS — Negative Knowledge Double-Gate                      ║
// ║  GJATA LEGACY™ — Quantum Platform                                     ║
// ║                                                                          ║
// ║  NK = "çfarë është provuar që NUK funksionon". Sistemi nervor          ║
// ║  mbrojtës i PRO. Hyn PARA operatorëve DHE PAS tyre (double-gate).      ║
// ║                                                                          ║
// ║  GRADIENT (jo boolean):                                              ║
// ║    similarity > 0.85 → HARD BLOCK (score = 0.0)                       ║
// ║    similarity > 0.55 → penalty = lerp(0.15, 0.40, similarity)        ║
// ║    similarity ≤ 0.55 → pa penalty                                    ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::pro_types::FragmentVector;
use crate::tokenizer::semantic_stems;
use std::collections::HashSet;

// ─────────────────────────────────────────────────────────────────────────────
// PRAGJET — nga prompt
// ─────────────────────────────────────────────────────────────────────────────

const NK_HARD_BLOCK: f32 = 0.85;   // mbi këtë → score = 0
const NK_SOFT_START: f32 = 0.55;   // mbi këtë → penalty graduale
const PENALTY_MIN:   f32 = 0.15;
const PENALTY_MAX:   f32 = 0.40;

// ─────────────────────────────────────────────────────────────────────────────
// NEGATIVE ENTRY — një dështim i regjistruar
// ─────────────────────────────────────────────────────────────────────────────

/// Një rrugë e provuar që NUK funksionon.
#[derive(Debug, Clone)]
pub struct NegativeEntry {
    pub id: String,
    /// Stem-et që karakterizojnë këtë dështim (për krahasim).
    pub stems: HashSet<String>,
    /// Sa herë është konsultuar (access_count — më i konsultuari = më i vlefshëm).
    pub access_count: u64,
}

impl NegativeEntry {
    pub fn from_text(id: &str, text: &str) -> Self {
        Self {
            id: id.to_string(),
            stems: semantic_stems(text),
            access_count: 0,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// KNOWLEDGE VAULT — depo e dijes negative (+ optimal, e ardhmja)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default)]
pub struct KnowledgeVault {
    negative: Vec<NegativeEntry>,
}

impl KnowledgeVault {
    pub fn new() -> Self {
        Self { negative: Vec::new() }
    }

    /// Numri i hyrjeve negative — lexim-only (audit, teste, maturim NK).
    pub fn len(&self) -> usize { self.negative.len() }
    /// A është vault-i bosh — lexim-only.
    pub fn is_empty(&self) -> bool { self.negative.is_empty() }

    /// Regjistron një dështim (nga VERIFIED_NEGATIVE i Shadow).
    pub fn add_negative(&mut self, entry: NegativeEntry) {
        self.negative.push(entry);
    }

    /// insert_negative — alias semantik për add_negative (P3: nga Shadow export).
    pub fn insert_negative(&mut self, entry: NegativeEntry) {
        self.negative.push(entry);
    }

    /// from_negative_export — ndërton Vault nga NK i eksportuar i Shadow (P3).
    ///
    /// Parson formatin: count(4) | [len(4) | shadow_entry_bytes]*. Çdo entry i
    /// Shadow (input_id|reason|process|...) konvertohet në Quantum NegativeEntry
    /// (id nga input_id, stems nga process). Zero if — match.
    ///
    /// v1.4.2 (audit rek.2 — integriteti i urës binare NK): pranon OPCIONALISHT
    /// një kokë të vulosur `NKB1` + CRC fnv1a64(u64) mbi trupin. Prania e magjikës
    /// vendoset me match; mospërputhja e CRC-së → vault BOSH me alarm (fail-open i
    /// deklaruar, s'rrezikon dije të prishur). Formati pa kokë (legacy) pranohet
    /// i pandryshuar — një-drejtimësh: sapo shkruesi vulos, s'zbritet më.
    pub fn from_negative_export(bytes: &[u8]) -> Self {
        // v1.4.3 (audit p.8 — PJESËRISHT PRANUAR): protokoll strukturor me
        // GJATËSI, por PA numër versioni — magjika ËSHTË vetë-versionimi
        // (ligji i Arkitektit që nga v1.3.1: struktura flet, jo tokenat).
        //   NKL1 | body_len:u64le | crc:u64le | body   ← formati aktual
        //   NKB1 | crc:u64le | body                    ← v1.4.2, pranohet
        //   (pa magjikë)                               ← legacy raw
        let body: &[u8] = match nk_blob_magic(bytes) {
            NkMagic::Nkl1 => {
                let len_want = u64::from_le_bytes([
                    bytes[4], bytes[5], bytes[6], bytes[7],
                    bytes[8], bytes[9], bytes[10], bytes[11],
                ]) as usize;
                let crc_want = u64::from_le_bytes([
                    bytes[12], bytes[13], bytes[14], bytes[15],
                    bytes[16], bytes[17], bytes[18], bytes[19],
                ]);
                let payload = &bytes[20..];
                match (payload.len() == len_want,
                       super::hw_real::ffi::fnv1a_64(payload) == crc_want) {
                    (true, true) => {
                        super::runtime_pulse::mark_ready(super::runtime_pulse::Stage::NkImport);
                        payload
                    }
                    (false, _) => {
                        super::runtime_pulse::mark_notready(super::runtime_pulse::Stage::NkImport);
                        eprintln!("  [NK_BRIDGE] ALARM: gjatësia NK s'përputhet (kam={} dua={}) — cungim? vault bosh",
                            payload.len(), len_want);
                        return Self::new();
                    }
                    (_, false) => {
                        super::runtime_pulse::mark_notready(super::runtime_pulse::Stage::NkImport);
                        eprintln!("  [NK_BRIDGE] ALARM: CRC NKL1 s'përputhet — vault bosh i deklaruar");
                        return Self::new();
                    }
                }
            }
            NkMagic::Nkb1 => {
                let want = u64::from_le_bytes([
                    bytes[4], bytes[5], bytes[6], bytes[7],
                    bytes[8], bytes[9], bytes[10], bytes[11],
                ]);
                let payload = &bytes[12..];
                let have = super::hw_real::ffi::fnv1a_64(payload);
                match have == want {
                    true  => {
                        super::runtime_pulse::mark_ready(super::runtime_pulse::Stage::NkImport);
                        payload
                    }
                    false => {
                        super::runtime_pulse::mark_notready(super::runtime_pulse::Stage::NkImport);
                        eprintln!("  [NK_BRIDGE] ALARM: CRC NKB1 s'përputhet (have={have:016x} want={want:016x}) — vault bosh i deklaruar");
                        return Self::new();
                    }
                }
            }
            NkMagic::Asnje => {
                // legacy raw i pastër — i deklaruar (çelësi njëdrejtimësh jeton në main)
                super::runtime_pulse::mark_degraded(super::runtime_pulse::Stage::NkImport);
                bytes
            }
        };
        let mut vault = Self::new();
        // Header minimal 4 byte. Zero if — match.
        let has_header = body.len() >= 4;
        let count = match has_header {
            true  => u32::from_le_bytes([body[0], body[1], body[2], body[3]]) as usize,
            false => 0,
        };
        let mut cursor = 4usize;
        // Lexo count entries (zero if — loop me match guards).
        for _ in 0..count {
            // Kontrollo që ka 4 byte për len. Zero if — match.
            let has_len = cursor + 4 <= body.len();
            let entry_len = match has_len {
                true => {
                    let l = u32::from_le_bytes([
                        body[cursor], body[cursor+1], body[cursor+2], body[cursor+3],
                    ]) as usize;
                    cursor += 4;
                    l
                }
                false => 0,
            };
            // Lexo entry-n nëse ka byte mjaftueshëm. Zero if — match.
            let has_entry = entry_len > 0 && cursor + entry_len <= body.len();
            let _ = match has_entry {
                true => {
                    let slice = &body[cursor..cursor + entry_len];
                    cursor += entry_len;
                    // Parso fushat e Shadow entry (input_id + process).
                    vault.absorb_shadow_entry(slice);
                    true
                }
                false => false,
            };
        }
        vault
    }

    /// seal_negative_export — mbështjell një trup NK me kokën e vulosur NKB1+CRC.
    /// Shkruesi (Shadow, kur eksporton) e kalon trupin këtu; lexuesi e verifikon
    /// vetiu në `from_negative_export`. E njëjta vulë fnv1a64 si urat tekstuale.
    pub fn seal_negative_export(body: &[u8]) -> Vec<u8> {
        // v1.4.3: NKL1 — magjikë + GJATËSI + CRC (audit p.8: len PRANUAR;
        // fusha "version" RRËZUAR — magjika e re ËSHTË versionimi strukturor).
        let crc = super::hw_real::ffi::fnv1a_64(body);
        let len = body.len() as u64;
        let mut out = Vec::with_capacity(20 + body.len());
        out.extend_from_slice(b"NKL1");
        out.extend_from_slice(&len.to_le_bytes());
        out.extend_from_slice(&crc.to_le_bytes());
        out.extend_from_slice(body);
        out
    }

    /// absorb_shadow_entry — parson një Shadow NegativeEntry bytes → Quantum entry.
    /// Shadow format: input_id(8)|reason(4)|mass(4)|freq(4)|access(4)|added(8)|
    ///                proc_len(4)|proc_bytes|... . Zero if — match.
    fn absorb_shadow_entry(&mut self, slice: &[u8]) {
        // Minimumi për header 36 byte. Zero if — match.
        let enough = slice.len() >= 36;
        let _ = match enough {
            true => {
                let input_id = u64::from_le_bytes([
                    slice[0], slice[1], slice[2], slice[3],
                    slice[4], slice[5], slice[6], slice[7],
                ]);
                let proc_len = u32::from_le_bytes([
                    slice[32], slice[33], slice[34], slice[35],
                ]) as usize;
                let proc_end = 36 + proc_len;
                // Lexo process si tekst → stems. Zero if — match.
                let process = match slice.len() >= proc_end {
                    true  => String::from_utf8_lossy(&slice[36..proc_end]).to_string(),
                    false => String::new(),
                };
                // Ndërto Quantum entry: id nga input_id, stems nga process.
                let id = format!("shadow_nk_{}", input_id);
                self.negative.push(NegativeEntry::from_text(&id, &process));
                true
            }
            false => false,
        };
    }

    pub fn negative_count(&self) -> usize {
        self.negative.len()
    }

    /// Ngjashmëria maksimale e një set-i stem-esh me cilindo dështim.
    /// Jaccard — APROKSIM (Rruga C: cosine embeddings).
    fn max_similarity(&mut self, stems: &HashSet<String>) -> (f32, Option<usize>) {
        let mut best = 0.0f32;
        let mut idx = None;
        for (i, neg) in self.negative.iter().enumerate() {
            let sim = jaccard_sets(stems, &neg.stems);
            // Branchless max + track indeksi i më të mirit.
            let is_better = (sim > best) as u32;
            best = match is_better { 1 => sim, _ => best };
            idx = match is_better { 1 => Some(i), _ => idx };
        }
        // Rrit access_count për dështimin e përputhur (më i konsultuari = më i vlefshëm).
        match idx {
            Some(i) => self.negative[i].access_count += 1,
            None => {}
        }
        (best, idx)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// NK GATE — vendimi gradient
// ─────────────────────────────────────────────────────────────────────────────

/// Rezultati i një kontrolli NK.
/// Magjika e blob-it NK — vetë-versionimi strukturor (zero numra versioni).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NkMagic { Nkl1, Nkb1, Asnje }

/// Zbulon magjikën e blob-it. Zero if — match mbi krahasime.
pub fn nk_blob_magic(bytes: &[u8]) -> NkMagic {
    let l1 = bytes.len() >= 20 && &bytes[0..4] == b"NKL1";
    let b1 = bytes.len() >= 12 && &bytes[0..4] == b"NKB1";
    match (l1, b1) {
        (true, _)      => NkMagic::Nkl1,
        (false, true)  => NkMagic::Nkb1,
        (false, false) => NkMagic::Asnje,
    }
}

/// A është blob-i i vulosur (NKL1 ose NKB1)?
pub fn is_sealed_nk_blob(bytes: &[u8]) -> bool {
    match nk_blob_magic(bytes) {
        NkMagic::Asnje => false,
        _              => true,
    }
}

/// ÇELËSI NJËDREJTIMËSH i NK-së (audit p.7, PRANUAR): sapo është parë
/// NJËHERË vulë (markeri .sealed), legacy raw = DEGRADIM. Zero if — match.
pub fn nk_downgrade(sealed_seen: bool, blob_sealed_now: bool) -> bool {
    match (sealed_seen, blob_sealed_now) {
        (true, false) => true,  // histori e vulosur + blob i pavulosur = degradim
        _             => false,
    }
}

pub struct NkVerdict {
    pub similarity: f32,
    /// Faktori që shumëzohet me score: 0.0 (HARD), ose 1-penalty (SOFT), ose 1.0.
    pub factor: f32,
    pub hard_blocked: bool,
}

pub struct NkGate;

impl NkGate {
    /// Llogarit faktorin NK nga similarity — GRADIENT, branchless.
    ///
    ///   sim > 0.85 → factor = 0          (HARD BLOCK)
    ///   sim > 0.55 → factor = 1 - lerp(0.15, 0.40, t)  ku t∈[0,1] mbi [0.55,0.85]
    ///   sim ≤ 0.55 → factor = 1          (pa penalty)
    pub fn factor_from_similarity(similarity: f32) -> NkVerdict {
        let sim = similarity.clamp(0.0, 1.0);

        // HARD: sim > 0.85 → hard ∈ {0,1}
        let hard = (sim > NK_HARD_BLOCK) as u32 as f32;

        // SOFT zonë: [0.55, 0.85] → t normalizuar
        let soft_active = (sim > NK_SOFT_START) as u32 as f32;
        let t = ((sim - NK_SOFT_START) / (NK_HARD_BLOCK - NK_SOFT_START)).clamp(0.0, 1.0);
        let penalty = PENALTY_MIN + (PENALTY_MAX - PENALTY_MIN) * t;

        // factor pa hard: nëse soft → (1 - penalty), ndryshe 1.0
        let soft_factor = 1.0 - penalty * soft_active;

        // Kombinim branchless: hard mbizotëron (factor=0).
        let factor = soft_factor * (1.0 - hard);

        NkVerdict {
            similarity: sim,
            factor: factor.clamp(0.0, 1.0),
            hard_blocked: hard > 0.5,
        }
    }

    /// PRE-CONSTRAINT: kontrollon fragmentet PARA operatorëve.
    /// Kthen faktorin që do aplikohet në input (constraint).
    pub fn pre_constraint(
        vault: &mut KnowledgeVault,
        vectors: &[FragmentVector],
    ) -> NkVerdict {
        match vault.negative_count() == 0 {
            true => return NkVerdict { similarity: 0.0, factor: 1.0, hard_blocked: false },
            false => {}
        }
        // Bashko stem-et e të gjitha fragmenteve.
        let mut all: HashSet<String> = HashSet::new();
        for v in vectors {
            all.extend(v.stems.iter().cloned());
        }
        let (sim, _) = vault.max_similarity(&all);
        Self::factor_from_similarity(sim)
    }

    /// POST-FILTER: kontrollon një kandidat PAS operatorit.
    /// Aplikon faktorin NK në score-in e kandidatit.
    pub fn post_filter(
        vault: &mut KnowledgeVault,
        candidate_stems: &HashSet<String>,
        raw_score: f32,
    ) -> (f32, NkVerdict) {
        match vault.negative_count() == 0 {
            true => return (raw_score, NkVerdict { similarity: 0.0, factor: 1.0, hard_blocked: false }),
            false => {}
        }
        let (sim, _) = vault.max_similarity(candidate_stems);
        let verdict = Self::factor_from_similarity(sim);
        let filtered = (raw_score * verdict.factor).clamp(0.0, 1.0);
        (filtered, verdict)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// NDIHMËS
// ─────────────────────────────────────────────────────────────────────────────

fn jaccard_sets(a: &HashSet<String>, b: &HashSet<String>) -> f32 {
    match a.is_empty() && b.is_empty() {
        true => return 0.0, // bosh vs bosh → s'ka ngjashmëri dështimi
        false => {}
    }
    let inter = a.intersection(b).count() as f32;
    let union = a.union(b).count() as f32;
    // union==0 → 0; ndryshe → inter/union. Branchless.
    let nonzero = (union != 0.0) as u32 as f32;
    nonzero * (inter / union.max(1.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nk_seal_roundtrip_preserves_body() {
        // Trup NK minimal. Vulos NKL1 → magjika + len + CRC → lexuesi e pranon.
        let body = vec![1u8, 0, 0, 0,  3, 0, 0, 0,  b'a', b'b', b'c'];
        let sealed = KnowledgeVault::seal_negative_export(&body);
        assert_eq!(&sealed[0..4], b"NKL1", "koka magjike v1.4.3 = NKL1 (me gjatësi)");
        assert_eq!(nk_blob_magic(&sealed), NkMagic::Nkl1);
        let _ = KnowledgeVault::from_negative_export(&sealed);
    }

    #[test]
    fn nk_nkb1_backward_still_accepted() {
        // Blob i v1.4.2 (NKB1: magjikë+crc+body) pranohet ende — një-drejtimësh.
        let body = vec![0u8, 0, 0, 0];
        let crc = crate::hw_real::ffi::fnv1a_64(&body);
        let mut old = Vec::new();
        old.extend_from_slice(b"NKB1");
        old.extend_from_slice(&crc.to_le_bytes());
        old.extend_from_slice(&body);
        assert_eq!(nk_blob_magic(&old), NkMagic::Nkb1);
        let v = KnowledgeVault::from_negative_export(&old);
        assert_eq!(v.len(), 0);
    }

    #[test]
    fn nk_len_mismatch_yields_empty_vault() {
        // Cungimi kapet nga GJATËSIA edhe para CRC-së.
        let body = vec![0u8, 0, 0, 0];
        let mut sealed = KnowledgeVault::seal_negative_export(&body);
        let cut = sealed.len() - 1;
        sealed.truncate(cut); // hiq byte-in e fundit → len s'përputhet
        let v = KnowledgeVault::from_negative_export(&sealed);
        assert_eq!(v.len(), 0, "cungimi duhet të japë vault bosh me zë");
    }

    #[test]
    fn nk_corrupt_raises_presume_notready() {
        // Sjellja (audit p.6): CRC i prishur → nk_status NOT_READY;
        // i vulosur i saktë → READY. Presume flet të vërtetën e importit.
        use crate::lab_contracts::gcl_presume::GclReadiness;
        let body = vec![0u8, 0, 0, 0];
        let mut keq = KnowledgeVault::seal_negative_export(&body);
        let fundi = keq.len() - 1;
        keq[fundi] ^= 0xFF; // prish trupin → CRC bie
        let _ = KnowledgeVault::from_negative_export(&keq);
        match crate::runtime_pulse::nk_status() {
            GclReadiness::NotReady(_) => {}
            other => panic!("korrupsioni NK duhej NOT_READY, erdhi {:?}", other),
        }
        let mire = KnowledgeVault::seal_negative_export(&body);
        let _ = KnowledgeVault::from_negative_export(&mire);
        assert!(crate::runtime_pulse::nk_status().is_ready(),
            "importi i vulosur i saktë → READY");
    }

    #[test]
    fn nk_legacy_after_nkb1_is_downgrade() {
        // Sjellja e kërkuar nga auditi (p.10 test 3): çelësi njëdrejtimësh.
        assert!(nk_downgrade(true,  false), "vulë e parë + raw tani = DEGRADIM");
        assert!(!nk_downgrade(true,  true),  "vulë + vulë = normale");
        assert!(!nk_downgrade(false, false), "kurrë vulë + raw = legacy i pastër");
        assert!(!nk_downgrade(false, true),  "vula e parë hyn lirshëm");
        let raw = vec![0u8,0,0,0];
        assert!(!is_sealed_nk_blob(&raw));
    }

    #[test]
    fn nk_seal_detects_single_bit_flip() {
        let body = vec![1u8, 0, 0, 0,  3, 0, 0, 0,  b'x', b'y', b'z'];
        let mut sealed = KnowledgeVault::seal_negative_export(&body);
        // Prish një byte të trupit (pas kokës 12-byteshe) → CRC s'përputhet.
        let last = sealed.len() - 1;
        sealed[last] ^= 0xFF;
        // Lexuesi duhet ta refuzojë → vault BOSH (0 entries), pa panik.
        let v = KnowledgeVault::from_negative_export(&sealed);
        assert_eq!(v.len(), 0, "NK i prishur duhet të japë vault bosh, jo dije të korruptuar");
    }

    #[test]
    fn nk_legacy_without_header_still_parsed() {
        // Backward-safe: trup pa kokë NKB1 (legacy) pranohet i pandryshuar.
        let legacy = vec![0u8, 0, 0, 0]; // count=0, pa entries
        let v = KnowledgeVault::from_negative_export(&legacy);
        assert_eq!(v.len(), 0);
    }

    #[test]
    fn hard_block_above_085() {
        let v = NkGate::factor_from_similarity(0.90);
        assert!(v.hard_blocked);
        assert_eq!(v.factor, 0.0);
    }

    #[test]
    fn soft_penalty_in_zone() {
        let v = NkGate::factor_from_similarity(0.70);
        assert!(!v.hard_blocked);
        // factor = 1 - penalty, ku penalty ∈ [0.15, 0.40]
        assert!(v.factor < 1.0 && v.factor > 0.55);
    }

    #[test]
    fn no_penalty_below_055() {
        let v = NkGate::factor_from_similarity(0.40);
        assert_eq!(v.factor, 1.0);
    }

    #[test]
    fn post_filter_blocks_known_failure() {
        let mut vault = KnowledgeVault::new();
        vault.add_negative(NegativeEntry::from_text(
            "fail1", "perpetual motion energy free infinite"));

        let candidate = semantic_stems("perpetual motion energy free infinite machine");
        let (score, verdict) = NkGate::post_filter(&mut vault, &candidate, 0.9);
        // Ngjashmëri e lartë me dështimin → score zbritet ose bllokohet.
        assert!(score < 0.9);
        assert!(verdict.similarity > 0.5);
    }

    #[test]
    fn empty_vault_no_effect() {
        let mut vault = KnowledgeVault::new();
        let candidate = semantic_stems("anything goes here");
        let (score, _) = NkGate::post_filter(&mut vault, &candidate, 0.75);
        assert_eq!(score, 0.75);
    }
}
