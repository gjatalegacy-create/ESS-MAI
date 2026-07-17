// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  PROGRESSIVE_DEBATIC / types.rs                                          ║
// ║  GJATA LEGACY™ — ESS-MAI Quantum                                         ║
// ║                                                                          ║
// ║  ROLI: Kontratat e tipeve për modulin PD (cognitive excavation layer).  ║
// ║                                                                          ║
// ║  ÇFARË ËSHTË PD:                                                         ║
// ║    — NUK është debater/argument engine.                                 ║
// ║    — Është shtresë gërmimi kognitiv: gjen "gjëgjëzën" e fshehur në       ║
// ║      mendimin e shfaqur, gërmon trace-in, dhe — kur lind një GeniusSignal║
// ║      — dorëzon material tek PIM.                                         ║
// ║    — Kundërshton VETËM kur mungon baza reale (pressure budget).          ║
// ║                                                                          ║
// ║  Përshtatur në arkitekturën sovrane: ZERO if/else (match/branchless),    ║
// ║  pa varësi të jashtme, id-të me FNV-1a (hash-i real i sistemit).         ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::pd_spine_contract::{PdSpineCompletion, PdSpineRequest};

// Seal-i kanonik i modulit PD. Burimi i vetëm është sovereign::laws,
// që PD të mos mbajë një derivim paralel jashtë regjistrit qendror.
pub const PD_SEAL: u64 = crate::sovereign::laws::SEAL_PD;
/// Quantum nuk e zotëron këtë seal; vetëm njeh vlerën kanonike të autoritetit
/// final të Shadow për të verifikuar dëftesën e output-it.
pub const EXPECTED_SHADOW_VERIFICATION_SEAL: u64 =
    crate::sovereign::laws::module_seal(b"SHADOW_VERIFICATION_OUTPUT");

/// id 12-shenjësh heksadecimal nga FNV-1a (deterministik, hash-i real i sistemit).
pub fn pd_id(parts: &[&[u8]]) -> String {
    // Përziej pjesët në mënyrë branchless (fold mbi FNV).
    let h = parts.iter().fold(0xcbf29ce484222325u64, |acc, p| {
        crate::hw_real::ffi::fnv1a_64(*p) ^ acc.wrapping_mul(1099511628211)
    });
    format!("{:012x}", h & 0xFFFF_FFFF_FFFF)
}

// ─────────────────────────────────────────────────────────────────────────────
// DebateMode — 4 mënyrat e angazhimit
// ─────────────────────────────────────────────────────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebateMode {
    Intellect,   // nevoja praktike — shtyn lehtë
    Philosophy,  // intuitë pa strukturë — PD ndërton scaffold
    Scientific,  // koncepti lidhet me matje — kërkon kauzalitet/invariantë
    Novel,       // zona e frontierit — kundërshtim shumë i butë
}

impl DebateMode {
    pub fn label(self) -> &'static str {
        match self {
            DebateMode::Intellect  => "INTELLECT",
            DebateMode::Philosophy => "PHILOSOPHY",
            DebateMode::Scientific => "SCIENTIFIC",
            DebateMode::Novel      => "NOVEL",
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// CognitiveSignal — çdo mesazh i shfaqur, i procesuar gjatë ingest
// ─────────────────────────────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct CognitiveSignal {
    pub signal_id:      String,
    pub turn_index:     u32,
    pub raw_content:    String,
    pub content_length: u32,
    pub idea_density:   u32,      // fixed mass [0..10_000]
    pub energy_level:   u32,      // fixed mass [0..10_000]
    pub concept_refs:   Vec<String>,
    pub contradicts:    Vec<String>,
    pub returns_to:     Vec<String>,
    pub timestamp:      u64,
}

// ─────────────────────────────────────────────────────────────────────────────
// EpistemicTrace — gjurma e akumuluar gjatë sesionit
// ─────────────────────────────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct EpistemicTrace {
    pub trace_id:             String,
    pub session_id:           String,
    pub signals:              Vec<CognitiveSignal>,
    pub total_turns:          u32,
    pub dominant_concept:     Option<String>,
    pub concept_map:          std::collections::HashMap<String, u32>,
    pub energy_trajectory:    Vec<u32>,
    pub idea_evolution:       Vec<u32>,
    pub accumulated_mass:     u32,
    pub structural_coherence: u32,
    pub implicit_primitives:  Vec<String>,
    pub contradiction_pairs:  Vec<(String, String)>,
    pub territory:            String,
    pub started_at:           u64,
    pub last_updated_at:      u64,
}

