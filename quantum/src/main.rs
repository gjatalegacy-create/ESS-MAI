// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  MAIN.RS — QUANTUM PLATFORM  — RRJEDHA E PLOTË E KOMUNIKIMIT      ║
// ║  Light → Quantum → [pipeline] → Shadow → Light                        ║
// ║  GJATA LEGACY™                                                        ║
// ╚══════════════════════════════════════════════════════════════════════════╝

mod shadow_process_bridge;

use quantum_lim::lab_contracts::{PrimitiveSplit, NegativePath, CollapseOutcome, CollapsePhase, UncertaintyLedger, pair_space, GjataCollapseLaw};
use quantum_lim::ultimatum_collapse_law::{UltimatumCollapseLaw, UclEvent};
use quantum_lim::{
    LinearInfoMetricEngine, LimHwRealBridge, ProEngine, ScientificReasoningKernel,
    PassInfoMetric, KnowledgeVault, EpistemicState,
    QuantumShadowBridge, LightQuantumBridge, SupremeOutcome,
    QuantumInput,
    RingBuffer, EbpfPacket, LgcBridge, SEAL_EBPF, SEAL_PD,
};

fn pd_layer_evidence(
    layer: &quantum_lim::pd_spine_contract::PdLayerReceipt,
    result_material: Vec<u8>,
) -> shadow_contracts::PdLayerEvidenceWire {
    shadow_contracts::PdLayerEvidenceWire {
        layer_mask: layer.layer_mask,
        activation_id: layer.activation_id,
        activation_contract_digest: layer.activation_contract_digest,
        gcl_process_digest: layer.gcl_process_digest,
        parent_result_digest: layer.parent_result_digest,
        result_material,
        result_material_digest: layer.result_material_digest,
        result_digest: layer.result_digest,
        completed: layer.completed,
    }
}

fn pd_continuum_state_code(
    state: quantum_lim::pd_continuum_contract::PdContinuumState,
) -> u8 {
    use quantum_lim::pd_continuum_contract::PdContinuumState;
    match state {
        PdContinuumState::I0Stimulated => 0,
        PdContinuumState::QuestionIncremented => 1,
        PdContinuumState::SpineActivated => 2,
        PdContinuumState::VerifiedOutput => 3,
        PdContinuumState::IzCompleted => 4,
        PdContinuumState::NextI0Derived => 5,
    }
}

fn pd_continuum_evidence(
    continuum: &quantum_lim::pd_continuum_contract::PdContinuumActivation,
) -> shadow_contracts::PdContinuumEvidenceWire {
    shadow_contracts::PdContinuumEvidenceWire {
        contract_version: continuum.contract_version,
        session_digest: continuum.stimulus.session_digest,
        initial_i_digest: continuum.stimulus.initial_i_digest,
        user_input_digest: continuum.stimulus.user_input_digest,
        i0_digest: continuum.stimulus.i0_digest,
        stimulus_digest: continuum.stimulus.stimulus_digest,
        stimulus_created_at: continuum.stimulus.created_at,
        stimulus_state: pd_continuum_state_code(continuum.stimulus.state),
        unit_count: continuum.question_increment.unit_count,
        question_digest: continuum.question_increment.question_digest,
        response_kind_digest: continuum.question_increment.response_kind_digest,
        increment_digest: continuum.question_increment.increment_digest,
        increment_created_at: continuum.question_increment.created_at,
        increment_state: pd_continuum_state_code(continuum.question_increment.state),
        activation_digest: continuum.activation_digest,
        activation_state: pd_continuum_state_code(continuum.state),
    }
}

fn pd_activation_evidence(
    activation: &quantum_lim::pd_spine_contract::PdActivationContract,
) -> shadow_contracts::PdActivationEvidenceWire {
    use quantum_lim::pd_spine_contract::{PdActivationOrigin, PdCognitiveMode};
    let mode_code = match activation.mode {
        PdCognitiveMode::Intellect => 0,
        PdCognitiveMode::Philosophy => 1,
        PdCognitiveMode::Scientific => 2,
        PdCognitiveMode::Novel => 3,
    };
    let origin_code = match activation.origin {
        PdActivationOrigin::NormalTurn => 0,
        PdActivationOrigin::GeniusAutonomousResearch => 1,
        PdActivationOrigin::BudgetExhausted => 2,
        PdActivationOrigin::SessionClosed => 3,
    };
    shadow_contracts::PdActivationEvidenceWire {
        mode_code,
        origin_code,
        response_kind_digest: activation.response_kind_digest,
        genius_signal_digest: activation.genius_signal_digest,
        trace_mass: activation.trace_mass.raw(),
        structural_coherence: activation.structural_coherence.raw(),
        required_layer_mask: activation.required_layer_mask,
        contract_digest: activation.contract_digest,
    }
}

fn pd_spine_evidence(
    completion: &quantum_lim::pd_spine_contract::PdSpineCompletion,
    layer1_material: Vec<u8>,
    layer2_material: Vec<u8>,
    layer3_material: Vec<u8>,
) -> shadow_contracts::PdSpineEvidenceWire {
    let request = &completion.request;
    shadow_contracts::PdSpineEvidenceWire {
        contract_version: request.contract_version,
        activation_id: request.activation_id,
        session_digest: request.session_digest,
        parent_i0_digest: request.parent_i0_digest,
        trace_digest: request.trace_digest,
        turn_digest: request.turn_digest,
        created_at: request.created_at,
        continuum: pd_continuum_evidence(&request.continuum),
        activation: pd_activation_evidence(&request.activation),
        continuum_activation_digest: request.continuum.activation_digest,
        activation_contract_digest: request.activation.contract_digest,
        gcl_law_seal: request.gcl.law_seal,
        gcl_system_laws_seal: request.gcl.system_laws_seal,
        gcl_phase_digest: request.gcl.phase_digest,
        gcl_process_digest: request.gcl.process_digest,
        layer_mask: completion.layer_mask,
        layer1: pd_layer_evidence(&completion.layer1, layer1_material),
        layer2: pd_layer_evidence(&completion.layer2, layer2_material),
        layer3: pd_layer_evidence(&completion.layer3, layer3_material),
        evidence_digest: completion.evidence_digest,
        mpro_mass: completion.mpro_mass.raw(),
        ready_for_shadow: completion.ready_for_shadow,
        completed_at: completion.completed_at,
        completion_digest: completion.completion_digest(),
    }
}

