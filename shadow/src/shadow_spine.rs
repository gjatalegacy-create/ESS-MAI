// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  SHADOW_SPINE.RS — Kurrizi Vertikal i Shadow-it (PATCHIM)            ║
// ║  GJATA LEGACY™ — ESS-MAI Shadow                          (SPINE)       ║
// ║                                                                          ║
// ║  PATCHIM I THELLË: lidh shtresat e Shadow-it në NJË rrjedhë koherente,  ║
// ║  pa shkelur asnjë rol. Para patchimit, Layer 2 dhe Layer 3 ishin ishuj  ║
// ║  të shkëputur — ky kurriz i bashkon vertikalisht.                      ║
// ║                                                                          ║
// ║  RRJEDHA E PLOTË E SHADOW-it (si një trup):                           ║
// ║    1. Night Watch (L3)        → gjykim paraprak mbi 7 shtresa           ║
// ║    2. Final Verdict Court (L2) → vendimi suprem (gjashtë gjykata)       ║
// ║    3. Verification Memory (L2) → thith ledger + negative (mëson)        ║
// ║    4. Wisdom Warehouse (L3)    → regjistron lëvizjen (precedent)        ║
// ║                                                                          ║
// ║  KUFIRI: Shadow vendos/gjykon/trashëgon — kurrizi vetëm ORKESTRON       ║
// ║  rrjedhën ekzistuese, s'shton vendim të ri. Zero if/else.             ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::lab_contracts::evidence::{EvidenceKind, EvidencePackage};
use crate::lab_contracts::pressure::{PressureReport, PressureSource};
use crate::lab_contracts::trl::{TrlEvidence, TrlVector};
use crate::lab_contracts::trust::{TrustContext, TrustGuidance};
use crate::lab_contracts::verdict::VerificationState;
use crate::types::{LightEnvelope, PassPackage};
use sha2::{Digest, Sha256};
use crate::lab_contracts_v11::watch::NightWatchReport;
use crate::layer2::shadow_courts::{FinalVerdictCourt, SupremeVerification};
use crate::layer2::verification_memory::{VerificationMemoryIndex, Precedent};
use crate::layer3::shadow_night_watch::{NightWatch, WatchVerdict};
use crate::layer3::shadow_wisdom::{ShadowWisdom, WisdomWarehouse};

// ─────────────────────────────────────────────────────────────────────────────
// SHADOW SPINE RESULT — rezultati i rrjedhës së plotë vertikale
// ─────────────────────────────────────────────────────────────────────────────

/// Rezultati i plotë i kurrizit të Shadow-it — të gjitha shtresat në një.
#[derive(Debug, Clone)]
pub struct ShadowSpineResult {
    /// Gjykimi paraprak i sentinelit (L3).
    pub watch_verdict:   WatchVerdict,
    /// Vendimi suprem (L2).
    pub supreme:         SupremeVerification,
    /// Precedenti i përditësuar pas mësimit (L2).
    pub precedent:       Precedent,
    /// A u përforcua kujtesa (ledger u thith).
    pub memory_updated:  bool,
}

// ─────────────────────────────────────────────────────────────────────────────
// SHADOW SPINE — orkestron rrjedhën vertikale L3→L2→L2→L3
// ─────────────────────────────────────────────────────────────────────────────

pub struct ShadowSpine;

impl ShadowSpine {
    /// adjudicate_full — rrjedha e plotë e Shadow-it nga sentineli te kujtesa.
    ///
    /// Lidh të 4 hapat: night_watch → final_verdict → memory → wisdom.
    /// Çdo hap përdor modulet ekzistuese (s'shton vendim të ri). Zero if.
    pub fn adjudicate_full(
        pkg: &EvidencePackage,
        watch_report: &NightWatchReport,
        memory: &mut VerificationMemoryIndex,
        warehouse: &mut WisdomWarehouse,
        recorded_at: u64,
    ) -> ShadowSpineResult {
        // HAP 1 (L3): Night Watch gjykon paraprakisht mbi 7 shtresat.
        let watch_verdict = NightWatch::judge(watch_report);

        // HAP 2 (L2): Final Verdict Court jep vendimin suprem.
        let supreme = FinalVerdictCourt::adjudicate(pkg, recorded_at);

        // HAP 3 (L2): Verification Memory thith ledger-in (mëson nga vendimi).
        memory.absorb_ledger(supreme.ledger_entry.clone());
        // Nëse ka negative knowledge, thithet gjithashtu (dështimi = aset).
        Self::absorb_negative_if_present(memory, &supreme);
        let memory_updated = true;

        // Precedenti i përditësuar (sinjali që Light do lexojë).
        let precedent = memory.precedent_for(pkg.claim_id);

        // HAP 4 (L3): Wisdom Warehouse regjistron lëvizjen (për të 3 platformat).
        let significance = supreme.ledger_entry.strength();
        let note = ShadowWisdom::note_movement(
            pkg.claim_id,
            pkg.trust_context.domain.clone(),
            Self::movement_description(&supreme, &watch_verdict),
            significance,
            recorded_at,
        );
        warehouse.store(note);

        ShadowSpineResult {
            watch_verdict,
            supreme,
            precedent,
            memory_updated,
        }
    }

