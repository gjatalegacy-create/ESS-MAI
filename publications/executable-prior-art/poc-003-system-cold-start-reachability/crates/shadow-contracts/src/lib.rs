//! ESS-MAI Shadow process contracts — public form, zero sovereign authority.
//!
//! This crate intentionally contains only transport shapes and deterministic
//! codecs. It cannot construct `Shadow`, open the sovereign vault, issue an
//! LgcToken, seal a verdict, or produce a `VerificationReceipt`. Those actions
//! exist only inside the `shadow_platform` executable.

use sha2::{Digest, Sha256};
use std::fmt;

pub mod negative_asset;

pub const PROTOCOL_VERSION: u16 = 10;
pub const MASS_SCALE: u32 = 10_000;
pub const MPRO_EVALUATIONS: usize = 16;
pub const REQUIRED_ACTION_MASK: u64 = 0x0000_0000_0000_03FE;
pub const ACTION_STAGE_HPRO: u8 = 1;
pub const ACTION_STAGE_PRO: u8 = 2;
pub const ACTION_STAGE_NPRO: u8 = 3;
pub const ACTION_STAGE_SRK: u8 = 4;
pub const ACTION_STAGE_APRO: u8 = 5;
pub const ACTION_STAGE_MPRO: u8 = 6;
pub const ACTION_STAGE_PIM: u8 = 7;
pub const ACTION_STAGE_NPIM: u8 = 8;
pub const ACTION_STAGE_HCP: u8 = 9;
pub const REQUIRED_ACTION_ORDER: [u8; 9] = [
    ACTION_STAGE_HPRO,
    ACTION_STAGE_PRO,
    ACTION_STAGE_NPRO,
    ACTION_STAGE_NPIM,
    ACTION_STAGE_SRK,
    ACTION_STAGE_PIM,
    ACTION_STAGE_APRO,
    ACTION_STAGE_MPRO,
    ACTION_STAGE_HCP,
];

fn expected_action_word_count(stage: u8) -> Option<usize> {
    match stage {
        ACTION_STAGE_HPRO => Some(5),
        ACTION_STAGE_PRO => Some(3),
        ACTION_STAGE_NPRO => Some(4),
        ACTION_STAGE_SRK => Some(6),
        ACTION_STAGE_APRO => Some(4),
        ACTION_STAGE_MPRO => Some(21),
        ACTION_STAGE_PIM => Some(6),
        ACTION_STAGE_NPIM => Some(5),
        ACTION_STAGE_HCP => Some(7),
        _ => None,
    }
}
pub const MAX_FRAME_BYTES: usize = 64 * 1024 * 1024;
pub const MAX_FIELD_BYTES: usize = 16 * 1024 * 1024;
pub const MAX_VECTOR_ITEMS: usize = 1_000_000;

const MAGIC: [u8; 8] = *b"ESSMSW01";
const KIND_CYCLE_REQUEST: u8 = 1;
const KIND_CYCLE_RESPONSE: u8 = 2;
const KIND_NEGATIVE_REQUEST: u8 = 3;
const KIND_NEGATIVE_RESPONSE: u8 = 4;
const KIND_PROJECT_REGISTER_REQUEST: u8 = 5;
const KIND_PROJECT_REGISTER_RESPONSE: u8 = 6;
const KIND_LIGHT_PROJECT_INTAKE_REQUEST: u8 = 7;
const KIND_LIGHT_PROJECT_INTAKE_RESPONSE: u8 = 8;
const KIND_QUANTUM_PROJECT_EXEC_REQUEST: u8 = 9;
const KIND_QUANTUM_PROJECT_EXEC_RESPONSE: u8 = 10;
const KIND_SELECTION_REQUEST: u8 = 11;
const KIND_SELECTION_RESPONSE: u8 = 12;

pub const SELECTION_MAX_TERMS: usize = 256;
pub const SELECTION_MAX_CANDIDATES: u16 = 128;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionRequester {
    BesaNlight = 1,
    AshtQuantum = 2,
}

