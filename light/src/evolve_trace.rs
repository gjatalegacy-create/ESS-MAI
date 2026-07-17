// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  EVOLVE_TRACE.RS — ESS-MAI LIGHT PLATFORM                             ║
// ║  Gjata Legacy™  |  Arkitekt: Bledar Gjata                             ║
// ║                                                                          ║
// ║  Evolucioni i gjurmës gjatë pipeline.                                  ║
// ║  I njëjti algoritëm LGC — por tani me fraksinim:                      ║
// ║    x1234 → x1234_hiskng_1 / x1234_geokng_1 / x1234_mathkng_1         ║
// ║                                                                          ║
// ║  LIDHJET:                                                               ║
// ║    trace_info    → jep TraceInfo (baza e origjinit)                    ║
// ║    lgc_algorithm → generate_kodunik() (vulosin e degës)               ║
// ║    legalgocrypt  → Trace{base,generated,transform} (evolucioni)        ║
// ║    ess_mai_system::dsl → SFX_* (emrat e degëve)                       ║
// ║                                                                          ║
// ║  Roli: TRACER — ndjek evoluimin, kurrë nuk vendos.                   ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::lgc_algorithm::{
    compute_domain_sha, compute_sha256, generate_kodunik_checked, is_canonical_sha256,
};
use crate::legalgocrypt::Trace;
use crate::trace_info::TraceInfo;
use crate::ess_mai_system::dsl;
use crate::ess_mai_system::limits::MAX_TRACE_BRANCHES;

// ════════════════════════════════════════════════════════════════════════════
// §1  KODUNIK-SEED — bazë deterministe 64-hex për vulën KODUNIK të degës.
//     SHËNIM LIGJOR (gjata_collapse_law): ky NUK është hash-i i trace-it dhe
//     NUK "pret Shadow". Hash-i real i trace-it është `input_sha256`, i
//     llogaritur MENJËHERË në Light (§1b). Shadow VETËM verifikon e vulos.
//     generate_kodunik() kërkon str me ≥8 chars hex
//     Prodhojmë 64-char hex nga trace ID — identifikues lokal i degës
// ════════════════════════════════════════════════════════════════════════════

// ════════════════════════════════════════════════════════════════════════════
// §2  TRACE BRANCH — një degë e vetme e evolucionit
// ════════════════════════════════════════════════════════════════════════════

// ─────────────────────────────────────────────────────────────────────────────
// §1b  SHA-256 REAL NË LIGHT — Light kap dhe gjurmon; hash-i lind KËTU.
// ─────────────────────────────────────────────────────────────────────────────
/// Gjendja e trace-it në ligjin gjata_collapse_law:
/// Light e mban PENDING; VETËM vula e Shadow e kalon në Verified/Rejected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraceState {
    Pending,
    Verified,
    Rejected,
}

impl TraceState {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Pending  => "PENDING",
            Self::Verified => "VERIFIED",
            Self::Rejected => "REJECTED",
        }
    }
}

#[derive(Debug, Clone)]
pub struct TraceBranch {
    /// ID e trace prind (TraceInfo.id)
    pub parent_id:    u64,

    /// Kodi i degës: "HISKNG", "GEOKNG", "MATHKNG", ...
    pub branch_tag:   String,

    /// KODUNIK seal i kësaj dege — KDS:SHA8-CTR6-TAG
    pub kodunik:      String,

    /// Trace struct: { base, generated, transform }
    pub trace:        Trace,

    /// Indeksi i degës brenda prindërit (1, 2, 3, ...)
    pub branch_index: u32,

    /// Koherenca e degës [0,1] — sa mbetet e lidhur me primitivin.
    /// Drift midis degëve llogaritet nga këto vlera.
    /// Vjen nga Quantum (paketimi i propozimit); deri atëherë: 1.0.
    pub coherence: f32,
}

impl TraceBranch {
    /// Label human-readable: x1234_hiskng_1
    pub fn label(&self) -> String {
        format!(
            "x{:04x}_{}_{}", 
            self.parent_id & 0xFFFF,
            self.branch_tag.to_lowercase(),
            self.branch_index
        )
    }