impl EpistemicTrace {
    pub fn new(session_id: &str, territory: &str, started_at: u64) -> Self {
        EpistemicTrace {
            trace_id: pd_id(&[session_id.as_bytes(), b"trace"]),
            session_id: session_id.to_string(),
            signals: Vec::new(),
            total_turns: 0,
            dominant_concept: None,
            concept_map: std::collections::HashMap::new(),
            energy_trajectory: Vec::new(),
            idea_evolution: Vec::new(),
            accumulated_mass: 0,
            structural_coherence: 0,
            implicit_primitives: Vec::new(),
            contradiction_pairs: Vec::new(),
            territory: territory.to_string(),
            started_at,
            last_updated_at: started_at,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// GeniusSignal — momenti kur sistemi "ndjen" diçka reale (3 sinjale)
// ─────────────────────────────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct GeniusSignal {
    pub signal_id:             String,
    pub session_id:            String,
    pub returns_to_idea:       bool,
    pub energy_increasing:     bool,
    pub concept_gaining_shape: bool,
    pub returns_score:         u32,
    pub energy_score:          u32,
    pub shape_score:           u32,
    pub genius_score:          u32,
    pub detected_concept:      String,
    pub detected_at_turn:      u32,
    pub trace_ref:             String,
    pub timestamp:             u64,
}

// ─────────────────────────────────────────────────────────────────────────────
// PressureBudget — sa mund të shtyjë PD para se të tërhiqet
// ─────────────────────────────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct PressureBudget {
    pub challenges_issued:        u32,
    pub current_mode:             DebateMode,
    pub budget_exhausted:         bool,
    pub user_resistance_detected: bool,
}

impl PressureBudget {
    // Limitet absolute sipas mode-it (nga pseudo).
    pub const MAX_INTELLECT:  u32 = 2;
    pub const MAX_PHILOSOPHY: u32 = 3;
    pub const MAX_SCIENTIFIC: u32 = 4;
    pub const MAX_NOVEL:      u32 = 2;

    pub fn new(mode: DebateMode) -> Self {
        PressureBudget {
            challenges_issued: 0,
            current_mode: mode,
            budget_exhausted: false,
            user_resistance_detected: false,
        }
    }

