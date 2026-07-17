// ╔══════════════════════════════════════════════════════════════════════════╗
// ║ NURA LEGACY — UI e vjetër, kufi minimal v1.6.4                         ║
// ║                                                                          ║
// ║ Përgjegjësitë e vetme:                                                   ║
// ║   1. pranon materialin e projektit të përdoruesit;                       ║
// ║   2. ia dorëzon Light-it përmes `--project-route-once`;                  ║
// ║   3. pasqyron komandën emocionale të prodhuar nga gjendja reale.         ║
// ║                                                                          ║
// ║ Nuk krijon GCL, Vulë 500, TRL, verdict, reasoning ose rrugë Shadow.      ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

#[path = "../../../../ui_contracts/emotional_command.rs"]
mod emotional_command;
use emotional_command::EmotionalCommand;

const MAX_PROJECT_CONTENT_BYTES: usize = 16 * 1024 * 1024;
const MAX_PROJECT_FILE_BYTES: usize = 16 * 1024 * 1024;
const MAX_PROJECT_FILES_BYTES: usize = 48 * 1024 * 1024;
const MAX_PROJECT_UPLOAD_BYTES: usize = 60 * 1024 * 1024;
const MAX_PROJECT_FILES: usize = 64;

#[cfg(target_os = "windows")]
const LIGHT_BIN: &str = "light-platform.exe";
#[cfg(not(target_os = "windows"))]
const LIGHT_BIN: &str = "light-platform";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UiProjectFile {
    name: String,
    mime_type: String,
    bytes: Vec<u8>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UiProjectUpload {
    project_name: String,
    project_description: String,
    project_content: String,
    domain: String,
    hypothesis: String,
    assumptions: Vec<String>,
    documentation_description: String,
    files: Vec<UiProjectFile>,
}

#[derive(Debug, Serialize)]
struct ProjectUploadResponse {
    accepted_into_gcl: bool,
    reason_code: u8,
    project_id: u64,
    trace_id: u64,
    context_sha256: String,
    uploaded_files: usize,
}

#[tauri::command]
fn upload_project(request: UiProjectUpload) -> Result<ProjectUploadResponse, String> {
    validate_ui_upload(&request)?;

    let files = request.files.iter().map(|file| {
        shadow_contracts::ProjectEvidenceFileWire {
            kind: evidence_kind(&file.name, &file.mime_type),
            bytes: file.bytes.clone(),
        }
    }).collect::<Vec<_>>();

    // UI furnizon vetëm materialin e përdoruesit. Light prodhon kontratën
    // GCL dhe Vulën 500; UI nuk mban fusha autoriteti në wire.
    let intake = shadow_contracts::LightProjectIntakeRequestWire {
        project_name: request.project_name.trim().to_string(),
        project_description: request.project_description.trim().to_string(),
        project_content: request.project_content,
        domain: request.domain.trim().to_string(),
        lang_code: "sq".to_string(),
        evolve_summary: "OLD_UI_UPLOAD_ONLY_V164".to_string(),
        hypothesis: request.hypothesis.trim().to_string(),
        assumptions: request.assumptions.into_iter()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
            .collect(),
        documentation_description: request.documentation_description.trim().to_string(),
        files,
    };

    let encoded = shadow_contracts::encode_light_project_intake_request(&intake);
    let response = invoke_light_project_route(&encoded)?;
    let witness = response.witness.as_ref();

    Ok(ProjectUploadResponse {
        accepted_into_gcl: response.accepted_into_gcl,
        reason_code: response.reason_code,
        project_id: witness.map(|value| value.project_id).unwrap_or(0),
        trace_id: witness.map(|value| value.trace_id).unwrap_or(0),
        context_sha256: witness
            .map(|value| value.context_sha256.clone())
            .unwrap_or_default(),
        uploaded_files: intake.files.len(),
    })
}

/// UI-ja e vjetër vetëm pasqyron emocionin e një gjendjeje të ardhur nga
/// sistemi. Ajo nuk krijon tekst përgjigjeje dhe nuk ndryshon verdict-in.
#[tauri::command]
fn reflect_system_emotion(trace_id: u64, runtime_output: String) -> EmotionalCommand {
    EmotionalCommand::from_runtime_output(trace_id, &runtime_output)
}

fn validate_ui_upload(request: &UiProjectUpload) -> Result<(), String> {
    let file_bytes = request.files.iter()
        .try_fold(0usize, |total, file| total.checked_add(file.bytes.len()))
        .ok_or_else(|| "madhësia e skedarëve tejkaloi kufirin numerik".to_string())?;
    let upload_bytes = request.project_content.len().checked_add(file_bytes)
        .ok_or_else(|| "madhësia e upload-it tejkaloi kufirin numerik".to_string())?;
    let each_file_bounded = request.files.iter()
        .all(|file| file.bytes.len() <= MAX_PROJECT_FILE_BYTES);
    match (
        request.project_name.trim().is_empty(),
        request.project_content.is_empty(),
        request.domain.trim().is_empty(),
        request.hypothesis.trim().is_empty(),
        request.documentation_description.trim().is_empty(),
        request.project_content.len() <= MAX_PROJECT_CONTENT_BYTES,
        request.files.len() <= MAX_PROJECT_FILES,
        each_file_bounded,
        file_bytes <= MAX_PROJECT_FILES_BYTES,
        upload_bytes <= MAX_PROJECT_UPLOAD_BYTES,
    ) {
        (true, _, _, _, _, _, _, _, _, _) => Err("emri i projektit mungon".to_string()),
        (_, true, _, _, _, _, _, _, _, _) => Err("përmbajtja e projektit mungon".to_string()),
        (_, _, true, _, _, _, _, _, _, _) => Err("domeni i projektit mungon".to_string()),
        (_, _, _, true, _, _, _, _, _, _) => Err("hipoteza e projektit mungon".to_string()),
        (_, _, _, _, true, _, _, _, _, _) => Err("përshkrimi i dokumentacionit mungon".to_string()),
        (_, _, _, _, _, false, _, _, _, _) => Err("përmbajtja e projektit kalon 16 MiB".to_string()),
        (_, _, _, _, _, _, false, _, _, _) => Err("numri i skedarëve kalon 64".to_string()),
        (_, _, _, _, _, _, _, false, _, _) => Err("një skedar kalon 16 MiB".to_string()),
        (_, _, _, _, _, _, _, _, false, _) => Err("skedarët e projektit kalojnë 48 MiB".to_string()),
        (_, _, _, _, _, _, _, _, _, false) => Err("upload-i i plotë kalon 60 MiB".to_string()),
        _ => Ok(()),
    }
}

fn evidence_kind(name: &str, mime_type: &str) -> u8 {
    let lower_name = name.to_ascii_lowercase();
    let lower_mime = mime_type.to_ascii_lowercase();
    match (
        lower_mime.starts_with("image/"),
        lower_mime.starts_with("video/"),
        lower_mime.starts_with("text/")
            || lower_mime.contains("pdf")
            || lower_mime.contains("document")
            || lower_mime.contains("zip")
            || lower_name.ends_with(".docx")
            || lower_name.ends_with(".xlsx")
            || lower_name.ends_with(".pptx"),
    ) {
        (true, _, _) => shadow_contracts::PROJECT_EVIDENCE_IMAGE,
        (_, true, _) => shadow_contracts::PROJECT_EVIDENCE_VIDEO,
        (_, _, true) => shadow_contracts::PROJECT_EVIDENCE_DOCUMENT,
        _ => shadow_contracts::PROJECT_EVIDENCE_UNKNOWN,
    }
}

fn invoke_light_project_route(
    request_bytes: &[u8],
) -> Result<shadow_contracts::LightProjectIntakeResponseWire, String> {
    let light = find_light()?;
    let handoff_dir = std::env::var("ESSMAI_HANDOFF_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| std::env::temp_dir().join("essmai_v164_handoff"));
    std::fs::create_dir_all(&handoff_dir)
        .map_err(|e| format!("handoff dir s'u krijua: {e}"))?;

    let nonce = shadow_contracts::fnv1a64(&[
        request_bytes,
        &std::process::id().to_le_bytes(),
        &now_ns().to_le_bytes(),
    ].concat());
    let request_path = handoff_dir.join(format!("old_ui_project_{nonce:016x}.request"));
    let response_path = handoff_dir.join(format!("old_ui_project_{nonce:016x}.response"));
    std::fs::write(&request_path, request_bytes)
        .map_err(|e| format!("kërkesa e upload-it s'u shkrua: {e}"))?;
    let _ = std::fs::remove_file(&response_path);

    let output = Command::new(&light)
        .arg("--project-route-once")
        .arg(&request_path)
        .arg(&response_path)
        .env("ESSMAI_HANDOFF_DIR", &handoff_dir)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();
    let _ = std::fs::remove_file(&request_path);
    let output = output.map_err(|e| format!(
        "Light nuk u nis nga '{}': {e}", light.display(),
    ))?;
    match output.status.success() {
        true => {}
        false => {
            let _ = std::fs::remove_file(&response_path);
            return Err(format!(
                "Light refuzoi upload-in (code={:?}): {}",
                output.status.code(),
                String::from_utf8_lossy(&output.stderr),
            ));
        }
    }

    let response_bytes = std::fs::read(&response_path)
        .map_err(|e| format!("Light nuk prodhoi përgjigje upload-i: {e}"))?;
    let _ = std::fs::remove_file(&response_path);
    shadow_contracts::decode_light_project_intake_response(&response_bytes)
        .map_err(|e| format!("përgjigjja e Light u refuzua: {e}"))
}

fn find_light() -> Result<PathBuf, String> {
    let mut candidates = Vec::new();
    match std::env::var("ESSMAI_LIGHT") {
        Ok(value) => candidates.push(PathBuf::from(value)),
        Err(_) => {}
    }
    match std::env::current_exe() {
        Ok(exe) => match exe.parent() {
            Some(parent) => candidates.push(parent.join(LIGHT_BIN)),
            None => {}
        },
        Err(_) => {}
    }
    for root in [".", "..", "../..", "../../..", "../../../.."] {
        candidates.push(Path::new(root).join("target").join("release").join(LIGHT_BIN));
        candidates.push(Path::new(root).join("target").join("debug").join(LIGHT_BIN));
    }
    for candidate in candidates {
        match candidate.is_file() {
            true => return std::fs::canonicalize(&candidate)
                .map_err(|e| format!("rruga e Light s'u kanonikalizua: {e}")),
            false => {}
        }
    }
    Err(format!(
        "Light mungon. Ndërto workspace-in ose cakto ESSMAI_LIGHT me rrugë absolute te {LIGHT_BIN}",
    ))
}

fn now_ns() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|value| value.as_nanos() as u64)
        .unwrap_or(0)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            upload_project,
            reflect_system_emotion,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Nura Legacy old UI");
}
