//! v1.6.4 — Light client for Shadow APUPK project registration.
//!
//! Light prepares the user-project identity and Vula 500. Shadow main owns the
//! persistent APUPK store and returns a bounded ProjectContextWitness. The
//! witness authorizes Quantum to process that project; it never exposes the
//! Shadow vault or APUPK memory.

use shadow_contracts::{
    ProjectContextWitnessWire, ProjectRegistrationRequestWire,
    ProjectRegistrationResponseWire,
};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use sha2::{Digest, Sha256};

pub fn register_project(
    request: ProjectRegistrationRequestWire,
) -> Result<ProjectContextWitnessWire, String> {
    let identity = format!("project-{:016x}-{:016x}", request.project_id, request.trace_id);
    let request_bytes = shadow_contracts::encode_project_registration_request(&request);
    let response_bytes = invoke("--project-register-once", &identity, &request_bytes)?;
    let response: ProjectRegistrationResponseWire =
        shadow_contracts::decode_project_registration_response(&response_bytes)
            .map_err(|e| format!("Shadow APUPK response u refuzua: {e}"))?;
    match (response.accepted, response.witness) {
        (true, Some(witness)) => {
            let identity_matches = witness.project_id == request.project_id
                && witness.user_id == request.user_id
                && witness.trace_id == request.trace_id
                && witness.light_sovereign_flags == request.light_sovereign_flags;
            let expected_context = crate::gcl_project_contract::context_sha256(
                &crate::gcl_project_contract::ProjectContextMaterial {
                    project_id: witness.project_id,
                    user_id: witness.user_id,
                    trace_id: witness.trace_id,
                    revision: witness.revision,
                    project_title: &request.project_title,
                    content_sha256: &witness.content_sha256,
                    light_sovereign_flags: witness.light_sovereign_flags,
                },
            );
            match identity_matches
                && expected_context == witness.context_sha256
                && crate::gcl_project_contract::seal_is_500(witness.light_sovereign_flags)
            {
                true => Ok(witness),
                false => Err(
                    "Shadow ProjectContextWitness nuk lidhet me kërkesën APUPK".to_string(),
                ),
            }
        }
        (true, None) => Err("Shadow pranoi APUPK pa ProjectContextWitness".to_string()),
        (false, _) => Err(format!("Shadow APUPK refuzoi projektin reason={}", response.reason_code)),
    }
}

/// Ekzekuton projektin në procesin real Quantum, pa kufirin 2048-byte të
/// bus-it legacy. Kërkesa lidhet me APUPK witness dhe SHA-256 e payload-it;
/// Quantum e riverifikon përpara se të nisë procedimin shkencor dhe Shadow.
pub fn execute_project_with_quantum_main(
    payload: String,
    witness: &ProjectContextWitnessWire,
) -> Result<(), String> {
    let (request_bytes, request_sha256) = quantum_project_request(payload, witness);
    let response_bytes = invoke_quantum(
        "--project-process-once",
        &format!("project-{:016x}-{:016x}", witness.project_id, witness.trace_id),
        &request_bytes,
    )?;
    let response = shadow_contracts::decode_quantum_project_execution_response(&response_bytes)
        .map_err(|e| format!("Quantum project response u refuzua: {e}"))?;
    match response.completed
        && response.reason_code == 0
        && response.project_id == witness.project_id
        && response.project_trace_id == witness.trace_id
        && response.request_sha256 == request_sha256
    {
        true => Ok(()),
        false => Err(format!(
            "Quantum nuk e mbylli projektin: reason={} project={} trace={}",
            response.reason_code, response.project_id, response.project_trace_id,
        )),
    }
}

