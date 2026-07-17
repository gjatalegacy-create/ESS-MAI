//! Nura Legacy Tauri 2 shell.
//!
//! Security boundary:
//! UI/Tauri -> Light only. Light remains the sole coordinator that may route
//! to Quantum and Shadow. This crate cannot mint tokens, issue verdicts,
//! construct GCL authority, or invoke Quantum/Shadow directly.

use serde::{Deserialize, Serialize};
use shadow_contracts::{
    LightProjectIntakeRequestWire, ProjectEvidenceFileWire,
    PROJECT_EVIDENCE_DOCUMENT, PROJECT_EVIDENCE_IMAGE, PROJECT_EVIDENCE_UNKNOWN,
    PROJECT_EVIDENCE_VIDEO,
};
use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};

#[path = "../../ui_contracts/emotional_command.rs"]
mod emotional_command;
use emotional_command::EmotionalCommand;

const MAX_CHAT_BYTES: usize = 64 * 1024;
const MAX_PROJECT_TEXT_BYTES: usize = 2 * 1024 * 1024;
const MAX_PROJECT_FILES: usize = 8;
const MAX_PROJECT_FILE_BYTES: usize = 8 * 1024 * 1024;
const MAX_PROJECT_TOTAL_FILE_BYTES: usize = 24 * 1024 * 1024;

static UI_TRACE_COUNTER: AtomicU64 = AtomicU64::new(1);
static HANDOFF_COUNTER: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Serialize)]
struct NuraUiResponse {
    nura_text: String,
    nura_verified: bool,
    trace_id: u64,
    emotional: EmotionalCommand,
}

#[derive(Debug, Serialize)]
struct RuntimeStatus {
    light_available: bool,
    light_path: String,
    boundary: &'static str,
    normal_conversation: &'static str,
    project_workspace: &'static str,
    scientific_project: &'static str,
    quantum_role: &'static str,
    shadow_role: &'static str,
    token_policy: &'static str,
}

#[derive(Debug, Deserialize)]
struct ProjectFileInput {
    name: String,
    mime_type: String,
    bytes: Vec<u8>,
}

#[derive(Debug, Deserialize)]
struct ProjectSubmission {
    route: String,
    project_name: String,
    project_description: String,
    project_content: String,
    domain: String,
    language_code: String,
    evolution_summary: String,
    hypothesis: String,
    assumptions: Vec<String>,
    documentation_description: String,
    files: Vec<ProjectFileInput>,
}

#[derive(Debug, Serialize)]
struct ProjectSubmissionResponse {
    accepted: bool,
    reason_code: u8,
    route: String,
    project_id: Option<u64>,
    trace_id: Option<u64>,
    revision: Option<u32>,
    context_sha256: Option<String>,
    content_sha256: Option<String>,
    message: String,
}

#[cfg(target_os = "windows")]
const LIGHT_BIN: &str = "light-platform.exe";
#[cfg(not(target_os = "windows"))]
const LIGHT_BIN: &str = "light-platform";

fn canonical_file(path: PathBuf, source: &str) -> Result<PathBuf, String> {
    let canonical = fs::canonicalize(&path)
        .map_err(|e| format!("{source}: '{}' nuk u gjet: {e}", path.display()))?;
    match canonical.is_file() {
        true => Ok(canonical),
        false => Err(format!("{source}: '{}' nuk është skedar", canonical.display())),
    }
}

fn find_light() -> Result<PathBuf, String> {
    if let Ok(raw) = std::env::var("ESSMAI_LIGHT") {
        let configured = PathBuf::from(raw);
        match configured.is_absolute() {
            true => return canonical_file(configured, "ESSMAI_LIGHT"),
            false => return Err("ESSMAI_LIGHT duhet të jetë rrugë absolute".to_string()),
        }
    }

    let current = std::env::current_exe()
        .map_err(|e| format!("current_exe dështoi: {e}"))?;
    let mut candidates = Vec::new();
    if let Some(parent) = current.parent() {
        candidates.push(parent.join(LIGHT_BIN));
        if let Some(grandparent) = parent.parent() {
            candidates.push(grandparent.join(LIGHT_BIN));
        }
    }
    candidates.push(PathBuf::from("target/release").join(LIGHT_BIN));
    candidates.push(PathBuf::from("../target/release").join(LIGHT_BIN));
    candidates.push(PathBuf::from("../../target/release").join(LIGHT_BIN));

    for candidate in candidates {
        if candidate.is_file() {
            return canonical_file(candidate, "Light runtime");
        }
    }
    Err(format!(
        "Binar i Light-it mungon. Ndërto workspace-in ose cakto ESSMAI_LIGHT me rrugë absolute te {LIGHT_BIN}."
    ))
}

fn validate_text(label: &str, value: &str, max_bytes: usize, required: bool) -> Result<(), String> {
    let trimmed = value.trim();
    if required && trimmed.is_empty() {
        return Err(format!("{label} është i detyrueshëm"));
    }
    if value.len() > max_bytes {
        return Err(format!("{label} kalon kufirin {max_bytes} byte"));
    }
    if value.chars().any(|ch| ch == '\0') {
        return Err(format!("{label} përmban NUL të palejuar"));
    }
    Ok(())
}

