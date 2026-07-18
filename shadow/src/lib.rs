// v1.5.9 — KY SKEDAR NUK ËSHTË MË TARGET LIBRARIE.
// Cargo.toml ka `autolib = false`; `shadow/src/main.rs` e përfshin këtë trup
// me `include!("lib.rs")`. Prandaj autoriteti më poshtë kompilohet vetëm brenda
// procesit Shadow dhe nuk mund të linkohet si rlib/staticlib nga Quantum.

// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LIB.RS — ESS-MAI SHADOW PLATFORM                                ║
// ║  Gjata Legacy™  |  Arkitekt: Bledar Gjata                             ║
// ║                                                                          ║
// ║  AUTORITETI SUPREM EPISTEMIK — memoria sovrane.                       ║
// ║  Shadow VETËM verifikon, kategorizon dhe sistematizon dijen. Nuk      ║
// ║  propozon, nuk arsyeton, nuk eliminon (ato i bën Quantum). I VETMI    ║
// ║  që kupton vulën 500 dhe vendos 0/1; i vetmi që shkruan Persistent.  ║
// ║                                                                          ║
// ║  ARKITEKTURA:                                                     ║
// ║    • DY origjina, struct të NDARA: PassPackage (Quantum) +            ║
// ║      LightEnvelope (Light). Bashkohen VETËM në ShadowPassage          ║
// ║      (shadow_pipeline). NJË hyrje (Shadow::ingest), NJË pipeline.     ║
// ║    • LIGJI 0 (zero-copy): verifikuesit nuk klonojnë inputin;          ║
// ║      origjinat zhvendosen; payload-i materializohet në vault një herë. ║
// ║    • Ligjet sovrane: enforce_sovereign_laws (refuzim i fortë).        ║
// ║    • Bitmask C `LGC_LAW_*` = gjendje dije; shtresa Rust = qeverisje.  ║
// ║                                                                          ║
// ║  Lidhjet me ekosistemin:                                              ║
// ║    Light  → LightEnvelope   (shadow_seal_bridge: vula 500 verbërisht) ║
// ║    Quantum→ PassPackage     (quantum_shadow_bridge: ShadowPassPackage)║
// ╚══════════════════════════════════════════════════════════════════════════╝

// ── Modulet reale (LINJA B) ─────────────────────────────────────────────────
pub mod types;
pub mod knowledge_vault;
pub mod vault_disk;
pub mod sovereign_log;
pub mod knowledge_lineage;
pub mod primitive_chain;     // FAZA 2: përforcimi i zinxhirit primitiv (invariantë I1-I3)
pub mod verefied_diary_supremelaw; // MUSKULI i Shadow — i ushqyer nga gjata_collapse_law
pub mod legacy_maturation;   // FAZA 3: përforcimi i maturimit Primitive→Legacy (M1-M3)
pub mod legacy_observer;     // FAZA 7: vëzhguesi i përhershëm i primitivëve (prejardhja)
pub mod shadow_lab;          // FAZA 8: Digital Lab në karakterin e Shadow (gjykim me TRL)
pub mod media_formats;       // FAZA 9: njohja e formateve (imazh/video/dokument) për faktim
pub mod shadow_genius_novel; // FAZA 9: bartësi i TRL 4 — inovacion faktik me dokumentacion
pub mod shadow_eco;          // FAZA 9: orkestratori ekstrem — sistemon dijen sipas llojit
pub mod seal_registry;       // FAZA 4-5: regjistri qendror i vulave (i përbashkët 3 platformat)
pub mod lab_contracts;       // LAYER 2: gjuha e përbashkët (byte-for-byte identik në 3 platformat)
pub mod layer2;              // LAYER 2: Supreme Verification Lab (6 gjykatat)
pub mod lab_contracts_v11;   // LAYER 3: gjuha e zgjeruar (byte-for-byte identik në 3)
pub mod layer3;              // LAYER 3: Night Watch + Wisdom + Magazina
pub mod shadow_spine;        // PATCHIM: kurrizi vertikal (L3→L2→L2→L3)
pub mod shadow_true_knowledge; // KNOWLEDGE: pranon dijen pip (kërkon trace fillestar)
pub mod phase9_integration;  // FAZA 9: lidh spine+true_knowledge me rrjedhën (ligjet)
pub mod pd_continuum_contract; // v1.5.6: kontrata identike i₀→output+iZ→next i₀
pub mod pd_spine_contract; // v1.5.6: kontrata identike PD/Spine9
pub mod living_trust_contract; // v1.6.4: Living Trust lidh projektin me SHA-256 të plotë
pub mod gcl_project_contract; // v1.6.4: projekti shkencor brenda verdictit GCL
pub mod ess_mai_heart_byte;  // HEART_BYTE: regjistron vendimet HCP_PRO te ledger ekzistues
pub mod sovereign_ffi_gate;
pub mod luvik;
pub mod shadow_destfake;
pub mod shadow_apupk;
pub mod shadow_snb;
pub mod bridge;
pub mod ffi_ring;

pub mod shadow_router;
pub mod shadow_matrix;
pub mod shadow_gen5;
pub mod shadow_type;
pub mod shadow_temporal;
pub mod shadow_sovereign;
pub mod shadow_emergence;
pub mod shadow_consensus;
pub mod shadow_judiciary;

