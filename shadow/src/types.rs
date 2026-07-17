// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  TYPES.RS — ESS-MAI SHADOW PLATFORM                              ║
// ║  Gjata Legacy™  |  Arkitekt: Bledar Gjata                             ║
// ║                                                                          ║
// ║  Tipet kryesore të Shadow — LINJA B (kanonike).                       ║
// ║                                                                          ║
// ║  — DY ORIGJINA, TË NDARA:                                        ║
// ║    • PassPackage   → origjina QUANTUM (propozim/arsyetim)             ║
// ║    • LightEnvelope → origjina LIGHT  (transport + vulë 500)           ║
// ║    Ato MBETEN STRUCTE TË NDARA. Bashkohen VETËM brenda ShadowPassage  ║
// ║    (ndërtuar nga shadow_pipeline.rs). Asnjëra nuk futet te tjetra.    ║
// ║                                                                          ║
// ║  LIGJI 0 — ZERO-COPY SOVEREIGN:                                       ║
// ║    Shadow është memoria sovrane. Verifikuesit NUK klonojnë inputin.   ║
// ║    pkg/light ZHVENDOSEN (move) në ShadowPassage; payload-i ZHVENDOSET  ║
// ║    në vault në kufirin e vetëm të materializimit. Leximet vetëm me &. ║
// ║                                                                          ║
// ║  ZERO if/else në rrugët e verdiktit. ZERO ML.                        ║
// ╚══════════════════════════════════════════════════════════════════════════╝

// ════════════════════════════════════════════════════════════════════════════
// §1  SHADOW PATH — 3 rrugët sekuenciale
// ════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShadowPath {
    Fast,      /* S.Router → S.Matrix → S.Judiciary → shadow_gj_legacy     */
    Standard,  /* + S.Gen5 → S.Type → S.Sovereign                          */
    Deep,      /* + S.Temporal → S.Emergence → S.Consensus                 */
}

impl ShadowPath {
    /// Kodim u8 për WAL/trace (ZERO if/else — indeks i enum-it).
    #[inline]
    pub fn as_u8(self) -> u8 { self as u8 }

    /// Dekodim nga u8 (saturim te Deep për vlera të panjohura).
    #[inline]
    pub fn from_u8(v: u8) -> Self {
        // Tabelë e prerë; pa degëzim semantik mbi vlerën.
        const TABLE: [ShadowPath; 3] = [ShadowPath::Fast, ShadowPath::Standard, ShadowPath::Deep];
        TABLE[(v as usize).min(2)]
    }
}

/// GJURMË PRIMITIVE — historiku i një inputi sa herë konfirmohet në një domain.
/// Gjurmueshmëria mbart historikun: ku/si/kur u gjet dhe me çfarë mase + vulë.
/// Pesë domain-e të ndryshme me vulë konsistente → kristalizim Legacy.
#[derive(Debug, Clone, PartialEq)]
pub struct PrimitiveTrace {
    pub domain:     String,     // territory/domain ku u përforcua
    pub timestamp:  u64,        // kur (ns)
    pub mass:       f32,        // me çfarë mase epistemike
    pub path_taken: ShadowPath, // si (Fast/Standard/Deep)
    pub lgc_seal:   u32,        // vula 500 (0xA451 çdo herë)
}

