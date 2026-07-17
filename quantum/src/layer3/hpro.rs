// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAYER3/HPRO.RS — Hardware Primitive Reasoning Origin (Quantum)      ║
// ║  GJATA LEGACY™ — ESS-MAI Quantum                          (LAYER 3)    ║
// ║                                                                          ║
// ║  HPRO është FAMILJE OPERATORËSH që jeton MBI harduerin dhe ushqen       ║
// ║  presionin. NUK është brenda pro.rs — është shtresë mbi motorin.       ║
// ║  Kur HPRO mbaron, e LËSHON harduerin (e lë të lirë).                   ║
// ║                                                                          ║
// ║  6 OPERATORËT:                                                         ║
// ║   • prod_formula  → formula fizike/matematike → kufijtë + shtytja       ║
// ║   • control_role  → ndan cilët parametra device duhen lartë/poshtë      ║
// ║                     (mban stabilitet, harduer s'prishet)               ║
// ║   • wcfd   → web_connection_from_device (optimalet; kufij nga prod_f.)  ║
// ║   • nwcfd  → negative_web_connection (minimalet; kufij nga prod_f.)     ║
// ║   • hwcfd  → hypothesis_web_connection (hipoteza të faktuara liter.)    ║
// ║                                                                          ║
// ║  KUFIJTË ABSOLUTË:                                                     ║
// ║   • Operatorët web janë DEVICE-MEDIATED (trait) — kurrë s'prekin web-in ║
// ║     drejtpërdrejt; device-i e bën kërkimin. wcfd/nwcfd/hwcfd marrin     ║
// ║     kufij nga prod_formula (s'dalin jashtë buxhetit fizik).            ║
// ║   • HPRO NUK vendos verdikt (Shadow vendos), vetëm ushqen presionin.   ║
// ║  Zero if/else — formula + match.                                      ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::lab_contracts_v11::hardware::{HardwareBounds, DeviceSnapshot};
use crate::hw_real::{HardwareReleaseReceipt, HwLease, HwManager, MeasurementState};

// ─────────────────────────────────────────────────────────────────────────────
// KONSTANTET — pragjet fizike (prod_formula)
// ─────────────────────────────────────────────────────────────────────────────

/// Kosto për operator (bytes) — bazë për paralelizmin.
pub const HPRO_COST_PER_OP: u64 = 64_000_000; // 64 MB
/// Marzhi minimal i energjisë për të lejuar thellim.
pub const HPRO_ENERGY_FLOOR: f32 = 0.20;
/// Headroom termik minimal për të lejuar thellim.
pub const HPRO_THERMAL_FLOOR: f32 = 0.15;
/// Raporti maksimal i RAM-it që HPRO merr (lë buffer për device).
pub const HPRO_RAM_CEILING: f32 = 0.80;

// ─────────────────────────────────────────────────────────────────────────────
// OPERATOR 1: prod_formula — kufijtë fizikë nga formula
// ─────────────────────────────────────────────────────────────────────────────

pub struct ProdFormula;

