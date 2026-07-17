// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAYER3/MOD.RS — Quantum Layer 3 (Hardware Pressure + Wisdom)       ║
// ║  GJATA LEGACY™ — ESS-MAI Quantum                                       ║
// ║                                                                          ║
// ║  Shtresa e tretë e Quantum-it: presioni shtrihet mbi harduerin.        ║
// ║   • HPRO  → familje operatorësh mbi harduer (prod_formula, control_role,║
// ║             wcfd/nwcfd/hwcfd device-mediated); ushqen presionin         ║
// ║   • HPIM  → paketon trace-in e HPRO për PIM (presioni s'futet te PIM)   ║
// ║   • quantum_wisdom → observon si u gjet primitiv/legacy                ║
// ║                                                                          ║
// ║  HPRO lëshon harduerin kur mbaron. Zero if/else.                      ║
// ╚══════════════════════════════════════════════════════════════════════════╝

pub mod hpro;            // Hardware Primitive Reasoning Origin (6 operatorë)
pub mod hpim;            // Hardware Pass Info Metric (paketon për PIM)
pub mod quantum_wisdom;  // observon zbulimin e primitiv/legacy
pub mod layer3_flow;     // orkestruesi i Shtresës 3 (HPRO→HPIM + wisdom packets)
pub mod hcp_pro_l3;      // HCP_PRO aktiv paralel (ngre të gjitha familjet njëheresh)
pub mod mpro;            // MPRO mates determinist → Score Vector → TrustContext

// ── Ri-eksportim ────────────────────────────────────────────────────────────
pub use hpro::{
    HproEngine, HproResult, ProdFormula, ControlRole, WebConnector,
    StabilityPlan, ParamPriority,
    WebProbeKind, WebProbeRequest, DeviceWebProbe,
    HPRO_COST_PER_OP, HPRO_ENERGY_FLOOR, HPRO_THERMAL_FLOOR, HPRO_RAM_CEILING,
};
pub use hpim::{HpimEngine, HpimPackage, HproTraceStep};
pub use quantum_wisdom::QuantumWisdom;
pub use layer3_flow::{QuantumLayer3Flow, QuantumLayer3Result};
pub use hcp_pro_l3::{HcpProL3, ParallelActivation, ProFamily, FamilySignals};
pub use mpro::{Mpro, MeasurementSet, ScoreVector};
