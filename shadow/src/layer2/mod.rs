// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAYER2/MOD.RS — Shadow Layer 2 (Supreme Verification Laboratory)   ║
// ║  GJATA LEGACY™ — ESS-MAI Shadow                                       ║
// ║                                                                          ║
// ║  Shtresa e dytë laboratorike e Shadow-it: verifikim sovran, jo thjesht  ║
// ║  vendim. Gjashtë gjykata që e bëjnë gjykimin më të rreptë dhe më të     ║
// ║  gjurmueshëm, pa e thjeshtuar autoritetin suprem.                      ║
// ║                                                                          ║
// ║  Merr EvidencePackage → prodhon VerificationState + LabVerdict +        ║
// ║  VerificationLedgerEntry + NegativeKnowledgeRecord. Zero if/else.      ║
// ╚══════════════════════════════════════════════════════════════════════════╝

pub mod shadow_courts;  // 6 gjykatat e verifikimit suprem
pub mod lab_flow;       // FAZA 5: orkestruesi i rrjedhës lab-to-lab (hook i pastër)
pub mod verification_memory; // SHKRIRJE: Persistent Lab Memory (cikli vetë-përmirësues)

// ── Ri-eksportim ────────────────────────────────────────────────────────────
pub use shadow_courts::{
    AdversarialCourt, EvidenceCourt, CrossDomainCourt, TrlCourt,
    NegativeKnowledgeCourt, FinalVerdictCourt, SupremeVerification,
    SHADOW_ACCEPT_CONFIDENCE, SHADOW_SURVIVED_MIN, SHADOW_CROSS_DOMAIN_MIN,
    SHADOW_PHYSICAL_EVIDENCE_TRL,
};
pub use lab_flow::{LabFlow, FlowStage, FlowResult};
pub use verification_memory::{VerificationMemoryIndex, Precedent};
