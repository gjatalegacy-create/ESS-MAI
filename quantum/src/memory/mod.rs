// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  MEMORY/MOD.RS — Kontrata e Memorjes + Dija (Teorite 22, 23, 25, 26)  ║
// ║  GJATA LEGACY™ — Quantum Platform                                     ║
// ║                                                                          ║
// ║  RREGULLI: Quantum NUK mban memory persistente.                       ║
// ║   - boundary: ephemeral-only + gardian që refuzon persistent          ║
// ║   - raw_cognitive_trace: append-only, VETËM Shadow shkruan verdict     ║
// ║   - module_reputation: ephemeral, clear() me ciklin                   ║
// ║   - knowledge_lineage: konsulton refs nga Shadow, s'ruan vetë          ║
// ╚══════════════════════════════════════════════════════════════════════════╝

pub mod boundary;
pub mod raw_cognitive_trace;
pub mod module_reputation;
pub mod knowledge_lineage;

pub use boundary::{
    MemoryBoundary, MemoryScope, MemoryViolation,
    EphemeralStore, PersistentGuard,
};
pub use raw_cognitive_trace::{
    RawCognitiveTrace, CognitiveTrace, ShadowVerdict, EventType, Outcome, TraceStats,
};
pub use module_reputation::{
    ModuleReputation, ReputationRecord, RepOutcome, ShadowResult,
};
pub use knowledge_lineage::{
    KnowledgeLineage, LineageContext, OptimalEntry, NegativeEntry,
    LINEAGE_BOOST, LINEAGE_PENALTY,
};
