// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  PROGRESSIVE_DEBATIC / mod.rs                                            ║
// ║  GJATA LEGACY™ — ESS-MAI Quantum                                         ║
// ║                                                                          ║
// ║  Shtresë gërmimi kognitiv (PD): kod real ekzekutiv në default mode, e    ║
// ║  përshtatur me ligjet sovrane (ZERO if/else, pa varësi të jashtme, fail- ║
// ║  closed, id-të me FNV-1a të sistemit).                                    ║
// ║                                                                          ║
// ║  Rrjedha: LIM → PD → LIM → ... (PD ushqen LIM-in e dytë; vazhdon nëpër   ║
// ║  PRO/SRK/PIM si është). PD prodhon EpistemicTrace + GeniusSignal +        ║
// ║  PdCognitivePackage; fushat relevante përkthehen për UI nga pd_light      ║
// ║  (shtresë gjuhësore në Light). PD NUK ka rrjedhë direkte te PIM.          ║
// ║                                                                          ║
// ║  Nën-modulet:                                                            ║
// ║    types           — kontratat prepared/pending/verified/next-i₀         ║
// ║    genius_detector — detektori i 3 sinjaleve (returns/energy/shape)      ║
// ║    core            — vetëm motori kognitiv (ingest/trace/output)         ║
// ║    seal            — vetëm autoriteti SEAL_PD + LgcToken privat          ║
// ║    runtime         — PD→Spine9→L1/2/3→receipt→output+iZ→next i₀         ║
// ╚══════════════════════════════════════════════════════════════════════════╝

pub mod types;
pub mod genius_detector;
pub mod core;
pub mod seal;
pub mod runtime;

// Re-eksporte për konsumatorin (pipeline / main).
pub use types::{
    PD_SEAL, EXPECTED_SHADOW_VERIFICATION_SEAL, pd_id,
    DebateMode, CognitiveSignal, EpistemicTrace, GeniusSignal, PressureBudget,
    PdResponseType, PdTurn, PdEngineOutput, PdPreClassification, PdCognitivePackage, PdError,
    PdPreSealReceipt, PdPendingNextI0, PdSpineCycle, PdPreparedCycle, PdVerificationCompletion,
    PdContinuationBasis, PdNextI0, PdVerifiedOutput, PdIzCompletion,
    PdContinuumClosure, PdFinalization,
};
pub use genius_detector::GeniusDetector;
pub use self::core::{ProgressiveDebatic, PdSession, build_package};
