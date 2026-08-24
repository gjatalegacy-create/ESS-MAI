//! New orchestration harness for the bounded POC-003 experiment.

use crate::alnur_karina_athar::{
    AlnurKarinaAthar, CoordinateCollapseEvidence, PrimitiveAnchorEvidence,
    PrimitiveSplitEvidence, TraceEvidence, PRIMITIVE_SEAL_MASKED,
};
use crate::besa_nlight::BesaNlight;
use crate::lab_contracts::{CollapsePhase, GjataCollapseLaw};
use quantum_lim::lab_contracts::PrimitiveSplit;
use quantum_lim::ultimatum_collapse_law::{
    AshtScopeEvidence, UclEvent, UltimatumCollapseLaw,
};
use shadow_contracts::{
    KnowledgeSelectionRequestWire, SelectionPurpose, SelectionRequester,
    SELECTION_MAX_CANDIDATES,
};
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

const FIXED_INPUT: &str =
    "deterministic reasoning produces verifiable primitive knowledge without hallucination";
const TRACE_ID: &str = "POC003_TRACE";
const PRIMITIVE_ANCHOR: u64 = 0x0000_0000_0000_0007;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExperimentMode {
    EmptyColdStart,
    ExactPairPositiveControl,
}

impl ExperimentMode {
    fn environment_value(self) -> &'static str {
        match self {
            Self::EmptyColdStart => "EMPTY",
            Self::ExactPairPositiveControl => "EXACT_PAIR_CONTROL",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::EmptyColdStart => "EMPTY_COLD_START",
            Self::ExactPairPositiveControl => "EXACT_PAIR_POSITIVE_CONTROL",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExperimentReport {
    pub mode: ExperimentMode,
    pub gcl_parent_authority: bool,
    pub shadow_selection_processes: usize,
    pub besa_positive_count: usize,
    pub besa_negative_count: usize,
    pub besa_empty_selection_accepted: bool,
    pub asht_succeeded: bool,
    pub asht_error: Option<String>,
    pub post_asht_probe_reached: bool,
    pub production_shadow_commit_executed: bool,
    pub classification: &'static str,
}

impl std::fmt::Display for ExperimentReport {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(formatter, "artifact=ESS-MAI-SYSTEM-POC-003")?;
        writeln!(formatter, "mode={}", self.mode.label())?;
        writeln!(formatter, "gcl_parent_authority={}", self.gcl_parent_authority)?;
        writeln!(
            formatter,
            "shadow_selection_processes={}",
            self.shadow_selection_processes
        )?;
        writeln!(
            formatter,
            "besa_selection=positive:{} negative:{} accepted_empty:{}",
            self.besa_positive_count,
            self.besa_negative_count,
            self.besa_empty_selection_accepted
        )?;
        writeln!(formatter, "asht_succeeded={}", self.asht_succeeded)?;
        writeln!(
            formatter,
            "asht_error={}",
            self.asht_error.as_deref().unwrap_or("NONE")
        )?;
        writeln!(
            formatter,
            "post_asht_probe_reached={}",
            self.post_asht_probe_reached
        )?;
        writeln!(
            formatter,
            "production_shadow_commit_executed={}",
            self.production_shadow_commit_executed
        )?;
        write!(formatter, "classification={}", self.classification)
    }
}

struct EnvironmentGuard {
    prior_handoff: Option<OsString>,
    prior_shadow: Option<OsString>,
    prior_mode: Option<OsString>,
}

impl EnvironmentGuard {
    fn install(handoff: &Path, shadow_binary: &Path, mode: ExperimentMode) -> Self {
        let guard = Self {
            prior_handoff: std::env::var_os("ESSMAI_HANDOFF_DIR"),
            prior_shadow: std::env::var_os("ESSMAI_SHADOW_BIN"),
            prior_mode: std::env::var_os("ESSMAI_POC_SELECTION_MODE"),
        };
        std::env::set_var("ESSMAI_HANDOFF_DIR", handoff);
        std::env::set_var("ESSMAI_SHADOW_BIN", shadow_binary);
        std::env::set_var("ESSMAI_POC_SELECTION_MODE", mode.environment_value());
        guard
    }

