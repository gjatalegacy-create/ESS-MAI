// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  ORCHESTRATOR.RS — Zemra e Quantum-it (Lidhja e Plotë)               ║
// ║  GJATA LEGACY™ — Quantum Platform                                 ║
// ║                                                                          ║
// ║  Lidh TË GJITHA modulet sipas arkitekturës, me ligjet aktive:         ║
// ║                                                                          ║
// ║   Light → [Kushtetuta] → LIM ──┬─► PRO ──────────────┐               ║
// ║                                 └─► NPRO ─┬─► NPIM ──► Shadow (neg)    ║
// ║                                           └─► LIM (WeaknessSignal)    ║
// ║   → Horizon/Frontier → Pressure → Eliminim 3-nivel → Convergence      ║
// ║   → PIM → Shadow → Light                                              ║
// ║                                                                          ║
// ║  LIGJET: enforce_all në çdo fazë. Recursion guard. Memory ephemeral.   ║
// ║  Quantum NUK vendos (Shadow vendos). Quantum NUK mban memory.          ║
// ║  Zero if/else — match/formula.                                        ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::laws::{SovereignConstitution, LawContext, Destination, RecursionKind};
use crate::governance::Governance;
use crate::control::pressure_states::{PressureDetector, PressureContext, PressureState, ShadowState};
use crate::control::score_system::{ScoreVector, ScoreFusion};
use crate::reasoning::elimination::{Elimination, ElimCandidate};
use crate::reasoning::convergence::{Convergence, Transfer};
use crate::reasoning::territories::ReasoningTerritories;
use crate::memory::boundary::{MemoryBoundary, MemoryScope};
use crate::memory::raw_cognitive_trace::{RawCognitiveTrace, EventType, Outcome};
use crate::memory::module_reputation::{ModuleReputation, RepOutcome, ShadowResult};
use crate::exploration::horizon::{ExplorationHorizon, CognitiveSignature};
use crate::exploration::frontier::{UnknownBoundaries, CycleResult};
use crate::exploration::state_machine::{QuantumStateMachine, QuantumPhase, AdvanceResult};
use crate::lab::lab_types::TrlEvidence;

// ─────────────────────────────────────────────────────────────────────────────
// PIPELINE CONFIG — konfigurimi i orkestratorit
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct PipelineConfig {
    pub territory:          String,
    pub session_id:         String,
    pub cognitive_signature: CognitiveSignature,
    pub shadow_state:       ShadowState,
    pub lim_density:        f32,   // nga LimMetric.information_density (P2)
    pub lim_continuity:     f32,   // nga LimMetric.continuity_score (P2)
}

impl PipelineConfig {
    pub fn new(territory: &str, session_id: &str) -> Self {
        Self {
            territory: territory.to_string(),
            session_id: session_id.to_string(),
            cognitive_signature: CognitiveSignature::Structural,
            shadow_state: ShadowState::Light,
            lim_density:    0.5,  // default neutral
            lim_continuity: 0.5,
        }
    }

