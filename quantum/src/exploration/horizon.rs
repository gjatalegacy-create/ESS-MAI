// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  EXPLORATION/HORIZON.RS — Horizon Topology + CognitiveSignature       ║
// ║  GJATA LEGACY™ — Quantum Platform (Teorite 11, 13)                    ║
// ║                                                                          ║
// ║  Kufiri njohës s'është sferë — është manifold me 7 akse që deformohet ║
// ║  dinamikisht. Horizoni adaptohet sipas SI mendon përdoruesi           ║
// ║  (CognitiveSignature), jo çfarë kërkon.                              ║
// ║                                                                          ║
// ║  6 signatures: SHALLOW→FRONTIER, secila me multiplikues të vet.        ║
// ║  Aksjet kurrë s'dalin nga [FLOOR, CEILING] (clamp_to_bounds).         ║
// ║                                                                          ║
// ║  Pseudo përdor IF/MATCH; KONVERTUAR plotësisht në match (zero if/else).║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::control::pressure_states::PressureState;

// ─────────────────────────────────────────────────────────────────────────────
// 7 AKSET — defaults / ceilings / floors (identike me pseudo)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
pub struct HorizonAxes {
    pub lineage_distance:        f32,
    pub contradiction_tolerance: f32,
    pub novelty_radius:          f32,
    pub operator_depth:          f32,  // f32 për multiplikim; clamp në u32 jashtë
    pub cross_territory_limit:   f32,
    pub pressure_escape_window:  f32,
    pub anomaly_allowance:       f32,
}

impl HorizonAxes {
    /// DEFAULTS — konservativë.
    pub const DEFAULTS: HorizonAxes = HorizonAxes {
        lineage_distance: 0.40, contradiction_tolerance: 0.25,
        novelty_radius: 0.35, operator_depth: 3.0,
        cross_territory_limit: 2.0, pressure_escape_window: 0.15,
        anomaly_allowance: 0.10,
    };

    /// CEILINGS — kurrë mbi këto.
    pub const CEILINGS: HorizonAxes = HorizonAxes {
        lineage_distance: 0.90, contradiction_tolerance: 0.70,
        novelty_radius: 0.80, operator_depth: 7.0,
        cross_territory_limit: 4.0, pressure_escape_window: 0.40,
        anomaly_allowance: 0.35,
    };

    /// FLOORS — kurrë nën këto.
    pub const FLOORS: HorizonAxes = HorizonAxes {
        lineage_distance: 0.10, contradiction_tolerance: 0.05,
        novelty_radius: 0.10, operator_depth: 1.0,
        cross_territory_limit: 0.0, pressure_escape_window: 0.05,
        anomaly_allowance: 0.02,
    };

