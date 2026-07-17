// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  HW_ADAPTER.RS — Ura LIM ↔ Hardware                                   ║
// ║  GJATA LEGACY™ — Quantum Platform / Hardware Bridge                   ║
// ║                                                                          ║
// ║  Përdor FORMULAT reale të hw_core.rs (territories.rs):                 ║
// ║   • DepthHint::from_mass — thresholds 0.36/0.65/0.99                  ║
// ║   • DEPTH_TO_PCT — sa % RAM bllokohet                                 ║
// ║   • ActParallel — n_parallel = floor(allocated / 64MB), clamp [1,4]   ║
// ║                                                                          ║
// ║  PA seal-et runtime (hw_lgc_open) — ato janë makineri sovrane që      ║
// ║  kërkon FFI termik; këtu vetëm matematika e budgetit (ekzekutues).     ║
// ║                                                                          ║
// ║  NDRYSHIM FILOZOFIK (sipas arkitektit):                              ║
// ║   hw_core origjinal: DEPTH_TO_PCT[Shallow] = 0.00 (zero RAM).         ║
// ║   KËTU: Shallow = 0.10 — "e thjeshta është primitive, hardware        ║
// ║   konsumohet edhe për reasoning të thjeshtë; primitivit s'i dihet".   ║
// ╚══════════════════════════════════════════════════════════════════════════╝

// ─────────────────────────────────────────────────────────────────────────────
// DEPTH HINT — identik me territories.rs
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DepthHint {
    Shallow,   // mass < 0.36
    Medium,    // mass 0.36–0.65
    Deep,      // mass 0.65–0.99
    VeryDeep,  // mass ≥ 0.99
}

impl DepthHint {
    /// Nga territories.rs: from_mass me thresholds 0.36/0.65/0.99.
    /// Branchless-friendly por kushtet janë gating (jo error).
    pub fn from_mass(mass: f32) -> Self {
        // Renditja e ruajtur: i pari true fiton. Zero if — match mbi tuple.
        match (mass >= 0.99, mass >= 0.65, mass >= 0.36) {
            (true, _, _) => DepthHint::VeryDeep,
            (_, true, _) => DepthHint::Deep,
            (_, _, true) => DepthHint::Medium,
            _            => DepthHint::Shallow,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            DepthHint::Shallow  => "SHALLOW",
            DepthHint::Medium   => "MEDIUM",
            DepthHint::Deep     => "DEEP",
            DepthHint::VeryDeep => "VERY_DEEP",
        }
    }

