// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  EXPLORATION/FRONTIER.RS — Epistemic Frontier Detector (Teoria 14)    ║
// ║  GJATA LEGACY™ — Quantum Platform                                     ║
// ║                                                                          ║
// ║  NOVEL PRIMITIVE: jo "nuk di" — por "ku lineazhi i qytetërimit nuk ka  ║
// ║  strukturë të stabilizuar". 7 akse, tavane ABSOLUTE.                  ║
// ║                                                                          ║
// ║  Tavanet: entropy ≤ 0.85 | contradiction ≤ 0.75 | recursion ≤ 7       ║
// ║           | semantic_drift ≤ 0.80                                     ║
// ║  Tejkalim → tkurrje e menjëhershme (ceiling breach).                  ║
// ║                                                                          ║
// ║  Pseudo përdor IF; KONVERTUAR plotësisht në match/formula (zero if).   ║
// ╚══════════════════════════════════════════════════════════════════════════╝

// ─────────────────────────────────────────────────────────────────────────────
// 7 AKSET E FRONTIERIT
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
pub struct FrontierAxes {
    pub frontier_depth:           f32,  // sa thellë në terra incognita
    pub unresolved_cycles:        u32,  // cikle pa konvergjencë
    pub lineage_void_score:       f32,  // sa bosh është lineage
    pub contradiction_density:    f32,  // densiteti i kontradiktave
    pub convergence_failure_rate: f32,  // % cikle pa konvergjencë
    pub semantic_drift_score:     f32,  // largësia nga semantic core
    pub authority_distance:       f32,  // largësia nga shadow_gj_legacy.lgc
}

