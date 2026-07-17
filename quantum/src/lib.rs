#![warn(clippy::unwrap_used)] // HAPI 7.4: bën .unwrap() të dukshëm (path kritike → .expect/error handling)
// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LIB.RS — QUANTUM PLATFORM  (PERSOSMËRI)                          ║
// ║  GJATA LEGACY™ — ESS-MAI Quantum Platform                             ║
// ║                                                                          ║
// ║  Pipeline:  Sovereign(0-copy) → LIM → HW_REAL → PRO → SRK → PIM       ║
// ║  Komunikimet:                                                         ║
// ║    bridge_light   ↔ Light    (QuantumInput/Output serialize)      ║
// ║    bridge_shadow  → Shadow   (QuantumInbound + vula 500)          ║
// ╚══════════════════════════════════════════════════════════════════════════╝

// ── Ligji 0-COPY Sovereign ───────────────────────────────────────────────
pub mod sovereign;
pub mod lab_contracts;   // LAYER 2: gjuha e përbashkët (byte-for-byte identik në 3 platformat)
pub mod layer2;          // LAYER 2: Deep Probe Lab (Pressure Engine + Abyssal Probe)
pub mod lab_contracts_v11; // LAYER 3: gjuha e zgjeruar (byte-for-byte identik në 3)
pub mod layer3;          // LAYER 3: HPRO + HPIM + quantum_wisdom
pub mod hcp_pro;         // LAYER 1: HCP_PRO controller aktiv orkestrimi (token mint/burn)
pub mod quantum_spine;   // PATCHIM: kurrizi vertikal (L3→L2→L3→L2)
pub mod phase9_integration; // FAZA 9: lidh quantum_spine me rrjedhën (5 ligjet)
pub mod pd_continuum_contract; // v1.5.6: i + U → i₀ → 1Q → output + iZ → next i₀
pub mod pd_spine_contract; // v1.5.6: kontrata PD→Spine9→Layer1/2/3 e lidhur me continuum
pub mod living_trust_contract; // v1.6.4: Living Trust lidh projektin me SHA-256 të plotë
pub mod gcl_project_contract; // v1.6.4: GCL Scientific Project Continuum
pub mod project_workspace_router; // v1.6.5: project-only storage/conversation orientation

// ── KUSHTETUTA + KONTROLLI (Teorite 52, 32, 35) ──────────────────────────
pub mod laws;        // 5 ligjet sovereign
pub mod governance;  // recursion control, override detection
pub mod control;     // pressure states + score system

// ── ARSYETIMI (Teorite 27, 30, 31, 34) ───────────────────────────────────
pub mod reasoning;   // elimination 3-nivel + semantic graph + convergence + territories

// ── MEMORIA + DIJA (Teorite 22, 23, 25, 26) ──────────────────────────────
// Quantum NUK mban memory persistente — kontrata mohuese + ephemeral.
pub mod memory;

// ── EKSPLORIMI (Teorite 7, 11, 13, 14) ────────────────────────────────────
pub mod exploration; // horizon manifold + frontier detector + state machine

// ── ORKESTRATORI (Lidhja e Plotë e Pipeline-it) ──────────────────────────
pub mod orchestrator; // zemra që lidh të gjitha modulet sipas arkitekturës
pub mod hardening;    // patch përforcues: konsensus + kalibrim + konflikt + qëndrueshmëri

// ── LIM ──────────────────────────────────────────────────────────────────
pub mod tokenizer;
pub mod lim_types;
pub mod lim_collector;
pub mod lim_measurer;
pub mod lim_classifier;
pub mod lim;

// ── Hardware (adapter i simuluar — i ruajtur për krahasim) ───────────────
pub mod hardware;
pub mod lim_hw_bridge;

// ── Hardware REAL (lexon RAM/CPU/termik të vërtetë) ──────────────────────
pub mod hw_real;
pub mod lim_hw_real_bridge;

// ── PRO ──────────────────────────────────────────────────────────────────
pub mod pro_types;
pub mod pro_operator;
pub mod pro_nk_gate;
pub mod pro;
pub mod runtime_pulse; // PULSI REAL i organeve — presume lexon jetën, jo premtimin (v1.4.3)
pub mod token_forge;   // BURIMI I DYTË i token-it — farka e pavarur (v1.4.3)

// ── NPRO + NPIM (Eliminimi Paralel + Negative Knowledge) ─────────────────
// NPRO: 4 operatorët MBRAPSHT → eliminimet + WeaknessSignal
// NPIM: paketon negative knowledge me argumenta për Shadow
pub mod npro_operator;
pub mod npro;
pub mod npim;
pub mod npro_lim_bridge;   // WeaknessSignal → LIM (kufijtë për PRO)
pub mod apro;              // APRO: argumentuesi (familja e 4-t e MPRO 16)
pub mod ultimatum_collapse_law; // MUSKULI i Quantum — i ushqyer nga gjata_collapse_law
pub mod npro_hardening;    // forcimi i NPRO/NPIM (konsensus+ceiling+severity)