impl ProdFormula {
    /// compute_bounds — formula fizike/matematike → HardwareBounds.
    ///
    /// Nga gjendja e device-it llogarit: sa RAM, sa paralel, sa shtytje,
    /// a lejohet thellim i thellë, sa buffer të ruajë. Zero if — formula + match.
    pub fn compute_bounds(snapshot: &DeviceSnapshot) -> HardwareBounds {
        // A është e sigurt të thellohet (energ + termik)?
        let safe = snapshot.is_safe_to_deepen();

        // RAM budget: % e RAM-it të lirë, e kufizuar nga tavani (lë buffer).
        let free = snapshot.ram_free_bytes;
        let ceiling_bytes = ((free as f32) * HPRO_RAM_CEILING) as u64;
        // Nëse jo e sigurt → 0 budget (match mbi safe).
        let ram_budget_bytes = match safe {
            true  => ceiling_bytes,
            false => 0,
        };

        // Paralelizmi: floor(budget / cost_per_op), kufizuar [1,4].
        let raw_parallel = (ram_budget_bytes / HPRO_COST_PER_OP) as u8;
        let parallel_limit = raw_parallel.clamp(1, 4);

        // Shtytja e presionit: nga marzhi i energjisë + headroom termik.
        let energy = snapshot.energy_margin.clamp(0.0, 1.0);
        let thermal = snapshot.thermal_headroom();
        let raw_push = (energy * 0.6 + thermal * 0.4).clamp(0.0, 1.0);
        // Nëse jo e sigurt → 0 shtytje.
        let pressure_push = (safe as u32 as f32) * raw_push;

        // Thellim i thellë lejohet vetëm me energ të lartë + termik i mirë.
        let deep_energy = energy >= 0.50;
        let deep_thermal = thermal >= 0.40;
        let deep_allowed = safe & deep_energy & deep_thermal;

        // Marzhi i sigurisë: ç'mbetet nga tavani (buffer për device).
        let safety_margin = (1.0 - HPRO_RAM_CEILING).clamp(0.0, 1.0);

        HardwareBounds {
            ram_budget_bytes,
            parallel_limit,
            pressure_push,
            deep_allowed,
            safety_margin,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// OPERATOR 2: control_role — stabiliteti i parametrave të device-it
// ─────────────────────────────────────────────────────────────────────────────

/// Roli i një parametri device gjatë thellimit (cili lartë, cili poshtë).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamPriority {
    /// Duhet i lartë (kritik për thellim).
    High,
    /// I balancuar.
    Balanced,
    /// Duhet i ulët (për të ruajtur stabilitet).
    Low,
}

impl ParamPriority {
    pub fn label(self) -> &'static str {
        match self {
            ParamPriority::High     => "HIGH",
            ParamPriority::Balanced => "BALANCED",
            ParamPriority::Low      => "LOW",
        }
    }
}

/// Vendimi i stabilitetit — cili parametër sa prioritet.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StabilityPlan {
    pub cpu_priority:    ParamPriority,
    pub ram_priority:    ParamPriority,
    pub thermal_guard:   ParamPriority,  // sa fort ruhet termiku
    pub safe_to_proceed: bool,
}

pub struct ControlRole;

