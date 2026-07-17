// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LIM_TYPES.RS — Strukturat bazë të Linear Info Metric                 ║
// ║  GJATA LEGACY™ — Quantum Platform                                     ║
// ║                                                                          ║
// ║  Besnik ndaj lim_core.pseudo: 8 akset, EpistemicState, OperatorGate.   ║
// ╚══════════════════════════════════════════════════════════════════════════╝

// ─────────────────────────────────────────────────────────────────────────────
// LINEAR INFO METRIC — 8 akset epistemike
// Nga: STRUCT LinearInfoMetric (lim_core.pseudo)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct LinearInfoMetric {
    // 8 akset — të gjithë f32 [0.0, 1.0]
    pub information_density:       f32,  // sa informacion i ri ekziston
    pub continuity_score:          f32,  // a lidhen fragmentet
    pub contradiction_presence:    f32,  // sa konflikt ka (invertohet)
    pub semantic_integrity:        f32,  // strukturë logjike
    pub hypothesis_support_score:  f32,  // bazë për reasoning serioz
    pub contextual_linearity:      f32,  // lineariteti i kontekstit
    pub exploration_readiness:     f32,  // sa thellë lejohet Quantum
    pub reasoning_depth_allowance: f32,  // kufi i thellësisë PRO

    /// Agregat i peshuar i 8 akseve.
    pub epistemic_mass: f32,

    /// Gjendja epistemike (faza 2; None në faza 1).
    pub epistemic_state: Option<EpistemicState>,

    // Metadata
    pub session_id:  String,
    pub territory:   String,
    pub computed_at: u64,
    pub phase:       LimPhase,
}

/// Faza e LIM-it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LimPhase {
    PrePro,       // Faza 1 — para PRO
    PostQuantum,  // Faza 2 — pas Quantum
}

// ─────────────────────────────────────────────────────────────────────────────
// EPISTEMIC STATE — klasifikimi nga epistemic_mass
// Nga: ENUM EpistemicState (lim_core.pseudo)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum EpistemicState {
    /// 30-35%: TRL0 reasoning possibility — pak strukturë.
    Hypothesis {
        confidence_range: (f32, f32),
        reason:           String,
        pro_mode:         String,
    },
    /// 36-98%: stabël për eksplorim më të thellë.
    PotentialHypothesis {
        confidence_range: (f32, f32),
        stability:        f32,
        trl_candidate:    u8,
        pro_mode:         String,
    },
    /// 99-100%: kandidat për validim eksperimental (JO fakt absolut).
    FactCandidate {
        confidence_range: (f32, f32),
        validation_path:  String,
        pro_mode:         String,
    },
}

impl EpistemicState {
    /// Emri i shkurtër i gjendjes — për probe/callback.
    pub fn kind_str(&self) -> &'static str {
        match self {
            Self::Hypothesis { .. }          => "HYPOTHESIS",
            Self::PotentialHypothesis { .. } => "POTENTIAL_HYPOTHESIS",
            Self::FactCandidate { .. }       => "FACT_CANDIDATE",
        }
    }

    /// pro_mode-i që i përket gjendjes — udhëzon PRO-n.
    pub fn pro_mode(&self) -> &str {
        match self {
            Self::Hypothesis { pro_mode, .. }          => pro_mode,
            Self::PotentialHypothesis { pro_mode, .. } => pro_mode,
            Self::FactCandidate { pro_mode, .. }       => pro_mode,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// OPERATOR GATE — çfarë operatori PRO aktivizohet
// Nga: STRUCT OperatorGate (lim_analyzers.pseudo)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct OperatorGate {
    pub operator:  Operator,
    pub active:    bool,
    pub priority:  u8,    // 1-4 (1 = më i lartë)
    pub reason:    String,
    pub max_depth: u8,
}

/// Katër operatorët e PRO.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Reverse,   // shkak nga efekti
    Mirror,    // gjej analogji
    Contrast,  // gjej diferencat
    Trace,     // mbush boshllëqet (fallback)
}

impl Operator {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Reverse  => "REVERSE",
            Self::Mirror   => "MIRROR",
            Self::Contrast => "CONTRAST",
            Self::Trace    => "TRACE",
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// DEPTH CONFIG — sa thellë shkon PRO
// Nga: STRUCT DepthConfig (lim_analyzers.pseudo)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct DepthConfig {
    pub operator_depth:          u32,   // max depth për PRO (1-7)
    pub cross_territory_allowed: bool,
    pub reconstruction_mode:     bool,  // PRO fokus në TRACE
    pub turbo_allowed:           bool,  // 200→1000 reasoning
}

// ─────────────────────────────────────────────────────────────────────────────
// EPISTEMIC SCAFFOLD — output i fazës 1 (hyrja për PRO)
// Nga: STRUCT EpistemicScaffold (lim_analyzers.pseudo)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct EpistemicScaffold {
    pub lim:                   LinearInfoMetric,
    pub operator_gates:        Vec<OperatorGate>,
    pub depth_config:          DepthConfig,
    pub ready_for_pro:         bool,   // mass >= 0.20
    pub reconstruction_needed: bool,   // density < 0.35
}

impl EpistemicScaffold {
    /// Operatorët aktivë, të renditur sipas prioritetit (1 i pari).
    pub fn active_operators(&self) -> Vec<&OperatorGate> {
        let mut active: Vec<&OperatorGate> =
            self.operator_gates.iter().filter(|g| g.active).collect();
        active.sort_by_key(|g| g.priority);
        active
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// EPISTEMIC CLASSIFICATION — output i fazës 2 (hyrja për Shadow)
// Nga: STRUCT EpistemicClassification (lim_core.pseudo)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct EpistemicClassification {
    pub lim_pre:          LinearInfoMetric,
    pub lim_post:         LinearInfoMetric,
    pub state:            EpistemicState,
    pub delta:            f32,
    pub session_id:       String,
    pub territory:        String,
    pub ready_for_shadow: bool,
    pub shadow_note:      String,
}

// ─────────────────────────────────────────────────────────────────────────────
// LIM ERROR
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum LimError {
    SealInvalid,
    EmptyInput,
    InsufficientMaterial,  // Grupi 1: jo mjaftueshëm fragmente
}

impl std::fmt::Display for LimError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::SealInvalid          => write!(f, "LGC seal invalid"),
            Self::EmptyInput           => write!(f, "Input bosh"),
            Self::InsufficientMaterial => write!(f, "Material i pamjaftueshëm (Grupi 1)"),
        }
    }
}
