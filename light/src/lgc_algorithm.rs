// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LGC_ALGORITHM.RS — ALGORITMI SOFTUERIK LGC (Rust)                    ║
// ║  GJATA LEGACY™  — ESS MAI                                              ║
// ║                                                                          ║
// ║  Gjeneron KODUNIK — i padeshifrueshëm në binar.                       ║
// ║  Python versioni (lgc_algorithm_software.py) ELIMINOHET.               ║
// ║                                                                          ║
// ║  ALGORITMI:                                                              ║
// ║  SHA256(input) → HMAC-SHA256(sha, counter, ts) → KODUNIK              ║
// ║  Format: KDS:SHA8-COUNTER6HEX-SUFFIX4                                  ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use std::sync::Mutex;

type HmacSha256 = Hmac<Sha256>;

const KODUNIK_HMAC_LEN: usize = 12;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LgcAlgorithmError {
    InvalidSha256,
    InvalidToken(&'static str),
    HmacFailure,
    RegistryUnavailable,
}

impl std::fmt::Display for LgcAlgorithmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSha256 => write!(f, "LGC_INVALID_SHA256"),
            Self::InvalidToken(name) => write!(f, "LGC_INVALID_TOKEN:{name}"),
            Self::HmacFailure => write!(f, "LGC_HMAC_FAILURE"),
            Self::RegistryUnavailable => write!(f, "LGC_REGISTRY_UNAVAILABLE"),
        }
    }
}

// Secret key — e inkorporuar në binar (jo loadohet nga jashtë)
// Rust e inkorporon direkt në .text sektion → e padeshifrueshme
const LGC_SECRET: &[u8] = b"GJATA_LEGACY_SOVEREIGN_KEY_ESS_MAI_V1_RUST";

// Atomik counter — thread-safe pa Mutex
static COUNTER: AtomicU64 = AtomicU64::new(0);

// RUNTIME HARDENING (      , nga GPT runtime_patch): regjistër KODUNIK për
// verifikim REAL (sha256 + HMAC), jo vetëm përputhje prefiksi SHA.
#[derive(Debug, Clone)]
struct KodunikRecord {
    sha256:    String,
    hmac_sig:  String,
    counter:   u64,
    timestamp: u64,
}

// ─────────────────────────────────────────────────────────────────────────────
// KODUNIK RESULT
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct KodunikResult {
    pub code:      String,   // KDS:XXXXXXXX-YYYYYY-SUFFIX4
    pub sha256:    String,   // SHA i inputit
    pub hmac_sig:  String,   // HMAC-SHA256 12 char
    pub counter:   u64,
    pub timestamp: u64,
    pub valid:     bool,
}