impl ControlRole {
    /// plan — ndan cilët parametra duhen lartë/poshtë për të mos prishur device.
    ///
    /// Nëse termiku afër kufirit → thermal_guard High (mbron device).
    /// Nëse RAM e ulët → ram_priority High. Zero if — match mbi gjendjet.
    pub fn plan(snapshot: &DeviceSnapshot) -> StabilityPlan {
        let thermal_tight = snapshot.thermal_headroom() < 0.30;
        let ram_tight = snapshot.ram_free_ratio() < 0.30;
        let cpu_high = snapshot.cpu_load > 0.70;

        // Thermal guard: i lartë nëse termiku i ngushtë.
        let thermal_guard = match thermal_tight {
            true  => ParamPriority::High,
            false => ParamPriority::Balanced,
        };

        // RAM priority: i lartë nëse RAM e ngushtë.
        let ram_priority = match ram_tight {
            true  => ParamPriority::High,
            false => ParamPriority::Balanced,
        };

        // CPU priority: i ulët nëse CPU tashmë e ngarkuar (ruaj stabilitet).
        let cpu_priority = match cpu_high {
            true  => ParamPriority::Low,
            false => ParamPriority::Balanced,
        };

        // Vazhdohet vetëm nëse jo termik kritik dhe device i sigurt.
        let safe_to_proceed = snapshot.is_safe_to_deepen() & !thermal_tight;

        StabilityPlan {
            cpu_priority,
            ram_priority,
            thermal_guard,
            safe_to_proceed,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// OPERATORËT 3-5: web device-mediated (wcfd / nwcfd / hwcfd)
// ─────────────────────────────────────────────────────────────────────────────

/// Lloji i kërkimit web (përmes device, kurrë drejtpërdrejt).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebProbeKind {
    /// Optimalet (wcfd).
    Optimal,
    /// Minimalet (nwcfd).
    Minimal,
    /// Hipoteza të faktuara literaturisht (hwcfd).
    Hypothesis,
}

impl WebProbeKind {
    pub fn label(self) -> &'static str {
        match self {
            WebProbeKind::Optimal    => "WCFD_OPTIMAL",
            WebProbeKind::Minimal    => "NWCFD_MINIMAL",
            WebProbeKind::Hypothesis => "HWCFD_HYPOTHESIS",
        }
    }
}

/// Kërkesa për kërkim web device-mediated — me kufij nga prod_formula.
#[derive(Debug, Clone, PartialEq)]
pub struct WebProbeRequest {
    pub claim_id:    u64,
    pub kind:        WebProbeKind,
    pub query:       String,
    /// Kufiri i burimeve (nga prod_formula — s'del jashtë buxhetit).
    pub source_cap:  u32,
    /// A lejohet (varet nga kufijtë e hardware).
    pub permitted:   bool,
}

/// INTERFEJSI device-mediated — device-i e implementon kërkimin REAL.
/// Quantum KURRË s'prek web-in drejtpërdrejt; vetëm dërgon kërkesën te device.
pub trait DeviceWebProbe {
    /// probe — device-i kryen kërkimin dhe kthen numrin e rezultateve.
    /// (Implementimi real bëhet nga device-i kur sistemi instalohet.)
    fn probe(&self, request: &WebProbeRequest) -> u32;
}

pub struct WebConnector;

impl WebConnector {
    /// build_request — ndërton kërkesë web me kufij nga prod_formula.
    ///
    /// source_cap derivohet nga shtytja e lejuar (bounds.pressure_push):
    /// më shumë shtytje → më shumë burime të lejuara. Zero if — formula + match.
    pub fn build_request(
        claim_id: u64,
        kind: WebProbeKind,
        query: String,
        bounds: &HardwareBounds,
    ) -> WebProbeRequest {
        // Kufiri i burimeve nga shtytja (0..=10 burime).
        let cap_raw = (bounds.pressure_push * 10.0) as u32;
        let source_cap = cap_raw.clamp(0, 10);

        // Lejohet vetëm nëse kufijtë lejojnë thellim.
        let permitted = bounds.allows_deepening();

        WebProbeRequest {
            claim_id,
            kind,
            query,
            source_cap,
            permitted,
        }
    }