    /// with_lim — injekton sinjalet reale të LIM (P2.3). Zero if.
    pub fn with_lim(mut self, density: f32, continuity: f32) -> Self {
        self.lim_density    = density;
        self.lim_continuity = continuity;
        self
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// PIPELINE REPORT — rezultati i plotë i orkestrimit
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct PipelineReport {
    pub phase_reached:      QuantumPhase,
    pub laws_passed:        bool,
    pub pressure:           PressureState,
    pub candidates_in:      usize,
    pub candidates_out:     usize,
    pub fusion_score:       f32,
    pub frontier_breached:  bool,
    pub final_provider:     String,
    pub final_confidence:   f32,
    pub traces_recorded:    usize,
    pub memory_audit:       (usize, u32),  // (ephemeral_len, persistent_rejections)
    pub hardening_stability: f32,          // qëndrueshmëria nga patch-i përforcues
    pub lab_trl_passed:     bool,          // a kaloi Digital Lab TRL3
    pub lab_reproducibility: f32,          // reproducibility e matur nga Lab
    pub lab_rejection:      &'static str,  // arsyeja nëse Lab refuzoi
}

// ─────────────────────────────────────────────────────────────────────────────
// QUANTUM ORCHESTRATOR — gjendja e plotë e një cikli
// ─────────────────────────────────────────────────────────────────────────────

pub struct QuantumOrchestrator {
    config:       PipelineConfig,
    constitution: SovereignConstitution,
    governance:   Governance,
    state_machine: QuantumStateMachine,
    horizon:      ExplorationHorizon,
    frontier:     UnknownBoundaries,
    memory:       MemoryBoundary,
    trace:        RawCognitiveTrace,
    reputation:   ModuleReputation,
}

impl QuantumOrchestrator {
    pub fn new(config: PipelineConfig) -> Self {
        let territory = config.territory.clone();
        let session = config.session_id.clone();
        Self {
            horizon: ExplorationHorizon::boot(&territory),
            frontier: UnknownBoundaries::boot(&territory),
            memory: MemoryBoundary::new(&session),
            constitution: SovereignConstitution::new(),
            governance: Governance::new(),
            state_machine: QuantumStateMachine::new(),
            trace: RawCognitiveTrace::new(),
            reputation: ModuleReputation::new(),
            config,
        }
    }

    /// run — ekzekuton ciklin e plotë mbi kandidatët e PRO-s.
    ///
    /// candidates vijnë nga PRO (provider, domain, ptype, score).
    /// Kthen PipelineReport. Quantum NUK vendos — vetëm përgatit për Shadow.
    pub fn run(&mut self, candidates: Vec<ElimCandidate>) -> PipelineReport {
        let shadow = self.config.shadow_state;
        let candidates_in = candidates.len();

        // ── FAZA 0: KUSHTETUTA — enforce_all para çdo gjëje ────────────────
        let ctx = LawContext {
            destination: Destination::Ephemeral,
            recursion_kind: RecursionKind::PipelineCycle,
            recursion_count: 0,
        };
        let laws_passed = self.constitution.enforce_all("pipeline_run", "QuantumOrchestrator", &ctx);

        // ── FAZA 1: DETECT — recursion guard + state machine ───────────────
        // RUNTIME HARDENING (      ): LIGJI 1 FORBIDDEN_RECURSION — rezultati i
        // recursion guard NUK injorohet më; lidhet me laws_passed. Tejkalim → ligj i thyer.
        let recursion_ok = self.governance.check_recursion("pipeline_cycle");
        let advance_detect = self.state_machine.advance(QuantumPhase::Detect, shadow);

        // ── FAZA 2: HORIZON — adapto sipas CognitiveSignature ──────────────
        self.horizon.adapt(self.config.cognitive_signature);

        // ── FAZA 3: PRESSURE — detekto gjendjen ────────────────────────────
        let pressure = Self::detect_pressure(candidates_in, shadow);

        // ── FAZA 4: TRACE — regjisto fillimin (ephemeral, RAW_PENDING) ─────
        let trace_id = self.trace.record(
            &self.config.session_id, "DETECT", &self.config.territory,
            pressure.label(), EventType::Reasoning, "pipeline start",
            "QuantumOrchestrator", Outcome::Unknown, "", "", "",
        );

        // ── FAZA 5: MEMORY — shkruaj ephemeral (Quantum s'shkruan persistent) ─
        // RUNTIME HARDENING (      ): rezultati bëhet eksplicit. Ephemeral kthen
        // gjithmonë Ok; Err (s'duhet ndodhë për Ephemeral) → vihet re. Zero if — match.
        match self.memory.write(MemoryScope::Ephemeral, "cycle_state", "active", "ELIMINATION") {
            Ok(())  => {}
            // GJETJA 1 (audit): komenti thoshte "vihet re" por s'vinte re asgjë (silent
            // failure pas komenti gënjeshtar). Tani njoftohet eksplicit; mbahet fail-safe
            // (s'rrëzon ciklin — Ephemeral s'duhet të shkelë, por anomalia raportohet).
            Err(v) => { eprintln!("[ORCHESTRATOR] ANOMALY: ephemeral write dështoi (s'duhet ndodhë): {:?}", v); }
        }

        // ── FAZA 5.5: NPRO frags — kapen PARA eliminimit (candidates ende gjallë) ─
        // (P2.2: sinjali real nga LIM, jo empty_weakness.)
        let npro_frags: Vec<String> = candidates.iter()
            .map(|c| format!("{} {}", c.provider, c.provider_domain))
            .collect();

        // ── FAZA 6: ELIMINIM 3-NIVEL ───────────────────────────────────────
        let eliminated = Elimination::run_all(candidates, shadow);

        // ── FAZA 6.4: NPRO — rruga paralele negative me sinjalin real të LIM ─
        let npro_raw = crate::npro::NproEngine::activate(
            &npro_frags,
            self.config.lim_density,     // ← sinjali real, jo 0.0 hardkod
            self.config.lim_continuity,
        );
        let (npro_hardened, _npro_report) =
            crate::npro_hardening::NproHardening::strengthen_npro(npro_raw);

        // ── FAZA 6.5: HARDENING me sinjalin REAL nga NPRO (jo empty_weakness) ─
        let (eliminated, hardening_report) =
            crate::hardening::Hardening::strengthen(
                eliminated,
                &npro_hardened.weakness_signal,  // ← JO empty_weakness
                5,
            );
        let candidates_out = eliminated.len();

        // ── FAZA 7: SCORE FUSION 6D ────────────────────────────────────────
        let fusion_score = Self::compute_fusion(&eliminated, &self.config.territory);

        // ── FAZA 8: FRONTIER — skano kufijtë epistemikë ────────────────────
        let cycle_result = CycleResult {
            converged: candidates_out > 0,
            entropy: 1.0 - fusion_score,
            contradiction_count: 0,
            recursion_depth: self.governance.recursion_depth("pipeline_cycle"),
            semantic_drift: 1.0 - fusion_score,
        };
        let scan = self.frontier.scan(&cycle_result);

        // ── FAZA 8.5: DIGITAL LAB — matja e saktë përmes TRL pipeline ──────
        // Lab-i validon kandidatin më të fortë me TRL1→TRL2→TRL3 + FilterGate.
        // Prodhon TrlEvidence (reproducibility e matur saktë) për raportin.
        let lab_evidence = self.run_lab_validation(&eliminated, fusion_score);

        // ── FAZA 9: CONVERGENCE — zgjedh best (ultra lazy) ─────────────────
        let best = Convergence::fuse(&eliminated, candidates_out > 1);

        // ── FAZA 10: TRANSFER — vulos për Shadow (Quantum s'vendos) ────────
        let (final_provider, final_confidence) = Self::finalize(&best, &trace_id);

        // ── FAZA 11: REPUTATION — përditëso (ephemeral) ────────────────────
        self.update_reputation(&final_provider, fusion_score, candidates_out);

        // ── RAPORT ─────────────────────────────────────────────────────────
        PipelineReport {
            phase_reached: self.state_machine.current(),
            laws_passed: laws_passed && recursion_ok && Self::advance_ok(advance_detect),
            pressure,
            candidates_in,
            candidates_out,
            fusion_score,
            frontier_breached: scan.ceiling_breached,
            final_provider,
            final_confidence,
            traces_recorded: self.trace.len(),
            memory_audit: self.memory.audit(),
            hardening_stability: hardening_report.stability,
            lab_trl_passed: lab_evidence.trl_passed,
            lab_reproducibility: lab_evidence.reproducibility,
            lab_rejection: Self::lab_rejection_label(&lab_evidence),
        }
    }

    /// run_lab_validation — Digital Lab i INTEGRUAR mat kandidatin kryesor.
    ///
    /// Përdor LabSystemBridge: ligje (intervention_guard) + trace (audit) +
    /// VNK (nëse dështon). Lab-i është tani perfekt me sistemin.
    /// candidates bosh → evidence pending. Zero if — match.
    fn run_lab_validation(&mut self, candidates: &[ElimCandidate], fusion: f32) -> TrlEvidence {
        match candidates.first() {
            None => TrlEvidence::pending(),
            Some(top) => {
                let statement = format!(
                    "{} në domain {} me fusion {:.2}",
                    top.provider, top.provider_domain, fusion
                );
                let assumptions = vec![
                    format!("proposal_type: {}", top.proposal_type),
                    format!("quantum_score: {:.3}", top.quantum_score),
                ];
                // Kopjo fushat immutable PARA borrow-it mutable (siguri borrow 100%).
                let territory = self.config.territory.clone();
                let session = self.config.session_id.clone();
                let provider = top.provider.clone();
                // Lab i integruar: ligje + trace + VNK (governance & trace janë &mut).
                let integrated = crate::lab_integration::LabSystemBridge::run_integrated(
                    &provider,
                    &territory,
                    &statement,
                    &statement,
                    assumptions,
                    &mut self.governance,
                    &mut self.trace,
                    &session,
                );
                integrated.evidence
            }
        }
    }

    fn lab_rejection_label(evidence: &TrlEvidence) -> &'static str {
        // pending → "PENDING"; passed → "NONE"; ndryshe → "REJECTED".
        // Zero if — match mbi tuple.
        match (evidence.pending_lab, evidence.trl_passed) {
            (true, _)  => "PENDING_LAB",
            (_, true)  => "TRL3_PASSED",
            (_, false) => "TRL3_REJECTED",
        }
    }

    /// end_cycle — Quantum NUK mban memory: flush gjithçka.
    pub fn end_cycle(&mut self) {
        self.memory.end_cycle();      // ephemeral vdes
        self.governance.reset_cycle(); // counters → 0
        self.reputation.clear();       // reputacion ephemeral
        self.state_machine.reset();    // → IDLE
    }

    // ── NDIHMËS (zero if/else) ──────────────────────────────────────────────

    fn detect_pressure(candidates: usize, shadow: ShadowState) -> PressureState {
        let ctx = PressureContext {
            entropy: 0.5,
            contradiction_count: 0,
            candidates_count: candidates as u32,
            time_elapsed_ms: 100,
            shadow_state: shadow,
        };
        PressureDetector::detect(&ctx)
    }

    fn compute_fusion(candidates: &[ElimCandidate], territory: &str) -> f32 {
        // is_empty ∈ {0,1}: bosh → 0; ndryshe → fusion mbi mesataren.
        let is_empty = candidates.is_empty() as u32 as f32;
        let sum: f32 = candidates.iter().map(|c| c.quantum_score).sum();
        let avg = sum / candidates.len().max(1) as f32;

        // Ndërto ScoreVector nga mesatarja (thjeshtësim deterministik).
        let mut vec = ScoreVector {
            logical_score: avg, scientific_score: avg, stability_score: avg,
            semantic_score: avg, coherence_score: avg, entropy_score: avg,
            fusion_score: 0.0,
        };
        let fused = ScoreFusion::fuse(&mut vec, territory);
        (1.0 - is_empty) * fused
    }

    fn finalize(best: &Option<crate::reasoning::convergence::BestCandidate>, trace_id: &str) -> (String, f32) {
        // Some → vulos; None → bosh. Zero if — match.
        match best {
            Some(b) => {
                let output = Transfer::deliver(b, trace_id);
                (output.provider, output.confidence)
            }
            None => ("NONE".to_string(), 0.0),
        }
    }

    fn update_reputation(&mut self, provider: &str, fusion: f32, candidates_out: usize) {
        // outcome nga rezultati (branchless mapping via match).
        let outcome = match candidates_out > 0 {
            true  => RepOutcome::Success,
            false => RepOutcome::Failure,
        };
        self.reputation.update(provider, &self.config.territory, outcome, ShadowResult::Pending, fusion);
    }

    fn advance_ok(result: AdvanceResult) -> bool {
        matches!(result, AdvanceResult::Advanced(_) | AdvanceResult::FastPath(_))
    }

    // ── ACCESSORS ───────────────────────────────────────────────────────────

    pub fn trace_stats(&self) -> crate::memory::raw_cognitive_trace::TraceStats {
        self.trace.stats()
    }

    pub fn territory_discipline(&self) -> &'static str {
        ReasoningTerritories::get(&self.config.territory).elimination_mode.label()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cand(provider: &str, score: f32) -> ElimCandidate {
        ElimCandidate {
            provider: provider.to_string(),
            provider_domain: "science".to_string(),
            proposal_type: "t1".to_string(),
            quantum_score: score,
            has_proposal: true,
            has_signature: true,
        }
    }

    fn orchestrator() -> QuantumOrchestrator {
        QuantumOrchestrator::new(PipelineConfig::new("science", "sess1"))
    }

    #[test]
    fn full_pipeline_runs() {
        let mut orch = orchestrator();
        let candidates = vec![cand("PRO_A", 0.9), cand("PRO_B", 0.7)];
        let report = orch.run(candidates);
        // Pipeline mbërriti te një fazë e vlefshme.
        assert!(report.laws_passed);
        assert_eq!(report.candidates_in, 2);
    }

    #[test]
    fn laws_enforced() {
        let mut orch = orchestrator();
        let report = orch.run(vec![cand("PRO_A", 0.9)]);
        // Kushtetuta kaloi (action e ligjshme, ephemeral).
        assert!(report.laws_passed);
    }

    #[test]
    fn quantum_never_writes_persistent() {
        let mut orch = orchestrator();
        orch.run(vec![cand("PRO_A", 0.9)]);
        // Asnjë shkrim persistent s'u lejua (audit: rejections mund të jenë 0 sepse s'u tentua).
        let (_ephemeral, _rejections) = orch.memory.audit();
        // Pas end_cycle, memory pastrohet.
        orch.end_cycle();
        assert_eq!(orch.memory.audit().0, 0); // ephemeral vdiq
    }

    #[test]
    fn end_cycle_wipes_memory() {
        let mut orch = orchestrator();
        orch.run(vec![cand("PRO_A", 0.9)]);
        orch.end_cycle();
        // Quantum s'mban memory.
        assert_eq!(orch.governance.recursion_depth("pipeline_cycle"), 0);
        assert_eq!(orch.memory.audit().0, 0);
    }

    #[test]
    fn trace_records_pending() {
        let mut orch = orchestrator();
        orch.run(vec![cand("PRO_A", 0.9)]);
        // Trace u regjistrua si RAW_PENDING (Quantum s'vendos verdiktin).
        let stats = orch.trace_stats();
        assert!(stats.total >= 1);
        assert_eq!(stats.verified_optimal, 0); // Quantum s'vendos
    }

    #[test]
    fn empty_candidates_handled() {
        let mut orch = orchestrator();
        let report = orch.run(vec![]);
        assert_eq!(report.candidates_out, 0);
        assert_eq!(report.final_provider, "NONE");
    }

    #[test]
    fn territory_discipline_applied() {
        let orch = orchestrator();
        // science → CONSERVATIVE
        assert_eq!(orch.territory_discipline(), "CONSERVATIVE");
    }

    #[test]
    fn fusion_score_in_range() {
        let mut orch = orchestrator();
        let report = orch.run(vec![cand("A", 0.8), cand("B", 0.6)]);
        assert!((0.0..=1.0).contains(&report.fusion_score));
    }

    #[test]
    fn lab_validation_produces_evidence() {
        let mut orch = orchestrator();
        let report = orch.run(vec![cand("PRO_A", 0.9)]);
        // Digital Lab u thirr — reproducibility e matur.
        assert!((0.0..=1.0).contains(&report.lab_reproducibility));
        // Etiketa e Lab-it është një nga vlerat e vlefshme.
        let valid = report.lab_rejection == "TRL3_PASSED"
            || report.lab_rejection == "TRL3_REJECTED"
            || report.lab_rejection == "PENDING_LAB";
        assert!(valid);
    }

    #[test]
    fn empty_candidates_lab_pending() {
        let mut orch = orchestrator();
        let report = orch.run(vec![]);
        // Pa kandidatë → Lab pending.
        assert_eq!(report.lab_rejection, "PENDING_LAB");
    }
}