    /// absorb_negative_if_present — thith negative knowledge nëse ekziston.
    /// Zero if — match mbi Option.
    fn absorb_negative_if_present(memory: &mut VerificationMemoryIndex, supreme: &SupremeVerification) {
        // Match mbi Option<NegativeKnowledgeRecord> — zero if.
        match &supreme.negative_record {
            Some(record) => memory.absorb_negative(record.clone()),
            None => {}
        }
    }

    /// movement_description — përshkrim i lëvizjes për urtësi. Zero if — match.
    fn movement_description(supreme: &SupremeVerification, watch: &WatchVerdict) -> String {
        let verdict_bit = supreme.verdict.bit();
        format!(
            "shadow_spine: verdict={} | watch_bit={} | state={}",
            verdict_bit,
            watch.bit,
            supreme.state.label()
        )
    }

    /// precedent_signal_for_light — nxjerr sinjalin f32 që Light do lexojë.
    /// Mbyll ciklin: Shadow mëson → Light ndërton besim më të zgjuar. Zero if.
    pub fn precedent_signal_for_light(result: &ShadowSpineResult) -> f32 {
        result.precedent.trust_signal()
    }

    /// Porta production e kurrizit Shadow. Gjendja mbahet nga runtime-i,
    /// ndërsa ShadowSpine garanton rendin e vetëm të multi-verifikimit.
    pub fn adjudicate_runtime(
        runtime: &mut ShadowVerificationRuntime,
        input_id: u64,
        pkg: &PassPackage,
        light: &LightEnvelope,
        recorded_at: u64,
    ) -> MultiVerificationAttestation {
        runtime.adjudicate_runtime(input_id, pkg, light, recorded_at)
    }
}


// ─────────────────────────────────────────────────────────────────────────────
// PRODUCTION MULTI-VERIFICATION RUNTIME — v1.6.9
// ─────────────────────────────────────────────────────────────────────────────

/// Dëshmia e brendshme e shumë-verifikimit. Nuk është verdict publik:
/// vetëm S.Judiciary dhe ShadowGjLegacy mbeten autoritetet e vendimit.
#[derive(Debug, Clone, Copy)]
pub struct MultiVerificationAttestation {
    pub state: VerificationState,
    pub court_bit: u8,
    pub watch_bit: Option<u8>,
    pub watch_score: f32,
    pub final_bit: u8,
    pub score: f32,
    pub precedent_signal: f32,
    pub reason_code: u8,
}

/// Gjendja e kujtesës së shumë-verifikimit që jeton brenda procesit Shadow.
/// Clone përdoret për staging: memoria angazhohet vetëm pasi pipeline-i kalon.
#[derive(Debug, Clone)]
pub struct ShadowVerificationRuntime {
    memory: VerificationMemoryIndex,
    warehouse: WisdomWarehouse,
}

impl ShadowVerificationRuntime {
    pub fn new() -> Self {
        Self {
            memory: VerificationMemoryIndex::new(),
            warehouse: WisdomWarehouse::new(),
        }
    }

    pub fn ledger_size(&self) -> usize { self.memory.ledger_size() }
    pub fn wisdom_count(&self) -> usize { self.warehouse.count() }