    /// A është dega valid (ka KODUNIK)?
    pub fn is_valid(&self) -> bool {
        self.kodunik.starts_with("KDS:")
    }
}

// ════════════════════════════════════════════════════════════════════════════
// §3  EVOLVE TRACE — kontejneri kryesor i evolucionit
// ════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub struct EvolveTrace {
    /// TraceInfo.id — origjina e paprekur
    pub origin_id:    u64,

    /// TraceInfo.label() — T3_a3f4b2c1
    pub origin_label: String,

    /// Primitive Anchor autoritative (TraceInfo.id_hex), pa identitet paralel.
    pub primitive_anchor: String,

    /// Të gjitha degët e prodhuara
    pub branches:     Vec<TraceBranch>,

    /// Gjenerimi aktual — rrit me çdo branch()
    pub generation:   u64,

    /// Gjendja DSL e pipeline-it
    pub state:        dsl::AlgorithmState,

    /// VULA 500 (e maskuar me XOR) — fillon SEALED, humbet me drift > prag.
    /// 0xA451 = (500 & 0xFFFF) ^ 0xA5A5  → shadow_gj_legacy e njeh.
    /// 0x0000 = u fragmentua → jo primitiv.
    pub primitive_flags: u32,

    // ══ gjata_collapse_law — gjurma e hash-uar dhe vula e verifikimit ══════
    /// SHA-256 REAL i inputit — llogaritet MENJËHERË në Light (from_trace).
    pub input_sha256:  String,
    /// SHA-256 REAL i outputit — vendoset kur përgjigja formohet (set_output).
    pub output_sha256: Option<String>,
    /// Vula e Shadow (0xA451 kur pranohet) — vjen VETËM nga verifikimi final.
    pub shadow_seal:   Option<u32>,
    /// Pending → (Verified | Rejected) — kalon VETËM me vulën e Shadow.
    pub trace_state:   TraceState,
}

// Vula e primitivit në formë transporti (XOR-maskuar) — identike me C kernel.
// shadow_gj_legacy.c: (flags & 0xFFFF) ^ 0xA5A5 == 500
const SEAL_PRIMITIVE_MASKED: u32 = (500u32 & 0x0000_FFFF) ^ 0xA5A5;  // = 0xA451
const SEAL_STRIPPED:         u32 = 0x0000_0000;

// Prag i fragmentimit: drift mbi këtë → vula humbet.
const DRIFT_THRESHOLD: f32 = 0.15;

impl EvolveTrace {
    /// Krijon EvolveTrace nga TraceInfo
    /// Thirrur nga light_coordinator sapo trace_info ka stampuar inputin
    pub fn from_trace(info: &TraceInfo) -> Self {
        // Rrugë pa raw-bytes (urë e vulës / teste): hash-i mbetet bosh —
        // "s'u dha input" është e vërtetë e deklaruar, kurrë hash i rremë.
        Self {
            origin_id:    info.id,
            origin_label: info.label(),
            primitive_anchor: info.id_hex(),
            branches:     Vec::new(),
            generation:   0,
            state:        dsl::AlgorithmState::Evolving,
            // Fillon i vulosur — primitiv i fortë deri sa të provohet ndryshe.
            primitive_flags: SEAL_PRIMITIVE_MASKED,
            input_sha256:  String::new(),
            output_sha256: None,
            shadow_seal:   None,
            trace_state:   TraceState::Pending,
        }
    }

    /// RRUGA REALE (gjata_collapse_law): hash-i i trace-it lind NË LIGHT, TANI.
    /// Thirrur nga light_coordinator me bytes e papërpunuara të inputit.
    pub fn from_trace_with_input(info: &TraceInfo, raw_input: &[u8]) -> Self {
        let mut t = Self::from_trace(info);
        t.input_sha256 = compute_sha256(raw_input);
        t
    }