/// v1.6.5 — orienton vetëm pjesën Project drejt magazinimit/bisedës.
/// Përgjigjja nuk përmban token, receipt ose verdict; Light verifikon vetëm
/// identitetin APUPK, SHA-256 dhe deklarimin e kufirit `authority=NONE`.
pub fn orient_project_workspace_with_quantum_main(
    payload: String,
    witness: &ProjectContextWitnessWire,
) -> Result<(), String> {
    let (request_bytes, request_sha256) = quantum_project_request(payload, witness);
    let response_bytes = invoke_quantum(
        "--project-workspace-once",
        &format!("project-workspace-{:016x}-{:016x}", witness.project_id, witness.trace_id),
        &request_bytes,
    )?;
    let response = std::str::from_utf8(&response_bytes)
        .map_err(|_| "Quantum Project Workspace response nuk është UTF-8".to_string())?;
    validate_project_workspace_response(response, witness, &request_sha256)
}

fn quantum_project_request(
    payload: String,
    witness: &ProjectContextWitnessWire,
) -> (Vec<u8>, String) {
    let payload_sha256 = format!("{:x}", Sha256::digest(payload.as_bytes()));
    let request = shadow_contracts::QuantumProjectExecutionRequestWire {
        project_id: witness.project_id,
        project_trace_id: witness.trace_id,
        project_context_sha256: witness.context_sha256.clone(),
        payload_sha256,
        payload,
    };
    let request_bytes = shadow_contracts::encode_quantum_project_execution_request(&request);
    let request_sha256 = format!("{:x}", Sha256::digest(&request_bytes));
    (request_bytes, request_sha256)
}

fn validate_project_workspace_response(
    response: &str,
    witness: &ProjectContextWitnessWire,
    request_sha256: &str,
) -> Result<(), String> {
    let mut route = "";
    let mut project_id = "";
    let mut project_trace_id = "";
    let mut project_context_sha256 = "";
    let mut returned_request_sha256 = "";
    let mut workspace_sha256 = "";
    let mut material_sha256 = "";
    let mut conversation_turn_sha256 = "";
    let mut legacy_route = "";
    let mut authority = "";
    let mut token_policy = "";

    let mut parts = response.trim().split('|');
    match parts.next() {
        Some("ESSMAI_Q_PROJECT_WORKSPACE_V165") => {}
        _ => return Err("Quantum Project Workspace magic/version e pavlefshme".to_string()),
    }
    for part in parts {
        match part.split_once('=') {
            Some(("route", value)) => route = value,
            Some(("project_id", value)) => project_id = value,
            Some(("project_trace_id", value)) => project_trace_id = value,
            Some(("project_context_sha256", value)) => project_context_sha256 = value,
            Some(("request_sha256", value)) => returned_request_sha256 = value,
            Some(("workspace_sha256", value)) => workspace_sha256 = value,
            Some(("material_sha256", value)) => material_sha256 = value,
            Some(("conversation_turn_sha256", value)) => conversation_turn_sha256 = value,
            Some(("legacy_route", value)) => legacy_route = value,
            Some(("authority", value)) => authority = value,
            Some(("token_policy", value)) => token_policy = value,
            _ => {}
        }
    }

    let route_valid = matches!(
        route,
        "PROJECT_STORAGE" | "PROJECT_CONVERSATION" | "PROJECT_STORAGE_AND_CONVERSATION"
    );
    let identity_valid = u64::from_str_radix(project_id, 16) == Ok(witness.project_id)
        && u64::from_str_radix(project_trace_id, 16) == Ok(witness.trace_id)
        && project_context_sha256 == witness.context_sha256
        && returned_request_sha256 == request_sha256;
    let hashes_valid = [workspace_sha256, material_sha256, conversation_turn_sha256]
        .iter()
        .all(|value| value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit()));
    let boundary_valid = legacy_route == "--project-process-once"
        && authority == "NONE"
        && token_policy == "UNCHANGED";

    match route_valid && identity_valid && hashes_valid && boundary_valid {
        true => Ok(()),
        false => Err("Quantum Project Workspace response nuk ruan kufirin/identitetin".to_string()),
    }
}