    /// Kryen verifikimin e shumëfishtë mbi TË NJËJTIN state hyrës:
    /// Evidence/Adversarial/TRL/CrossDomain + NightWatch opsional + precedent.
    /// Rezultati nuk shkruhet në vault dhe nuk prodhon verdict publik.
    pub fn adjudicate_runtime(
        &mut self,
        input_id: u64,
        pkg: &PassPackage,
        light: &LightEnvelope,
        recorded_at: u64,
    ) -> MultiVerificationAttestation {
        let evidence = Self::evidence_from_runtime(input_id, pkg, light, recorded_at);
        let watch = light.device_watch.as_ref().map(NightWatch::judge);
        let mut supreme = FinalVerdictCourt::adjudicate(&evidence, recorded_at);
        let court_bit = supreme.verdict.bit();
        let watch_bit = watch.as_ref().map(|v| v.bit);
        let watch_score = watch.as_ref().map(|v| v.confidence).unwrap_or(1.0);
        let sentinel_bit = watch_bit.unwrap_or(1);
        let final_bit = court_bit & sentinel_bit;
        // Ledger-i duhet të regjistrojë rezultatin real të të gjithë verifikimeve.
        supreme.ledger_entry.final_verdict = final_bit;
        self.memory.absorb_ledger(supreme.ledger_entry.clone());
        Self::absorb_negative_if_present(&mut self.memory, &supreme);
        let precedent = self.memory.precedent_for(input_id);
        let precedent_signal = precedent.trust_signal();
        let court_score = supreme.ledger_entry.strength();
        let score = (court_score * watch_score * (0.75 + 0.25 * precedent_signal))
            .clamp(0.0, 1.0) * final_bit as f32;
        let description = format!(
            "multi_verification: court={} watch={} final={} state={} precedent={:.4}",
            court_bit,
            watch_bit.map(|v| v.to_string()).unwrap_or_else(|| "NA".to_string()),
            final_bit,
            supreme.state.label(),
            precedent_signal,
        );
        self.warehouse.store(ShadowWisdom::note_movement(
            input_id,
            light.territory.clone(),
            description,
            score,
            recorded_at,
        ));
        MultiVerificationAttestation {
            state: supreme.state,
            court_bit,
            watch_bit,
            watch_score,
            final_bit,
            score,
            precedent_signal,
            reason_code: match (court_bit, sentinel_bit) {
                (1, 1) => 0xA9,
                (0, _) => 0xA1,
                (_, 0) => 0xA3,
                _ => 0xAF,
            },
        }
    }

    fn fixed01(value: u32) -> f32 {
        (value as f32 / 10_000.0).clamp(0.0, 1.0)
    }