    /// Rruga LGC: pranon SHA-në e vetme autoritative dhe nuk e rillogarit.
    pub fn from_legacy_anchor(
        info: &TraceInfo,
        authoritative_sha256: &str,
        primitive_anchor: &str,
    ) -> Result<Self, EvolveError> {
        if !is_canonical_sha256(authoritative_sha256)
            || primitive_anchor != info.id_hex()
        {
            return Err(EvolveError::InvalidAnchor);
        }
        let mut trace = Self::from_trace(info);
        trace.input_sha256 = authoritative_sha256.to_string();
        trace.primitive_anchor = primitive_anchor.to_string();
        Ok(trace)
    }

    /// Vendos SHA-256 REAL të outputit sapo përgjigja formohet në Light.
    /// Gjendja mbetet Pending — vetëm Shadow e kalon më tej.
    pub fn set_output(&mut self, output: &[u8]) {
        self.output_sha256 = Some(compute_sha256(output));
    }

    /// Vula finale e Shadow: verdict_bit ∈ {0,1} + vula (0xA451 kur pranohet).
    /// Shadow VETËM verifikon e vulos — kurrë s'llogarit hash-in e trace-it.
    pub fn apply_shadow_verdict(&mut self, verdict_bit: u8, seal: u32) {
        self.shadow_seal = Some(seal);
        self.trace_state = match verdict_bit {
            1 => TraceState::Verified,
            _ => TraceState::Rejected,
        };
    }

    /// Shton një degë të re me suffix të dhënë
    ///
    /// Thirrur nga light_coordinator kur inputi ndahet në knowledge types.
    /// Callback: generate_kodunik() nga lgc_algorithm, Trace nga legalgocrypt.
    ///
    /// Kthehet: TraceBranch me KODUNIK seal
    pub fn branch(&mut self, suffix_tag: &str) -> Result<TraceBranch, EvolveError> {
        let next_generation = self.generation.wrapping_add(1).to_le_bytes();
        let origin = self.origin_id.to_le_bytes();
        let branch_sha = compute_domain_sha(
            "ESSMAI/EVOLVE/BRANCH/V2",
            &[
                self.input_sha256.as_bytes(),
                self.primitive_anchor.as_bytes(),
                &origin,
                &next_generation,
                suffix_tag.as_bytes(),
            ],
        );
        let kodunik = generate_kodunik_checked(&branch_sha, suffix_tag)
            .map_err(|e| EvolveError::KodunikFailure(e.to_string()))?;
        self.branch_with_kodunik(suffix_tag, &kodunik.code)
    }

    /// Shton degën autoritative me KODUNIK-un e prodhuar nga receipt-i LGC.
    pub fn branch_with_kodunik(
        &mut self,
        suffix_tag: &str,
        kodunik: &str,
    ) -> Result<TraceBranch, EvolveError> {
        if self.branches.len() >= MAX_TRACE_BRANCHES {
            return Err(EvolveError::MaxBranchesReached(MAX_TRACE_BRANCHES));
        }
        if !dsl::is_valid_suffix(suffix_tag) {
            return Err(EvolveError::InvalidSuffix(suffix_tag.to_string()));
        }
        if !kodunik.starts_with("KDS:") {
            return Err(EvolveError::KodunikFailure(
                "KODUNIK pa prefiksin KDS".to_string(),
            ));
        }

        self.generation = self.generation.wrapping_add(1);
        let idx = self.branches.len() as u32 + 1;
        let branch = TraceBranch {
            parent_id: self.origin_id,
            branch_tag: suffix_tag.to_string(),
            kodunik: kodunik.to_string(),
            trace: Trace {
                base: self.origin_id & 0x00FF_FFFF,
                generated: self.generation,
                transform: Some(idx),
            },
            branch_index: idx,
            coherence: 1.0,
        };
        self.branches.push(branch.clone());
        self.recompute_seal();
        Ok(branch)
    }