fn invoke(mode: &str, identity: &str, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
    let shadow_bin = resolve_shadow_binary()?;
    let handoff_dir = handoff_dir()?;
    let nonce = shadow_contracts::fnv1a64(
        format!("{identity}:{}:{}:{mode}", std::process::id(), now_ns()).as_bytes(),
    );
    let request_path = handoff_dir.join(format!("light_project_request_{nonce:016x}.bin"));
    let response_path = handoff_dir.join(format!("light_project_response_{nonce:016x}.bin"));
    std::fs::write(&request_path, request_bytes)
        .map_err(|e| format!("kërkesa APUPK për Shadow main.rs s'u shkrua: {e}"))?;
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
    let status = status.map_err(|e| format!(
        "Shadow main.rs nuk u nis nga '{}': {e}", shadow_bin.display()
    ))?;
    match status.success() {
        true => {}
        false => {
            let _ = std::fs::remove_file(&response_path);
            return Err(format!("Shadow main.rs doli me status {status}"));
        }
    }
    let response = std::fs::read(&response_path)
        .map_err(|e| format!("Shadow main.rs nuk prodhoi APUPK response: {e}"));
    let _ = std::fs::remove_file(&response_path);
    response
}

fn invoke_quantum(
    mode: &str, identity: &str, request_bytes: &[u8],
) -> Result<Vec<u8>, String> {
    let quantum_bin = resolve_quantum_binary()?;
    let handoff_dir = handoff_dir()?;
    let nonce = shadow_contracts::fnv1a64(
        format!("{identity}:{}:{}:{mode}", std::process::id(), now_ns()).as_bytes(),
    );
    let request_path = handoff_dir.join(format!("quantum_project_request_{nonce:016x}.bin"));
    let response_path = handoff_dir.join(format!("quantum_project_response_{nonce:016x}.bin"));
    std::fs::write(&request_path, request_bytes)
        .map_err(|e| format!("kërkesa për Quantum main.rs s'u shkrua: {e}"))?;
    let _ = std::fs::remove_file(&response_path);
    let status = Command::new(&quantum_bin)
        .arg(mode).arg(&request_path).arg(&response_path)
        .stdin(Stdio::null()).stdout(Stdio::inherit()).stderr(Stdio::inherit())
        .status();
    let _ = std::fs::remove_file(&request_path);
    let status = status.map_err(|e| format!(
        "Quantum main.rs nuk u nis nga '{}': {e}", quantum_bin.display(),
    ))?;
    match status.success() {
        true => {}
        false => {
            let _ = std::fs::remove_file(&response_path);
            return Err(format!("Quantum main.rs doli me status {status}"));
        }
    }
    let response = std::fs::read(&response_path)
        .map_err(|e| format!("Quantum main.rs nuk prodhoi project response: {e}"));
    let _ = std::fs::remove_file(&response_path);
    response
}

fn resolve_quantum_binary() -> Result<PathBuf, String> {
    match std::env::var("ESSMAI_QUANTUM_BIN") {
        Ok(raw) => validate_binary(PathBuf::from(raw), "ESSMAI_QUANTUM_BIN"),
        Err(_) => {
            let current = std::env::current_exe()
                .map_err(|e| format!("current_exe dështoi: {e}"))?;
            let parent = current.parent()
                .ok_or_else(|| "Light executable pa parent".to_string())?;
            validate_binary(parent.join(quantum_executable_name()), "sibling target")
        }
    }
}

fn quantum_executable_name() -> &'static Path {
    #[cfg(windows)]
    { Path::new("quantum-platform.exe") }
    #[cfg(not(windows))]
    { Path::new("quantum-platform") }
}

fn now_ns() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0)
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
                .ok_or_else(|| "Light executable pa parent".to_string())?;
            validate_binary(parent.join(shadow_executable_name()), "sibling target")
        }
    }
}

