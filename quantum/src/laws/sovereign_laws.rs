// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAWS/SOVEREIGN_LAWS.RS — Kushtetuta e Brendshme (Teoria 52)          ║
// ║  GJATA LEGACY™ — Quantum Platform                                     ║
// ║                                                                          ║
// ║  5 ligje të panegociueshme. Aktive gjithmonë. Zero if/else.           ║
// ║   1. FORBIDDEN_RECURSION  — Quantum s'thërret vetveten pa limit       ║
// ║   2. FORBIDDEN_BYPASS     — asnjë s'kapërcen shadow/lgc               ║
// ║   3. REASONING_PURITY     — vetëm Shadow shkruan knowledge            ║
// ║   4. CONVERGENCE_INTEGRITY— output me seal + shadow approval          ║
// ║   5. MODULE_AUTHORITY     — çdo modul brenda autoritetit të vet       ║
// ║                                                                          ║
// ║  Pseudo-kodi përdor IF; këtu KONVERTUAR në match/formula (ligji 0).   ║
// ╚══════════════════════════════════════════════════════════════════════════╝

// ─────────────────────────────────────────────────────────────────────────────
// LIGJI 1 — FORBIDDEN RECURSION
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
pub struct ForbiddenRecursion {
    pub max_pipeline_cycles:   u32,
    pub max_reasoning_waves:   u32,
    pub max_callback_chain:    u32,
    pub max_lab_retry:         u32,
    pub max_cluster_rebalance: u32,
}

impl ForbiddenRecursion {
    pub const fn standard() -> Self {
        Self {
            max_pipeline_cycles:   3,
            max_reasoning_waves:   10,
            max_callback_chain:    5,
            max_lab_retry:         2,
            max_cluster_rebalance: 2,
        }
    }

