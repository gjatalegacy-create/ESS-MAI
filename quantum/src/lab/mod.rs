// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAB/MOD.RS — Digital Lab (Teoria 44)                                ║
// ║  GJATA LEGACY™ — Quantum Platform                                     ║
// ║                                                                          ║
// ║  Matja e saktë përmes TRL Pipeline 3-fazor.                          ║
// ║  Lab MAT → prodhon TrlEvidence → PIM përdor për potentiality.          ║
// ╚══════════════════════════════════════════════════════════════════════════╝

pub mod lab_types;
pub mod digital_lab;

pub use lab_types::{
    Concept, Hypothesis, TrlEvidence, Trl3Result,
    SimulationResult, SimOutcome, SimMetrics,
    TrlStatus, RejectionReason, LabError, lab_hash,
};
pub use digital_lab::{
    DigitalLab, SimulationEngine, ResultEvaluator, FilterGate,
    THRESHOLD_TRL3,
};