    fn restore(name: &str, value: Option<&OsString>) {
        match value {
            Some(value) => std::env::set_var(name, value),
            None => std::env::remove_var(name),
        }
    }
}

impl Drop for EnvironmentGuard {
    fn drop(&mut self) {
        Self::restore("ESSMAI_HANDOFF_DIR", self.prior_handoff.as_ref());
        Self::restore("ESSMAI_SHADOW_BIN", self.prior_shadow.as_ref());
        Self::restore("ESSMAI_POC_SELECTION_MODE", self.prior_mode.as_ref());
    }
}

fn runtime_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

fn fixed_split() -> PrimitiveSplit {
    PrimitiveSplit {
        xi: vec!["deterministic".to_string()],
        yi: [
            "hallucination",
            "knowledge",
            "primitive",
            "produces",
            "reasoning",
            "verifiable",
            "without",
        ]
        .into_iter()
        .map(str::to_string)
        .collect(),
    }
}

fn digest_terms(domain: &str, terms: &[String]) -> String {
    let mut canonical = Vec::new();
    for term in terms {
        canonical.extend_from_slice(&(term.len() as u64).to_le_bytes());
        canonical.extend_from_slice(term.as_bytes());
    }
    crate::lgc_algorithm::compute_domain_sha(domain, &[canonical.as_slice()])
}

fn split_sha256(split: &PrimitiveSplit) -> String {
    let xi_sha = digest_terms("ESSMAI/LGC/XI/V2", &split.xi);
    let yi_sha = digest_terms("ESSMAI/LGC/YI/V2", &split.yi);
    crate::lgc_algorithm::compute_domain_sha(
        "ESSMAI/LGC/SPLIT/V2",
        &[
            xi_sha.as_bytes(),
            yi_sha.as_bytes(),
            split.to_wire().as_bytes(),
        ],
    )
}

fn write_besa_companion(
    handoff: &Path,
    parent_i0: &str,
    split_sha256: &str,
    besa: &crate::alnur_karina_athar::BesaAttestation,
) -> Result<PathBuf, String> {
    let body = format!(
        "{}|{}|{:016x}|{}|{}|{}|{}|{}|{}",
        TRACE_ID,
        parent_i0,
        PRIMITIVE_ANCHOR,
        besa.trace_id,
        split_sha256,
        besa.hold_id,
        besa.hold_sha256,
        besa.selection_sha256,
        besa.enrichment_sha256,
    );
    let line = quantum_lim::lab_contracts::pa_wire::seal_body_verified(&body, &[])
        .map_err(|reason| format!("Besa companion source verification failed: {reason}"))?;
    let path = handoff.join("light_besa_export.txt");
    std::fs::write(&path, line)
        .map_err(|error| format!("Besa companion write failed: {error}"))?;
    Ok(path)
}

fn selection_hold_count(handoff: &Path) -> usize {
    std::fs::read_dir(handoff.join("selection_holds"))
        .ok()
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().and_then(|value| value.to_str()) == Some("hold"))
        .count()
}

pub fn authority_wrong_phase_is_rejected() -> bool {
    let light_rejected =
        AlnurKarinaAthar::commission(GjataCollapseLaw::issue(CollapsePhase::Reasoning)).is_err();
    let quantum_rejected =
        UltimatumCollapseLaw::commission(GjataCollapseLaw::issue(CollapsePhase::Coordination))
            .is_err();
    light_rejected && quantum_rejected
}