    fn evidence_from_runtime(
        input_id: u64,
        pkg: &PassPackage,
        light: &LightEnvelope,
        recorded_at: u64,
    ) -> EvidencePackage {
        let project = pkg.scientific_project.as_ref();
        let fallback_conf = pkg.epistemic_mass
            .min(pkg.evidence_density)
            .min(pkg.reproducibility)
            .clamp(0.0, 1.0);
        let confidence = project
            .map(|p| Self::fixed01(p.trl_confidence))
            .unwrap_or(fallback_conf);
        let reproducibility = project
            .map(|p| Self::fixed01(p.trl_reproducibility))
            .unwrap_or(pkg.reproducibility.clamp(0.0, 1.0));
        let trl_level = project
            .map(|p| p.trl_level.min(4))
            .unwrap_or_else(|| TrlVector::trl_from_confidence(confidence).min(3));
        let trl_passed = project.map(|p| p.trl_passed).unwrap_or(trl_level > 0);
        let cross_domain_checks: Vec<String> = project
            .filter(|p| !p.domain.is_empty() && p.domain != light.territory)
            .map(|p| vec![format!("{}->{}", light.territory, p.domain)])
            .unwrap_or_default();
        let cross_domain_tested = !cross_domain_checks.is_empty();

        let contradiction = (1.0 - pkg.logical_coherence).clamp(0.0, 1.0);
        let weakness = (1.0 - ((pkg.causal_integrity + pkg.convergence_strength) * 0.5))
            .clamp(0.0, 1.0);
        let mut failures = Vec::new();
        match pkg.conservation_ok { true => {}, false => failures.push("conservation_failed".to_string()) }
        match pkg.candidate_scores.is_empty() { true => failures.push("candidate_set_empty".to_string()), false => {} }
        let survived_strength = pkg.reasoning_strength().min(1.0 - contradiction).min(1.0 - weakness);
        let survived_level = match (
            survived_strength >= 0.85,
            survived_strength >= 0.65,
            survived_strength >= 0.40,
        ) {
            (true, _, _) => 3,
            (false, true, _) => 2,
            (false, false, true) => 1,
            _ => 0,
        };
        let pressure = PressureReport {
            claim_id: input_id,
            source: PressureSource::QuantumNpro,
            contradiction_score: contradiction,
            weakness_score: weakness,
            failure_modes: failures,
            pressure_path: "QUANTUM_PRO>NPRO>PIM>SPINE9".to_string(),
            survived_level,
        };

        let sealed = light.is_sealed() as u8 as f32;
        let chain_strength = (light.proof_chain.len() as f32 / 4.0).clamp(0.0, 1.0);
        let trust_score = (sealed * 0.70 + chain_strength * 0.30).clamp(0.0, 1.0);
        let guidance = match (trust_score >= 0.75, trust_score >= 0.45, sealed > 0.0) {
            (true, _, _) => TrustGuidance::Recommended,
            (false, true, _) => TrustGuidance::Neutral,
            (false, false, true) => TrustGuidance::Risky,
            _ => TrustGuidance::Unknown,
        };
        let trust = TrustContext {
            trace_id: input_id,
            domain: light.territory.clone(),
            trust_score,
            reliability_score: chain_strength,
            verification_score: (sealed * chain_strength).clamp(0.0, 1.0),
            history_refs: light.proof_chain.clone(),
            guidance,
        };

        let mut evidence_chain: Vec<String> = light.proof_chain.iter()
            .map(|id| format!("LIGHT:{id:016x}"))
            .collect();
        match pkg.final_evidence_digest {
            0 => {},
            d => evidence_chain.push(format!("PIM:{d:016x}")),
        }
        match project {
            Some(p) if !p.evidence_sha256.is_empty() => evidence_chain.push(p.evidence_sha256.clone()),
            _ => {},
        }
        let kind = match project {
            Some(p) if p.trl_level >= 4 && p.trl_passed && !p.files.is_empty() && !p.lab_test_id.is_empty()
                => EvidenceKind::Operational,
            Some(p) if !p.files.is_empty() => EvidenceKind::Documented,
            _ => EvidenceKind::Reasoned,
        };

        let claim = project
            .map(|p| p.project_title.clone())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| pkg.package_id.clone());
        let reasoning = format!(
            "gcl={:016x};spine={:016x};action={:016x};mask={:016x}/{:016x};strength={:.6}",
            pkg.pd_gcl_process_digest,
            pkg.spine_completion_digest,
            pkg.quantum_action_state,
            pkg.quantum_action_mask,
            pkg.quantum_required_action_mask,
            pkg.reasoning_strength(),
        );
        let mut hasher = Sha256::new();
        hasher.update(b"ESSMAI_SHADOW_MULTI_VERIFICATION_V1");
        hasher.update(input_id.to_le_bytes());
        hasher.update(pkg.package_id.as_bytes());
        hasher.update(reasoning.as_bytes());
        hasher.update(light.territory.as_bytes());
        hasher.update(recorded_at.to_le_bytes());
        let evidence_hash: [u8; 32] = hasher.finalize().into();
        EvidencePackage {
            claim_id: input_id,
            claim,
            reasoning,
            trl_evidence: TrlEvidence {
                claim_id: input_id,
                trl_level,
                trl_passed,
                reasoning_depth: pkg.quantum_action_mask.count_ones()
                    .saturating_add(pkg.candidate_scores.len() as u32),
                cross_domain_tested,
                pressure_applied: true,
                confidence,
                reproducibility,
                evidence_hash,
                timestamp: recorded_at,
            },
            trust_context: trust,
            pressure_report: pressure,
            evidence_chain,
            cross_domain_checks,
            kind,
        }
    }
}

impl Default for ShadowVerificationRuntime {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lab_contracts::trl::TrlEvidence;
    use crate::lab_contracts::trust::{TrustContext, TrustGuidance};
    use crate::lab_contracts::pressure::{PressureReport, PressureSource};
    use crate::lab_contracts::evidence::EvidenceKind;
    use crate::lab_contracts_v11::watch::SevenLayerInfo;

    fn strong_pkg() -> EvidencePackage {
        let mut e = TrlEvidence::pending(1);
        e.trl_level = 3; e.trl_passed = true; e.confidence = 0.8;
        e.cross_domain_tested = true; e.timestamp = 100;
        let mut p = PressureReport::none(1, PressureSource::QuantumNpro);
        p.pressure_path = "NReverse".to_string();
        p.survived_level = 3;
        EvidencePackage {
            claim_id: 1, claim: "X".to_string(), reasoning: "Y".to_string(),
            trl_evidence: e,
            trust_context: TrustContext {
                trace_id: 1, domain: "physics".to_string(),
                trust_score: 0.7, reliability_score: 0.6, verification_score: 0.5,
                history_refs: vec![1], guidance: TrustGuidance::Recommended,
            },
            pressure_report: p,
            evidence_chain: vec![],
            cross_domain_checks: vec!["d2".to_string()],
            kind: EvidenceKind::Reasoned,
        }
    }

    fn full_watch() -> NightWatchReport {
        NightWatchReport {
            claim_id: 1,
            layers: SevenLayerInfo {
                policy_alignment: 0.8, literature_support: 0.7, source_consensus: 0.9,
                recency: 0.6, credibility: 0.85, contradiction_free: 0.75, cross_coverage: 0.7,
            },
            via_device: true, sources_count: 5, gathered_at: 100,
        }
    }

