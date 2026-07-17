// ╔══════════════════════════════════════════════════════════════════════════╗
// ║ PROGRESSIVE_DEBATIC / runtime.rs — CIKLI PD→SPINE9→LAYERS→PRE-SEAL      ║
// ║                                                                          ║
// ║ 1) PD merr inputin dhe krijon PdSpineRequest — ende PA PendingNextI0.   ║
// ║ 2) PD aktivizon Spine 9; Layer 1/2/3 mbyllen dhe kthejnë completion.     ║
// ║ 3) Vetëm completion-i i plotë prodhon PendingNextI0 + PD pre-seal.      ║
// ║ 4) Shadow mbyll XY; receipt-i final liron PD Light → Nura.               ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::pd_spine_contract::{
    PdActivationContract, PdActivationOrigin, PdCognitiveMode,
    PdSpineCompletion, PdSpineRequest,
};
use super::core::ProgressiveDebatic;
use super::seal::PdSealAuthority;
use super::types::*;

impl ProgressiveDebatic {
    pub fn open_session_sealed(
        &mut self,
        session_id: &str,
        territory: &str,
        mode: DebateMode,
        ts: u64,
        runtime_seal: u64,
    ) -> Result<(), PdError> {
        PdSealAuthority::authorize_ingest(runtime_seal, || {
            self.open_session(session_id, territory, mode, ts)
        })
    }

    /// Compatibility entry: autorizon ingest-in, por nuk krijon pre-seal.
    pub fn ingest_sealed(
        &mut self,
        session_id: &str,
        content: &str,
        ts: u64,
        runtime_seal: u64,
    ) -> Result<PdEngineOutput, PdError> {
        PdSealAuthority::authorize_ingest(runtime_seal, || ())?;
        self.ingest(session_id, content, ts)
    }

    /// FAZA A — PD krijon output-in kognitiv dhe kërkesën e aktivizimit Spine 9.
    /// As PendingNextI0 dhe as pre-seal nuk ekzistojnë në këtë fazë.
    pub fn ingest_for_spine_sealed(
        &mut self,
        session_id: &str,
        parent_i0: &str,
        content: &str,
        ts: u64,
        runtime_seal: u64,
    ) -> Result<PdSpineCycle, PdError> {
        PdSealAuthority::authorize_ingest(runtime_seal, || {
            self.ingest(session_id, content, ts).map(|output| {
                let (trace_ref, turn_ref) = output_refs(&output, session_id);
                let (question, response_kind) = output_question(&output);
                let activation = activation_from_output(&output, response_kind);
                let request = PdSpineRequest::new_with_activation_under_gcl(
                    session_id,
                    parent_i0,
                    content,
                    &question,
                    response_kind,
                    trace_ref,
                    turn_ref,
                    ts,
                    activation,
                    crate::lab_contracts::gjata_collapse_law::law_seal(),
                    crate::lab_contracts::gjata_collapse_law::system_laws_seal(),
                    crate::lab_contracts::CollapsePhase::Reasoning.label(),
                );
                PdSpineCycle { output, request }
            })
        })?
    }

