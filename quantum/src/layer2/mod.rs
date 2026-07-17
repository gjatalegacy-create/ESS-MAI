// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAYER2/MOD.RS — Quantum Layer 2 (Deep Probe Laboratory)            ║
// ║  GJATA LEGACY™ — ESS-MAI Quantum                                       ║
// ║                                                                          ║
// ║  Shtresa e dytë laboratorike e Quantum-it: presion i hapur, agresiv,    ║
// ║  i thellë. NUK zëvendëson NPRO/digital_lab — i FORCON dhe i ORKESTRON.   ║
// ║                                                                          ║
// ║  Përbërësit:                                                          ║
// ║    • pressure_engine → forcon NPRO, prodhon PressureReport             ║
// ║    • abyssal_probe   → çon idenë në limit (Initial/Pressure/Abyssal)   ║
// ║    • verification_package_builder → asamblon EvidencePackage për Shadow ║
// ║                                                                          ║
// ║  Presioni jeton këtu. Matja TRL mbetet te digital_lab/PIM. Zero if/else.║
// ╚══════════════════════════════════════════════════════════════════════════╝

pub mod pressure_engine;             // Motori i presionit (forcon NPRO)
pub mod abyssal_probe;               // Sonda e thellimit (drejt kufirit)
pub mod verification_package_builder;
pub mod hcp_pro_l2;      // HCP_PRO orkestrim me presion (lidh L1↔L3) // Ndërtuesi i dosjes së provës

// ── Ri-eksportim ────────────────────────────────────────────────────────────
pub use pressure_engine::{PressureEngine, PRESSURE_WEAKNESS_HIGH, PRESSURE_CONTRADICTION_HIGH};
pub use abyssal_probe::{AbyssalProbe, ProbeDepth, ProbeResult, TruthVerdict};
pub use verification_package_builder::VerificationPackageBuilder;
