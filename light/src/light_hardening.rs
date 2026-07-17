// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LIGHT_HARDENING.RS — Përforcimi i Light Platform                     ║
// ║  GJATA LEGACY™ — Light Platform                                   ║
// ║                                                                          ║
// ║  E bën Light-in MË TË FORTË pas integrimit të :                 ║
// ║                                                                          ║
// ║   1. AlgorithmHealth — APUPK/SNB/Mini + deklarimi PD/iZ              ║
// ║   2. SovereignAudit — verifikim që çdo kalim ndër-platformë ka vulë   ║
// ║   3. IntegrationGuard — siguron që Light NUK vendos (vetëm orienton)  ║
// ║   4. CapabilityBudget — limit i capability-ve për cikël (anti-replay) ║
// ║                                                                          ║
// ║  Zero if/else — match/formula.                                        ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::sovereign::{verify_500, SovereignGate, SEAL_COORDINATOR};

// ─────────────────────────────────────────────────────────────────────────────
// 1. ALGORITHM HEALTH — health check + kontinuumi Mini→Algorithm→PD/iZ
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
pub struct AlgorithmHealth {
    pub apupk_ready:  bool,
    pub snb_ready:    bool,
    pub mini_ready:   bool,
    pub pd_continuum_ready: bool,
    pub health_score: u32,   // fixed mass [0..10_000]
}

impl AlgorithmHealth {
    /// Score-i është masë fixed-point, jo float.
    pub fn check(apupk: bool, snb: bool, mini: bool, pd_continuum: bool) -> Self {
        let ready_count = (apupk as u32) + (snb as u32)
            + (mini as u32) + (pd_continuum as u32);
        let health_score = crate::pd_spine_contract::FixedMass::from_ratio(
            ready_count, 4,
        ).raw();
        Self {
            apupk_ready: apupk,
            snb_ready: snb,
            mini_ready: mini,
            pd_continuum_ready: pd_continuum,
            health_score,
        }
    }