    /// FAZA B — rezultatet e Layer 1/2/3 kthehen te PD. Vetëm pas kësaj
    /// krijohen kandidati PendingNextI0 dhe pre-seal-i i lidhur me Spine 9.
    pub fn prepare_after_spine_sealed(
        &self,
        session_id: &str,
        parent_i0: &str,
        cycle: PdSpineCycle,
        completion: PdSpineCompletion,
        ts: u64,
        runtime_seal: u64,
    ) -> Result<PdPreparedCycle, PdError> {
        let same_activation = cycle.request == completion.request;
        let same_session = completion.request.session_digest
            == crate::pd_spine_contract::digest_parts(&[session_id.as_bytes()]);
        let same_parent = completion.request.parent_i0_digest
            == crate::pd_spine_contract::digest_parts(&[parent_i0.as_bytes()]);

        match (same_activation, same_session, same_parent, completion.closes_all_layers()) {
            (true, true, true, true) => {}
            (false, _, _, _) => return Err(PdError::BridgeFailed(
                "Spine 9 completion nuk i përket aktivizimit të PD".to_string(),
            )),
            (_, false, _, _) => return Err(PdError::BridgeFailed(
                "Spine 9 completion ka session tjetër".to_string(),
            )),
            (_, _, false, _) => return Err(PdError::BridgeFailed(
                "Spine 9 completion ka parent i₀ tjetër".to_string(),
            )),
            (_, _, _, false) => return Err(PdError::BridgeFailed(
                "Layer 1/2/3 nuk u mbyllën; PendingNextI0 refuzohet".to_string(),
            )),
        }

        let authorized = PdSealAuthority::authorize_after_spine(
            runtime_seal, session_id, parent_i0, ts, completion,
        )?;
        let pending_next = pending_from_output(
            &cycle.output, session_id, parent_i0, ts,
            authorized.completion.clone(), authorized.pre_seal.clone(),
        );
        Ok(PdPreparedCycle {
            output: cycle.output,
            pending_next,
            pre_seal: Some(authorized.pre_seal),
            spine_completion: authorized.completion,
            gcl_token: Some(authorized.gcl_token),
        })
    }

