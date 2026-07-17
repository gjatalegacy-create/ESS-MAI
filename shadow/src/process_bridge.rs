//! v1.6.4 — complete mediation + scientific project continuum + Living Trust.
//!
//! Only the Shadow executable calls this module. It decodes public wire
//! contracts, opens the sovereign persistent vault, feeds the Light PA gate,
//! invokes the internal Shadow core, and writes a sealed response. No other
//! crate can link the core because the package has `autolib = false`.

use shadow_contracts::{
    LightInboundWire, NegativeKnowledgeWire, QuantumInboundWire,
    ProjectContextWitnessWire, ProjectRegistrationRequestWire,
    ProjectRegistrationResponseWire, ShadowCycleResponse,
    ShadowVerdictWire, VerificationReceiptWire,
};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use std::fs::OpenOptions;
use std::io::Write as _;

pub fn dispatch_from_args() -> Option<i32> {
    let args: Vec<String> = std::env::args().collect();
    match args.as_slice() {
        [_, flag, request, response] if flag == "--bridge-once" => {
            Some(exit_code(run_cycle(Path::new(request), Path::new(response))))
        }
        #[cfg(feature = "dev_harness")]
        [_, flag, request, response] if flag == "--negative-once" => {
            Some(exit_code(run_negative(Path::new(request), Path::new(response))))
        }
        [_, flag, request, response] if flag == "--project-register-once" => {
            Some(exit_code(run_project_registration(Path::new(request), Path::new(response))))
        }
        [_, flag, ..] if flag.starts_with("--bridge")
            || flag.starts_with("--negative")
            || flag.starts_with("--project") => {
            eprintln!("[SHADOW MAIN] argumente të pavlefshme për portën e ndërmjetësuar");
            Some(64)
        }
        _ => None,
    }
}

fn exit_code(result: Result<(), String>) -> i32 {
    match result {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("[SHADOW MAIN] FAIL-CLOSED: {e}");
            70
        }
    }
}


/// Dry ndër-proces fail-closed për APUPK WAL. Shadow main ekzekutohet si
/// proces one-shot; mutex-i i SovereignLog mbron vetëm thread-et e një procesi.
/// `create_new` garanton një shkrues/lexues sovran në një kohë. Një proces i
/// rrëzuar mund të lërë lock-un; kjo kërkon ndërhyrje operatori dhe nuk
/// rikuperohet automatikisht, sepse rikuperimi pa attestation do të ishte
/// hamendësim mbi pronësinë e magazinës.
struct ApupkProcessLock {
    path: PathBuf,
}

impl ApupkProcessLock {
    fn acquire() -> Result<Self, String> {
        let path = PathBuf::from(super::handoff_path("shadow_apupk_v163.lock"));
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .map_err(|e| format!(
                "APUPK WAL është i zënë ose ka lock të pambyllur '{}': {e}",
                path.display(),
            ))?;
        writeln!(file, "pid={}\nmode=shadow-main-mediation", std::process::id())
            .map_err(|e| format!("APUPK lock s'u identifikua: {e}"))?;
        file.flush().map_err(|e| format!("APUPK lock s'u flush-ua: {e}"))?;
        file.sync_all().map_err(|e| format!("APUPK lock s'u fsync-ua: {e}"))?;
        Ok(Self { path })
    }
}

impl Drop for ApupkProcessLock {
    fn drop(&mut self) {
        match std::fs::remove_file(&self.path) {
            Ok(()) => {}
            Err(e) => eprintln!(
                "[SHADOW MAIN/APUPK] ALARM: lock '{}' s'u hoq: {e}",
                self.path.display(),
            ),
        }
    }
}

fn init_apupk_for_process() -> Result<(), String> {
    let path = PathBuf::from(super::handoff_path("shadow_apupk_v163.wal"));
    match crate::shadow_apupk::init_apupk_disk(&path) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => Ok(()),
        Err(e) => Err(format!("APUPK WAL s'u hap: {e}")),
    }
}