    /// Si branch(), por me koherencë eksplicite nga Quantum.
    /// Pas shtimit, ri-llogarit vulën: nëse drift midis degëve > prag → STRIPPED.
    pub fn branch_scored(
        &mut self,
        suffix_tag: &str,
        coherence:  f32,
    ) -> Result<TraceBranch, EvolveError> {
        let mut b = self.branch(suffix_tag)?;
        let coh = coherence.clamp(0.0, 1.0);
        // branch() sapo shtoi një degë → last element ekziston gjithmonë.
        // Përdorim slice indexing mbi gjatësinë, pa branch vendimi.
        let last = self.branches.len() - 1;
        self.branches[last].coherence = coh;
        b.coherence = coh;
        self.recompute_seal();
        Ok(b)
    }

    /// VULA 500 — rregulli i mbijetesës, BRANCHLESS (zero if/else).
    ///
    /// drift = max|coherence_i − coherence_0|  mbi të gjitha degët.
    /// is_sealed = (drift <= 0.15) → 1, ndryshe 0
    /// primitive_flags = is_sealed * 0xA451  +  (1−is_sealed) * 0x0000
    ///
    /// Interpretimi: derisa degët mbeten brenda pragut të koherencës,
    /// inputi është ende një PRIMITIV i fortë → vula mbahet. Kur divergjenca
    /// kalon pragun (fragmentim në gjendje/knowledge të ndara), vula shkëputet
    /// → shadow_gj_legacy do shpallë 0 (i verifikueshëm, jo primitive_knowledge).
    ///
    /// Ruajmë seal_strength si metrikë e vazhdueshme (audit/Quantum), por
    /// vendimi binar i vulës varet drejtpërdrejt nga pragu — pa zonë gri.
    fn recompute_seal(&mut self) {
        let drift = self.max_drift();

        // Branchless: (drift <= prag) si u32 ∈ {0,1}.
        // Strip i menjëhershëm sapo drift e kalon pragun — pa 0.99 të lirshme.
        let is_sealed = (drift <= DRIFT_THRESHOLD) as u32;

        self.primitive_flags = is_sealed * SEAL_PRIMITIVE_MASKED
            + (1 - is_sealed) * SEAL_STRIPPED;
    }

    /// Forca e vulës si metrikë e vazhdueshme [0,1] — për audit & Quantum.
    /// Nuk vendos vetë; recompute_seal() vendos mbi pragun.
    pub fn seal_strength(&self) -> f32 {
        let drift = self.max_drift();
        let drift_factor = (drift - DRIFT_THRESHOLD).max(0.0) / (1.0 - DRIFT_THRESHOLD);
        (1.0 - drift_factor).clamp(0.0, 1.0)
    }

    /// Drift maksimal i koherencës midis degës 0 dhe çdo dege tjetër.
    /// 0 ose 1 degë → drift 0 (s'ka fragmentim të mundshëm).
    fn max_drift(&self) -> f32 {
        // pa branch: nëse < 2 degë, iteratori është bosh → fold jep 0.0
        let base = self.branches.first().map(|b| b.coherence).unwrap_or(1.0);
        self.branches.iter()
            .skip(1)
            .map(|b| (b.coherence - base).abs())
            .fold(0.0f32, f32::max)
    }

    /// Vula aktuale (e maskuar) — për bus_legacy / lgc_bridge.
    /// 0xA451 = primitiv i fortë, 0x0000 = i fragmentuar.
    pub fn primitive_flags(&self) -> u32 {
        self.primitive_flags
    }

    /// A mbijetoi primitivi? (true = vula intakte)
    pub fn is_primitive_sealed(&self) -> bool {
        self.primitive_flags == SEAL_PRIMITIVE_MASKED
    }

    /// Numri i degëve aktuale
    pub fn branch_count(&self) -> usize {
        self.branches.len()
    }

    /// Rezyme e evolucionit: T3_a3f4b2c1→[x1234_hiskng_1,x1234_geokng_1]
    pub fn summary(&self) -> String {
        match self.branches.is_empty() {
            true => return format!("{}→[]", self.origin_label),
            false => {}
        }
        let labels: Vec<String> = self.branches.iter().map(|b| b.label()).collect();
        format!("{}→[{}]", self.origin_label, labels.join(","))
    }