    pub fn finalize_after_verification(
        &self,
        pending: PdPendingNextI0,
        completion: PdVerificationCompletion,
    ) -> PdFinalization {
        let same_session = pending.session_id == completion.session_id;
        let same_parent = pending.parent_i0 == completion.parent_i0;
        let pre_sealed = PdSealAuthority::verifies(
            &pending.pre_seal,
            &pending.session_id,
            &pending.parent_i0,
        );
        let spine_closed = pending.spine_completion.closes_all_layers()
            && pending.spine_completion.completion_digest()
                == pending.pre_seal.spine_completion_digest;
        let expected_receipt = crate::lab_contracts::verification_receipt::receipt_id(
            &completion.session_id,
            &completion.parent_i0,
            completion.primitive_anchor,
            completion.xy_digest,
            completion.pd_binding_digest,
            completion.pd_continuum_activation_digest,
            &completion.living_trust_sha256,
            completion.y_verdict,
            completion.x_verdict,
            completion.verification_generation,
            completion.verification_seal,
        );
        let candidate_bound = completion.pd_binding_digest != 0
            && completion.pd_binding_digest == pending.binding_digest();
        let continuum_bound = completion.pd_continuum_activation_digest != 0
            && completion.pd_continuum_activation_digest
                == pending.spine_completion.request.continuum.activation_digest;
        let expected_trust_kind = match (completion.y_verdict, completion.x_verdict) {
            (1, 1) => crate::living_trust_contract::TRUST_KIND_CONSTRUCTIVE,
            (0, 0) => crate::living_trust_contract::TRUST_KIND_RIGOROUS_NEGATIVE,
            _ => crate::living_trust_contract::TRUST_KIND_NONE,
        };
        let living_trust = crate::living_trust_contract::LivingTrustSeal {
            identity_sha256: completion.living_trust_sha256,
            intensity: completion.living_trust_intensity,
            kind: completion.living_trust_kind,
            sovereign_value: completion.living_trust_sovereign_value,
        };
        let living_trust_valid = completion.living_trust_sha256 != [0u8; 32]
            && completion.living_trust_intensity
                <= crate::living_trust_contract::TRUST_INTENSITY_SCALE
            && completion.living_trust_kind == expected_trust_kind
            && completion.living_trust_sovereign_value
                == crate::living_trust_contract::SOVEREIGN_SEAL_PRIMITIVE
            && completion.living_trust_digest == living_trust.identity_digest();
        let verification_sealed = completion.verification_seal
                == EXPECTED_SHADOW_VERIFICATION_SEAL
            && completion.verification_generation != 0
            && completion.primitive_anchor != 0
            && completion.xy_digest != 0
            && completion.verification_receipt_id == expected_receipt
            && living_trust_valid;

        match (
            same_session,
            same_parent,
            pre_sealed,
            spine_closed,
            candidate_bound,
            continuum_bound,
            verification_sealed,
            completion.y_verdict,
            completion.x_verdict,
        ) {
            (true, true, true, true, true, true, true, 1, 1) => {
                let question = pending.prepared_question.clone();
                match build_continuum(
                    &pending,
                    &completion,
                    question,
                    PdContinuationBasis::VerifiedPositive,
                ) {
                    Ok(closure) => PdFinalization::Released(closure),
                    Err(reason) => PdFinalization::Rejected { reason },
                }
            }
            (true, true, true, true, true, true, true, 0, 0) => {
                let question = format!(
                    "Cila provë konkrete duhet të zëvendësojë bazën e rrëzuar: {}",
                    pending.prepared_question
                );
                match build_continuum(
                    &pending,
                    &completion,
                    question,
                    PdContinuationBasis::VerifiedNegativeRebuild,
                ) {
                    Ok(closure) => PdFinalization::RebuiltFromNegative(closure),
                    Err(reason) => PdFinalization::Rejected { reason },
                }
            }
            (false, _, _, _, _, _, _, _, _) => PdFinalization::Rejected {
                reason: "PD session nuk përputhet me dëftesën e Shadow".to_string(),
            },
            (_, false, _, _, _, _, _, _, _) => PdFinalization::Rejected {
                reason: "PD parent i₀ nuk përputhet me output-in e vulosur".to_string(),
            },
            (_, _, false, _, _, _, _, _, _) => PdFinalization::Rejected {
                reason: "kandidati PD nuk ka pre-seal të vlefshëm".to_string(),
            },
            (_, _, _, false, _, _, _, _, _) => PdFinalization::Rejected {
                reason: "pre-seal nuk është i lidhur me mbylljen e Spine 9".to_string(),
            },
            (_, _, _, _, false, _, _, _, _) => PdFinalization::Rejected {
                reason: "receipt-i i Shadow nuk është i lidhur me kandidatin PD".to_string(),
            },
            (_, _, _, _, _, false, _, _, _) => PdFinalization::Rejected {
                reason: "receipt-i i Shadow nuk mban aktivizimin i₀ + 1Q të PD".to_string(),
            },
            (_, _, _, _, _, _, false, _, _) => PdFinalization::Rejected {
                reason: "output-i final nuk ka seal/receipt kanonik të Shadow".to_string(),
            },
            _ => PdFinalization::Rejected {
                reason: format!(
                    "gjendje jo-kushtetuese Y={} X={} — pritej 1/1 ose 0/0",
                    completion.y_verdict, completion.x_verdict
                ),
            },
        }
    }
}

fn map_mode(mode: DebateMode) -> PdCognitiveMode {
    match mode {
        DebateMode::Intellect => PdCognitiveMode::Intellect,
        DebateMode::Philosophy => PdCognitiveMode::Philosophy,
        DebateMode::Scientific => PdCognitiveMode::Scientific,
        DebateMode::Novel => PdCognitiveMode::Novel,
    }
}