fn run_project_registration(request_path: &Path, response_path: &Path) -> Result<(), String> {
    require_main_mediation("project-register-once")?;
    let request_bytes = std::fs::read(request_path)
        .map_err(|e| format!("kërkesa APUPK s'u lexua: {e}"))?;
    let request = shadow_contracts::decode_project_registration_request(&request_bytes)
        .map_err(|e| format!("kërkesa APUPK u refuzua: {e}"))?;

    let response = match validate_project_registration(&request) {
        Ok(()) => {
            let _apupk_lock = ApupkProcessLock::acquire()?;
            init_apupk_for_process()?;
            let shadow = crate::Shadow::with_disk(super::wisdom_vault_path())
                .map_err(|e| format!("vault-i sovran s'u hap për APUPK: {e}"))?;
            let inbound = crate::shadow_apupk::ApupkInbound {
                trace: crate::shadow_apupk::ApupkTrace {
                    trace_id: request.trace_id,
                    project_id: request.project_id,
                    user_id: request.user_id,
                    initial_trace: request.initial_trace.clone(),
                    timestamp_ns: request.last_update_ns,
                },
                initial_progress: crate::shadow_apupk::ProjectProgress {
                    project_id: request.project_id,
                    progress_percentage: request.progress_percentage,
                    last_update_ns: request.last_update_ns,
                    notes: request.notes.clone(),
                },
                project_title: request.project_title.clone(),
                light_sovereign_flags: request.light_sovereign_flags,
                project_content: request.project_content.clone(),
            };
            shadow.receive_apupk_durable(inbound)
                .map_err(|e| format!("Shadow APUPK refuzoi ruajtjen durable të projektit: {e:?}"))?;
            let entry = crate::shadow_apupk::apupk_memory().get(request.project_id)
                .ok_or_else(|| "APUPK store nuk e riktheu projektin e saporuajtur".to_string())?;
            let content_sha256 = format!("{:x}", Sha256::digest(entry.content.as_bytes()));
            let context_sha256 = crate::gcl_project_contract::context_sha256(
                &crate::gcl_project_contract::ProjectContextMaterial {
                    project_id: entry.project_id,
                    user_id: entry.user_id,
                    trace_id: entry.trace_id,
                    revision: entry.revisions,
                    project_title: &entry.project_title,
                    content_sha256: &content_sha256,
                    light_sovereign_flags: entry.light_sovereign_flags,
                },
            );
            ProjectRegistrationResponseWire {
                accepted: true,
                reason_code: 0,
                witness: Some(ProjectContextWitnessWire {
                    project_id: entry.project_id,
                    user_id: entry.user_id,
                    trace_id: entry.trace_id,
                    revision: entry.revisions,
                    content_sha256,
                    light_sovereign_flags: entry.light_sovereign_flags,
                    context_sha256,
                }),
            }
        }
        Err(reason_code) => ProjectRegistrationResponseWire {
            accepted: false,
            reason_code,
            witness: None,
        },
    };
    atomic_write(
        response_path,
        &shadow_contracts::encode_project_registration_response(&response),
    )
}

fn validate_project_registration(request: &ProjectRegistrationRequestWire) -> Result<(), u8> {
    match (
        request.project_id != 0,
        request.user_id != 0,
        request.trace_id != 0,
        !request.project_title.trim().is_empty(),
        request.trace_id == crate::gcl_project_contract::project_trace_id(
            request.project_id, request.user_id, &request.project_title,
        ),
        !request.initial_trace.is_empty(),
        !request.project_content.is_empty(),
        request.progress_percentage.is_finite(),
        crate::gcl_project_contract::seal_is_500(request.light_sovereign_flags),
    ) {
        (true, true, true, true, true, true, true, true, true) => Ok(()),
        (false, _, _, _, _, _, _, _, _) => Err(1),
        (_, false, _, _, _, _, _, _, _) => Err(2),
        (_, _, false, _, _, _, _, _, _) => Err(3),
        (_, _, _, false, _, _, _, _, _) => Err(4),
        (_, _, _, _, false, _, _, _, _) => Err(5),
        (_, _, _, _, _, false, _, _, _) => Err(6),
        (_, _, _, _, _, _, false, _, _) => Err(7),
        (_, _, _, _, _, _, _, false, _) => Err(8),
        (_, _, _, _, _, _, _, _, false) => Err(9),
    }
}

