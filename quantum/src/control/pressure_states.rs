// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  CONTROL/PRESSURE_STATES.RS — 6 Gjendje Adaptive (Teoria 32)          ║
// ║  GJATA LEGACY™ — Quantum Platform                                     ║
// ║                                                                          ║
// ║  Presioni epistemik vendos sjelljen e eliminimit/konvergjencës.       ║
// ║   LOW / HIGH / EMERGENCY / EXPLORATORY / DEEP / OVERLOAD              ║
// ║  Çdo gjendje: threshold, max_candidates, recursion, timeout.          ║
// ║                                                                          ║
// ║  detect() ka prioritet — pseudo përdor IF; KONVERTUAR në match.       ║
// ╚══════════════════════════════════════════════════════════════════════════╝

// ─────────────────────────────────────────────────────────────────────────────
// PRESSURE STATE
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PressureState {
    Low,         // normale, exploratory i lirë
    High,        // kufij të shtrënguar
    Emergency,   // duhet 1 rezultat sa më shpejt
    Exploratory, // ruan diversitet, eliminim i butë
    Deep,        // iteracion i thellë
    Overload,    // shumë kontradikta, freeze partial
}

impl PressureState {
    pub fn label(self) -> &'static str {
        match self {
            PressureState::Low         => "LOW",
            PressureState::High        => "HIGH",
            PressureState::Emergency   => "EMERGENCY",
            PressureState::Exploratory => "EXPLORATORY",
            PressureState::Deep        => "DEEP",
            PressureState::Overload    => "OVERLOAD",
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// PRESSURE CONFIG — sjellja për çdo gjendje
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
pub struct PressureConfig {
    pub state:                 PressureState,
    pub elimination_threshold: f32,
    pub max_candidates:        u32,
    pub recursion_limit:       u32,
    pub timeout_ms:            u32,
    pub allow_contradiction:   bool,
    pub require_lineage_check: bool,
}

impl PressureConfig {
    /// CONFIGS — vlerat IDENTIKE me pseudo, përmes match (zero if).
    pub fn for_state(state: PressureState) -> Self {
        match state {
            PressureState::Low => Self {
                state, elimination_threshold: 0.30, max_candidates: 50,
                recursion_limit: 10, timeout_ms: 10000,
                allow_contradiction: true, require_lineage_check: false,
            },
            PressureState::High => Self {
                state, elimination_threshold: 0.60, max_candidates: 10,
                recursion_limit: 5, timeout_ms: 3000,
                allow_contradiction: false, require_lineage_check: true,
            },
            PressureState::Emergency => Self {
                state, elimination_threshold: 0.80, max_candidates: 3,
                recursion_limit: 2, timeout_ms: 1000,
                allow_contradiction: false, require_lineage_check: false,
            },
            PressureState::Exploratory => Self {
                state, elimination_threshold: 0.20, max_candidates: 100,
                recursion_limit: 15, timeout_ms: 30000,
                allow_contradiction: true, require_lineage_check: true,
            },
            PressureState::Deep => Self {
                state, elimination_threshold: 0.50, max_candidates: 20,
                recursion_limit: 20, timeout_ms: 60000,
                allow_contradiction: false, require_lineage_check: true,
            },
            PressureState::Overload => Self {
                state, elimination_threshold: 0.90, max_candidates: 3,
                recursion_limit: 2, timeout_ms: 2000,
                allow_contradiction: false, require_lineage_check: true,
            },
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// SHADOW STATE (nga shadow_gj_legacy) — për detect
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShadowState {
    Light,
    Pressured,
    Absorbing,
    FastPath,
}

// ─────────────────────────────────────────────────────────────────────────────
// DETECT — llogarit gjendjen nga konteksti (zero if/else)
// ─────────────────────────────────────────────────────────────────────────────

/// Konteksti për detect.
#[derive(Debug, Clone, Copy)]
pub struct PressureContext {
    pub entropy:             f32,
    pub contradiction_count: u32,
    pub candidates_count:    u32,
    pub time_elapsed_ms:     u32,
    pub shadow_state:        ShadowState,
}

pub struct PressureDetector;

impl PressureDetector {
    /// detect — prioriteti i pseudo-kodit i ruajtur, KONVERTUAR në match-guards.
    ///
    /// Rendi (i pari që përputhet fiton):
    ///  1. contradiction > 10        → OVERLOAD
    ///  2. shadow ABSORBING          → EMERGENCY
    ///  3. entropy > 0.80            → HIGH
    ///  4. time > 20000ms            → EMERGENCY
    ///  5. candidates > 80 & entropy < 0.30 → EXPLORATORY
    ///  6. shadow PRESSURED          → HIGH
    ///  7. default                   → LOW
    pub fn detect(ctx: &PressureContext) -> PressureState {
        // Ndërto sinjalet boolean → tuple → match (zero if-chain).
        let overload  = ctx.contradiction_count > 10;
        let absorbing = ctx.shadow_state == ShadowState::Absorbing;
        let high_ent  = ctx.entropy > 0.80;
        let timeout   = ctx.time_elapsed_ms > 20000;
        let explore   = ctx.candidates_count > 80 && ctx.entropy < 0.30;
        let pressured = ctx.shadow_state == ShadowState::Pressured;

        // Match mbi tuple — i pari true fiton (prioritet i ruajtur).
        match (overload, absorbing, high_ent, timeout, explore, pressured) {
            (true, _, _, _, _, _)            => PressureState::Overload,
            (_, true, _, _, _, _)            => PressureState::Emergency,
            (_, _, true, _, _, _)            => PressureState::High,
            (_, _, _, true, _, _)            => PressureState::Emergency,
            (_, _, _, _, true, _)            => PressureState::Exploratory,
            (_, _, _, _, _, true)            => PressureState::High,
            _                                 => PressureState::Low,
        }
    }

    /// Shkurtore: detect + config.
    pub fn detect_config(ctx: &PressureContext) -> PressureConfig {
        PressureConfig::for_state(Self::detect(ctx))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx(entropy: f32, contradictions: u32, candidates: u32, time: u32, shadow: ShadowState) -> PressureContext {
        PressureContext {
            entropy, contradiction_count: contradictions,
            candidates_count: candidates, time_elapsed_ms: time,
            shadow_state: shadow,
        }
    }

    #[test]
    fn overload_has_priority() {
        // contradiction > 10 fiton edhe me entropy të lartë.
        let c = ctx(0.95, 15, 5, 100, ShadowState::Absorbing);
        assert_eq!(PressureDetector::detect(&c), PressureState::Overload);
    }

    #[test]
    fn absorbing_gives_emergency() {
        let c = ctx(0.5, 2, 5, 100, ShadowState::Absorbing);
        assert_eq!(PressureDetector::detect(&c), PressureState::Emergency);
    }

    #[test]
    fn high_entropy_gives_high() {
        let c = ctx(0.85, 2, 5, 100, ShadowState::Light);
        assert_eq!(PressureDetector::detect(&c), PressureState::High);
    }

    #[test]
    fn timeout_gives_emergency() {
        let c = ctx(0.5, 2, 5, 25000, ShadowState::Light);
        assert_eq!(PressureDetector::detect(&c), PressureState::Emergency);
    }

    #[test]
    fn many_candidates_low_entropy_explores() {
        let c = ctx(0.2, 2, 90, 100, ShadowState::Light);
        assert_eq!(PressureDetector::detect(&c), PressureState::Exploratory);
    }

    #[test]
    fn default_is_low() {
        let c = ctx(0.4, 2, 20, 100, ShadowState::Light);
        assert_eq!(PressureDetector::detect(&c), PressureState::Low);
    }

    #[test]
    fn configs_match_pseudo() {
        let low = PressureConfig::for_state(PressureState::Low);
        assert_eq!(low.elimination_threshold, 0.30);
        assert_eq!(low.max_candidates, 50);
        let emerg = PressureConfig::for_state(PressureState::Emergency);
        assert_eq!(emerg.elimination_threshold, 0.80);
        assert_eq!(emerg.max_candidates, 3);
        let overload = PressureConfig::for_state(PressureState::Overload);
        assert_eq!(overload.elimination_threshold, 0.90);
    }
}