fn activation_from_output(output: &PdEngineOutput, response_kind: &str) -> PdActivationContract {
    match output {
        PdEngineOutput::Continue { turn } => PdActivationContract::new(
            map_mode(turn.mode_at_turn), PdActivationOrigin::NormalTurn, response_kind,
            turn.genius_check.map(u64::from).unwrap_or(0),
            turn.user_signal.idea_density, turn.user_signal.energy_level,
        ),
        PdEngineOutput::GeniusDetected { genius_signal, package } => PdActivationContract::new(
            PdCognitiveMode::Novel, PdActivationOrigin::GeniusAutonomousResearch, response_kind,
            crate::pd_spine_contract::digest_parts(&[
                genius_signal.signal_id.as_bytes(), genius_signal.trace_ref.as_bytes(),
                &genius_signal.genius_score.to_le_bytes(),
            ]),
            package.accumulated_mass, package.structural_coherence,
        ),
        PdEngineOutput::BudgetExhausted { .. } => PdActivationContract::new(
            PdCognitiveMode::Intellect, PdActivationOrigin::BudgetExhausted, response_kind, 0, 0, 0,
        ),
        PdEngineOutput::SessionClosed { .. } => PdActivationContract::new(
            PdCognitiveMode::Intellect, PdActivationOrigin::SessionClosed, response_kind, 0, 0, 0,
        ),
    }
}

fn output_refs<'a>(output: &'a PdEngineOutput, fallback: &'a str) -> (&'a str, &'a str) {
    match output {
        PdEngineOutput::Continue { turn } =>
            (&turn.user_signal.signal_id, &turn.turn_id),
        PdEngineOutput::GeniusDetected { genius_signal, package } =>
            (&genius_signal.trace_ref, &package.package_id),
        PdEngineOutput::BudgetExhausted { trace_id } =>
            (trace_id, trace_id),
        PdEngineOutput::SessionClosed { session_id, .. } =>
            (session_id, fallback),
    }
}

fn output_question(output: &PdEngineOutput) -> (String, &'static str) {
    match output {
        PdEngineOutput::Continue { turn } => (
            turn.pd_response_hint.clone(),
            turn.pd_response_type.label(),
        ),
        PdEngineOutput::GeniusDetected { package, .. } => (
            format!(
                "Cila provë konkrete e ngre konceptin '{}' në hapin tjetër?",
                package.dominant_concept
            ),
            PdResponseType::ScaffoldStructure.label(),
        ),
        PdEngineOutput::BudgetExhausted { .. } => (String::new(), "BUDGET_EXHAUSTED"),
        PdEngineOutput::SessionClosed { .. } => (String::new(), "SESSION_CLOSED"),
    }
}

fn pending_from_output(
    output: &PdEngineOutput,
    session_id: &str,
    parent_i0: &str,
    ts: u64,
    completion: PdSpineCompletion,
    pre_seal: PdPreSealReceipt,
) -> Option<PdPendingNextI0> {
    let candidate = match output {
        PdEngineOutput::Continue { turn } => {
            let (question, _) = output_question(output);
            Some((turn.turn_id.clone(), question, turn.pd_response_type))
        }
        PdEngineOutput::GeniusDetected { genius_signal, .. } => {
            let (question, _) = output_question(output);
            Some((
                genius_signal.signal_id.clone(),
                question,
                PdResponseType::ScaffoldStructure,
            ))
        }
        _ => None,
    };

    candidate.map(|(turn_id, prepared_question, response_type)| PdPendingNextI0 {
        session_id: session_id.to_string(),
        parent_i0: parent_i0.to_string(),
        turn_id,
        prepared_question,
        response_type,
        prepared_at: ts,
        spine_completion: completion,
        pre_seal,
    })
}

fn continuum_basis(basis: PdContinuationBasis) -> crate::pd_continuum_contract::PdContinuumBasis {
    match basis {
        PdContinuationBasis::VerifiedPositive =>
            crate::pd_continuum_contract::PdContinuumBasis::VerifiedPositive,
        PdContinuationBasis::VerifiedNegativeRebuild =>
            crate::pd_continuum_contract::PdContinuumBasis::VerifiedNegativeRebuild,
    }
}