fn run_cycle(request_path: &Path, response_path: &Path) -> Result<(), String> {
    require_main_mediation("bridge-once")?;
    let request_bytes = std::fs::read(request_path)
        .map_err(|e| format!("kërkesa wire s'u lexua: {e}"))?;
    let request = shadow_contracts::decode_cycle_request(&request_bytes)
        .map_err(|e| format!("kërkesa wire u refuzua: {e}"))?;

    validate_cycle_identity(&request.quantum, &request.light)?;
    validate_negative_shape(&request.negative)?;
    validate_final_evidence(&request.quantum, &request.negative)?;
    let _apupk_lock = match request.quantum.final_evidence.scientific_project.is_some() {
        true => {
            let lock = ApupkProcessLock::acquire()?;
            init_apupk_for_process()?;
            validate_scientific_project_against_apupk(&request.quantum)?;
            Some(lock)
        }
        false => None,
    };

    let vault_path = super::wisdom_vault_path();
    let shadow = crate::Shadow::with_disk(&vault_path)
        .map_err(|e| format!("vault-i sovran s'u hap: {e}"))?;

    let pa_path = super::pa_export_path();
    let fed = shadow
        .feed_primitive_anchors(&pa_path, &request.quantum.session_id)
        .map_err(|e| format!("PA gate refuzoi sesionin: {e}"))?;
    println!(
        "  [SHADOW MAIN/MEDIATION] session={} PA_fed={} vault={}",
        request.quantum.session_id, fed, vault_path
    );

    let negative = request.negative;
    let response = shadow
        .ingest_bridged(into_quantum(request.quantum), into_light(request.light))
        .map_err(|e| format!("ingest_bridged dështoi: {e}"))?;

    let must_persist_negative = matches!(
        (response.verdict.verified, response.verdict.primitive, response.verdict.project_status),
        (0, 0, _) | (_, _, shadow_contracts::PROJECT_STATUS_RIGOROUS_NEGATIVE)
    );
    let negative_persisted = match must_persist_negative {
        true => persist_negative(&shadow, &negative).is_ok(),
        false => false,
    };
    match (must_persist_negative, negative_persisted) {
        (true, true) | (false, false) => {}
        (true, false) => eprintln!(
            "  [SHADOW MAIN/MEDIATION] negative e verifikuar nuk u persistua për input={}",
            negative.input_id
        ),
        (false, true) => unreachable!("positive/hold path cannot persist negative knowledge"),
    }

    let receipt = response.verification_receipt;
    let verdict = response.verdict;
    let wire = ShadowCycleResponse {
        session_id: response.session_id,
        verdict: ShadowVerdictWire {
            verified: verdict.verified,
            primitive: verdict.primitive,
            knowledge_band: verdict.knowledge_band.as_u8(),
            lgc_law: verdict.lgc_law,
            legacy_score: verdict.legacy_score,
            lgc_dsl: verdict.lgc_dsl,
            living_trust_action_state: verdict.living_trust_action_state,
            living_trust_action_mask: verdict.living_trust_action_mask,
            living_trust_required_action_mask: verdict.living_trust_required_action_mask,
            living_trust_system_laws_seal: verdict.living_trust_system_laws_seal,
            living_trust_sovereign_flags: verdict.living_trust_sovereign_flags,
            living_trust_sovereign_value: verdict.living_trust_sovereign_value,
            living_trust_sha256:
                crate::living_trust_contract::hex_32(&verdict.living_trust_sha256),
            living_trust_intensity: verdict.living_trust_intensity,
            living_trust_kind: verdict.living_trust_kind,
            project_id: verdict.project_id,
            project_status: verdict.project_status,
            project_context_digest: verdict.project_context_digest,
            project_evidence_digest: verdict.project_evidence_digest,
            project_context_sha256:
                crate::gcl_project_contract::sha256_hex(&verdict.project_context_sha256),
            project_evidence_sha256:
                crate::gcl_project_contract::sha256_hex(&verdict.project_evidence_sha256),
            novel_factualized: verdict.novel_factualized,
            novel_trl_level: verdict.novel_trl_level,
            novel_proof_score: verdict.novel_proof_score,
            novel_rejection: verdict.novel_rejection,
        },
        verification_receipt: VerificationReceiptWire {
            session_id: receipt.session_id,
            parent_i0: receipt.parent_i0,
            primitive_anchor: receipt.primitive_anchor,
            xy_digest: receipt.xy_digest,
            pd_binding_digest: receipt.pd_binding_digest,
            pd_continuum_activation_digest: receipt.pd_continuum_activation_digest,
            living_trust_digest: receipt.living_trust_digest,
            living_trust_sha256:
                crate::living_trust_contract::hex_32(&receipt.living_trust_sha256),
            y_verdict: receipt.y_verdict,
            x_verdict: receipt.x_verdict,
            seal: receipt.seal,
            generation: receipt.generation,
            receipt_id: receipt.receipt_id,
        },
        negative_persisted,
    };
    atomic_write(response_path, &shadow_contracts::encode_cycle_response(&wire))
}