    /// Kthen degën sipas suffix tag
    pub fn find_branch(&self, tag: &str) -> Option<&TraceBranch> {
        self.branches.iter().find(|b| b.branch_tag == tag)
    }

    /// Vendos gjendjen e re DSL — me validim tranzicioni
    pub fn set_state(&mut self, next: dsl::AlgorithmState) -> Result<(), EvolveError> {
        match self.state.can_transition_to(&next) {
            true => {
            self.state = next;
            Ok(())
            }
            false => {
            Err(EvolveError::InvalidStateTransition {
                from: self.state.label().to_string(),
                to: next.label().to_string(),
            })
            }
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// §4  GABIME
// ════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, PartialEq)]
pub enum EvolveError {
    MaxBranchesReached(usize),
    InvalidSuffix(String),
    InvalidAnchor,
    KodunikFailure(String),
    InvalidStateTransition { from: String, to: String },
}

impl std::fmt::Display for EvolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MaxBranchesReached(n) =>
                write!(f, "EVOLVE_ERR: max branches reached ({})", n),
            Self::InvalidSuffix(s) =>
                write!(f, "EVOLVE_ERR: invalid suffix '{}'", s),
            Self::InvalidAnchor =>
                write!(f, "EVOLVE_ERR: invalid primitive anchor or SHA-256"),
            Self::KodunikFailure(reason) =>
                write!(f, "EVOLVE_ERR: KODUNIK failure: {}", reason),
            Self::InvalidStateTransition { from, to } =>
                write!(f, "EVOLVE_ERR: invalid transition {}→{}", from, to),
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// TESTE — gjata_collapse_law: hash REAL në Light, Shadow vetëm vulos
// ════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod law_hash_tests {
    use super::*;
    use crate::trace_info::TraceInfo;

    #[test]
    fn input_sha256_born_in_light_immediately() {
        let raw = b"flamur kuq zi shqiponje";
        let t = EvolveTrace::from_trace_with_input(&TraceInfo::new(raw), raw);
        // SHA-256("flamur kuq zi shqiponje") — vektor i llogaritur, 64 hex.
        assert_eq!(t.input_sha256.len(), 64);
        assert!(t.input_sha256.chars().all(|c| c.is_ascii_hexdigit()));
        assert_eq!(t.trace_state, TraceState::Pending);
        assert_eq!(t.output_sha256, None);
        assert_eq!(t.shadow_seal, None);
    }

    #[test]
    fn same_input_same_hash_deterministic() {
        let raw = b"determinizem";
        let a = EvolveTrace::from_trace_with_input(&TraceInfo::new(raw), raw);
        let b = EvolveTrace::from_trace_with_input(&TraceInfo::new(raw), raw);
        assert_eq!(a.input_sha256, b.input_sha256);
    }

    #[test]
    fn output_set_then_shadow_seals_verified() {
        let raw = b"input";
        let mut t = EvolveTrace::from_trace_with_input(&TraceInfo::new(raw), raw);
        t.set_output(b"pergjigja e verifikuar");
        assert!(t.output_sha256.is_some());
        assert_eq!(t.trace_state, TraceState::Pending); // ende — Shadow s'ka folur
        t.apply_shadow_verdict(1, 0xA451);
        assert_eq!(t.trace_state, TraceState::Verified);
        assert_eq!(t.shadow_seal, Some(0xA451));
    }

    #[test]
    fn shadow_zero_bit_rejects() {
        let raw = b"x";
        let mut t = EvolveTrace::from_trace_with_input(&TraceInfo::new(raw), raw);
        t.apply_shadow_verdict(0, 0x0000);
        assert_eq!(t.trace_state, TraceState::Rejected);
    }

    #[test]
    fn legacy_from_trace_declares_no_hash_honestly() {
        // Rruga pa raw (urë e vulës/teste): hash bosh = e vërtetë e deklaruar.
        let raw = b"y";
        let t = EvolveTrace::from_trace(&TraceInfo::new(raw));
        assert!(t.input_sha256.is_empty());
        assert_eq!(t.trace_state, TraceState::Pending);
    }
}
