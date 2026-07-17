// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  EXPLORATION/STATE_MACHINE.RS — QuantumPhase (Teoria 7)               ║
// ║  GJATA LEGACY™ — Quantum Platform                                     ║
// ║                                                                          ║
// ║  2 shtresa:                                                             ║
// ║   1. ShadowState (native): LIGHT/PRESSURED/ABSORBING/FAST_PATH        ║
// ║   2. QuantumPhase (i ri): IDLE→DETECT→...→TRANSFER                    ║
// ║                                                                          ║
// ║  Shadow vendos nëse Quantum mund të avancojë në çdo tranzicion.        ║
// ║   FAST_PATH: skip MAP+EXPAND → GENERATE                              ║
// ║   ABSORBING: bllokon çdo fazë pas DETECT                             ║
// ║                                                                          ║
// ║  Pseudo përdor IF; KONVERTUAR plotësisht në match (zero if/else).      ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::control::pressure_states::ShadowState;

// ─────────────────────────────────────────────────────────────────────────────
// QUANTUM PHASE — 9 faza
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QuantumPhase {
    Idle      = 0,
    Detect    = 1,
    Map       = 2,
    Expand    = 3,
    Generate  = 4,
    Eliminate = 5,
    Converge  = 6,
    Validate  = 7,
    Transfer  = 8,
}