fn verified_nura_line(runtime_output: &str) -> Option<String> {
    runtime_output.lines().rev().find_map(|line| {
        line.trim()
            .strip_prefix("Nura:")
            .map(|text| text.trim().to_string())
    })
}

#[tauri::command]
fn runtime_status() -> RuntimeStatus {
    let light = find_light();
    RuntimeStatus {
        light_available: light.is_ok(),
        light_path: light
            .map(|path| path.display().to_string())
            .unwrap_or_else(|error| error),
        boundary: "UI → Tauri 2 → Light only",
        normal_conversation: "Light coordination → Quantum reasoning → Shadow verification → Light output",
        project_workspace: "Light APUPK registration → Quantum project storage/conversation orientation",
        scientific_project: "Explicit legacy scientific route under GCL; never selected implicitly",
        quantum_role: "Reasoning and project-workspace orientation; no UI authority",
        shadow_role: "Multi-step verification, APUPK persistence, TRL4 factualization under GCL",
        token_policy: "UNCHANGED — UI creates and transports no LGC, Forge, capability, receipt, or trust token",
    }
}

#[tauri::command]
fn ask_nura(text: String, deep_research: bool) -> Result<NuraUiResponse, String> {
    validate_text("Pyetja", &text, MAX_CHAT_BYTES, true)?;
    let trimmed = text.trim();
    let light = find_light()?;
    let trace_id = UI_TRACE_COUNTER.fetch_add(1, Ordering::SeqCst);
    let payload = match deep_research {
        true => format!("[DEEP_RESEARCH]\n{trimmed}\n"),
        false => format!("{trimmed}\n"),
    };

    let mut child = Command::new(&light)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Light nuk u nis nga '{}': {e}", light.display()))?;

    let mut stdin = child
        .stdin
        .take()
        .ok_or_else(|| "stdin i Light-it mungon".to_string())?;
    stdin
        .write_all(payload.as_bytes())
        .map_err(|e| format!("stdin i Light-it dështoi: {e}"))?;
    drop(stdin);

    let output = child
        .wait_with_output()
        .map_err(|e| format!("Light nuk u prit dot: {e}"))?;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    if !output.status.success() {
        return Err(format!(
            "Light doli me code={:?}:\n{}\n{}",
            output.status.code(), stdout, stderr
        ));
    }

    let runtime_output = match stdout.trim().is_empty() {
        true => format!("(Light përfundoi pa stdout)\n{stderr}"),
        false => stdout,
    };
    let emotional = EmotionalCommand::from_runtime_output(trace_id, &runtime_output);
    let line = verified_nura_line(&runtime_output);
    Ok(NuraUiResponse {
        nura_verified: line.is_some(),
        nura_text: line.unwrap_or_default(),
        trace_id,
        emotional,
    })
}

fn evidence_kind(mime_type: &str, name: &str) -> u8 {
    let mime = mime_type.to_ascii_lowercase();
    let lower_name = name.to_ascii_lowercase();
    if mime.starts_with("image/") {
        PROJECT_EVIDENCE_IMAGE
    } else if mime.starts_with("video/") {
        PROJECT_EVIDENCE_VIDEO
    } else if mime.starts_with("text/")
        || mime == "application/pdf"
        || lower_name.ends_with(".md")
        || lower_name.ends_with(".doc")
        || lower_name.ends_with(".docx")
        || lower_name.ends_with(".txt")
        || lower_name.ends_with(".pdf")
    {
        PROJECT_EVIDENCE_DOCUMENT
    } else {
        PROJECT_EVIDENCE_UNKNOWN
    }
}

fn validate_project(request: &ProjectSubmission) -> Result<(), String> {
    validate_text("Emri i projektit", &request.project_name, 512, true)?;
    validate_text("Përshkrimi", &request.project_description, MAX_PROJECT_TEXT_BYTES, false)?;
    validate_text("Materiali i projektit", &request.project_content, MAX_PROJECT_TEXT_BYTES, true)?;
    validate_text("Domeni", &request.domain, 512, true)?;
    validate_text("Kodi i gjuhës", &request.language_code, 32, false)?;
    validate_text("Përmbledhja e evolucionit", &request.evolution_summary, MAX_PROJECT_TEXT_BYTES, false)?;
    validate_text("Hipoteza", &request.hypothesis, MAX_PROJECT_TEXT_BYTES, true)?;
    validate_text(
        "Përshkrimi i dokumentacionit",
        &request.documentation_description,
        MAX_PROJECT_TEXT_BYTES,
        true,
    )?;
    if request.assumptions.len() > 256 {
        return Err("Supozimet kalojnë kufirin 256".to_string());
    }
    for assumption in &request.assumptions {
        validate_text("Supozimi", assumption, 16 * 1024, false)?;
    }
    if request.files.len() > MAX_PROJECT_FILES {
        return Err(format!("Lejohen maksimumi {MAX_PROJECT_FILES} skedarë"));
    }
    let mut total = 0usize;
    for file in &request.files {
        validate_text("Emri i skedarit", &file.name, 512, true)?;
        if file.bytes.len() > MAX_PROJECT_FILE_BYTES {
            return Err(format!("Skedari '{}' kalon 8 MiB", file.name));
        }
        total = total
            .checked_add(file.bytes.len())
            .ok_or_else(|| "Madhësia totale e skedarëve overflow".to_string())?;
    }
    if total > MAX_PROJECT_TOTAL_FILE_BYTES {
        return Err("Skedarët së bashku kalojnë 24 MiB".to_string());
    }
    match request.route.as_str() {
        "workspace" | "scientific" => Ok(()),
        _ => Err("Rruga duhet të jetë workspace ose scientific".to_string()),
    }
}