pub mod sovereign_guard;
pub mod shadow_gj_legacy;
pub mod shadow_pipeline;
pub mod shadow_gateway;
pub mod shadow_runtime_pulse; // PULSI REAL i autoritetit — presume lexon jetën (v1.4.5)

// ── API e brendshme e binarit Shadow ─────────────────────────────────────────
// `pub` këtu nuk krijon API të linkueshme: Cargo nuk prodhon target library.
// build.rs e verifikon këtë invariant dhe main.rs është ndërmjetësi i vetëm.

// Fasada + përgjigjet
pub use shadow_gateway::{Shadow, ShadowResponse, ShadowStats};

// Tipet e dy origjinave + verdiktet
pub use types::{
    KnowledgeBand, LightEnvelope, NegativeContext, PassPackage, PrimitiveTrace, ShadowError,
    ShadowNode, ShadowPassage, ShadowPath, ShadowVerdict, SupremeVerdict,
};

// Arkivi sovran
pub use knowledge_vault::{InMemoryBackend, KnowledgeVault, StoreKind, VaultBackend};
// Durabiliteti (FAZA 2)
pub use vault_disk::DiskBackend;
// Ligji i gjurmueshmërisë (trace-ose-fshi) + rregulla Primitive→Legacy ndër-domain
pub use knowledge_lineage::{
    cross_domain_legacy_ready, enforce_traceability, ledger, unique_domains, Lineage,
    LineageLedger, TraceError, MIN_LEGACY_DOMAINS,
};
// FAZA 2 — përforcimi i zinxhirit primitiv
pub use primitive_chain::{
    ChainIntegrity, ChainStrength, seal_consistent, temporally_monotonic, mass_floor_met,
    PRIMITIVE_MASS_FLOOR,
};
// FAZA 3 — përforcimi i maturimit legacy
pub use legacy_maturation::{
    MaturationSignals, MaturationState, MaturationGate, LegacyReason,
    MIN_REAL_HITS, MIN_TEMPORAL_STABILITY, MIN_CONSULTATIONS,
};
// FAZA 7 — vëzhguesi i përhershëm i primitivëve
pub use legacy_observer::{
    LegacyObserver, LegacyBirth, ObservationStats,
};
// FAZA 8 — Digital Lab në karakterin e Shadow
pub use shadow_lab::{
    ShadowLab, ShadowLabOutcome, TrlInput, TrlVerdict, TrlRejection, ShadowFilterGate,
    SHADOW_THRESHOLD_TRL3, SHADOW_MIN_TRL,
};
// FAZA 9 — njohja e formateve + bartësi TRL 4
pub use media_formats::{
    MediaKind, MediaFormat, FormatDetector,
};
pub use shadow_genius_novel::{
    GeniusNovel, NovelEvidence, NovelVerdict, NovelRejection, FactualInnovation,
    EvidenceFile, TRL4_FACTUAL, SHADOW_TRL4,
};
pub use shadow_eco::{
    ShadowEco, EpistemicClass, ClassificationInput, ClassificationResult,
};
// FAZA 4-5 — regjistri qendror i vulave (anti-përplasje ndër-platformë)
pub use seal_registry::{
    SealRegistry, SealEntry, Platform, CollisionReport, build_registry, module_seal,
    REGISTRY_SIZE,
};
// Porta sovrane FFI
pub use sovereign_ffi_gate::{
    sovereign_issue_capability, sovereign_validate_and_write, seal_verified_output,
    CapHandle, VerificationReceipt,
    S_LAB_TRL1, S_LAB_TRL2, S_SHADOW_WRITE, S_SHADOW_VERIFY,
};
// Porta sovrane e rreptë (Luvik) + eliminimi i infos pa gjurmë (destfake)
pub use luvik::{Luvik, LuvikReject, VerifiedKnowledge};
pub use shadow_destfake::{destfake, DestFake, DestFakeAction};
// Kujtesat sovrane nga Light: APUPK (njohuri projekti) + SNB (bug-e)
pub use shadow_apupk::{
    apupk_memory, init_apupk_disk, ApupkEntry, ApupkInbound, ApupkReject, ApupkTrace,
    ProjectProgress, ShadowApupkMemory,
};
pub use shadow_snb::{
    init_snb_disk, snb_store, BugEntry, BugInbound, ShadowSnb, SnbReceipt, SnbReject, SnbSeverity,
};
// WAL gjenerik sovran (durabiliteti i APUPK/SNB)
pub use sovereign_log::{RecReader, RecWriter, SovereignLog};
// Vetëm tipat e hyrjes janë aktivë në production; main.rs/process_bridge është ura.
pub use bridge::{LightInbound, QuantumInbound};
#[cfg(test)]
pub use bridge::{
    DefaultLightShadowBridge, LightShadowBridge,
    ShadowLightResponse, SupremeOutcome,
};
pub use ffi_ring::{RingSlot, ShadowRing, RING_CAP, SLOT_BYTES};

// Orkestratori + ligjet sovrane (për përdorim të avancuar / testim)
pub use shadow_gj_legacy::{KernelStats, ShadowGjLegacy};
pub use shadow_pipeline::{mark_time_degraded, now_ns, run_pipeline, stable_id, time_degraded};
pub use sovereign_guard::enforce_sovereign_laws;
