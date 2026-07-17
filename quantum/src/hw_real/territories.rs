// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  resource_territory.rs — Hapi 7                                        ║
// ║  reserve_territory.rs  — Hapi 8                                        ║
// ║  pressure_territory.rs — Hapi 9                                        ║
// ║  GJATA LEGACY™ — Quantum Platform                                     ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::hw_real::ffi::{
    CHwSnapshot, HwLgcError,
    S_RESOURCE_TERRITORY, S_RESERVE_TERRITORY, S_PRESSURE_TERRITORY,
    hw_lgc_open, kernel_hw_snapshot,
};

// ════════════════════════════════════════════════════════════════════════════
// RESOURCE TERRITORY — Hapi 7
// Triggered nga LIM kur hyn në deep reasoning.
// Merr 1 snapshot — ephemeral, vdes me ciklin.
// ════════════════════════════════════════════════════════════════════════════

/// Sinjali nga LIM që tregon thellësinë e reasoning-ut
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DepthHint {
    Shallow,   // mass < 0.36  — hardware gate nuk aktivizohet
    Medium,    // mass 0.36–0.65
    Deep,      // mass 0.65–0.99
    VeryDeep,  // mass ≥ 0.99 (FactCandidate)
}

impl DepthHint {
    /// Krijon DepthHint nga epistemic mass e LIM
    pub fn from_mass(mass: f32) -> Self {
        match mass {
            m if m >= 0.99 => DepthHint::VeryDeep,
            m if m >= 0.65 => DepthHint::Deep,
            m if m >= 0.36 => DepthHint::Medium,
            _              => DepthHint::Shallow,
        }
    }

    /// A kërkon aktivizimin e hardware gate?
    /// Vetëm Deep dhe VeryDeep e aktivizojnë
    pub fn requires_hw_gate(self) -> bool {
        matches!(self, DepthHint::Deep | DepthHint::VeryDeep)
    }

    /// A është reasoning-u i thellë? (Deep ose VeryDeep) — zero if.
    pub fn is_deep(self) -> bool {
        matches!(self, DepthHint::Deep | DepthHint::VeryDeep)
    }

    /// Emri i nivelit — për shfaqje/raport.
    pub fn name(self) -> &'static str {
        match self {
            DepthHint::Shallow  => "SHALLOW",
            DepthHint::Medium   => "MEDIUM",
            DepthHint::Deep     => "DEEP",
            DepthHint::VeryDeep => "VERY_DEEP",
        }
    }
}

/// Snapshot i paprocessuar i harduerit — output nga Resource_Territory
#[derive(Debug, Clone)]
pub struct ResourceSnapshot {
    pub inner:       CHwSnapshot,   // raw C snapshot
    pub depth_hint:  DepthHint,     // sinjali nga LIM
    pub ts_ns:       u64,
}

impl ResourceSnapshot {
    /// ram_free / ram_total ∈ [0.0, 1.0]
    pub fn ram_free_ratio(&self) -> f32 {
        match self.inner.ram_total_bytes == 0 {
            true => return 0.0,
            false => {}
        }
        let r = self.inner.ram_free_bytes as f32
              / self.inner.ram_total_bytes as f32;
        r.clamp(0.0, 1.0)
    }

    pub fn ram_free_bytes(&self) -> u64  { self.inner.ram_free_bytes }
    pub fn ram_total_bytes(&self) -> u64 { self.inner.ram_total_bytes }
    pub fn cpu_load_pct(&self) -> f32    { self.inner.cpu_load_pct }
    pub fn cores_active(&self) -> u32   { self.inner.cores_active }
    pub fn energy_margin(&self) -> f32  { self.inner.energy_margin_pct }
}

/// Resource_Territory modul
pub struct ResourceTerritory;