    /// Kategoria e recursion-it për t'u kontrolluar.
    /// enforce: count <= limit. Zero if — krahasim direkt boolean.
    pub fn enforce(&self, kind: RecursionKind, count: u32) -> bool {
        let limit = match kind {
            RecursionKind::PipelineCycle   => self.max_pipeline_cycles,
            RecursionKind::ReasoningWave    => self.max_reasoning_waves,
            RecursionKind::CallbackChain    => self.max_callback_chain,
            RecursionKind::LabRetry         => self.max_lab_retry,
            RecursionKind::ClusterRebalance => self.max_cluster_rebalance,
        };
        count <= limit
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RecursionKind {
    PipelineCycle,
    ReasoningWave,
    CallbackChain,
    LabRetry,
    ClusterRebalance,
}

// ─────────────────────────────────────────────────────────────────────────────
// LIGJI 2 — FORBIDDEN BYPASS
// ─────────────────────────────────────────────────────────────────────────────

pub struct ForbiddenBypass;

impl ForbiddenBypass {
    /// 5 entitetet e mbrojtura — s'mund të kapërcehen.
    pub const PROTECTED: [&'static str; 5] = [
        "shadow_gj_legacy.lgc",
        "LEGALGOCRYPT",
        "SupremeGate",
        "FilterGate",
        "PERSISTENT_STORE",
    ];

    /// enforce: asnjë "bypass_X" ose "skip_X" për X të mbrojtur.
    /// Zero if — përdor iterator + all() (formula boolean).
    pub fn enforce(action: &str) -> bool {
        // Për çdo entitet të mbrojtur, action NUK duhet të përmbajë bypass/skip.
        Self::PROTECTED.iter().all(|p| {
            let bypass = format!("bypass_{}", p);
            let skip = format!("skip_{}", p);
            // none-contains = (!a) AND (!b) — pa if.
            (!action.contains(&bypass)) && (!action.contains(&skip))
        })
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// LIGJI 3 — REASONING PURITY (Quantum NUK shkruan knowledge)
// ─────────────────────────────────────────────────────────────────────────────

pub struct ReasoningPurity;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Destination {
    VerifiedOptimal,
    VerifiedNegative,
    PersistentStore,
    Ephemeral,       // Quantum lejohet vetëm këtu
    Other,
}

impl ReasoningPurity {
    /// I VETMI shkrues i autorizuar për knowledge persistent.
    pub const AUTHORIZED_WRITER: &'static str = "shadow_gj_legacy.lgc";

    /// enforce: destinacionet e mbrojtura → vetëm Shadow shkruan.
    /// Quantum (çdo writer != shadow) → ephemeral OK, persistent JO.
    /// Zero if — match mbi (destination, writer_is_shadow).
    pub fn enforce(writer: &str, destination: Destination) -> bool {
        let is_shadow = (writer == Self::AUTHORIZED_WRITER) as u8;
        match destination {
            // Destinacionet e mbrojtura: lejohen VETËM nga Shadow.
            Destination::VerifiedOptimal
            | Destination::VerifiedNegative
            | Destination::PersistentStore => is_shadow == 1,
            // Ephemeral dhe Other: lejohen për këdo (Quantum përfshirë).
            Destination::Ephemeral | Destination::Other => true,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// LIGJI 4 — CONVERGENCE INTEGRITY
// ─────────────────────────────────────────────────────────────────────────────

/// Kontrata e output-it final që ligji kontrollon.
#[derive(Debug, Clone)]
pub struct FinalOutputCheck {
    pub has_lgc_seal:    bool,
    pub shadow_approved: bool,
    pub confidence:      f32,
}

pub struct ConvergenceIntegrity;

impl ConvergenceIntegrity {
    pub const MIN_CONFIDENCE: f32 = 0.30;

    /// enforce: seal AND shadow_approved AND confidence >= 0.30.
    /// Zero if — produkt boolean i tre kushteve.
    pub fn enforce(output: &FinalOutputCheck) -> bool {
        let seal_ok = output.has_lgc_seal as u8;
        let approved = output.shadow_approved as u8;
        let conf_ok = (output.confidence >= Self::MIN_CONFIDENCE) as u8;
        // Produkt: të tria duhet 1.
        (seal_ok * approved * conf_ok) == 1
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// LIGJI 5 — MODULE AUTHORITY
// ─────────────────────────────────────────────────────────────────────────────

pub struct ModuleAuthority;

impl ModuleAuthority {
    /// Kthen veprimet e lejuara për një modul (AUTHORITY_MAP).
    /// "*" = të gjitha.
    fn allowed_actions(module: &str) -> &'static [&'static str] {
        match module {
            "shadow_gj_legacy.lgc" => &["*"],
            "LEGALGOCRYPT"         => &["seal", "verify", "dispatch"],
            "QuantumOrchestrator"  => &["pipeline", "routing", "boot"],
            "QuantumBridge"        => &["filter", "verify", "transport"],
            "LAB_TRL3"             => &["concept", "hypothesis", "simulation"],
            "ELIMINATION"          => &["eliminate", "score", "rank"],
            "CONVERGENCE"          => &["fuse", "select", "deliver"],
            _                      => &[],
        }
    }

    /// enforce: "*" → gjithmonë; ose action fillon me një të lejuar.
    /// Zero if — kombinim boolean + any().
    pub fn enforce(module: &str, action: &str) -> bool {
        let allowed = Self::allowed_actions(module);
        let wildcard = allowed.iter().any(|a| *a == "*");
        let prefix_match = allowed.iter().any(|a| action.starts_with(a));
        wildcard || prefix_match
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ENFORCE ALL — kushtetuta e plotë
// ─────────────────────────────────────────────────────────────────────────────

/// Konteksti për enforce_all.
#[derive(Debug, Clone)]
pub struct LawContext {
    pub destination: Destination,
    pub recursion_kind: RecursionKind,
    pub recursion_count: u32,
}

pub struct SovereignConstitution {
    recursion: ForbiddenRecursion,
}

impl SovereignConstitution {
    pub fn new() -> Self {
        Self { recursion: ForbiddenRecursion::standard() }
    }

    /// enforce_all: të 4 ligjet aktive (1,2,3,5) si produkt boolean.
    /// (Ligji 4 aplikohet veçmas te output-i final.)
    /// Zero if — AND i drejtpërdrejtë.
    pub fn enforce_all(&self, action: &str, caller: &str, ctx: &LawContext) -> bool {
        ForbiddenRecursion::enforce(&self.recursion, ctx.recursion_kind, ctx.recursion_count)
            && ForbiddenBypass::enforce(action)
            && ReasoningPurity::enforce(caller, ctx.destination)
            && ModuleAuthority::enforce(caller, action)
    }
}

impl Default for SovereignConstitution {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recursion_within_limit() {
        let r = ForbiddenRecursion::standard();
        assert!(r.enforce(RecursionKind::PipelineCycle, 3));   // == limit OK
        assert!(!r.enforce(RecursionKind::PipelineCycle, 4));  // > limit FAIL
        assert!(r.enforce(RecursionKind::ReasoningWave, 10));
        assert!(!r.enforce(RecursionKind::ReasoningWave, 11));
    }

    #[test]
    fn bypass_blocks_protected() {
        assert!(ForbiddenBypass::enforce("normal_action"));
        assert!(!ForbiddenBypass::enforce("bypass_LEGALGOCRYPT now"));
        assert!(!ForbiddenBypass::enforce("skip_SupremeGate"));
        assert!(!ForbiddenBypass::enforce("try bypass_PERSISTENT_STORE"));
    }

    #[test]
    fn purity_only_shadow_writes_knowledge() {
        // Quantum (jo shadow) → s'shkruan dot VERIFIED_*.
        assert!(!ReasoningPurity::enforce("QuantumOrchestrator", Destination::VerifiedOptimal));
        assert!(!ReasoningPurity::enforce("QuantumBridge", Destination::PersistentStore));
        // Shadow → lejohet.
        assert!(ReasoningPurity::enforce("shadow_gj_legacy.lgc", Destination::VerifiedOptimal));
        // Quantum → ephemeral OK (Quantum NUK mban memory persistent).
        assert!(ReasoningPurity::enforce("QuantumOrchestrator", Destination::Ephemeral));
    }

    #[test]
    fn convergence_needs_all_three() {
        let good = FinalOutputCheck { has_lgc_seal: true, shadow_approved: true, confidence: 0.5 };
        assert!(ConvergenceIntegrity::enforce(&good));
        // Mungon seal
        let no_seal = FinalOutputCheck { has_lgc_seal: false, shadow_approved: true, confidence: 0.5 };
        assert!(!ConvergenceIntegrity::enforce(&no_seal));
        // Confidence ulët
        let low = FinalOutputCheck { has_lgc_seal: true, shadow_approved: true, confidence: 0.2 };
        assert!(!ConvergenceIntegrity::enforce(&low));
    }

    #[test]
    fn authority_respects_map() {
        assert!(ModuleAuthority::enforce("shadow_gj_legacy.lgc", "anything")); // *
        assert!(ModuleAuthority::enforce("ELIMINATION", "eliminate_local"));
        assert!(!ModuleAuthority::enforce("ELIMINATION", "deliver"));  // jo e saj
        assert!(ModuleAuthority::enforce("CONVERGENCE", "deliver_final"));
    }

    #[test]
    fn enforce_all_combines() {
        let c = SovereignConstitution::new();
        let ctx = LawContext {
            destination: Destination::Ephemeral,
            recursion_kind: RecursionKind::PipelineCycle,
            recursion_count: 1,
        };
        // QuantumOrchestrator, action e lejuar, ephemeral → OK.
        assert!(c.enforce_all("pipeline_run", "QuantumOrchestrator", &ctx));
        // Bypass → FAIL pavarësisht tjerash.
        assert!(!c.enforce_all("bypass_SupremeGate", "QuantumOrchestrator", &ctx));
    }
}