#[cfg(feature = "dev_harness")]
fn run_negative(request_path: &Path, response_path: &Path) -> Result<(), String> {
    require_main_mediation("negative-once")?;
    let request_bytes = std::fs::read(request_path)
        .map_err(|e| format!("kërkesa negative s'u lexua: {e}"))?;
    let request = shadow_contracts::decode_negative_request(&request_bytes)
        .map_err(|e| format!("kërkesa negative u refuzua: {e}"))?;
    validate_negative_shape(&request.negative)?;

    let shadow = crate::Shadow::with_disk(super::wisdom_vault_path())
        .map_err(|e| format!("vault-i sovran s'u hap: {e}"))?;
    let persisted = persist_negative(&shadow, &request.negative).is_ok();
    let negative_export = match (persisted, request.export_negative_knowledge) {
        (true, true) => shadow.export_negative_knowledge(),
        _ => Vec::new(),
    };
    let response = shadow_contracts::ShadowNegativeResponse { persisted, negative_export };
    atomic_write(response_path, &shadow_contracts::encode_negative_response(&response))
}

fn require_main_mediation(site: &str) -> Result<(), String> {
    match crate::phase9_integration::Phase9Integration::verify_no_bypass(site) {
        true => Ok(()),
        false => Err(format!("Phase9 refuzoi bypass-in te {site}")),
    }
}

fn validate_negative_shape(n: &NegativeKnowledgeWire) -> Result<(), String> {
    match (
        n.mass.is_finite(),
        n.suggestion_code <= 2,
        !n.process.is_empty(),
        !n.bytes.is_empty(),
    ) {
        (true, true, true, true) => Ok(()),
        (false, _, _, _) => Err("Negative Knowledge ka masë NaN/Infinity".to_string()),
        (_, false, _, _) => Err("Negative Knowledge ka suggestion_code jashtë {0,1,2}".to_string()),
        (_, _, false, _) => Err("Negative Knowledge ka process bosh".to_string()),
        (_, _, _, false) => Err("Negative Knowledge ka evidence blob bosh".to_string()),
    }
}

fn validate_cycle_identity(q: &QuantumInboundWire, l: &LightInboundWire) -> Result<(), String> {
    match (
        q.session_id.is_empty(),
        q.session_id == l.session_id,
        q.territory == l.territory,
        q.primitive_flags == l.primitive_flags,
    ) {
        (false, true, true, true) => Ok(()),
        (true, _, _, _) => Err("session_id bosh".to_string()),
        (_, false, _, _) => Err("origjinat Quantum/Light kanë session të ndryshëm".to_string()),
        (_, _, false, _) => Err("origjinat Quantum/Light kanë territory të ndryshëm".to_string()),
        (_, _, _, false) => Err("origjinat Quantum/Light kanë primitive_flags të ndryshëm".to_string()),
    }
}