    fn idx(self) -> usize {
        match self {
            DepthHint::Shallow  => 0,
            DepthHint::Medium   => 1,
            DepthHint::Deep     => 2,
            DepthHint::VeryDeep => 3,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// KONSTANTET — nga hw_core.rs (me ndryshimin filozofik te Shallow)
// ─────────────────────────────────────────────────────────────────────────────

// hw_core origjinal: [0.0, 0.50, 0.80, 1.00]
// KËTU Shallow=0.10: primitivi merr budget minimal, jo zero.
const DEPTH_TO_PCT: [f32; 4] = [0.10, 0.50, 0.80, 1.00];

/// 64MB për operator — nga hw_core::COST_PER_OPERATOR_MB.
const COST_PER_OPERATOR_BYTES: u64 = 64 * 1024 * 1024;

// ─────────────────────────────────────────────────────────────────────────────
// HW BUDGET — nga hw_core::HwBudget
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct HwBudget {
    pub allocated_bytes: u64,
    pub pct_used:        f32,
    pub depth_hint:      DepthHint,
    pub available_total: u64,
}

impl HwBudget {
    /// Invariant: allocated ≤ available (nga hw_core).
    pub fn is_valid(&self) -> bool {
        self.allocated_bytes <= self.available_total
    }

    /// Thellësia hardware si faktor [0,1] — për shumëzimin me depth epistemik.
    /// = pct_used (sa % e RAM-it të disponueshëm u bllokua).
    pub fn depth_factor(&self) -> f32 {
        self.pct_used.clamp(0.0, 1.0)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// SCALE HARDUER — nga hw_core::ScaleHarduer::allocate
// ─────────────────────────────────────────────────────────────────────────────

pub struct ScaleHarduer;

impl ScaleHarduer {
    /// Alokon RAM sipas DepthHint. Formula: allocated = available × DEPTH_TO_PCT.
    /// (PA seal — vetëm matematika reale e budgetit.)
    pub fn allocate(available_bytes: u64, depth_hint: DepthHint) -> HwBudget {
        let pct = DEPTH_TO_PCT[depth_hint.idx()];
        let allocated = (available_bytes as f64 * pct as f64) as u64;
        // available==0 → 0; ndryshe → raporti. Branchless.
        let avail_nonzero = (available_bytes != 0) as u32 as f32;
        let pct_actual = avail_nonzero * (allocated as f32 / available_bytes.max(1) as f32);

        HwBudget {
            allocated_bytes: allocated,
            pct_used: pct_actual.clamp(0.0, 1.0),
            depth_hint,
            available_total: available_bytes,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ACT PARALLEL — nga hw_core::ActParallel::decide
// RAM vendos sa operatorë PRO ekzekutohen paralel.
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ParallelDecision {
    /// numri i operatorëve paralel ∈ [1, 4]
    pub n_parallel:  u8,
    /// bytes të alokuara
    pub budget_used: u64,
    /// a është kapaciteti i plotë (4 operatorë)
    pub full_parallel: bool,
}

pub struct ActParallel;

impl ActParallel {
    /// n_parallel = floor(allocated / 64MB), clamp [1, 4].
    /// Me 64MB→1 operator; me 256MB+→4 paralel.
    pub fn decide(budget: &HwBudget) -> ParallelDecision {
        // COST==0 → 4; ndryshe → allocated/COST. Branchless.
        let cost_zero = (COST_PER_OPERATOR_BYTES == 0) as u64;
        let n_raw = cost_zero * 4u64
            + (1 - cost_zero) * (budget.allocated_bytes / COST_PER_OPERATOR_BYTES.max(1));
        let n_parallel = n_raw.clamp(1, 4) as u8;

        ParallelDecision {
            n_parallel,
            budget_used: budget.allocated_bytes,
            full_parallel: n_parallel == 4,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// RESOURCE SNAPSHOT — sa RAM ka realisht
// Default: vlerë e simuluar (ARM-class). Real: do lexonte /proc/meminfo.
// ─────────────────────────────────────────────────────────────────────────────

/// Gjendja e burimeve fizike.
#[derive(Debug, Clone)]
pub struct ResourceSnapshot {
    pub ram_free_bytes: u64,
    pub energy_margin:  f32,   // [0,1] — bateria
}

impl ResourceSnapshot {
    /// TEST-ONLY (blinduar): snapshot me vlera fikse për teste njësi.
    /// Gate-uar pas #[cfg(test)] → NUK ekziston në build-in e prodhimit, pra
    /// s'mund të thirret kurrë nga rrjedha runtime. Prodhimi lexon hardware
    /// real (hw_real::ffi me feature hw_kernel). Kjo është garancia strukturore
    /// kundër simulimit: simulatori fizikisht mungon në binarin e prodhimit.
    #[cfg(test)]
    pub fn simulated() -> Self {
        Self {
            ram_free_bytes: 4 * 1024 * 1024 * 1024, // 4GB (test fixture)
            energy_margin:  1.0,
        }
    }

    /// Snapshot me vlera të dhëna (integrim me matje reale nga jashtë).
    pub fn with(ram_free_bytes: u64, energy_margin: f32) -> Self {
        Self { ram_free_bytes, energy_margin: energy_margin.clamp(0.0, 1.0) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn depth_hint_thresholds() {
        assert_eq!(DepthHint::from_mass(0.20), DepthHint::Shallow);
        assert_eq!(DepthHint::from_mass(0.50), DepthHint::Medium);
        assert_eq!(DepthHint::from_mass(0.70), DepthHint::Deep);
        assert_eq!(DepthHint::from_mass(0.99), DepthHint::VeryDeep);
    }

    #[test]
    fn shallow_gets_minimal_not_zero() {
        // Filozofia: primitivi merr budget minimal, JO zero.
        let snap = ResourceSnapshot::simulated();
        let budget = ScaleHarduer::allocate(snap.ram_free_bytes, DepthHint::Shallow);
        assert!(budget.allocated_bytes > 0, "Shallow s'duhet të jetë zero RAM");
        assert!((budget.pct_used - 0.10).abs() < 0.01);
    }

    #[test]
    fn verydeep_gets_all() {
        let snap = ResourceSnapshot::simulated();
        let budget = ScaleHarduer::allocate(snap.ram_free_bytes, DepthHint::VeryDeep);
        assert!((budget.pct_used - 1.00).abs() < 0.01);
        assert!(budget.is_valid());
    }

    #[test]
    fn parallel_scales_with_ram() {
        // 64MB → 1 operator
        let small = HwBudget {
            allocated_bytes: 64 * 1024 * 1024,
            pct_used: 0.1, depth_hint: DepthHint::Shallow,
            available_total: 640 * 1024 * 1024,
        };
        assert_eq!(ActParallel::decide(&small).n_parallel, 1);

        // 256MB → 4 operatorë
        let big = HwBudget {
            allocated_bytes: 256 * 1024 * 1024,
            pct_used: 1.0, depth_hint: DepthHint::VeryDeep,
            available_total: 256 * 1024 * 1024,
        };
        assert_eq!(ActParallel::decide(&big).n_parallel, 4);
    }
}