    /// max_for — limiti i kundërshtimeve për mode-in aktual. Zero if — match.
    pub fn max_for(mode: DebateMode) -> u32 {
        match mode {
            DebateMode::Intellect  => Self::MAX_INTELLECT,
            DebateMode::Philosophy => Self::MAX_PHILOSOPHY,
            DebateMode::Scientific => Self::MAX_SCIENTIFIC,
            DebateMode::Novel      => Self::MAX_NOVEL,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// PdResponseType — çfarë vendos PD të bëjë në çdo kthesë
// ─────────────────────────────────────────────────────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PdResponseType {
    Help,
    Orient,
    Clarify,
    ChallengeGentle,
    ChallengeWithFacts,
    ScaffoldBullet,
    ScaffoldMeasure,
    ScaffoldStructure,
    Listen,
}

impl PdResponseType {
    pub fn label(self) -> &'static str {
        match self {
            PdResponseType::Help               => "HELP",
            PdResponseType::Orient             => "ORIENT",
            PdResponseType::Clarify            => "CLARIFY",
            PdResponseType::ChallengeGentle    => "CHALLENGE_GENTLE",
            PdResponseType::ChallengeWithFacts => "CHALLENGE_WITH_FACTS",
            PdResponseType::ScaffoldBullet     => "SCAFFOLD_BULLET",
            PdResponseType::ScaffoldMeasure    => "SCAFFOLD_MEASURE",
            PdResponseType::ScaffoldStructure  => "SCAFFOLD_STRUCTURE",
            PdResponseType::Listen             => "LISTEN",
        }
    }

    /// is_challenge — a është kundërshtim (konsumon buxhet). Zero if — match.
    pub fn is_challenge(self) -> bool {
        match self {
            PdResponseType::ChallengeGentle | PdResponseType::ChallengeWithFacts => true,
            _ => false,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// PdTurn — çdo kthesë (sinjali + vendimi i PD)
// ─────────────────────────────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct PdTurn {
    pub turn_id:          String,
    pub turn_index:       u32,
    pub user_signal:      CognitiveSignal,
    pub pd_response_type: PdResponseType,
    pub pd_response_hint: String,
    pub challenge_issued: bool,
    pub mode_at_turn:     DebateMode,
    pub genius_check:     Option<u32>,
}

// ─────────────────────────────────────────────────────────────────────────────
// PdEngineOutput — output i motorit pas çdo kthese
// ─────────────────────────────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub enum PdEngineOutput {
    Continue        { turn: PdTurn },
    GeniusDetected  { genius_signal: GeniusSignal, package: PdCognitivePackage },
    BudgetExhausted { trace_id: String },
    SessionClosed   { session_id: String, reason: String },
}

// ─────────────────────────────────────────────────────────────────────────────
// PD DY-FAZËSH — parapërgatitje → verifikim final → i₀ i radhës
// ─────────────────────────────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct PdPreSealReceipt {
    pub seal:               u64,
    pub session_id:         String,
    pub parent_i0:          String,
    pub receipt_id:         String,
    pub prepared_at:        u64,
    pub spine_activation_id: u64,
    pub spine_completion_digest: u64,
    pub layer_mask:         u8,
    pub mpro_mass:          u32,
    pub action_sha256:      [u8; 32],
    pub law_trace_sha256:   [u8; 32],
}

#[derive(Debug, Clone)]
pub struct GclActionAuthorizationToken {
    pub contract_version: u32,
    pub action_sha256: [u8; 32],
    pub law_trace_sha256: [u8; 32],
    pub law_mask: u32,
    pub verdict: u8,
    pub issued_at: u64,
}

#[derive(Debug, Clone)]
pub struct PdAuthorizedCompletion {
    pub completion: PdSpineCompletion,
    pub pre_seal: PdPreSealReceipt,
    pub gcl_token: GclActionAuthorizationToken,
}

#[derive(Debug, Clone)]
pub struct PdPendingNextI0 {
    pub session_id:        String,
    pub parent_i0:         String,
    pub turn_id:           String,
    pub prepared_question: String,
    pub response_type:     PdResponseType,
    pub prepared_at:       u64,
    pub spine_completion:  PdSpineCompletion,
    pub pre_seal:          PdPreSealReceipt,
}

impl PdPendingNextI0 {
    /// Identiteti i kandidatit që Shadow duhet të lidhë me receipt-in final.
    /// Ndryshimi i pyetjes, turn-it, pre-seal-it ose completion-it ndryshon digest-in.
    pub fn binding_digest(&self) -> u64 {
        crate::pd_spine_contract::digest_parts(&[
            self.session_id.as_bytes(),
            self.parent_i0.as_bytes(),
            self.turn_id.as_bytes(),
            self.prepared_question.as_bytes(),
            self.response_type.label().as_bytes(),
            &self.prepared_at.to_le_bytes(),
            self.pre_seal.receipt_id.as_bytes(),
            &self.pre_seal.spine_activation_id.to_le_bytes(),
            &self.pre_seal.spine_completion_digest.to_le_bytes(),
            &self.pre_seal.mpro_mass.to_le_bytes(),
            &self.pre_seal.action_sha256,
            &self.pre_seal.law_trace_sha256,
            &self.spine_completion.request.continuum.activation_digest.to_le_bytes(),
            &self.spine_completion.request.continuum.question_increment.increment_digest.to_le_bytes(),
            b"PD_CANDIDATE_BINDING_V156",
        ])
    }
}

/// Cikli i parë i PD: inputi është përpunuar, por kandidati NUK është ende
/// PendingNextI0. PD duhet të aktivizojë Spine 9 dhe të marrë mbylljen e të
/// tre shtresave përpara se të prodhojë pre-seal.
#[derive(Debug, Clone)]
pub struct PdSpineCycle {
    pub output:  PdEngineOutput,
    pub request: PdSpineRequest,
}

/// Cikli i dytë: vetëm pas PdSpineCompletion krijohen PendingNextI0/pre-seal.
#[derive(Debug, Clone)]
pub struct PdPreparedCycle {
    pub output:       PdEngineOutput,
    pub pending_next: Option<PdPendingNextI0>,
    pub pre_seal:     Option<PdPreSealReceipt>,
    pub spine_completion: PdSpineCompletion,
    pub gcl_token: Option<GclActionAuthorizationToken>,
}

/// Dëftesa e përfundimit të output-it aktual. Vjen nga Shadow pasi XY është
/// verifikuar dhe seal-i i autoritetit është hapur me LgcToken brenda Shadow.
/// Token-i nuk transportohet; transportohet vetëm dëftesa e vulosur.
#[derive(Debug, Clone)]
pub struct PdVerificationCompletion {
    pub session_id:                   String,
    pub parent_i0:                    String,
    pub primitive_anchor:             u64,
    pub xy_digest:                    u64,
    pub pd_binding_digest:            u64,
    pub pd_continuum_activation_digest: u64,
    pub y_verdict:                    u8,
    pub x_verdict:                    u8,
    pub verification_seal:            u64,
    pub verification_generation:      u64,
    pub verification_receipt_id:      String,
    pub living_trust_sha256:           [u8; 32],
    pub living_trust_intensity:        u32,
    pub living_trust_kind:             u8,
    pub living_trust_sovereign_value:  u32,
    pub living_trust_digest:           u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PdContinuationBasis {
    VerifiedPositive,
    VerifiedNegativeRebuild,
}

impl PdContinuationBasis {
    pub fn label(self) -> &'static str {
        match self {
            Self::VerifiedPositive        => "VERIFIED_POSITIVE",
            Self::VerifiedNegativeRebuild => "VERIFIED_NEGATIVE_REBUILD",
        }
    }
}

#[derive(Debug, Clone)]
pub struct PdNextI0 {
    pub session_id:              String,
    pub parent_i0:               String,
    pub next_i0_id:              String,
    pub question:                String,
    pub basis:                   PdContinuationBasis,
    pub y_verdict:               u8,
    pub x_verdict:               u8,
    pub verification_seal:       u64,
    pub verification_generation: u64,
    pub verification_receipt_id: String,
    pub primitive_anchor:        u64,
    pub xy_digest:               u64,
    pub pd_binding_digest:       u64,
    pub continuum_activation_digest: u64,
    pub question_increment_digest: u64,
    pub pd_output_id:            String,
    pub pd_output_digest:        u64,
    pub iz_id:                    String,
    pub iz_digest:                u64,
    pub iz_sha256:                [u8; 32],
    pub living_trust_sha256:      [u8; 32],
    pub living_trust_intensity:   u32,
    pub living_trust_kind:        u8,
    pub living_trust_sovereign_value: u32,
    pub living_trust_digest:      u64,
    pub continuum_completion_digest: u64,
}

#[derive(Debug, Clone)]
pub struct PdVerifiedOutput {
    pub output_id: String,
    pub output_digest: u64,
    pub session_id: String,
    pub parent_i0: String,
    pub turn_id: String,
    pub question: String,
    pub response_type: PdResponseType,
    pub mpro_mass: u32,
    pub y_verdict: u8,
    pub x_verdict: u8,
    pub verification_receipt_id: String,
}

#[derive(Debug, Clone)]
pub struct PdIzCompletion {
    pub iz_id: String,
    pub iz_digest: u64,
    pub iz_sha256: [u8; 32],
    pub origin_i0: String,
    pub question_increment_digest: u64,
    pub pd_output_digest: u64,
    pub verification_receipt_id: String,
    pub basis: PdContinuationBasis,
    pub living_trust_sha256: [u8; 32],
    pub living_trust_intensity: u32,
    pub living_trust_kind: u8,
    pub living_trust_sovereign_value: u32,
    pub living_trust_digest: u64,
}

#[derive(Debug, Clone)]
pub struct PdContinuumClosure {
    pub output: PdVerifiedOutput,
    pub iz: PdIzCompletion,
    pub next_i0: PdNextI0,
    pub contract: crate::pd_continuum_contract::PdContinuumCompletion,
}

#[derive(Debug, Clone)]
pub enum PdFinalization {
    Released(PdContinuumClosure),
    RebuiltFromNegative(PdContinuumClosure),
    Rejected { reason: String },
}

// ─────────────────────────────────────────────────────────────────────────────
// PdPreClassification — sugjerimi i PD për PIM (jo fact_candidate)
// ─────────────────────────────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct PdPreClassification {
    pub suggested_category: String,  // "Hypothesis" | "PotentialHypothesis"
    pub confidence_hint:    u32,     // fixed mass [3_000..7_000]
    pub reasoning:          String,
}

// ─────────────────────────────────────────────────────────────────────────────
// PdCognitivePackage — material final për PIM
// ─────────────────────────────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct PdCognitivePackage {
    pub package_id:        String,
    pub session_id:        String,
    pub genius_signal:     GeniusSignal,
    pub pre_classification: PdPreClassification,
    pub accumulated_text:  String,   // konkatenim i sinjaleve kryesore
    pub dominant_concept:  String,
    pub concept_territory: String,
    pub accumulated_mass:  u32,
    pub structural_coherence: u32,
    pub accumulated_turns: u32,
    pub estimated_trl:     u8,        // TRL0/1/2 — vlerësim i PD
    pub origin:            String,    // "PROGRESSIVE_DEBATIC"
    pub created_at:        u64,
}