fn run(name: &str, light_payload: &str) -> bool {
    println!("\n{}", "═".repeat(76));
    println!("  {}", name);
    println!("{}", "═".repeat(76));
    quantum_lim::runtime_pulse::begin_cycle();

    // ════════ LIGHT → QUANTUM (deserialize) ════════
    let input = match QuantumInput::deserialize(light_payload) {
        Ok(i) => i,
        Err(e) => { println!("  [LIGHT→Q] gabim deserialize: {:?}", e); return false; }
    };
    println!("  [LIGHT→Q]  trace={} domain={} text=\"{}\"",
        input.trace_id, input.domain, input.text);
    let quantum_input_sha256 = quantum_lim::sovereign::sha256_hex(input.text.as_bytes());
    match quantum_input_sha256 == input.input_sha256.to_ascii_lowercase() {
        true => println!("             input_sha256={} (Light→Quantum ✓)", input.input_sha256),
        false => {
            eprintln!("  [LIGHT→Q] FAIL-CLOSED: SHA-256 i inputit nuk përputhet me Light");
            return false;
        }
    }
    println!("             carries_seal={}", input.carries_seal());
    // Qasje E TIPIZUAR te evolve (RREGULLIM     : lidh evolve_typed me konsumator).
    // KUFI I JASHTËM (fail-closed): wire 'evolve' i prishur = input i REFUZUAR
    // me emër fushe — kurrë normalizim i heshtur (audit: EVOLVING/0 e butë u hoq).
    let evolve = match input.evolve_typed() {
        Ok(e)  => e,
        Err(e) => {
            eprintln!("  [BRIDGE_LIGHT] INPUT I REFUZUAR (wire i gabuar): {:?}", e);
            return false;
        }
    };
    println!("             evolve[typed]: gen={} state={} primitive={}",
        evolve.generation, evolve.state_tag, evolve.is_primitive());

    // ════════ SOVEREIGN 0-COPY (ring → Ev → LIM) ════════
    // Transporti zero-copy (LIGJI 0) i sinjalit HYRËS përmes ringut. Akset NUK
    // janë konstante demo dhe NUK simulojnë eBPF: derivohen DETERMINISTIKISHT
    // nga inputi real (FNV-1a i tekstit hyrës → 8 byte → [0,1]). Kjo është
    // sinjal paraprak FAKTIK i transportuar (jo LIM-i final, që llogaritet më
    // poshtë nga fragmentet reale). Same input → same axes (deterministik).
    let ring = RingBuffer::new();
    let h = quantum_lim::lab_contracts::gjata_collapse_law::law_fnv1a64(input.text.as_bytes());
    let axes = [
        ((h        & 0xFF) as f32) / 255.0,
        (((h >> 8)  & 0xFF) as f32) / 255.0,
        (((h >> 16) & 0xFF) as f32) / 255.0,
        (((h >> 24) & 0xFF) as f32) / 255.0,
        (((h >> 32) & 0xFF) as f32) / 255.0,
        (((h >> 40) & 0xFF) as f32) / 255.0,
        (((h >> 48) & 0xFF) as f32) / 255.0,
        (((h >> 56) & 0xFF) as f32) / 255.0,
    ];
    // Fail-loud: ring i sapokrijuar s'mbushet, por dështimi s'fshihet në heshtje.
    match ring.push(EbpfPacket::from_axes(axes)) {
        Ok(())  => {}
        Err(_)  => eprintln!("  [SOVEREIGN] ALARM: ring 0-copy plot — paketa hyrëse s'u fut (transport i degraduar)"),
    }
    let bridge = LgcBridge::new(SEAL_EBPF);
    let ev_mass = match bridge.receive(&ring, SEAL_EBPF) {
        Ok(ev) => ev.mass(),
        Err(_) => 0.0,
    };
    println!("  [SOVEREIGN] ring 0-copy → Ev.mass={:.3} (sinjal hyrës i derivuar, transport LIGJI 0)", ev_mass);

    // ════════ QUANTUM PIPELINE ════════
    let territory = &input.domain;
    let mut lim = LinearInfoMetricEngine::new(&input.trace_id, territory);
    // ══ KOLAPS/1→2: merr (Xi, Yi) nga PA-skedari i Light (një burim i vetëm).
    // Formula: PA → (Xi, Yi) lindi në Light; Quantum e merr për eliminim.
    // Transport fail-open: mungesa → split pa material → refuzim i ndershëm.
    let (source_i0, psplit) = read_primitive_context(&input.trace_id);
    println!("  [KOLAPS/2] ARSYETIMI nis: i₀={} Xi={} Yi={} (nga Light)",
        source_i0.as_deref().unwrap_or("MISSING"),
        psplit.xi.len(), psplit.yi.len());

    // ── MUSKULI ultimatum_collapse_law — komisionohet nga Direktiva Supreme.
    // Avancon me hapat REALË; breach mblidhet → FAIL-CLOSED para inbound.
    let mut ucl_breach: Option<&'static str> = None;
    let mut ucl = match UltimatumCollapseLaw::commission(
        GjataCollapseLaw::issue(CollapsePhase::Reasoning)) {
        Ok(m)  => m,
        Err(b) => {
            eprintln!("  [ULTIMATUM] FAIL-CLOSED (komisionim): {}", b.text);
            return false;
        }
    };
    let ucl_step = |m: &mut UltimatumCollapseLaw, ev: UclEvent,
                        breach: &mut Option<&'static str>| {
        match m.advance(ev) {
            Ok(_)  => {}
            Err(b) => { *breach = (*breach).or(Some(b.text)); }
        }
    };
    ucl_step(&mut ucl, UclEvent::LoadXiYi, &mut ucl_breach); // Xi‖Yi u ngarkuan

    lim.collect(&input.text);

    // ════════ LIM → PD → LIM (dy faza: pre-seal → verification completion) ════════
    // PD analizon i₀ aktual dhe përgatit vetëm një kandidat PENDING për i₀ e
    // radhës. Asgjë nuk del te Light/Nura para se Shadow të mbyllë XY~0/1 dhe
    // të prodhojë dëftesën e output-it të vulosur me token verifikimi.
    use quantum_lim::progressive_debatic::{
        ProgressiveDebatic, DebateMode, PdEngineOutput, PdError, PdPreparedCycle,
        PdSpineCycle, PdCognitivePackage, PdFinalization, PdVerificationCompletion,
        PdNextI0, PdContinuumClosure,
    };
    let mut pd = ProgressiveDebatic::new();
    // Mode-i fillestar merret vetëm nga territory i deklaruar; nuk hamendësohet
    // Novel nga teksti. Novel lind vetëm kur GeniusSignal provon origjinën te user.
    let pd_mode = match territory.trim().to_ascii_lowercase().as_str() {
        "philosophy" | "philosophical" => DebateMode::Philosophy,
        "science" | "scientific" | "research" => DebateMode::Scientific,
        _ => DebateMode::Intellect,
    };
    match pd.open_session_sealed(
        &input.trace_id, territory, pd_mode, input.ts, SEAL_PD
    ) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("  [PD]       seal/session refuzuar: {e}");
            return false;
        }
    }

    // FAZA A: PD krijon vetëm kërkesën për Spine 9. PendingNextI0/pre-seal
    // ndalohen deri sa Layer 1/2/3 të kthejnë completion-in e tyre.
    let pd_spine_cycle: Option<PdSpineCycle> = match source_i0.as_deref() {
        Some(parent_i0) => match pd.ingest_for_spine_sealed(
            &input.trace_id,
            parent_i0,
            &input.text,
            input.ts,
            SEAL_PD,
        ) {
            Ok(cycle) => Some(cycle),
            Err(PdError::SealInvalid(reason)) => {
                eprintln!("  [PD]       seal i pavlefshëm (fail-closed): {reason}");
                return false;
            }
            Err(e) => {
                println!("  [PD]       gabim kognitiv (fail-open, PRO vazhdon pa PD): {:?}", e);
                None
            }
        },
        None => {
            quantum_lim::runtime_pulse::mark_degraded(
                quantum_lim::runtime_pulse::Stage::PdContinuum,
            );
            eprintln!("  [PD]       pa Primitive Anchor real: i₀→iZ nuk aktivizohet");
            None
        }
    };

    let pd_output = pd_spine_cycle.as_ref().map(|cycle| cycle.output.clone());
    let mut pd_pending_package: Option<PdCognitivePackage> = None;

    let pd_feed: String = match pd_output {
        Some(PdEngineOutput::GeniusDetected { genius_signal, package }) => {
            println!("  [PD]       GeniusSignal score={} → koncept=\"{}\" (TRL{})",
                quantum_lim::pd_spine_contract::FixedMass::new(genius_signal.genius_score).decimal_4(),
                package.dominant_concept,
                package.estimated_trl,
            );
            // Paketa ruhet PENDING. Handoff-i te Light bëhet vetëm pasi output-i
            // aktual të jetë mbyllur nga Verification Collapse në Shadow.
            let feed = package.dominant_concept.clone();
            pd_pending_package = Some(package);
            feed
        }
        Some(PdEngineOutput::Continue { turn }) => {
            println!("  [PD]       {:?} | prepared=\"{}\" (PENDING Shadow XY)",
                turn.pd_response_type, turn.pd_response_hint);
            // Hint-i ushqen LIM-in e dytë si material semantik, por NUK del te
            // Nura. Kandidati i tipizuar ndodhet te pd_pending_next.
            turn.pd_response_hint
        }
        Some(PdEngineOutput::BudgetExhausted { trace_id }) => {
            println!("  [PD]       buxhet i shterur (trace={}) → PRO merr dorezën", trace_id);
            "deterministic elimination produces verifiable knowledge through filters".to_string()
        }
        Some(PdEngineOutput::SessionClosed { reason, .. }) => {
            println!("  [PD]       sesion i mbyllur: {}", reason);
            "deterministic elimination produces verifiable knowledge through filters".to_string()
        }
        None => "deterministic elimination produces verifiable knowledge through filters".to_string(),
    };
    lim.collect(&pd_feed);

    let scaffold = match lim.pre_pro_scan() {
        Ok(s) => s,
        Err(e) => { println!("  [LIM] {}", e); return false; }
    };
    let lim_metric = scaffold.lim.clone();
    ucl_step(&mut ucl, UclEvent::MeasureLim8, &mut ucl_breach); // 8 akset u matën
    println!("  [LIM]      mass={:.3}", scaffold.lim.epistemic_mass);

    // ══ HARDWARE RUNTIME v1.6.8 — NJË MATJE, NJË LEASE, NJË CIKËL ══════
    let l3_claim = quantum_lim::lab_contracts::gjata_collapse_law::law_fnv1a64(
        input.trace_id.as_bytes());
    let hcp_depth = quantum_lim::hw_real::territories::DepthHint::from_mass(
        lim_metric.epistemic_mass);
    let mut hardware_runtime = quantum_lim::hw_real::HardwareRuntimeContext::capture(
        l3_claim,
        hcp_depth,
    );

    match hardware_runtime.thermal_state {
        quantum_lim::hw_real::thermal::ThermalState::Normal => {
            quantum_lim::runtime_pulse::mark_ready(
                quantum_lim::runtime_pulse::Stage::Thermal);
        }
        _ => {
            quantum_lim::runtime_pulse::mark_notready(
                quantum_lim::runtime_pulse::Stage::Thermal);
        }
    }

    let mut hpro_runtime = quantum_lim::layer3::hpro::HproEngine::acquire(
        &hardware_runtime.device_snapshot,
        hardware_runtime.measurement_state,
    );
    hardware_runtime.install_initial_lease(hpro_runtime.lease);

    // HCP_PRO aplikohet PARA PRO-së: lease-i i ndryshuar kontrollon punën e
    // këtij cikli, jo vetëm një log ose një cikël të ardhshëm hipotetik.
    let hcp_env = hardware_runtime.pressure_envelope();
    let hcp_signals = [
        (quantum_lim::Operator::Reverse,  lim_metric.information_density),
        (quantum_lim::Operator::Mirror,   lim_metric.semantic_integrity),
        (quantum_lim::Operator::Contrast, lim_metric.contradiction_presence),
        (quantum_lim::Operator::Trace,    lim_metric.continuity_score),
    ];
    let thermal_hot = hardware_runtime.thermal_state.effective_hot(hcp_depth);
    let mut hcp_gate = quantum_lim::SovereignGate::new();
    let hcp_application = quantum_lim::hcp_pro::HcpPro::orchestrate_and_apply(
        &mut hcp_gate,
        &mut hardware_runtime,
        &hcp_env,
        &hcp_signals,
        thermal_hot,
    );
    hpro_runtime.apply_lease(hcp_application.lease_after);

    println!(
        "  [HCP_APPLY] requested={} applied={} sealed={} lease {}MB/{} → {}MB/{} mask=0x{:X}→0x{:X} flexed={}",
        hcp_application.decision.hw_directive.label(),
        hcp_application.applied_directive.label(),
        hcp_application.decision.sealed,
        hcp_application.lease_before.granted_bytes / (1024 * 1024),
        hcp_application.lease_before.granted_parallel,
        hcp_application.lease_after.granted_bytes / (1024 * 1024),
        hcp_application.lease_after.granted_parallel,
        hcp_application.receipt.operator_mask_before,
        hcp_application.receipt.operator_mask_after,
        hcp_application.receipt.flexed,
    );

    // Buffer-at e reasoning-ut lindin PAS context-it dhe kufizohen nga lease-i.
    // Kështu RAM budget nuk mbetet raport: ai kontrollon working set-in real që
    // klonohet/vektorizohet nga Quantum. Në çdo early return, buffer-at dalin
    // nga scope-i para HwRuntimeContext::drop().
    let collected_frags = lim.collector().fragments();
    let (fragment_limit, estimated_working_set) = match hardware_runtime.lease() {
        Some(lease) => lease.admit_fragment_prefix(collected_frags),
        None => (0, 0),
    };
    let frags: Vec<String> = collected_frags
        .iter()
        .take(fragment_limit)
        .cloned()
        .collect();
    hardware_runtime.register_buffers(frags.len(), estimated_working_set);
    println!(
        "  [HW_WORKSET] admitted_fragments={}/{} estimated_bytes={} lease_bytes={}",
        frags.len(),
        collected_frags.len(),
        estimated_working_set,
        hardware_runtime.lease().map(|lease| lease.granted_bytes).unwrap_or(0),
    );

    // ══ LAYER 3 (HPRO→HPIM) — konsumon të NJËJTIN lease të aplikuar ═══════
    let l3 = quantum_lim::layer3::QuantumLayer3Flow::from_hpro(
        l3_claim,
        &hpro_runtime,
    );
    println!(
        "  [LAYER3]   HPRO→HPIM: parallel_used={} deep_allowed={} lease_active={} ready_for_pim={}",
        l3.hpim_package.parallel_used,
        l3.hpim_package.deep_allowed,
        l3.hpim_package.lease_active,
        l3.ready_for_pim,
    );
    let hpro_action = [
        l3_claim,
        u64::from(l3.hpim_package.parallel_used),
        u64::from(l3.hpim_package.deep_allowed as u8),
        u64::from(l3.hpim_package.lease_active as u8),
        l3.hpim_package.trace_len() as u64,
    ];
    quantum_lim::runtime_pulse::mark_action(
        quantum_lim::runtime_pulse::Stage::Hpro,
        &hpro_action,
    );

    // ── SPINE 9 — përdor hardware-in e parapërgatitur; nuk ekzekuton HPRO të dytë.
    let spine9_trl = quantum_lim::lab_contracts::trl::TrlEvidence::pending(l3_claim);
    let spine9_trust = quantum_lim::lab_contracts::trust::TrustContext::empty(
        l3_claim,
        input.domain.clone(),
    );
    let phase9 = quantum_lim::phase9_integration::Phase9Integration::reason_with_runtime_hardware(
        l3_claim,
        &frags,
        l3.clone(),
        spine9_trl,
        spine9_trust,
        lim_metric.epistemic_mass,
        lim_metric.continuity_score,
    );
    // VERIFIKIM RUNTIME i ligjit: Quantum prodhon PROVË, jo verdikt.
    let q_evidence_only =
        quantum_lim::phase9_integration::Phase9Integration::produces_evidence_not_verdict(&phase9);
    match (phase9.law_check.is_passed(), q_evidence_only, &phase9.spine) {
        (true, true, Some(sp)) => {
            println!("  [SPINE9]   within_bounds={} evidence_gati={} (Quantum prodhon PROVË, s'vendos)",
                sp.within_bounds, phase9.ready_for_shadow);
        }
        (_, false, _) => {
            // Strukturalisht i pamundur (Quantum s'ka API verdikti); fail-closed
            // dëshmi — nëse ndonjëherë do prishej, e kapim, s'e maskojmë.
            println!("  [SPINE9]   SHKELJE: dalja e Quantum u perceptua si verdikt — kufiri u ruajt");
        }
        _ => { println!("  [SPINE9]   provë s'u ndërtua (ligjet s'kaluan: {})",
                   phase9.law_check.label()); }
    }


    // HCP_PRO Layer 2 konsumon probe-in e Spine 9 PA token/vendim të dytë.
    // Një claim që thyhet nën presion mbyll operator admission para PRO-së.
    let hcp_l2_adjustment = match &phase9.spine {
        Some(spine) => Some(
            quantum_lim::layer2::hcp_pro_l2::HcpProL2::adjust_applied_with_pressure(
                &hcp_application,
                &spine.probe,
            ),
        ),
        None => None,
    };
    let hcp_l2_allows_pro = match &hcp_l2_adjustment {
        Some(adjusted) => {
            quantum_lim::layer2::hcp_pro_l2::HcpProL2::directs_deeper(adjusted)
        }
        None => false,
    };
    match hcp_l2_allows_pro {
        true => {}
        false => hardware_runtime.set_operator_admission_mask(0),
    }

    // Vetëm tani ndërtohet scaffold-i që PRO konsumon: lease + mask pas L2.
    let hw = LimHwRealBridge::finalize_with_runtime(scaffold, &hardware_runtime);
    println!(
        "  [HW_REAL]  state={}/{} depth={} RAM={}/{}MB cpu={:.1}% cores={}/{} cooling={} lease={}MB parallel={} mask=0x{:X} final={:.3}",
        hardware_runtime.measurement_state.label(),
        hardware_runtime.measurement_source.label(),
        hw.depth_hint.name(),
        hw.ram_free_bytes / (1024 * 1024),
        hw.ram_total_bytes / (1024 * 1024),
        hw.cpu_load * 100.0,
        hw.cores_active,
        hw.cores_total,
        hardware_runtime.cooling_control_available,
        hw.allocated_bytes / (1024 * 1024),
        hw.n_parallel,
        hw.operator_admission_mask,
        hw.depth_final,
    );


    // P3.3: Lexo NK të akumuluar nga Shadow (sinkronizim, jo vault bosh).
    // v1.4.3 (audit p.7, PRANUAR): ÇELËSI NJËDREJTIMËSH edhe këtu — sapo
    // është parë NJËHERË blob i vulosur (markeri .sealed), legacy pa magjikë
    // = DEGRADIM → vault bosh me zë + rrjedha. I njëjti ligj si PA/PD.
    let nk_marker_sealed = std::fs::metadata(handoff_path("shadow_nk_export.sealed")).is_ok();
    let mut vault = match std::fs::read(handoff_path("shadow_nk_export.bin")) {
        Ok(bytes) => match quantum_lim::pro_nk_gate::nk_downgrade(
            nk_marker_sealed,
            quantum_lim::pro_nk_gate::is_sealed_nk_blob(&bytes)) {
            true => {
                let _ = quantum_lim::lab_contracts::rrjedha::note(
                    &rrjedha_dir(), "QUANTUM", "main::nk_import",
                    quantum_lim::lab_contracts::rrjedha::RrjedhaClass::TransportDowngrade,
                    "NK legacy PAS blob-i të vulosur (.sealed ekziston) — dija e prishur refuzohet");
                quantum_lim::runtime_pulse::mark_notready(quantum_lim::runtime_pulse::Stage::NkImport);
                eprintln!("  [NK_BRIDGE] ALARM DEGRADIM: legacy pas vule — vault bosh i deklaruar");
                KnowledgeVault::new()
            }
            false => KnowledgeVault::from_negative_export(&bytes),
        },
        Err(_)    => KnowledgeVault::new(), // herën e parë: bosh
    };
    let pro = ProEngine::activate(&hw, &frags, &mut vault);
    // Të gjithë worker-ët scoped janë bashkuar para kthimit nga activate().
    hardware_runtime.note_workers_joined(pro.workers_joined);
    let action_fixed = |value: f32| -> u64 {
        u64::from(shadow_contracts::fixed_mass(value).unwrap_or(0))
    };
    let pro_action = [
        pro.candidates.len() as u64,
        pro.candidates.first().map(|c| action_fixed(c.score)).unwrap_or(0),
        pro.candidates.last().map(|c| action_fixed(c.score)).unwrap_or(0),
        pro.workers_joined as u64,
        pro.fragments_admitted as u64,
        pro.working_set_bytes,
    ];
    quantum_lim::runtime_pulse::mark_action(
        quantum_lim::runtime_pulse::Stage::Pro,
        &pro_action,
    );
    ucl_step(&mut ucl, UclEvent::SearchPro, &mut ucl_breach); // i≈XY u kërkua (4 op)
    println!(
        "  [PRO]      {} kandidatë | operators_run={} workers_joined={} fragments={} working_set={}B",
        pro.candidates.len(),
        pro.operators_run,
        pro.workers_joined,
        pro.fragments_admitted,
        pro.working_set_bytes,
    );

    // ════════ NPRO — RRUGA PARALELE NEGATIVE ════════
    // NPRO kalkulon eliminimet me 4 operatorët MBRAPSHT.
    // P4: sinjalet reale nga LIM (jo 0.6 arbitrare).
    let npro = quantum_lim::NproEngine::activate(
        &frags,
        lim_metric.information_density,  // ← real
        lim_metric.continuity_score,     // ← real
    );

    // ════════ FORCIM NPRO — konsensus + ceiling adaptiv ════════
    let (npro, npro_hard) = quantum_lim::NproHardening::strengthen_npro(npro);
    println!("  [NPRO]     {} eliminime | forbidden_paths={:?} | avg_weakness={:.3}",
        npro.eliminations.len(),
        npro.weakness_signal.forbidden_paths,
        npro.weakness_signal.avg_weakness);
    println!("  [NPRO+]    konsensus={:.3} | ceiling {:.2}→{:.2} (adaptiv) | forbidden {}→{}",
        npro_hard.consensus_strength, npro_hard.original_ceiling,
        npro_hard.adaptive_ceiling, npro_hard.forbidden_before, npro_hard.forbidden_after);

    // NPRO → LIM: WeaknessSignal vendos kufij (forbidden paths çaktivizohen).
    let forbidden_n = quantum_lim::NproLimBridge::forbidden_applied(&npro.weakness_signal);
    println!("  [NPRO→LIM] {} forbidden paths → kufij për PRO (PRO shkon më thellë)", forbidden_n);

    // ══ FORMULA: +(-Xi) + (-Yi) — eliminimi i hapësirave primitive.
    // LIM ia përcaktoi sinjalet reale NPRO-s (density/continuity ↑); NPRO
    // nxori forbidden_paths; tani termat e (Xi,Yi) që bien në to shënohen
    // NEGATIVE — rrugë të eliminuara, të ruajtura si ASET (kurrë të fshira).
    let npro_action = [
        npro.eliminations.len() as u64,
        action_fixed(npro.weakness_signal.avg_weakness),
        npro.weakness_signal.forbidden_paths.len() as u64,
        action_fixed(npro_hard.consensus_strength),
    ];
    quantum_lim::runtime_pulse::mark_action(
        quantum_lim::runtime_pulse::Stage::Npro,
        &npro_action,
    );
    let neg_paths = mark_negative_spaces(&psplit, &npro.weakness_signal);
    println!("  [KOLAPS/2] eliminime hapësirash: {} rrugë negative (-Xi/-Yi)",
        neg_paths.len());
    ucl_step(&mut ucl, UclEvent::MarkNegatives, &mut ucl_breach); // (-Xi)+(-Yi) u shënuan

    // NPRO → NPIM: paketon negative knowledge me argumenta.
    let mut npim = quantum_lim::NegativePassMetric::measure(&npro, &input.trace_id);

    // ════════ FORCIM NPIM — evidence weighting + severity ════════
    // Hardening-u bëhet pjesë e paketës autoritative; nuk mbetet vetëm log.
    let original_negativity = npim.negativity;
    npim.negativity = quantum_lim::NpimHardening::strengthen_negativity(
        npim.negativity, &npro.eliminations);
    println!("  [NPIM]     {}", npim.report());
    println!("  [NPIM+]    negativity {:.3}→{:.3} (severity escalation) | failure_density={:.3}",
        original_negativity.negativity_score, npim.negativity.negativity_score,
        npim.negativity.failure_density);

    // NPIM → Shadow: NegativeInbound (byte-for-byte me Shadow NegativeContext).
    let neg_inbound = quantum_lim::NpimShadowBridge::build(&npim, territory);
    let npim_action = [
        action_fixed(npim.negativity.failure_density),
        action_fixed(npim.negativity.contradiction_strength),
        action_fixed(npim.negativity.elimination_certainty),
        action_fixed(npim.negativity.negativity_score),
        npim.arguments.len() as u64,
    ];
    quantum_lim::runtime_pulse::mark_action(
        quantum_lim::runtime_pulse::Stage::Npim,
        &npim_action,
    );
    println!("  [NPIM→SHADOW] NegativeContext: reason={} process={} mass={:.3} freq={}",
        neg_inbound.failure_reason_code, neg_inbound.process,
        neg_inbound.mass_at_fail, neg_inbound.frequency);
    println!("             (gati për Shadow::on_negative → VERIFIED_NEGATIVE)");

    let srk = ScientificReasoningKernel::new(&input.trace_id, territory);
    let reasoning = srk.process(&pro);
    let srk_action = [
        reasoning.explanations.len() as u64,
        reasoning.evidence_chain.len() as u64,
        reasoning.conservation_checks.len() as u64,
        reasoning.conservation_eliminated as u64,
        action_fixed(reasoning.uncertainty.combined),
        reasoning.best_explanation.as_ref()
            .map(|e| action_fixed(e.ibe_score))
            .unwrap_or(0),
    ];
    quantum_lim::runtime_pulse::mark_action(
        quantum_lim::runtime_pulse::Stage::Srk,
        &srk_action,
    );

    let state = EpistemicState::PotentialHypothesis {
        confidence_range: (0.36, 0.98), stability: reasoning.uncertainty.combined,
        trl_candidate: 2, pro_mode: "deep".to_string(),
    };
    let pass = PassInfoMetric::measure(&reasoning, &state, &input.trace_id);
    let pim_action = [
        action_fixed(pass.potentiality.evidence_density),
        action_fixed(pass.potentiality.logical_coherence),
        action_fixed(pass.potentiality.causal_integrity),
        action_fixed(pass.potentiality.convergence_strength),
        action_fixed(pass.potentiality.reproducibility),
        pass.proof_chain.len() as u64,
    ];
    quantum_lim::runtime_pulse::mark_action(
        quantum_lim::runtime_pulse::Stage::Pim,
        &pim_action,
    );

    // ══ FORMULA: XiYi → XY — kolapsi i arsyetimit në propozim.
    // PRO-fryma: i~ (indicia e përbashkët) = bashkëprezenca më e fortë e një
    // çifti (x∈Xi, y∈Yi) në fragmentet reale; MPRO-fryma: masa e matur nga
    // bashkëprezenca × epistemic_mass. Hapësirë e zbrazët → REFUZIM i
    // ndershëm (sistemi kurrë s'shpik përgjigje).
    let (collapse, xy_survivors) = collapse_reasoning(&frags, &psplit, lim_metric.epistemic_mass);
    match &collapse {
        CollapseOutcome::Proposed { x, y, mass } =>
            println!("  [KOLAPS/2] XY PROPOZUAR: ({} ↔ {}) mass_bashkëprezencë={:.3} | mbijetues={}",
                x, y, mass, xy_survivors),
        CollapseOutcome::Refused { reason } =>
            println!("  [KOLAPS/2] REFUZUAR (ndershëm): {}", reason),
    }

    // ══ APRO — argumenton kandidatët e PRO dhe eliminimet e NPRO (ligji: i≈XY → APRO).
    let apro_arg = quantum_lim::apro::Apro::argue(&pro, &npro);
    let apro_action = [
        u64::from(apro_arg.measures[0]),
        u64::from(apro_arg.measures[1]),
        u64::from(apro_arg.measures[2]),
        u64::from(apro_arg.measures[3]),
    ];
    quantum_lim::runtime_pulse::mark_action(
        quantum_lim::runtime_pulse::Stage::Apro,
        &apro_action,
    );
    println!("  [APRO]     {}", apro_arg.report());
    ucl_step(&mut ucl, UclEvent::ArgueApro, &mut ucl_breach); // 4 argumentet qëndruan

    // ══ MPRO — 16 MATJET (PRO 4 + NPRO 4 + HPRO 4 + APRO 4) → masë vektoriale.
    // Çdo matje 0/1 vjen nga rezultate REALE të fazave — zero hamendje.
    let pro4: [u8; 4] = [
        match pro.candidates.iter().any(|c| matches!(c.operator, quantum_lim::lim_types::Operator::Reverse))  { true => 1, false => 0 },
        match pro.candidates.iter().any(|c| matches!(c.operator, quantum_lim::lim_types::Operator::Mirror))   { true => 1, false => 0 },
        match pro.candidates.iter().any(|c| matches!(c.operator, quantum_lim::lim_types::Operator::Contrast)) { true => 1, false => 0 },
        match pro.candidates.iter().any(|c| matches!(c.operator, quantum_lim::lim_types::Operator::Trace))    { true => 1, false => 0 },
    ];
    let npro4: [u8; 4] = [
        match npro.eliminations.iter().any(|e| e.operator == "NReverse")  { true => 1, false => 0 },
        match npro.eliminations.iter().any(|e| e.operator == "NMirror")   { true => 1, false => 0 },
        match npro.eliminations.iter().any(|e| e.operator == "NContrast") { true => 1, false => 0 },
        match npro.eliminations.iter().any(|e| e.operator == "NTrace")    { true => 1, false => 0 },
    ];
    // HPRO4 — 4 matje REALE nga Layer 3 (HPIM), jo proxy të thjeshta:
    // paralelizmi fizik, thellimi, gjurma dhe lease-i i vlefshëm për workload.
    // Kjo lidh daljen e Shtresës 3 me vektorin 16-matjesh të MPRO-s.
    let hpro4: [u8; 4] = [
        match l3.hpim_package.parallel_used >= 1 { true => 1, false => 0 },
        match l3.hpim_package.deep_allowed       { true => 1, false => 0 },
        match l3.hpim_package.trace_len() > 0    { true => 1, false => 0 },
        match l3.ready_for_pim                   { true => 1, false => 0 },
    ];
    // HCP_PRO Layer 3 konsumon të NJËJTIN application receipt dhe vendos cilat
    // familje lejohen të hyjnë në MPRO. Kjo është admission reale, jo raport.
    let hcp_l3_signals = vec![
        quantum_lim::layer3::hcp_pro_l3::FamilySignals::new(
            quantum_lim::layer3::hcp_pro_l3::ProFamily::Pro,
            pro4.map(f32::from),
        ),
        quantum_lim::layer3::hcp_pro_l3::FamilySignals::new(
            quantum_lim::layer3::hcp_pro_l3::ProFamily::Npro,
            npro4.map(f32::from),
        ),
        quantum_lim::layer3::hcp_pro_l3::FamilySignals::new(
            quantum_lim::layer3::hcp_pro_l3::ProFamily::Hpro,
            hpro4.map(f32::from),
        ),
        quantum_lim::layer3::hcp_pro_l3::FamilySignals::new(
            quantum_lim::layer3::hcp_pro_l3::ProFamily::Apro,
            apro_arg.measures.map(f32::from),
        ),
    ];
    let hcp_l3_activation = quantum_lim::layer3::hcp_pro_l3::HcpProL3::activate_from_application(
        &hcp_application,
        &hcp_l3_signals,
        &hpro_runtime,
        hcp_l2_allows_pro,
        phase9.law_check.is_passed(),
    );
    let family_is_active = |family: quantum_lim::layer3::hcp_pro_l3::ProFamily| -> bool {
        hcp_l3_activation
            .families
            .iter()
            .any(|entry| {
                (entry.family == family)
                    & (entry.order == quantum_lim::hcp_pro::OperatorOrder::GoDeeper)
            })
    };
    let gate_family = |values: [u8; 4], family| -> [u8; 4] {
        match family_is_active(family) {
            true => values,
            false => [0; 4],
        }
    };
    let pro4_admitted = gate_family(
        pro4,
        quantum_lim::layer3::hcp_pro_l3::ProFamily::Pro,
    );
    let npro4_admitted = gate_family(
        npro4,
        quantum_lim::layer3::hcp_pro_l3::ProFamily::Npro,
    );
    let hpro4_admitted = gate_family(
        hpro4,
        quantum_lim::layer3::hcp_pro_l3::ProFamily::Hpro,
    );
    let apro4_admitted = gate_family(
        apro_arg.measures,
        quantum_lim::layer3::hcp_pro_l3::ProFamily::Apro,
    );
    println!(
        "  [HCP_L3]   active_families={}/{} sealed={} lease_parallel={}",
        hcp_l3_activation.active_count,
        hcp_l3_activation.families.len(),
        hcp_l3_activation.sealed,
        hcp_application.lease_after.granted_parallel,
    );

    let mset = quantum_lim::layer3::mpro::MeasurementSet::new(
        pro4_admitted,
        npro4_admitted,
        hpro4_admitted,
        apro4_admitted,
    );
    let sv = quantum_lim::layer3::mpro::Mpro::measure_sealed(
        &mset,
        quantum_lim::layer3::mpro::Mpro::mpro_seal(),
    );

    // ══ PROBABILITETI FAKTIK (next-level): deterministik → probabilitet → faktik.
    //   raporti vektorial (positives/16) × evidenca e matur (epistemic_mass LIM).
    let evidence_mass_fixed = quantum_lim::pd_spine_contract::FixedMass::new(
        (lim_metric.epistemic_mass.clamp(0.0, 1.0)
            * quantum_lim::pd_spine_contract::MASS_SCALE as f32) as u32,
    );
    let mpro_factic = quantum_lim::pd_spine_contract::FixedMass::new(
        sv.mass.saturating_mul(evidence_mass_fixed.raw())
            / quantum_lim::pd_spine_contract::MASS_SCALE,
    );
    let mpro_action = [
        u64::from(pro4[0]), u64::from(pro4[1]), u64::from(pro4[2]), u64::from(pro4[3]),
        u64::from(npro4[0]), u64::from(npro4[1]), u64::from(npro4[2]), u64::from(npro4[3]),
        u64::from(hpro4[0]), u64::from(hpro4[1]), u64::from(hpro4[2]), u64::from(hpro4[3]),
        u64::from(apro_arg.measures[0]), u64::from(apro_arg.measures[1]),
        u64::from(apro_arg.measures[2]), u64::from(apro_arg.measures[3]),
        u64::from(sv.positives), u64::from(sv.total), u64::from(sv.mass),
        u64::from(evidence_mass_fixed.raw()), u64::from(mpro_factic.raw()),
    ];
    quantum_lim::runtime_pulse::mark_action(
        quantum_lim::runtime_pulse::Stage::Mpro,
        &mpro_action,
    );
    println!("  [MPRO]     {}/{} matje → masë={} | × evidencë {} → FAKTIK={} | sealed={}",
        sv.positives, sv.total,
        quantum_lim::pd_spine_contract::FixedMass::new(sv.mass).decimal_4(),
        evidence_mass_fixed.decimal_4(), mpro_factic.decimal_4(), sv.sealed);
    ucl_step(&mut ucl, UclEvent::MeasureMpro16, &mut ucl_breach); // 16 → faktik
    ucl_step(&mut ucl, UclEvent::Package, &mut ucl_breach); // PIM(pass)+NPIM tashmë të matura

    // ══ LAW 0 — "Every state must reduce uncertainty" (dëshmia e gjallë).
    // Hapësira e mundësive me numra REALË: n² → |Xi|×|Yi| → pas eliminimit
    // → mbijetuesit e i≈XY → 1 (XY) ose 0 (refuzim). Shkelje = fail-closed.
    let mut law0 = UncertaintyLedger::new();
    let terms_n   = (psplit.xi.len() + psplit.yi.len()) as f32;
    let space0    = pair_space(psplit.xi.len(), psplit.yi.len());
    let neg_xi    = neg_paths.iter().filter(|n| n.space == "Xi").count();
    let neg_yi    = neg_paths.iter().filter(|n| n.space == "Yi").count();
    let space_neg = pair_space(psplit.xi.len().saturating_sub(neg_xi),
                               psplit.yi.len().saturating_sub(neg_yi));
    let space_xy  = match &collapse {
        CollapseOutcome::Proposed { .. } => 1.0f32,
        CollapseOutcome::Refused  { .. } => 0.0f32,
    };
    let law_chain = law0
        .record(CollapsePhase::Coordination, "pa_split",  terms_n * terms_n, space0)
        .and_then(|_| law0.record(CollapsePhase::Reasoning, "npro_elim", space0, space_neg))
        .and_then(|_| law0.record(CollapsePhase::Reasoning, "pro_i≈XY",  space_neg, (xy_survivors as f32).min(space_neg)))
        .and_then(|_| law0.record(CollapsePhase::Reasoning, "mpro_XY",   (xy_survivors as f32).min(space_neg), space_xy));
    match law_chain {
        Ok(())   => println!("  [LAW0]     {}", law0.chain_report()),
        Err(v)   => {
            eprintln!("  [LAW0] FAIL-CLOSED: {}", v.report());
            return false; // shkelja e ligjit kushtetues NDAL rrjedhën — kurrë heshtur
        }
    }
    println!("  [PIM]      potentiality={:.3} → {}",
        pass.potentiality.potentiality_score,
        pass.shadow_suggestion.suggested_verdict.label());

    // ════════ QUANTUM → SHADOW (ura, vula 500) ════════
    let sealed = input.carries_seal() && pass.potentiality.logical_coherence > 0.5;
    let inbound = QuantumShadowBridge::build(
        &lim_metric, &pro, &reasoning, &pass, &input.trace_id, territory, sealed,
    );

    // RUNTIME HARDENING (      ): fail-closed — paketa duhet valide para Shadow.
    // Asnjë silent success; paketë e gjymtuar → ndalon me reason code. Zero if — match.
    match inbound.is_valid() {
        true => {}
        false => {
            eprintln!("  [Q→SHADOW] REFUZUAR (fail-closed): {}", inbound.reason_invalid());
            return false;
        }
    }

    let (_pkg, light_in) = QuantumShadowBridge::split(&inbound);
    let seal_label = match inbound.is_sealed() {
        true  => "SEALED 500",
        false => "STRIPPED",
    };
    println!("  [Q→SHADOW] QuantumInbound: flags=0x{:04X} ({}) | LightInbound.seal_ok={}",
        inbound.primitive_flags, seal_label, light_in.seal_ok());
    println!("             (gati për Shadow::ingest_quantum / receive_from_light)");

    // ══ HCP_PRO — vendimi i aplikuar më herët tani kalon si provë te Shadow ══
    let token2 = quantum_lim::token_forge::mint(
        0x544F_4B45_4E32_0001, // "TOKEN2"
    );
    quantum_lim::runtime_pulse::mark_ready(
        quantum_lim::runtime_pulse::Stage::TokenForge,
    );
    println!(
        "  [TOKEN2]   burimi i dytë SHA-256: {} — derdhja #{}",
        quantum_lim::token_forge::token_hex(&token2),
        quantum_lim::token_forge::minted_count(),
    );

    let mut hcp_decision = hcp_application.decision.clone();
    // Shadow duhet të marrë efektin REAL të aplikuar, jo vetëm kërkesën fillestare.
    hcp_decision.hw_directive = hcp_application.applied_directive;
    let hcp_id = quantum_lim::hw_real::ffi::fnv1a_64(input.trace_id.as_bytes());
    let hcp_inbound = match &hcp_l2_adjustment {
        Some(adjusted) => quantum_lim::bridge_shadow::HcpShadowBridge::from_l2_decision(
            hcp_id,
            adjusted,
            0x4843_5050_524F_0001u64,
        ),
        None => quantum_lim::bridge_shadow::HcpShadowBridge::from_orchestration(
            hcp_id,
            &hcp_decision,
            0x4843_5050_524F_0001u64,
        ),
    };
    let (hcp_f_id, hcp_f_gen, hcp_f_nonce, hcp_f_bit) = hcp_inbound.as_record_args();
    println!(
        "  [HCP_PRO]  requested={} applied={} sealed={} token_gen={} lease_after=0x{:X}",
        hcp_application.decision.hw_directive.label(),
        hcp_decision.hw_directive.label(),
        hcp_decision.sealed,
        hcp_decision.token_gen,
        hcp_application.lease_after.lease_id,
    );
    let hcp_action = [
        hcp_f_id,
        hcp_f_gen,
        hcp_f_nonce,
        u64::from(hcp_f_bit),
        u64::from(hcp_decision.sealed as u8),
        hcp_application.lease_after.lease_id,
        hcp_l3_activation.active_count as u64,
    ];
    quantum_lim::runtime_pulse::mark_action(
        quantum_lim::runtime_pulse::Stage::Hcp,
        &hcp_action,
    );

    // ══ PD FAZA B — Spine 9 kthen Layer 1/2/3 te PD; vetëm tani lind pre-seal.
    let pd_prepared: Option<PdPreparedCycle> = match pd_spine_cycle {
        Some(cycle) => {
            let (layer2_done, layer3_done, evidence_digest) = match &phase9.spine {
                Some(spine) => {
                    let l2 = !spine.evidence.claim.is_empty()
                        && !spine.evidence.reasoning.is_empty();
                    let l3 = spine.hardware.ready_for_pim;
                    let digest = quantum_lim::pd_spine_contract::digest_parts(&[
                        spine.evidence.claim.as_bytes(),
                        spine.evidence.reasoning.as_bytes(),
                        &spine.probe.claim_id.to_le_bytes(),
                    ]);
                    (l2, l3, digest)
                }
                None => (false, false, 0),
            };
            let layer1_done = hcp_decision.sealed;
            let layer_mask = ((layer1_done as u8)
                    * quantum_lim::pd_spine_contract::LAYER_1_MASK)
                | ((layer2_done as u8)
                    * quantum_lim::pd_spine_contract::LAYER_2_MASK)
                | ((layer3_done as u8)
                    * quantum_lim::pd_spine_contract::LAYER_3_MASK);
            let activation_contract_digest = cycle.request.activation.contract_digest;
            let layer1 = quantum_lim::pd_spine_contract::PdLayerReceipt::new(
                quantum_lim::pd_spine_contract::LAYER_1_MASK,
                cycle.request.activation_id,
                activation_contract_digest,
                cycle.request.gcl.process_digest,
                0,
                quantum_lim::pd_spine_contract::digest_parts(&[
                    &hcp_decision.token_gen.to_le_bytes(), &[hcp_decision.sealed as u8],
                ]),
                layer1_done,
            );
            let layer2 = quantum_lim::pd_spine_contract::PdLayerReceipt::new(
                quantum_lim::pd_spine_contract::LAYER_2_MASK,
                cycle.request.activation_id,
                activation_contract_digest,
                cycle.request.gcl.process_digest,
                layer1.result_digest,
                evidence_digest,
                layer2_done,
            );
            let layer3 = quantum_lim::pd_spine_contract::PdLayerReceipt::new(
                quantum_lim::pd_spine_contract::LAYER_3_MASK,
                cycle.request.activation_id,
                activation_contract_digest,
                cycle.request.gcl.process_digest,
                layer2.result_digest,
                quantum_lim::pd_spine_contract::digest_parts(&[
                    &sv.mass.to_le_bytes(), &[layer3_done as u8],
                ]),
                layer3_done,
            );
            let spine_completion = quantum_lim::pd_spine_contract::PdSpineCompletion {
                request: cycle.request.clone(), layer_mask, layer1, layer2, layer3,
                evidence_digest: quantum_lim::pd_spine_contract::digest_parts(&[
                    &evidence_digest.to_le_bytes(), &hcp_decision.token_gen.to_le_bytes(),
                    &sv.mass.to_le_bytes(), &activation_contract_digest.to_le_bytes(),
                ]),
                mpro_mass: quantum_lim::pd_spine_contract::FixedMass::new(sv.mass),
                ready_for_shadow: phase9.ready_for_shadow && q_evidence_only
                    && layer1_done && layer2_done && layer3_done,
                completed_at: input.ts,
            };
            let parent_i0 = match source_i0.as_deref() {
                Some(v) => v,
                None => {
                    eprintln!("  [PD→SPINE9] i₀ burimor mungon; pre-seal refuzohet");
                    return false;
                }
            };
            match pd.prepare_after_spine_sealed(
                &input.trace_id,
                parent_i0,
                cycle,
                spine_completion,
                input.ts,
                SEAL_PD,
            ) {
                Ok(prepared) => {
                    println!("  [PD→SPINE9] Layer 1/2/3 u kthyen te PD → PendingNextI0/pre-seal aktiv");
                    Some(prepared)
                }
                Err(e) => {
                    eprintln!("  [PD→SPINE9] pre-seal refuzuar: {e}");
                    None
                }
            }
        }
        None => None,
    };
    let (
        pd_pending_next,
        pd_binding_digest,
        pd_continuum_activation_digest,
        pd_parent_i0,
        pd_spine_evidence,
    ) = match pd_prepared {
        Some(PdPreparedCycle { pending_next, spine_completion, .. }) => {
            let binding = pending_next.as_ref().map(|pending| pending.binding_digest());
            let continuum = pending_next.as_ref().map(|pending|
                pending.spine_completion.request.continuum.activation_digest
            );
            let parent_i0 = pending_next.as_ref().map(|pending| pending.parent_i0.clone());
            let mut layer1_material = Vec::new();
            layer1_material.extend_from_slice(&hcp_decision.token_gen.to_le_bytes());
            layer1_material.push(hcp_decision.sealed as u8);
            let layer2_material = match &phase9.spine {
                Some(spine) => {
                    let mut material = Vec::new();
                    material.extend_from_slice(spine.evidence.claim.as_bytes());
                    material.extend_from_slice(spine.evidence.reasoning.as_bytes());
                    material.extend_from_slice(&spine.probe.claim_id.to_le_bytes());
                    material
                }
                None => Vec::new(),
            };
            let mut layer3_material = Vec::new();
            layer3_material.extend_from_slice(&sv.mass.to_le_bytes());
            layer3_material.push(spine_completion.layer3.completed as u8);
            let spine_evidence = Some(pd_spine_evidence(
                &spine_completion,
                layer1_material,
                layer2_material,
                layer3_material,
            ));
            (pending_next, binding, continuum, parent_i0, spine_evidence)
        }
        None => (None, None, None, None, None),
    };

    // PD është procesi aktivizues i Spine 9. Pa vazhdimësinë PD/GCL nuk del
    // paketë drejt Shadow; ky është kufi kushtetues, jo fail-open terminal.
    let pd_spine_evidence = match (
        pd_spine_evidence,
        pd_binding_digest,
        pd_continuum_activation_digest,
    ) {
        (Some(spine), Some(_), Some(_)) => spine,
        _ => {
            eprintln!("  [PD→SHADOW] FAIL-CLOSED: paketa finale s'ka vazhdimësi PD/GCL/Spine9");
            return false;
        }
    };

    // ════════ SHADOW — COMPLETE MEDIATION PËRMES main.rs (v1.6.0) ════════
    // Quantum nuk linkon më shadow_platform. Ai njeh vetëm format wire të
    // shadow_contracts; autoriteti, vault-i, ingest-i dhe receipt-i ekzistojnë
    // vetëm brenda procesit shadow_platform/main.rs.
    ucl_step(&mut ucl, UclEvent::IssueUltimatum, &mut ucl_breach); // XY | Refuzim

    // ── PORTA E LIGJIT: "Asgjë nuk del pa kaluar Quantum_Collapse." ────────
    match (ucl_breach, ucl.is_fulfilled()) {
        (None, true) => println!("  [ULTIMATUM] muskuli u plotësua: {:?} (Reasoning Collapse ✓)",
            ucl.state()),
        (b, _) => {
            eprintln!("  [ULTIMATUM] FAIL-CLOSED: {} — asgjë s'del drejt Shadow",
                b.unwrap_or("muskuli s'arriti UltimatumIssued"));
            return false;
        }
    }

    let (neg_reason, neg_process, neg_mass, neg_freq, neg_blob) =
        neg_inbound.as_on_negative_args();

    // ══ PIM + NPIM + MPRO — PAKETA FINALE E EVIDENCAVE ═══════════════════
    // Modulet nuk komunikojnë veçmas me Shadow. PIM/NPIM paketojnë provat;
    // MPRO sjell 16 matjet reale; Shadow i rillogarit dhe i vulos.
    let mut mpro_measurements = Vec::with_capacity(
        shadow_contracts::MPRO_EVALUATIONS,
    );
    mpro_measurements.extend_from_slice(&pro4);
    mpro_measurements.extend_from_slice(&npro4);
    mpro_measurements.extend_from_slice(&hpro4);
    mpro_measurements.extend_from_slice(&apro_arg.measures);

    let pim_suggestion = match &pass.shadow_suggestion.suggested_verdict {
        quantum_lim::pim::SuggestedVerdict::ConsiderOptimal => 1u8,
        quantum_lim::pim::SuggestedVerdict::HoldForTesting => 2u8,
        quantum_lim::pim::SuggestedVerdict::ExploreFurther => 0u8,
    };
    let npim_suggestion = match npim.suggestion {
        quantum_lim::npim::NegativeSuggestion::ConfirmNegative => 2u8,
        quantum_lim::npim::NegativeSuggestion::HoldNegative => 1u8,
        quantum_lim::npim::NegativeSuggestion::WeakElimination => 0u8,
    };
    let fixed = |value: f32| -> u32 {
        shadow_contracts::fixed_mass(value).unwrap_or(0)
    };
    let pim_proof_chain: Vec<String> = pass.proof_chain.iter().map(|proof| {
        format!("{}|{}|{}", proof.source, proof.claim, proof.verifiable as u8)
    }).collect();
    let npim_arguments: Vec<String> = npim.arguments.iter().map(|argument| {
        format!(
            "{}|{}|{}|{:08x}",
            argument.operator,
            argument.cause,
            argument.consequence,
            argument.weakness_score.to_bits(),
        )
    }).collect();
    let quantum_action_state = quantum_lim::runtime_pulse::action_state();
    let quantum_action_evidence: Vec<shadow_contracts::ActionEvidenceWire> =
        quantum_lim::runtime_pulse::action_evidence().into_iter().map(|entry|
            shadow_contracts::ActionEvidenceWire {
                stage: entry.stage,
                evidence_words: entry.evidence_words,
            }
        ).collect();
    let quantum_action_mask = quantum_lim::runtime_pulse::action_mask();
    let quantum_required_action_mask = quantum_lim::runtime_pulse::REQUIRED_ACTION_MASK;
    match quantum_lim::runtime_pulse::action_convergence_complete() {
        true => {}
        false => {
            eprintln!(
                "  [GCL/BESIM] FAIL-CLOSED: konvergjenca e organeve është e paplotë mask=0x{:04X} required=0x{:04X}",
                quantum_action_mask,
                quantum_required_action_mask,
            );
            return false;
        }
    }

    // ══ GCL SCIENTIFIC PROJECT CONTINUUM v1.6.4 ═══════════════════════════
    // Projekti nuk hap degë paralele: dëshmitari APUPK i Shadow-it validohet
    // në hyrje; Digital Lab prodhon TRL/trace pa verdict; i gjithë materiali
    // futet në të njëjtën FinalEvidenceWire që kalon Shadow main mediation.
    let scientific_project = match input.scientific_project.as_ref() {
        Some(project) => {
            let mut governance = quantum_lim::Governance::new();
            let mut cognitive_trace = quantum_lim::RawCognitiveTrace::new();
            let lab = quantum_lim::LabSystemBridge::run_integrated(
                &project.title,
                &input.domain,
                &input.text,
                &project.hypothesis,
                project.assumptions.clone(),
                &mut governance,
                &mut cognitive_trace,
                &input.trace_id,
            );
            let file_material = project.files.clone();
            let evidence_sha256 = quantum_lim::gcl_project_contract::evidence_sha256(
                &quantum_lim::gcl_project_contract::ProjectEvidenceMaterial {
                    context_sha256: &project.context.context_sha256,
                    project_title: &project.title,
                    domain: &input.domain,
                    hypothesis: &project.hypothesis,
                    assumptions: &project.assumptions,
                    gcl_process_digest: pd_spine_evidence.gcl_process_digest,
                    trl_level: lab.evidence.trl_level,
                    trl_passed: lab.evidence.trl_passed,
                    trl_confidence: fixed(lab.evidence.confidence),
                    trl_reproducibility: fixed(lab.evidence.reproducibility),
                    lab_test_id: lab.evidence.lab_test_id.as_deref().unwrap_or(""),
                    findings: &lab.evidence.findings,
                    description: &project.description,
                    files: &file_material,
                },
            );
            let wire_files = file_material.into_iter().map(|(kind, bytes)|
                shadow_contracts::ProjectEvidenceFileWire { kind, bytes }
            ).collect::<Vec<_>>();
            println!(
                "  [GCL/PROJECT] project={} TRL{} passed={} laws={} evidence={}",
                project.context.project_id,
                lab.evidence.trl_level,
                lab.evidence.trl_passed,
                lab.laws_passed,
                &evidence_sha256[..16],
            );
            Some(shadow_contracts::ScientificProjectWire {
                context: project.context.clone(),
                project_title: project.title.clone(),
                domain: input.domain.clone(),
                hypothesis: project.hypothesis.clone(),
                assumptions: project.assumptions.clone(),
                gcl_process_digest: pd_spine_evidence.gcl_process_digest,
                trl_level: lab.evidence.trl_level,
                trl_passed: lab.evidence.trl_passed,
                trl_confidence: fixed(lab.evidence.confidence),
                trl_reproducibility: fixed(lab.evidence.reproducibility),
                lab_test_id: lab.evidence.lab_test_id.unwrap_or_default(),
                findings: lab.evidence.findings,
                description: project.description.clone(),
                files: wire_files,
                evidence_sha256,
            })
        }
        None => None,
    };

    let mut final_evidence = shadow_contracts::FinalEvidenceWire {
        pd_spine: pd_spine_evidence,
        quantum_action_state,
        quantum_action_evidence,
        quantum_action_mask,
        quantum_required_action_mask,
        light_input_sha256: input.input_sha256.to_ascii_lowercase(),
        light_input_bytes: input.text.as_bytes().to_vec(),
        pim_evidence_density: fixed(pass.potentiality.evidence_density),
        pim_logical_coherence: fixed(pass.potentiality.logical_coherence),
        pim_causal_integrity: fixed(pass.potentiality.causal_integrity),
        pim_convergence_strength: fixed(pass.potentiality.convergence_strength),
        pim_reproducibility: fixed(pass.potentiality.reproducibility),
        pim_potentiality_score: fixed(pass.potentiality.potentiality_score),
        pim_suggestion,
        pim_proof_chain,
        npim_failure_density: fixed(npim.negativity.failure_density),
        npim_contradiction_strength: fixed(npim.negativity.contradiction_strength),
        npim_elimination_certainty: fixed(npim.negativity.elimination_certainty),
        npim_negativity_score: fixed(npim.negativity.negativity_score),
        npim_suggestion,
        npim_arguments,
        npim_arguments_blob_digest: shadow_contracts::fnv1a64(neg_blob),
        mpro_measurements,
        mpro_positives: sv.positives,
        mpro_total: sv.total,
        mpro_vector_mass: sv.mass,
        mpro_evidence_mass: evidence_mass_fixed.raw(),
        mpro_factic_mass: mpro_factic.raw(),
        scientific_project,
        package_digest: 0,
    };
    final_evidence.package_digest = final_evidence.recompute_digest();
    let (expected_project_id, expected_project_context_sha256, expected_project_evidence_sha256,
         expected_project_context_digest, expected_project_evidence_digest) =
        match final_evidence.scientific_project.as_ref() {
            Some(project) => {
                let context_sha256 = match quantum_lim::gcl_project_contract::parse_sha256(
                    &project.context.context_sha256,
                ) {
                    Some(value) => value,
                    None => {
                        eprintln!("  [GCL/PROJECT] FAIL-CLOSED: context SHA-256 jo kanonik");
                        return false;
                    }
                };
                let evidence_sha256 = match quantum_lim::gcl_project_contract::parse_sha256(
                    &project.evidence_sha256,
                ) {
                    Some(value) => value,
                    None => {
                        eprintln!("  [GCL/PROJECT] FAIL-CLOSED: evidence SHA-256 jo kanonik");
                        return false;
                    }
                };
                (
                    project.context.project_id,
                    context_sha256,
                    evidence_sha256,
                    quantum_lim::gcl_project_contract::digest64_bytes(&context_sha256),
                    quantum_lim::gcl_project_contract::digest64_bytes(&evidence_sha256),
                )
            }
            None => (0, [0u8; 32], [0u8; 32], 0, 0),
        };
    match final_evidence.verifies_internal() {
        true => println!(
            "  [PIM/NPIM/MPRO] paketë finale e verifikueshme digest={:016x}",
            final_evidence.package_digest,
        ),
        false => {
            eprintln!("  [PIM/NPIM/MPRO] FAIL-CLOSED: paketa finale nuk rillogaritet");
            return false;
        }
    }

    let shadow_request = shadow_contracts::ShadowCycleRequest {
        quantum: shadow_contracts::QuantumInboundWire {
            package_id:           input.trace_id.clone(),
            session_id:           input.trace_id.clone(),
            territory:            territory.to_string(),
            epistemic_mass:       lim_metric.epistemic_mass,
            evidence_density:     pass.potentiality.evidence_density,
            logical_coherence:    pass.potentiality.logical_coherence,
            causal_integrity:     pass.potentiality.causal_integrity,
            convergence_strength: pass.potentiality.convergence_strength,
            reproducibility:      pass.potentiality.reproducibility,
            potentiality_score:   pass.potentiality.potentiality_score,
            ibe_score:            pass.potentiality.evidence_density,
            chain_coherence:      pass.potentiality.logical_coherence,
            conservation_ok:      pass.potentiality.causal_integrity > 0.5,
            candidate_scores:     pro.candidates.iter().map(|c| c.score).collect(),
            suggested_verdict:    pim_suggestion,
            primitive_flags:      inbound.primitive_flags,
            raw_bytes:            light_in.payload.clone(),
            xy_x: match &collapse {
                CollapseOutcome::Proposed { x, .. } => Some(x.clone()),
                CollapseOutcome::Refused  { .. }    => None,
            },
            xy_y: match &collapse {
                CollapseOutcome::Proposed { y, .. } => Some(y.clone()),
                CollapseOutcome::Refused  { .. }    => None,
            },
            xy_mass: match &collapse {
                CollapseOutcome::Proposed { .. } => Some(mpro_factic.raw()),
                CollapseOutcome::Refused  { .. } => None,
            },
            pd_binding_digest,
            pd_continuum_activation_digest,
            pd_parent_i0,
            collapse_refused: match &collapse {
                CollapseOutcome::Proposed { .. }     => None,
                CollapseOutcome::Refused  { reason } => Some(reason.clone()),
            },
            negative_paths: neg_paths.iter().map(|n| n.to_wire()).collect(),
            hcp_input_id:         Some(hcp_f_id),
            hcp_token_gen:        Some(hcp_f_gen),
            hcp_token_nonce:      Some(hcp_f_nonce),
            hcp_directive_bit:    Some(hcp_f_bit),
            final_evidence,
        },
        light: shadow_contracts::LightInboundWire {
            session_id:      light_in.session_id,
            territory:       light_in.territory,
            primitive_flags: light_in.primitive_flags,
            epistemic_mass:  light_in.epistemic_mass,
            proof_chain:     light_in.proof_chain,
            payload:         light_in.payload,
        },
        negative: shadow_contracts::NegativeKnowledgeWire {
            input_id: hcp_id,
            reason: neg_reason,
            process: neg_process.to_string(),
            mass: neg_mass,
            frequency: neg_freq,
            suggestion_code: neg_inbound.suggestion_code,
            bytes: neg_blob.to_vec(),
            ts_ns: shadow_process_bridge::now_ns(),
        },
    };

    let shadow_response = match shadow_process_bridge::execute_cycle(shadow_request) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("  [Q→SHADOW MAIN] FAIL-CLOSED: {e}");
            return false;
        }
    };

    // GCL VULA E GJALLË × QUANTUM: marrësi rillogarit të njëjtin puls SHA-256.
    // Shadow sjell verdiktin; Quantum sjell action_state-in e vet real dhe ligjet
    // byte-identike. Asnjëra palë nuk i beson thjesht string-ut të wire-it.
    let trust_wire_identity = match quantum_lim::living_trust_contract::parse_hex_32(
        &shadow_response.verdict.living_trust_sha256,
    ) {
        Some(value) => value,
        None => {
            eprintln!("  [GCL/BESIM×Q] FAIL-CLOSED: SHA-256 i Shadow nuk është kanonik");
            return false;
        }
    };
    let shadow_project_context_sha256 = match
        quantum_lim::gcl_project_contract::parse_sha256(
            &shadow_response.verdict.project_context_sha256,
        ) {
            Some(value) => value,
            None => {
                eprintln!("  [GCL/BESIM×Q] FAIL-CLOSED: project context SHA-256 i Shadow jo kanonik");
                return false;
            }
        };
    let shadow_project_evidence_sha256 = match
        quantum_lim::gcl_project_contract::parse_sha256(
            &shadow_response.verdict.project_evidence_sha256,
        ) {
            Some(value) => value,
            None => {
                eprintln!("  [GCL/BESIM×Q] FAIL-CLOSED: project evidence SHA-256 i Shadow jo kanonik");
                return false;
            }
        };
    let trust_proof = quantum_lim::living_trust_contract::LivingTrustProof {
        action_state: quantum_action_state,
        action_mask: quantum_action_mask,
        required_action_mask: quantum_required_action_mask,
        verified: shadow_response.verdict.verified,
        primitive: shadow_response.verdict.primitive,
        knowledge_band: shadow_response.verdict.knowledge_band,
        lgc_law: shadow_response.verdict.lgc_law,
        system_laws_seal:
            quantum_lim::lab_contracts::gjata_collapse_law::system_laws_seal(),
        scientific_project_sha256: expected_project_evidence_sha256,
        scientific_project_verdict_sha256:
            quantum_lim::gcl_project_contract::verdict_sha256_or_zero(
                quantum_lim::gcl_project_contract::ProjectVerdictMaterial {
                    project_id: shadow_response.verdict.project_id,
                    project_status: shadow_response.verdict.project_status,
                    project_context_sha256: shadow_project_context_sha256,
                    project_evidence_sha256: shadow_project_evidence_sha256,
                    novel_factualized: shadow_response.verdict.novel_factualized,
                    novel_trl_level: shadow_response.verdict.novel_trl_level,
                    novel_proof_score: shadow_response.verdict.novel_proof_score,
                    novel_rejection: shadow_response.verdict.novel_rejection,
                },
            ),
        sovereign_flags: inbound.primitive_flags,
    };
    let quantum_living_trust = match quantum_lim::living_trust_contract::compute(
        trust_proof,
        shadow_response.verdict.legacy_score,
    ) {
        Some(seal) => seal,
        None => {
            eprintln!("  [GCL/BESIM×Q] FAIL-CLOSED: verdikti nuk prodhoi Besim kushtetues");
            return false;
        }
    };
    let trust_matches = shadow_response.verdict.living_trust_action_state
            == quantum_action_state
        && shadow_response.verdict.living_trust_action_mask == quantum_action_mask
        && shadow_response.verdict.living_trust_required_action_mask
            == quantum_required_action_mask
        && shadow_response.verdict.project_id == expected_project_id
        && shadow_response.verdict.project_context_digest == expected_project_context_digest
        && shadow_response.verdict.project_evidence_digest == expected_project_evidence_digest
        && shadow_project_context_sha256 == expected_project_context_sha256
        && shadow_project_evidence_sha256 == expected_project_evidence_sha256
        && shadow_response.verdict.living_trust_system_laws_seal
            == trust_proof.system_laws_seal
        && shadow_response.verdict.living_trust_sovereign_flags
            == trust_proof.sovereign_flags
        && shadow_response.verdict.living_trust_sovereign_value
            == quantum_living_trust.sovereign_value
        && trust_wire_identity == quantum_living_trust.identity_sha256
        && shadow_response.verdict.living_trust_intensity
            == quantum_living_trust.intensity
        && shadow_response.verdict.living_trust_kind == quantum_living_trust.kind;
    match trust_matches {
        true => println!(
            "  [GCL/BESIM×Q] SHA256={} intensity={} kind={} (Shadow=Quantum ✓)",
            quantum_living_trust.identity_hex(),
            quantum_living_trust.intensity,
            quantum_living_trust.kind,
        ),
        false => {
            eprintln!("  [GCL/BESIM×Q] FAIL-CLOSED: Shadow dhe Quantum nuk prodhuan të njëjtën vulë");
            return false;
        }
    }
    let living_trust_digest = quantum_living_trust.identity_digest();
    match shadow_response.verification_receipt.living_trust_digest == living_trust_digest {
        true => {}
        false => {
            eprintln!("  [GCL/BESIM×Q] FAIL-CLOSED: receipt-i sovran nuk mban digest-in e Besimit");
            return false;
        }
    }

    // FAZA 2 E PD: nuk kërkon seal nga Shadow. Merr output-in aktual tashmë të
    // mbyllur nga Shadow: receipt-i provon se seal-i i verifikimit hapi dhe
    // konsumoi LgcToken brenda autoritetit të Shadow.
    let verification = PdVerificationCompletion {
        session_id: shadow_response.verification_receipt.session_id.clone(),
        parent_i0: shadow_response.verification_receipt.parent_i0.clone(),
        primitive_anchor: shadow_response.verification_receipt.primitive_anchor,
        xy_digest: shadow_response.verification_receipt.xy_digest,
        pd_binding_digest: shadow_response.verification_receipt.pd_binding_digest,
        pd_continuum_activation_digest:
            shadow_response.verification_receipt.pd_continuum_activation_digest,
        y_verdict: shadow_response.verification_receipt.y_verdict,
        x_verdict: shadow_response.verification_receipt.x_verdict,
        verification_seal: shadow_response.verification_receipt.seal,
        verification_generation: shadow_response.verification_receipt.generation,
        verification_receipt_id: shadow_response.verification_receipt.receipt_id.clone(),
        living_trust_sha256: quantum_living_trust.identity_sha256,
        living_trust_intensity: quantum_living_trust.intensity,
        living_trust_kind: quantum_living_trust.kind,
        living_trust_sovereign_value: quantum_living_trust.sovereign_value,
        living_trust_digest,
    };

    let pd_closure: Option<PdContinuumClosure> = match pd_pending_next {
        Some(pending) => match pd.finalize_after_verification(pending, verification) {
            PdFinalization::Released(closure) => {
                println!(
                    "  [PD_CONTINUUM] output={} + iZ={} → next-i₀={} (XY=1/1)",
                    closure.output.output_id, closure.iz.iz_id, closure.next_i0.next_i0_id,
                );
                quantum_lim::runtime_pulse::mark_ready(
                    quantum_lim::runtime_pulse::Stage::PdContinuum,
                );
                Some(closure)
            }
            PdFinalization::RebuiltFromNegative(closure) => {
                println!(
                    "  [PD_CONTINUUM] output={} + iZ={} → next-i₀={} (XY=0/0 rebuild)",
                    closure.output.output_id, closure.iz.iz_id, closure.next_i0.next_i0_id,
                );
                quantum_lim::runtime_pulse::mark_ready(
                    quantum_lim::runtime_pulse::Stage::PdContinuum,
                );
                Some(closure)
            }
            PdFinalization::Rejected { reason } => {
                quantum_lim::runtime_pulse::mark_notready(
                    quantum_lim::runtime_pulse::Stage::PdContinuum,
                );
                eprintln!("  [PD]       kontinuumi i₀→iZ u refuzua: {reason}");
                None
            }
        },
        None => {
            quantum_lim::runtime_pulse::mark_degraded(
                quantum_lim::runtime_pulse::Stage::PdContinuum,
            );
            None
        }
    };
    let pd_next: Option<&PdNextI0> = pd_closure.as_ref().map(|c| &c.next_i0);

    // Reasoning quhet i plotë vetëm pasi PD ka mbyllur output+iZ ose ka
    // deklaruar qartë gjendjen e tij. Kështu GCL_PRESUME lexon pulsin real.
    {
        use quantum_lim::lab_contracts::{gcl_presume, CollapsePhase};
        let d = std::env::var("ESSMAI_HANDOFF_DIR")
            .unwrap_or_else(|_| std::env::temp_dir().to_string_lossy().into_owned());
        let _ = gcl_presume::sweep("QUANTUM", CollapsePhase::Reasoning, &d);
    }

    // Vendimi REAL nga Shadow (verified/primitive 0/1).
    let sv = &shadow_response.verdict;
    let outcome = match (sv.is_verified(), sv.is_primitive()) {
        (true, true)  => SupremeOutcome::PrimitiveKnowledge,
        (true, false) => SupremeOutcome::VerifiableNotPrimitive,
        _             => SupremeOutcome::VerifiableNotPrimitive,
    };
    println!("  [SHADOW]   verdict={} (bit={}) | verified={} primitive={} band={}",
        outcome.label(), outcome.bit(),
        sv.verified, sv.primitive, sv.knowledge_band_label());

    // ════════ NPIM → SHADOW MAIN ════════
    // Negative Knowledge u dërgua në të NJËJTËN kërkesë të ndërmjetësuar dhe
    // u persistua nga e njëjta instancë Shadow që prodhoi receipt-in.
    let npim_persisted = shadow_response.negative_persisted;
    let negative_verdict = matches!((sv.verified, sv.primitive), (0, 0));
    match (negative_verdict, npim_persisted) {
        (true, true) => println!(
            "  [NPIM→SHADOW MAIN] negative e verifikuar u ruajt: code={} mass={:.3} freq={}",
            neg_reason, neg_mass, neg_freq,
        ),
        (true, false) => eprintln!(
            "  [NPIM→SHADOW MAIN] FAIL-CLOSED: negative e verifikuar s'u persistua"
        ),
        (false, false) => println!(
            "  [NPIM→SHADOW MAIN] rruga nuk është negative; asnjë Negative Knowledge nuk u shkrua"
        ),
        (false, true) => eprintln!(
            "  [NPIM→SHADOW MAIN] ALARM: rruga pozitive/hold nuk duhet të persistojë Negative Knowledge"
        ),
    }

    // Quantum PD → PD Light → Nura. Për rrugën negative kërkohet që NPIM të
    // jetë ruajtur; për rrugën pozitive receipt-i 1/1 mjafton.
    let may_export_pd = match pd_next.map(|n| n.basis) {
        Some(quantum_lim::progressive_debatic::PdContinuationBasis::VerifiedNegativeRebuild) =>
            npim_persisted,
        _ => true,
    };
    match (may_export_pd, pd_pending_package.as_ref(), pd_closure.as_ref()) {
        (true, Some(pkg), closure) => export_pd_handoff(
            &input.trace_id,
            pkg,
            closure,
            &shadow_response.verification_receipt,
            &shadow_response.verdict,
        ),
        (true, None, Some(closure)) => export_pd_probe(
            &input.trace_id,
            closure,
            &shadow_response.verification_receipt,
            &shadow_response.verdict,
        ),
        (false, _, Some(closure)) => eprintln!(
            "  [PD→LIGHT] next-i₀={} u mbajt: Negative Knowledge s'u persistua",
            closure.next_i0.next_i0_id,
        ),
        _ => {}
    }

    // ════════ QUANTUM → LIGHT (serialize) ════════
    let axis_score = pro.candidates.first().map(|c| c.score * 100.0).unwrap_or(0.0);
    let output = LightQuantumBridge::build_output(
        &input.trace_id, outcome,
        pass.potentiality.potentiality_score, axis_score,
        territory, hw.depth_hint.name(),
        "reasoning i përfunduar", input.ts + 1,
    );
    let payload = output.serialize();
    println!("  [Q→LIGHT]  {}", payload);

    // Workload-i mbaroi: buffer-at dalin nga scope-i para context-it;
    // receipt-i mban numrin e tyre të regjistruar dhe worker-at tashmë joined.
    drop(frags);
    let release_receipt = hardware_runtime.release("quantum_cycle_complete");
    hpro_runtime.mark_released(release_receipt);
    println!(
        "  [HW_RELEASE] cycle=0x{:X} lease=0x{:X} workers={} buffers={} buffer_bytes={} completed={} reason={}",
        release_receipt.cycle_id,
        release_receipt.lease_id,
        release_receipt.workers_joined,
        release_receipt.buffers_released,
        release_receipt.buffer_bytes_released,
        release_receipt.completed,
        release_receipt.reason,
    );
    true
}