impl ResourceTerritory {
    /// Merr snapshot të gjendjes reale hardware.
    /// Kjo funksion thirret nga Orchestrator kur LIM sinjalizon deep reasoning.
    ///
    /// [in]  depth_hint — sinjal nga LIM (mass-based)
    /// [ret] ResourceSnapshot ose HwLgcError
    pub fn snapshot(depth_hint: DepthHint) -> Result<ResourceSnapshot, HwLgcError> {
        let tok = hw_lgc_open(S_RESOURCE_TERRITORY, S_RESOURCE_TERRITORY)?;
        let inner = kernel_hw_snapshot(&tok)?;
        Ok(ResourceSnapshot {
            ts_ns: inner.ts_ns,
            inner,
            depth_hint,
        })
    }
}

// ════════════════════════════════════════════════════════════════════════════
// RESERVE TERRITORY — Hapi 8
//
// Merr ResourceSnapshot.
// Formula:
//   free_ram          = snapshot.ram_free_bytes
//   safety_buffer     = free_ram × (1 - RESERVE_RATIO)   [20-25% e lirë]
//   system_reserved   = free_ram × RESERVE_RATIO          [75-80% për sistem]
//
// RESERVE_RATIO ∈ [0.75, 0.80] sipas DepthHint:
//   Medium   → 0.75
//   Deep     → 0.78
//   VeryDeep → 0.80
// ════════════════════════════════════════════════════════════════════════════

/// Territori maksimal që sistemi mund të përdorë
#[derive(Debug, Clone)]
pub struct ReserveSnapshot {
    /// bytes të rezervuara për ESS-MAI [75-80% e free_ram]
    pub system_reserved_bytes: u64,
    /// bytes të lira për device — E PAPREKSHME [20-25% e free_ram]
    pub safety_buffer_bytes:   u64,
    /// raporti i rezervimit i aplikuar ∈ [0.75, 0.80]
    pub reserve_ratio:         f32,
    /// referenca tek ResourceSnapshot
    pub free_ram_origin:       u64,
    pub depth_hint:            DepthHint,
    pub ts_ns:                 u64,
}

impl ReserveSnapshot {
    /// system_reserved / total_free ∈ [0.75, 0.80]
    pub fn efficiency(&self) -> f32 {
        match self.free_ram_origin == 0 {
            true => return 0.0,
            false => {}
        }
        (self.system_reserved_bytes as f32 / self.free_ram_origin as f32)
            .clamp(0.0, 1.0)
    }
}

pub struct ReserveTerritory;

impl ReserveTerritory {
    /// RESERVE_RATIO sipas thellësisë
    fn reserve_ratio(hint: DepthHint) -> f32 {
        match hint {
            DepthHint::Shallow  => 0.0,   // nuk aktivizohet kurrë
            DepthHint::Medium   => 0.75,
            DepthHint::Deep     => 0.78,
            DepthHint::VeryDeep => 0.80,
        }
    }

    /// Aplikon rezervimin mbi ResourceSnapshot
    pub fn apply(snap: &ResourceSnapshot) -> Result<ReserveSnapshot, HwLgcError> {
        let tok = hw_lgc_open(S_RESERVE_TERRITORY, S_RESERVE_TERRITORY)?;
        let _ = tok; // token i konsumar: provon sovereignty

        let free_ram = snap.ram_free_bytes();
        let ratio    = Self::reserve_ratio(snap.depth_hint);

        // Invariant: ratio ∈ [0.75, 0.80] për reasoning i thellë
        debug_assert!(
            ratio == 0.0 || (ratio >= 0.75 && ratio <= 0.80),
            "RESERVE_RATIO invariant violated: {ratio}"
        );

        let system_reserved = (free_ram as f64 * ratio as f64) as u64;
        let safety_buffer   = free_ram.saturating_sub(system_reserved);

        Ok(ReserveSnapshot {
            system_reserved_bytes: system_reserved,
            safety_buffer_bytes:   safety_buffer,
            reserve_ratio:         ratio,
            free_ram_origin:       free_ram,
            depth_hint:            snap.depth_hint,
            ts_ns:                 snap.inner.ts_ns,
        })
    }
}