fn build_continuum(
    pending: &PdPendingNextI0,
    completion: &PdVerificationCompletion,
    question: String,
    basis: PdContinuationBasis,
) -> Result<PdContinuumClosure, String> {
    use crate::pd_continuum_contract as continuum;

    let activation = &pending.spine_completion.request.continuum;
    let question_increment_digest = activation.question_increment.increment_digest;
    let shared_basis = continuum_basis(basis);
    let output_digest = continuum::derive_pd_output_digest(
        activation.activation_digest,
        pending.binding_digest(),
        question_increment_digest,
        &completion.verification_receipt_id,
        &question,
        completion.y_verdict,
        completion.x_verdict,
        shared_basis,
    );
    let output_id = continuum::pd_output_id(output_digest);
    let living_trust_digest = completion.living_trust_digest;
    let iz_digest = continuum::derive_iz_digest(
        activation.activation_digest,
        question_increment_digest,
        output_digest,
        living_trust_digest,
        &completion.verification_receipt_id,
        shared_basis,
    );
    let iz_id = continuum::iz_id(iz_digest);
    let mut iz_material = Vec::with_capacity(160);
    iz_material.extend_from_slice(b"ESS_MAI_IZ_V157");
    iz_material.extend_from_slice(&activation.activation_digest.to_le_bytes());
    iz_material.extend_from_slice(&question_increment_digest.to_le_bytes());
    iz_material.extend_from_slice(&output_digest.to_le_bytes());
    iz_material.extend_from_slice(completion.verification_receipt_id.as_bytes());
    iz_material.extend_from_slice(completion.living_trust_sha256.as_slice());
    iz_material.extend_from_slice(&completion.living_trust_intensity.to_le_bytes());
    iz_material.push(completion.living_trust_kind);
    iz_material.extend_from_slice(&completion.living_trust_sovereign_value.to_le_bytes());
    iz_material.extend_from_slice(&living_trust_digest.to_le_bytes());
    iz_material.extend_from_slice(pending.pre_seal.action_sha256.as_slice());
    iz_material.extend_from_slice(pending.pre_seal.law_trace_sha256.as_slice());
    let iz_sha256 = crate::sovereign::sha256(&iz_material);
    let next_i0_id = continuum::derive_next_i0_id(output_digest, iz_digest);
    let contract = continuum::PdContinuumCompletion::new(
        activation.activation_digest,
        question_increment_digest,
        output_digest,
        iz_digest,
        living_trust_digest,
        next_i0_id.clone(),
        &completion.verification_receipt_id,
        shared_basis,
    );

    match contract.closes_formula(&completion.verification_receipt_id) {
        false => return Err(
            "PD continuum nuk mbylli formulën output + iZ → next i₀".to_string(),
        ),
        true => {}
    }

    let output = PdVerifiedOutput {
        output_id: output_id.clone(),
        output_digest,
        session_id: pending.session_id.clone(),
        parent_i0: pending.parent_i0.clone(),
        turn_id: pending.turn_id.clone(),
        question: question.clone(),
        response_type: pending.response_type,
        mpro_mass: pending.spine_completion.mpro_mass.raw(),
        y_verdict: completion.y_verdict,
        x_verdict: completion.x_verdict,
        verification_receipt_id: completion.verification_receipt_id.clone(),
    };
    let iz = PdIzCompletion {
        iz_id: iz_id.clone(),
        iz_digest,
        iz_sha256,
        origin_i0: pending.parent_i0.clone(),
        question_increment_digest,
        pd_output_digest: output_digest,
        verification_receipt_id: completion.verification_receipt_id.clone(),
        basis,
        living_trust_sha256: completion.living_trust_sha256,
        living_trust_intensity: completion.living_trust_intensity,
        living_trust_kind: completion.living_trust_kind,
        living_trust_sovereign_value: completion.living_trust_sovereign_value,
        living_trust_digest,
    };
    let next_i0 = PdNextI0 {
        session_id: pending.session_id.clone(),
        parent_i0: pending.parent_i0.clone(),
        next_i0_id,
        question,
        basis,
        y_verdict: completion.y_verdict,
        x_verdict: completion.x_verdict,
        verification_seal: completion.verification_seal,
        verification_generation: completion.verification_generation,
        verification_receipt_id: completion.verification_receipt_id.clone(),
        primitive_anchor: completion.primitive_anchor,
        xy_digest: completion.xy_digest,
        pd_binding_digest: pending.binding_digest(),
        continuum_activation_digest: activation.activation_digest,
        question_increment_digest,
        pd_output_id: output_id,
        pd_output_digest: output_digest,
        iz_id,
        iz_digest,
        iz_sha256,
        living_trust_sha256: completion.living_trust_sha256,
        living_trust_intensity: completion.living_trust_intensity,
        living_trust_kind: completion.living_trust_kind,
        living_trust_sovereign_value: completion.living_trust_sovereign_value,
        living_trust_digest,
        continuum_completion_digest: contract.completion_digest,
    };
    Ok(PdContinuumClosure { output, iz, next_i0, contract })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pd_spine_contract::{
        FixedMass, PdLayerReceipt, LAYER_1_MASK, LAYER_2_MASK, LAYER_3_MASK, ALL_LAYERS_MASK,
    };

    fn spine_completion() -> PdSpineCompletion {
        let request = PdSpineRequest::new_with_activation_under_gcl(
            "s1",
            "i0-A",
            "input",
            "Cila është prova?",
            "CLARIFY",
            "trace-A",
            "turn-A",
            10,
            PdActivationContract::new(
                PdCognitiveMode::Intellect,
                PdActivationOrigin::NormalTurn,
                "CLARIFY",
                0,
                0,
                0,
            ),
            crate::lab_contracts::gjata_collapse_law::law_seal(),
            crate::lab_contracts::gjata_collapse_law::system_laws_seal(),
            crate::lab_contracts::CollapsePhase::Reasoning.label(),
        );
        let gcl = request.gcl.process_digest;
        let l1 = PdLayerReceipt::new(
            LAYER_1_MASK, request.activation_id, request.activation.contract_digest,
            gcl, 0, 11, true,
        );
        let l2 = PdLayerReceipt::new(
            LAYER_2_MASK, request.activation_id, request.activation.contract_digest,
            gcl, l1.result_digest, 12, true,
        );
        let l3 = PdLayerReceipt::new(
            LAYER_3_MASK, request.activation_id, request.activation.contract_digest,
            gcl, l2.result_digest, 13, true,
        );
        PdSpineCompletion {
            request, layer_mask: ALL_LAYERS_MASK, layer1: l1, layer2: l2, layer3: l3,
            evidence_digest: 0xC3, mpro_mass: FixedMass::from_ratio(8, 16),
            ready_for_shadow: true, completed_at: 11,
        }
    }

    fn pending() -> PdPendingNextI0 {
        let authorized = PdSealAuthority::authorize_after_spine(
            PD_SEAL, "s1", "i0-A", 12, spine_completion(),
        ).expect("pre-seal pas Spine9");
        PdPendingNextI0 {
            session_id: "s1".to_string(),
            parent_i0: "i0-A".to_string(),
            turn_id: "t1".to_string(),
            prepared_question: "Cila është prova?".to_string(),
            response_type: PdResponseType::Clarify,
            prepared_at: 12,
            spine_completion: authorized.completion,
            pre_seal: authorized.pre_seal,
        }
    }

    fn completion_for(pending: &PdPendingNextI0, y: u8, x: u8) -> PdVerificationCompletion {
        let session_id = pending.session_id.clone();
        let parent_i0 = pending.parent_i0.clone();
        let primitive_anchor = 0xA1;
        let xy_digest = 0xB2;
        let pd_binding_digest = pending.binding_digest();
        let pd_continuum_activation_digest =
            pending.spine_completion.request.continuum.activation_digest;
        let verification_generation = 7;
        let verification_seal = EXPECTED_SHADOW_VERIFICATION_SEAL;
        let trust_kind = match (y, x) {
            (1, 1) => crate::living_trust_contract::TRUST_KIND_CONSTRUCTIVE,
            (0, 0) => crate::living_trust_contract::TRUST_KIND_RIGOROUS_NEGATIVE,
            _ => crate::living_trust_contract::TRUST_KIND_NONE,
        };
        let living_trust = crate::living_trust_contract::LivingTrustSeal {
            identity_sha256: [0xAB; 32],
            intensity: 7_500,
            kind: trust_kind,
            sovereign_value: crate::living_trust_contract::SOVEREIGN_SEAL_PRIMITIVE,
        };
        let living_trust_digest = living_trust.identity_digest();
        let verification_receipt_id =
            crate::lab_contracts::verification_receipt::receipt_id(
                &session_id, &parent_i0, primitive_anchor, xy_digest,
                pd_binding_digest, pd_continuum_activation_digest,
                &living_trust.identity_sha256, y, x, verification_generation, verification_seal,
            );
        PdVerificationCompletion {
            session_id,
            parent_i0,
            primitive_anchor,
            xy_digest,
            pd_binding_digest,
            pd_continuum_activation_digest,
            y_verdict: y,
            x_verdict: x,
            verification_seal,
            verification_generation,
            verification_receipt_id,
            living_trust_sha256: living_trust.identity_sha256,
            living_trust_intensity: living_trust.intensity,
            living_trust_kind: living_trust.kind,
            living_trust_sovereign_value: living_trust.sovereign_value,
            living_trust_digest,
        }
    }

    #[test]
    fn preseal_cannot_exist_without_all_layers() {
        let mut c = spine_completion();
        c.layer_mask = crate::pd_spine_contract::LAYER_2_MASK
            | crate::pd_spine_contract::LAYER_3_MASK;
        let r = PdSealAuthority::authorize_after_spine(
            PD_SEAL, "s1", "i0-A", 12, c,
        );
        assert!(matches!(r, Err(PdError::BridgeFailed(_))));
    }

    #[test]
    fn pending_cannot_become_next_i0_without_verification_receipt() {
        let pd = ProgressiveDebatic::new();
        let p = pending();
        let mut c = completion_for(&p, 1, 1);
        c.verification_seal = 0;
        assert!(matches!(
            pd.finalize_after_verification(p, c),
            PdFinalization::Rejected { .. }
        ));
    }

    #[test]
    fn verified_xy_releases_output_plus_iz_as_next_i0() {
        let pd = ProgressiveDebatic::new();
        let p = pending();
        let receipt = completion_for(&p, 1, 1);
        match pd.finalize_after_verification(p, receipt.clone()) {
            PdFinalization::Released(closure) => {
                assert!(closure.contract.closes_formula(
                    &receipt.verification_receipt_id,
                ));
                assert_eq!(
                    closure.next_i0.next_i0_id,
                    crate::pd_continuum_contract::derive_next_i0_id(
                        closure.output.output_digest,
                        closure.iz.iz_digest,
                    ),
                );
                assert_ne!(closure.output.output_id, closure.iz.iz_id);
            }
            other => panic!("pritej Released, u mor: {other:?}"),
        }
    }

    #[test]
    fn verified_negative_rebuilds_deterministically() {
        let pd = ProgressiveDebatic::new();
        let p = pending();
        assert!(matches!(
            pd.finalize_after_verification(p.clone(), completion_for(&p, 0, 0)),
            PdFinalization::RebuiltFromNegative(_)
        ));
    }

    #[test]
    fn receipt_for_another_pd_candidate_is_rejected() {
        let pd = ProgressiveDebatic::new();
        let p = pending();
        let mut other = pending();
        other.prepared_question.push_str(" tjetër");
        let completion = completion_for(&other, 1, 1);
        assert!(matches!(
            pd.finalize_after_verification(p, completion),
            PdFinalization::Rejected { .. }
        ));
    }
}