/// v1.6.5 — porta e re Project Workspace.
///
/// Kjo rrugë vetëm orienton projektin drejt magazinimit/bisedës dhe prodhon
/// identitete SHA-256 të rekordeve. Ajo nuk thërret `run`, nuk aktivizon PD/TRL
/// dhe nuk hap ose krijon token. Rruga legacy `--project-process-once` mbetet
/// e pandryshuar poshtë saj.
fn dispatch_project_workspace_once(args: &[String]) -> Option<i32> {
    match args {
        [_, flag, request_path, response_path] if flag == "--project-workspace-once" => {
            Some(match run_project_workspace_once(
                std::path::Path::new(request_path),
                std::path::Path::new(response_path),
            ) {
                Ok(()) => 0,
                Err(error) => {
                    eprintln!("[QUANTUM PROJECT WORKSPACE] {error}");
                    70
                }
            })
        }
        [_, flag, ..] if flag.starts_with("--project-workspace") => {
            eprintln!("[QUANTUM PROJECT WORKSPACE] argumente të pavlefshme");
            Some(64)
        }
        _ => None,
    }
}

fn run_project_workspace_once(
    request_path: &std::path::Path,
    response_path: &std::path::Path,
) -> Result<(), String> {
    let bytes = std::fs::read(request_path)
        .map_err(|e| format!("project workspace request s'u lexua: {e}"))?;
    let request_sha256 = quantum_lim::sovereign::sha256_hex(&bytes);
    let request = shadow_contracts::decode_quantum_project_execution_request(&bytes)
        .map_err(|e| format!("project workspace request u refuzua: {e}"))?;
    let payload_sha256 = quantum_lim::sovereign::sha256_hex(request.payload.as_bytes());
    let input = QuantumInput::deserialize(&request.payload)
        .map_err(|e| format!("QuantumInput i project workspace u refuzua: {e:?}"))?;
    let project = input.scientific_project.as_ref()
        .ok_or_else(|| "project workspace request nuk mban ScientificProjectInput".to_string())?;
    let request_matches = request.project_id != 0
        && request.project_trace_id != 0
        && request.payload_sha256 == payload_sha256
        && request.project_id == project.context.project_id
        && request.project_trace_id == project.context.trace_id
        && request.project_context_sha256 == project.context.context_sha256;
    match request_matches {
        true => {}
        false => return Err(
            "project workspace request nuk lidhet me payload/APUPK witness".to_string(),
        ),
    }

    let orientation = quantum_lim::project_workspace_router::orient(
        quantum_lim::project_workspace_router::ProjectWorkspaceMaterial {
            project_id: request.project_id,
            project_trace_id: request.project_trace_id,
            project_context_sha256: &request.project_context_sha256,
            request_sha256: &request_sha256,
            domain: &input.domain,
            turn_text: &input.text,
            title: &project.title,
            hypothesis: &project.hypothesis,
            assumptions: &project.assumptions,
            description: &project.description,
            files: &project.files,
        },
    ).map_err(|error| format!("project workspace orientation u refuzua: {error:?}"))?;

    atomic_write_project_response(response_path, orientation.encode_line().as_bytes())
}