impl QuantumPhase {
    pub fn label(self) -> &'static str {
        match self {
            QuantumPhase::Idle      => "IDLE",
            QuantumPhase::Detect    => "DETECT",
            QuantumPhase::Map       => "MAP",
            QuantumPhase::Expand    => "EXPAND",
            QuantumPhase::Generate  => "GENERATE",
            QuantumPhase::Eliminate => "ELIMINATE",
            QuantumPhase::Converge  => "CONVERGE",
            QuantumPhase::Validate  => "VALIDATE",
            QuantumPhase::Transfer  => "TRANSFER",
        }
    }

    pub fn rank(self) -> u8 {
        self as u8
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// TRANSITIONS — tranzicionet e lejuara (match, zero if)
// ─────────────────────────────────────────────────────────────────────────────

/// is_valid_transition — a lejohet from→to.
/// Zero if — match mbi tuple.
fn is_valid_transition(from: QuantumPhase, to: QuantumPhase) -> bool {
    use QuantumPhase::*;
    matches!(
        (from, to),
        (Idle, Detect)
        | (Detect, Map)
        | (Map, Expand)
        | (Expand, Generate)
        | (Generate, Eliminate)
        | (Eliminate, Converge)
        | (Converge, Validate)
        | (Validate, Transfer)
        | (Validate, Idle)      // nëse shadow refuzon
        | (Transfer, Idle)
    )
}

// ─────────────────────────────────────────────────────────────────────────────
// ADVANCE RESULT — çfarë ndodhi me tranzicionin
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AdvanceResult {
    Advanced(QuantumPhase),  // kaloi në fazën e re
    FastPath(QuantumPhase),  // FAST_PATH skip → GENERATE
    BlockedAbsorbing,        // ABSORBING e bllokoi
    InvalidTransition,       // tranzicion i palejuar
}

// ─────────────────────────────────────────────────────────────────────────────
// QUANTUM STATE MACHINE
// ─────────────────────────────────────────────────────────────────────────────

pub struct QuantumStateMachine {
    phase: QuantumPhase,
}

impl QuantumStateMachine {
    pub fn new() -> Self {
        Self { phase: QuantumPhase::Idle }
    }

    pub fn current(&self) -> QuantumPhase {
        self.phase
    }

    /// advance — kalo në fazën tjetër, me Shadow që vendos.
    /// Zero if — match mbi (valid, shadow_state, phase, to).
    pub fn advance(&mut self, to: QuantumPhase, shadow_state: ShadowState) -> AdvanceResult {
        // 1. Tranzicion i vlefshëm?
        let valid = is_valid_transition(self.phase, to);

        // 2. FAST_PATH: DETECT→MAP bëhet DETECT→GENERATE.
        let is_fast_skip = shadow_state == ShadowState::FastPath
            && self.phase == QuantumPhase::Detect
            && to == QuantumPhase::Map;

        // 3. ABSORBING: bllokon çdo fazë pas DETECT.
        let is_absorbing_block = shadow_state == ShadowState::Absorbing
            && self.phase.rank() > QuantumPhase::Detect.rank();

        // Match mbi gjendjen (prioritet: invalid → absorbing → fast → normal).
        match (valid, is_absorbing_block, is_fast_skip) {
            // Tranzicion i palejuar.
            (false, _, _) => AdvanceResult::InvalidTransition,
            // ABSORBING bllokon.
            (true, true, _) => AdvanceResult::BlockedAbsorbing,
            // FAST_PATH skip → GENERATE.
            (true, false, true) => {
                self.phase = QuantumPhase::Generate;
                AdvanceResult::FastPath(QuantumPhase::Generate)
            }
            // Avancim normal.
            (true, false, false) => {
                self.phase = to;
                AdvanceResult::Advanced(to)
            }
        }
    }

    /// reset → IDLE.
    pub fn reset(&mut self) {
        self.phase = QuantumPhase::Idle;
    }
}

impl Default for QuantumStateMachine {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_idle() {
        let sm = QuantumStateMachine::new();
        assert_eq!(sm.current(), QuantumPhase::Idle);
    }

    #[test]
    fn normal_advance() {
        let mut sm = QuantumStateMachine::new();
        let r = sm.advance(QuantumPhase::Detect, ShadowState::Light);
        assert_eq!(r, AdvanceResult::Advanced(QuantumPhase::Detect));
        assert_eq!(sm.current(), QuantumPhase::Detect);
    }

    #[test]
    fn invalid_transition_rejected() {
        let mut sm = QuantumStateMachine::new();
        // IDLE → GENERATE s'lejohet (vetëm IDLE→DETECT).
        let r = sm.advance(QuantumPhase::Generate, ShadowState::Light);
        assert_eq!(r, AdvanceResult::InvalidTransition);
        assert_eq!(sm.current(), QuantumPhase::Idle); // s'ndryshoi
    }

    #[test]
    fn fast_path_skips_to_generate() {
        let mut sm = QuantumStateMachine::new();
        sm.advance(QuantumPhase::Detect, ShadowState::Light); // → DETECT
        // FAST_PATH: DETECT→MAP bëhet → GENERATE.
        let r = sm.advance(QuantumPhase::Map, ShadowState::FastPath);
        assert_eq!(r, AdvanceResult::FastPath(QuantumPhase::Generate));
        assert_eq!(sm.current(), QuantumPhase::Generate);
    }

    #[test]
    fn absorbing_blocks_after_detect() {
        let mut sm = QuantumStateMachine::new();
        sm.advance(QuantumPhase::Detect, ShadowState::Light); // → DETECT
        sm.advance(QuantumPhase::Map, ShadowState::Light);    // → MAP
        // ABSORBING bllokon avancim pas DETECT.
        let r = sm.advance(QuantumPhase::Expand, ShadowState::Absorbing);
        assert_eq!(r, AdvanceResult::BlockedAbsorbing);
    }

    #[test]
    fn full_pipeline_flow() {
        let mut sm = QuantumStateMachine::new();
        let phases = [
            QuantumPhase::Detect, QuantumPhase::Map, QuantumPhase::Expand,
            QuantumPhase::Generate, QuantumPhase::Eliminate, QuantumPhase::Converge,
            QuantumPhase::Validate, QuantumPhase::Transfer,
        ];
        for p in phases {
            let r = sm.advance(p, ShadowState::Light);
            assert_eq!(r, AdvanceResult::Advanced(p));
        }
        assert_eq!(sm.current(), QuantumPhase::Transfer);
    }

    #[test]
    fn validate_can_return_to_idle() {
        let mut sm = QuantumStateMachine::new();
        // Ngjit deri te VALIDATE.
        for p in [QuantumPhase::Detect, QuantumPhase::Map, QuantumPhase::Expand,
                  QuantumPhase::Generate, QuantumPhase::Eliminate, QuantumPhase::Converge,
                  QuantumPhase::Validate] {
            sm.advance(p, ShadowState::Light);
        }
        // VALIDATE→IDLE (shadow refuzon) — lejohet.
        let r = sm.advance(QuantumPhase::Idle, ShadowState::Light);
        assert_eq!(r, AdvanceResult::Advanced(QuantumPhase::Idle));
    }

    #[test]
    fn reset_returns_idle() {
        let mut sm = QuantumStateMachine::new();
        sm.advance(QuantumPhase::Detect, ShadowState::Light);
        sm.reset();
        assert_eq!(sm.current(), QuantumPhase::Idle);
    }
}