// ── SRK ──────────────────────────────────────────────────────────────────
pub mod srk_types;
pub mod srk_ibe;
pub mod srk;

// ── PIM ──────────────────────────────────────────────────────────────────
pub mod pim;
pub mod lab;             // Digital Lab — TRL pipeline për matjen e saktë (Teoria 44)
pub mod lab_integration; // Digital Lab i lidhur PERFEKT me sistemin (ligje+trace+VNK)

// ── PROGRESSIVE_DEBATIC (PD) ──────────────────────────────────────────────
// Shtresë gërmimi kognitiv e rikthyer nga      (pseudo → kod real ekzekutiv).
// Rrjedha: LIM → PD → LIM → PRO(+npro/apro…) → PIM(+npim…). Zero if/else.
pub mod progressive_debatic;

// ── URAT E KOMUNIKIMIT ────────────────────────────────────────────────────
pub mod bridge_shadow;   // Quantum → Shadow 
pub mod bridge_light;    // Light  ↔ Quantum

// ════════════════════════════════════════════════════════════════════════
// RIEKSPORTE
// ════════════════════════════════════════════════════════════════════════

// Sovereign (ligji 0-copy)
pub use sovereign::{
    gate, dot8, elim, admit4, module_seal,
    RingBuffer, RingProducer, RingConsumer, EbpfPacket, Ev, LgcBridge, SovereignGate, CapHandle,
    SEAL_LIM, SEAL_PRO, SEAL_SRK, SEAL_PIM, SEAL_PD, SEAL_EBPF,
};

// Kushtetuta + Kontrolli
pub use laws::{SovereignConstitution, LawContext, RecursionKind, Destination};
pub use governance::Governance;
pub use control::{
    PressureState, PressureConfig, PressureDetector, PressureContext, ShadowState,
    ScoreVector, Weights, Scorer, ScoreFusion,
};

// Arsyetimi
pub use reasoning::{
    Elimination, ElimCandidate,
    SemanticGraph, Relation,
    Convergence, BestCandidate, SelectionMethod, Transfer, FinalOutput,
    ReasoningTerritories, Territory, EliminationMode,
};

// Memoria + Dija (Quantum s'mban memory persistente)
pub use memory::{
    MemoryBoundary, MemoryScope, MemoryViolation,
    RawCognitiveTrace, CognitiveTrace, ShadowVerdict, EventType, Outcome,
    ModuleReputation, RepOutcome, ShadowResult,
    KnowledgeLineage, LineageContext, OptimalEntry, NegativeEntry,
};

// Eksplorimi
pub use exploration::{
    ExplorationHorizon, CognitiveSignature,
    UnknownBoundaries, FrontierState, CeilingBreach, CycleResult,
    QuantumStateMachine, QuantumPhase, AdvanceResult,
};

// Orkestratori
pub use orchestrator::{QuantumOrchestrator, PipelineConfig, PipelineReport};
pub use hardening::{Hardening, HardeningReport, CrossOperatorConsensus, ConfidenceCalibration, PronoConflict, StabilityMargin};

// Digital Lab
pub use lab::{
    DigitalLab, Concept, Hypothesis, TrlEvidence, Trl3Result,
    SimulationEngine, FilterGate, RejectionReason, THRESHOLD_TRL3,
};
pub use lab_integration::{
    LabSystemBridge, IntegratedLabResult, LabNegativeKnowledge, EpistemicTrlBridge,
};

// LIM
pub use lim::LinearInfoMetricEngine;
pub use lim_types::{LinearInfoMetric, EpistemicState, EpistemicScaffold, Operator, LimError};

// Hardware
pub use hardware::ResourceSnapshot;
pub use lim_hw_bridge::{LimHwBridge, HardwareScaffold};
pub use lim_hw_real_bridge::{LimHwRealBridge, RealHardwareScaffold};
pub use hw_real::{DepthHint as RealDepthHint, kernel_hw_available};

// PRO
pub use pro::ProEngine;
pub use pro_types::{PROCandidate, ProResult};
pub use pro_nk_gate::KnowledgeVault;

// NPRO + NPIM
pub use npro::{NproEngine, NproResult, NegativeElimination, WeaknessSignal, WEAKNESS_CEILING};
pub use npim::{
    NegativePassMetric, NegativeKnowledgePackage, NegativityProfile,
    NegativeSuggestion, NegativeArgument,
};
pub use npro_lim_bridge::NproLimBridge;
pub use npro_hardening::{
    NproHardening, NpimHardening, NproHardeningReport,
    WeaknessConsensus, AdaptiveCeiling, EvidenceWeighting, SeverityEscalation,
};

// SRK
pub use srk::ScientificReasoningKernel;
pub use srk_types::ReasoningPackage;

// PIM
pub use pim::{PassInfoMetric, PassPackage};

// Urat
pub use bridge_shadow::{
    QuantumShadowBridge, QuantumInbound, LightInbound,
    SupremeOutcome, ShadowLightResponse,
    NegativeInbound, NpimShadowBridge,
};
pub use bridge_light::{
    QuantumInput, QuantumOutput, OutputVerdict, LightQuantumBridge, LightBridgeError,
};