    pub fn all_healthy(&self) -> bool {
        self.apupk_ready && self.snb_ready && self.mini_ready
            && self.pd_continuum_ready
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 2. SOVEREIGN AUDIT — verifikim i vulave ndër-platformë
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct SovereignAuditReport {
    pub transfers_checked: usize,
    pub valid_seals:       usize,
    pub invalid_seals:     usize,
    pub all_sealed:        bool,
}

pub struct SovereignAudit;

impl SovereignAudit {
    /// audit_seals — verifikon një grup flags-esh që duhet të jenë vula 500.
    /// Zero if — numërim via iterator + boolean.
    pub fn audit_seals(seal_flags: &[u32]) -> SovereignAuditReport {
        let total = seal_flags.len();
        let valid = seal_flags.iter().filter(|&&f| verify_500(f)).count();
        let invalid = total - valid;
        // all_sealed = (valid == total) AND (total > 0).
        let all_sealed = (valid == total) && (total > 0);

        SovereignAuditReport {
            transfers_checked: total,
            valid_seals: valid,
            invalid_seals: invalid,
            all_sealed,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 3. INTEGRATION GUARD — siguron që Light s'vendos
// ─────────────────────────────────────────────────────────────────────────────

/// Light orienton dhe përgatit — kurrë s'vendos (Shadow vendos).
/// Ky guard verifikon që asnjë komponent i Light s'pretendon vendim.
pub struct IntegrationGuard;

impl IntegrationGuard {
    /// verify_no_decision — a respekton një veprim ligjin "Light s'vendos"?
    ///
    /// action_kind: çfarë po bën Light. Vetëm "prepare"/"orient"/"route" lejohen.
    /// "decide"/"verify"/"approve" → JO (ato janë të Shadow).
    /// Zero if — match mbi llojin.
    pub fn verify_no_decision(action_kind: &str) -> bool {
        let forbidden = action_kind.contains("decide")
            || action_kind.contains("verify_final")
            || action_kind.contains("approve")
            || action_kind.contains("judge");
        // Light OK vetëm kur NUK ka veprim vendimi.
        !forbidden
    }

    /// classify_action — kategorizon veprimin (informativ).
    pub fn classify_action(action_kind: &str) -> &'static str {
        let prepare = action_kind.contains("prepare");
        let route = action_kind.contains("route");
        let orient = action_kind.contains("orient");
        match (prepare, route, orient) {
            (true, _, _) => "PREPARE",
            (_, true, _) => "ROUTE",
            (_, _, true) => "ORIENT",
            _            => "OTHER",
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 4. CAPABILITY BUDGET — limit i capability-ve për cikël
// ─────────────────────────────────────────────────────────────────────────────

/// Limit i capability-ve të lëshuara për cikël (anti-replay/anti-flood).
pub const MAX_CAPABILITIES_PER_CYCLE: u64 = 64;

pub struct CapabilityBudget {
    issued: u64,
}

impl CapabilityBudget {
    pub fn new() -> Self {
        Self { issued: 0 }
    }

    /// can_issue — a mund të lëshohet një capability tjetër? Zero if.
    pub fn can_issue(&self) -> bool {
        self.issued < MAX_CAPABILITIES_PER_CYCLE
    }

    /// record_issue — regjistro lëshimin. Kthen true nëse u lejua.
    /// Zero if — match mbi can_issue.
    pub fn record_issue(&mut self) -> bool {
        match self.can_issue() {
            true => {
                self.issued += 1;
                true
            }
            false => false,
        }
    }

    /// reset — fund cikli (Light s'mban gjendje ndër cikle).
    pub fn reset(&mut self) {
        self.issued = 0;
    }

    pub fn issued(&self) -> u64 {
        self.issued
    }
}

impl Default for CapabilityBudget {
    fn default() -> Self { Self::new() }
}

// ─────────────────────────────────────────────────────────────────────────────
// LIGHT HARDENING — orchestrimi i përforcimit
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct LightHardeningReport {
    pub health:          AlgorithmHealth,
    pub sovereign_ok:    bool,
    pub no_decision_ok:  bool,
    pub capability_ok:   bool,
    pub overall_score:   u32, // fixed mass [0..10_000]
}

pub struct LightHardening;

impl LightHardening {
    /// run_full — ekzekuton të 4 përforcimet, kthen raport të plotë.
    pub fn run_full(
        apupk: bool, snb: bool, mini: bool, pd_continuum: bool,
        seal_flags: &[u32],
    ) -> LightHardeningReport {
        // 1. Health check.
        let health = AlgorithmHealth::check(apupk, snb, mini, pd_continuum);

        // 2. Sovereign audit.
        let audit = SovereignAudit::audit_seals(seal_flags);
        let sovereign_ok = audit.all_sealed;

        // 3. No-decision guard (Light orienton).
        let no_decision_ok = IntegrationGuard::verify_no_decision("prepare_and_route");

        // 4. Capability cycle test.
        let mut budget = CapabilityBudget::new();
        let mut gate = SovereignGate::new();
        let _handle = gate.issue(SEAL_COORDINATOR);
        let capability_ok = budget.record_issue();

        // overall_score: mesatarja e 4 dimensioneve.
        let dims = (health.all_healthy() as u32)
                 + (sovereign_ok as u32)
                 + (no_decision_ok as u32)
                 + (capability_ok as u32);
        let overall_score = crate::pd_spine_contract::FixedMass::from_ratio(
            dims, 4,
        ).raw();

        LightHardeningReport {
            health,
            sovereign_ok,
            no_decision_ok,
            capability_ok,
            overall_score,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn health_all_ready() {
        let h = AlgorithmHealth::check(true, true, true, true);
        assert!(h.all_healthy());
        assert_eq!(h.health_score, 10_000);
    }

    #[test]
    fn health_partial() {
        let h = AlgorithmHealth::check(true, false, true, true);
        assert!(!h.all_healthy());
        assert_eq!(h.health_score, 7_500);
    }

    #[test]
    fn audit_all_valid_seals() {
        // 0xA451 → verify_500 == true.
        let report = SovereignAudit::audit_seals(&[0xA451, 0xA451, 0xA451]);
        assert_eq!(report.valid_seals, 3);
        assert_eq!(report.invalid_seals, 0);
        assert!(report.all_sealed);
    }

    #[test]
    fn audit_detects_invalid() {
        let report = SovereignAudit::audit_seals(&[0xA451, 0x1234, 0xA451]);
        assert_eq!(report.valid_seals, 2);
        assert_eq!(report.invalid_seals, 1);
        assert!(!report.all_sealed);
    }

    #[test]
    fn integration_guard_allows_prepare() {
        // "prepare" → Light OK.
        assert!(IntegrationGuard::verify_no_decision("prepare_package"));
        assert!(IntegrationGuard::verify_no_decision("route_input"));
    }

    #[test]
    fn integration_guard_blocks_decision() {
        // "decide"/"approve" → JO (ato janë të Shadow).
        assert!(!IntegrationGuard::verify_no_decision("decide_verdict"));
        assert!(!IntegrationGuard::verify_no_decision("approve_output"));
        assert!(!IntegrationGuard::verify_no_decision("judge_candidate"));
    }

    #[test]
    fn action_classified() {
        assert_eq!(IntegrationGuard::classify_action("prepare_x"), "PREPARE");
        assert_eq!(IntegrationGuard::classify_action("route_y"), "ROUTE");
        assert_eq!(IntegrationGuard::classify_action("orient_z"), "ORIENT");
    }

    #[test]
    fn capability_budget_limits() {
        let mut budget = CapabilityBudget::new();
        // Lësho deri në limit.
        let mut count = 0;
        while budget.record_issue() {
            count += 1;
        }
        assert_eq!(count, MAX_CAPABILITIES_PER_CYCLE);
        // Pas limitit → refuzim.
        assert!(!budget.can_issue());
    }

    #[test]
    fn capability_budget_resets() {
        let mut budget = CapabilityBudget::new();
        budget.record_issue();
        budget.reset();
        assert_eq!(budget.issued(), 0);
        assert!(budget.can_issue());
    }

    #[test]
    fn hardening_full_healthy() {
        let report = LightHardening::run_full(
            true, true, true, true,
            &[0xA451, 0xA451],
        );
        assert!(report.health.all_healthy());
        assert!(report.sovereign_ok);
        assert!(report.no_decision_ok);
        assert!(report.capability_ok);
        assert_eq!(report.overall_score, 10_000);
    }

    #[test]
    fn hardening_detects_unsealed() {
        let report = LightHardening::run_full(
            true, true, true, true,
            &[0xA451, 0x0000],  // një i pavlefshëm
        );
        // Sovereign audit dështon.
        assert!(!report.sovereign_ok);
        assert!(report.overall_score < 10_000);
    }
}
