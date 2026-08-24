//! Mediated Light -> Shadow read-only selection client.

use shadow_contracts::{
    KnowledgeSelectionRequestWire, KnowledgeSelectionResponseWire, SelectionRequester,
};
#[cfg(test)]
use shadow_contracts::CandidateStatisticWire;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Gatishmëria e urës së vetme Light→Shadow që lejohet në Coordination:
/// përzgjedhje statistikore read-only. Nuk nis proces dhe nuk krijon verdikt.
#[cfg(not(test))]
pub fn gcl_status() -> crate::lab_contracts::gcl_presume::GclReadiness {
    match (handoff_dir(), resolve_shadow_binary()) {
        (Ok(_), Ok(_)) => crate::lab_contracts::gcl_presume::GclReadiness::Ready,
        (Err(_), _) => crate::lab_contracts::gcl_presume::GclReadiness::NotReady(
            "ESSMAI_HANDOFF_DIR nuk është gati për selection",
        ),
        (_, Err(_)) => crate::lab_contracts::gcl_presume::GclReadiness::NotReady(
            "Shadow binary absolut mungon për selection read-only",
        ),
    }
}

#[cfg(test)]
pub fn gcl_status() -> crate::lab_contracts::gcl_presume::GclReadiness {
    crate::lab_contracts::gcl_presume::GclReadiness::Ready
}

#[cfg(not(test))]
pub fn execute_selection(
    request: &KnowledgeSelectionRequestWire,
) -> Result<KnowledgeSelectionResponseWire, String> {
    match (request.requester, request.shape_is_valid()) {
        (SelectionRequester::BesaNlight, true) => {}
        (SelectionRequester::AshtQuantum, _) => {
            return Err("Light cannot issue an Asht selection request".to_string())
        }
        (_, false) => return Err("Light selection request shape invalid".to_string()),
    }
    let request_bytes = shadow_contracts::encode_selection_request(request);
    let shadow_bin = resolve_shadow_binary()?;
    let handoff = handoff_dir()?;
    let nonce = shadow_contracts::sha256_hex(&[
        b"ESSMAI/LIGHT/SELECTION/NONCE/V178",
        request.session_id.as_bytes(),
        request.split_sha256.as_bytes(),
        &std::process::id().to_le_bytes(),
        &now_ns()?.to_le_bytes(),
    ]);
    let request_path = handoff.join(format!("besa_selection_request_{nonce}.bin"));
    let response_path = handoff.join(format!("besa_selection_response_{nonce}.bin"));
    let _transients = SelectionTransients {
        request: request_path.clone(),
        response: response_path.clone(),
    };
    std::fs::write(&request_path, &request_bytes)
        .map_err(|error| format!("Besa selection request write failed: {error}"))?;
    cleanup_transient(&response_path);
    let status = Command::new(&shadow_bin)
        .arg("--selection-once")
        .arg(&request_path)
        .arg(&response_path)
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|error| {
            format!(
                "Shadow selection process did not start from '{}': {error}",
                shadow_bin.display()
            )
        })?;
    match status.success() {
        true => {}
        false => {
            return Err(format!("Shadow selection process exited with {status}"));
        }
    }
    let response_bytes = std::fs::read(&response_path)
        .map_err(|error| format!("Shadow selection response read failed: {error}"))?;
    let response = shadow_contracts::decode_selection_response(&response_bytes)
        .map_err(|error| format!("Shadow selection response decode failed: {error}"))?;
    match response.shape_is_valid_for(request) {
        true => Ok(response),
        false => Err("Shadow selection response was not bound to Besa request".to_string()),
    }
}