fn validate_final_evidence(
    q: &QuantumInboundWire,
    negative: &NegativeKnowledgeWire,
) -> Result<(), String> {
    let finite_scalars = [
        q.epistemic_mass,
        q.evidence_density,
        q.logical_coherence,
        q.causal_integrity,
        q.convergence_strength,
        q.reproducibility,
        q.potentiality_score,
        q.ibe_score,
        q.chain_coherence,
        negative.mass,
    ].iter().all(|value| value.is_finite())
        && q.candidate_scores.iter().all(|value| value.is_finite());
    match finite_scalars {
        true => {}
        false => return Err("NaN/Infinity u refuzua para Shadow core".to_string()),
    }

    let evidence = &q.final_evidence;
    match evidence.verifies_internal() {
        true => {}
        false => return Err("paketa finale PIM/NPIM/MPRO nuk rillogaritet".to_string()),
    }
    let input_sha256 = format!("{:x}", Sha256::digest(&evidence.light_input_bytes));
    match input_sha256 == evidence.light_input_sha256.to_ascii_lowercase() {
        true => {}
        false => return Err("SHA-256 i inputit Light nuk rillogaritet në Shadow".to_string()),
    }
    match evidence.scientific_project.as_ref() {
        Some(project) => validate_scientific_project_material(project, evidence)?,
        None => {}
    }
    match shadow_contracts::fnv1a64(&negative.bytes)
        == evidence.npim_arguments_blob_digest {
        true => {}
        false => return Err("blob-i NPIM/Negative Knowledge nuk përputhet me paketën finale".to_string()),
    }

    let pd = &evidence.pd_spine;
    let continuum_identity = pd.created_at == pd.continuum.stimulus_created_at
        && pd.session_digest == pd.continuum.session_digest
        && pd.parent_i0_digest == pd.continuum.i0_digest
        && pd.activation.response_kind_digest == pd.continuum.response_kind_digest
        && pd.continuum.user_input_digest
            == shadow_contracts::digest_parts(&[evidence.light_input_bytes.as_slice()])
        && pd.continuum.initial_i_digest == shadow_contracts::digest_parts(&[
            q.session_id.as_bytes(), b"GCL_INITIAL_I",
        ]);
    let spine_material_identity = match q.hcp_token_gen {
        Some(token_gen) => {
            let mut layer1_material = Vec::new();
            layer1_material.extend_from_slice(&token_gen.to_le_bytes());
            layer1_material.push(pd.layer1.completed as u8);
            let mut layer3_material = Vec::new();
            layer3_material.extend_from_slice(&pd.mpro_mass.to_le_bytes());
            layer3_material.push(pd.layer3.completed as u8);
            let expected_completion_evidence = shadow_contracts::digest_parts(&[
                &pd.layer2.result_material_digest.to_le_bytes(),
                &token_gen.to_le_bytes(),
                &pd.mpro_mass.to_le_bytes(),
                &pd.activation_contract_digest.to_le_bytes(),
            ]);
            pd.layer1.result_material == layer1_material
                && pd.layer3.result_material == layer3_material
                && pd.evidence_digest == expected_completion_evidence
        }
        None => false,
    };
    match (
        pd.gcl_law_seal
            == crate::lab_contracts::gjata_collapse_law::law_seal(),
        pd.gcl_system_laws_seal
            == crate::lab_contracts::gjata_collapse_law::system_laws_seal(),
        pd.session_digest == shadow_contracts::digest_parts(&[q.session_id.as_bytes()]),
        match q.pd_parent_i0.as_deref() {
            Some(parent) => pd.parent_i0_digest
                == shadow_contracts::digest_parts(&[parent.as_bytes()]),
            None => false,
        },
        q.pd_continuum_activation_digest == Some(pd.continuum_activation_digest),
        continuum_identity,
        spine_material_identity,
    ) {
        (true, true, true, true, true, true, true) => {}
        (false, _, _, _, _, _, _) | (_, false, _, _, _, _, _) => {
            return Err("paketa finale nuk i përket ligjit GCL të Shadow".to_string())
        }
        (_, _, false, _, _, _, _) => {
            return Err("session-i Quantum nuk përputhet me procesin PD/Spine9".to_string())
        }
        (_, _, _, false, _, _, _) => {
            return Err("parent i₀ nuk përputhet me procesin PD/Spine9".to_string())
        }
        (_, _, _, _, false, _, _) => {
            return Err("continuum activation nuk përputhet me procesin PD/Spine9".to_string())
        }
        (_, _, _, _, _, false, _) => {
            return Err("i + U → i₀ → 1Q nuk përputhet me identitetin Light/Quantum".to_string())
        }
        (_, _, _, _, _, _, false) => {
            return Err("materiali i Layer 1/2/3 ose completion evidence nuk rillogaritet".to_string())
        }
    }

    let fixed = |value: f32| -> Result<u32, String> {
        shadow_contracts::fixed_mass(value)
            .ok_or_else(|| "masë jo-finite në paketën finale".to_string())
    };
    let stage_words = |stage: u8| -> Option<&[u64]> {
        evidence.quantum_action_evidence.iter()
            .find(|entry| entry.stage == stage)
            .map(|entry| entry.evidence_words.as_slice())
    };
    let pro_first = match q.candidate_scores.first() {
        Some(value) => u64::from(fixed(*value)?),
        None => 0,
    };
    let pro_last = match q.candidate_scores.last() {
        Some(value) => u64::from(fixed(*value)?),
        None => 0,
    };
    let hpro_bound = match stage_words(shadow_contracts::ACTION_STAGE_HPRO) {
        Some(words) => words[0] != 0
            && u64::from((words[1] >= 1) as u8)
                == u64::from(evidence.mpro_measurements[8])
            && words[2] == u64::from(evidence.mpro_measurements[9])
            && u64::from((words[4] > 0) as u8)
                == u64::from(evidence.mpro_measurements[10])
            && words[3] == u64::from(evidence.mpro_measurements[11]),
        None => false,
    };
    let pro_expected = [q.candidate_scores.len() as u64, pro_first, pro_last];
    let pro_bound = match stage_words(shadow_contracts::ACTION_STAGE_PRO) {
        Some(words) => words == pro_expected.as_slice(),
        None => false,
    };
    let npro_bound = match stage_words(shadow_contracts::ACTION_STAGE_NPRO) {
        Some(words) => words[0] == evidence.npim_arguments.len() as u64
            && words[1] <= u64::from(shadow_contracts::MASS_SCALE)
            && words[3] <= u64::from(shadow_contracts::MASS_SCALE),
        None => false,
    };
    let srk_bound = match stage_words(shadow_contracts::ACTION_STAGE_SRK) {
        Some(words) => words[0] <= q.candidate_scores.len() as u64
            && words[1] == evidence.pim_proof_chain.len() as u64
            && words[3] <= words[2]
            && words[4] <= u64::from(shadow_contracts::MASS_SCALE)
            && words[5] <= u64::from(shadow_contracts::MASS_SCALE),
        None => false,
    };
    let apro_expected = evidence.mpro_measurements[12..16]
        .iter().map(|value| u64::from(*value)).collect::<Vec<_>>();
    let apro_bound = match stage_words(shadow_contracts::ACTION_STAGE_APRO) {
        Some(words) => words == apro_expected.as_slice(),
        None => false,
    };
    let mut expected_mpro = evidence.mpro_measurements.iter()
        .map(|value| u64::from(*value)).collect::<Vec<_>>();
    expected_mpro.extend_from_slice(&[
        u64::from(evidence.mpro_positives),
        u64::from(evidence.mpro_total),
        u64::from(evidence.mpro_vector_mass),
        u64::from(evidence.mpro_evidence_mass),
        u64::from(evidence.mpro_factic_mass),
    ]);
    let mpro_bound = match stage_words(shadow_contracts::ACTION_STAGE_MPRO) {
        Some(words) => words == expected_mpro.as_slice(),
        None => false,
    };
    let pim_expected = [
        u64::from(evidence.pim_evidence_density),
        u64::from(evidence.pim_logical_coherence),
        u64::from(evidence.pim_causal_integrity),
        u64::from(evidence.pim_convergence_strength),
        u64::from(evidence.pim_reproducibility),
        evidence.pim_proof_chain.len() as u64,
    ];
    let pim_action_bound = match stage_words(shadow_contracts::ACTION_STAGE_PIM) {
        Some(words) => words == pim_expected.as_slice(),
        None => false,
    };
    let npim_expected = [
        u64::from(evidence.npim_failure_density),
        u64::from(evidence.npim_contradiction_strength),
        u64::from(evidence.npim_elimination_certainty),
        u64::from(evidence.npim_negativity_score),
        evidence.npim_arguments.len() as u64,
    ];
    let npim_action_bound = match stage_words(shadow_contracts::ACTION_STAGE_NPIM) {
        Some(words) => words == npim_expected.as_slice(),
        None => false,
    };
    let hcp_bound = match (
        stage_words(shadow_contracts::ACTION_STAGE_HCP),
        q.hcp_input_id,
        q.hcp_token_gen,
        q.hcp_token_nonce,
        q.hcp_directive_bit,
    ) {
        (Some(words), Some(id), Some(generation), Some(nonce), Some(bit)) => {
            let expected = [id, generation, nonce, u64::from(bit), 1];
            words == expected.as_slice()
        }
        _ => false,
    };
    match (
        hpro_bound,
        pro_bound,
        npro_bound,
        srk_bound,
        apro_bound,
        mpro_bound,
        pim_action_bound,
        npim_action_bound,
        hcp_bound,
    ) {
        (true, true, true, true, true, true, true, true, true) => {}
        _ => return Err(
            "ledger-i Untrust nuk lidhet fushë-për-fushë me HPRO/PRO/NPRO/SRK/APRO/MPRO/PIM/NPIM/HCP"
                .to_string(),
        ),
    }

    let pim_matches = fixed(q.evidence_density)? == evidence.pim_evidence_density
        && fixed(q.logical_coherence)? == evidence.pim_logical_coherence
        && fixed(q.causal_integrity)? == evidence.pim_causal_integrity
        && fixed(q.convergence_strength)? == evidence.pim_convergence_strength
        && fixed(q.reproducibility)? == evidence.pim_reproducibility
        && fixed(q.potentiality_score)? == evidence.pim_potentiality_score
        && q.suggested_verdict == evidence.pim_suggestion;
    let npim_matches = fixed(negative.mass)? == evidence.npim_negativity_score
        && negative.frequency as usize == evidence.npim_arguments.len()
        && negative.suggestion_code == evidence.npim_suggestion;
    let mpro_matches = fixed(q.epistemic_mass)? == evidence.mpro_evidence_mass
        && match (&q.xy_mass, &q.collapse_refused) {
            (Some(mass), None) => *mass == evidence.mpro_factic_mass,
            (None, Some(_)) => true,
            _ => false,
        };
    match (pim_matches, npim_matches, mpro_matches) {
        (true, true, true) => Ok(()),
        (false, _, _) => Err("PIM wire nuk përputhet me paketën finale".to_string()),
        (_, false, _) => Err("NPIM wire nuk përputhet me argumentet negative".to_string()),
        (_, _, false) => Err("MPRO/XY nuk përputhet me 16 matjet e paketuara".to_string()),
    }
}