fn handoff_paths() -> Result<(PathBuf, PathBuf), String> {
    let directory = std::env::temp_dir().join("essmai-ui-handoff-v167");
    fs::create_dir_all(&directory)
        .map_err(|e| format!("Dosja e handoff-it nuk u krijua: {e}"))?;
    let nonce = HANDOFF_COUNTER.fetch_add(1, Ordering::SeqCst);
    let stem = format!("{}-{nonce:016x}", std::process::id());
    Ok((
        directory.join(format!("project-request-{stem}.bin")),
        directory.join(format!("project-response-{stem}.bin")),
    ))
}

fn remove_quietly(path: &Path) {
    let _ = fs::remove_file(path);
}

#[tauri::command]
fn submit_project(request: ProjectSubmission) -> Result<ProjectSubmissionResponse, String> {
    validate_project(&request)?;
    let files = request
        .files
        .iter()
        .map(|file| ProjectEvidenceFileWire {
            kind: evidence_kind(&file.mime_type, &file.name),
            bytes: file.bytes.clone(),
        })
        .collect::<Vec<_>>();
    let wire = LightProjectIntakeRequestWire {
        project_name: request.project_name.trim().to_string(),
        project_description: request.project_description.trim().to_string(),
        project_content: request.project_content,
        domain: request.domain.trim().to_string(),
        lang_code: match request.language_code.trim().is_empty() {
            true => "sq".to_string(),
            false => request.language_code.trim().to_string(),
        },
        evolve_summary: request.evolution_summary.trim().to_string(),
        hypothesis: request.hypothesis.trim().to_string(),
        assumptions: request
            .assumptions
            .into_iter()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
            .collect(),
        documentation_description: request.documentation_description.trim().to_string(),
        files,
    };

    let request_bytes = shadow_contracts::encode_light_project_intake_request(&wire);
    let (request_path, response_path) = handoff_paths()?;
    fs::write(&request_path, request_bytes)
        .map_err(|e| format!("Project request nuk u shkrua: {e}"))?;
    remove_quietly(&response_path);

    let light = find_light()?;
    let flag = match request.route.as_str() {
        "workspace" => "--project-route-once",
        "scientific" => "--project-route-legacy-once",
        _ => unreachable!("validated route"),
    };
    let status = Command::new(&light)
        .arg(flag)
        .arg(&request_path)
        .arg(&response_path)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();
    remove_quietly(&request_path);
    let output = status.map_err(|e| format!("Light project route nuk u nis: {e}"))?;
    if !output.status.success() {
        remove_quietly(&response_path);
        return Err(format!(
            "Light project route dështoi code={:?}:\n{}\n{}",
            output.status.code(),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        ));
    }
    let response_bytes = fs::read(&response_path)
        .map_err(|e| format!("Light nuk prodhoi project response: {e}"))?;
    remove_quietly(&response_path);
    let response = shadow_contracts::decode_light_project_intake_response(&response_bytes)
        .map_err(|e| format!("Project response u refuzua: {e}"))?;

    let (project_id, trace_id, revision, context_sha256, content_sha256) = response
        .witness
        .as_ref()
        .map(|witness| {
            (
                Some(witness.project_id),
                Some(witness.trace_id),
                Some(witness.revision),
                Some(witness.context_sha256.clone()),
                Some(witness.content_sha256.clone()),
            )
        })
        .unwrap_or((None, None, None, None, None));
    let accepted = response.accepted_into_gcl && response.witness.is_some();
    Ok(ProjectSubmissionResponse {
        accepted,
        reason_code: response.reason_code,
        route: request.route,
        project_id,
        trace_id,
        revision,
        context_sha256,
        content_sha256,
        message: match accepted {
            true => "Light pranoi materialin dhe mbylli rrugën e zgjedhur pa autoritet UI".to_string(),
            false => format!("Light refuzoi projektin reason={}", response.reason_code),
        },
    })
}

fn main() {
    if let Err(error) = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            runtime_status,
            ask_nura,
            submit_project
        ])
        .run(tauri::generate_context!())
    {
        eprintln!("Nura Legacy Tauri 2 dështoi: {error}");
    }
}
