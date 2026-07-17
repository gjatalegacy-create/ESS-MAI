// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  SOVEREIGN/MOD.RS — Ligji 0-COPY Sovereign (Light Platform)           ║
// ║  GJATA LEGACY™ — Light Platform                                   ║
// ║                                                                          ║
// ║  Ligjet matematike + capability tokens + ring buffer 0-copy.          ║
// ║  IDENTIK me Quantum sovereign — vula 500 ndër platforma.              ║
// ║  Light përdor sovereign për të mbrojtur kalimet FFI (light_buss,      ║
// ║  buss_legacy) me CapHandle një-përdorimësh.                          ║
// ╚══════════════════════════════════════════════════════════════════════════╝

pub mod laws;
pub mod lgc_gate;
pub mod ring;
pub mod seal_registry;   // Regjistri qendror i vulave (anti-përplasje)

// Ligjet matematike + vulat
pub use laws::{
    gate, dot8, admit4, fnv, module_seal,
    SEAL_COORDINATOR, SEAL_APUPK, SEAL_SNB, SEAL_MINI_EVOLVER,
    SEAL_QUANTUM_BUSS, SEAL_SHADOW_BUSS,
    SEAL_XOR, SEAL_MASK, SEAL_PRIMITIVE, SEAL_MASKED, verify_500,
};

// Capability tokens (ligji FFI që mbijeton)
pub use lgc_gate::{
    LgcToken, LgcGate, SovErr, CapHandle, SovereignGate,
};

// Ring buffer 0-copy + EpistemicVector
pub use ring::{
    EbpfPacket, RingBuffer, RingProducer, RingConsumer,
};

// Regjistri qendror i vulave
pub use seal_registry::{
    SealRegistry, SealEntry, Platform, CollisionReport,
    build_registry, REGISTRY_SIZE,
};