    #[test]
    fn full_flow_connects_all_layers() {
        let mut memory = VerificationMemoryIndex::new();
        let mut warehouse = WisdomWarehouse::new();
        let result = ShadowSpine::adjudicate_full(
            &strong_pkg(), &full_watch(), &mut memory, &mut warehouse, 1000,
        );
        // L3 night watch gjykoi.
        assert_eq!(result.watch_verdict.bit, 1);
        // L2 supreme vendosi.
        assert!(result.supreme.verdict.is_accepted());
        // L2 memory u përditësua.
        assert!(result.memory_updated);
        assert_eq!(memory.ledger_size(), 1);
        // L3 wisdom regjistroi.
        assert_eq!(warehouse.count(), 1);
    }

    #[test]
    fn precedent_feeds_light() {
        let mut memory = VerificationMemoryIndex::new();
        let mut warehouse = WisdomWarehouse::new();
        let result = ShadowSpine::adjudicate_full(
            &strong_pkg(), &full_watch(), &mut memory, &mut warehouse, 1000,
        );
        // Sinjali për Light del nga precedenti.
        let signal = ShadowSpine::precedent_signal_for_light(&result);
        assert!(signal >= 0.0 && signal <= 1.0);
    }

    #[test]
    fn memory_learns_over_repeated_calls() {
        let mut memory = VerificationMemoryIndex::new();
        let mut warehouse = WisdomWarehouse::new();
        // Tri thirrje për të njëjtin claim → kujtesa rritet.
        for t in 0..3 {
            ShadowSpine::adjudicate_full(&strong_pkg(), &full_watch(), &mut memory, &mut warehouse, 1000 + t);
        }
        assert_eq!(memory.ledger_size(), 3);
        // Precedenti tani ka histori.
        let p = memory.precedent_for(1);
        assert!(p.has_precedent());
        assert_eq!(p.total_attempts(), 3);
    }

    #[test]
    fn negative_knowledge_absorbed_on_rejection() {
        // Pkg me presion të dobët → refuzohet → negative knowledge thithet.
        let mut pkg = strong_pkg();
        pkg.pressure_report.survived_level = 0;
        pkg.pressure_report.failure_modes.push("breaks".to_string());
        let mut memory = VerificationMemoryIndex::new();
        let mut warehouse = WisdomWarehouse::new();
        ShadowSpine::adjudicate_full(&pkg, &full_watch(), &mut memory, &mut warehouse, 1000);
        // Negative knowledge u regjistrua.
        assert!(memory.is_known_failure(1));
    }

    fn runtime_pkg(id: &str, strength: f32) -> PassPackage {
        PassPackage {
            package_id: id.to_string(),
            epistemic_mass: strength,
            evidence_density: strength,
            logical_coherence: strength,
            causal_integrity: strength,
            convergence_strength: strength,
            reproducibility: strength,
            potentiality_score: strength,
            ibe_score: strength,
            chain_coherence: strength,
            conservation_ok: true,
            candidate_scores: vec![strength, strength],
            suggested_verdict: 1,
            final_evidence_digest: 0xAA55,
            quantum_action_state: 0x55AA,
            quantum_action_mask: 0x0F,
            quantum_required_action_mask: 0x0F,
            pd_gcl_process_digest: 0x11,
            spine_completion_digest: 0x22,
            scientific_project: None,
            raw_bytes: vec![1, 2, 3],
        }
    }

    #[test]
    fn production_runtime_multi_verification_is_stateful() {
        let mut runtime = ShadowVerificationRuntime::new();
        let light = LightEnvelope::sealed("s", "INDUSTRIAL", vec![1, 2, 3, 4], vec![]);
        let first = runtime.adjudicate_runtime(77, &runtime_pkg("p", 0.95), &light, 1000);
        let second = runtime.adjudicate_runtime(77, &runtime_pkg("p", 0.95), &light, 1001);
        assert_eq!(first.final_bit, 1);
        assert_eq!(second.final_bit, 1);
        assert_eq!(runtime.ledger_size(), 2);
        assert_eq!(runtime.wisdom_count(), 2);
        assert!(second.precedent_signal >= first.precedent_signal);
    }

    #[test]
    fn weak_runtime_package_fails_closed() {
        let mut runtime = ShadowVerificationRuntime::new();
        let light = LightEnvelope::stripped("s", "INDUSTRIAL", vec![], vec![]);
        let result = runtime.adjudicate_runtime(88, &runtime_pkg("weak", 0.10), &light, 1000);
        assert_eq!(result.final_bit, 0);
        assert_eq!(result.score, 0.0);
    }

}