pub fn run_experiment(
    mode: ExperimentMode,
    shadow_binary: &Path,
    handoff: &Path,
) -> Result<ExperimentReport, String> {
    let _exclusive = runtime_lock()
        .lock()
        .map_err(|error| format!("POC runtime lock poisoned: {error}"))?;
    match (shadow_binary.is_absolute(), shadow_binary.is_file()) {
        (true, true) => {}
        (false, _) => return Err("Shadow selector path must be absolute".to_string()),
        (_, false) => return Err("Shadow selector binary is missing".to_string()),
    }
    std::fs::create_dir_all(handoff)
        .map_err(|error| format!("handoff directory creation failed: {error}"))?;
    let _environment = EnvironmentGuard::install(handoff, shadow_binary, mode);

    let split = fixed_split();
    let split_digest = split_sha256(&split);
    let parent_i0 = shadow_contracts::sha256_hex(&[FIXED_INPUT.as_bytes()]);
    let coordination = GjataCollapseLaw::issue(CollapsePhase::Coordination);
    let gcl_parent_authority = GjataCollapseLaw::verify(
        &coordination,
        CollapsePhase::Coordination,
    ) && authority_wrong_phase_is_rejected();
    let mut light = AlnurKarinaAthar::commission(coordination)
        .map_err(|breach| format!("Light commission failed: {}", breach.text))?;
    light
        .record_trace(&TraceEvidence {
            trace_id: TRACE_ID.to_string(),
            raw_len: FIXED_INPUT.len(),
            input_sha256: parent_i0.clone(),
        })
        .map_err(|breach| format!("Light trace failed: {}", breach.text))?;
    light
        .record_anchor(&PrimitiveAnchorEvidence {
            trace_id: TRACE_ID.to_string(),
            primitive_anchor: format!("{PRIMITIVE_ANCHOR:016x}"),
            input_sha256: parent_i0.clone(),
            primitive_flags: PRIMITIVE_SEAL_MASKED,
        })
        .map_err(|breach| format!("Light anchor failed: {}", breach.text))?;
    light
        .record_split(&PrimitiveSplitEvidence {
            trace_id: TRACE_ID.to_string(),
            split_sha256: split_digest.clone(),
            source_terms: split.xi.len().saturating_add(split.yi.len()),
            xi_len: split.xi.len(),
            yi_len: split.yi.len(),
            disjoint: true,
            complete: true,
            has_material: split.has_material(),
        })
        .map_err(|breach| format!("Light split failed: {}", breach.text))?;
    let besa_command = light
        .command_besa(&coordination)
        .map_err(|breach| format!("Besa command failed: {}", breach.text))?;
    let besa_request = KnowledgeSelectionRequestWire {
        version: shadow_contracts::PROTOCOL_VERSION,
        requester: SelectionRequester::BesaNlight,
        purpose: SelectionPurpose::XiYiStatistics,
        session_id: TRACE_ID.to_string(),
        parent_i0: parent_i0.clone(),
        primitive_anchor: PRIMITIVE_ANCHOR,
        trace_id: TRACE_ID.to_string(),
        split_sha256: split_digest.clone(),
        xi: split.xi.clone(),
        yi: split.yi.clone(),
        upstream_hold_sha256: None,
        evidence_sha256: shadow_contracts::sha256_hex(&[
            b"ESSMAI/POC003/BESA-REQUEST",
            parent_i0.as_bytes(),
            split_digest.as_bytes(),
        ]),
        max_positive: SELECTION_MAX_CANDIDATES,
        max_negative: SELECTION_MAX_CANDIDATES,
    };
    let besa = BesaNlight::from_command(besa_command)
        .commission(&coordination)?
        .request_xi_yi_statistics(besa_request)?
        .weave_context()?
        .attest()?;
    light
        .record_collapse(
            &CoordinateCollapseEvidence {
                split_sha256: split_digest.clone(),
                binding_sha256: shadow_contracts::sha256_hex(&[
                    b"ESSMAI/POC003/COORDINATE-COLLAPSE",
                    split_digest.as_bytes(),
                ]),
                before: 64,
                after: 7,
            },
            &besa,
        )
        .map_err(|breach| format!("Light collapse failed: {}", breach.text))?;

    let companion_path = write_besa_companion(handoff, &parent_i0, &split_digest, &besa)?;
    let companion = crate::asht_quantum::read_besa_companion(
        &companion_path,
        TRACE_ID,
        &parent_i0,
        PRIMITIVE_ANCHOR,
        &split,
    )?;
    let reasoning = GjataCollapseLaw::issue(CollapsePhase::Reasoning);
    let mut ucl = UltimatumCollapseLaw::commission(reasoning)
        .map_err(|breach| format!("Quantum commission failed: {}", breach.text))?;
    ucl.advance(UclEvent::LoadXiYi)
        .map_err(|breach| format!("Quantum Xi/Yi load failed: {}", breach.text))?;
    let asht_command = ucl
        .command_asht(&AshtScopeEvidence {
            trace_id: TRACE_ID.to_string(),
            split_sha256: split_digest.clone(),
            besa_hold_sha256: companion.hold_sha256.clone(),
        })
        .map_err(|breach| format!("Asht command failed: {}", breach.text))?;
    let asht_request = KnowledgeSelectionRequestWire {
        version: shadow_contracts::PROTOCOL_VERSION,
        requester: SelectionRequester::AshtQuantum,
        purpose: SelectionPurpose::EvidenceCandidates,
        session_id: TRACE_ID.to_string(),
        parent_i0: parent_i0.clone(),
        primitive_anchor: PRIMITIVE_ANCHOR,
        trace_id: TRACE_ID.to_string(),
        split_sha256: split_digest.clone(),
        xi: split.xi.clone(),
        yi: split.yi.clone(),
        upstream_hold_sha256: Some(companion.hold_sha256.clone()),
        evidence_sha256: shadow_contracts::sha256_hex(&[
            b"ESSMAI/ASHT/REQUEST/V178",
            companion.attestation_sha256().as_bytes(),
            parent_i0.as_bytes(),
        ]),
        max_positive: SELECTION_MAX_CANDIDATES,
        max_negative: SELECTION_MAX_CANDIDATES,
    };
    let asht_result = crate::asht_quantum::AshtQuantum::from_command(asht_command)
        .commission()
        .and_then(|asht| asht.request_evidence_candidates(asht_request))
        .and_then(crate::asht_quantum::AshtQuantum::attest);
    let probe_path = handoff.join("post_asht_final_shadow_probe.txt");
    let (asht_succeeded, asht_error) = match (mode, asht_result) {
        (ExperimentMode::EmptyColdStart, Err(reason))
            if reason == "request-bound relevance found no exact positive candidate" =>
        {
            (false, Some(reason))
        }
        (ExperimentMode::EmptyColdStart, Err(reason)) => {
            return Err(format!("unexpected empty-state Asht failure: {reason}"))
        }
        (ExperimentMode::EmptyColdStart, Ok(_)) => {
            return Err("empty-state Asht unexpectedly succeeded".to_string())
        }
        (ExperimentMode::ExactPairPositiveControl, Ok(_)) => {
            std::fs::write(&probe_path, b"POST_ASHT_REACHED; NOT_A_PRODUCTION_COMMIT\n")
                .map_err(|error| format!("post-Asht probe write failed: {error}"))?;
            (true, None)
        }
        (ExperimentMode::ExactPairPositiveControl, Err(reason)) => {
            return Err(format!("exact-pair positive control failed: {reason}"))
        }
    };
    let post_asht_probe_reached = probe_path.exists();
    let classification = match mode {
        ExperimentMode::EmptyColdStart => "COLD_START_REACHABILITY_GAP_REPRODUCED",
        ExperimentMode::ExactPairPositiveControl => "POST_ASHT_REACHABILITY_CONTROL_PASS",
    };
    Ok(ExperimentReport {
        mode,
        gcl_parent_authority,
        shadow_selection_processes: selection_hold_count(handoff),
        besa_positive_count: besa.positive_count,
        besa_negative_count: besa.negative_count,
        besa_empty_selection_accepted: besa.positive_count == 0 && besa.negative_count == 0,
        asht_succeeded,
        asht_error,
        post_asht_probe_reached,
        production_shadow_commit_executed: false,
        classification,
    })
}