    /// clamp_to_bounds — çdo aks në [FLOOR, CEILING]. Zero if — clamp aritmetik.
    pub fn clamp_to_bounds(&mut self) {
        let f = &Self::FLOORS;
        let c = &Self::CEILINGS;
        self.lineage_distance = self.lineage_distance.clamp(f.lineage_distance, c.lineage_distance);
        self.contradiction_tolerance = self.contradiction_tolerance.clamp(f.contradiction_tolerance, c.contradiction_tolerance);
        self.novelty_radius = self.novelty_radius.clamp(f.novelty_radius, c.novelty_radius);
        self.operator_depth = self.operator_depth.clamp(f.operator_depth, c.operator_depth);
        self.cross_territory_limit = self.cross_territory_limit.clamp(f.cross_territory_limit, c.cross_territory_limit);
        self.pressure_escape_window = self.pressure_escape_window.clamp(f.pressure_escape_window, c.pressure_escape_window);
        self.anomaly_allowance = self.anomaly_allowance.clamp(f.anomaly_allowance, c.anomaly_allowance);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// COGNITIVE SIGNATURE — SI mendon përdoruesi
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CognitiveSignature {
    Shallow,      // mendim lineal, pak thellësi
    Structural,   // mendim sistematik, kërkon lidhje
    Analytical,   // mendim analitik, kërkon prova
    Scientific,   // mendim shkencor, toleron kontradiktë
    Frontier,     // mendim i frontierit, zbulim aktiv
    Engineering,  // mendim teknik, precision mbi novelty
}

impl CognitiveSignature {
    pub fn label(self) -> &'static str {
        match self {
            CognitiveSignature::Shallow     => "SHALLOW",
            CognitiveSignature::Structural  => "STRUCTURAL",
            CognitiveSignature::Analytical  => "ANALYTICAL",
            CognitiveSignature::Scientific  => "SCIENTIFIC",
            CognitiveSignature::Frontier    => "FRONTIER",
            CognitiveSignature::Engineering => "ENGINEERING",
        }
    }

    /// cot_depth — thellësia CoT (2 shallow → 7 frontier).
    pub fn cot_depth(self) -> u32 {
        match self {
            CognitiveSignature::Shallow     => 2,
            CognitiveSignature::Structural  => 3,
            CognitiveSignature::Analytical  => 4,
            CognitiveSignature::Engineering => 4,
            CognitiveSignature::Scientific  => 5,
            CognitiveSignature::Frontier    => 7,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// HORIZON MULTIPLIERS — si modifikohen aksjet (identike me pseudo)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
pub struct HorizonMultipliers {
    pub lineage_distance_mult:        f32,
    pub contradiction_tolerance_mult: f32,
    pub novelty_radius_mult:          f32,
    pub operator_depth_mult:          f32,
    pub cross_territory_mult:         f32,
}

impl HorizonMultipliers {
    /// for_signature — multiplikuesit nga pseudo (zero if — match).
    pub fn for_signature(sig: CognitiveSignature) -> Self {
        match sig {
            CognitiveSignature::Shallow => Self {
                lineage_distance_mult: 0.60, contradiction_tolerance_mult: 0.50,
                novelty_radius_mult: 0.55, operator_depth_mult: 0.60,
                cross_territory_mult: 0.50,
            },
            CognitiveSignature::Structural => Self {
                lineage_distance_mult: 0.80, contradiction_tolerance_mult: 0.70,
                novelty_radius_mult: 0.70, operator_depth_mult: 0.80,
                cross_territory_mult: 0.75,
            },
            CognitiveSignature::Analytical => Self {
                lineage_distance_mult: 0.85, contradiction_tolerance_mult: 0.80,
                novelty_radius_mult: 0.75, operator_depth_mult: 0.90,
                cross_territory_mult: 0.80,
            },
            CognitiveSignature::Scientific => Self {
                lineage_distance_mult: 1.10, contradiction_tolerance_mult: 1.20,
                novelty_radius_mult: 1.15, operator_depth_mult: 1.10,
                cross_territory_mult: 1.10,
            },
            CognitiveSignature::Frontier => Self {
                lineage_distance_mult: 1.30, contradiction_tolerance_mult: 1.40,
                novelty_radius_mult: 1.35, operator_depth_mult: 1.20,
                cross_territory_mult: 1.30,
            },
            CognitiveSignature::Engineering => Self {
                lineage_distance_mult: 0.90, contradiction_tolerance_mult: 0.75,
                novelty_radius_mult: 0.85, operator_depth_mult: 1.00,
                cross_territory_mult: 0.80,
            },
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// EXPLORATION HORIZON — manifold deformueshëm
// ─────────────────────────────────────────────────────────────────────────────

pub struct ExplorationHorizon {
    pub axes:             HorizonAxes,
    pub signature:        CognitiveSignature,
    pub territory:        String,
    pub current_pressure: PressureState,
    pub deform_count:     u32,
}

impl ExplorationHorizon {
    /// boot — inicializim me defaults (LGC verify bëhet jashtë).
    pub fn boot(territory: &str) -> Self {
        Self {
            axes: HorizonAxes::DEFAULTS,
            signature: CognitiveSignature::Shallow,  // default konservativ
            territory: territory.to_string(),
            current_pressure: PressureState::Low,
            deform_count: 0,
        }
    }

    /// adapt — aplikon multiplikuesit e signature mbi aksjet, pastaj clamp.
    /// Zero if — multiplikim direkt + clamp_to_bounds.
    pub fn adapt(&mut self, signature: CognitiveSignature) {
        let m = HorizonMultipliers::for_signature(signature);
        let base = HorizonAxes::DEFAULTS;

        // Apliko mbi DEFAULTS (jo mbi aktualin — që adaptimi të jetë deterministik).
        self.axes.lineage_distance = base.lineage_distance * m.lineage_distance_mult;
        self.axes.contradiction_tolerance = base.contradiction_tolerance * m.contradiction_tolerance_mult;
        self.axes.novelty_radius = base.novelty_radius * m.novelty_radius_mult;
        self.axes.operator_depth = base.operator_depth * m.operator_depth_mult;
        self.axes.cross_territory_limit = base.cross_territory_limit * m.cross_territory_mult;

        // Aksjet kurrë jashtë kufijve.
        self.axes.clamp_to_bounds();
        self.signature = signature;
        self.deform_count += 1;
    }

    /// operator_depth_int — thellësia si u32 (për rekursion).
    pub fn operator_depth_int(&self) -> u32 {
        self.axes.operator_depth.round() as u32
    }

    /// allows_exploration — novelty_radius mbi një prag minimal.
    pub fn allows_exploration(&self, required_novelty: f32) -> bool {
        self.axes.novelty_radius >= required_novelty
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boot_uses_defaults() {
        let h = ExplorationHorizon::boot("science");
        assert!((h.axes.lineage_distance - 0.40).abs() < 1e-5);
        assert_eq!(h.signature, CognitiveSignature::Shallow);
    }

    #[test]
    fn shallow_narrows_horizon() {
        let mut h = ExplorationHorizon::boot("science");
        h.adapt(CognitiveSignature::Shallow);
        // lineage 0.40 * 0.60 = 0.24
        assert!((h.axes.lineage_distance - 0.24).abs() < 1e-4);
    }

    #[test]
    fn frontier_expands_horizon() {
        let mut h = ExplorationHorizon::boot("science");
        h.adapt(CognitiveSignature::Frontier);
        // lineage 0.40 * 1.30 = 0.52
        assert!((h.axes.lineage_distance - 0.52).abs() < 1e-4);
        // contradiction 0.25 * 1.40 = 0.35
        assert!((h.axes.contradiction_tolerance - 0.35).abs() < 1e-4);
    }

    #[test]
    fn axes_never_exceed_ceiling() {
        let mut h = ExplorationHorizon::boot("science");
        // Frontier shumëfish → por clamp duhet ta mbajë nën ceiling.
        h.adapt(CognitiveSignature::Frontier);
        assert!(h.axes.lineage_distance <= HorizonAxes::CEILINGS.lineage_distance);
        assert!(h.axes.contradiction_tolerance <= HorizonAxes::CEILINGS.contradiction_tolerance);
        assert!(h.axes.operator_depth <= HorizonAxes::CEILINGS.operator_depth);
    }

    #[test]
    fn axes_never_below_floor() {
        let mut h = ExplorationHorizon::boot("science");
        h.adapt(CognitiveSignature::Shallow);
        assert!(h.axes.lineage_distance >= HorizonAxes::FLOORS.lineage_distance);
        assert!(h.axes.operator_depth >= HorizonAxes::FLOORS.operator_depth);
    }

    #[test]
    fn cot_depth_scales_with_signature() {
        assert_eq!(CognitiveSignature::Shallow.cot_depth(), 2);
        assert_eq!(CognitiveSignature::Frontier.cot_depth(), 7);
    }

    #[test]
    fn deform_count_increments() {
        let mut h = ExplorationHorizon::boot("science");
        h.adapt(CognitiveSignature::Scientific);
        h.adapt(CognitiveSignature::Frontier);
        assert_eq!(h.deform_count, 2);
    }

    #[test]
    fn operator_depth_as_int() {
        let mut h = ExplorationHorizon::boot("science");
        h.adapt(CognitiveSignature::Frontier);
        // 3.0 * 1.20 = 3.6 → round → 4
        assert_eq!(h.operator_depth_int(), 4);
    }
}
