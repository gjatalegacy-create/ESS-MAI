// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  SOVEREIGN/MOD.RS — Ligji 0-COPY Sovereign (adaptuar në Quantum)      ║
// ║  GJATA LEGACY™ — Quantum Platform                                     ║
// ║                                                                          ║
// ║  Ligjet matematike + capability tokens + ring buffer 0-copy.          ║
// ║  eBPF/Hardware → RingBuffer → LgcBridge → EpistemicVector → LIM       ║
// ╚══════════════════════════════════════════════════════════════════════════╝

pub mod laws;
pub mod lgc_gate;
pub mod ring;
pub mod sha256;   // SHA-256 i vërtetë (FIPS 180-4) për Digital Lab
pub mod seal_registry;   // Regjistri qendror i vulave (anti-përplasje)

// Ligjet matematike
pub use laws::{
    gate, dot8, elim, admit4, fnv, module_seal,
    SEAL_LIM, SEAL_PRO, SEAL_SRK, SEAL_PIM, SEAL_PD, SEAL_EBPF,
};

// SHA-256 (FIPS 180-4)
pub use sha256::{sha256, sha256_hex};

// Capability tokens (LAW_3 mbijeton FFI)
pub use lgc_gate::{
    LgcToken, LgcGate, SovErr, CapHandle, SovereignGate,
};

// Ring buffer 0-copy + EpistemicVector
pub use ring::{
    RING_N, RING_MASK, EbpfPacket, RingBuffer, RingProducer, RingConsumer, Ev, LgcBridge,
};

// Regjistri qendror i vulave (anti-përplasje)
pub use seal_registry::{
    SealRegistry, SealEntry, Platform, CollisionReport,
    build_registry, REGISTRY_SIZE,
};