impl ShadowPath {
    pub fn label(&self) -> &'static str {
        match self {
            ShadowPath::Fast     => "FAST",
            ShadowPath::Standard => "STANDARD",
            ShadowPath::Deep     => "DEEP",
        }
    }

    /// Numri minimal i nodave që DUHET të ekzekutohen për këtë rrugë
    /// (përfshirë Router dhe Judiciary). Përdoret nga enforce_sovereign_laws.
    pub fn min_nodes(&self) -> usize {
        match self {
            ShadowPath::Fast     => 4, /* Router, Matrix, MultiVerification, Judiciary */
            ShadowPath::Standard => 7, /* + Gen5, Type, Sovereign                      */
            ShadowPath::Deep     => 10, /* + Temporal, Emergence, Consensus             */
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// §2  SHADOW NODE — 9 noda sekuenciale
// ════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShadowNode {
    Router,      /* S.Router     — zgjidh rrugën                           */
    Matrix,      /* S.Matrix     — sistematizon reasoning                   */
    Gen5,        /* S.Gen5       — verifikon strukturën vektoriale          */
    Type,        /* S.Type       — verifikon gjendjen e paketimit           */
    Temporal,    /* S.Temporal   — verifikon validitetin kohor (deep only)  */
    Sovereign,   /* S.Sovereign  — 5 ligjet sovrane (score)                 */
    Emergence,   /* S.Emergence  — tranzicioni (deep only)                  */
    Consensus,   /* S.Consensus  — konsensusi (deep only)                   */
    NightWatch,  /* S.NightWatch — sentinel i jashtëm (deep only, P5.2)     */
    Judiciary,   /* S.Judiciary  — gjykata epistemike                       */
    // Shtohet PAS Judiciary që discriminant-et historike 0..9 të mos lëvizin.
    MultiVerification, /* gjendja e shumë-verifikimit L2/L3, para Judiciary */
}

impl ShadowNode {
    pub fn label(&self) -> &'static str {
        match self {
            ShadowNode::Router    => "S.ROUTER",
            ShadowNode::Matrix    => "S.MATRIX",
            ShadowNode::Gen5      => "S.GEN5",
            ShadowNode::Type      => "S.TYPE",
            ShadowNode::Temporal  => "S.TEMPORAL",
            ShadowNode::Sovereign => "S.SOVEREIGN",
            ShadowNode::Emergence => "S.EMERGENCE",
            ShadowNode::Consensus => "S.CONSENSUS",
            ShadowNode::NightWatch => "S.NIGHTWATCH",
            ShadowNode::Judiciary => "S.JUDICIARY",
            ShadowNode::MultiVerification => "S.MULTI_VERIFY",
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// §3  SHADOW VERDICT — vendimi i çdo nodi (Copy — zero alokim shtesë)
// ════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy)]
pub struct ShadowVerdict {
    pub node:        ShadowNode,
    pub score:       f32,
    pub verdict:     u8,
    pub reason_code: u8,
    pub ts_ns:       u64,
}

impl ShadowVerdict {
    /// Krijon verdict nga score — ZERO if/else.
    /// score >= 0.5 → 1, score < 0.5 → 0.
    pub fn from_score(node: ShadowNode, score: f32, reason_code: u8, ts_ns: u64) -> Self {
        let score_c = score.clamp(0.0, 1.0);
        let verdict = ((score_c * 2.0 - 1.0).max(0.0).min(1.0).round()) as u8;
        Self { node, score: score_c, verdict, reason_code, ts_ns }
    }

    pub fn passed(&self) -> bool { self.verdict == 1 }
}

// ════════════════════════════════════════════════════════════════════════════
// §4  KNOWLEDGE BAND — banda e dijes
// ════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum KnowledgeBand {
    Negative    = 0,  /* [0.00 → 0.70] */
    Hypothesis  = 1,  /* [0.71 → 0.99] */
    Fact        = 2,  /* [1.00]         */
    Scientific  = 3,  /* dija ekzistente */
}

impl KnowledgeBand {
    /// Seleksion vektorial nga epistemic_mass — ZERO if/else.
    pub fn from_mass(mass: f32) -> Self {
        let m = mass.clamp(0.0, 1.0);

        let neg_w  = (0.71 - m).max(0.0) / 0.71;
        let hyp_w  = (m - 0.71).max(0.0) * (1.0 - m).max(0.0) * 14.64;
        let fact_w = (m - 0.99).max(0.0) * 100.0;

        let bands = [neg_w, hyp_w, fact_w, 0.0f32];
        let idx = bands
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
            .unwrap_or(0);

        match idx {
            1 => KnowledgeBand::Hypothesis,
            2 => KnowledgeBand::Fact,
            3 => KnowledgeBand::Scientific,
            _ => KnowledgeBand::Negative,
        }
    }

    pub fn as_u8(self) -> u8 { self as u8 }

    pub fn label(self) -> &'static str {
        match self {
            KnowledgeBand::Negative   => "neg",
            KnowledgeBand::Hypothesis => "hyp",
            KnowledgeBand::Fact       => "fact",
            KnowledgeBand::Scientific => "sci",
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// §5  LIGHT ENVELOPE — origjina LIGHT (transport + vulë 500). STRUCT I NDARË.
// ════════════════════════════════════════════════════════════════════════════

/// Envelope-i që vjen nga Light (`shadow_seal_bridge`). Light NUK vendos —
/// llogarit vulën dhe e TRANSPORTON. VETËM shadow_gj_legacy e interpreton
/// `primitive_flags` si vulën 500.
///
/// Vula udhëton e maskuar: `(500 & 0xFFFF) ^ 0xA5A5 = 0xA451`.
///
/// LIGJI 0: zhvendoset (move) në ShadowPassage; nuk klonohet.
#[derive(Debug, Clone, Default)]
pub struct LightEnvelope {
    pub session_id:      String,
    pub territory:       String,
    pub primitive_flags: u32,        /* 0xA451 (e maskuar) ose 0x0000 (e hequr) */
    pub proof_chain:     Vec<u64>,
    pub payload:         Vec<u8>,
    /// P5.3: feed sentineli nga Light (None nëse Light s'e ushqeu). Default None.
    pub device_watch:    Option<crate::lab_contracts_v11::watch::NightWatchReport>,
}

impl LightEnvelope {
    pub const SEAL_PRIMITIVE_MASKED: u32 = 0xA451;
    pub const SEAL_STRIPPED: u32 = 0x0000;

    pub fn sealed(session_id: &str, territory: &str, proof_chain: Vec<u64>, payload: Vec<u8>) -> Self {
        Self {
            session_id: session_id.to_string(),
            territory:  territory.to_string(),
            primitive_flags: Self::SEAL_PRIMITIVE_MASKED,
            proof_chain,
            payload,
            device_watch: None,
        }
    }

    /// P5.3: device_watch_report — feed sentineli (None nëse Light s'e ushqeu). Zero if.
    pub fn device_watch_report(&self)
        -> Option<crate::lab_contracts_v11::watch::NightWatchReport>
    {
        self.device_watch.clone()
    }

    pub fn stripped(session_id: &str, territory: &str, proof_chain: Vec<u64>, payload: Vec<u8>) -> Self {
        Self {
            session_id: session_id.to_string(),
            territory:  territory.to_string(),
            primitive_flags: Self::SEAL_STRIPPED,
            proof_chain,
            payload,
            device_watch: None,
        }
    }

    pub fn is_sealed(&self) -> bool {
        self.primitive_flags == Self::SEAL_PRIMITIVE_MASKED
    }
}

// ════════════════════════════════════════════════════════════════════════════
// §6  PASS PACKAGE — origjina QUANTUM (propozim). STRUCT I NDARË nga Light.
// ════════════════════════════════════════════════════════════════════════════


/// Materiali shkencor i projektit të përdoruesit që ka kaluar Light/APUPK,
/// Shadow context witness dhe Quantum Digital Lab nën të njëjtin GCL process.
/// Nuk përmban handle të magazinës; vetëm identitete/prova të kufizuara.
#[derive(Debug, Clone)]
pub struct ScientificProjectContext {
    pub project_id: u64,
    pub user_id: u64,
    pub trace_id: u64,
    pub revision: u32,
    pub content_sha256: String,
    pub light_sovereign_flags: u32,
    pub context_sha256: String,
    pub project_title: String,
    pub domain: String,
    pub hypothesis: String,
    pub assumptions: Vec<String>,
    pub gcl_process_digest: u64,
    pub trl_level: u8,
    pub trl_passed: bool,
    pub trl_confidence: u32,
    pub trl_reproducibility: u32,
    pub lab_test_id: String,
    pub findings: String,
    pub description: String,
    pub files: Vec<(u8, Vec<u8>)>,
    pub evidence_sha256: String,
}

/// PassPackage — propozimi që Quantum dërgon (`quantum_shadow_bridge`).
/// Struktura është IDENTIKE me `ShadowPassPackage` të urës Quantum.
///
/// Origjina LIGHT (vula/transport) udhëton VEÇMAS si `LightEnvelope` dhe
/// bashkohet me këtë vetëm brenda `ShadowPassage` në pipeline.
#[derive(Debug, Clone, Default)]
pub struct PassPackage {
    pub package_id:           String,

    /// Epistemic mass nga LIM [0.0-1.0]
    pub epistemic_mass:       f32,

    /// 5 dimensionet e potencialitetit (nga PIM)
    pub evidence_density:     f32,
    pub logical_coherence:    f32,
    pub causal_integrity:     f32,
    pub convergence_strength: f32,
    pub reproducibility:      f32,

    pub potentiality_score:   f32,
    pub ibe_score:            f32,
    pub chain_coherence:      f32,
    pub conservation_ok:      bool,

    pub candidate_scores:     Vec<f32>,
    pub suggested_verdict:    u8,

    /// Gjurmë e paketës finale PIM/NPIM/MPRO. Matrix e sheh si kontekst;
    /// verifikimi matematik është kryer fail-closed te Shadow main.rs.
    pub final_evidence_digest: u64,
    /// Gjendja e konverguar e aksioneve Quantum; zero hash gjatë ciklit.
    pub quantum_action_state: u64,
    pub quantum_action_mask: u64,
    pub quantum_required_action_mask: u64,
    pub pd_gcl_process_digest: u64,
    pub spine_completion_digest: u64,

    /// Projekti shkencor opsional; gjykohet vetëm brenda judge_supreme.
    pub scientific_project: Option<ScientificProjectContext>,

    /// Raw bytes (materializohen në KnowledgeVault me ZHVENDOSJE, jo klon).
    pub raw_bytes:            Vec<u8>,
}

impl PassPackage {
    /// Knowledge band nga epistemic_mass — zero if/else.
    pub fn knowledge_band(&self) -> KnowledgeBand {
        KnowledgeBand::from_mass(self.epistemic_mass)
    }

    /// Reasoning strength — vektor i peshuar.
    pub fn reasoning_strength(&self) -> f32 {
        (self.ibe_score        * 0.35
       + self.causal_integrity * 0.35
       + self.conservation_ok as u8 as f32 * 0.30)
        .clamp(0.0, 1.0)
    }
}

// ════════════════════════════════════════════════════════════════════════════
// §7  SHADOW PASSAGE — BASHKIMI i dy origjinave (ndërtohet në pipeline)
// ════════════════════════════════════════════════════════════════════════════

/// ShadowPassage — bartësi i brendshëm që BASHKON Quantum (package) + Light
/// (light) dhe udhëton sekuencialisht nëpër çdo nod.
///
/// LIGJI 0 (zero-copy): `package` dhe `light` ZHVENDOSEN këtu (move) — nuk
/// klonohen kurrë. Nodet lexojnë `&passage.package` (= `&PassPackage`).
/// `verdicts` është gjendje pune (≤9 elemente Copy), jo klon i inputit.
///
/// Qëllimisht NUK implementon Clone — bashkimi nuk dyfishohet.
#[derive(Debug)]
pub struct ShadowPassage {
    /// Input identifier
    pub input_id:        u64,

    /// Vula 500 — kopjuar nga light (u32 Copy, jo klon heap).
    /// Mbartur verbërisht; VETËM shadow_gj_legacy e shikon si 500.
    pub primitive_flags: u32,

    /// Origjina QUANTUM (e zhvendosur brenda)
    pub package:         PassPackage,

    /// Origjina LIGHT (e zhvendosur brenda)
    pub light:           LightEnvelope,

    /// Rruga e zgjedhur nga S.Router
    pub path:            ShadowPath,

    /// Verdiktet e akumuluara — index 0 = i pari
    pub verdicts:        Vec<ShadowVerdict>,
    /// Gjendja e fundit e shumë-verifikimit (jo verdict publik).
    pub verification_state: crate::lab_contracts::verdict::VerificationState,
    /// Sinjali i precedentit të sistematizuar në Shadow.
    pub verification_precedent: f32,
}

impl ShadowPassage {
    /// Bashkon dy origjinat me ZHVENDOSJE (move). Vula merret nga `light`.
    pub fn new(input_id: u64, package: PassPackage, light: LightEnvelope, path: ShadowPath) -> Self {
        let flags = light.primitive_flags;
        Self {
            input_id,
            primitive_flags: flags,
            package,
            light,
            path,
            verdicts: Vec::with_capacity(11),
            verification_state: crate::lab_contracts::verdict::VerificationState::Unverified,
            verification_precedent: 0.0,
        }
    }

    pub fn push_verdict(&mut self, verdict: ShadowVerdict) {
        self.verdicts.push(verdict);
    }

    /// push_watch_signal — shton sinjalin e NightWatch si verdict (P5.2). Zero if.
    /// bit==0 → presion negativ mbi Consensus (reason_code=watch).
    pub fn push_watch_signal(&mut self, bit: u8, confidence: f32) {
        self.verdicts.push(ShadowVerdict {
            node:        ShadowNode::NightWatch,
            score:       confidence,
            verdict:     bit,
            reason_code: 0x57,  // 'W' — watch
            ts_ns:       0,
        });
    }

    /// Regjistron rezultatin e shumë-verifikimit si portë e detyrueshme.
    pub fn push_multi_verification(
        &mut self,
        state: crate::lab_contracts::verdict::VerificationState,
        precedent: f32,
        score: f32,
        bit: u8,
        reason_code: u8,
        ts_ns: u64,
    ) {
        self.verification_state = state;
        self.verification_precedent = precedent.clamp(0.0, 1.0);
        self.verdicts.push(ShadowVerdict {
            node: ShadowNode::MultiVerification,
            score: score.clamp(0.0, 1.0),
            verdict: bit.min(1),
            reason_code,
            ts_ns,
        });
    }

    /// Porta e shumë-verifikimit; mungesa është fail-closed.
    pub fn multi_verification_verdict(&self) -> u8 {
        self.verdicts.iter().rev()
            .find(|v| v.node == ShadowNode::MultiVerification)
            .map(|v| v.verdict)
            .unwrap_or(0)
    }

    pub fn has_multi_verification(&self) -> bool {
        self.verdicts.iter().any(|v| v.node == ShadowNode::MultiVerification)
    }

    pub fn last_verdict(&self) -> Option<&ShadowVerdict> {
        self.verdicts.last()
    }

    /// Verdikti final nga S.Judiciary (0 nëse mungon)
    pub fn judiciary_verdict(&self) -> u8 {
        self.verdicts
            .iter()
            .rev()
            .find(|v| v.node == ShadowNode::Judiciary)
            .map(|v| v.verdict)
            .unwrap_or(0)
    }

    /// A ekziston një verdikt Judiciary? (enforce_sovereign_laws — L3)
    pub fn has_judiciary(&self) -> bool {
        self.verdicts.iter().any(|v| v.node == ShadowNode::Judiciary)
    }

    /// Score mesatar i të gjitha verdikteve — sinjal koherence
    pub fn aggregate_score(&self) -> f32 {
        match self.verdicts.is_empty() { true => return 0.0, false => {} }
        let sum: f32 = self.verdicts.iter().map(|v| v.score).sum();
        (sum / self.verdicts.len() as f32).clamp(0.0, 1.0)
    }

    /// A ka kaluar çdo nod? — shumëzim verdiktesh (zero if/else)
    pub fn all_passed(&self) -> bool {
        let product: u32 = self.verdicts
            .iter()
            .map(|v| v.verdict as u32)
            .product();
        product == 1
    }
}

// ════════════════════════════════════════════════════════════════════════════
// §8  NEGATIVE CONTEXT — dija negative me kontekst të plotë shkencor
// ════════════════════════════════════════════════════════════════════════════

/// Konteksti i një dije negative — shkak, proces, masë, frekuencë.
/// Shadow e trajton Negative Knowledge si KUFI aktiv, jo si dështim.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct NegativeContext {
    pub failure_reason_code: u32,
    pub process:             String,
    pub mass_at_fail:        f32,
    pub frequency:           u32,
}

// ════════════════════════════════════════════════════════════════════════════
// §9  SUPREME VERDICT — vendimi final i shadow_gj_legacy.lgc
// ════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub struct SupremeVerdict {
    /// 0/1 — inputi i verifikuar (judiciary; ligjet sovrane kaluan)
    pub verified:        u8,
    /// 0/1 — inputi PRIMITIVE (verified AND 500 seal)
    pub primitive:       u8,
    pub knowledge_band:  KnowledgeBand,
    /// LGC LAW bitmask — gjendja e dijes
    pub lgc_law:         u32,
    pub legacy_score:    f32,
    pub lgc_dsl:         String,
    /// Vula e gjallë lind vetëm në këtë verdikt; nuk persiston si state.
    pub living_trust_sha256: [u8; 32],
    pub living_trust_intensity: u32,
    pub living_trust_kind: u8,
    pub living_trust_action_state: u64,
    pub living_trust_action_mask: u64,
    pub living_trust_required_action_mask: u64,
    pub living_trust_system_laws_seal: u64,
    pub living_trust_sovereign_flags: u32,
    pub living_trust_sovereign_value: u32,
    /// Rezultati i projektit brenda të njëjtit verdikt suprem GCL.
    pub project_id: u64,
    pub project_status: u8,
    pub project_context_digest: u64,
    pub project_evidence_digest: u64,
    /// Identitetet e plota; u64 sipër mbeten vetëm indekse legacy.
    pub project_context_sha256: [u8; 32],
    pub project_evidence_sha256: [u8; 32],
    pub novel_factualized: bool,
    pub novel_trl_level: u8,
    pub novel_proof_score: u32,
    pub novel_rejection: u8,
}

impl SupremeVerdict {
    pub fn is_primitive(&self) -> bool { self.primitive == 1 }
    pub fn is_verified(&self)  -> bool { self.verified  == 1 }
    pub fn on_legacy_path(&self) -> bool {
        (self.lgc_law & 0x04) != 0 && self.legacy_score > 0.50
    }
}

// ════════════════════════════════════════════════════════════════════════════
// §10  SHADOW ERROR
// ════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, PartialEq)]
pub enum ShadowError {
    SealInvalid(String),
    BussError(i32),
    SerializationError(String),
    VaultWriteError(String),
    InvalidPath(String),
    Frozen(String),
    /// Shkelje e një ligji sovran (enforce_sovereign_laws) — refuzim i fortë.
    SovereignViolation(String),
    /// Transport PA/wire i KORRUPTUAR (format, hex, Xi/Yi wire) — shkelje
    /// KONTRATE transporti. HARDENING: korrupsioni s'kapërcehet kurrë si
    /// "asnjë ankorë" — fail-closed, sesioni refuzohet para ingest.
    TransportCorrupt(String),
}

impl std::fmt::Display for ShadowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SealInvalid(m)         => write!(f, "SHADOW_ERR: Seal invalid — {}", m),
            Self::BussError(c)           => write!(f, "SHADOW_ERR: Bus error code {}", c),
            Self::SerializationError(m)  => write!(f, "SHADOW_ERR: Serialization — {}", m),
            Self::VaultWriteError(m)     => write!(f, "SHADOW_ERR: Vault write — {}", m),
            Self::InvalidPath(m)         => write!(f, "SHADOW_ERR: Invalid path — {}", m),
            Self::Frozen(m)              => write!(f, "SHADOW_ERR: System frozen — {}", m),
            Self::SovereignViolation(m)  => write!(f, "SHADOW_ERR: Sovereign law violated — {}", m),
            Self::TransportCorrupt(m)    => write!(f, "SHADOW_ERR: Transport corrupt — {}", m),
        }
    }
}

impl std::error::Error for ShadowError {}