fn dispatch_project_process_once(args: &[String]) -> Option<i32> {
    match args {
        [_, flag, request_path, response_path] if flag == "--project-process-once" => {
            Some(match run_project_process_once(
                std::path::Path::new(request_path),
                std::path::Path::new(response_path),
            ) {
                Ok(()) => 0,
                Err(error) => {
                    eprintln!("[QUANTUM PROJECT/GCL] {error}");
                    70
                }
            })
        }
        [_, flag, ..] if flag.starts_with("--project-process") => {
            eprintln!("[QUANTUM PROJECT/GCL] argumente të pavlefshme");
            Some(64)
        }
        _ => None,
    }
}

fn run_project_process_once(
    request_path: &std::path::Path,
    response_path: &std::path::Path,
) -> Result<(), String> {
    let bytes = std::fs::read(request_path)
        .map_err(|e| format!("project request s'u lexua: {e}"))?;
    let request_sha256 = quantum_lim::sovereign::sha256_hex(&bytes);
    let request = shadow_contracts::decode_quantum_project_execution_request(&bytes)
        .map_err(|e| format!("project request u refuzua: {e}"))?;
    let payload_sha256 = quantum_lim::sovereign::sha256_hex(request.payload.as_bytes());
    let input = QuantumInput::deserialize(&request.payload)
        .map_err(|e| format!("QuantumInput i projektit u refuzua: {e:?}"))?;
    let project = input.scientific_project.as_ref()
        .ok_or_else(|| "project request nuk mban ScientificProjectInput".to_string())?;
    let request_matches = request.project_id != 0
        && request.project_trace_id != 0
        && request.payload_sha256 == payload_sha256
        && request.project_id == project.context.project_id
        && request.project_trace_id == project.context.trace_id
        && request.project_context_sha256 == project.context.context_sha256;
    match request_matches {
        true => {}
        false => return Err("project request nuk lidhet me payload/APUPK witness".to_string()),
    }
    let completed = run("PROJEKT SHKENCOR REAL (Light→Quantum main)", &request.payload);
    let response = shadow_contracts::QuantumProjectExecutionResponseWire {
        completed,
        reason_code: match completed { true => 0, false => 9 },
        project_id: request.project_id,
        project_trace_id: request.project_trace_id,
        request_sha256,
    };
    atomic_write_project_response(
        response_path,
        &shadow_contracts::encode_quantum_project_execution_response(&response),
    )
}