fn validate_scientific_project_material(
    project: &shadow_contracts::ScientificProjectWire,
    evidence: &shadow_contracts::FinalEvidenceWire,
) -> Result<(), String> {
    let context = &project.context;
    let expected_context = crate::gcl_project_contract::context_sha256(
        &crate::gcl_project_contract::ProjectContextMaterial {
            project_id: context.project_id,
            user_id: context.user_id,
            trace_id: context.trace_id,
            revision: context.revision,
            project_title: &project.project_title,
            content_sha256: &context.content_sha256,
            light_sovereign_flags: context.light_sovereign_flags,
        },
    );
    let files = project.files.iter().map(|f| (f.kind, f.bytes.clone()))
        .collect::<Vec<_>>();
    let expected_evidence = crate::gcl_project_contract::evidence_sha256(
        &crate::gcl_project_contract::ProjectEvidenceMaterial {
            context_sha256: &context.context_sha256,
            project_title: &project.project_title,
            domain: &project.domain,
            hypothesis: &project.hypothesis,
            assumptions: &project.assumptions,
            gcl_process_digest: project.gcl_process_digest,
            trl_level: project.trl_level,
            trl_passed: project.trl_passed,
            trl_confidence: project.trl_confidence,
            trl_reproducibility: project.trl_reproducibility,
            lab_test_id: &project.lab_test_id,
            findings: &project.findings,
            description: &project.description,
            files: &files,
        },
    );
    match (
        project.shape_is_valid(),
        context.trace_id == crate::gcl_project_contract::project_trace_id(
            context.project_id, context.user_id, &project.project_title,
        ),
        context.content_sha256 == evidence.light_input_sha256.to_ascii_lowercase(),
        crate::gcl_project_contract::seal_is_500(context.light_sovereign_flags),
        expected_context == context.context_sha256.to_ascii_lowercase(),
        project.gcl_process_digest == evidence.pd_spine.gcl_process_digest,
        expected_evidence == project.evidence_sha256.to_ascii_lowercase(),
    ) {
        (true, true, true, true, true, true, true) => Ok(()),
        (false, _, _, _, _, _, _) => Err("projekti shkencor ka formë wire të pavlefshme".to_string()),
        (_, false, _, _, _, _, _) => Err("titulli i projektit nuk prodhon APUPK trace_id e Light".to_string()),
        (_, _, false, _, _, _, _) => Err("projekti nuk lidhet me SHA-256 e inputit Light".to_string()),
        (_, _, _, false, _, _, _) => Err("projekti nuk mban Vulën Sovrane 500 të Light".to_string()),
        (_, _, _, _, false, _, _) => Err("ProjectContextWitness nuk rillogaritet".to_string()),
        (_, _, _, _, _, false, _) => Err("projekti nuk procedoi nën të njëjtin GCL process të Spine 9".to_string()),
        (_, _, _, _, _, _, false) => Err("evidenca shkencore e projektit nuk rillogaritet".to_string()),
    }
}