fn validate_binary(path: PathBuf, source: &str) -> Result<PathBuf, String> {
    match (path.is_absolute(), path.is_file()) {
        (true, true) => Ok(path),
        (false, _) => Err(format!("{source} duhet të japë rrugë absolute për binarin sovran")),
        (_, false) => Err(format!(
            "binari sovran mungon te '{}'. Ekzekuto: cargo build --workspace",
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
    fn shadow_binary_name_exists_for_platform() {
        assert!(!shadow_executable_name().as_os_str().is_empty());
        assert!(!quantum_executable_name().as_os_str().is_empty());
    }


    #[test]
    fn workspace_response_is_bound_without_authority_or_token() {
        let witness = ProjectContextWitnessWire {
            project_id: 77,
            user_id: 9,
            trace_id: 88,
            revision: 0,
            content_sha256: "11".repeat(32),
            light_sovereign_flags: 0x0000_A451,
            context_sha256: "22".repeat(32),
        };
        let request_sha256 = "33".repeat(32);
        let line = format!(
            "ESSMAI_Q_PROJECT_WORKSPACE_V165|route=PROJECT_STORAGE_AND_CONVERSATION|project_id={:016x}|project_trace_id={:016x}|project_context_sha256={}|request_sha256={}|workspace_sha256={}|material_sha256={}|conversation_turn_sha256={}|legacy_route=--project-process-once|authority=NONE|token_policy=UNCHANGED\n",
            witness.project_id,
            witness.trace_id,
            witness.context_sha256,
            request_sha256,
            "44".repeat(32),
            "55".repeat(32),
            "66".repeat(32),
        );
        assert!(validate_project_workspace_response(
            &line,
            &witness,
            &request_sha256,
        ).is_ok());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProjectRouteMode {
    Workspace,
    LegacyScientific,
}

/// Porta runtime e Light-it për projektet e përdoruesit.
///
/// - `--project-route-once`: magazinë + bisedë, pa procedim të plotë;
/// - `--project-route-legacy-once`: rruga shkencore ekzistuese v1.6.4.
pub fn dispatch_project_route(args: &[String]) -> Option<i32> {
    let selected = match args {
        [_, flag, request_path, response_path] if flag == "--project-route-once" => Some((
            ProjectRouteMode::Workspace,
            Path::new(request_path),
            Path::new(response_path),
        )),
        [_, flag, request_path, response_path] if flag == "--project-route-legacy-once" => Some((
            ProjectRouteMode::LegacyScientific,
            Path::new(request_path),
            Path::new(response_path),
        )),
        [_, flag, ..] if flag.starts_with("--project-route") => {
            eprintln!("[LIGHT PROJECT/GCL] argumente të pavlefshme");
            return Some(64);
        }
        _ => None,
    };
    match selected {
        Some((mode, request_path, response_path)) => Some(match run_project_route(
            request_path,
            response_path,
            mode,
        ) {
            Ok(()) => 0,
            Err(error) => {
                eprintln!("[LIGHT PROJECT/GCL] {error}");
                70
            }
        }),
        None => None,
    }
}

fn run_project_route(
    request_path: &Path,
    response_path: &Path,
    mode: ProjectRouteMode,
) -> Result<(), String> {
    let bytes = std::fs::read(request_path)
        .map_err(|e| format!("kërkesa e projektit s'u lexua: {e}"))?;
    let request = shadow_contracts::decode_light_project_intake_request(&bytes)
        .map_err(|e| format!("kërkesa e projektit u refuzua: {e}"))?;

    let response = match validate_intake(&request) {
        Ok(()) => {
            let timestamp_ns = current_time_ns();
            let user_id = std::env::var("ESSMAI_USER_ID")
                .ok()
                .and_then(|raw| raw.parse::<u64>().ok())
                .filter(|value| *value != 0)
                .unwrap_or(1);
            let identity_material = format!(
                "{}:{}:{}:{}",
                user_id,
                request.project_name.trim(),
                request.project_content.len(),
                timestamp_ns,
            );
            let project_id = shadow_contracts::fnv1a64(identity_material.as_bytes()).max(1);
            let mut gate = crate::sovereign::SovereignGate::new();
            let coordinator = crate::apupk::ApupkCoordinator::new();
            let project = crate::apupk::ProjectUpload {
                project_id,
                user_id,
                project_name: request.project_name.clone(),
                description: request.project_description.clone(),
                content: request.project_content.clone(),
                timestamp_ns,
            };
            let files = request.files.iter()
                .map(|file| (file.kind, file.bytes.clone()))
                .collect::<Vec<_>>();
            let route_result = match mode {
                ProjectRouteMode::Workspace =>
                    crate::sovereign_bridges::ApupkShadowBridge::route_project_workspace_under_gcl(
                        &mut gate,
                        &coordinator,
                        project,
                        &request.domain,
                        &request.lang_code,
                        &request.evolve_summary,
                        &request.hypothesis,
                        &request.assumptions,
                        &request.documentation_description,
                        &files,
                    ),
                ProjectRouteMode::LegacyScientific =>
                    crate::sovereign_bridges::ApupkShadowBridge::route_scientific_project_under_gcl(
                        &mut gate,
                        &coordinator,
                        project,
                        &request.domain,
                        &request.lang_code,
                        &request.evolve_summary,
                        &request.hypothesis,
                        &request.assumptions,
                        &request.documentation_description,
                        &files,
                    ),
            };
            match route_result {
                Ok(witness) => shadow_contracts::LightProjectIntakeResponseWire {
                    accepted_into_gcl: true,
                    reason_code: 0,
                    witness: Some(witness),
                },
                Err(error) => {
                    eprintln!("[LIGHT PROJECT/GCL] route u refuzua: {error}");
                    shadow_contracts::LightProjectIntakeResponseWire {
                        accepted_into_gcl: false,
                        reason_code: 9,
                        witness: None,
                    }
                }
            }
        }
        Err(reason_code) => shadow_contracts::LightProjectIntakeResponseWire {
            accepted_into_gcl: false,
            reason_code,
            witness: None,
        },
    };
    atomic_write(
        response_path,
        &shadow_contracts::encode_light_project_intake_response(&response),
    )
}

fn validate_intake(
    request: &shadow_contracts::LightProjectIntakeRequestWire,
) -> Result<(), u8> {
    let files_valid = request.files.len() <= 4_096
        && request.files.iter().all(|file| matches!(
            file.kind,
            shadow_contracts::PROJECT_EVIDENCE_UNKNOWN
                | shadow_contracts::PROJECT_EVIDENCE_IMAGE
                | shadow_contracts::PROJECT_EVIDENCE_VIDEO
                | shadow_contracts::PROJECT_EVIDENCE_DOCUMENT
        ));
    // UI-ja furnizon vetëm material. Identiteti, timestamp-i, kontrata GCL
    // dhe Vula prodhohen brenda Light-it.
    match (
        !request.project_name.trim().is_empty(),
        !request.project_content.is_empty(),
        !request.domain.trim().is_empty(),
        !request.hypothesis.trim().is_empty(),
        !request.documentation_description.trim().is_empty(),
        files_valid,
    ) {
        (true, true, true, true, true, true) => Ok(()),
        (false, _, _, _, _, _) => Err(1),
        (_, false, _, _, _, _) => Err(2),
        (_, _, false, _, _, _) => Err(3),
        (_, _, _, false, _, _) => Err(4),
        (_, _, _, _, false, _) => Err(5),
        (_, _, _, _, _, false) => Err(6),
    }
}

fn current_time_ns() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|value| value.as_nanos() as u64)
        .unwrap_or(0)
}

fn atomic_write(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let parent = path.parent().ok_or_else(|| "response path pa parent".to_string())?;
    std::fs::create_dir_all(parent)
        .map_err(|e| format!("response parent s'u krijua: {e}"))?;
    let temp = path.with_extension(format!("tmp.{}", std::process::id()));
    std::fs::write(&temp, bytes)
        .map_err(|e| format!("project response temp s'u shkrua: {e}"))?;
    match std::fs::rename(&temp, path) {
        Ok(()) => Ok(()),
        Err(first) => {
            let _ = std::fs::remove_file(path);
            std::fs::rename(&temp, path)
                .map_err(|second| format!("project response rename dështoi: {first}; {second}"))
        }
    }
}
