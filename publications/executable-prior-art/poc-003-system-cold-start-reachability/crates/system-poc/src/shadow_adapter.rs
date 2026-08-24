//! New surgical Shadow process adapter.
//!
//! It exposes only `--selection-once`. The default path constructs a store with
//! no writer and executes the exact production bounded-projection function.
//! `EXACT_PAIR_CONTROL` is an explicit experimental control and is never used
//! as a production seed or as evidence of a genesis solution.

use crate::selection_hold::SelectionHoldStore;
use shadow_contracts::{
    CandidateStatisticWire, KnowledgeSelectionRequestWire, KnowledgeSelectionResponseWire,
};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const MODE_ENV: &str = "ESSMAI_POC_SELECTION_MODE";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SelectorMode {
    Empty,
    ExactPairControl,
}

fn selector_mode() -> Result<SelectorMode, String> {
    match std::env::var(MODE_ENV).as_deref() {
        Ok("EMPTY") | Err(_) => Ok(SelectorMode::Empty),
        Ok("EXACT_PAIR_CONTROL") => Ok(SelectorMode::ExactPairControl),
        Ok(other) => Err(format!("unsupported {MODE_ENV} value: {other}")),
    }
}

fn now_ns() -> Result<u64, String> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos().min(u128::from(u64::MAX)) as u64)
        .map_err(|error| format!("Shadow selector clock failed: {error}"))
}

fn exact_pair_control(
    request: &KnowledgeSelectionRequestWire,
) -> Result<(Vec<CandidateStatisticWire>, Vec<CandidateStatisticWire>), String> {
    let positive_term = request
        .xi
        .first()
        .ok_or_else(|| "positive control requires an Xi term".to_string())?;
    let negative_term = request
        .yi
        .first()
        .ok_or_else(|| "positive control requires a Yi term".to_string())?;
    let positive = CandidateStatisticWire {
        candidate_id: 1,
        polarity: 1,
        domain: "primitive".to_string(),
        term_sha256: shadow_contracts::sha256_hex(&[
            b"ESSMAI/SELECTION/TERM/PRIMITIVE",
            positive_term.as_bytes(),
        ]),
        source_sha256: shadow_contracts::sha256_hex(&[
            b"ESSMAI/POC003/POSITIVE-CONTROL",
            positive_term.as_bytes(),
        ]),
        security_y: 1,
        trust_x: 1,
        confidence_fixed: 8_000,
        frequency: 1,
        domain_count: 1,
        legacy_score_fixed: 8_000,
    };
    let negative = CandidateStatisticWire {
        candidate_id: 2,
        polarity: 0,
        domain: "negative-control".to_string(),
        term_sha256: shadow_contracts::sha256_hex(&[
            b"ESSMAI/SELECTION/TERM/NEGATIVE",
            negative_term.as_bytes(),
        ]),
        source_sha256: shadow_contracts::sha256_hex(&[
            b"ESSMAI/POC003/NEGATIVE-CONTROL",
            negative_term.as_bytes(),
        ]),
        security_y: 0,
        trust_x: 0,
        confidence_fixed: 7_000,
        frequency: 1,
        domain_count: 1,
        legacy_score_fixed: 7_000,
    };
    Ok((vec![positive], vec![negative]))
}

fn candidates(
    request: &KnowledgeSelectionRequestWire,
) -> Result<(Vec<CandidateStatisticWire>, Vec<CandidateStatisticWire>), String> {
    match selector_mode()? {
        SelectorMode::Empty => crate::shadow_projection::project_generation_zero(
            request.max_positive,
            request.max_negative,
        ),
        SelectorMode::ExactPairControl => exact_pair_control(request),
    }
}

fn build_response(
    request: &KnowledgeSelectionRequestWire,
) -> Result<KnowledgeSelectionResponseWire, String> {
    match request.shape_is_valid() {
        true => {}
        false => return Err("selection request shape rejected".to_string()),
    }
    let (positive, negative) = candidates(request)?;
    let request_sha256 = crate::besa_nlight::selection_request_digest(request);
    let mut response = KnowledgeSelectionResponseWire {
        version: shadow_contracts::PROTOCOL_VERSION,
        requester: request.requester,
        session_id: request.session_id.clone(),
        parent_i0: request.parent_i0.clone(),
        primitive_anchor: request.primitive_anchor,
        split_sha256: request.split_sha256.clone(),
        upstream_hold_sha256: request.upstream_hold_sha256.clone(),
        request_sha256,
        selection_sha256: String::new(),
        hold_id: String::new(),
        hold_sha256: String::new(),
        positive,
        negative,
        law_seal: crate::lab_contracts::gjata_collapse_law::law_seal(),
        system_laws_seal: crate::lab_contracts::system_laws_seal(),
        expires_at_ns: now_ns()?
            .checked_add(15 * 60 * 1_000_000_000)
            .ok_or_else(|| "selection expiry overflow".to_string())?,
    };
    response.selection_sha256 = crate::besa_nlight::selection_response_digest(&response);
    let hold_material = shadow_contracts::sha256_hex(&[
        b"ESSMAI/SELECTION/HOLD-ID/V178",
        response.request_sha256.as_bytes(),
        response.selection_sha256.as_bytes(),
    ]);
    let prefix = hold_material
        .get(..32)
        .ok_or_else(|| "selection hold id derivation failed".to_string())?;
    response.hold_id = format!("h-{prefix}");
    response.hold_sha256 = SelectionHoldStore::recompute_hold_sha256(&response);
    match response.shape_is_valid_for(request) {
        true => Ok(response),
        false => Err("selection response failed internal validation".to_string()),
    }
}

fn handoff_root() -> Result<PathBuf, String> {
    std::env::var("ESSMAI_HANDOFF_DIR")
        .map(PathBuf::from)
        .map_err(|error| format!("ESSMAI_HANDOFF_DIR missing for selection: {error}"))
}

fn atomic_write(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let parent = path
        .parent()
        .ok_or_else(|| "selection response has no parent directory".to_string())?;
    std::fs::create_dir_all(parent)
        .map_err(|error| format!("selection response directory failed: {error}"))?;
    let temporary = parent.join(format!(
        ".poc003-selection-{}.tmp",
        std::process::id()
    ));
    std::fs::write(&temporary, bytes)
        .map_err(|error| format!("selection response temporary write failed: {error}"))?;
    std::fs::rename(&temporary, path)
        .map_err(|error| format!("selection response atomic publish failed: {error}"))
}

pub fn run_shadow_selector_once(request_path: &Path, response_path: &Path) -> Result<(), String> {
    let request_bytes = std::fs::read(request_path)
        .map_err(|error| format!("selection request read failed: {error}"))?;
    match request_bytes.len() <= shadow_contracts::MAX_FRAME_BYTES {
        true => {}
        false => return Err("selection request exceeds frame limit".to_string()),
    }
    let request = shadow_contracts::decode_selection_request(&request_bytes)
        .map_err(|error| format!("selection request decode failed: {error}"))?;
    let response = build_response(&request)?;
    let store = SelectionHoldStore::under_handoff(handoff_root()?);
    let receipt = store.stage(&response)?;
    match (
        receipt.hold_id == response.hold_id,
        receipt.hold_sha256 == response.hold_sha256,
        receipt.record_path.exists(),
    ) {
        (true, true, true) => atomic_write(
            response_path,
            &shadow_contracts::encode_selection_response(&response),
        ),
        (false, _, _) => Err("selection hold receipt id mismatch".to_string()),
        (_, false, _) => Err("selection hold receipt SHA mismatch".to_string()),
        (_, _, false) => Err("selection hold receipt path missing".to_string()),
    }
}