fn validate_scientific_project_against_apupk(q: &QuantumInboundWire) -> Result<(), String> {
    let project = match q.final_evidence.scientific_project.as_ref() {
        Some(project) => project,
        None => return Ok(()),
    };
    let entry = crate::shadow_apupk::apupk_memory().get(project.context.project_id)
        .ok_or_else(|| "projekti shkencor nuk ekziston në Shadow APUPK".to_string())?;
    let stored_sha256 = format!("{:x}", Sha256::digest(entry.content.as_bytes()));
    match (
        entry.project_id == project.context.project_id,
        entry.user_id == project.context.user_id,
        entry.trace_id == project.context.trace_id,
        entry.revisions == project.context.revision,
        entry.project_title == project.project_title,
        entry.light_sovereign_flags == project.context.light_sovereign_flags,
        stored_sha256 == project.context.content_sha256.to_ascii_lowercase(),
    ) {
        (true, true, true, true, true, true, true) => Ok(()),
        (false, _, _, _, _, _, _) => Err("APUPK project_id mismatch".to_string()),
        (_, false, _, _, _, _, _) => Err("APUPK user_id mismatch".to_string()),
        (_, _, false, _, _, _, _) => Err("APUPK trace_id mismatch".to_string()),
        (_, _, _, false, _, _, _) => Err("APUPK revision mismatch/replay".to_string()),
        (_, _, _, _, false, _, _) => Err("APUPK project_title mismatch".to_string()),
        (_, _, _, _, _, false, _) => Err("APUPK Vula 500 origin mismatch".to_string()),
        (_, _, _, _, _, _, false) => Err("APUPK content SHA-256 mismatch".to_string()),
    }
}