    /// execute_via_device — dërgon kërkesën te device (kurrë web i drejtpërdrejtë).
    /// Kthen numrin e rezultateve. Zero if — match mbi lejimin.
    pub fn execute_via_device<P: DeviceWebProbe>(
        request: &WebProbeRequest,
        device: &P,
    ) -> u32 {
        // Vetëm nëse lejuar → device kërkon; ndryshe 0.
        match request.permitted {
            true  => device.probe(request),
            false => 0,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// HPRO ENGINE — orkestron operatorët, lëshon harduerin në fund
// ─────────────────────────────────────────────────────────────────────────────

/// Rezultati i plotë i HPRO — kufijtë, plani dhe lease-i aktiv i workload-it.
#[derive(Debug, Clone)]
pub struct HproResult {
    pub bounds: HardwareBounds,
    pub stability: StabilityPlan,
    pub lease: HwLease,
    pub measurement_state: MeasurementState,
    pub release: Option<HardwareReleaseReceipt>,
    /// Fushë compatibility: derivon vetëm nga receipt-i real, kurrë literal.
    pub hardware_released: bool,
}

impl HproResult {
    /// Sinkronizon kufijtë publikë të HPRO me lease-in që HCP_PRO aplikoi.
    /// Kjo e bën HPIM/Layer3 të konsumojë grantën reale, jo kërkesën fillestare.
    pub fn apply_lease(&mut self, lease: HwLease) {
        self.lease = lease;
        self.bounds.ram_budget_bytes = lease.granted_bytes;
        self.bounds.parallel_limit =
            u8::try_from(lease.granted_parallel.min(4)).unwrap_or(4);
        match lease.granted_bytes > 0 {
            true => {}
            false => {
                self.bounds.pressure_push = 0.0;
                self.bounds.deep_allowed = false;
            }
        }
    }

    pub fn mark_released(&mut self, receipt: HardwareReleaseReceipt) {
        self.hardware_released = receipt.completed;
        self.release = Some(receipt);
        self.lease.active = false;
    }
}

pub struct HproEngine;

impl HproEngine {
    /// acquire — prod_formula + control_role + lease real nga HwManager.
    ///
    /// Lease-i kontrollon punën e PRO; HPRO nuk e lëshon para konsumimit.
    pub fn acquire(
        snapshot: &DeviceSnapshot,
        measurement_state: MeasurementState,
    ) -> HproResult {
        let mut bounds = ProdFormula::compute_bounds(snapshot);
        let stability = ControlRole::plan(snapshot);
        let lease = HwManager::govern(
            snapshot.ram_total_bytes,
            snapshot.ram_free_bytes,
            bounds.ram_budget_bytes,
            snapshot.cores_active.max(1),
            u32::from(bounds.effective_parallel()),
        );

        // Kufijtë publikë pasqyrojnë grantën reale, jo vetëm kërkesën teorike.
        bounds.ram_budget_bytes = lease.granted_bytes;
        bounds.parallel_limit = u8::try_from(lease.granted_parallel.min(4)).unwrap_or(4);
        match (lease.granted_bytes > 0, measurement_state.has_real_measurement()) {
            (true, true) => {}
            _ => {
                bounds.pressure_push = 0.0;
                bounds.deep_allowed = false;
            }
        }

        HproResult {
            bounds,
            stability,
            lease,
            measurement_state,
            release: None,
            hardware_released: false,
        }
    }

    /// run — compatibility për rrjedhat historike/testet: merr lease dhe e
    /// lëshon menjëherë sepse nuk ka workload consumer në këtë API.
    pub fn run(snapshot: &DeviceSnapshot) -> HproResult {
        let state = match snapshot.timestamp {
            0 => MeasurementState::NominalFallback,
            _ => MeasurementState::Measured,
        };
        let mut result = Self::acquire(snapshot, state);
        let released = result.lease.release();
        let receipt = HardwareReleaseReceipt {
            cycle_id: snapshot.timestamp,
            lease_id: result.lease.lease_id,
            workers_joined: 0,
            buffers_released: 0,
            buffer_bytes_released: 0,
            lease_released: released,
            completed: released,
            reason: "hpro_compatibility_no_work",
        };
        result.mark_released(receipt);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strong_device() -> DeviceSnapshot {
        DeviceSnapshot {
            ram_free_bytes: 4_000_000_000, ram_total_bytes: 8_000_000_000,
            cpu_load: 0.3, cores_active: 4, cores_total: 8,
            energy_margin: 0.9, on_battery: false,
            temp_c: 40.0, temp_limit_c: 90.0, timestamp: 100,
        }
    }

    fn weak_device() -> DeviceSnapshot {
        let mut d = DeviceSnapshot::idle();
        d.ram_free_bytes = 100_000_000;
        d.ram_total_bytes = 2_000_000_000;
        d.energy_margin = 0.1;  // nën floor
        d.temp_c = 85.0;
        d.temp_limit_c = 90.0;
        d
    }

    #[test]
    fn prod_formula_strong_device_allows() {
        let bounds = ProdFormula::compute_bounds(&strong_device());
        assert!(bounds.allows_deepening());
        assert!(bounds.ram_budget_bytes > 0);
        assert!(bounds.pressure_push > 0.0);
        assert!(bounds.deep_allowed); // energ 0.9 + termik i mirë
    }

    #[test]
    fn prod_formula_weak_device_blocks() {
        let bounds = ProdFormula::compute_bounds(&weak_device());
        assert!(!bounds.allows_deepening()); // jo e sigurt
        assert_eq!(bounds.ram_budget_bytes, 0);
        assert_eq!(bounds.pressure_push, 0.0);
    }

    #[test]
    fn prod_formula_keeps_safety_buffer() {
        let bounds = ProdFormula::compute_bounds(&strong_device());
        // RAM budget s'kalon 80% të lirë → buffer i ruajtur.
        let max_allowed = ((4_000_000_000u64 as f32) * 0.80) as u64;
        assert!(bounds.ram_budget_bytes <= max_allowed);
        assert!((bounds.safety_margin - 0.20).abs() < 1e-6);
    }

    #[test]
    fn control_role_thermal_guard() {
        // Device me termik të ngushtë → thermal_guard High.
        let mut d = strong_device();
        d.temp_c = 85.0; // afër kufirit 90
        let plan = ControlRole::plan(&d);
        assert_eq!(plan.thermal_guard, ParamPriority::High);
        assert!(!plan.safe_to_proceed); // termik kritik
    }

    #[test]
    fn control_role_balanced_when_healthy() {
        let plan = ControlRole::plan(&strong_device());
        assert_eq!(plan.thermal_guard, ParamPriority::Balanced);
        assert!(plan.safe_to_proceed);
    }

    #[test]
    fn web_request_bounded_by_prod_formula() {
        let bounds = ProdFormula::compute_bounds(&strong_device());
        let req = WebConnector::build_request(
            1, WebProbeKind::Optimal, "best material".to_string(), &bounds,
        );
        assert!(req.permitted); // kufijtë lejojnë
        assert!(req.source_cap <= 10);
    }

    #[test]
    fn web_request_blocked_on_weak_device() {
        let bounds = ProdFormula::compute_bounds(&weak_device());
        let req = WebConnector::build_request(
            1, WebProbeKind::Optimal, "q".to_string(), &bounds,
        );
        assert!(!req.permitted); // kufijtë s'lejojnë
    }

    // Mock device për test (implementon DeviceWebProbe).
    struct MockDevice { results: u32 }
    impl DeviceWebProbe for MockDevice {
        fn probe(&self, _request: &WebProbeRequest) -> u32 {
            self.results
        }
    }

    #[test]
    fn execute_via_device_respects_permission() {
        let bounds = ProdFormula::compute_bounds(&strong_device());
        let req = WebConnector::build_request(1, WebProbeKind::Optimal, "q".to_string(), &bounds);
        let device = MockDevice { results: 5 };
        let n = WebConnector::execute_via_device(&req, &device);
        assert_eq!(n, 5); // device kërkoi (i lejuar)
    }

    #[test]
    fn execute_blocked_returns_zero() {
        let bounds = ProdFormula::compute_bounds(&weak_device());
        let req = WebConnector::build_request(1, WebProbeKind::Minimal, "q".to_string(), &bounds);
        let device = MockDevice { results: 5 };
        let n = WebConnector::execute_via_device(&req, &device);
        assert_eq!(n, 0); // s'u lejua → 0
    }

    #[test]
    fn hpro_releases_hardware() {
        let result = HproEngine::run(&strong_device());
        assert!(result.hardware_released); // HPRO lëshon harduerin
        assert!(result.bounds.allows_deepening());
    }

    #[test]
    fn hpro_weak_device_minimal() {
        let result = HproEngine::run(&weak_device());
        assert!(result.hardware_released); // gjithmonë lëshon
        assert!(!result.bounds.allows_deepening());
    }

    #[test]
    fn nominal_fallback_cannot_claim_deepening() {
        let result = HproEngine::acquire(
            &strong_device(),
            MeasurementState::NominalFallback,
        );
        assert!(!result.bounds.deep_allowed);
        assert_eq!(result.bounds.pressure_push, 0.0);
        assert!(result.lease.active);
        assert!(!result.hardware_released);
    }

    #[test]
    fn acquire_returns_active_lease_until_runtime_releases_it() {
        let result = HproEngine::acquire(&strong_device(), MeasurementState::Measured);
        assert!(result.lease.active);
        assert!(result.release.is_none());
        assert!(!result.hardware_released);
    }

}