// ─────────────────────────────────────────────────────────────────────────────
// PdError
// ─────────────────────────────────────────────────────────────────────────────
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PdError {
    SealInvalid(String),
    SessionNotFound(String),
    BroadcastMissing,
    TraceIngestFailed(String),
    BridgeFailed(String),
    BudgetViolation,
}

impl std::fmt::Display for PdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PdError::SealInvalid(m)       => write!(f, "PD seal invalid: {m}"),
            PdError::SessionNotFound(m)   => write!(f, "PD session not found: {m}"),
            PdError::BroadcastMissing     => write!(f, "PD broadcast missing"),
            PdError::TraceIngestFailed(m) => write!(f, "PD trace ingest failed: {m}"),
            PdError::BridgeFailed(m)      => write!(f, "PD bridge failed: {m}"),
            PdError::BudgetViolation      => write!(f, "PD budget violation"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seal_is_deterministic_nonzero() {
        assert_ne!(PD_SEAL, 0);
        // I njëjti llogaritje → e njëjta vlerë (compile-time const).
        let recomputed = crate::sovereign::laws::module_seal(b"PROGRESSIVE_DEBATIC");
        assert_eq!(PD_SEAL, recomputed);
    }

    #[test]
    fn pd_id_is_deterministic_and_12_hex() {
        let a = pd_id(&[b"sess", b"trace"]);
        let b = pd_id(&[b"sess", b"trace"]);
        assert_eq!(a, b);
        assert_eq!(a.len(), 12);
        assert!(a.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn budget_max_matches_mode() {
        assert_eq!(PressureBudget::max_for(DebateMode::Intellect),  2);
        assert_eq!(PressureBudget::max_for(DebateMode::Philosophy), 3);
        assert_eq!(PressureBudget::max_for(DebateMode::Scientific), 4);
        assert_eq!(PressureBudget::max_for(DebateMode::Novel),      2);
    }

    #[test]
    fn response_challenge_classification() {
        assert!(PdResponseType::ChallengeGentle.is_challenge());
        assert!(PdResponseType::ChallengeWithFacts.is_challenge());
        assert!(!PdResponseType::Help.is_challenge());
        assert!(!PdResponseType::Listen.is_challenge());
    }
}