// ════════════════════════════════════════════════════════════════════════════
// PRESSURE TERRITORY — Hapi 9
//
// Mban hapësirën e rezervuar pa ndërhyrje nga jashtë.
// DINAMIKE: kur reasoning nuk ka nevojë për gjithçka →
//   lëshon exces → parandalon plotësimin e safety_buffer.
//
// Formula dinamike:
//   current_demand   = depth_demand_bytes(depth_hint)
//   if current_demand < system_reserved:
//       releasable   = (system_reserved - current_demand) × RELEASE_FACTOR
//       locked_bytes = system_reserved - releasable
//   else:
//       locked_bytes = system_reserved   [nevojitet e gjitha]
//
// RELEASE_FACTOR = 0.50 (lëshon gjysmën e excessit)
// ════════════════════════════════════════════════════════════════════════════

const PRESSURE_RELEASE_FACTOR: f32 = 0.50;

/// Demand estimation per depth level (bytes)
/// Vlera tipike — kalibrohen sipas workload real
const DEPTH_DEMAND_MB: [u64; 4] = [
    0,       // Shallow — nuk aktivizohet
    256,     // Medium  — 256MB
    512,     // Deep    — 512MB
    1024,    // VeryDeep — 1GB
];

fn depth_demand_bytes(hint: DepthHint) -> u64 {
    let idx = match hint {
        DepthHint::Shallow  => 0,
        DepthHint::Medium   => 1,
        DepthHint::Deep     => 2,
        DepthHint::VeryDeep => 3,
    };
    DEPTH_DEMAND_MB[idx] * 1024 * 1024
}

/// Pressure envelope — hapësira dinamike për reasoning
#[derive(Debug, Clone)]
pub struct PressureEnvelope {
    /// bytes të kyçura për reasoning (minimumi i garantuar)
    pub locked_bytes:     u64,
    /// bytes të lëshura për aplikacionet e tjera
    pub released_bytes:   u64,
    /// bytes totale të rezervuara (locked + released)
    pub total_reserved:   u64,
    /// demand aktual i reasoning-ut
    pub demand_bytes:     u64,
    /// nëse sistemi është nën presion (demand ≈ reserved)
    pub under_pressure:   bool,
    pub depth_hint:       DepthHint,
    pub ts_ns:            u64,
}

impl PressureEnvelope {
    /// Raporti i shfrytëzimit ∈ [0.0, 1.0]
    pub fn utilization(&self) -> f32 {
        match self.total_reserved == 0 {
            true => return 0.0,
            false => {}
        }
        (self.locked_bytes as f32 / self.total_reserved as f32).clamp(0.0, 1.0)
    }

    /// bytes të disponueshme për reasoning
    pub fn available_bytes(&self) -> u64 { self.locked_bytes }
}

pub struct PressureTerritory;

impl PressureTerritory {
    /// Krijon PressureEnvelope nga ReserveSnapshot
    pub fn prepare(reserve: &ReserveSnapshot) -> Result<PressureEnvelope, HwLgcError> {
        let tok = hw_lgc_open(S_PRESSURE_TERRITORY, S_PRESSURE_TERRITORY)?;
        let _ = tok;

        let total     = reserve.system_reserved_bytes;
        let demand    = depth_demand_bytes(reserve.depth_hint);

        let (locked, released) = match demand >= total {
            // nevojitet e gjitha — asgjë nuk lëshohet
            true => (total, 0u64),
            false => {
            let excess     = total.saturating_sub(demand);
            let releasable = (excess as f64 * PRESSURE_RELEASE_FACTOR as f64) as u64;
            let locked     = total.saturating_sub(releasable);
            (locked, releasable)
            }
        };

        // under_pressure kur demand > 90% e locked
        let under_pressure = demand > locked.saturating_mul(9) / 10;

        Ok(PressureEnvelope {
            locked_bytes:   locked,
            released_bytes: released,
            total_reserved: total,
            demand_bytes:   demand,
            under_pressure,
            depth_hint:     reserve.depth_hint,
            ts_ns:          reserve.ts_ns,
        })
    }
}