fn into_quantum(q: QuantumInboundWire) -> crate::bridge::QuantumInbound {
    crate::bridge::QuantumInbound {
        package_id: q.package_id,
        session_id: q.session_id,
        territory: q.territory,
        epistemic_mass: q.epistemic_mass,
        evidence_density: q.evidence_density,
        logical_coherence: q.logical_coherence,
        causal_integrity: q.causal_integrity,
        convergence_strength: q.convergence_strength,
        reproducibility: q.reproducibility,
        potentiality_score: q.potentiality_score,
        ibe_score: q.ibe_score,
        chain_coherence: q.chain_coherence,
        conservation_ok: q.conservation_ok,
        candidate_scores: q.candidate_scores,
        suggested_verdict: q.suggested_verdict,
        primitive_flags: q.primitive_flags,
        raw_bytes: q.raw_bytes,
        xy_x: q.xy_x,
        xy_y: q.xy_y,
        xy_mass: q.xy_mass,
        pd_binding_digest: q.pd_binding_digest,
        pd_continuum_activation_digest: q.pd_continuum_activation_digest,
        pd_parent_i0: q.pd_parent_i0,
        collapse_refused: q.collapse_refused,
        negative_paths: q.negative_paths,
        hcp_input_id: q.hcp_input_id,
        hcp_token_gen: q.hcp_token_gen,
        hcp_token_nonce: q.hcp_token_nonce,
        hcp_directive_bit: q.hcp_directive_bit,
        final_evidence_digest: q.final_evidence.package_digest,
        quantum_action_state: q.final_evidence.quantum_action_state,
        quantum_action_mask: q.final_evidence.quantum_action_mask,
        quantum_required_action_mask: q.final_evidence.quantum_required_action_mask,
        pd_gcl_process_digest: q.final_evidence.pd_spine.gcl_process_digest,
        spine_completion_digest: q.final_evidence.pd_spine.completion_digest,
        scientific_project: q.final_evidence.scientific_project.map(into_scientific_project),
    }
}


fn into_scientific_project(
    p: shadow_contracts::ScientificProjectWire,
) -> crate::types::ScientificProjectContext {
    crate::types::ScientificProjectContext {
        project_id: p.context.project_id,
        user_id: p.context.user_id,
        trace_id: p.context.trace_id,
        revision: p.context.revision,
        content_sha256: p.context.content_sha256,
        light_sovereign_flags: p.context.light_sovereign_flags,
        context_sha256: p.context.context_sha256,
        project_title: p.project_title,
        domain: p.domain,
        hypothesis: p.hypothesis,
        assumptions: p.assumptions,
        gcl_process_digest: p.gcl_process_digest,
        trl_level: p.trl_level,
        trl_passed: p.trl_passed,
        trl_confidence: p.trl_confidence,
        trl_reproducibility: p.trl_reproducibility,
        lab_test_id: p.lab_test_id,
        findings: p.findings,
        description: p.description,
        files: p.files.into_iter().map(|f| (f.kind, f.bytes)).collect(),
        evidence_sha256: p.evidence_sha256,
    }
}

fn into_light(l: LightInboundWire) -> crate::bridge::LightInbound {
    crate::bridge::LightInbound {
        session_id: l.session_id,
        territory: l.territory,
        primitive_flags: l.primitive_flags,
        epistemic_mass: l.epistemic_mass,
        proof_chain: l.proof_chain,
        payload: l.payload,
    }
}

fn persist_negative(shadow: &crate::Shadow, n: &NegativeKnowledgeWire) -> Result<(), String> {
    shadow
        .on_negative(
            n.input_id,
            n.reason,
            &n.process,
            n.mass,
            n.frequency,
            &n.bytes,
            n.ts_ns,
        )
        .map_err(|e| format!("on_negative dështoi: {e}"))
}

fn atomic_write(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let parent = path.parent().ok_or_else(|| "response path pa parent".to_string())?;
    std::fs::create_dir_all(parent).map_err(|e| format!("response parent s'u krijua: {e}"))?;
    let tmp = temporary_path(path);
    std::fs::write(&tmp, bytes).map_err(|e| format!("response temp s'u shkrua: {e}"))?;
    match std::fs::rename(&tmp, path) {
        Ok(()) => Ok(()),
        Err(e) => {
            let _ = std::fs::remove_file(&tmp);
            Err(format!("response nuk u publikua atomikisht: {e}"))
        }
    }
}

fn temporary_path(path: &Path) -> PathBuf {
    let mut tmp = path.as_os_str().to_os_string();
    tmp.push(format!(".{}.tmp", std::process::id()));
    PathBuf::from(tmp)
}