fn atomic_write_project_response(path: &std::path::Path, bytes: &[u8]) -> Result<(), String> {
    let parent = path.parent().ok_or_else(|| "project response pa parent".to_string())?;
    std::fs::create_dir_all(parent)
        .map_err(|e| format!("project response parent s'u krijua: {e}"))?;
    let mut tmp = path.as_os_str().to_os_string();
    tmp.push(format!(".{}.tmp", std::process::id()));
    let tmp = std::path::PathBuf::from(tmp);
    std::fs::write(&tmp, bytes)
        .map_err(|e| format!("project response temp s'u shkrua: {e}"))?;
    match std::fs::rename(&tmp, path) {
        Ok(()) => Ok(()),
        Err(error) => {
            let _ = std::fs::remove_file(&tmp);
            Err(format!("project response nuk u publikua atomikisht: {error}"))
        }
    }
}

fn main() {
    // ══ BOOT: sistemi E LEXON ligjin — gjata_collapse_law shpallet i gjallë.
    quantum_lim::lab_contracts::proclaim_law("QUANTUM");

    // ══ GCL_PRESUME (v1.4.1): gatishmëria nën ligj — komandanti (CollapsePhase)
    // urdhëron, presume ping-on; edhe modulet anësore përgjigjen me rol+status,
    // zero arsyetim shtesë në to. Sweep-i informon, KURRË s'bllokon nisjen.
    {
        use quantum_lim::lab_contracts::gcl_presume as presume;
        use quantum_lim::lab_contracts::CollapsePhase;
        fn gati_gjithnje() -> presume::GclReadiness { presume::GclReadiness::Ready }
    // v1.4.3 (audit p.3, PRANUAR): READY vjen nga PULSI runtime, jo konstantja.
    presume::register("hpro(layer3)",             presume::GclRole::Support("hpro"), quantum_lim::runtime_pulse::hpro_status);
    presume::register("pro(pro.rs)",              presume::GclRole::ProSelect,     quantum_lim::runtime_pulse::pro_status);
    presume::register("npro(npro_lim_bridge)",    presume::GclRole::NproEliminate, quantum_lim::runtime_pulse::npro_status);
    presume::register("mpro(lim_measurer)",       presume::GclRole::MproMeasure,   quantum_lim::runtime_pulse::mpro_status);
    presume::register("apro(apro.rs)",            presume::GclRole::AproArgue,     quantum_lim::runtime_pulse::apro_status);
    presume::register("pim(pim.rs)",              presume::GclRole::PimPackage,    quantum_lim::runtime_pulse::pim_status);
    presume::register("npim(npim.rs)",            presume::GclRole::NpimPackage,   quantum_lim::runtime_pulse::npim_status);
    presume::register("hcp_pro",                  presume::GclRole::Support("hcp"), quantum_lim::runtime_pulse::hcp_status);
    presume::register("primitive_split_reader",   presume::GclRole::Support("split-runtime"), quantum_lim::runtime_pulse::split_status);
    presume::register("token_forge(burimi 2)",    presume::GclRole::Support("token"),         quantum_lim::token_forge::status);
    presume::register("srk",                      presume::GclRole::Support("provë-shkencore"), quantum_lim::runtime_pulse::srk_status);
    presume::register("ultimatum_collapse_law",   presume::GclRole::Support("ucl"),        gati_gjithnje);
    presume::register(
        "progressive_debatic",
        presume::GclRole::PdContinuum,
        quantum_lim::runtime_pulse::pd_continuum_status,
    );
        // ZGJERIMI v1.4.2 (audit A6): modulet e harduerit + NK hyjnë në apel me
        // STATUS REAL. hw_gate/termik raportojnë Degraded kur `hw_kernel` s'është
        // aktiv (ESSMAI_HW=1) — matja fizike jo e garantuar, por KURRË bllokim.
        // v1.4.3 (audit p.5, PRANUAR): E VËRTETA e kompilimit + pulsi i leximit —
        // env-i NUK e falsifikon dot dhe NUK e fsheh dot kernelin real.
        presume::register("hw_pre_gate(10% floor)", presume::GclRole::Support("harduer"),  quantum_lim::runtime_pulse::hw_kernel_status);
        presume::register("thermal_guard",          presume::GclRole::Support("termik"),   quantum_lim::runtime_pulse::thermal_status);
        presume::register("nk_sync(shadow_nk)",     presume::GclRole::Support("dije-neg"), quantum_lim::runtime_pulse::nk_status);
        let dir_rrjedhe = std::env::var("ESSMAI_HANDOFF_DIR")
            .unwrap_or_else(|_| std::env::temp_dir().to_string_lossy().into_owned());
        // v1.4.3: në boot organet e Reasoning-ut S'KANË rrahur ende — apeli i
        // domosdoshëm i asaj faze bëhet NË RUNTIME (pas paketimit). Boot-i
        // pyet vetëm gatishmërinë e përgjithshme (Coordination: asgjë e
        // detyruar këtu) — zero alarme të rreme, zero READY gënjeshtare.
        let _ = presume::sweep("QUANTUM", CollapsePhase::Coordination, &dir_rrjedhe);

        // ══ FRYMA E PARË (v1.4.4): arsyetuesi provon çekiçin, farkën,
        // ligjin termik dhe vulën NK — në KËTË proces, në KËTË metal.
        use quantum_lim::lab_contracts::first_guardian as fryma;
        fn org_farka() -> bool {
            quantum_lim::token_forge::mint(0x4652_594D_4100_0001) != [0u8; 32]
        }
        fn org_termik_ligji() -> bool {
            use quantum_lim::hw_real::thermal::ThermalState;
            use quantum_lim::hw_real::territories::DepthHint;
            ThermalState::Unknown.effective_hot(DepthHint::VeryDeep)
                && !ThermalState::Normal.effective_hot(DepthHint::VeryDeep)
        }
        fn org_nk_vula() -> bool {
            let s = quantum_lim::pro_nk_gate::KnowledgeVault::seal_negative_export(&[1, 2, 3]);
            quantum_lim::pro_nk_gate::is_sealed_nk_blob(&s)
                && quantum_lim::pro_nk_gate::nk_downgrade(true, false)
        }
        presume::register("fryma_e_pare", presume::GclRole::Support("frymë"),
            quantum_lim::lab_contracts::first_guardian::breath_status);
        let _ = fryma::attest("QUANTUM", &dir_rrjedhe, &[
            fryma::RiteCheck { name: "fnv_known_vector",   run: fryma::known_vector_lock,   critical: true },
            fryma::RiteCheck { name: "pa_wire_roundtrip",  run: fryma::wire_roundtrip_lock,  critical: true },
            fryma::RiteCheck { name: "token_forge_derdh",  run: org_farka,        critical: false },
            fryma::RiteCheck { name: "termik_unknown_ligj", run: org_termik_ligji, critical: false },
            fryma::RiteCheck { name: "nk_vula_dhe_celesi", run: org_nk_vula,      critical: false },
        ]);
    }

    let args: Vec<String> = std::env::args().collect();
    match dispatch_project_workspace_once(&args) {
        Some(code) => std::process::exit(code),
        None => {}
    }
    match dispatch_project_process_once(&args) {
        Some(code) => std::process::exit(code),
        None => {}
    }

    println!("\n  ESS-MAI QUANTUM PLATFORM ");
    println!("  Rrjedha: Light → Quantum → Shadow → Light");
    println!("  Sovereign 0-copy | LIM | HW_REAL | PRO | NPRO/NPIM | SRK | PIM");
    println!("  Gjata Legacy™");

    // ══ DEKLARIMI SOVRAN I SENSORËVE (ZERO IMPROVIZIM) ══════════════════════
    // Vlerat hardware janë REALE vetëm me `--features hw_kernel` (kerneli C
    // lexon RAM/CPU/termik nga makina). Pa të, sistemi punon me vlera NOMINALE
    // të deklaruara — kurrë të maskuara si matje. Operatori e di GJITHMONË.
    match cfg!(feature = "hw_kernel") {
        true  => println!("  [HW]       SENSORË REALË: hw_kernel AKTIV — matjet vijnë nga makina"),
        false => eprintln!("  [HW]       KUJDES: HW_NOMINAL — sensorët JO realë (vlera të deklaruara nominale). Për matje reale: cargo build --features hw_kernel"),
    }

    // ══ RUNTIME I PRODHIMIT (default): konsumon INPUT REAL nga stdin ═══════
    // Formati real i send_to_quantum i Light-it, një payload për rresht.
    // Zero input i ngulitur, zero skenë demo — vetëm rrjedhë e vërtetë.
    // (Audit: demo-ja u hoq nga rruga e prodhimit; izolohet te feature
    //  dev_harness, i cili s'aktivizohet në build-in default.)
    #[cfg(not(feature = "dev_harness"))]
    {
        use std::io::BufRead as _;
        let stdin = std::io::stdin();
        let mut consumed: u64 = 0;
        for line in stdin.lock().lines() {
            let payload = match line {
                Ok(l)  => l,
                Err(e) => { eprintln!("[QUANTUM] stdin i palexueshëm: {e}"); break; }
            };
            match payload.trim().is_empty() {
                true  => {} // rresht bosh → kapërce (fund input i mundshëm)
                false => {
                    consumed += 1;
                    run("INPUT REAL (nga Light, stdin)", payload.trim());
                }
            }
        }
        match consumed {
            0 => {
                eprintln!("[QUANTUM] Asnjë input nga stdin.");
                eprintln!("  Përdorimi: `light | quantum`  ose  `echo \"<payload>\" | quantum`");
                eprintln!("  Payload real: trace_id=..;domain=..;lgc_seal=LGC::C|500;..;text=..");
                eprintln!("  (Skenat demo: kompilo me `--features dev_harness`.)");
            }
            n => println!("\n[QUANTUM] {n} input(e) reale u përpunuan nga stdin."),
        }
    }

    // ══ DEV HARNESS (opt-in, jashtë prodhimit): skenat demonstruese ════════
    #[cfg(feature = "dev_harness")]
    {
        eprintln!("  [DEV_HARNESS] skena demonstruese — JO rrugë prodhimi");
        run("RASTI 1 — INPUT PRIMITIV (me vulë)",
            "trace_id=T001;domain=science;contract_id=KDS:1;lgc_seal=LGC::C|500;lang_code=en;evolve=init;input_sha256=69dfafccfa1130c7c0acd53c57c616bd16d6f384247e99d78b0a4a032ca289b6;ts=1000;text=deterministic reasoning produces verifiable primitive knowledge");
        run("RASTI 2 — INPUT I FRAGMENTUAR (pa vulë)",
            "trace_id=T002;domain=industrial;contract_id=KDS:2;lgc_seal=LGC::C|0;lang_code=en;evolve=init;input_sha256=94e6f7a09486ed2f9e2db15ed94935a82f8f32709096d548007c938468e31730;ts=2000;text=the reactor is stable but also not stable under load");
        run_orchestrator_demo();
        run_lab_demo();
    }
}

