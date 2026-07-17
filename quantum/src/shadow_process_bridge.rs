//! v1.6.4 — Quantum client for the mandatory Shadow executable boundary.
//!
//! Quantum links only `shadow_contracts`. It cannot construct `Shadow`, call
//! `ingest_bridged`, open the vault, issue tokens, or seal receipts. Every real
//! verification and negative-knowledge write is mediated by `shadow_platform`
//! main.rs through a checksummed file contract.

use shadow_contracts::{ShadowCycleRequest, ShadowCycleResponse};
#[cfg(feature = "dev_harness")]
use shadow_contracts::{ShadowNegativeRequest, ShadowNegativeResponse};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub fn execute_cycle(
    request: ShadowCycleRequest,
) -> Result<ShadowCycleResponse, String> {
    let session = request.quantum.session_id.clone();
    let bytes = shadow_contracts::encode_cycle_request(&request);
    let response = invoke("--bridge-once", &session, &bytes)?;
    let decoded = shadow_contracts::decode_cycle_response(&response)
        .map_err(|e| format!("Shadow response u refuzua: {e}"))?;
    match decoded.session_id == session {
        true => Ok(decoded),
        false => Err(format!(
            "Shadow response session mismatch: expected={session} actual={}",
            decoded.session_id
        )),
    }
}

#[cfg(feature = "dev_harness")]
pub fn persist_negative(
    request: ShadowNegativeRequest,
) -> Result<ShadowNegativeResponse, String> {
    let identity = format!("negative-{:016x}", request.negative.input_id);
    let bytes = shadow_contracts::encode_negative_request(&request);
    let response = invoke("--negative-once", &identity, &bytes)?;
    shadow_contracts::decode_negative_response(&response)
        .map_err(|e| format!("Shadow negative response u refuzua: {e}"))
}

pub fn now_ns() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0)
}

fn invoke(mode: &str, identity: &str, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
    let shadow_bin = resolve_shadow_binary()?;
    let handoff_dir = handoff_dir()?;
    let nonce = shadow_contracts::fnv1a64(
        format!("{}:{}:{}:{}", identity, std::process::id(), now_ns(), mode).as_bytes(),
    );
    let request_path = handoff_dir.join(format!("shadow_request_{nonce:016x}.bin"));
    let response_path = handoff_dir.join(format!("shadow_response_{nonce:016x}.bin"));

    match std::fs::write(&request_path, request_bytes) {
        Ok(()) => {}
        Err(e) => return Err(format!("kërkesa për Shadow main.rs s'u shkrua: {e}")),
    }
    let _ = std::fs::remove_file(&response_path);

    let status = Command::new(&shadow_bin)
        .arg(mode)
        .arg(&request_path)
        .arg(&response_path)
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();

    let _ = std::fs::remove_file(&request_path);
    let status = match status {
        Ok(s) => s,
        Err(e) => return Err(format!(
            "Shadow main.rs nuk u nis nga '{}': {e}", shadow_bin.display()
        )),
    };
    match status.success() {
        true => {}
        false => {
            let _ = std::fs::remove_file(&response_path);
            return Err(format!("Shadow main.rs doli me status {status}"));
        }
    }

    let response = std::fs::read(&response_path)
        .map_err(|e| format!("Shadow main.rs nuk prodhoi response të lexueshëm: {e}"));
    let _ = std::fs::remove_file(&response_path);
    response
}

fn handoff_dir() -> Result<PathBuf, String> {
    let raw = std::env::var("ESSMAI_HANDOFF_DIR")
        .map_err(|_| "ESSMAI_HANDOFF_DIR mungon".to_string())?;
    std::fs::create_dir_all(&raw)
        .map_err(|e| format!("ESSMAI_HANDOFF_DIR s'u krijua: {e}"))?;
    std::fs::canonicalize(&raw)
        .map_err(|e| format!("ESSMAI_HANDOFF_DIR s'u kanonikalizua: {e}"))
}

fn resolve_shadow_binary() -> Result<PathBuf, String> {
    match std::env::var("ESSMAI_SHADOW_BIN") {
        Ok(raw) => validate_binary(PathBuf::from(raw), "ESSMAI_SHADOW_BIN"),
        Err(_) => {
            let current = std::env::current_exe()
                .map_err(|e| format!("current_exe dështoi: {e}"))?;
            let parent = current.parent()
                .ok_or_else(|| "quantum executable pa parent".to_string())?;
            validate_binary(parent.join(shadow_executable_name()), "sibling target")
        }
    }
}

fn validate_binary(path: PathBuf, source: &str) -> Result<PathBuf, String> {
    match (path.is_absolute(), path.is_file()) {
        (true, true) => Ok(path),
        (false, _) => Err(format!("{source} duhet të japë rrugë absolute për Shadow main.rs")),
        (_, false) => Err(format!(
            "Shadow main.rs mungon te '{}'. Ekzekuto fillimisht: cargo build --workspace",
            path.display()
        )),
    }
}

fn shadow_executable_name() -> &'static Path {
    #[cfg(windows)]
    { Path::new("shadow_platform.exe") }
    #[cfg(not(windows))]
    { Path::new("shadow_platform") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sibling_name_is_platform_specific_and_nonempty() {
        assert!(!shadow_executable_name().as_os_str().is_empty());
    }
}