impl SelectionRequester {
    fn decode(value: u8) -> Result<Self, WireError> {
        match value {
            1 => Ok(Self::BesaNlight),
            2 => Ok(Self::AshtQuantum),
            actual => Err(WireError::InvalidEnum {
                field: "selection_requester",
                actual,
            }),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionPurpose {
    XiYiStatistics = 1,
    EvidenceCandidates = 2,
}

impl SelectionPurpose {
    fn decode(value: u8) -> Result<Self, WireError> {
        match value {
            1 => Ok(Self::XiYiStatistics),
            2 => Ok(Self::EvidenceCandidates),
            actual => Err(WireError::InvalidEnum {
                field: "selection_purpose",
                actual,
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KnowledgeSelectionRequestWire {
    pub version: u16,
    pub requester: SelectionRequester,
    pub purpose: SelectionPurpose,
    pub session_id: String,
    pub parent_i0: String,
    pub primitive_anchor: u64,
    pub trace_id: String,
    pub split_sha256: String,
    pub xi: Vec<String>,
    pub yi: Vec<String>,
    pub upstream_hold_sha256: Option<String>,
    pub evidence_sha256: String,
    pub max_positive: u16,
    pub max_negative: u16,
}

impl KnowledgeSelectionRequestWire {
    pub fn shape_is_valid(&self) -> bool {
        let requester_purpose = matches!(
            (self.requester, self.purpose),
            (
                SelectionRequester::BesaNlight,
                SelectionPurpose::XiYiStatistics
            ) | (
                SelectionRequester::AshtQuantum,
                SelectionPurpose::EvidenceCandidates
            )
        );
        let upstream_shape = match self.requester {
            SelectionRequester::BesaNlight => self.upstream_hold_sha256.is_none(),
            SelectionRequester::AshtQuantum => self
                .upstream_hold_sha256
                .as_deref()
                .map(is_canonical_sha256)
                .unwrap_or(false),
        };
        self.version == PROTOCOL_VERSION
            && requester_purpose
            && !self.session_id.is_empty()
            && !self.parent_i0.is_empty()
            && self.primitive_anchor != 0
            && !self.trace_id.is_empty()
            && is_canonical_sha256(&self.split_sha256)
            && is_canonical_sha256(&self.evidence_sha256)
            && upstream_shape
            && self.xi.len().saturating_add(self.yi.len()) > 0
            && self.xi.len().saturating_add(self.yi.len()) <= SELECTION_MAX_TERMS
            && self
                .xi
                .iter()
                .chain(self.yi.iter())
                .all(|term| !term.trim().is_empty())
            && (1..=SELECTION_MAX_CANDIDATES).contains(&self.max_positive)
            && (1..=SELECTION_MAX_CANDIDATES).contains(&self.max_negative)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandidateStatisticWire {
    pub candidate_id: u64,
    pub polarity: u8,
    pub domain: String,
    pub term_sha256: String,
    pub source_sha256: String,
    pub security_y: u8,
    pub trust_x: u8,
    pub confidence_fixed: u32,
    pub frequency: u32,
    pub domain_count: u16,
    pub legacy_score_fixed: u32,
}

impl CandidateStatisticWire {
    pub fn shape_is_valid(&self) -> bool {
        self.candidate_id != 0
            && matches!(self.polarity, 0 | 1)
            && !self.domain.trim().is_empty()
            && is_canonical_sha256(&self.term_sha256)
            && is_canonical_sha256(&self.source_sha256)
            && self.security_y <= 1
            && self.trust_x <= 1
            && self.confidence_fixed <= MASS_SCALE
            && self.legacy_score_fixed <= MASS_SCALE
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KnowledgeSelectionResponseWire {
    pub version: u16,
    pub requester: SelectionRequester,
    pub session_id: String,
    pub parent_i0: String,
    pub primitive_anchor: u64,
    pub split_sha256: String,
    pub upstream_hold_sha256: Option<String>,
    pub request_sha256: String,
    pub selection_sha256: String,
    pub hold_id: String,
    pub hold_sha256: String,
    pub positive: Vec<CandidateStatisticWire>,
    pub negative: Vec<CandidateStatisticWire>,
    pub law_seal: u64,
    pub system_laws_seal: u64,
    pub expires_at_ns: u64,
}

impl KnowledgeSelectionResponseWire {
    pub fn shape_is_valid_for(&self, request: &KnowledgeSelectionRequestWire) -> bool {
        self.version == PROTOCOL_VERSION
            && request.shape_is_valid()
            && self.requester == request.requester
            && self.session_id == request.session_id
            && self.parent_i0 == request.parent_i0
            && self.primitive_anchor == request.primitive_anchor
            && self.split_sha256 == request.split_sha256
            && self.upstream_hold_sha256 == request.upstream_hold_sha256
            && is_canonical_sha256(&self.request_sha256)
            && is_canonical_sha256(&self.selection_sha256)
            && !self.hold_id.is_empty()
            && is_canonical_sha256(&self.hold_sha256)
            && self.positive.len() <= usize::from(request.max_positive)
            && self.negative.len() <= usize::from(request.max_negative)
            && self
                .positive
                .iter()
                .all(CandidateStatisticWire::shape_is_valid)
            && self
                .negative
                .iter()
                .all(CandidateStatisticWire::shape_is_valid)
            && self
                .positive
                .iter()
                .all(|candidate| candidate.polarity == 1)
            && self
                .negative
                .iter()
                .all(|candidate| candidate.polarity == 0)
            && self.law_seal != 0
            && self.system_laws_seal != 0
            && self.expires_at_ns != 0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MuscleEvidenceWire {
    pub besa_hold_id: String,
    pub besa_hold_sha256: String,
    pub besa_selection_sha256: String,
    pub besa_attestation_sha256: String,
    pub asht_hold_id: String,
    pub asht_hold_sha256: String,
    pub asht_selection_sha256: String,
    pub asht_attestation_sha256: String,
    pub split_sha256: String,
    pub combined_sha256: String,
}

impl MuscleEvidenceWire {
    pub fn recompute_combined_sha256(&self) -> String {
        sha256_hex(&[
            b"ESSMAI/MUSCLE/EVIDENCE/V178".as_slice(),
            self.besa_hold_id.as_bytes(),
            self.besa_hold_sha256.as_bytes(),
            self.besa_selection_sha256.as_bytes(),
            self.besa_attestation_sha256.as_bytes(),
            self.asht_hold_id.as_bytes(),
            self.asht_hold_sha256.as_bytes(),
            self.asht_selection_sha256.as_bytes(),
            self.asht_attestation_sha256.as_bytes(),
            self.split_sha256.as_bytes(),
        ])
    }

    pub fn verifies_internal(&self) -> bool {
        !self.besa_hold_id.is_empty()
            && !self.asht_hold_id.is_empty()
            && [
                self.besa_hold_sha256.as_str(),
                self.besa_selection_sha256.as_str(),
                self.besa_attestation_sha256.as_str(),
                self.asht_hold_sha256.as_str(),
                self.asht_selection_sha256.as_str(),
                self.asht_attestation_sha256.as_str(),
                self.split_sha256.as_str(),
                self.combined_sha256.as_str(),
            ]
            .iter()
            .all(|value| is_canonical_sha256(value))
            && self.combined_sha256 == self.recompute_combined_sha256()
    }
}

pub fn sha256_hex(parts: &[&[u8]]) -> String {
    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update((part.len() as u64).to_le_bytes());
        hasher.update(part);
    }
    format!("{:x}", hasher.finalize())
}

pub fn is_canonical_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

// ════════════════════════════════════════════════════════════════════════════
// GCL SCIENTIFIC PROJECT CONTINUUM — Light/APUPK → Shadow context → Quantum
// scientific processing → Shadow supreme verdict. Kjo nuk është rrjedhë
// paralele: evidenca udhëton brenda FinalEvidenceWire dhe vuloset nga i njëjti
// GCL/Living Trust.
// ════════════════════════════════════════════════════════════════════════════

pub const PROJECT_EVIDENCE_UNKNOWN: u8 = 0;
pub const PROJECT_EVIDENCE_IMAGE: u8 = 1;
pub const PROJECT_EVIDENCE_VIDEO: u8 = 2;
pub const PROJECT_EVIDENCE_DOCUMENT: u8 = 3;

/// Quantum/Digital Lab may propose evidence only through TRL 3.
/// TRL 4 is never accepted on the inbound wire; it is born only from the
/// multi-stage Shadow factualization under the same GCL process.
pub const QUANTUM_MAX_TRL: u8 = 3;
/// Shadow-only factual level after GCL binding, evidence verification,
/// sovereign (Y=1,X=1) confirmation and Novel proof.
pub const SHADOW_FACTUAL_TRL: u8 = 4;

pub const PROJECT_STATUS_NONE: u8 = 0;
pub const PROJECT_STATUS_UNDER_GCL: u8 = 1;
pub const PROJECT_STATUS_HOLD: u8 = 2;
pub const PROJECT_STATUS_RIGOROUS_NEGATIVE: u8 = 3;
pub const PROJECT_STATUS_NOVEL_FACTUAL: u8 = 4;

pub const NOVEL_REJECTION_NONE: u8 = 0;
pub const NOVEL_REJECTION_NO_VISUAL_PROOF: u8 = 1;
pub const NOVEL_REJECTION_DESCRIPTION_TOO_SHORT: u8 = 2;
pub const NOVEL_REJECTION_UNKNOWN_FORMATS: u8 = 3;
pub const NOVEL_REJECTION_TRL_NOT_READY: u8 = 4;
pub const NOVEL_REJECTION_SOVEREIGN_PAIR_NOT_VERIFIED: u8 = 5;
pub const NOVEL_REJECTION_TRACE_REJECTED: u8 = 6;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectContextWitnessWire {
    pub project_id: u64,
    pub user_id: u64,
    pub trace_id: u64,
    pub revision: u32,
    pub content_sha256: String,
    pub light_sovereign_flags: u32,
    /// SHA-256 kanonik i kontekstit, prodhuar nga Shadow main pasi APUPK u ruajt.
    pub context_sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectEvidenceFileWire {
    pub kind: u8,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScientificProjectWire {
    pub context: ProjectContextWitnessWire,
    /// Materiali kanonik i pyetjes që Quantum procedoi; Shadow e lidh me
    /// evidencën që rezultati TRL të mos shkëputet nga hipoteza e përdoruesit.
    pub project_title: String,
    pub domain: String,
    pub hypothesis: String,
    pub assumptions: Vec<String>,
    /// Duhet të jetë i njëjti GCL process digest i PD/Spine 9.
    pub gcl_process_digest: u64,
    pub trl_level: u8,
    pub trl_passed: bool,
    pub trl_confidence: u32,
    pub trl_reproducibility: u32,
    pub lab_test_id: String,
    pub findings: String,
    pub description: String,
    pub files: Vec<ProjectEvidenceFileWire>,
    /// SHA-256 i të gjithë materialit të projektit nën GCL.
    pub evidence_sha256: String,
}

impl ScientificProjectWire {
    pub fn shape_is_valid(&self) -> bool {
        self.context.project_id != 0
            && self.context.user_id != 0
            && self.context.trace_id != 0
            && self.context.content_sha256.len() == 64
            && self
                .context
                .content_sha256
                .chars()
                .all(|c| c.is_ascii_hexdigit())
            && self.context.context_sha256.len() == 64
            && self
                .context
                .context_sha256
                .chars()
                .all(|c| c.is_ascii_hexdigit())
            && !self.project_title.trim().is_empty()
            && !self.domain.trim().is_empty()
            && !self.hypothesis.trim().is_empty()
            && self.assumptions.len() <= 4_096
            && self.gcl_process_digest != 0
            && self.trl_level <= QUANTUM_MAX_TRL
            && (!self.trl_passed || self.trl_level > 0)
            && self.trl_confidence <= MASS_SCALE
            && self.trl_reproducibility <= MASS_SCALE
            && self.evidence_sha256.len() == 64
            && self.evidence_sha256.chars().all(|c| c.is_ascii_hexdigit())
            && self.files.len() <= 4_096
            && self.files.iter().all(|f| {
                matches!(
                    f.kind,
                    PROJECT_EVIDENCE_UNKNOWN
                        | PROJECT_EVIDENCE_IMAGE
                        | PROJECT_EVIDENCE_VIDEO
                        | PROJECT_EVIDENCE_DOCUMENT
                )
            })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProjectRegistrationRequestWire {
    pub project_id: u64,
    pub user_id: u64,
    pub trace_id: u64,
    pub project_title: String,
    pub initial_trace: String,
    pub project_content: String,
    pub progress_percentage: f32,
    pub last_update_ns: u64,
    pub notes: String,
    pub light_sovereign_flags: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectRegistrationResponseWire {
    pub accepted: bool,
    pub reason_code: u8,
    pub witness: Option<ProjectContextWitnessWire>,
}

/// Kërkesa e procesit Light për hyrjen e një projekti përdoruesi në GCL.
/// Kjo është portë intake; nuk përmban dhe nuk pretendon verdikt Novel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LightProjectIntakeRequestWire {
    pub project_name: String,
    pub project_description: String,
    pub project_content: String,
    pub domain: String,
    pub lang_code: String,
    pub evolve_summary: String,
    pub hypothesis: String,
    pub assumptions: Vec<String>,
    pub documentation_description: String,
    pub files: Vec<ProjectEvidenceFileWire>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LightProjectIntakeResponseWire {
    pub accepted_into_gcl: bool,
    pub reason_code: u8,
    pub witness: Option<ProjectContextWitnessWire>,
}

/// Kërkesë process-bound Light→Quantum për projekte që tejkalojnë kufirin
/// 2048-byte të bus-it legacy. Payload-i është forma ekzistuese QuantumInput,
/// por transportohet në frame të versionuar dhe lidhet me identitetin APUPK.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuantumProjectExecutionRequestWire {
    pub project_id: u64,
    pub project_trace_id: u64,
    pub project_context_sha256: String,
    pub payload_sha256: String,
    pub payload: String,
}

/// Dëshmi minimale se procesi Quantum përfundoi rrjedhën e plotë. Novel/hold/
/// negative nuk shpallen këtu; ato mbeten në SupremeVerdict/PD handoff.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuantumProjectExecutionResponseWire {
    pub completed: bool,
    pub reason_code: u8,
    pub project_id: u64,
    pub project_trace_id: u64,
    /// SHA-256 i frame-it të kërkesës; mbyll stale/swap të response-it.
    pub request_sha256: String,
}

/// Dëshmia e një Layer-i të Spine 9. Shadow e rillogarit receipt-in; kështu
/// Layers nuk paraqiten si autoritet më vete, por si thellim i të njëjtit GCL.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PdLayerEvidenceWire {
    pub layer_mask: u8,
    pub activation_id: u64,
    pub activation_contract_digest: u64,
    pub gcl_process_digest: u64,
    pub parent_result_digest: u64,
    pub result_material: Vec<u8>,
    pub result_material_digest: u64,
    pub result_digest: u64,
    pub completed: bool,
}

impl PdLayerEvidenceWire {
    pub fn recompute_material_digest(&self) -> u64 {
        digest_parts(&[self.result_material.as_slice()])
    }

    pub fn recompute_digest(&self) -> u64 {
        digest_parts(&[
            &self.layer_mask.to_le_bytes(),
            &self.activation_id.to_le_bytes(),
            &self.activation_contract_digest.to_le_bytes(),
            &self.gcl_process_digest.to_le_bytes(),
            &self.parent_result_digest.to_le_bytes(),
            &self.result_material_digest.to_le_bytes(),
            &[self.completed as u8],
            b"PD_LAYER_RECEIPT_UNDER_GCL_V160",
        ])
    }

    pub fn verifies_for(
        &self,
        expected_layer: u8,
        activation_id: u64,
        activation_contract_digest: u64,
        gcl_process_digest: u64,
        expected_parent: u64,
    ) -> bool {
        self.completed
            && self.layer_mask == expected_layer
            && self.activation_id == activation_id
            && self.activation_contract_digest == activation_contract_digest
            && self.gcl_process_digest == gcl_process_digest
            && self.parent_result_digest == expected_parent
            && !self.result_material.is_empty()
            && self.result_material_digest == self.recompute_material_digest()
            && self.result_digest == self.recompute_digest()
    }
}

/// Materiali i plotë i `i + U → i₀ → i₀ + 1Q → Spine 9`.
/// Shadow rillogarit stimulus-in, incrementin dhe activation-in; nuk i beson
/// vetëm `continuum_activation_digest` të ardhur nga Quantum.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PdContinuumEvidenceWire {
    pub contract_version: u32,
    pub session_digest: u64,
    pub initial_i_digest: u64,
    pub user_input_digest: u64,
    pub i0_digest: u64,
    pub stimulus_digest: u64,
    pub stimulus_created_at: u64,
    pub stimulus_state: u8,
    pub unit_count: u32,
    pub question_digest: u64,
    pub response_kind_digest: u64,
    pub increment_digest: u64,
    pub increment_created_at: u64,
    pub increment_state: u8,
    pub activation_digest: u64,
    pub activation_state: u8,
}

impl PdContinuumEvidenceWire {
    pub const CONTRACT_VERSION: u32 = 0x0001_0601;
    pub const QUESTION_INCREMENT_UNITS: u32 = 1;

    pub fn recompute_stimulus_digest(&self) -> u64 {
        digest_parts(&[
            &self.contract_version.to_le_bytes(),
            &self.session_digest.to_le_bytes(),
            &self.initial_i_digest.to_le_bytes(),
            &self.user_input_digest.to_le_bytes(),
            &self.i0_digest.to_le_bytes(),
            &self.stimulus_created_at.to_le_bytes(),
            b"I_PLUS_U_TO_I0",
        ])
    }

    pub fn recompute_increment_digest(&self) -> u64 {
        digest_parts(&[
            &self.contract_version.to_le_bytes(),
            &self.stimulus_digest.to_le_bytes(),
            &self.unit_count.to_le_bytes(),
            &self.question_digest.to_le_bytes(),
            &self.response_kind_digest.to_le_bytes(),
            &self.increment_created_at.to_le_bytes(),
            b"I0_PLUS_ONE_QUESTION_IQ",
        ])
    }

    pub fn recompute_activation_digest(&self) -> u64 {
        digest_parts(&[
            &self.contract_version.to_le_bytes(),
            &self.stimulus_digest.to_le_bytes(),
            &self.increment_digest.to_le_bytes(),
            b"PD_CONTINUUM_ACTIVATION",
        ])
    }

    pub fn verifies_internal(&self) -> bool {
        self.contract_version == Self::CONTRACT_VERSION
            && self.session_digest != 0
            && self.initial_i_digest != 0
            && self.user_input_digest != 0
            && self.i0_digest != 0
            && self.stimulus_digest == self.recompute_stimulus_digest()
            && self.stimulus_created_at == self.increment_created_at
            && self.stimulus_state == 0
            && self.unit_count == Self::QUESTION_INCREMENT_UNITS
            && self.question_digest != 0
            && self.response_kind_digest != 0
            && self.increment_digest == self.recompute_increment_digest()
            && self.increment_state == 1
            && self.activation_digest == self.recompute_activation_digest()
            && self.activation_state == 2
    }
}

/// Materiali i kontratës njohëse që PD përdor për të aktivizuar të gjithë
/// Spine 9. Mode ndryshon karakterin e thellimit; maska mbetet gjithmonë 111.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PdActivationEvidenceWire {
    pub mode_code: u8,
    pub origin_code: u8,
    pub response_kind_digest: u64,
    pub genius_signal_digest: u64,
    pub trace_mass: u32,
    pub structural_coherence: u32,
    pub required_layer_mask: u8,
    pub contract_digest: u64,
}

impl PdActivationEvidenceWire {
    fn mode_label(&self) -> Option<&'static [u8]> {
        match self.mode_code {
            0 => Some(b"INTELLECT"),
            1 => Some(b"PHILOSOPHY"),
            2 => Some(b"SCIENTIFIC"),
            3 => Some(b"NOVEL"),
            _ => None,
        }
    }

    fn origin_label(&self) -> Option<&'static [u8]> {
        match self.origin_code {
            0 => Some(b"NORMAL_TURN"),
            1 => Some(b"GENIUS_AUTONOMOUS_RESEARCH"),
            2 => Some(b"BUDGET_EXHAUSTED"),
            3 => Some(b"SESSION_CLOSED"),
            _ => None,
        }
    }

    pub fn recompute_contract_digest(&self) -> Option<u64> {
        let mode = self.mode_label()?;
        let origin = self.origin_label()?;
        Some(digest_parts(&[
            &PdSpineEvidenceWire::CONTRACT_VERSION.to_le_bytes(),
            mode,
            origin,
            &self.response_kind_digest.to_le_bytes(),
            &self.genius_signal_digest.to_le_bytes(),
            &self.trace_mass.to_le_bytes(),
            &self.structural_coherence.to_le_bytes(),
            &[self.required_layer_mask],
            b"PD_ACTIVATION_CONTRACT_V160",
        ]))
    }

    pub fn verifies_internal(&self) -> bool {
        let genius_origin_ok = match self.origin_code {
            1 => self.mode_code == 3 && self.genius_signal_digest != 0,
            0 | 2 | 3 => true,
            _ => false,
        };
        self.trace_mass <= MASS_SCALE
            && self.structural_coherence <= MASS_SCALE
            && self.required_layer_mask == PdSpineEvidenceWire::ALL_LAYERS_MASK
            && genius_origin_ok
            && self.recompute_contract_digest() == Some(self.contract_digest)
    }
}

/// Dëshmia e plotë e procesit GCL→PD→Spine9→Layer1→2→3. Ajo përmban
/// material të mjaftueshëm që Shadow ta rillogarisë procesin GCL, aktivizimin,
/// tre receipt-et dhe completion-in, jo vetëm t'i besojë digest-eve të Quantum.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PdSpineEvidenceWire {
    pub contract_version: u32,
    pub activation_id: u64,
    pub session_digest: u64,
    pub parent_i0_digest: u64,
    pub trace_digest: u64,
    pub turn_digest: u64,
    pub created_at: u64,
    pub continuum: PdContinuumEvidenceWire,
    pub activation: PdActivationEvidenceWire,
    pub continuum_activation_digest: u64,
    pub activation_contract_digest: u64,
    pub gcl_law_seal: u64,
    pub gcl_system_laws_seal: u64,
    pub gcl_phase_digest: u64,
    pub gcl_process_digest: u64,
    pub layer_mask: u8,
    pub layer1: PdLayerEvidenceWire,
    pub layer2: PdLayerEvidenceWire,
    pub layer3: PdLayerEvidenceWire,
    pub evidence_digest: u64,
    pub mpro_mass: u32,
    pub ready_for_shadow: bool,
    pub completed_at: u64,
    pub completion_digest: u64,
}

impl PdSpineEvidenceWire {
    pub const CONTRACT_VERSION: u32 = 0x0001_0600;
    pub const ALL_LAYERS_MASK: u8 = 0b111;

    pub fn recompute_gcl_process_digest(&self) -> u64 {
        digest_parts(&[
            &self.contract_version.to_le_bytes(),
            &self.gcl_law_seal.to_le_bytes(),
            &self.gcl_system_laws_seal.to_le_bytes(),
            &self.gcl_phase_digest.to_le_bytes(),
            &self.session_digest.to_le_bytes(),
            &self.parent_i0_digest.to_le_bytes(),
            &self.continuum_activation_digest.to_le_bytes(),
            &self.activation_contract_digest.to_le_bytes(),
            &self.created_at.to_le_bytes(),
            b"GCL_PD_SPINE_PROCESS_V160",
        ])
    }

    pub fn recompute_activation_id(&self) -> u64 {
        digest_parts(&[
            &self.contract_version.to_le_bytes(),
            &self.session_digest.to_le_bytes(),
            &self.parent_i0_digest.to_le_bytes(),
            &self.trace_digest.to_le_bytes(),
            &self.turn_digest.to_le_bytes(),
            &self.continuum_activation_digest.to_le_bytes(),
            &self.activation_contract_digest.to_le_bytes(),
            &self.gcl_process_digest.to_le_bytes(),
            &self.created_at.to_le_bytes(),
            b"PD_SPINE9_ACTIVATION_V160",
        ])
    }

    pub fn recompute_completion_digest(&self) -> u64 {
        digest_parts(&[
            &self.activation_id.to_le_bytes(),
            &self.activation_contract_digest.to_le_bytes(),
            &self.gcl_process_digest.to_le_bytes(),
            &self.layer1.result_digest.to_le_bytes(),
            &self.layer2.result_digest.to_le_bytes(),
            &self.layer3.result_digest.to_le_bytes(),
            &[self.layer_mask],
            &self.evidence_digest.to_le_bytes(),
            &self.mpro_mass.to_le_bytes(),
            &self.completed_at.to_le_bytes(),
            b"PD_SPINE9_COMPLETE_UNDER_GCL_V160",
        ])
    }

    pub fn verifies_internal(&self) -> bool {
        let phase_ok = self.gcl_phase_digest == digest_parts(&[b"KOLAPS_ARSYETIMI"]);
        let l1 = self.layer1.verifies_for(
            0b001,
            self.activation_id,
            self.activation_contract_digest,
            self.gcl_process_digest,
            0,
        );
        let l2 = self.layer2.verifies_for(
            0b010,
            self.activation_id,
            self.activation_contract_digest,
            self.gcl_process_digest,
            self.layer1.result_digest,
        );
        let l3 = self.layer3.verifies_for(
            0b100,
            self.activation_id,
            self.activation_contract_digest,
            self.gcl_process_digest,
            self.layer2.result_digest,
        );
        self.contract_version == Self::CONTRACT_VERSION
            && self.activation_id != 0
            && self.session_digest != 0
            && self.parent_i0_digest != 0
            && self.trace_digest != 0
            && self.turn_digest != 0
            && self.continuum.verifies_internal()
            && self.activation.verifies_internal()
            && self.continuum_activation_digest == self.continuum.activation_digest
            && self.activation_contract_digest == self.activation.contract_digest
            && self.gcl_law_seal != 0
            && self.gcl_system_laws_seal != 0
            && phase_ok
            && self.gcl_process_digest == self.recompute_gcl_process_digest()
            && self.activation_id == self.recompute_activation_id()
            && self.layer_mask == Self::ALL_LAYERS_MASK
            && l1
            && l2
            && l3
            && self.evidence_digest != 0
            && self.mpro_mass <= MASS_SCALE
            && self.ready_for_shadow
            && self.completion_digest == self.recompute_completion_digest()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionEvidenceWire {
    pub stage: u8,
    /// Materiali kanonik i organit. Shadow rillogarit kontributin; nuk i beson
    /// një `contribution` të gatshëm nga Quantum.
    pub evidence_words: Vec<u64>,
}

fn converge_action_words(words: &[u64]) -> u64 {
    words
        .iter()
        .enumerate()
        .fold(0x6A09_E667_F3BC_C909u64, |state, (index, word)| {
            state.rotate_left(((index as u32) % 47) + 5)
                ^ word
                    .rotate_left(((index as u32) % 31) + 1)
                    .wrapping_add(0x3C6E_F372_FE94_F82B ^ index as u64)
        })
}

fn action_stage_word(stage: u8) -> u64 {
    (u64::from(stage) + 1).wrapping_mul(0x9E37_79B9_7F4A_7C15)
}

fn replay_action_evidence(evidence: &[ActionEvidenceWire]) -> (u64, u64, bool) {
    let mut state = 0u64;
    let mut mask = 0u64;
    let mut unique = true;
    for entry in evidence {
        let bit = match entry.stage < 64 {
            true => 1u64 << u32::from(entry.stage),
            false => {
                unique = false;
                0
            }
        };
        match mask & bit {
            0 => {}
            _ => unique = false,
        }
        let contribution =
            converge_action_words(&entry.evidence_words) ^ action_stage_word(entry.stage);
        match expected_action_word_count(entry.stage) == Some(entry.evidence_words.len()) {
            true => {}
            false => unique = false,
        }
        state = state
            .rotate_left(11)
            .wrapping_add(contribution)
            .rotate_left(7);
        mask |= bit;
    }
    (state, mask, unique)
}

/// Paketa finale e evidencave që Quantum e prodhon vetëm PAS PIM/NPIM/MPRO.
/// Shadow nuk i zbulon evidencat; ai i rillogarit masat dhe vulos integritetin.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FinalEvidenceWire {
    pub pd_spine: PdSpineEvidenceWire,
    /// Evidence from the two subordinate pre-verdict muscles. Nbes verifies
    /// the referenced Shadow holds before the VDS judgement transition.
    pub muscle_evidence: MuscleEvidenceWire,
    /// Faza 1 e Besimit: konvergjenca e lehtë e HPRO/PRO/NPRO/SRK/APRO/
    /// MPRO/PIM/NPIM/HCP. Nuk është hash dhe nuk është vula; Shadow riluajnë
    /// fjalët kanonike dhe e konsumon gjendjen vetëm në verdikt.
    pub quantum_action_state: u64,
    /// Ledger-i i renditur i kontributeve reale; Shadow e riluajnë fold-in.
    pub quantum_action_evidence: Vec<ActionEvidenceWire>,
    /// Bitset-i provon se HPRO/PRO/NPRO/SRK/APRO/MPRO/PIM/NPIM/HCP
    /// mbaruan realisht. `required` është kontrata konstante e Untrust, e ruajtur në v1.6.3.
    pub quantum_action_mask: u64,
    pub quantum_required_action_mask: u64,
    /// SHA-256 dhe bytes e inputit që lindën në Light; Shadow i rillogarit.
    pub light_input_sha256: String,
    pub light_input_bytes: Vec<u8>,

    pub pim_evidence_density: u32,
    pub pim_logical_coherence: u32,
    pub pim_causal_integrity: u32,
    pub pim_convergence_strength: u32,
    pub pim_reproducibility: u32,
    pub pim_potentiality_score: u32,
    pub pim_suggestion: u8,
    pub pim_proof_chain: Vec<String>,

    pub npim_failure_density: u32,
    pub npim_contradiction_strength: u32,
    pub npim_elimination_certainty: u32,
    pub npim_negativity_score: u32,
    pub npim_suggestion: u8,
    pub npim_arguments: Vec<String>,
    /// Lidh blob-in real që shkon në Negative Knowledge me paketën NPIM.
    pub npim_arguments_blob_digest: u64,

    pub mpro_measurements: Vec<u8>,
    pub mpro_positives: u32,
    pub mpro_total: u32,
    pub mpro_vector_mass: u32,
    pub mpro_evidence_mass: u32,
    pub mpro_factic_mass: u32,

    /// Projekti shkencor është pjesë e së njëjtës paketë finale GCL.
    pub scientific_project: Option<ScientificProjectWire>,

    pub package_digest: u64,
}

impl FinalEvidenceWire {
    pub fn recompute_digest(&self) -> u64 {
        let mut w = Writer::new();
        w.bytes.extend_from_slice(b"ESS_MAI_FINAL_EVIDENCE_V164");
        put_pd_spine_evidence(&mut w, &self.pd_spine);
        put_muscle_evidence(&mut w, &self.muscle_evidence);
        w.u64(self.quantum_action_state);
        w.u32(self.quantum_action_evidence.len() as u32);
        for entry in &self.quantum_action_evidence {
            w.u8(entry.stage);
            w.u32(entry.evidence_words.len() as u32);
            for word in &entry.evidence_words {
                w.u64(*word);
            }
        }
        w.u64(self.quantum_action_mask);
        w.u64(self.quantum_required_action_mask);
        w.string(&self.light_input_sha256);
        w.bytes(&self.light_input_bytes);
        w.u32(self.pim_evidence_density);
        w.u32(self.pim_logical_coherence);
        w.u32(self.pim_causal_integrity);
        w.u32(self.pim_convergence_strength);
        w.u32(self.pim_reproducibility);
        w.u32(self.pim_potentiality_score);
        w.u8(self.pim_suggestion);
        w.vec_string(&self.pim_proof_chain);
        w.u32(self.npim_failure_density);
        w.u32(self.npim_contradiction_strength);
        w.u32(self.npim_elimination_certainty);
        w.u32(self.npim_negativity_score);
        w.u8(self.npim_suggestion);
        w.vec_string(&self.npim_arguments);
        w.u64(self.npim_arguments_blob_digest);
        w.vec_u8(&self.mpro_measurements);
        w.u32(self.mpro_positives);
        w.u32(self.mpro_total);
        w.u32(self.mpro_vector_mass);
        w.u32(self.mpro_evidence_mass);
        w.u32(self.mpro_factic_mass);
        put_scientific_project_option(&mut w, &self.scientific_project);
        fnv1a64(&w.bytes)
    }

    pub fn verifies_internal(&self) -> bool {
        let measured_positives = self
            .mpro_measurements
            .iter()
            .map(|v| u32::from(*v))
            .sum::<u32>();
        let all_binary = self.mpro_measurements.iter().all(|v| matches!(*v, 0 | 1));
        let expected_vector_mass = match self.mpro_total {
            0 => 0,
            total => self.mpro_positives.saturating_mul(MASS_SCALE) / total,
        };
        let expected_factic = self
            .mpro_vector_mass
            .saturating_mul(self.mpro_evidence_mass)
            / MASS_SCALE;
        let (replayed_state, replayed_mask, unique_actions) =
            replay_action_evidence(&self.quantum_action_evidence);
        let canonical_action_order = self.quantum_action_evidence.len()
            == REQUIRED_ACTION_ORDER.len()
            && self
                .quantum_action_evidence
                .iter()
                .zip(REQUIRED_ACTION_ORDER)
                .all(|(entry, expected_stage)| entry.stage == expected_stage);
        self.pd_spine.verifies_internal()
            && self.muscle_evidence.verifies_internal()
            && self.quantum_action_state != 0
            && !self.quantum_action_evidence.is_empty()
            && self.quantum_action_evidence.len() <= 64
            && unique_actions
            && canonical_action_order
            && replayed_state == self.quantum_action_state
            && replayed_mask == self.quantum_action_mask
            && self.quantum_required_action_mask == REQUIRED_ACTION_MASK
            && self.quantum_action_mask == self.quantum_required_action_mask
            && self.light_input_sha256.len() == 64
            && self
                .light_input_sha256
                .chars()
                .all(|c| c.is_ascii_hexdigit())
            && !self.light_input_bytes.is_empty()
            && self.pim_suggestion <= 2
            && self.npim_suggestion <= 2
            && !self.pim_proof_chain.is_empty()
            && self.npim_arguments_blob_digest != 0
            && self.mpro_measurements.len() == MPRO_EVALUATIONS
            && all_binary
            && self.mpro_total == MPRO_EVALUATIONS as u32
            && self.mpro_positives == measured_positives
            && self.mpro_vector_mass == expected_vector_mass
            && self.mpro_evidence_mass <= MASS_SCALE
            && self.mpro_factic_mass == expected_factic
            && self
                .scientific_project
                .as_ref()
                .map(|p| p.shape_is_valid())
                .unwrap_or(true)
            && self.package_digest == self.recompute_digest()
    }
}

pub fn digest_parts(parts: &[&[u8]]) -> u64 {
    parts.iter().fold(0xcbf2_9ce4_8422_2325u64, |acc, part| {
        part.iter().fold(acc, |hash, byte| {
            (hash ^ u64::from(*byte)).wrapping_mul(0x0000_0100_0000_01b3)
        })
    })
}

pub fn fixed_mass(value: f32) -> Option<u32> {
    match value.is_finite() {
        true => Some((value.clamp(0.0, 1.0) * MASS_SCALE as f32) as u32),
        false => None,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct QuantumInboundWire {
    pub package_id: String,
    pub session_id: String,
    pub territory: String,
    pub epistemic_mass: f32,
    pub evidence_density: f32,
    pub logical_coherence: f32,
    pub causal_integrity: f32,
    pub convergence_strength: f32,
    pub reproducibility: f32,
    pub potentiality_score: f32,
    pub ibe_score: f32,
    pub chain_coherence: f32,
    pub conservation_ok: bool,
    pub candidate_scores: Vec<f32>,
    pub suggested_verdict: u8,
    pub primitive_flags: u32,
    pub raw_bytes: Vec<u8>,
    pub xy_x: Option<String>,
    pub xy_y: Option<String>,
    pub xy_mass: Option<u32>,
    pub pd_binding_digest: Option<u64>,
    pub pd_continuum_activation_digest: Option<u64>,
    pub pd_parent_i0: Option<String>,
    pub collapse_refused: Option<String>,
    pub negative_paths: Vec<String>,
    pub hcp_input_id: Option<u64>,
    pub hcp_token_gen: Option<u64>,
    pub hcp_token_nonce: Option<u64>,
    pub hcp_directive_bit: Option<u8>,
    pub final_evidence: FinalEvidenceWire,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LightInboundWire {
    pub session_id: String,
    pub territory: String,
    pub primitive_flags: u32,
    pub epistemic_mass: f32,
    pub proof_chain: Vec<u64>,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NegativeKnowledgeWire {
    pub input_id: u64,
    pub reason: u32,
    pub process: String,
    pub mass: f32,
    pub frequency: u32,
    pub suggestion_code: u8,
    pub bytes: Vec<u8>,
    pub ts_ns: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShadowCycleRequest {
    pub quantum: QuantumInboundWire,
    pub light: LightInboundWire,
    pub negative: NegativeKnowledgeWire,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationReceiptWire {
    pub session_id: String,
    pub parent_i0: String,
    pub primitive_anchor: u64,
    pub xy_digest: u64,
    pub pd_binding_digest: u64,
    pub pd_continuum_activation_digest: u64,
    pub living_trust_digest: u64,
    pub living_trust_sha256: String,
    pub y_verdict: u8,
    pub x_verdict: u8,
    pub seal: u64,
    pub generation: u64,
    pub receipt_id: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShadowVerdictWire {
    pub verified: u8,
    pub primitive: u8,
    pub knowledge_band: u8,
    pub lgc_law: u32,
    pub legacy_score: f32,
    pub lgc_dsl: String,
    pub living_trust_action_state: u64,
    pub living_trust_action_mask: u64,
    pub living_trust_required_action_mask: u64,
    pub living_trust_system_laws_seal: u64,
    pub living_trust_sovereign_flags: u32,
    pub living_trust_sovereign_value: u32,
    pub living_trust_sha256: String,
    pub living_trust_intensity: u32,
    pub living_trust_kind: u8,
    /// GCL Scientific Project verdict (0 kur cikli nuk është projekt).
    pub project_id: u64,
    pub project_status: u8,
    pub project_context_digest: u64,
    pub project_evidence_digest: u64,
    /// SHA-256 të plota; digest-et u64 janë vetëm indekse compatibility.
    pub project_context_sha256: String,
    pub project_evidence_sha256: String,
    pub novel_factualized: bool,
    pub novel_trl_level: u8,
    pub novel_proof_score: u32,
    pub novel_rejection: u8,
}

impl ShadowVerdictWire {
    pub fn is_verified(&self) -> bool {
        self.verified == 1
    }
    pub fn is_primitive(&self) -> bool {
        self.primitive == 1
    }
    pub fn knowledge_band_label(&self) -> &'static str {
        match self.knowledge_band {
            1 => "hyp",
            2 => "fact",
            3 => "sci",
            _ => "neg",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShadowCycleResponse {
    pub session_id: String,
    pub verdict: ShadowVerdictWire,
    pub verification_receipt: VerificationReceiptWire,
    pub negative_persisted: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShadowNegativeRequest {
    pub negative: NegativeKnowledgeWire,
    pub export_negative_knowledge: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShadowNegativeResponse {
    pub persisted: bool,
    pub negative_export: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WireError {
    FrameTooLarge,
    Truncated,
    InvalidMagic,
    InvalidVersion(u16),
    InvalidKind { expected: u8, actual: u8 },
    InvalidChecksum,
    InvalidBool(u8),
    InvalidOption(u8),
    InvalidEnum { field: &'static str, actual: u8 },
    InvalidUtf8,
    FieldTooLarge,
    VectorTooLarge,
    TrailingBytes,
}

impl fmt::Display for WireError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for WireError {}

pub fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in bytes {
        h ^= *b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

struct Writer {
    bytes: Vec<u8>,
}
impl Writer {
    fn new() -> Self {
        Self { bytes: Vec::new() }
    }
    fn u8(&mut self, v: u8) {
        self.bytes.push(v);
    }
    fn bool(&mut self, v: bool) {
        self.u8(v as u8);
    }
    fn u16(&mut self, v: u16) {
        self.bytes.extend_from_slice(&v.to_le_bytes());
    }
    fn u32(&mut self, v: u32) {
        self.bytes.extend_from_slice(&v.to_le_bytes());
    }
    fn u64(&mut self, v: u64) {
        self.bytes.extend_from_slice(&v.to_le_bytes());
    }
    fn f32(&mut self, v: f32) {
        self.u32(v.to_bits());
    }
    fn bytes(&mut self, v: &[u8]) {
        self.u32(v.len() as u32);
        self.bytes.extend_from_slice(v);
    }
    fn string(&mut self, v: &str) {
        self.bytes(v.as_bytes());
    }
    fn option_u8(&mut self, v: Option<u8>) {
        match v {
            Some(x) => {
                self.u8(1);
                self.u8(x);
            }
            None => self.u8(0),
        }
    }
    fn option_u32(&mut self, v: Option<u32>) {
        match v {
            Some(x) => {
                self.u8(1);
                self.u32(x);
            }
            None => self.u8(0),
        }
    }
    fn option_u64(&mut self, v: Option<u64>) {
        match v {
            Some(x) => {
                self.u8(1);
                self.u64(x);
            }
            None => self.u8(0),
        }
    }
    fn option_string(&mut self, v: &Option<String>) {
        match v {
            Some(x) => {
                self.u8(1);
                self.string(x);
            }
            None => self.u8(0),
        }
    }
    fn vec_u8(&mut self, v: &[u8]) {
        self.u32(v.len() as u32);
        for x in v {
            self.u8(*x);
        }
    }
    fn vec_f32(&mut self, v: &[f32]) {
        self.u32(v.len() as u32);
        for x in v {
            self.f32(*x);
        }
    }
    fn vec_u64(&mut self, v: &[u64]) {
        self.u32(v.len() as u32);
        for x in v {
            self.u64(*x);
        }
    }
    fn vec_string(&mut self, v: &[String]) {
        self.u32(v.len() as u32);
        for x in v {
            self.string(x);
        }
    }
}

struct Reader<'a> {
    bytes: &'a [u8],
    pos: usize,
}
impl<'a> Reader<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, pos: 0 }
    }
    fn take(&mut self, n: usize) -> Result<&'a [u8], WireError> {
        let end = self.pos.checked_add(n).ok_or(WireError::Truncated)?;
        match end <= self.bytes.len() {
            true => {
                let out = &self.bytes[self.pos..end];
                self.pos = end;
                Ok(out)
            }
            false => Err(WireError::Truncated),
        }
    }
    fn u8(&mut self) -> Result<u8, WireError> {
        Ok(self.take(1)?[0])
    }
    fn bool(&mut self) -> Result<bool, WireError> {
        match self.u8()? {
            0 => Ok(false),
            1 => Ok(true),
            x => Err(WireError::InvalidBool(x)),
        }
    }
    fn u16(&mut self) -> Result<u16, WireError> {
        let b = self.take(2)?;
        Ok(u16::from_le_bytes([b[0], b[1]]))
    }
    fn u32(&mut self) -> Result<u32, WireError> {
        let b = self.take(4)?;
        Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }
    fn u64(&mut self) -> Result<u64, WireError> {
        let b = self.take(8)?;
        Ok(u64::from_le_bytes([
            b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7],
        ]))
    }
    fn f32(&mut self) -> Result<f32, WireError> {
        Ok(f32::from_bits(self.u32()?))
    }
    fn bytes(&mut self) -> Result<Vec<u8>, WireError> {
        let n = self.u32()? as usize;
        match n <= MAX_FIELD_BYTES {
            true => Ok(self.take(n)?.to_vec()),
            false => Err(WireError::FieldTooLarge),
        }
    }
    fn string(&mut self) -> Result<String, WireError> {
        String::from_utf8(self.bytes()?).map_err(|_| WireError::InvalidUtf8)
    }
    fn option_tag(&mut self) -> Result<bool, WireError> {
        match self.u8()? {
            0 => Ok(false),
            1 => Ok(true),
            x => Err(WireError::InvalidOption(x)),
        }
    }
    fn option_u8(&mut self) -> Result<Option<u8>, WireError> {
        match self.option_tag()? {
            true => Ok(Some(self.u8()?)),
            false => Ok(None),
        }
    }
    fn option_u32(&mut self) -> Result<Option<u32>, WireError> {
        match self.option_tag()? {
            true => Ok(Some(self.u32()?)),
            false => Ok(None),
        }
    }
    fn option_u64(&mut self) -> Result<Option<u64>, WireError> {
        match self.option_tag()? {
            true => Ok(Some(self.u64()?)),
            false => Ok(None),
        }
    }
    fn option_string(&mut self) -> Result<Option<String>, WireError> {
        match self.option_tag()? {
            true => Ok(Some(self.string()?)),
            false => Ok(None),
        }
    }
    fn count(&mut self) -> Result<usize, WireError> {
        let n = self.u32()? as usize;
        match n <= MAX_VECTOR_ITEMS {
            true => Ok(n),
            false => Err(WireError::VectorTooLarge),
        }
    }
    fn vec_u8(&mut self) -> Result<Vec<u8>, WireError> {
        let n = self.count()?;
        let mut out = Vec::with_capacity(n);
        for _ in 0..n {
            out.push(self.u8()?);
        }
        Ok(out)
    }
    fn vec_f32(&mut self) -> Result<Vec<f32>, WireError> {
        let n = self.count()?;
        let mut out = Vec::with_capacity(n);
        for _ in 0..n {
            out.push(self.f32()?);
        }
        Ok(out)
    }
    fn vec_u64(&mut self) -> Result<Vec<u64>, WireError> {
        let n = self.count()?;
        let mut out = Vec::with_capacity(n);
        for _ in 0..n {
            out.push(self.u64()?);
        }
        Ok(out)
    }
    fn vec_string(&mut self) -> Result<Vec<String>, WireError> {
        let n = self.count()?;
        let mut out = Vec::with_capacity(n);
        for _ in 0..n {
            out.push(self.string()?);
        }
        Ok(out)
    }
    fn finish(self) -> Result<(), WireError> {
        match self.pos == self.bytes.len() {
            true => Ok(()),
            false => Err(WireError::TrailingBytes),
        }
    }
}

fn frame(kind: u8, payload: Vec<u8>) -> Vec<u8> {
    let mut out = Writer::new();
    out.bytes.extend_from_slice(&MAGIC);
    out.u16(PROTOCOL_VERSION);
    out.u8(kind);
    out.u8(0);
    out.u64(payload.len() as u64);
    out.bytes.extend_from_slice(&payload);
    let checksum = fnv1a64(&out.bytes);
    out.u64(checksum);
    out.bytes
}

fn unframe(bytes: &[u8], expected_kind: u8) -> Result<&[u8], WireError> {
    match bytes.len() <= MAX_FRAME_BYTES {
        true => {}
        false => return Err(WireError::FrameTooLarge),
    }
    let mut r = Reader::new(bytes);
    let magic = r.take(8)?;
    match magic == MAGIC.as_slice() {
        true => {}
        false => return Err(WireError::InvalidMagic),
    }
    let version = r.u16()?;
    match version == PROTOCOL_VERSION {
        true => {}
        false => return Err(WireError::InvalidVersion(version)),
    }
    let kind = r.u8()?;
    match kind == expected_kind {
        true => {}
        false => {
            return Err(WireError::InvalidKind {
                expected: expected_kind,
                actual: kind,
            })
        }
    }
    let _reserved = r.u8()?;
    let payload_len = r.u64()? as usize;
    match payload_len <= MAX_FRAME_BYTES {
        true => {}
        false => return Err(WireError::FrameTooLarge),
    }
    let payload_start = r.pos;
    let _ = r.take(payload_len)?;
    let checksum_start = r.pos;
    let checksum = r.u64()?;
    r.finish()?;
    match fnv1a64(&bytes[..checksum_start]) == checksum {
        true => Ok(&bytes[payload_start..payload_start + payload_len]),
        false => Err(WireError::InvalidChecksum),
    }
}

fn put_pd_layer_evidence(w: &mut Writer, l: &PdLayerEvidenceWire) {
    w.u8(l.layer_mask);
    w.u64(l.activation_id);
    w.u64(l.activation_contract_digest);
    w.u64(l.gcl_process_digest);
    w.u64(l.parent_result_digest);
    w.bytes(&l.result_material);
    w.u64(l.result_material_digest);
    w.u64(l.result_digest);
    w.bool(l.completed);
}

fn get_pd_layer_evidence(r: &mut Reader<'_>) -> Result<PdLayerEvidenceWire, WireError> {
    Ok(PdLayerEvidenceWire {
        layer_mask: r.u8()?,
        activation_id: r.u64()?,
        activation_contract_digest: r.u64()?,
        gcl_process_digest: r.u64()?,
        parent_result_digest: r.u64()?,
        result_material: r.bytes()?,
        result_material_digest: r.u64()?,
        result_digest: r.u64()?,
        completed: r.bool()?,
    })
}

fn put_pd_continuum_evidence(w: &mut Writer, c: &PdContinuumEvidenceWire) {
    w.u32(c.contract_version);
    w.u64(c.session_digest);
    w.u64(c.initial_i_digest);
    w.u64(c.user_input_digest);
    w.u64(c.i0_digest);
    w.u64(c.stimulus_digest);
    w.u64(c.stimulus_created_at);
    w.u8(c.stimulus_state);
    w.u32(c.unit_count);
    w.u64(c.question_digest);
    w.u64(c.response_kind_digest);
    w.u64(c.increment_digest);
    w.u64(c.increment_created_at);
    w.u8(c.increment_state);
    w.u64(c.activation_digest);
    w.u8(c.activation_state);
}

fn get_pd_continuum_evidence(r: &mut Reader<'_>) -> Result<PdContinuumEvidenceWire, WireError> {
    Ok(PdContinuumEvidenceWire {
        contract_version: r.u32()?,
        session_digest: r.u64()?,
        initial_i_digest: r.u64()?,
        user_input_digest: r.u64()?,
        i0_digest: r.u64()?,
        stimulus_digest: r.u64()?,
        stimulus_created_at: r.u64()?,
        stimulus_state: r.u8()?,
        unit_count: r.u32()?,
        question_digest: r.u64()?,
        response_kind_digest: r.u64()?,
        increment_digest: r.u64()?,
        increment_created_at: r.u64()?,
        increment_state: r.u8()?,
        activation_digest: r.u64()?,
        activation_state: r.u8()?,
    })
}

fn put_pd_activation_evidence(w: &mut Writer, a: &PdActivationEvidenceWire) {
    w.u8(a.mode_code);
    w.u8(a.origin_code);
    w.u64(a.response_kind_digest);
    w.u64(a.genius_signal_digest);
    w.u32(a.trace_mass);
    w.u32(a.structural_coherence);
    w.u8(a.required_layer_mask);
    w.u64(a.contract_digest);
}

fn get_pd_activation_evidence(r: &mut Reader<'_>) -> Result<PdActivationEvidenceWire, WireError> {
    Ok(PdActivationEvidenceWire {
        mode_code: r.u8()?,
        origin_code: r.u8()?,
        response_kind_digest: r.u64()?,
        genius_signal_digest: r.u64()?,
        trace_mass: r.u32()?,
        structural_coherence: r.u32()?,
        required_layer_mask: r.u8()?,
        contract_digest: r.u64()?,
    })
}

fn put_pd_spine_evidence(w: &mut Writer, p: &PdSpineEvidenceWire) {
    w.u32(p.contract_version);
    w.u64(p.activation_id);
    w.u64(p.session_digest);
    w.u64(p.parent_i0_digest);
    w.u64(p.trace_digest);
    w.u64(p.turn_digest);
    w.u64(p.created_at);
    put_pd_continuum_evidence(w, &p.continuum);
    put_pd_activation_evidence(w, &p.activation);
    w.u64(p.continuum_activation_digest);
    w.u64(p.activation_contract_digest);
    w.u64(p.gcl_law_seal);
    w.u64(p.gcl_system_laws_seal);
    w.u64(p.gcl_phase_digest);
    w.u64(p.gcl_process_digest);
    w.u8(p.layer_mask);
    put_pd_layer_evidence(w, &p.layer1);
    put_pd_layer_evidence(w, &p.layer2);
    put_pd_layer_evidence(w, &p.layer3);
    w.u64(p.evidence_digest);
    w.u32(p.mpro_mass);
    w.bool(p.ready_for_shadow);
    w.u64(p.completed_at);
    w.u64(p.completion_digest);
}

fn get_pd_spine_evidence(r: &mut Reader<'_>) -> Result<PdSpineEvidenceWire, WireError> {
    Ok(PdSpineEvidenceWire {
        contract_version: r.u32()?,
        activation_id: r.u64()?,
        session_digest: r.u64()?,
        parent_i0_digest: r.u64()?,
        trace_digest: r.u64()?,
        turn_digest: r.u64()?,
        created_at: r.u64()?,
        continuum: get_pd_continuum_evidence(r)?,
        activation: get_pd_activation_evidence(r)?,
        continuum_activation_digest: r.u64()?,
        activation_contract_digest: r.u64()?,
        gcl_law_seal: r.u64()?,
        gcl_system_laws_seal: r.u64()?,
        gcl_phase_digest: r.u64()?,
        gcl_process_digest: r.u64()?,
        layer_mask: r.u8()?,
        layer1: get_pd_layer_evidence(r)?,
        layer2: get_pd_layer_evidence(r)?,
        layer3: get_pd_layer_evidence(r)?,
        evidence_digest: r.u64()?,
        mpro_mass: r.u32()?,
        ready_for_shadow: r.bool()?,
        completed_at: r.u64()?,
        completion_digest: r.u64()?,
    })
}

fn put_project_context(w: &mut Writer, x: &ProjectContextWitnessWire) {
    w.u64(x.project_id);
    w.u64(x.user_id);
    w.u64(x.trace_id);
    w.u32(x.revision);
    w.string(&x.content_sha256);
    w.u32(x.light_sovereign_flags);
    w.string(&x.context_sha256);
}
fn get_project_context(r: &mut Reader<'_>) -> Result<ProjectContextWitnessWire, WireError> {
    Ok(ProjectContextWitnessWire {
        project_id: r.u64()?,
        user_id: r.u64()?,
        trace_id: r.u64()?,
        revision: r.u32()?,
        content_sha256: r.string()?,
        light_sovereign_flags: r.u32()?,
        context_sha256: r.string()?,
    })
}
fn put_scientific_project(w: &mut Writer, x: &ScientificProjectWire) {
    put_project_context(w, &x.context);
    w.string(&x.project_title);
    w.string(&x.domain);
    w.string(&x.hypothesis);
    w.vec_string(&x.assumptions);
    w.u64(x.gcl_process_digest);
    w.u8(x.trl_level);
    w.bool(x.trl_passed);
    w.u32(x.trl_confidence);
    w.u32(x.trl_reproducibility);
    w.string(&x.lab_test_id);
    w.string(&x.findings);
    w.string(&x.description);
    w.u32(x.files.len() as u32);
    for f in &x.files {
        w.u8(f.kind);
        w.bytes(&f.bytes);
    }
    w.string(&x.evidence_sha256);
}
fn get_scientific_project(r: &mut Reader<'_>) -> Result<ScientificProjectWire, WireError> {
    let context = get_project_context(r)?;
    let project_title = r.string()?;
    let domain = r.string()?;
    let hypothesis = r.string()?;
    let assumptions = r.vec_string()?;
    let gcl_process_digest = r.u64()?;
    let trl_level = r.u8()?;
    let trl_passed = r.bool()?;
    let trl_confidence = r.u32()?;
    let trl_reproducibility = r.u32()?;
    let lab_test_id = r.string()?;
    let findings = r.string()?;
    let description = r.string()?;
    let n = r.count()?;
    let mut files = Vec::with_capacity(n);
    for _ in 0..n {
        files.push(ProjectEvidenceFileWire {
            kind: r.u8()?,
            bytes: r.bytes()?,
        });
    }
    let evidence_sha256 = r.string()?;
    Ok(ScientificProjectWire {
        context,
        project_title,
        domain,
        hypothesis,
        assumptions,
        gcl_process_digest,
        trl_level,
        trl_passed,
        trl_confidence,
        trl_reproducibility,
        lab_test_id,
        findings,
        description,
        files,
        evidence_sha256,
    })
}
fn put_scientific_project_option(w: &mut Writer, x: &Option<ScientificProjectWire>) {
    match x {
        Some(v) => {
            w.u8(1);
            put_scientific_project(w, v);
        }
        None => w.u8(0),
    }
}
fn get_scientific_project_option(
    r: &mut Reader<'_>,
) -> Result<Option<ScientificProjectWire>, WireError> {
    match r.option_tag()? {
        true => Ok(Some(get_scientific_project(r)?)),
        false => Ok(None),
    }
}

fn put_muscle_evidence(w: &mut Writer, evidence: &MuscleEvidenceWire) {
    w.string(&evidence.besa_hold_id);
    w.string(&evidence.besa_hold_sha256);
    w.string(&evidence.besa_selection_sha256);
    w.string(&evidence.besa_attestation_sha256);
    w.string(&evidence.asht_hold_id);
    w.string(&evidence.asht_hold_sha256);
    w.string(&evidence.asht_selection_sha256);
    w.string(&evidence.asht_attestation_sha256);
    w.string(&evidence.split_sha256);
    w.string(&evidence.combined_sha256);
}

fn get_muscle_evidence(r: &mut Reader<'_>) -> Result<MuscleEvidenceWire, WireError> {
    Ok(MuscleEvidenceWire {
        besa_hold_id: r.string()?,
        besa_hold_sha256: r.string()?,
        besa_selection_sha256: r.string()?,
        besa_attestation_sha256: r.string()?,
        asht_hold_id: r.string()?,
        asht_hold_sha256: r.string()?,
        asht_selection_sha256: r.string()?,
        asht_attestation_sha256: r.string()?,
        split_sha256: r.string()?,
        combined_sha256: r.string()?,
    })
}

fn put_final_evidence(w: &mut Writer, e: &FinalEvidenceWire) {
    put_pd_spine_evidence(w, &e.pd_spine);
    put_muscle_evidence(w, &e.muscle_evidence);
    w.u64(e.quantum_action_state);
    w.u32(e.quantum_action_evidence.len() as u32);
    for entry in &e.quantum_action_evidence {
        w.u8(entry.stage);
        w.u32(entry.evidence_words.len() as u32);
        for word in &entry.evidence_words {
            w.u64(*word);
        }
    }
    w.u64(e.quantum_action_mask);
    w.u64(e.quantum_required_action_mask);
    w.string(&e.light_input_sha256);
    w.bytes(&e.light_input_bytes);
    w.u32(e.pim_evidence_density);
    w.u32(e.pim_logical_coherence);
    w.u32(e.pim_causal_integrity);
    w.u32(e.pim_convergence_strength);
    w.u32(e.pim_reproducibility);
    w.u32(e.pim_potentiality_score);
    w.u8(e.pim_suggestion);
    w.vec_string(&e.pim_proof_chain);
    w.u32(e.npim_failure_density);
    w.u32(e.npim_contradiction_strength);
    w.u32(e.npim_elimination_certainty);
    w.u32(e.npim_negativity_score);
    w.u8(e.npim_suggestion);
    w.vec_string(&e.npim_arguments);
    w.u64(e.npim_arguments_blob_digest);
    w.vec_u8(&e.mpro_measurements);
    w.u32(e.mpro_positives);
    w.u32(e.mpro_total);
    w.u32(e.mpro_vector_mass);
    w.u32(e.mpro_evidence_mass);
    w.u32(e.mpro_factic_mass);
    put_scientific_project_option(w, &e.scientific_project);
    w.u64(e.package_digest);
}

fn get_final_evidence(r: &mut Reader<'_>) -> Result<FinalEvidenceWire, WireError> {
    let pd_spine = get_pd_spine_evidence(r)?;
    let muscle_evidence = get_muscle_evidence(r)?;
    let quantum_action_state = r.u64()?;
    let action_len = r.u32()? as usize;
    match action_len <= 64 {
        true => {}
        false => return Err(WireError::VectorTooLarge),
    }
    let mut quantum_action_evidence = Vec::with_capacity(action_len);
    for _ in 0..action_len {
        let stage = r.u8()?;
        let words_len = r.u32()? as usize;
        match words_len > 0 && words_len <= 64 {
            true => {}
            false => return Err(WireError::VectorTooLarge),
        }
        let mut evidence_words = Vec::with_capacity(words_len);
        for _ in 0..words_len {
            evidence_words.push(r.u64()?);
        }
        quantum_action_evidence.push(ActionEvidenceWire {
            stage,
            evidence_words,
        });
    }
    Ok(FinalEvidenceWire {
        pd_spine,
        muscle_evidence,
        quantum_action_state,
        quantum_action_evidence,
        quantum_action_mask: r.u64()?,
        quantum_required_action_mask: r.u64()?,
        light_input_sha256: r.string()?,
        light_input_bytes: r.bytes()?,
        pim_evidence_density: r.u32()?,
        pim_logical_coherence: r.u32()?,
        pim_causal_integrity: r.u32()?,
        pim_convergence_strength: r.u32()?,
        pim_reproducibility: r.u32()?,
        pim_potentiality_score: r.u32()?,
        pim_suggestion: r.u8()?,
        pim_proof_chain: r.vec_string()?,
        npim_failure_density: r.u32()?,
        npim_contradiction_strength: r.u32()?,
        npim_elimination_certainty: r.u32()?,
        npim_negativity_score: r.u32()?,
        npim_suggestion: r.u8()?,
        npim_arguments: r.vec_string()?,
        npim_arguments_blob_digest: r.u64()?,
        mpro_measurements: r.vec_u8()?,
        mpro_positives: r.u32()?,
        mpro_total: r.u32()?,
        mpro_vector_mass: r.u32()?,
        mpro_evidence_mass: r.u32()?,
        mpro_factic_mass: r.u32()?,
        scientific_project: get_scientific_project_option(r)?,
        package_digest: r.u64()?,
    })
}

fn put_quantum(w: &mut Writer, q: &QuantumInboundWire) {
    w.string(&q.package_id);
    w.string(&q.session_id);
    w.string(&q.territory);
    w.f32(q.epistemic_mass);
    w.f32(q.evidence_density);
    w.f32(q.logical_coherence);
    w.f32(q.causal_integrity);
    w.f32(q.convergence_strength);
    w.f32(q.reproducibility);
    w.f32(q.potentiality_score);
    w.f32(q.ibe_score);
    w.f32(q.chain_coherence);
    w.bool(q.conservation_ok);
    w.vec_f32(&q.candidate_scores);
    w.u8(q.suggested_verdict);
    w.u32(q.primitive_flags);
    w.bytes(&q.raw_bytes);
    w.option_string(&q.xy_x);
    w.option_string(&q.xy_y);
    w.option_u32(q.xy_mass);
    w.option_u64(q.pd_binding_digest);
    w.option_u64(q.pd_continuum_activation_digest);
    w.option_string(&q.pd_parent_i0);
    w.option_string(&q.collapse_refused);
    w.vec_string(&q.negative_paths);
    w.option_u64(q.hcp_input_id);
    w.option_u64(q.hcp_token_gen);
    w.option_u64(q.hcp_token_nonce);
    w.option_u8(q.hcp_directive_bit);
    put_final_evidence(w, &q.final_evidence);
}
fn get_quantum(r: &mut Reader<'_>) -> Result<QuantumInboundWire, WireError> {
    Ok(QuantumInboundWire {
        package_id: r.string()?,
        session_id: r.string()?,
        territory: r.string()?,
        epistemic_mass: r.f32()?,
        evidence_density: r.f32()?,
        logical_coherence: r.f32()?,
        causal_integrity: r.f32()?,
        convergence_strength: r.f32()?,
        reproducibility: r.f32()?,
        potentiality_score: r.f32()?,
        ibe_score: r.f32()?,
        chain_coherence: r.f32()?,
        conservation_ok: r.bool()?,
        candidate_scores: r.vec_f32()?,
        suggested_verdict: r.u8()?,
        primitive_flags: r.u32()?,
        raw_bytes: r.bytes()?,
        xy_x: r.option_string()?,
        xy_y: r.option_string()?,
        xy_mass: r.option_u32()?,
        pd_binding_digest: r.option_u64()?,
        pd_continuum_activation_digest: r.option_u64()?,
        pd_parent_i0: r.option_string()?,
        collapse_refused: r.option_string()?,
        negative_paths: r.vec_string()?,
        hcp_input_id: r.option_u64()?,
        hcp_token_gen: r.option_u64()?,
        hcp_token_nonce: r.option_u64()?,
        hcp_directive_bit: r.option_u8()?,
        final_evidence: get_final_evidence(r)?,
    })
}
fn put_light(w: &mut Writer, l: &LightInboundWire) {
    w.string(&l.session_id);
    w.string(&l.territory);
    w.u32(l.primitive_flags);
    w.f32(l.epistemic_mass);
    w.vec_u64(&l.proof_chain);
    w.bytes(&l.payload);
}
fn get_light(r: &mut Reader<'_>) -> Result<LightInboundWire, WireError> {
    Ok(LightInboundWire {
        session_id: r.string()?,
        territory: r.string()?,
        primitive_flags: r.u32()?,
        epistemic_mass: r.f32()?,
        proof_chain: r.vec_u64()?,
        payload: r.bytes()?,
    })
}
fn put_negative(w: &mut Writer, n: &NegativeKnowledgeWire) {
    w.u64(n.input_id);
    w.u32(n.reason);
    w.string(&n.process);
    w.f32(n.mass);
    w.u32(n.frequency);
    w.u8(n.suggestion_code);
    w.bytes(&n.bytes);
    w.u64(n.ts_ns);
}
fn get_negative(r: &mut Reader<'_>) -> Result<NegativeKnowledgeWire, WireError> {
    Ok(NegativeKnowledgeWire {
        input_id: r.u64()?,
        reason: r.u32()?,
        process: r.string()?,
        mass: r.f32()?,
        frequency: r.u32()?,
        suggestion_code: r.u8()?,
        bytes: r.bytes()?,
        ts_ns: r.u64()?,
    })
}
fn put_receipt(w: &mut Writer, x: &VerificationReceiptWire) {
    w.string(&x.session_id);
    w.string(&x.parent_i0);
    w.u64(x.primitive_anchor);
    w.u64(x.xy_digest);
    w.u64(x.pd_binding_digest);
    w.u64(x.pd_continuum_activation_digest);
    w.u64(x.living_trust_digest);
    w.string(&x.living_trust_sha256);
    w.u8(x.y_verdict);
    w.u8(x.x_verdict);
    w.u64(x.seal);
    w.u64(x.generation);
    w.string(&x.receipt_id);
}
fn get_receipt(r: &mut Reader<'_>) -> Result<VerificationReceiptWire, WireError> {
    Ok(VerificationReceiptWire {
        session_id: r.string()?,
        parent_i0: r.string()?,
        primitive_anchor: r.u64()?,
        xy_digest: r.u64()?,
        pd_binding_digest: r.u64()?,
        pd_continuum_activation_digest: r.u64()?,
        living_trust_digest: r.u64()?,
        living_trust_sha256: r.string()?,
        y_verdict: r.u8()?,
        x_verdict: r.u8()?,
        seal: r.u64()?,
        generation: r.u64()?,
        receipt_id: r.string()?,
    })
}
fn put_verdict(w: &mut Writer, v: &ShadowVerdictWire) {
    w.u8(v.verified);
    w.u8(v.primitive);
    w.u8(v.knowledge_band);
    w.u32(v.lgc_law);
    w.f32(v.legacy_score);
    w.string(&v.lgc_dsl);
    w.u64(v.living_trust_action_state);
    w.u64(v.living_trust_action_mask);
    w.u64(v.living_trust_required_action_mask);
    w.u64(v.living_trust_system_laws_seal);
    w.u32(v.living_trust_sovereign_flags);
    w.u32(v.living_trust_sovereign_value);
    w.string(&v.living_trust_sha256);
    w.u32(v.living_trust_intensity);
    w.u8(v.living_trust_kind);
    w.u64(v.project_id);
    w.u8(v.project_status);
    w.u64(v.project_context_digest);
    w.u64(v.project_evidence_digest);
    w.string(&v.project_context_sha256);
    w.string(&v.project_evidence_sha256);
    w.bool(v.novel_factualized);
    w.u8(v.novel_trl_level);
    w.u32(v.novel_proof_score);
    w.u8(v.novel_rejection);
}
fn get_verdict(r: &mut Reader<'_>) -> Result<ShadowVerdictWire, WireError> {
    Ok(ShadowVerdictWire {
        verified: r.u8()?,
        primitive: r.u8()?,
        knowledge_band: r.u8()?,
        lgc_law: r.u32()?,
        legacy_score: r.f32()?,
        lgc_dsl: r.string()?,
        living_trust_action_state: r.u64()?,
        living_trust_action_mask: r.u64()?,
        living_trust_required_action_mask: r.u64()?,
        living_trust_system_laws_seal: r.u64()?,
        living_trust_sovereign_flags: r.u32()?,
        living_trust_sovereign_value: r.u32()?,
        living_trust_sha256: r.string()?,
        living_trust_intensity: r.u32()?,
        living_trust_kind: r.u8()?,
        project_id: r.u64()?,
        project_status: r.u8()?,
        project_context_digest: r.u64()?,
        project_evidence_digest: r.u64()?,
        project_context_sha256: r.string()?,
        project_evidence_sha256: r.string()?,
        novel_factualized: r.bool()?,
        novel_trl_level: r.u8()?,
        novel_proof_score: r.u32()?,
        novel_rejection: r.u8()?,
    })
}

pub fn encode_cycle_request(x: &ShadowCycleRequest) -> Vec<u8> {
    let mut w = Writer::new();
    put_quantum(&mut w, &x.quantum);
    put_light(&mut w, &x.light);
    put_negative(&mut w, &x.negative);
    frame(KIND_CYCLE_REQUEST, w.bytes)
}
pub fn decode_cycle_request(bytes: &[u8]) -> Result<ShadowCycleRequest, WireError> {
    let mut r = Reader::new(unframe(bytes, KIND_CYCLE_REQUEST)?);
    let out = ShadowCycleRequest {
        quantum: get_quantum(&mut r)?,
        light: get_light(&mut r)?,
        negative: get_negative(&mut r)?,
    };
    r.finish()?;
    Ok(out)
}
pub fn encode_cycle_response(x: &ShadowCycleResponse) -> Vec<u8> {
    let mut w = Writer::new();
    w.string(&x.session_id);
    put_verdict(&mut w, &x.verdict);
    put_receipt(&mut w, &x.verification_receipt);
    w.bool(x.negative_persisted);
    frame(KIND_CYCLE_RESPONSE, w.bytes)
}
pub fn decode_cycle_response(bytes: &[u8]) -> Result<ShadowCycleResponse, WireError> {
    let mut r = Reader::new(unframe(bytes, KIND_CYCLE_RESPONSE)?);
    let out = ShadowCycleResponse {
        session_id: r.string()?,
        verdict: get_verdict(&mut r)?,
        verification_receipt: get_receipt(&mut r)?,
        negative_persisted: r.bool()?,
    };
    r.finish()?;
    Ok(out)
}
pub fn encode_negative_request(x: &ShadowNegativeRequest) -> Vec<u8> {
    let mut w = Writer::new();
    put_negative(&mut w, &x.negative);
    w.bool(x.export_negative_knowledge);
    frame(KIND_NEGATIVE_REQUEST, w.bytes)
}
pub fn decode_negative_request(bytes: &[u8]) -> Result<ShadowNegativeRequest, WireError> {
    let mut r = Reader::new(unframe(bytes, KIND_NEGATIVE_REQUEST)?);
    let out = ShadowNegativeRequest {
        negative: get_negative(&mut r)?,
        export_negative_knowledge: r.bool()?,
    };
    r.finish()?;
    Ok(out)
}
pub fn encode_negative_response(x: &ShadowNegativeResponse) -> Vec<u8> {
    let mut w = Writer::new();
    w.bool(x.persisted);
    w.bytes(&x.negative_export);
    frame(KIND_NEGATIVE_RESPONSE, w.bytes)
}
pub fn decode_negative_response(bytes: &[u8]) -> Result<ShadowNegativeResponse, WireError> {
    let mut r = Reader::new(unframe(bytes, KIND_NEGATIVE_RESPONSE)?);
    let out = ShadowNegativeResponse {
        persisted: r.bool()?,
        negative_export: r.bytes()?,
    };
    r.finish()?;
    Ok(out)
}

pub fn encode_project_registration_request(x: &ProjectRegistrationRequestWire) -> Vec<u8> {
    let mut w = Writer::new();
    w.u64(x.project_id);
    w.u64(x.user_id);
    w.u64(x.trace_id);
    w.string(&x.project_title);
    w.string(&x.initial_trace);
    w.string(&x.project_content);
    w.f32(x.progress_percentage);
    w.u64(x.last_update_ns);
    w.string(&x.notes);
    w.u32(x.light_sovereign_flags);
    frame(KIND_PROJECT_REGISTER_REQUEST, w.bytes)
}
pub fn decode_project_registration_request(
    bytes: &[u8],
) -> Result<ProjectRegistrationRequestWire, WireError> {
    let mut r = Reader::new(unframe(bytes, KIND_PROJECT_REGISTER_REQUEST)?);
    let out = ProjectRegistrationRequestWire {
        project_id: r.u64()?,
        user_id: r.u64()?,
        trace_id: r.u64()?,
        project_title: r.string()?,
        initial_trace: r.string()?,
        project_content: r.string()?,
        progress_percentage: r.f32()?,
        last_update_ns: r.u64()?,
        notes: r.string()?,
        light_sovereign_flags: r.u32()?,
    };
    r.finish()?;
    Ok(out)
}
pub fn encode_project_registration_response(x: &ProjectRegistrationResponseWire) -> Vec<u8> {
    let mut w = Writer::new();
    w.bool(x.accepted);
    w.u8(x.reason_code);
    match &x.witness {
        Some(v) => {
            w.u8(1);
            put_project_context(&mut w, v);
        }
        None => w.u8(0),
    }
    frame(KIND_PROJECT_REGISTER_RESPONSE, w.bytes)
}
pub fn decode_project_registration_response(
    bytes: &[u8],
) -> Result<ProjectRegistrationResponseWire, WireError> {
    let mut r = Reader::new(unframe(bytes, KIND_PROJECT_REGISTER_RESPONSE)?);
    let accepted = r.bool()?;
    let reason_code = r.u8()?;
    let witness = match r.option_tag()? {
        true => Some(get_project_context(&mut r)?),
        false => None,
    };
    r.finish()?;
    Ok(ProjectRegistrationResponseWire {
        accepted,
        reason_code,
        witness,
    })
}

fn put_evidence_files(w: &mut Writer, files: &[ProjectEvidenceFileWire]) {
    w.u32(files.len() as u32);
    for file in files {
        w.u8(file.kind);
        w.bytes(&file.bytes);
    }
}

fn get_evidence_files(r: &mut Reader<'_>) -> Result<Vec<ProjectEvidenceFileWire>, WireError> {
    let count = r.count()?;
    let mut files = Vec::with_capacity(count);
    for _ in 0..count {
        files.push(ProjectEvidenceFileWire {
            kind: r.u8()?,
            bytes: r.bytes()?,
        });
    }
    Ok(files)
}

pub fn encode_light_project_intake_request(x: &LightProjectIntakeRequestWire) -> Vec<u8> {
    let mut w = Writer::new();
    w.string(&x.project_name);
    w.string(&x.project_description);
    w.string(&x.project_content);
    w.string(&x.domain);
    w.string(&x.lang_code);
    w.string(&x.evolve_summary);
    w.string(&x.hypothesis);
    w.vec_string(&x.assumptions);
    w.string(&x.documentation_description);
    put_evidence_files(&mut w, &x.files);
    frame(KIND_LIGHT_PROJECT_INTAKE_REQUEST, w.bytes)
}

pub fn decode_light_project_intake_request(
    bytes: &[u8],
) -> Result<LightProjectIntakeRequestWire, WireError> {
    let mut r = Reader::new(unframe(bytes, KIND_LIGHT_PROJECT_INTAKE_REQUEST)?);
    let out = LightProjectIntakeRequestWire {
        project_name: r.string()?,
        project_description: r.string()?,
        project_content: r.string()?,
        domain: r.string()?,
        lang_code: r.string()?,
        evolve_summary: r.string()?,
        hypothesis: r.string()?,
        assumptions: r.vec_string()?,
        documentation_description: r.string()?,
        files: get_evidence_files(&mut r)?,
    };
    r.finish()?;
    Ok(out)
}

pub fn encode_light_project_intake_response(x: &LightProjectIntakeResponseWire) -> Vec<u8> {
    let mut w = Writer::new();
    w.bool(x.accepted_into_gcl);
    w.u8(x.reason_code);
    match &x.witness {
        Some(witness) => {
            w.u8(1);
            put_project_context(&mut w, witness);
        }
        None => w.u8(0),
    }
    frame(KIND_LIGHT_PROJECT_INTAKE_RESPONSE, w.bytes)
}

pub fn decode_light_project_intake_response(
    bytes: &[u8],
) -> Result<LightProjectIntakeResponseWire, WireError> {
    let mut r = Reader::new(unframe(bytes, KIND_LIGHT_PROJECT_INTAKE_RESPONSE)?);
    let accepted_into_gcl = r.bool()?;
    let reason_code = r.u8()?;
    let witness = match r.option_tag()? {
        true => Some(get_project_context(&mut r)?),
        false => None,
    };
    r.finish()?;
    Ok(LightProjectIntakeResponseWire {
        accepted_into_gcl,
        reason_code,
        witness,
    })
}

pub fn encode_quantum_project_execution_request(x: &QuantumProjectExecutionRequestWire) -> Vec<u8> {
    let mut w = Writer::new();
    w.u64(x.project_id);
    w.u64(x.project_trace_id);
    w.string(&x.project_context_sha256);
    w.string(&x.payload_sha256);
    w.string(&x.payload);
    frame(KIND_QUANTUM_PROJECT_EXEC_REQUEST, w.bytes)
}

pub fn decode_quantum_project_execution_request(
    bytes: &[u8],
) -> Result<QuantumProjectExecutionRequestWire, WireError> {
    let mut r = Reader::new(unframe(bytes, KIND_QUANTUM_PROJECT_EXEC_REQUEST)?);
    let out = QuantumProjectExecutionRequestWire {
        project_id: r.u64()?,
        project_trace_id: r.u64()?,
        project_context_sha256: r.string()?,
        payload_sha256: r.string()?,
        payload: r.string()?,
    };
    r.finish()?;
    Ok(out)
}

pub fn encode_quantum_project_execution_response(
    x: &QuantumProjectExecutionResponseWire,
) -> Vec<u8> {
    let mut w = Writer::new();
    w.bool(x.completed);
    w.u8(x.reason_code);
    w.u64(x.project_id);
    w.u64(x.project_trace_id);
    w.string(&x.request_sha256);
    frame(KIND_QUANTUM_PROJECT_EXEC_RESPONSE, w.bytes)
}

pub fn decode_quantum_project_execution_response(
    bytes: &[u8],
) -> Result<QuantumProjectExecutionResponseWire, WireError> {
    let mut r = Reader::new(unframe(bytes, KIND_QUANTUM_PROJECT_EXEC_RESPONSE)?);
    let out = QuantumProjectExecutionResponseWire {
        completed: r.bool()?,
        reason_code: r.u8()?,
        project_id: r.u64()?,
        project_trace_id: r.u64()?,
        request_sha256: r.string()?,
    };
    r.finish()?;
    Ok(out)
}

fn put_candidate_statistic(w: &mut Writer, candidate: &CandidateStatisticWire) {
    w.u64(candidate.candidate_id);
    w.u8(candidate.polarity);
    w.string(&candidate.domain);
    w.string(&candidate.term_sha256);
    w.string(&candidate.source_sha256);
    w.u8(candidate.security_y);
    w.u8(candidate.trust_x);
    w.u32(candidate.confidence_fixed);
    w.u32(candidate.frequency);
    w.u16(candidate.domain_count);
    w.u32(candidate.legacy_score_fixed);
}

fn get_candidate_statistic(r: &mut Reader<'_>) -> Result<CandidateStatisticWire, WireError> {
    Ok(CandidateStatisticWire {
        candidate_id: r.u64()?,
        polarity: r.u8()?,
        domain: r.string()?,
        term_sha256: r.string()?,
        source_sha256: r.string()?,
        security_y: r.u8()?,
        trust_x: r.u8()?,
        confidence_fixed: r.u32()?,
        frequency: r.u32()?,
        domain_count: r.u16()?,
        legacy_score_fixed: r.u32()?,
    })
}

fn put_candidates(w: &mut Writer, candidates: &[CandidateStatisticWire]) {
    w.u32(candidates.len() as u32);
    for candidate in candidates {
        put_candidate_statistic(w, candidate);
    }
}

fn get_candidates(r: &mut Reader<'_>) -> Result<Vec<CandidateStatisticWire>, WireError> {
    let count = r.count()?;
    let mut candidates = Vec::with_capacity(count);
    for _ in 0..count {
        candidates.push(get_candidate_statistic(r)?);
    }
    Ok(candidates)
}

pub fn encode_selection_request(request: &KnowledgeSelectionRequestWire) -> Vec<u8> {
    let mut w = Writer::new();
    w.u16(request.version);
    w.u8(request.requester as u8);
    w.u8(request.purpose as u8);
    w.string(&request.session_id);
    w.string(&request.parent_i0);
    w.u64(request.primitive_anchor);
    w.string(&request.trace_id);
    w.string(&request.split_sha256);
    w.vec_string(&request.xi);
    w.vec_string(&request.yi);
    w.option_string(&request.upstream_hold_sha256);
    w.string(&request.evidence_sha256);
    w.u16(request.max_positive);
    w.u16(request.max_negative);
    frame(KIND_SELECTION_REQUEST, w.bytes)
}

pub fn decode_selection_request(bytes: &[u8]) -> Result<KnowledgeSelectionRequestWire, WireError> {
    let mut r = Reader::new(unframe(bytes, KIND_SELECTION_REQUEST)?);
    let request = KnowledgeSelectionRequestWire {
        version: r.u16()?,
        requester: SelectionRequester::decode(r.u8()?)?,
        purpose: SelectionPurpose::decode(r.u8()?)?,
        session_id: r.string()?,
        parent_i0: r.string()?,
        primitive_anchor: r.u64()?,
        trace_id: r.string()?,
        split_sha256: r.string()?,
        xi: r.vec_string()?,
        yi: r.vec_string()?,
        upstream_hold_sha256: r.option_string()?,
        evidence_sha256: r.string()?,
        max_positive: r.u16()?,
        max_negative: r.u16()?,
    };
    r.finish()?;
    Ok(request)
}

pub fn encode_selection_response(response: &KnowledgeSelectionResponseWire) -> Vec<u8> {
    let mut w = Writer::new();
    w.u16(response.version);
    w.u8(response.requester as u8);
    w.string(&response.session_id);
    w.string(&response.parent_i0);
    w.u64(response.primitive_anchor);
    w.string(&response.split_sha256);
    w.option_string(&response.upstream_hold_sha256);
    w.string(&response.request_sha256);
    w.string(&response.selection_sha256);
    w.string(&response.hold_id);
    w.string(&response.hold_sha256);
    put_candidates(&mut w, &response.positive);
    put_candidates(&mut w, &response.negative);
    w.u64(response.law_seal);
    w.u64(response.system_laws_seal);
    w.u64(response.expires_at_ns);
    frame(KIND_SELECTION_RESPONSE, w.bytes)
}

pub fn decode_selection_response(
    bytes: &[u8],
) -> Result<KnowledgeSelectionResponseWire, WireError> {
    let mut r = Reader::new(unframe(bytes, KIND_SELECTION_RESPONSE)?);
    let response = KnowledgeSelectionResponseWire {
        version: r.u16()?,
        requester: SelectionRequester::decode(r.u8()?)?,
        session_id: r.string()?,
        parent_i0: r.string()?,
        primitive_anchor: r.u64()?,
        split_sha256: r.string()?,
        upstream_hold_sha256: r.option_string()?,
        request_sha256: r.string()?,
        selection_sha256: r.string()?,
        hold_id: r.string()?,
        hold_sha256: r.string()?,
        positive: get_candidates(&mut r)?,
        negative: get_candidates(&mut r)?,
        law_seal: r.u64()?,
        system_laws_seal: r.u64()?,
        expires_at_ns: r.u64()?,
    };
    r.finish()?;
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn layer(
        layer_mask: u8,
        activation_id: u64,
        activation_contract_digest: u64,
        gcl_process_digest: u64,
        parent_result_digest: u64,
        result_material: Vec<u8>,
    ) -> PdLayerEvidenceWire {
        let result_material_digest = digest_parts(&[result_material.as_slice()]);
        let mut layer = PdLayerEvidenceWire {
            layer_mask,
            activation_id,
            activation_contract_digest,
            gcl_process_digest,
            parent_result_digest,
            result_material,
            result_material_digest,
            result_digest: 0,
            completed: true,
        };
        layer.result_digest = layer.recompute_digest();
        layer
    }

    fn pd_spine() -> PdSpineEvidenceWire {
        let contract_version = PdSpineEvidenceWire::CONTRACT_VERSION;
        let session_digest = digest_parts(&[b"s"]);
        let parent_i0_digest = digest_parts(&[b"i0"]);
        let trace_digest = digest_parts(&[b"trace"]);
        let turn_digest = digest_parts(&[b"turn"]);
        let created_at = 10u64;

        let initial_i_digest = digest_parts(&[b"s", b"GCL_INITIAL_I"]);
        let user_input_digest = digest_parts(&[b"input"]);
        let stimulus_digest = digest_parts(&[
            &PdContinuumEvidenceWire::CONTRACT_VERSION.to_le_bytes(),
            &session_digest.to_le_bytes(),
            &initial_i_digest.to_le_bytes(),
            &user_input_digest.to_le_bytes(),
            &parent_i0_digest.to_le_bytes(),
            &created_at.to_le_bytes(),
            b"I_PLUS_U_TO_I0",
        ]);
        let question_digest = digest_parts(&[b"question"]);
        let response_kind_digest = digest_parts(&[b"CLARIFY"]);
        let increment_digest = digest_parts(&[
            &PdContinuumEvidenceWire::CONTRACT_VERSION.to_le_bytes(),
            &stimulus_digest.to_le_bytes(),
            &PdContinuumEvidenceWire::QUESTION_INCREMENT_UNITS.to_le_bytes(),
            &question_digest.to_le_bytes(),
            &response_kind_digest.to_le_bytes(),
            &created_at.to_le_bytes(),
            b"I0_PLUS_ONE_QUESTION_IQ",
        ]);
        let continuum_activation_digest = digest_parts(&[
            &PdContinuumEvidenceWire::CONTRACT_VERSION.to_le_bytes(),
            &stimulus_digest.to_le_bytes(),
            &increment_digest.to_le_bytes(),
            b"PD_CONTINUUM_ACTIVATION",
        ]);
        let continuum = PdContinuumEvidenceWire {
            contract_version: PdContinuumEvidenceWire::CONTRACT_VERSION,
            session_digest,
            initial_i_digest,
            user_input_digest,
            i0_digest: parent_i0_digest,
            stimulus_digest,
            stimulus_created_at: created_at,
            stimulus_state: 0,
            unit_count: PdContinuumEvidenceWire::QUESTION_INCREMENT_UNITS,
            question_digest,
            response_kind_digest,
            increment_digest,
            increment_created_at: created_at,
            increment_state: 1,
            activation_digest: continuum_activation_digest,
            activation_state: 2,
        };

        let mut activation = PdActivationEvidenceWire {
            mode_code: 0,
            origin_code: 0,
            response_kind_digest,
            genius_signal_digest: 0,
            trace_mass: 0,
            structural_coherence: 0,
            required_layer_mask: PdSpineEvidenceWire::ALL_LAYERS_MASK,
            contract_digest: 0,
        };
        activation.contract_digest = activation.recompute_contract_digest().unwrap();
        let activation_contract_digest = activation.contract_digest;

        let gcl_law_seal = digest_parts(&[b"gjata_collapse_law"]);
        let gcl_system_laws_seal = digest_parts(&[b"ESS_MAI_SYSTEM_LAWS"]);
        let gcl_phase_digest = digest_parts(&[b"KOLAPS_ARSYETIMI"]);
        let gcl_process_digest = digest_parts(&[
            &contract_version.to_le_bytes(),
            &gcl_law_seal.to_le_bytes(),
            &gcl_system_laws_seal.to_le_bytes(),
            &gcl_phase_digest.to_le_bytes(),
            &session_digest.to_le_bytes(),
            &parent_i0_digest.to_le_bytes(),
            &continuum_activation_digest.to_le_bytes(),
            &activation_contract_digest.to_le_bytes(),
            &created_at.to_le_bytes(),
            b"GCL_PD_SPINE_PROCESS_V160",
        ]);
        let activation_id = digest_parts(&[
            &contract_version.to_le_bytes(),
            &session_digest.to_le_bytes(),
            &parent_i0_digest.to_le_bytes(),
            &trace_digest.to_le_bytes(),
            &turn_digest.to_le_bytes(),
            &continuum_activation_digest.to_le_bytes(),
            &activation_contract_digest.to_le_bytes(),
            &gcl_process_digest.to_le_bytes(),
            &created_at.to_le_bytes(),
            b"PD_SPINE9_ACTIVATION_V160",
        ]);
        let layer1 = layer(
            0b001,
            activation_id,
            activation_contract_digest,
            gcl_process_digest,
            0,
            11u64.to_le_bytes().to_vec(),
        );
        let layer2 = layer(
            0b010,
            activation_id,
            activation_contract_digest,
            gcl_process_digest,
            layer1.result_digest,
            12u64.to_le_bytes().to_vec(),
        );
        let layer3 = layer(
            0b100,
            activation_id,
            activation_contract_digest,
            gcl_process_digest,
            layer2.result_digest,
            13u64.to_le_bytes().to_vec(),
        );
        let mut spine = PdSpineEvidenceWire {
            contract_version,
            activation_id,
            session_digest,
            parent_i0_digest,
            trace_digest,
            turn_digest,
            created_at,
            continuum,
            activation,
            continuum_activation_digest,
            activation_contract_digest,
            gcl_law_seal,
            gcl_system_laws_seal,
            gcl_phase_digest,
            gcl_process_digest,
            layer_mask: PdSpineEvidenceWire::ALL_LAYERS_MASK,
            layer1,
            layer2,
            layer3,
            evidence_digest: 14,
            mpro_mass: 5000,
            ready_for_shadow: true,
            completed_at: 15,
            completion_digest: 0,
        };
        spine.completion_digest = spine.recompute_completion_digest();
        spine
    }

    fn negative() -> NegativeKnowledgeWire {
        NegativeKnowledgeWire {
            input_id: 7,
            reason: 9,
            process: "npim".into(),
            mass: 0.4,
            frequency: 2,
            suggestion_code: 1,
            bytes: vec![1, 2, 3],
            ts_ns: 11,
        }
    }

    fn muscle_evidence() -> MuscleEvidenceWire {
        let mut evidence = MuscleEvidenceWire {
            besa_hold_id: "besa-hold".into(),
            besa_hold_sha256: "11".repeat(32),
            besa_selection_sha256: "22".repeat(32),
            besa_attestation_sha256: "33".repeat(32),
            asht_hold_id: "asht-hold".into(),
            asht_hold_sha256: "44".repeat(32),
            asht_selection_sha256: "55".repeat(32),
            asht_attestation_sha256: "66".repeat(32),
            split_sha256: "77".repeat(32),
            combined_sha256: String::new(),
        };
        evidence.combined_sha256 = evidence.recompute_combined_sha256();
        evidence
    }

    #[test]
    fn cycle_roundtrip_and_checksum() {
        let request = ShadowCycleRequest {
            quantum: QuantumInboundWire {
                package_id: "p".into(),
                session_id: "s".into(),
                territory: "t".into(),
                epistemic_mass: 0.5,
                evidence_density: 0.6,
                logical_coherence: 0.7,
                causal_integrity: 0.8,
                convergence_strength: 0.9,
                reproducibility: 1.0,
                potentiality_score: 0.4,
                ibe_score: 0.3,
                chain_coherence: 0.2,
                conservation_ok: true,
                candidate_scores: vec![0.1, 0.9],
                suggested_verdict: 1,
                primitive_flags: 0xA451,
                raw_bytes: vec![4, 5],
                xy_x: Some("x".into()),
                xy_y: Some("y".into()),
                xy_mass: Some(500),
                pd_binding_digest: Some(12),
                pd_continuum_activation_digest: Some(13),
                pd_parent_i0: Some("i0".into()),
                collapse_refused: None,
                negative_paths: vec!["n".into()],
                hcp_input_id: Some(1),
                hcp_token_gen: Some(2),
                hcp_token_nonce: Some(3),
                hcp_directive_bit: Some(1),
                final_evidence: {
                    let mut e = FinalEvidenceWire {
                        pd_spine: pd_spine(),
                        muscle_evidence: muscle_evidence(),
                        quantum_action_state: {
                            let evidence = vec![
                                ActionEvidenceWire {
                                    stage: 1,
                                    evidence_words: vec![11; 5],
                                },
                                ActionEvidenceWire {
                                    stage: 2,
                                    evidence_words: vec![22; 3],
                                },
                                ActionEvidenceWire {
                                    stage: 3,
                                    evidence_words: vec![33; 4],
                                },
                                ActionEvidenceWire {
                                    stage: 8,
                                    evidence_words: vec![88; 5],
                                },
                                ActionEvidenceWire {
                                    stage: 4,
                                    evidence_words: vec![44; 6],
                                },
                                ActionEvidenceWire {
                                    stage: 7,
                                    evidence_words: vec![77; 6],
                                },
                                ActionEvidenceWire {
                                    stage: 5,
                                    evidence_words: vec![55; 4],
                                },
                                ActionEvidenceWire {
                                    stage: 6,
                                    evidence_words: vec![66; 21],
                                },
                                ActionEvidenceWire {
                                    stage: 9,
                                    evidence_words: vec![99; 7],
                                },
                            ];
                            replay_action_evidence(&evidence).0
                        },
                        quantum_action_evidence: vec![
                            ActionEvidenceWire {
                                stage: 1,
                                evidence_words: vec![11; 5],
                            },
                            ActionEvidenceWire {
                                stage: 2,
                                evidence_words: vec![22; 3],
                            },
                            ActionEvidenceWire {
                                stage: 3,
                                evidence_words: vec![33; 4],
                            },
                            ActionEvidenceWire {
                                stage: 8,
                                evidence_words: vec![88; 5],
                            },
                            ActionEvidenceWire {
                                stage: 4,
                                evidence_words: vec![44; 6],
                            },
                            ActionEvidenceWire {
                                stage: 7,
                                evidence_words: vec![77; 6],
                            },
                            ActionEvidenceWire {
                                stage: 5,
                                evidence_words: vec![55; 4],
                            },
                            ActionEvidenceWire {
                                stage: 6,
                                evidence_words: vec![66; 21],
                            },
                            ActionEvidenceWire {
                                stage: 9,
                                evidence_words: vec![99; 7],
                            },
                        ],
                        quantum_action_mask: REQUIRED_ACTION_MASK,
                        quantum_required_action_mask: REQUIRED_ACTION_MASK,
                        light_input_sha256: "00".repeat(32),
                        light_input_bytes: b"input".to_vec(),
                        pim_evidence_density: 6000,
                        pim_logical_coherence: 7000,
                        pim_causal_integrity: 8000,
                        pim_convergence_strength: 9000,
                        pim_reproducibility: 10000,
                        pim_potentiality_score: 4000,
                        pim_suggestion: 1,
                        pim_proof_chain: vec!["proof".into()],
                        npim_failure_density: 4000,
                        npim_contradiction_strength: 3000,
                        npim_elimination_certainty: 5000,
                        npim_negativity_score: 4000,
                        npim_suggestion: 1,
                        npim_arguments: vec!["negative".into()],
                        npim_arguments_blob_digest: fnv1a64(&[1, 2, 3]),
                        mpro_measurements: vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0],
                        mpro_positives: 8,
                        mpro_total: 16,
                        mpro_vector_mass: 5000,
                        mpro_evidence_mass: 5000,
                        mpro_factic_mass: 2500,
                        scientific_project: None,
                        package_digest: 0,
                    };
                    e.package_digest = e.recompute_digest();
                    e
                },
            },
            light: LightInboundWire {
                session_id: "s".into(),
                territory: "t".into(),
                primitive_flags: 0xA451,
                epistemic_mass: 0.5,
                proof_chain: vec![8],
                payload: vec![9],
            },
            negative: negative(),
        };
        let bytes = encode_cycle_request(&request);
        assert!(request.quantum.final_evidence.verifies_internal());
        assert_eq!(decode_cycle_request(&bytes).unwrap(), request);
        let mut corrupted = bytes;
        corrupted[25] ^= 1;
        assert_eq!(
            decode_cycle_request(&corrupted),
            Err(WireError::InvalidChecksum)
        );
    }

    #[test]
    fn action_ledger_replay_rejects_tamper_duplicate_and_wrong_schema() {
        let canonical = vec![
            ActionEvidenceWire {
                stage: 1,
                evidence_words: vec![11; 5],
            },
            ActionEvidenceWire {
                stage: 2,
                evidence_words: vec![22; 3],
            },
            ActionEvidenceWire {
                stage: 3,
                evidence_words: vec![33; 4],
            },
            ActionEvidenceWire {
                stage: 8,
                evidence_words: vec![88; 5],
            },
            ActionEvidenceWire {
                stage: 4,
                evidence_words: vec![44; 6],
            },
            ActionEvidenceWire {
                stage: 7,
                evidence_words: vec![77; 6],
            },
            ActionEvidenceWire {
                stage: 5,
                evidence_words: vec![55; 4],
            },
            ActionEvidenceWire {
                stage: 6,
                evidence_words: vec![66; 21],
            },
            ActionEvidenceWire {
                stage: 9,
                evidence_words: vec![99; 7],
            },
        ];
        let baseline = replay_action_evidence(&canonical);
        assert!(baseline.2);
        assert_eq!(baseline.1, REQUIRED_ACTION_MASK);

        let mut tampered = canonical.clone();
        tampered[4].evidence_words[0] ^= 1;
        assert_ne!(replay_action_evidence(&tampered).0, baseline.0);

        let mut duplicate = canonical.clone();
        duplicate[4].stage = ACTION_STAGE_NPRO;
        assert!(!replay_action_evidence(&duplicate).2);

        let mut wrong_schema = canonical;
        wrong_schema[0].evidence_words.pop();
        assert!(!replay_action_evidence(&wrong_schema).2);
    }

    #[test]
    fn selection_roundtrip_and_bounds_are_fail_closed() {
        let request = KnowledgeSelectionRequestWire {
            version: PROTOCOL_VERSION,
            requester: SelectionRequester::BesaNlight,
            purpose: SelectionPurpose::XiYiStatistics,
            session_id: "session".into(),
            parent_i0: "i0".into(),
            primitive_anchor: 7,
            trace_id: "trace".into(),
            split_sha256: "11".repeat(32),
            xi: vec!["alpha".into()],
            yi: vec!["beta".into()],
            upstream_hold_sha256: None,
            evidence_sha256: "22".repeat(32),
            max_positive: 4,
            max_negative: 4,
        };
        assert!(request.shape_is_valid());
        assert_eq!(
            decode_selection_request(&encode_selection_request(&request)).unwrap(),
            request
        );

        let candidate = CandidateStatisticWire {
            candidate_id: 1,
            polarity: 1,
            domain: "science".into(),
            term_sha256: "33".repeat(32),
            source_sha256: "44".repeat(32),
            security_y: 1,
            trust_x: 1,
            confidence_fixed: 8_000,
            frequency: 2,
            domain_count: 1,
            legacy_score_fixed: 7_000,
        };
        let response = KnowledgeSelectionResponseWire {
            version: PROTOCOL_VERSION,
            requester: SelectionRequester::BesaNlight,
            session_id: request.session_id.clone(),
            parent_i0: request.parent_i0.clone(),
            primitive_anchor: request.primitive_anchor,
            split_sha256: request.split_sha256.clone(),
            upstream_hold_sha256: request.upstream_hold_sha256.clone(),
            request_sha256: "55".repeat(32),
            selection_sha256: "66".repeat(32),
            hold_id: "hold".into(),
            hold_sha256: "77".repeat(32),
            positive: vec![candidate],
            negative: Vec::new(),
            law_seal: 1,
            system_laws_seal: 2,
            expires_at_ns: 3,
        };
        assert!(response.shape_is_valid_for(&request));
        assert_eq!(
            decode_selection_response(&encode_selection_response(&response)).unwrap(),
            response
        );

        let mut oversized = request;
        oversized.max_positive = SELECTION_MAX_CANDIDATES + 1;
        assert!(!oversized.shape_is_valid());
    }

    #[test]
    fn muscle_evidence_digest_detects_split_tamper() {
        let mut evidence = muscle_evidence();
        assert!(evidence.verifies_internal());
        evidence.split_sha256 = "88".repeat(32);
        assert!(!evidence.verifies_internal());
    }

    #[test]
    fn response_roundtrip_binds_living_trust_to_receipt() {
        let response = ShadowCycleResponse {
            session_id: "s".into(),
            verdict: ShadowVerdictWire {
                verified: 1,
                primitive: 1,
                knowledge_band: 2,
                lgc_law: 0x47,
                legacy_score: 0.75,
                lgc_dsl: "LGC::SHADOW|PRIMITIVE|fact|LAW_1000111".into(),
                living_trust_action_state: 0x1122_3344_5566_7788,
                living_trust_action_mask: REQUIRED_ACTION_MASK,
                living_trust_required_action_mask: REQUIRED_ACTION_MASK,
                living_trust_system_laws_seal: 0x8877_6655_4433_2211,
                living_trust_sovereign_flags: 0xA451,
                living_trust_sovereign_value: 500,
                living_trust_sha256: "ab".repeat(32),
                living_trust_intensity: 7_500,
                living_trust_kind: 1,
                project_id: 0,
                project_status: PROJECT_STATUS_NONE,
                project_context_digest: 0,
                project_evidence_digest: 0,
                project_context_sha256: "00".repeat(32),
                project_evidence_sha256: "00".repeat(32),
                novel_factualized: false,
                novel_trl_level: 0,
                novel_proof_score: 0,
                novel_rejection: 0,
            },
            verification_receipt: VerificationReceiptWire {
                session_id: "s".into(),
                parent_i0: "i0".into(),
                primitive_anchor: 7,
                xy_digest: 9,
                pd_binding_digest: 11,
                pd_continuum_activation_digest: 13,
                living_trust_digest: 15,
                living_trust_sha256: "ab".repeat(32),
                y_verdict: 1,
                x_verdict: 1,
                seal: 17,
                generation: 19,
                receipt_id: "r".into(),
            },
            negative_persisted: true,
        };
        assert_eq!(
            decode_cycle_response(&encode_cycle_response(&response)).unwrap(),
            response,
        );
    }

    #[test]
    fn negative_roundtrip() {
        let request = ShadowNegativeRequest {
            negative: negative(),
            export_negative_knowledge: true,
        };
        assert_eq!(
            decode_negative_request(&encode_negative_request(&request)).unwrap(),
            request
        );
    }
    #[test]
    fn scientific_project_wire_roundtrip_binds_research_question() {
        let project = ScientificProjectWire {
            context: ProjectContextWitnessWire {
                project_id: 77,
                user_id: 9,
                trace_id: 88,
                revision: 1,
                content_sha256: "aa".repeat(32),
                light_sovereign_flags: 0xA451,
                context_sha256: "bb".repeat(32),
            },
            project_title: "motor novel".into(),
            domain: "science".into(),
            hypothesis: "X prodhon Y".into(),
            assumptions: vec!["A".into(), "B".into()],
            gcl_process_digest: 11,
            trl_level: 3,
            trl_passed: true,
            trl_confidence: 8_000,
            trl_reproducibility: 7_000,
            lab_test_id: "LAB-77".into(),
            findings: "evidence".into(),
            description: "si doli risia dhe si u provua".into(),
            files: vec![ProjectEvidenceFileWire {
                kind: PROJECT_EVIDENCE_DOCUMENT,
                bytes: b"document".to_vec(),
            }],
            evidence_sha256: "cc".repeat(32),
        };
        let mut writer = Writer::new();
        put_scientific_project(&mut writer, &project);
        let mut reader = Reader::new(&writer.bytes);
        assert!(project.shape_is_valid());
        assert_eq!(get_scientific_project(&mut reader).unwrap(), project);
        assert_eq!(reader.finish(), Ok(()));

        let mut illegal_shadow_level = project.clone();
        illegal_shadow_level.trl_level = SHADOW_FACTUAL_TRL;
        assert!(!illegal_shadow_level.shape_is_valid());
    }

    #[test]
    fn quantum_project_execution_roundtrip_binds_payload_and_context() {
        let request = QuantumProjectExecutionRequestWire {
            project_id: 77,
            project_trace_id: 88,
            project_context_sha256: "aa".repeat(32),
            payload_sha256: "bb".repeat(32),
            payload: "trace_id=88;text=project".into(),
        };
        assert_eq!(
            decode_quantum_project_execution_request(&encode_quantum_project_execution_request(
                &request
            ),)
            .unwrap(),
            request,
        );
        let response = QuantumProjectExecutionResponseWire {
            completed: true,
            reason_code: 0,
            project_id: 77,
            project_trace_id: 88,
            request_sha256: "dd".repeat(32),
        };
        assert_eq!(
            decode_quantum_project_execution_response(&encode_quantum_project_execution_response(
                &response
            ),)
            .unwrap(),
            response,
        );
    }

    #[test]
    fn project_intake_and_registration_roundtrip() {
        let request = LightProjectIntakeRequestWire {
            project_name: "motor novel".into(),
            project_description: "projekt shkencor".into(),
            project_content: "materiali i projektit".into(),
            domain: "science".into(),
            lang_code: "sq".into(),
            evolve_summary: "trace".into(),
            hypothesis: "X prodhon Y".into(),
            assumptions: vec!["A".into(), "B".into()],
            documentation_description: "si u ndërtua dhe si u provua".into(),
            files: vec![ProjectEvidenceFileWire {
                kind: PROJECT_EVIDENCE_DOCUMENT,
                bytes: b"evidence".to_vec(),
            }],
        };
        assert_eq!(
            decode_light_project_intake_request(&encode_light_project_intake_request(&request))
                .unwrap(),
            request,
        );

        let witness = ProjectContextWitnessWire {
            project_id: 77,
            user_id: 9,
            trace_id: 88,
            revision: 0,
            content_sha256: "aa".repeat(32),
            light_sovereign_flags: 0xA451,
            context_sha256: "bb".repeat(32),
        };
        let registration = ProjectRegistrationResponseWire {
            accepted: true,
            reason_code: 0,
            witness: Some(witness.clone()),
        };
        assert_eq!(
            decode_project_registration_response(&encode_project_registration_response(
                &registration
            ))
            .unwrap(),
            registration,
        );
        let response = LightProjectIntakeResponseWire {
            accepted_into_gcl: true,
            reason_code: 0,
            witness: Some(witness),
        };
        assert_eq!(
            decode_light_project_intake_response(&encode_light_project_intake_response(&response))
                .unwrap(),
            response,
        );
    }
}