/// Demonstron Digital Lab (dev_harness only).
#[cfg(feature = "dev_harness")]
fn run_lab_demo() {
    use quantum_lim::lab::digital_lab::DigitalLab;

    println!("\n{}", "═".repeat(76));
    println!("  DIGITAL LAB — Matja e Saktë (TRL1 → TRL2 → TRL3 + FilterGate)");
    println!("{}", "═".repeat(76));

    // Rasti 1: hipotezë e fortë (duhet të promovohet).
    let strong = DigitalLab::run_full_pipeline(
        "Deterministic Elimination",
        "science",
        "deterministic elimination produces verifiable knowledge without hallucination",
        "elimination measurably reduces false outputs versus probabilistic generation",
        vec![
            "no probabilistic sampling in core".to_string(),
            "formal invariants hold unconditionally".to_string(),
        ],
    );
    match strong {
        Ok(r) => {
            println!("  [RASTI 1 — hipotezë e fortë]");
            println!("    TRL3 score={:.3} | gate_passed={} | promoted={}",
                r.score, r.gate_passed, r.promoted);
            println!("    rejection_reason={}", r.rejection_reason.label());
            println!("    [EVIDENCE→PIM] trl={} passed={} reproducibility={:.3}",
                r.evidence.trl_level, r.evidence.trl_passed, r.evidence.reproducibility);
            println!("    findings: {}", r.evidence.findings);
        }
        Err(e) => println!("    gabim: {:?}", e),
    }

    // Rasti 2: hipotezë pa assumptions (FilterGate refuzon — jo reproducible).
    let weak = DigitalLab::run_full_pipeline(
        "Vague Claim",
        "science",
        "something works somehow",
        "it just works",
        vec![],  // pa assumptions → jo reproducible → FilterGate refuzon
    );
    match weak {
        Ok(r) => {
            println!("  [RASTI 2 — pa assumptions, FilterGate INVARIANT]");
            println!("    promoted={} | rejection_reason={} (invarianti mbrojti)",
                r.promoted, r.rejection_reason.label());
            println!("    [EVIDENCE→PIM] pending_lab={} (PIM e di që s'kaloi)",
                r.evidence.pending_lab);
        }
        Err(e) => println!("    gabim: {:?}", e),
    }

    println!("  ✓ FilterGate = invariant (asnjë promovim pa të)");
    println!("  ✓ Lab MAT → TrlEvidence → PIM (matja e saktë e reproducibility)");
    println!("  ✓ Quantum s'vendos — Shadow konfirmon promovimin final");

    // ── LAB I INTEGRUAR PERFEKT ME SISTEMIN (ligje + trace + VNK) ──────────
    run_integrated_lab_demo();
}