impl FrontierAxes {
    pub fn initial() -> Self {
        Self {
            frontier_depth: 0.0, unresolved_cycles: 0,
            lineage_void_score: 0.0, contradiction_density: 0.0,
            convergence_failure_rate: 0.0, semantic_drift_score: 0.0,
            authority_distance: 0.0,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// TAVANET ABSOLUTE (identike me pseudo)
// ─────────────────────────────────────────────────────────────────────────────

pub const ENTROPY_CEILING:        f32 = 0.85;
pub const CONTRADICTION_CEILING:  f32 = 0.75;
pub const RECURSION_CEILING:      u32 = 7;
pub const SEMANTIC_DRIFT_CEILING: f32 = 0.80;

// ─────────────────────────────────────────────────────────────────────────────
// FRONTIER STATE — gjendja epistemike
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FrontierState {
    Stable,       // brenda lineage të njohur
    Exploratory,  // duke zgjeruar
    Contracting,  // duke u tkurrur (pas breach)
    Transition,   // afër kalimit të fazës
    Void,         // terra incognita e thellë
}

impl FrontierState {
    pub fn label(self) -> &'static str {
        match self {
            FrontierState::Stable      => "STABLE",
            FrontierState::Exploratory => "EXPLORATORY",
            FrontierState::Contracting => "CONTRACTING",
            FrontierState::Transition  => "TRANSITION",
            FrontierState::Void        => "VOID",
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// CEILING BREACH — cili tavan u tejkalua
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CeilingBreach {
    pub entropy:        bool,
    pub contradiction:  bool,
    pub recursion:      bool,
    pub semantic_drift: bool,
}

impl CeilingBreach {
    /// any — a u tejkalua ndonjë tavan. Zero if — OR boolean.
    pub fn any(&self) -> bool {
        self.entropy || self.contradiction || self.recursion || self.semantic_drift
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// CIKLI — input për scan
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
pub struct CycleResult {
    pub converged:           bool,
    pub entropy:             f32,
    pub contradiction_count: u32,
    pub recursion_depth:     u32,
    pub semantic_drift:      f32,
}

// ─────────────────────────────────────────────────────────────────────────────
// UNKNOWN BOUNDARIES — Frontier Detector
// ─────────────────────────────────────────────────────────────────────────────

pub struct UnknownBoundaries {
    pub axes:           FrontierAxes,
    pub frontier_state: FrontierState,
    pub territory:      String,
    cycles_total:       u32,
    cycles_failed:      u32,
}

impl UnknownBoundaries {
    pub fn boot(territory: &str) -> Self {
        Self {
            axes: FrontierAxes::initial(),
            frontier_state: FrontierState::Stable,
            territory: territory.to_string(),
            cycles_total: 0,
            cycles_failed: 0,
        }
    }

    /// update_from_cycle — përditëson aksjet nga rezultati.
    /// Zero if — aritmetikë boolean + lerp.
    pub fn update_from_cycle(&mut self, cycle: &CycleResult) {
        self.cycles_total += 1;
        // converged=false → cycles_failed++ (branchless).
        self.cycles_failed += (!cycle.converged) as u32;

        // unresolved_cycles: rritet nëse s'konvergoi, reset nëse po.
        // Branchless: unresolved = (unresolved + 1) * (!converged).
        let not_conv = (!cycle.converged) as u32;
        self.axes.unresolved_cycles = (self.axes.unresolved_cycles + 1) * not_conv;

        // convergence_failure_rate = failed / total.
        self.axes.convergence_failure_rate =
            self.cycles_failed as f32 / self.cycles_total as f32;

        // contradiction_density nga count (normalizuar).
        self.axes.contradiction_density = (cycle.contradiction_count as f32 / 20.0).clamp(0.0, 1.0);

        // semantic_drift — lerp drejt vlerës së re (smoothing).
        self.axes.semantic_drift_score = Self::lerp(
            self.axes.semantic_drift_score, cycle.semantic_drift, 0.30
        );

        // frontier_depth — lerp nga drift + failure rate.
        let target_depth = (self.axes.semantic_drift_score + self.axes.convergence_failure_rate) / 2.0;
        self.axes.frontier_depth = Self::lerp(self.axes.frontier_depth, target_depth, 0.25);
    }

    fn lerp(a: f32, b: f32, t: f32) -> f32 {
        a + (b - a) * t
    }

    /// check_ceiling_breach — kontrollon 4 tavanet ABSOLUTE.
    /// Zero if — krahasime boolean direkte.
    pub fn check_ceiling_breach(&self, cycle: &CycleResult) -> CeilingBreach {
        CeilingBreach {
            entropy:        cycle.entropy > ENTROPY_CEILING,
            contradiction:  self.axes.contradiction_density > CONTRADICTION_CEILING,
            recursion:      cycle.recursion_depth > RECURSION_CEILING,
            semantic_drift: self.axes.semantic_drift_score > SEMANTIC_DRIFT_CEILING,
        }
    }

    /// compute_frontier_state — gjendja nga aksjet (zero if — match guards).
    pub fn compute_frontier_state(&self, breach: &CeilingBreach) -> FrontierState {
        let breached = breach.any();
        let deep_void = self.axes.frontier_depth > 0.70;
        let exploring = self.axes.frontier_depth > 0.30;
        let transitioning = self.axes.convergence_failure_rate > 0.50;

        // Prioritet: breach → Contracting; void → Void; transition; exploring; stable.
        match (breached, deep_void, transitioning, exploring) {
            (true, _, _, _)  => FrontierState::Contracting,
            (_, true, _, _)  => FrontierState::Void,
            (_, _, true, _)  => FrontierState::Transition,
            (_, _, _, true)  => FrontierState::Exploratory,
            _                => FrontierState::Stable,
        }
    }

    /// scan — cikli i plotë: update → state → breach check.
    pub fn scan(&mut self, cycle: &CycleResult) -> BoundaryScan {
        self.update_from_cycle(cycle);
        let breach = self.check_ceiling_breach(cycle);
        self.frontier_state = self.compute_frontier_state(&breach);

        BoundaryScan {
            frontier_state:   self.frontier_state,
            ceiling_breached: breach.any(),
            phase_transition: self.axes.convergence_failure_rate > 0.90,
            frontier_depth:   self.axes.frontier_depth,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// BOUNDARY SCAN — rezultati
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
pub struct BoundaryScan {
    pub frontier_state:   FrontierState,
    pub ceiling_breached: bool,
    pub phase_transition: bool,
    pub frontier_depth:   f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cycle(converged: bool, entropy: f32, contradictions: u32, recursion: u32, drift: f32) -> CycleResult {
        CycleResult {
            converged, entropy, contradiction_count: contradictions,
            recursion_depth: recursion, semantic_drift: drift,
        }
    }

    #[test]
    fn boot_is_stable() {
        let b = UnknownBoundaries::boot("science");
        assert_eq!(b.frontier_state, FrontierState::Stable);
        assert_eq!(b.axes.frontier_depth, 0.0);
    }

    #[test]
    fn entropy_breach_detected() {
        let b = UnknownBoundaries::boot("science");
        let c = cycle(true, 0.90, 0, 3, 0.2);  // entropy > 0.85
        let breach = b.check_ceiling_breach(&c);
        assert!(breach.entropy);
        assert!(breach.any());
    }

    #[test]
    fn recursion_breach_detected() {
        let b = UnknownBoundaries::boot("science");
        let c = cycle(true, 0.5, 0, 8, 0.2);  // recursion > 7
        let breach = b.check_ceiling_breach(&c);
        assert!(breach.recursion);
    }

    #[test]
    fn breach_causes_contracting() {
        let mut b = UnknownBoundaries::boot("science");
        let c = cycle(true, 0.90, 0, 3, 0.2);  // entropy breach
        let scan = b.scan(&c);
        assert_eq!(scan.frontier_state, FrontierState::Contracting);
        assert!(scan.ceiling_breached);
    }

    #[test]
    fn unresolved_cycles_accumulate() {
        let mut b = UnknownBoundaries::boot("science");
        b.update_from_cycle(&cycle(false, 0.5, 0, 3, 0.2)); // s'konvergoi
        b.update_from_cycle(&cycle(false, 0.5, 0, 3, 0.2)); // s'konvergoi
        assert_eq!(b.axes.unresolved_cycles, 2);
        // Konvergim → reset.
        b.update_from_cycle(&cycle(true, 0.5, 0, 3, 0.2));
        assert_eq!(b.axes.unresolved_cycles, 0);
    }

    #[test]
    fn failure_rate_computed() {
        let mut b = UnknownBoundaries::boot("science");
        b.update_from_cycle(&cycle(false, 0.5, 0, 3, 0.2)); // fail
        b.update_from_cycle(&cycle(true, 0.5, 0, 3, 0.2));  // ok
        // 1 fail / 2 total = 0.5
        assert!((b.axes.convergence_failure_rate - 0.5).abs() < 1e-5);
    }

    #[test]
    fn deep_drift_leads_to_void() {
        let mut b = UnknownBoundaries::boot("science");
        // Disa cikle me drift të lartë → frontier_depth rritet.
        for _ in 0..10 {
            b.scan(&cycle(true, 0.5, 0, 3, 0.9));
        }
        // frontier_depth duhet të jetë rritur ndjeshëm.
        assert!(b.axes.frontier_depth > 0.30);
    }

    #[test]
    fn ceilings_match_theory() {
        assert_eq!(ENTROPY_CEILING, 0.85);
        assert_eq!(CONTRADICTION_CEILING, 0.75);
        assert_eq!(RECURSION_CEILING, 7);
        assert_eq!(SEMANTIC_DRIFT_CEILING, 0.80);
    }
}
