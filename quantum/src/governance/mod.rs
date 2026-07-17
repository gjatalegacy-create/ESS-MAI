// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  GOVERNANCE/MOD.RS — Mbrojtja Operative (Teoria mbështetëse)          ║
// ║  GJATA LEGACY™ — Quantum Platform                                     ║
// ║                                                                          ║
// ║  Kontrolli i recursion-it, override detection, illegal reasoning.     ║
// ║  Ushqen LIGJIN 1 (FORBIDDEN_RECURSION) të kushtetutës.                ║
// ║  Zero if/else — match/formula.                                        ║
// ║                                                                          ║
// ║  SHËNIM: counters janë EPHEMERAL (jetojnë vetëm brenda ciklit).        ║
// ║  Quantum NUK mban memory persistent — këto vdesin me reset_cycle.      ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use std::collections::HashMap;

// ─────────────────────────────────────────────────────────────────────────────
// RECURSION LIMITS — identike me pseudo
// ─────────────────────────────────────────────────────────────────────────────

fn recursion_limit(context_key: &str) -> u32 {
    match context_key {
        "pipeline_cycle"    => 3,
        "reasoning_wave"    => 10,
        "callback_chain"    => 5,
        "lab_trl_retry"     => 2,
        "cluster_rebalance" => 2,
        _                   => 3,  // default
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// GOVERNANCE — gjendja ephemeral (vdes me ciklin)
// ─────────────────────────────────────────────────────────────────────────────

pub struct Governance {
    recursion_counters: HashMap<String, u32>,
    override_attempts:  u32,
    blocked_count:      u32,
}

impl Governance {
    pub fn new() -> Self {
        Self {
            recursion_counters: HashMap::new(),
            override_attempts:  0,
            blocked_count:      0,
        }
    }

    // ── RECURSION LIMITS ───────────────────────────────────────────────────

    /// check_recursion: current < limit → inkremento, OK; ndryshe FAIL.
    /// Zero if — match mbi rezultatin e krahasimit.
    pub fn check_recursion(&mut self, context_key: &str) -> bool {
        let current = *self.recursion_counters.get(context_key).unwrap_or(&0);
        let limit = recursion_limit(context_key);

        match current < limit {
            true => {
                self.recursion_counters.insert(context_key.to_string(), current + 1);
                true
            }
            false => false,
        }
    }

    /// reset_recursion: ul counter-in në 0 (fund cikli).
    pub fn reset_recursion(&mut self, context_key: &str) {
        self.recursion_counters.insert(context_key.to_string(), 0);
    }

    /// reset_cycle: pastron TË GJITHA counters (Quantum s'mban memory).
    pub fn reset_cycle(&mut self) {
        self.recursion_counters.clear();
    }

    // ── OVERRIDE DETECTION ─────────────────────────────────────────────────

    /// detect_override: çdo sinjal force/override/bypass/skip → true.
    /// Zero if — any() mbi vektorin e sinjaleve.
    pub fn detect_override(&mut self, action: &str, ignore_shadow: bool, force_approve: bool) -> bool {
        let signals = [
            action.contains("force"),
            action.contains("override"),
            action.contains("bypass"),
            action.contains("skip"),
            ignore_shadow,
            force_approve,
        ];
        let detected = signals.iter().any(|&s| s);

        // Inkremento counter-in nëse u detektua (branchless: + detected as u32).
        self.override_attempts += detected as u32;
        detected
    }

    // ── ILLEGAL REASONING DETECTION ────────────────────────────────────────

    /// detect_illegal_reasoning: kontrollon score-t invalid + self-reference.
    /// candidates: (provider, proposal_contains_query, quantum_score).
    /// Zero if — mbledh issues me iterator, kthen (count > 0).
    pub fn detect_illegal_reasoning(&mut self, candidates: &[(String, bool, f32)], _query: &str) -> bool {
        // Issue 1: self-reference (proposal përmban query AND provider = QUANTUM).
        let self_ref = candidates.iter()
            .filter(|(provider, contains_q, _)| *contains_q && provider.contains("QUANTUM"))
            .count();

        // Issue 2: score invalid (> 1.0 ose < 0.0).
        let invalid_score = candidates.iter()
            .filter(|(_, _, score)| *score > 1.0 || *score < 0.0)
            .count();

        let total_issues = self_ref + invalid_score;
        // record + kthim boolean (count > 0).
        let illegal = (total_issues > 0) as u8;
        self.blocked_count += illegal as u32;
        illegal == 1
    }

    // ── METRICS ────────────────────────────────────────────────────────────

    pub fn override_count(&self) -> u32 { self.override_attempts }
    pub fn blocked_count(&self) -> u32 { self.blocked_count }

    pub fn recursion_depth(&self, context_key: &str) -> u32 {
        *self.recursion_counters.get(context_key).unwrap_or(&0)
    }
}

impl Default for Governance {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recursion_increments_until_limit() {
        let mut g = Governance::new();
        // pipeline_cycle limit = 3
        assert!(g.check_recursion("pipeline_cycle")); // 0→1
        assert!(g.check_recursion("pipeline_cycle")); // 1→2
        assert!(g.check_recursion("pipeline_cycle")); // 2→3
        assert!(!g.check_recursion("pipeline_cycle")); // 3 >= 3 FAIL
    }

    #[test]
    fn reset_clears_counter() {
        let mut g = Governance::new();
        g.check_recursion("reasoning_wave");
        assert_eq!(g.recursion_depth("reasoning_wave"), 1);
        g.reset_recursion("reasoning_wave");
        assert_eq!(g.recursion_depth("reasoning_wave"), 0);
    }

    #[test]
    fn reset_cycle_clears_all() {
        let mut g = Governance::new();
        g.check_recursion("pipeline_cycle");
        g.check_recursion("callback_chain");
        g.reset_cycle(); // Quantum s'mban memory
        assert_eq!(g.recursion_depth("pipeline_cycle"), 0);
        assert_eq!(g.recursion_depth("callback_chain"), 0);
    }

    #[test]
    fn override_detected_on_signals() {
        let mut g = Governance::new();
        assert!(g.detect_override("force_approve_now", false, false));
        assert!(g.detect_override("normal", true, false)); // ignore_shadow
        assert!(!g.detect_override("normal_action", false, false));
        assert_eq!(g.override_count(), 2);
    }

    #[test]
    fn illegal_score_detected() {
        let mut g = Governance::new();
        let bad = vec![("QUANTUM_A".to_string(), false, 1.5)]; // score > 1.0
        assert!(g.detect_illegal_reasoning(&bad, "q"));
        let good = vec![("PRO_B".to_string(), false, 0.8)];
        assert!(!g.detect_illegal_reasoning(&good, "q"));
    }

    #[test]
    fn self_reference_detected() {
        let mut g = Governance::new();
        // provider QUANTUM + proposal përmban query
        let loop_cand = vec![("QUANTUM_X".to_string(), true, 0.7)];
        assert!(g.detect_illegal_reasoning(&loop_cand, "query"));
    }
}