/// Demonstron Digital Lab të INTEGRUAR perfekt me sistemin.
#[cfg(feature = "dev_harness")]
fn run_integrated_lab_demo() {
    use quantum_lim::lab_integration::LabSystemBridge;
    use quantum_lim::governance::Governance;
    use quantum_lim::memory::raw_cognitive_trace::RawCognitiveTrace;

    println!("\n{}", "═".repeat(76));
    println!("  DIGITAL LAB PERFEKT ME SISTEMIN — ligje + trace + VNK + EpistemicState");
    println!("{}", "═".repeat(76));

    let mut governance = Governance::new();
    let mut trace = RawCognitiveTrace::new();

    // Rasti i fortë: kalon ligjet, regjistron trace, prodhon evidencë.
    let result = LabSystemBridge::run_integrated(
        "Sovereign Reasoning",
        "science",
        "deterministic elimination with formal invariants",
        "elimination reduces hallucination measurably and reproducibly",
        vec![
            "no probabilistic sampling".to_string(),
            "formal invariants hold".to_string(),
        ],
        &mut governance,
        &mut trace,
        "integrated_sess",
    );

    println!("  [LIGJET]   laws_passed={} | blocked_reason={}",
        result.laws_passed, result.blocked_reason);
    println!("  [TRACE]    {} gjurmë të regjistruara (TRL1→TRL2→TRL3 me lineage)",
        result.trace_ids.len());
    println!("  [EVIDENCE] trl_level={} passed={} reproducibility={:.3} (→PIM)",
        result.evidence.trl_level, result.evidence.trl_passed,
        result.evidence.reproducibility);
    // v1.4.4 (DEEPTECH): dimensioni i arsyetimit i vektorit TRL — i printuar
    // me etiketën uniforme; input/verification plotësohen nga Light/Shadow
    // (TrlVerdict) në ciklin e ardhshëm të transportit.
    let trl_vec = quantum_lim::lab_contracts::trl::TrlVector::new(
        0, result.evidence.trl_level, 0);
    println!("  [TRL_VEKTOR] {} (deeptech: sistemi = zinxhiri i dobët)", trl_vec.label());

    // VNK demonstrim: hipotezë e dobët → negative knowledge.
    let mut gov2 = Governance::new();
    let mut trace2 = RawCognitiveTrace::new();
    let weak = LabSystemBridge::run_integrated(
        "Vague", "science", "idea", "claim", vec![],
        &mut gov2, &mut trace2, "vnk_sess",
    );
    let vnk_status = match &weak.negative_knowledge {
        Some(vnk) => format!("VNK prodhuar: code={} ({})", vnk.failure_reason_code, vnk.rejection),
        None => "pa VNK".to_string(),
    };
    println!("  [VNK]      {} → për Shadow (byte-for-byte NegativeContext)", vnk_status);

    // v1.6.4: VNK e projektit nuk persiston më me portën e veçuar
    // `--negative-once`. Ajo paketohet brenda FinalEvidenceWire dhe kalon
    // të gjitha klasat e verifikimit në Shadow main → judge_supreme.
    let vnk_ready_for_full_cycle = weak.negative_knowledge.is_some();
    println!(
        "  [VNK→GCL FULL CYCLE] packaged={} — persistimi lejohet vetëm pas verdiktit suprem",
        vnk_ready_for_full_cycle,
    );

    println!("  ✓ Lab respekton ligjet (intervention_guard)");
    println!("  ✓ Lab lë gjurmë në raw_cognitive_trace (audit i plotë)");
    println!("  ✓ Dështimi i Lab → VNK për Shadow");
    println!("  ✓ Lab NUK vendos, Lab NUK mban memory persistent");
}

/// Demonstron orkestratorin që lidh TË GJITHA modulet sipas arkitekturës.
#[cfg(feature = "dev_harness")]
fn run_orchestrator_demo() {
    use quantum_lim::orchestrator::{QuantumOrchestrator, PipelineConfig};
    use quantum_lim::reasoning::elimination::ElimCandidate;

    println!("\n{}", "═".repeat(76));
    println!("  ORKESTRATORI I PLOTË — të gjitha modulet v1.6 + NPRO/NPIM + Hardening");
    println!("{}", "═".repeat(76));

    // P2.3: konfiguro me sinjale LIM (demo: vlera reale nga një input shkencor).
    // Në rrjedhën reale këto vijnë nga lim_metric i run()-it.
    let config = PipelineConfig::new("science", "demo_session")
        .with_lim(0.72, 0.68);  // density + continuity (demo shkencor)
    let mut orch = QuantumOrchestrator::new(config);

    // Kandidatë (vijnë nga PRO në rrjedhën reale).
    let candidates = vec![
        ElimCandidate {
            provider: "PRO_REVERSE".to_string(),
            provider_domain: "science".to_string(),
            proposal_type: "causal".to_string(),
            quantum_score: 0.92, has_proposal: true, has_signature: true,
        },
        ElimCandidate {
            provider: "PRO_MIRROR".to_string(),
            provider_domain: "science".to_string(),
            proposal_type: "analogy".to_string(),
            quantum_score: 0.78, has_proposal: true, has_signature: true,
        },
        ElimCandidate {
            provider: "PRO_TRACE".to_string(),
            provider_domain: "science".to_string(),
            proposal_type: "path".to_string(),
            quantum_score: 0.65, has_proposal: true, has_signature: true,
        },
    ];

    let report = orch.run(candidates);

    println!("  [KUSHTETUTA] enforce_all (5 ligjet) → laws_passed={}", report.laws_passed);
    println!("  [DISIPLINA]  territory=science → {}", orch.territory_discipline());
    println!("  [PRESSURE]   gjendja → {:?}", report.pressure);
    println!("  [ELIMINIM]   {} kandidatë → {} mbijetuan (3-nivel)",
        report.candidates_in, report.candidates_out);
    println!("  [HARDENING]  qëndrueshmëria → {:.3} (konsensus+kalibrim+konflikt+stabilitet)",
        report.hardening_stability);
    println!("  [FUSION]     6D fusion score → {:.3}", report.fusion_score);
    println!("  [FRONTIER]   ceiling breached → {}", report.frontier_breached);
    println!("  [FAZA]       state machine → {:?}", report.phase_reached);
    println!("  [CONVERGENCE] best provider → {} (confidence {:.3})",
        report.final_provider, report.final_confidence);
    println!("  [TRACE]      {} gjurmë RAW_PENDING (Quantum s'vendos verdiktin)",
        report.traces_recorded);
    println!("  [MEMORY]     audit → ephemeral={} | persistent_rejections={}",
        report.memory_audit.0, report.memory_audit.1);

    // Quantum NUK mban memory — flush gjithçka.
    orch.end_cycle();
    println!("  [END_CYCLE]  memory flush → Quantum s'mban memory (ephemeral=0)");
}

// ─────────────────────────────────────────────────────────────────────────
// Handoff Quantum→Light për pd_light: eksporton fushat relevante të strukturës
// së PD në file (njëjta konventë si PA-export te Light). Format:
//   "session_id|concept|mass|coherence|genius|trl|probe_hint\n" — 7 fusha, append-only.
// pd_light (në Light) e lexon dhe e përkthen në gjuhë për UI — pa reasoning.
// Fail-open: dështimi i shkrimit s'ndalon rrjedhën (handoff UI, jo varësi
// kritike e reasoning-ut; rrjedha epistemike LIM→PD→LIM→PRO vazhdon pavarësisht).
// ─────────────────────────────────────────────────────────────────────────
// ─────────────────────────────────────────────────────────────────────────
// KOLAPSI I ARSYETIMIT — fragmentet e formulës të gdhendura (jo engine!)
//   read_primitive_context: PA → i₀ + (Xi,Yi) nga skedari i Light (fail-open)
//   mark_negative_spaces  : +(-Xi)+(-Yi) — LIM/NPRO shënon rrugët negative
//   collapse_reasoning    : XiYi → XY | Refuzim (kurrë shpikje)
// ─────────────────────────────────────────────────────────────────────────

/// Lexon (Xi, Yi) të koordinuara nga Light për këtë session.
/// Formati 5-fushësh: "sid|pa_hex|ts|xi:a,b|yi:c" — merret rreshti i FUNDIT
/// i session-it (konsistent me gjithë uratë). Mungesë/keqformim → split bosh.
fn read_primitive_context(session_id: &str) -> (Option<String>, PrimitiveSplit) {
    // `session_id` identifikon ciklin; `parent_i0` vjen vetëm nga PA real i
    // Light-it. Këto dy identitete nuk shkrihen më në një String të vetëm.
    let empty = || PrimitiveSplit { xi: Vec::new(), yi: Vec::new() };
    let content = match std::fs::read_to_string(handoff_path("light_pa_export.txt")) {
        Ok(c)  => c,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => return (None, empty()),
            other_kind => {
                quantum_lim::runtime_pulse::mark_notready(
                    quantum_lim::runtime_pulse::Stage::Split,
                );
                let _ = quantum_lim::lab_contracts::rrjedha::note(
                    &rrjedha_dir(), "QUANTUM", "main::read_primitive_context",
                    quantum_lim::lab_contracts::rrjedha::RrjedhaClass::DiskDenied,
                    &format!("leximi i PA dështoi: {:?} ({})", other_kind, e),
                );
                eprintln!("  [PA-SPLIT] ALARM: disku refuzoi leximin ({other_kind:?}) — i₀ mungon");
                return (None, empty());
            }
        },
    };
    let prefix = format!("{}|", session_id);
    let sealed_exists = content.lines()
        .filter(|line| line.starts_with(prefix.as_str()))
        .any(|line| line.contains("|c:"));
    let last = content.lines()
        .filter(|line| line.starts_with(prefix.as_str()))
        .last();
    let line = match last {
        Some(value) => value,
        None => return (None, empty()),
    };

    match (quantum_lim::lab_contracts::pa_wire::verify_line(line), sealed_exists) {
        (quantum_lim::lab_contracts::pa_wire::PaWireVerdict::SealedOk, _) => {}
        (quantum_lim::lab_contracts::pa_wire::PaWireVerdict::LegacyOk, false) => {
            let _ = quantum_lim::lab_contracts::rrjedha::note(
                &rrjedha_dir(), "QUANTUM", "main::read_primitive_context",
                quantum_lim::lab_contracts::rrjedha::RrjedhaClass::Other,
                &format!("LEGACY_PRANUAR kind=PA session={session_id} sealed_para=false"),
            );
        }
        (quantum_lim::lab_contracts::pa_wire::PaWireVerdict::LegacyOk, true) => {
            quantum_lim::runtime_pulse::mark_notready(
                quantum_lim::runtime_pulse::Stage::Split,
            );
            let _ = quantum_lim::lab_contracts::rrjedha::note(
                &rrjedha_dir(), "QUANTUM", "main::read_primitive_context",
                quantum_lim::lab_contracts::rrjedha::RrjedhaClass::TransportDowngrade,
                &format!("rresht legacy pas vule (session={session_id})"),
            );
            return (None, empty());
        }
        (quantum_lim::lab_contracts::pa_wire::PaWireVerdict::Corrupt(why), _) => {
            let class = match why.contains("përputhet") {
                true  => quantum_lim::lab_contracts::rrjedha::RrjedhaClass::TransportBitflip,
                false => quantum_lim::lab_contracts::rrjedha::RrjedhaClass::TransportTruncated,
            };
            quantum_lim::runtime_pulse::mark_notready(
                quantum_lim::runtime_pulse::Stage::Split,
            );
            let _ = quantum_lim::lab_contracts::rrjedha::note(
                &rrjedha_dir(), "QUANTUM", "main::read_primitive_context",
                class, &format!("{why} (session={session_id})"),
            );
            return (None, empty());
        }
    }

    let body = match line.rsplit_once("|c:") {
        Some((value, _)) => value,
        None => line,
    };
    let parts: Vec<&str> = body.split('|').collect();
    match parts.as_slice() {
        [_sid, pa_hex, _ts, xi_seg, yi_seg] => {
            let pa_id = match u64::from_str_radix(pa_hex.trim(), 16) {
                Ok(value) if value != 0 => value,
                _ => {
                    quantum_lim::runtime_pulse::mark_notready(
                        quantum_lim::runtime_pulse::Stage::Split,
                    );
                    return (None, empty());
                }
            };
            match PrimitiveSplit::from_wire(xi_seg, yi_seg) {
                Some(split) => {
                    quantum_lim::runtime_pulse::mark_ready(
                        quantum_lim::runtime_pulse::Stage::Split,
                    );
                    (Some(format!("i0-{pa_id:016x}")), split)
                }
                None => {
                    quantum_lim::runtime_pulse::mark_notready(
                        quantum_lim::runtime_pulse::Stage::Split,
                    );
                    (None, empty())
                }
            }
        }
        _ => {
            quantum_lim::runtime_pulse::mark_degraded(
                quantum_lim::runtime_pulse::Stage::Split,
            );
            (None, empty())
        }
    }
}

/// +(-Xi) + (-Yi): termat e hapësirave që bien në forbidden_paths të NPRO
/// (të përcaktuara nga sinjalet REALE të LIM) shënohen rrugë NEGATIVE.
/// Dija negative është ASET — ruhet, transportohet te Shadow, s'fshihet kurrë.
fn mark_negative_spaces(
    split: &PrimitiveSplit,
    weakness: &quantum_lim::npro::WeaknessSignal,
) -> Vec<NegativePath> {
    let mut out: Vec<NegativePath> = Vec::new();
    for term in split.xi.iter() {
        match weakness.is_forbidden(term) {
            true => out.push(NegativePath {
                term:   term.clone(),
                space:  "Xi",
                reason: "npro_forbidden(lim_signals)".to_string(),
            }),
            false => {}
        }
    }
    for term in split.yi.iter() {
        match weakness.is_forbidden(term) {
            true => out.push(NegativePath {
                term:   term.clone(),
                space:  "Yi",
                reason: "npro_forbidden(lim_signals)".to_string(),
            }),
            false => {}
        }
    }
    out
}

/// XiYi → XY: kërkimi i i~ (indicia e përbashkët) — çifti (x∈Xi, y∈Yi) me
/// bashkëprezencën më të fortë në fragmentet reale të LIM-it; masa = matje
/// (bashkëprezencë e normalizuar × epistemic_mass). Determinist i plotë:
/// barazi → rendi leksikografik (x, pastaj y). Hapësirë e zbrazët → Refuzim.
fn collapse_reasoning(
    frags: &[String],
    split: &PrimitiveSplit,
    epistemic_mass: f32,
) -> (CollapseOutcome, u32) {
    match split.has_material() {
        false => return (CollapseOutcome::Refused {
            reason: "pa material primitiv: Xi=∅ ∧ Yi=∅".to_string(),
        }, 0),
        true => {}
    }
    match split.xi.is_empty() || split.yi.is_empty() {
        true => return (CollapseOutcome::Refused {
            reason: "hapësirë e njëanshme: XiYi kërkon të dyja".to_string(),
        }, 0),
        false => {}
    }

    let mut best: Option<(String, String, u32)> = None;
    let mut survivors: u32 = 0;
    for x in split.xi.iter() {
        for y in split.yi.iter() {
            let co: u32 = frags.iter()
                .map(|fr| {
                    let fl = fr.to_lowercase();
                    match fl.contains(x.as_str()) && fl.contains(y.as_str()) {
                        true  => 1u32,
                        false => 0u32,
                    }
                })
                .sum();
            survivors += match co > 0 { true => 1, false => 0 };
            let better = match &best {
                None => co > 0,
                Some((bx, by, bco)) => match co.cmp(bco) {
                    std::cmp::Ordering::Greater => true,
                    std::cmp::Ordering::Less    => false,
                    std::cmp::Ordering::Equal   =>
                        match (x.as_str(), y.as_str()) < (bx.as_str(), by.as_str()) {
                            true  => true,
                            false => false,
                        },
                },
            };
            match better {
                true  => { best = Some((x.clone(), y.clone(), co)); }
                false => {}
            }
        }
    }

    match best {
        Some((x, y, co)) => {
            let denom = frags.len().max(1) as f32;
            let mass  = (co as f32 / denom) * epistemic_mass.max(0.0);
            (CollapseOutcome::Proposed { x, y, mass: mass.clamp(0.0, 1.0) }, survivors)
        }
        None => (CollapseOutcome::Refused {
            reason: "XiYi=∅: asnjë bashkëprezencë (x∈Xi, y∈Yi) në fragmente".to_string(),
        }, 0),
    }
}

