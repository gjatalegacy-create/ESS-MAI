// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  EXPLORATION/MOD.RS — Eksplorimi (Teorite 7, 11, 13, 14)             ║
// ║  GJATA LEGACY™ — Quantum Platform                                     ║
// ║                                                                          ║
// ║   horizon: manifold deformueshëm + CognitiveSignature                 ║
// ║   frontier: Epistemic Frontier Detector me tavane absolute            ║
// ║   state_machine: QuantumPhase IDLE→...→TRANSFER                       ║
// ╚══════════════════════════════════════════════════════════════════════════╝

pub mod horizon;
pub mod frontier;
pub mod state_machine;

pub use horizon::{
    ExplorationHorizon, HorizonAxes, CognitiveSignature, HorizonMultipliers,
};
pub use frontier::{
    UnknownBoundaries, FrontierAxes, FrontierState, CeilingBreach,
    CycleResult, BoundaryScan,
    ENTROPY_CEILING, CONTRADICTION_CEILING, RECURSION_CEILING, SEMANTIC_DRIFT_CEILING,
};
pub use state_machine::{
    QuantumStateMachine, QuantumPhase, AdvanceResult,
};
