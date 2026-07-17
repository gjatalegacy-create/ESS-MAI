//! v1.6.5 — orientimi i izoluar i Project Workspace në Quantum.
//!
//! Kjo portë pranon vetëm një `ScientificProjectInput` të verifikuar nga
//! Light/APUPK dhe prodhon orientim deterministik për:
//! - magazinën e projektit;
//! - bisedën e projektit;
//! - ose të dyja bashkë.
//!
//! Kufiri është i qëllimshëm: moduli nuk ruan skedarë, nuk jep verdict,
//! nuk aktivizon TRL/PD dhe nuk hap/krijon capability token. Rruga ekzistuese
//! `--project-process-once` mbetet rruga legacy e procedimit shkencor GCL.

use sha2::{Digest, Sha256};

pub const PROJECT_WORKSPACE_VERSION: u32 = 0x0001_0605;
pub const PROJECT_WORKSPACE_DOMAIN: &[u8] = b"ESS_MAI_QUANTUM_PROJECT_WORKSPACE_V165";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ProjectWorkspaceRoute {
    Storage = 1,
    Conversation = 2,
    StorageAndConversation = 3,
}

impl ProjectWorkspaceRoute {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Storage => "PROJECT_STORAGE",
            Self::Conversation => "PROJECT_CONVERSATION",
            Self::StorageAndConversation => "PROJECT_STORAGE_AND_CONVERSATION",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ProjectWorkspaceMaterial<'a> {
    pub project_id: u64,
    pub project_trace_id: u64,
    pub project_context_sha256: &'a str,
    pub request_sha256: &'a str,
    pub domain: &'a str,
    pub turn_text: &'a str,
    pub title: &'a str,
    pub hypothesis: &'a str,
    pub assumptions: &'a [String],
    pub description: &'a str,
    pub files: &'a [(u8, Vec<u8>)],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectWorkspaceOrientation {
    pub route: ProjectWorkspaceRoute,
    pub project_id: u64,
    pub project_trace_id: u64,
    pub project_context_sha256: String,
    pub request_sha256: String,
    pub workspace_sha256: String,
    pub material_sha256: String,
    pub conversation_turn_sha256: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectWorkspaceError {
    MissingProjectIdentity,
    InvalidProjectContextSha256,
    InvalidRequestSha256,
    MissingProjectMaterial,
}

impl ProjectWorkspaceOrientation {
    /// Wire i përgjigjes përmban vetëm identitete dhe orientim. Nuk ka token,
    /// receipt ose verdict; `authority=NONE` është deklarim i kufirit.
    pub fn encode_line(&self) -> String {
        format!(
            "ESSMAI_Q_PROJECT_WORKSPACE_V165|route={}|project_id={:016x}|project_trace_id={:016x}|project_context_sha256={}|request_sha256={}|workspace_sha256={}|material_sha256={}|conversation_turn_sha256={}|legacy_route=--project-process-once|authority=NONE|token_policy=UNCHANGED\n",
            self.route.label(),
            self.project_id,
            self.project_trace_id,
            self.project_context_sha256,
            self.request_sha256,
            self.workspace_sha256,
            self.material_sha256,
            self.conversation_turn_sha256,
        )
    }
}

pub fn orient(
    material: ProjectWorkspaceMaterial<'_>,
) -> Result<ProjectWorkspaceOrientation, ProjectWorkspaceError> {
    match material.project_id != 0 && material.project_trace_id != 0 {
        true => {}
        false => return Err(ProjectWorkspaceError::MissingProjectIdentity),
    }
    match is_sha256(material.project_context_sha256) {
        true => {}
        false => return Err(ProjectWorkspaceError::InvalidProjectContextSha256),
    }
    match is_sha256(material.request_sha256) {
        true => {}
        false => return Err(ProjectWorkspaceError::InvalidRequestSha256),
    }

    let has_material = !material.title.trim().is_empty()
        || !material.hypothesis.trim().is_empty()
        || !material.description.trim().is_empty()
        || !material.assumptions.is_empty()
        || !material.files.is_empty();
    let has_turn = !material.turn_text.trim().is_empty();
    match has_material || has_turn {
        true => {}
        false => return Err(ProjectWorkspaceError::MissingProjectMaterial),
    }

    let route = route_from_domain_and_shape(material.domain, has_material, has_turn);
    let workspace_sha256 = workspace_sha256(
        material.project_id,
        material.project_trace_id,
        material.project_context_sha256,
    );
    let material_sha256 = project_material_sha256(&material);
    let conversation_turn_sha256 = conversation_turn_sha256(&material, &workspace_sha256);

    Ok(ProjectWorkspaceOrientation {
        route,
        project_id: material.project_id,
        project_trace_id: material.project_trace_id,
        project_context_sha256: material.project_context_sha256.to_ascii_lowercase(),
        request_sha256: material.request_sha256.to_ascii_lowercase(),
        workspace_sha256,
        material_sha256,
        conversation_turn_sha256,
    })
}

fn route_from_domain_and_shape(
    domain: &str,
    has_material: bool,
    has_turn: bool,
) -> ProjectWorkspaceRoute {
    let normalized = domain.trim().to_ascii_lowercase();
    let explicit_storage = matches!(
        normalized.as_str(),
        "project-storage" | "project_store" | "project:store"
    );
    let explicit_conversation = matches!(
        normalized.as_str(),
        "project-conversation" | "project-chat" | "project:chat"
    );
    let explicit_both = matches!(
        normalized.as_str(),
        "project-workspace" | "project-storage-chat" | "project:workspace"
    );

    match (
        explicit_both,
        explicit_storage,
        explicit_conversation,
        has_material,
        has_turn,
    ) {
        (true, _, _, _, _) => ProjectWorkspaceRoute::StorageAndConversation,
        (false, true, _, _, _) => ProjectWorkspaceRoute::Storage,
        (false, false, true, _, _) => ProjectWorkspaceRoute::Conversation,
        (false, false, false, true, true) => ProjectWorkspaceRoute::StorageAndConversation,
        (false, false, false, true, false) => ProjectWorkspaceRoute::Storage,
        (false, false, false, false, true) => ProjectWorkspaceRoute::Conversation,
        (false, false, false, false, false) => ProjectWorkspaceRoute::Conversation,
    }
}

fn workspace_sha256(
    project_id: u64,
    project_trace_id: u64,
    project_context_sha256: &str,
) -> String {
    let mut h = Sha256::new();
    h.update(PROJECT_WORKSPACE_DOMAIN);
    h.update(PROJECT_WORKSPACE_VERSION.to_le_bytes());
    h.update(b"WORKSPACE_ID");
    h.update(project_id.to_le_bytes());
    h.update(project_trace_id.to_le_bytes());
    update_text(&mut h, &project_context_sha256.to_ascii_lowercase());
    hex(&h.finalize())
}

fn project_material_sha256(material: &ProjectWorkspaceMaterial<'_>) -> String {
    let mut h = Sha256::new();
    h.update(PROJECT_WORKSPACE_DOMAIN);
    h.update(PROJECT_WORKSPACE_VERSION.to_le_bytes());
    h.update(b"PROJECT_MATERIAL");
    update_text(&mut h, material.title);
    update_text(&mut h, material.hypothesis);
    h.update((material.assumptions.len() as u64).to_le_bytes());
    for assumption in material.assumptions {
        update_text(&mut h, assumption);
    }
    update_text(&mut h, material.description);
    h.update((material.files.len() as u64).to_le_bytes());
    for (kind, bytes) in material.files {
        h.update([*kind]);
        h.update((bytes.len() as u64).to_le_bytes());
        h.update(bytes);
    }
    hex(&h.finalize())
}

fn conversation_turn_sha256(
    material: &ProjectWorkspaceMaterial<'_>,
    workspace_sha256: &str,
) -> String {
    let mut h = Sha256::new();
    h.update(PROJECT_WORKSPACE_DOMAIN);
    h.update(PROJECT_WORKSPACE_VERSION.to_le_bytes());
    h.update(b"PROJECT_CONVERSATION_TURN");
    update_text(&mut h, workspace_sha256);
    update_text(&mut h, material.domain);
    update_text(&mut h, material.turn_text);
    hex(&h.finalize())
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn update_text(h: &mut Sha256, value: &str) {
    h.update((value.len() as u64).to_le_bytes());
    h.update(value.as_bytes());
}

fn hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0F) as usize] as char);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn material<'a>(domain: &'a str, turn_text: &'a str) -> ProjectWorkspaceMaterial<'a> {
        ProjectWorkspaceMaterial {
            project_id: 77,
            project_trace_id: 88,
            project_context_sha256: "aa11aa11aa11aa11aa11aa11aa11aa11aa11aa11aa11aa11aa11aa11aa11",
            request_sha256: "bb22bb22bb22bb22bb22bb22bb22bb22bb22bb22bb22bb22bb22bb22bb22",
            domain,
            turn_text,
            title: "Project NURA",
            hypothesis: "A mund të mbahet konteksti i projektit i ndarë?",
            assumptions: &[],
            description: "Material i projektit",
            files: &[],
        }
    }

    #[test]
    fn project_workspace_orients_without_authority_fields() {
        let result = orient(material("project-workspace", "vazhdo bisedën"))
            .expect("project workspace duhet të orientohet");
        assert_eq!(result.route, ProjectWorkspaceRoute::StorageAndConversation);
        let line = result.encode_line();
        assert!(line.contains("authority=NONE"));
        assert!(line.contains("token_policy=UNCHANGED"));
        assert!(line.contains("legacy_route=--project-process-once"));
        assert!(!line.to_ascii_lowercase().contains("lgctoken"));
    }

    #[test]
    fn explicit_chat_stays_project_conversation_only() {
        let result = orient(material("project-chat", "çfarë ndryshoi në projekt?"))
            .expect("project chat duhet të orientohet");
        assert_eq!(result.route, ProjectWorkspaceRoute::Conversation);
    }

    #[test]
    fn same_input_produces_same_record_hashes() {
        let first = orient(material("project-storage", "")).expect("first");
        let second = orient(material("project-storage", "")).expect("second");
        assert_eq!(first.workspace_sha256, second.workspace_sha256);
        assert_eq!(first.material_sha256, second.material_sha256);
        assert_eq!(first.conversation_turn_sha256, second.conversation_turn_sha256);
    }
}