/// Sanitizim wire: '|' e newline s'lejohen brenda fushave të URA 1.
fn wire_clean(s: &str) -> String {
    s.replace('|', "/").replace('\n', " ").replace('\r', " ")
}

/// Dosja e ledger-it të rrjedhave (dëshmi, kurrë autoritet): env i garantuar
/// nga boot-i sovran; temp ABSOLUT vetëm në teste. Zero if — vetëm match.
fn rrjedha_dir() -> String {
    match std::env::var("ESSMAI_HANDOFF_DIR") {
        Ok(d)  => d,
        Err(_) => std::env::temp_dir().to_string_lossy().into_owned(),
    }
}

/// Handoff i vazhdimit të mbyllur: PD mund ta ketë parapërgatitur pyetjen,
/// por ajo del nga Quantum vetëm PAS Verification Collapse të i₀ aktual dhe
/// vetëm kur lidhet byte-për-byte me receipt-in final të Shadow. Sentinel
/// "~pd_probe~" ruan faktin se s'ka paketë të kristalizuar, jo se është e
/// paverifikuar; Nura merr vetëm `PdNextI0` të liruar determinisht.
/// ZGJIDHJA e cwd-dependencës (#8): path-et e handoff zgjidhen nga
/// ESSMAI_HANDOFF_DIR (dosja e përbashkët e të tre binarëve) — BOOT SOVRAN:
/// mungesa e env = FATAL exit(1) në nisje; ZERO fallback relativ (v1.3.1).
fn handoff_path(file: &str) -> String {
    match std::env::var("ESSMAI_HANDOFF_DIR") {
        Ok(dir) => {
            // BOOT SOVRAN: dosja krijohet po të mungojë dhe kanonikalizohet
            // në ABSOLUTE — zero varësi nga cwd, gjurmueshmëri e përsëritshme.
            match std::fs::create_dir_all(&dir) {
                Ok(())  => {}
                Err(e)  => {
                    eprintln!("[BOOT] FATAL (fail-closed): ESSMAI_HANDOFF_DIR='{dir}' s'u krijua dot ({e}).");
                    std::process::exit(1);
                }
            }
            let abs = match std::fs::canonicalize(&dir) {
                Ok(p)  => p,
                Err(e) => {
                    eprintln!("[BOOT] FATAL (fail-closed): ESSMAI_HANDOFF_DIR='{dir}' s'u kanonikalizua ({e}).");
                    std::process::exit(1);
                }
            };
            let joined = abs.join(file);
            joined.to_string_lossy().into_owned()
        }
        Err(_) => {
            // ZERO IMPROVIZIM: pa rrugë të deklaruar s'ka nisje — rrugët
            // relative (cwd) dëmtojnë determinizmin dhe gjurmën e auditimit.
            eprintln!("[BOOT] FATAL (fail-closed): ESSMAI_HANDOFF_DIR mungon.");
            eprintln!("[BOOT] Cakto dosjen e përbashkët të urave, p.sh.:");
            eprintln!("[BOOT]   Linux/macOS:  export ESSMAI_HANDOFF_DIR=/opt/essmai/handoff");
            eprintln!("[BOOT]   Windows:      $env:ESSMAI_HANDOFF_DIR='C:\\essmai\\handoff'");
            std::process::exit(1);
        }
    }
}

fn export_pd_probe(
    session_id: &str,
    closure: &quantum_lim::progressive_debatic::PdContinuumClosure,
    receipt: &shadow_contracts::VerificationReceiptWire,
    verdict: &shadow_contracts::ShadowVerdictWire,
) {
    export_pd_verified_line(
        session_id,
        "~pd_probe~",
        0,
        0,
        0,
        0,
        Some(closure),
        receipt,
        verdict,
        "main::export_pd_probe",
    );
}

fn export_pd_handoff(
    session_id: &str,
    pkg: &quantum_lim::progressive_debatic::PdCognitivePackage,
    closure: Option<&quantum_lim::progressive_debatic::PdContinuumClosure>,
    receipt: &shadow_contracts::VerificationReceiptWire,
    verdict: &shadow_contracts::ShadowVerdictWire,
) {
    export_pd_verified_line(
        session_id,
        &pkg.dominant_concept,
        pkg.accumulated_mass,
        pkg.structural_coherence,
        pkg.genius_signal.genius_score,
        pkg.estimated_trl,
        closure,
        receipt,
        verdict,
        "main::export_pd_handoff",
    );

    // Dimensioni REASONING shkruhet vetëm pasi paketa u mbyll nga Shadow.
    let tbody = format!("{}|rsn|{}", session_id, pkg.estimated_trl);
    let tline = quantum_lim::lab_contracts::pa_wire::seal_body(&tbody);
    use std::io::Write as _;
    match std::fs::OpenOptions::new().create(true).append(true)
        .open(handoff_path("trl_vector.txt")) {
        Ok(mut tf) => match tf.write_all(tline.as_bytes()).and_then(|_| tf.flush()) {
            Ok(()) => {}
            Err(e) => eprintln!("  [TRL_VEKTOR] ALARM: dimensioni rsn s'u shkrua ({e})"),
        },
        Err(e) => eprintln!("  [TRL_VEKTOR] ALARM: trl_vector.txt s'u hap ({e})"),
    }
}

#[allow(clippy::too_many_arguments)]
fn export_pd_verified_line(
    session_id: &str,
    concept: &str,
    mass: u32,
    coherence: u32,
    genius: u32,
    trl: u8,
    closure: Option<&quantum_lim::progressive_debatic::PdContinuumClosure>,
    receipt: &shadow_contracts::VerificationReceiptWire,
    verdict: &shadow_contracts::ShadowVerdictWire,
    source: &'static str,
) {
    // v1.6.2: receipt-i SHA-256, Vula e Gjallë ×3 dhe formula
    // `PD output + iZ(Besim) → next i₀` udhëtojnë në një dorëzim të vetëm.
    let receipt_trust_identity = match quantum_lim::living_trust_contract::parse_hex_32(
        &receipt.living_trust_sha256,
    ) {
        Some(value) => value,
        None => {
            eprintln!("  [PD→LIGHT] ALARM: receipt-i nuk mban SHA-256 kanonik të Besimit");
            return;
        }
    };
    let receipt_expected = quantum_lim::lab_contracts::verification_receipt::receipt_id(
        &receipt.session_id,
        &receipt.parent_i0,
        receipt.primitive_anchor,
        receipt.xy_digest,
        receipt.pd_binding_digest,
        receipt.pd_continuum_activation_digest,
        &receipt_trust_identity,
        receipt.y_verdict,
        receipt.x_verdict,
        receipt.generation,
        receipt.seal,
    );
    let receipt_valid = receipt.session_id == session_id
        && !receipt.parent_i0.is_empty()
        && receipt.primitive_anchor != 0
        && receipt.xy_digest != 0
        && receipt.pd_binding_digest != 0
        && receipt.pd_continuum_activation_digest != 0
        && receipt.y_verdict == verdict.verified
        && receipt.x_verdict == verdict.primitive
        && quantum_lim::lab_contracts::verification_receipt::constitutional_pair(
            receipt.y_verdict, receipt.x_verdict,
        )
        && receipt.receipt_id == receipt_expected;
    match receipt_valid {
        true => {}
        false => {
            eprintln!("  [PD→LIGHT] ALARM: verification receipt nuk lidhet me i₀/XY/verdict-in aktual");
            return;
        }
    }

    let trust_identity = match quantum_lim::living_trust_contract::parse_hex_32(
        &verdict.living_trust_sha256,
    ) {
        Some(value) => value,
        None => {
            eprintln!("  [PD→LIGHT] ALARM: Vula e Gjallë nuk ka SHA-256 kanonik");
            return;
        }
    };
    let project_context_sha256 = match quantum_lim::gcl_project_contract::parse_sha256(
        &verdict.project_context_sha256,
    ) {
        Some(value) => value,
        None => {
            eprintln!("  [PD→LIGHT] ALARM: project context SHA-256 jo kanonik");
            return;
        }
    };
    let project_evidence_sha256 = match quantum_lim::gcl_project_contract::parse_sha256(
        &verdict.project_evidence_sha256,
    ) {
        Some(value) => value,
        None => {
            eprintln!("  [PD→LIGHT] ALARM: project evidence SHA-256 jo kanonik");
            return;
        }
    };
    let trust_proof = quantum_lim::living_trust_contract::LivingTrustProof {
        action_state: verdict.living_trust_action_state,
        action_mask: verdict.living_trust_action_mask,
        required_action_mask: verdict.living_trust_required_action_mask,
        verified: verdict.verified,
        primitive: verdict.primitive,
        knowledge_band: verdict.knowledge_band,
        lgc_law: verdict.lgc_law,
        system_laws_seal: verdict.living_trust_system_laws_seal,
        scientific_project_sha256: project_evidence_sha256,
        scientific_project_verdict_sha256:
            quantum_lim::gcl_project_contract::verdict_sha256_or_zero(
                quantum_lim::gcl_project_contract::ProjectVerdictMaterial {
                    project_id: verdict.project_id,
                    project_status: verdict.project_status,
                    project_context_sha256,
                    project_evidence_sha256,
                    novel_factualized: verdict.novel_factualized,
                    novel_trl_level: verdict.novel_trl_level,
                    novel_proof_score: verdict.novel_proof_score,
                    novel_rejection: verdict.novel_rejection,
                },
            ),
        sovereign_flags: verdict.living_trust_sovereign_flags,
    };
    let trust = match quantum_lim::living_trust_contract::compute_with_intensity(
        trust_proof,
        verdict.living_trust_intensity,
    ) {
        Some(value) => value,
        None => {
            eprintln!("  [PD→LIGHT] ALARM: prova e Besimit nuk është kushtetuese");
            return;
        }
    };
    let trust_valid = trust.identity_sha256 == trust_identity
        && receipt_trust_identity == trust_identity
        && trust.kind == verdict.living_trust_kind
        && trust.sovereign_value == verdict.living_trust_sovereign_value;
    match trust_valid {
        true => {}
        false => {
            eprintln!("  [PD→LIGHT] ALARM: prova dhe vula e Besimit nuk përputhen");
            return;
        }
    }
    let living_trust_digest = trust.identity_digest();
    match receipt.living_trust_digest == living_trust_digest {
        true => {}
        false => {
            eprintln!("  [PD→LIGHT] ALARM: receipt-i nuk është lidhur me Vulën e Gjallë");
            return;
        }
    }

    let (question, next_i0_id, basis, question_increment_digest,
         pd_output_id, pd_output_digest, iz_id, iz_digest, completion_digest) =
        match closure {
            Some(c) => {
                let n = &c.next_i0;
                let next_bound = n.session_id == receipt.session_id
                    && n.parent_i0 == receipt.parent_i0
                    && n.y_verdict == receipt.y_verdict
                    && n.x_verdict == receipt.x_verdict
                    && n.verification_seal == receipt.seal
                    && n.verification_generation == receipt.generation
                    && n.verification_receipt_id == receipt.receipt_id
                    && n.primitive_anchor == receipt.primitive_anchor
                    && n.xy_digest == receipt.xy_digest
                    && n.pd_binding_digest == receipt.pd_binding_digest
                    && n.continuum_activation_digest
                        == receipt.pd_continuum_activation_digest
                    && n.living_trust_sha256 == trust.identity_sha256
                    && n.living_trust_intensity == trust.intensity
                    && n.living_trust_kind == trust.kind
                    && n.living_trust_sovereign_value == trust.sovereign_value
                    && n.living_trust_digest == living_trust_digest
                    && c.iz.living_trust_digest == living_trust_digest
                    && c.contract.living_trust_digest == living_trust_digest
                    && c.contract.closes_formula(&receipt.receipt_id)
                    && c.contract.completion_digest == n.continuum_completion_digest
                    && c.output.output_digest == n.pd_output_digest
                    && c.iz.iz_digest == n.iz_digest;
                match next_bound {
                    true => (
                        wire_clean(&n.question),
                        wire_clean(&n.next_i0_id),
                        n.basis.label().to_string(),
                        n.question_increment_digest,
                        wire_clean(&n.pd_output_id),
                        n.pd_output_digest,
                        wire_clean(&n.iz_id),
                        n.iz_digest,
                        n.continuum_completion_digest,
                    ),
                    false => {
                        eprintln!("  [PD→LIGHT] ALARM: output+iZ+Besim nuk janë fëmijë të receipt-it final");
                        return;
                    }
                }
            }
            None => return,
        };

    // 45 fusha trupi + CRC (46 total): context + evidence + status Novel nën GCL.
    let body = format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{:016x}|{}|{}|{}|{:016x}|{:016x}|{:016x}|{:016x}|{:016x}|{}|{:016x}|{}|{:016x}|{:016x}|{:016x}|{:016x}|{:016x}|{}|{:016x}|{}|{}|{}|{}|{}|{}|{}|{:08x}|{:016x}|{:08x}|{:08x}|{}|{}|{}|{:016x}",
        wire_clean(session_id),
        wire_clean(concept),
        mass,
        coherence,
        genius,
        trl,
        question,
        wire_clean(&receipt.parent_i0),
        next_i0_id,
        receipt.y_verdict,
        receipt.x_verdict,
        receipt.seal,
        receipt.generation,
        wire_clean(&receipt.receipt_id),
        basis,
        receipt.primitive_anchor,
        receipt.xy_digest,
        receipt.pd_binding_digest,
        receipt.pd_continuum_activation_digest,
        question_increment_digest,
        pd_output_id,
        pd_output_digest,
        iz_id,
        iz_digest,
        completion_digest,
        verdict.living_trust_action_state,
        verdict.living_trust_action_mask,
        verdict.living_trust_required_action_mask,
        wire_clean(&verdict.project_evidence_sha256),
        verdict.project_id,
        wire_clean(&verdict.project_context_sha256),
        verdict.project_status,
        verdict.novel_factualized as u8,
        verdict.novel_trl_level,
        verdict.novel_proof_score,
        verdict.novel_rejection,
        verdict.knowledge_band,
        verdict.lgc_law,
        verdict.living_trust_system_laws_seal,
        verdict.living_trust_sovereign_flags,
        verdict.living_trust_sovereign_value,
        wire_clean(&verdict.living_trust_sha256),
        verdict.living_trust_intensity,
        verdict.living_trust_kind,
        living_trust_digest,
    );

    let line = match quantum_lim::lab_contracts::pa_wire::seal_body_verified(
        &body,
        &[6, 7],
    ) {
        Ok(l) => l,
        Err(pse) => {
            let _ = quantum_lim::lab_contracts::rrjedha::note(
                &rrjedha_dir(),
                "QUANTUM",
                source,
                quantum_lim::lab_contracts::rrjedha::RrjedhaClass::WireInvariant,
                pse,
            );
            eprintln!("  [PD→LIGHT] ALARM: handoff i verifikuar u ndal NË BURIM ({pse})");
            return;
        }
    };

    use std::io::Write as _;
    match std::fs::OpenOptions::new().create(true).append(true)
        .open(handoff_path("quantum_pd_export.txt")) {
        Ok(mut f) => match f.write_all(line.as_bytes()).and_then(|_| f.flush()) {
            Ok(()) => {}
            Err(e) => eprintln!("  [PD→LIGHT] ALARM: handoff s'u shkrua ({e})"),
        },
        Err(e) => eprintln!("  [PD→LIGHT] ALARM: quantum_pd_export.txt s'u hap ({e})"),
    }
}