impl KodunikResult {
    /// Verifikon autenticitetin e KODUNIK (      : sha256 check para HMAC).
    pub fn verify(&self, original_sha: &str) -> bool {
        // RUNTIME HARDENING: sha256 duhet të përputhet PARA verifikimit HMAC.
        // Zero if — match mbi barazinë.
        self.valid
            && self.sha256 == original_sha
            && verify_kodunik_attestation(
                &self.code,
                original_sha,
                &self.hmac_sig,
                self.counter,
                self.timestamp,
            )
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ALGORITMI KRYESOR
// ─────────────────────────────────────────────────────────────────────────────

/// Gjeneron SHA256 hex nga data arbitrare
pub fn compute_sha256(data: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(data);
    h.finalize()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

pub fn is_canonical_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

/// SHA-256 me domain separation dhe length-prefix për çdo pjesë.
pub fn compute_domain_sha(domain: &str, parts: &[&[u8]]) -> String {
    let mut h = Sha256::new();
    h.update(b"ESSMAI-DOMAIN-SHA256-V1\0");
    h.update((domain.len() as u64).to_le_bytes());
    h.update(domain.as_bytes());
    for part in parts {
        h.update((part.len() as u64).to_le_bytes());
        h.update(part);
    }
    h.finalize()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

fn is_clean_kodunik_token(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 16
        && value.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
}

pub fn verify_kodunik_attestation(
    code: &str,
    sha256: &str,
    hmac_sig: &str,
    counter: u64,
    timestamp: u64,
) -> bool {
    if !is_canonical_sha256(sha256)
        || hmac_sig.len() != KODUNIK_HMAC_LEN
        || !hmac_sig
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'A'..=b'F').contains(&b))
    {
        return false;
    }
    let sha_prefix = sha256[..8].to_uppercase();
    let expected_prefix = format!("KDS:{sha_prefix}-{:06X}-", counter % 0xFF_FFFF);
    if !code.starts_with(&expected_prefix) || code.len() <= expected_prefix.len() {
        return false;
    }
    let expected = generate_hmac(sha256, counter, timestamp, code);
    expected.len() >= KODUNIK_HMAC_LEN
        && constant_time_eq(&expected[..KODUNIK_HMAC_LEN], hmac_sig)
}

/// HMAC-SHA256 → 12 char uppercase
fn generate_hmac(sha256: &str, counter: u64, ts: u64, code: &str) -> String {
    let msg = format!("ESSMAI/LGC/KODUNIK/V2|{}|{}|{}|{}", sha256, counter, ts, code);
    // INVARIANT I GARANTUAR: HMAC-SHA256 pranon çelës të çdo gjatësie, dhe
    // LGC_SECRET është konstante valide e inkorporuar në binar → new_from_slice
    // s'dështon KURRË. Fail-closed pa panik (ligji: zero panik në runtime):
    // dega Err (strukturalisht e paarritshme) kthen shënues korrupsioni që
    // vulën NUK e vlen — s'shkatërron sealing-un, e refuzon. Ok-rruga (e vetmja
    // që ekzekutohet) mbetet byte-për-byte e pandryshuar.
    match HmacSha256::new_from_slice(LGC_SECRET) {
        Ok(mut mac) => {
            mac.update(msg.as_bytes());
            mac.finalize()
                .into_bytes()
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>()
                .to_uppercase()
        }
        Err(_) => String::from("HMAC_INIT_FAIL"), // fail-closed: vulë e pavlefshme → refuzim
    }
}

/// Krahasim constant-time — parandalon timing attacks
fn constant_time_eq(a: &str, b: &str) -> bool {
    match a.len() != b.len() {
        true => return false,
        false => {}
    }
    a.bytes()
        .zip(b.bytes())
        .fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

// ─────────────────────────────────────────────────────────────────────────────
// LGC ALGORITHM — SINGLETON (Rust static initialization — thread-safe)
// ─────────────────────────────────────────────────────────────────────────────

pub struct LgcAlgorithm {
    generated:     AtomicU64,
    verified_ok:   AtomicU64,
    verified_fail: AtomicU64,
    // RUNTIME HARDENING (      ): regjistër KODUNIK→meta për verifikim real.
    registry:      Mutex<HashMap<String, KodunikRecord>>,
}

// Global singleton — inicializohet 1 herë, lazy, thread-safe
static LGC_ALGO: std::sync::OnceLock<LgcAlgorithm> = std::sync::OnceLock::new();

impl LgcAlgorithm {
    pub fn instance() -> &'static Self {
        LGC_ALGO.get_or_init(|| LgcAlgorithm {
            generated:     AtomicU64::new(0),
            verified_ok:   AtomicU64::new(0),
            verified_fail: AtomicU64::new(0),
            registry:      Mutex::new(HashMap::new()),
        })
    }

    /// Gjeneron KODUNIK nga SHA256
    /// Format: KDS:SHA8CHAR-MODID6HEX-SUFFIX4TIMESTAMP
    pub fn generate(&self, sha256: &str, prefix: &str, suffix: &str) -> KodunikResult {
        let counter = COUNTER.fetch_add(1, Ordering::SeqCst);
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // SHA duhet të jetë 64 hex chars
        let sha_part = sha256
            .get(..8)
            .unwrap_or("00000000")
            .to_uppercase();

        let counter_hex = format!("{:06X}", counter % 0xFFFFFF);
        let ts_suffix = format!("{}", ts % 10000);
        let code = format!("{}:{}-{}-{}{}", prefix, sha_part, counter_hex, suffix, ts_suffix);
        let hmac_sig = generate_hmac(sha256, counter, ts, &code);

        let mut result = KodunikResult {
            code,
            sha256: sha256.to_string(),
            hmac_sig: hmac_sig[..12].to_string(),
            counter,
            timestamp: ts,
            valid: true,
        };

        // RUNTIME HARDENING (      ): regjistro për verifikim real (match mbi lock).
        match self.registry.lock() {
            Ok(mut reg) => {
                reg.insert(
                    result.code.clone(),
                    KodunikRecord {
                        sha256:    result.sha256.clone(),
                        hmac_sig:  result.hmac_sig.clone(),
                        counter:   result.counter,
                        timestamp: result.timestamp,
                    },
                );
            }
            Err(_) => {
                result.valid = false;
            }
        }

        self.generated.fetch_add(1, Ordering::Relaxed);
        result
    }

    /// Gjenerim autoritativ: SHA kanonik dhe registry fail-closed.
    pub fn generate_checked(
        &self,
        sha256: &str,
        suffix: &str,
    ) -> Result<KodunikResult, LgcAlgorithmError> {
        if !is_canonical_sha256(sha256) {
            return Err(LgcAlgorithmError::InvalidSha256);
        }
        if !is_clean_kodunik_token(suffix) {
            return Err(LgcAlgorithmError::InvalidToken("suffix"));
        }
        let counter = COUNTER.fetch_add(1, Ordering::SeqCst);
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let code = format!(
            "KDS:{}-{:06X}-{}{}",
            sha256[..8].to_uppercase(),
            counter % 0xFF_FFFF,
            suffix,
            timestamp % 10_000,
        );
        let full_hmac = generate_hmac(sha256, counter, timestamp, &code);
        if full_hmac.len() < KODUNIK_HMAC_LEN {
            return Err(LgcAlgorithmError::HmacFailure);
        }
        let result = KodunikResult {
            code,
            sha256: sha256.to_string(),
            hmac_sig: full_hmac[..KODUNIK_HMAC_LEN].to_string(),
            counter,
            timestamp,
            valid: true,
        };
        let mut registry = self
            .registry
            .lock()
            .map_err(|_| LgcAlgorithmError::RegistryUnavailable)?;
        registry.insert(
            result.code.clone(),
            KodunikRecord {
                sha256: result.sha256.clone(),
                hmac_sig: result.hmac_sig.clone(),
                counter: result.counter,
                timestamp: result.timestamp,
            },
        );
        drop(registry);
        self.generated.fetch_add(1, Ordering::Relaxed);
        Ok(result)
    }

    /// Verifikon KODUNIK
    /// true = autentik, false = i falsifikuar
    pub fn verify(&self, kodunik: &str, original_sha: &str) -> bool {
        match kodunik.is_empty() || original_sha.is_empty() {
            true => {
                self.verified_fail.fetch_add(1, Ordering::Relaxed);
                return false;
            }
            false => {}
        }

        // RUNTIME HARDENING (      , nga GPT runtime_patch): verifikim REAL nga
        // registry — sha256 i ruajtur + HMAC, jo vetëm përputhje prefiksi SHA.
        // Zero if — match i ndërthurur mbi Result/Option/barazi.
        let ok = match self.registry.lock() {
            Ok(registry) => match registry.get(kodunik) {
                Some(meta) => match meta.sha256 == original_sha {
                    false => false,
                    true => {
                        let expected = generate_hmac(original_sha, meta.counter, meta.timestamp, kodunik);
                        constant_time_eq(&expected[..12], &meta.hmac_sig)
                    }
                },
                None => false,
            },
            Err(_) => false,
        };

        match ok {
            true  => self.verified_ok.fetch_add(1, Ordering::Relaxed),
            false => self.verified_fail.fetch_add(1, Ordering::Relaxed),
        };
        ok
    }

    pub fn stats(&self) -> (u64, u64, u64) {
        (
            self.generated.load(Ordering::Relaxed),
            self.verified_ok.load(Ordering::Relaxed),
            self.verified_fail.load(Ordering::Relaxed),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LegacyLgcError {
    InvalidDirective,
    InvalidTrace,
    SplitInvariant(&'static str),
    Law0(&'static str),
    Kodunik(String),
    Evolve(String),
    Envelope(String),
    Evidence(&'static str),
    Receipt(&'static str),
}

impl std::fmt::Display for LegacyLgcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidDirective => write!(f, "LEGACY_LGC_INVALID_GCL_DIRECTIVE"),
            Self::InvalidTrace => write!(f, "LEGACY_LGC_INVALID_TRACE"),
            Self::SplitInvariant(name) => write!(f, "LEGACY_LGC_SPLIT_INVARIANT:{name}"),
            Self::Law0(name) => write!(f, "LEGACY_LGC_LAW0:{name}"),
            Self::Kodunik(reason) => write!(f, "LEGACY_LGC_KODUNIK:{reason}"),
            Self::Evolve(reason) => write!(f, "LEGACY_LGC_EVOLVE:{reason}"),
            Self::Envelope(reason) => write!(f, "LEGACY_LGC_ENVELOPE:{reason}"),
            Self::Evidence(reason) => write!(f, "LEGACY_LGC_EVIDENCE:{reason}"),
            Self::Receipt(reason) => write!(f, "LEGACY_LGC_RECEIPT:{reason}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LegacyLgcReceipt {
    pub version: u8,
    pub trace_id: String,
    pub trace_origin: u64,
    pub raw_len: usize,
    pub input_sha256: String,
    pub primitive_anchor: String,
    pub primitive_flags: u32,
    pub split_wire: String,
    pub xi_len: usize,
    pub yi_len: usize,
    pub xi_sha256: String,
    pub yi_sha256: String,
    pub split_sha256: String,
    pub law_before: u64,
    pub law_after: u64,
    pub law_seal: u64,
    pub system_laws_seal: u64,
    pub binding_sha256: String,
    pub kodunik: KodunikResult,
    pub envelope: String,
    pub envelope_integrity_sha256: String,
    pub receipt_sha256: String,
}

impl LegacyLgcReceipt {
    pub fn verify_integrity(&self) -> Result<(), LegacyLgcError> {
        if self.version != 2
            || self.raw_len == 0
            || !is_canonical_sha256(&self.input_sha256)
            || !is_canonical_sha256(&self.xi_sha256)
            || !is_canonical_sha256(&self.yi_sha256)
            || !is_canonical_sha256(&self.split_sha256)
            || !is_canonical_sha256(&self.binding_sha256)
            || !is_canonical_sha256(&self.envelope_integrity_sha256)
            || !is_canonical_sha256(&self.receipt_sha256)
        {
            return Err(LegacyLgcError::Receipt("canonical_fields"));
        }
        let (xi_wire, yi_wire) = self
            .split_wire
            .split_once('|')
            .ok_or(LegacyLgcError::Receipt("split_wire"))?;
        let split = crate::lab_contracts::PrimitiveSplit::from_wire(xi_wire, yi_wire)
            .ok_or(LegacyLgcError::Receipt("split_wire"))?;
        if split.xi.len() != self.xi_len || split.yi.len() != self.yi_len {
            return Err(LegacyLgcError::Receipt("split_lengths"));
        }
        if digest_terms("ESSMAI/LGC/XI/V2", &split.xi) != self.xi_sha256
            || digest_terms("ESSMAI/LGC/YI/V2", &split.yi) != self.yi_sha256
        {
            return Err(LegacyLgcError::Receipt("term_digests"));
        }
        let expected_split_sha = compute_domain_sha(
            "ESSMAI/LGC/SPLIT/V2",
            &[
                self.xi_sha256.as_bytes(),
                self.yi_sha256.as_bytes(),
                self.split_wire.as_bytes(),
            ],
        );
        if expected_split_sha != self.split_sha256 {
            return Err(LegacyLgcError::Receipt("split_digest"));
        }
        let term_count = self
            .xi_len
            .checked_add(self.yi_len)
            .ok_or(LegacyLgcError::Law0("term_count_overflow"))? as u64;
        let before = term_count
            .checked_mul(term_count)
            .ok_or(LegacyLgcError::Law0("before_overflow"))?;
        let after = (self.xi_len as u64)
            .checked_mul(self.yi_len as u64)
            .ok_or(LegacyLgcError::Law0("after_overflow"))?;
        if before != self.law_before
            || after != self.law_after
            || self.law_after > self.law_before
        {
            return Err(LegacyLgcError::Law0("receipt_mismatch"));
        }
        let envelope = crate::legalgocrypt::LgcValidator::validate_v2(&self.envelope)
            .map_err(|e| LegacyLgcError::Envelope(e.to_string()))?;
        let fields = &envelope.fields;
        if fields.trace_id != self.trace_id
            || fields.trace_origin != self.trace_origin
            || fields.input_sha256 != self.input_sha256
            || fields.primitive_anchor != self.primitive_anchor
            || fields.primitive_flags != self.primitive_flags
            || fields.xi_len != self.xi_len
            || fields.yi_len != self.yi_len
            || fields.xi_sha256 != self.xi_sha256
            || fields.yi_sha256 != self.yi_sha256
            || fields.split_sha256 != self.split_sha256
            || fields.law_before != self.law_before
            || fields.law_after != self.law_after
            || fields.law_seal != self.law_seal
            || fields.system_laws_seal != self.system_laws_seal
            || fields.binding_sha256 != self.binding_sha256
            || fields.kodunik != self.kodunik.code
            || fields.hmac_sig != self.kodunik.hmac_sig
            || fields.counter != self.kodunik.counter
            || fields.timestamp != self.kodunik.timestamp
            || envelope.integrity_sha256 != self.envelope_integrity_sha256
        {
            return Err(LegacyLgcError::Receipt("envelope_lineage"));
        }
        if !self.kodunik.verify(&self.binding_sha256) {
            return Err(LegacyLgcError::Receipt("kodunik_attestation"));
        }
        let raw_len = (self.raw_len as u64).to_le_bytes();
        let receipt_sha = compute_domain_sha(
            "ESSMAI/LGC/RECEIPT/V2",
            &[
                self.envelope.as_bytes(),
                self.split_wire.as_bytes(),
                &raw_len,
            ],
        );
        if receipt_sha != self.receipt_sha256 {
            return Err(LegacyLgcError::Receipt("receipt_digest"));
        }
        Ok(())
    }

    pub fn verify_runtime(&self) -> Result<(), LegacyLgcError> {
        self.verify_integrity()?;
        if !verify_kodunik(&self.kodunik.code, &self.binding_sha256) {
            return Err(LegacyLgcError::Receipt("kodunik_registry"));
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct LegacyCoordination {
    pub receipt: LegacyLgcReceipt,
    pub primitive_split: crate::lab_contracts::PrimitiveSplit,
    pub evolve: crate::evolve_trace::EvolveTrace,
    muscle: crate::alnur_karina_athar::AlnurKarinaAthar,
}

impl LegacyCoordination {
    pub fn is_collapsed(&self) -> bool {
        self.muscle.is_collapsed()
    }

    pub fn is_fulfilled(&self) -> bool {
        self.muscle.is_fulfilled()
    }

    pub fn complete_handoff(
        &mut self,
        evidence: &crate::alnur_karina_athar::HandoffEvidence,
    ) -> Result<(), LegacyLgcError> {
        self.receipt.verify_runtime()?;
        if evidence.binding_sha256 != self.receipt.binding_sha256
            || evidence.receipt_sha256 != self.receipt.receipt_sha256
            || !evidence.kodunik_verified
        {
            return Err(LegacyLgcError::Evidence("handoff_lineage_mismatch"));
        }
        self.muscle
            .hand_off(evidence)
            .map_err(|e| LegacyLgcError::Evidence(e.text))
    }

    pub fn ready_for_handoff(&self) -> Result<(), LegacyLgcError> {
        self.receipt.verify_runtime()?;
        match (self.muscle.is_collapsed(), self.muscle.is_fulfilled()) {
            (true, false) => Ok(()),
            (false, _) => Err(LegacyLgcError::Evidence("coordinate_collapse_incomplete")),
            (true, true) => Err(LegacyLgcError::Evidence("handoff_already_committed")),
        }
    }

    pub fn ready_for_dispatch(&self) -> Result<(), LegacyLgcError> {
        self.receipt.verify_runtime()?;
        if !self.muscle.is_fulfilled() {
            return Err(LegacyLgcError::Evidence("handoff_not_committed"));
        }
        Ok(())
    }
}

fn digest_terms(domain: &str, terms: &[String]) -> String {
    let mut canonical = Vec::new();
    for term in terms {
        canonical.extend_from_slice(&(term.len() as u64).to_le_bytes());
        canonical.extend_from_slice(term.as_bytes());
    }
    compute_domain_sha(domain, &[&canonical])
}

pub struct LegacyLgcAlgorithm;

impl LegacyLgcAlgorithm {
    pub fn coordinate_under_gcl(
        directive: crate::lab_contracts::SupremeDirective,
        raw_i0: &[u8],
        trace: &crate::trace_info::TraceInfo,
        query: &crate::lingua::SystemQuery,
    ) -> Result<LegacyCoordination, LegacyLgcError> {
        use crate::alnur_karina_athar::{
            AlnurKarinaAthar, CoordinateCollapseEvidence, PrimitiveAnchorEvidence,
            PrimitiveSplitEvidence, TraceEvidence, PRIMITIVE_SEAL_MASKED,
        };
        use crate::lab_contracts::{CollapsePhase, GjataCollapseLaw};
        use crate::legalgocrypt::{
            legacy_v2_binding_sha, LegacyV2Fields, LgcGenerator, LgcValidator,
        };

        if !GjataCollapseLaw::verify(&directive, CollapsePhase::Coordination) {
            return Err(LegacyLgcError::InvalidDirective);
        }
        let trace_matches_raw = trace.raw_len == raw_i0.len()
            && trace.id == (crate::trace_info::fnv64(raw_i0) ^ trace.ts_ns);
        if raw_i0.is_empty() || !trace.is_valid() || !trace_matches_raw {
            return Err(LegacyLgcError::InvalidTrace);
        }

        let input_sha256 = compute_sha256(raw_i0);
        let primitive_anchor = trace.id_hex();
        let mut evolve = crate::evolve_trace::EvolveTrace::from_legacy_anchor(
            trace,
            &input_sha256,
            &primitive_anchor,
        )
        .map_err(|e| LegacyLgcError::Evolve(e.to_string()))?;

        let mut muscle = AlnurKarinaAthar::commission(directive)
            .map_err(|e| LegacyLgcError::Evidence(e.text))?;
        muscle
            .record_trace(&TraceEvidence {
                trace_id: trace.label(),
                raw_len: raw_i0.len(),
                input_sha256: input_sha256.clone(),
            })
            .map_err(|e| LegacyLgcError::Evidence(e.text))?;
        muscle
            .record_anchor(&PrimitiveAnchorEvidence {
                trace_id: trace.label(),
                primitive_anchor: primitive_anchor.clone(),
                input_sha256: input_sha256.clone(),
                primitive_flags: PRIMITIVE_SEAL_MASKED,
            })
            .map_err(|e| LegacyLgcError::Evidence(e.text))?;

        // Production callgraph: coordinator -> LegacyLgcAlgorithm -> split_primitive.
        let primitive_split = crate::primitive_split::split_primitive(query);
        if !primitive_split.has_material() {
            return Err(LegacyLgcError::SplitInvariant("no_material"));
        }
        let disjoint = primitive_split
            .xi
            .iter()
            .all(|term| !primitive_split.yi.contains(term));
        if !disjoint {
            return Err(LegacyLgcError::SplitInvariant("xi_yi_not_disjoint"));
        }
        let mut actual_terms: Vec<String> = primitive_split
            .xi
            .iter()
            .chain(primitive_split.yi.iter())
            .cloned()
            .collect();
        let mut expected_terms = query.domain_terms.clone();
        actual_terms.sort();
        expected_terms.sort();
        let complete = actual_terms == expected_terms;
        if !complete {
            return Err(LegacyLgcError::SplitInvariant("not_complete"));
        }

        let split_wire = primitive_split.to_wire();
        let xi_sha256 = digest_terms("ESSMAI/LGC/XI/V2", &primitive_split.xi);
        let yi_sha256 = digest_terms("ESSMAI/LGC/YI/V2", &primitive_split.yi);
        let split_sha256 = compute_domain_sha(
            "ESSMAI/LGC/SPLIT/V2",
            &[
                xi_sha256.as_bytes(),
                yi_sha256.as_bytes(),
                split_wire.as_bytes(),
            ],
        );
        let term_count = primitive_split
            .xi
            .len()
            .checked_add(primitive_split.yi.len())
            .ok_or(LegacyLgcError::Law0("term_count_overflow"))? as u64;
        let law_before = term_count
            .checked_mul(term_count)
            .ok_or(LegacyLgcError::Law0("before_overflow"))?;
        let law_after = (primitive_split.xi.len() as u64)
            .checked_mul(primitive_split.yi.len() as u64)
            .ok_or(LegacyLgcError::Law0("after_overflow"))?;
        if law_after > law_before {
            return Err(LegacyLgcError::Law0("uncertainty_expanded"));
        }

        let client_code = format!("GjL_{primitive_anchor}/LGC");
        let contract_number = format!("Gjal{}J", trace.id);
        let legacy = LgcGenerator::generate(
            "lgc",
            "lhg",
            "lgc_algorithm",
            "legacy_runtime",
            0,
            1,
            Some(1),
            "alb",
            "tir",
            &["GCL", "LGC", "PA"],
            "0:0+",
            "000:001",
            &["essmai", "lgh", "qnt", "shd"],
            &client_code,
            &contract_number,
        );
        let legacy_v1_sha256 = LgcGenerator::legacy_v1_sha256(&legacy);
        let mut fields = LegacyV2Fields {
            version: 2,
            trace_id: trace.label(),
            trace_origin: trace.id,
            generation: 1,
            input_sha256: input_sha256.clone(),
            primitive_anchor: primitive_anchor.clone(),
            primitive_flags: PRIMITIVE_SEAL_MASKED,
            xi_len: primitive_split.xi.len(),
            yi_len: primitive_split.yi.len(),
            xi_sha256: xi_sha256.clone(),
            yi_sha256: yi_sha256.clone(),
            split_sha256: split_sha256.clone(),
            law_before,
            law_after,
            law_seal: directive.seal,
            system_laws_seal: crate::lab_contracts::system_laws_seal(),
            legacy_v1_sha256,
            binding_sha256: String::new(),
            kodunik: String::new(),
            hmac_sig: String::new(),
            counter: 0,
            timestamp: 0,
            contract_lineage: "light_coordinator>quantum_platform:route_input".to_string(),
            bridge_lineage: "LIGHT>QUANTUM>SHADOW".to_string(),
        };
        fields.binding_sha256 = legacy_v2_binding_sha(&fields);
        let kodunik = LgcAlgorithm::instance()
            .generate_checked(&fields.binding_sha256, "LGC")
            .map_err(|e| LegacyLgcError::Kodunik(e.to_string()))?;
        fields.kodunik = kodunik.code.clone();
        fields.hmac_sig = kodunik.hmac_sig.clone();
        fields.counter = kodunik.counter;
        fields.timestamp = kodunik.timestamp;

        evolve
            .branch_with_kodunik(crate::ess_mai_system::dsl::SFX_INPUT, &kodunik.code)
            .map_err(|e| LegacyLgcError::Evolve(e.to_string()))?;
        let envelope_struct = LgcGenerator::generate_v2(legacy, fields)
            .map_err(|e| LegacyLgcError::Envelope(e.to_string()))?;
        let envelope_integrity_sha256 = envelope_struct.integrity_sha256.clone();
        let envelope = LgcGenerator::encode_v2(&envelope_struct);
        LgcValidator::validate_v2(&envelope)
            .map_err(|e| LegacyLgcError::Envelope(e.to_string()))?;
        let raw_len_bytes = (raw_i0.len() as u64).to_le_bytes();
        let receipt_sha256 = compute_domain_sha(
            "ESSMAI/LGC/RECEIPT/V2",
            &[
                envelope.as_bytes(),
                split_wire.as_bytes(),
                &raw_len_bytes,
            ],
        );

        muscle
            .record_split(&PrimitiveSplitEvidence {
                trace_id: trace.label(),
                split_sha256: split_sha256.clone(),
                source_terms: query.domain_terms.len(),
                xi_len: primitive_split.xi.len(),
                yi_len: primitive_split.yi.len(),
                disjoint,
                complete,
                has_material: primitive_split.has_material(),
            })
            .map_err(|e| LegacyLgcError::Evidence(e.text))?;
        muscle
            .record_collapse(&CoordinateCollapseEvidence {
                split_sha256: split_sha256.clone(),
                binding_sha256: kodunik.sha256.clone(),
                before: law_before,
                after: law_after,
            })
            .map_err(|e| LegacyLgcError::Evidence(e.text))?;

        let receipt = LegacyLgcReceipt {
            version: 2,
            trace_id: trace.label(),
            trace_origin: trace.id,
            raw_len: raw_i0.len(),
            input_sha256,
            primitive_anchor,
            primitive_flags: PRIMITIVE_SEAL_MASKED,
            split_wire,
            xi_len: primitive_split.xi.len(),
            yi_len: primitive_split.yi.len(),
            xi_sha256,
            yi_sha256,
            split_sha256,
            law_before,
            law_after,
            law_seal: directive.seal,
            system_laws_seal: crate::lab_contracts::system_laws_seal(),
            binding_sha256: kodunik.sha256.clone(),
            kodunik,
            envelope,
            envelope_integrity_sha256,
            receipt_sha256,
        };
        receipt.verify_runtime()?;
        Ok(LegacyCoordination {
            receipt,
            primitive_split,
            evolve,
            muscle,
        })
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// SHORTCUT FUNCTIONS (API publike)
// ─────────────────────────────────────────────────────────────────────────────

/// Gjeneron KODUNIK — shortcut
pub fn generate_kodunik(sha256: &str, suffix: &str) -> String {
    LgcAlgorithm::instance()
        .generate(sha256, "KDS", suffix)
        .code
}

pub fn generate_kodunik_checked(
    sha256: &str,
    suffix: &str,
) -> Result<KodunikResult, LgcAlgorithmError> {
    LgcAlgorithm::instance().generate_checked(sha256, suffix)
}

/// Verifikon KODUNIK — shortcut
pub fn verify_kodunik(kodunik: &str, sha256: &str) -> bool {
    LgcAlgorithm::instance().verify(kodunik, sha256)
}


#[cfg(test)]
mod tests {
    use super::*;

    fn coordinate(text: &str) -> LegacyCoordination {
        let raw = text.as_bytes();
        let trace = crate::trace_info::TraceInfo::new(raw);
        let query = crate::lingua::LinguaCore::new().parse_input(text);
        LegacyLgcAlgorithm::coordinate_under_gcl(
            crate::lab_contracts::GjataCollapseLaw::issue(
                crate::lab_contracts::CollapsePhase::Coordination,
            ),
            raw,
            &trace,
            &query,
        )
        .expect("Legacy coordination must complete under a valid GCL directive")
    }

    /// Verifikimi runtime kërkon registry + HMAC mbi kodin e plotë.
    #[test]
    fn verify_kodunik_requires_registry_and_hmac() {
        let sha = compute_sha256(b"registry-test");
        let result = LgcAlgorithm::instance().generate(&sha, "KDS", "TST");
        // KODUNIK i sapogjeneruar → autentik (në registry, HMAC përkon).
        assert!(verify_kodunik(&result.code, &sha));
        // SHA i gabuar → refuzohet (sha256 != original_sha).
        assert!(!verify_kodunik(
            &result.code,
            "0000000000000000000000000000000000000000000000000000000000000000"
        ));
        let tampered = format!("{}X", result.code);
        assert!(!verify_kodunik_attestation(
            &tampered,
            &sha,
            &result.hmac_sig,
            result.counter,
            result.timestamp,
        ));
    }

    #[test]
    fn coordinate_under_gcl_binds_single_trace_split_and_envelope() {
        let coordination = coordinate("analizo motorin industrial të kuq");
        assert!(coordination.is_collapsed());
        assert!(!coordination.is_fulfilled());
        coordination
            .receipt
            .verify_runtime()
            .expect("receipt must be self-consistent and registry-authenticated");
        assert_eq!(
            coordination.receipt.split_wire,
            coordination.primitive_split.to_wire(),
        );
        assert_eq!(
            coordination.receipt.input_sha256,
            coordination.evolve.input_sha256,
        );
        assert_eq!(
            coordination.receipt.primitive_anchor,
            coordination.evolve.primitive_anchor,
        );
    }

    #[test]
    fn receipt_raw_length_is_cryptographically_bound() {
        let coordination = coordinate("krijo raportin final të laboratorit");
        let mut tampered = coordination.receipt.clone();
        tampered.raw_len = tampered.raw_len.saturating_add(1);
        assert!(tampered.verify_integrity().is_err());
    }

    #[test]
    fn wrong_gcl_phase_cannot_run_legacy_coordination() {
        let text = "analizo sensorin termik";
        let raw = text.as_bytes();
        let trace = crate::trace_info::TraceInfo::new(raw);
        let query = crate::lingua::LinguaCore::new().parse_input(text);
        let result = LegacyLgcAlgorithm::coordinate_under_gcl(
            crate::lab_contracts::GjataCollapseLaw::issue(
                crate::lab_contracts::CollapsePhase::Reasoning,
            ),
            raw,
            &trace,
            &query,
        );
        assert!(matches!(result, Err(LegacyLgcError::InvalidDirective)));
    }

}