#[cfg(test)]
pub fn execute_selection(
    request: &KnowledgeSelectionRequestWire,
) -> Result<KnowledgeSelectionResponseWire, String> {
    match (request.requester, request.shape_is_valid()) {
        (SelectionRequester::BesaNlight, true) => {
            let mut response = KnowledgeSelectionResponseWire {
                version: shadow_contracts::PROTOCOL_VERSION,
                requester: SelectionRequester::BesaNlight,
                session_id: request.session_id.clone(),
                parent_i0: request.parent_i0.clone(),
                primitive_anchor: request.primitive_anchor,
                split_sha256: request.split_sha256.clone(),
                upstream_hold_sha256: request.upstream_hold_sha256.clone(),
                request_sha256: crate::besa_nlight::selection_request_digest(request),
                selection_sha256: String::new(),
                hold_id: "besa-test-hold".into(),
                hold_sha256: String::new(),
                positive: vec![CandidateStatisticWire {
                    candidate_id: 1,
                    polarity: 1,
                    domain: "test-positive".into(),
                    term_sha256: "44".repeat(32),
                    source_sha256: "55".repeat(32),
                    security_y: 1,
                    trust_x: 1,
                    confidence_fixed: 8_000,
                    frequency: 1,
                    domain_count: 1,
                    legacy_score_fixed: 8_000,
                }],
                negative: vec![CandidateStatisticWire {
                    candidate_id: 2,
                    polarity: 0,
                    domain: "test-negative".into(),
                    term_sha256: "66".repeat(32),
                    source_sha256: "77".repeat(32),
                    security_y: 0,
                    trust_x: 0,
                    confidence_fixed: 7_000,
                    frequency: 1,
                    domain_count: 1,
                    legacy_score_fixed: 7_000,
                }],
                law_seal: crate::lab_contracts::gjata_collapse_law::law_seal(),
                system_laws_seal: crate::lab_contracts::system_laws_seal(),
                expires_at_ns: u64::MAX,
            };
            response.selection_sha256 =
                crate::besa_nlight::selection_response_digest(&response);
            response.hold_sha256 = crate::besa_nlight::selection_hold_digest(&response);
            Ok(response)
        }
        (SelectionRequester::AshtQuantum, _) => {
            Err("Light cannot issue an Asht selection request".to_string())
        }
        (_, false) => Err("Light selection request shape invalid".to_string()),
    }
}

#[cfg(not(test))]
struct SelectionTransients {
    request: PathBuf,
    response: PathBuf,
}

#[cfg(not(test))]
impl Drop for SelectionTransients {
    fn drop(&mut self) {
        cleanup_transient(&self.request);
        cleanup_transient(&self.response);
    }
}

#[cfg(not(test))]
fn cleanup_transient(path: &Path) {
    match std::fs::remove_file(path) {
        Ok(()) => {}
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {}
            _ => eprintln!(
                "[BESA/SELECTION] transient cleanup defect {}: {error}",
                path.display()
            ),
        },
    }
}

#[cfg(not(test))]
fn now_ns() -> Result<u64, String> {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos().min(u128::from(u64::MAX)) as u64)
        .map_err(|error| format!("Besa selection clock failed: {error}"))
}

#[cfg(not(test))]
fn handoff_dir() -> Result<PathBuf, String> {
    let raw = std::env::var("ESSMAI_HANDOFF_DIR")
        .map_err(|_| "ESSMAI_HANDOFF_DIR missing for Besa selection".to_string())?;
    std::fs::create_dir_all(&raw)
        .map_err(|error| format!("ESSMAI_HANDOFF_DIR create failed: {error}"))?;
    std::fs::canonicalize(&raw)
        .map_err(|error| format!("ESSMAI_HANDOFF_DIR canonicalize failed: {error}"))
}

#[cfg(not(test))]
fn resolve_shadow_binary() -> Result<PathBuf, String> {
    let path = match std::env::var("ESSMAI_SHADOW_BIN") {
        Ok(raw) => PathBuf::from(raw),
        Err(_) => {
            let current =
                std::env::current_exe().map_err(|error| format!("current_exe failed: {error}"))?;
            let parent = current
                .parent()
                .ok_or_else(|| "Light executable has no parent".to_string())?;
            parent.join(shadow_executable_name())
        }
    };
    match (path.is_absolute(), executable_file(&path)) {
        (true, true) => Ok(path),
        (false, _) => Err("Shadow binary path must be absolute".to_string()),
        (_, false) => Err(format!(
            "Shadow binary missing or not executable at '{}'",
            path.display()
        )),
    }
}

#[cfg(not(test))]
fn executable_file(path: &Path) -> bool {
    match path.metadata() {
        Ok(metadata) if metadata.is_file() => {
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt as _;
                metadata.permissions().mode() & 0o111 != 0
            }
            #[cfg(not(unix))]
            {
                true
            }
        }
        _ => false,
    }
}

#[cfg(all(not(test), windows))]
fn shadow_executable_name() -> &'static Path {
    Path::new("shadow_platform.exe")
}

#[cfg(all(not(test), not(windows)))]
fn shadow_executable_name() -> &'static Path {
    Path::new("shadow_platform")
}
