// ╔══════════════════════════════════════════════════════════════════════════╗
// ║ PROGRESSIVE_DEBATIC / seal.rs — GCL constitutional authorization       ║
// ║ ESS-MAI v1.6.0: GCL governs activation→Layers and verifies closure.    ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::pd_spine_contract::PdSpineCompletion;
use crate::sovereign::{LgcGate, LgcToken, SovErr};
use super::types::{
    pd_id, GclActionAuthorizationToken, PdAuthorizedCompletion, PdError,
    PdPreSealReceipt, PD_SEAL,
};

pub(crate) struct PdSealAuthority;

const LAW_CONTINUUM: u32 = 1 << 0;
const LAW_PD_ACTIVATION: u32 = 1 << 1;
const LAW_LAYER_LINEAGE: u32 = 1 << 2;
const LAW_GENIUS_RESEARCH: u32 = 1 << 3;
const LAW_READY_FOR_SHADOW: u32 = 1 << 4;
const REQUIRED_LAWS: u32 = LAW_CONTINUUM | LAW_PD_ACTIVATION | LAW_LAYER_LINEAGE | LAW_READY_FOR_SHADOW;

impl PdSealAuthority {
    pub(crate) fn authorize_ingest<T>(runtime_seal: u64, op: impl FnOnce() -> T) -> Result<T, PdError> {
        let _token = LgcGate::new(PD_SEAL).open(runtime_seal).map_err(pd_seal_error)?;
        Ok(op())
    }

    /// GCL/ESS-MAI consumes the raw completion. A successful collapse creates
    /// a new authorized state; the raw candidate is never moved while borrowed.
    pub(crate) fn authorize_after_spine(
        runtime_seal: u64,
        session_id: &str,
        parent_i0: &str,
        ts: u64,
        completion: PdSpineCompletion,
    ) -> Result<PdAuthorizedCompletion, PdError> {
        let gcl_continuous = completion.request.gcl.verifies_for(
            crate::lab_contracts::gjata_collapse_law::law_seal(),
            crate::lab_contracts::gjata_collapse_law::system_laws_seal(),
            completion.request.session_digest,
            completion.request.parent_i0_digest,
            completion.request.continuum.activation_digest,
            completion.request.activation.contract_digest,
            completion.request.created_at,
        );
        match (gcl_continuous, completion.closes_all_layers()) {
            (true, true) => {}
            (false, _) => return Err(PdError::BridgeFailed(
                "GCL refuzoi completion: Spine 9/Layer1→2→3 nuk mbajtën të njëjtin proces GCL".to_string(),
            )),
            (_, false) => return Err(PdError::BridgeFailed(
                "GCL refuzoi completion: PD/Spine9/Layer1→2→3 lineage nuk mbyllet".to_string(),
            )),
        }
        let token = LgcGate::new(PD_SEAL).open(runtime_seal).map_err(pd_seal_error)?;
        let law_mask = law_mask(&completion);
        if (law_mask & REQUIRED_LAWS) != REQUIRED_LAWS {
            return Err(PdError::BridgeFailed(
                "GCL refuzoi completion: ligjet kushtetuese të ESS-MAI nuk u plotësuan".to_string(),
            ));
        }
        let action_sha256 = action_sha256(session_id, parent_i0, &completion);
        let law_trace_sha256 = law_trace_sha256(action_sha256, law_mask, &completion);
        let gcl_token = GclActionAuthorizationToken {
            contract_version: crate::pd_spine_contract::PD_SPINE_CONTRACT_VERSION,
            action_sha256,
            law_trace_sha256,
            law_mask,
            verdict: 1,
            issued_at: ts,
        };
        let pre_seal = receipt_from_token(
            &token, session_id, parent_i0, ts, &completion, &gcl_token,
        );
        Ok(PdAuthorizedCompletion { completion, pre_seal, gcl_token })
    }

    pub(crate) fn verifies(receipt: &PdPreSealReceipt, session_id: &str, parent_i0: &str) -> bool {
        let expected = pd_id(&[
            session_id.as_bytes(), parent_i0.as_bytes(), &receipt.prepared_at.to_le_bytes(),
            &receipt.spine_activation_id.to_le_bytes(), &receipt.spine_completion_digest.to_le_bytes(),
            &[receipt.layer_mask], &receipt.mpro_mass.to_le_bytes(), &receipt.action_sha256,
            &receipt.law_trace_sha256, &PD_SEAL.to_le_bytes(), b"GCL_PD_PRESEAL_V160",
        ]);
        receipt.seal == PD_SEAL && receipt.session_id == session_id && receipt.parent_i0 == parent_i0
            && receipt.layer_mask == crate::pd_spine_contract::ALL_LAYERS_MASK
            && receipt.spine_activation_id != 0 && receipt.spine_completion_digest != 0
            && receipt.action_sha256 != [0; 32] && receipt.law_trace_sha256 != [0; 32]
            && receipt.receipt_id == expected
    }
}

fn law_mask(completion: &PdSpineCompletion) -> u32 {
    let mut mask = 0u32;
    mask |= (completion.request.continuum.is_ready() as u32) * LAW_CONTINUUM;
    mask |= (completion.request.activation.verifies() as u32) * LAW_PD_ACTIVATION;
    mask |= (completion.closes_all_layers() as u32) * LAW_LAYER_LINEAGE;
    mask |= (completion.ready_for_shadow as u32) * LAW_READY_FOR_SHADOW;
    mask |= ((completion.request.activation.origin
        == crate::pd_spine_contract::PdActivationOrigin::GeniusAutonomousResearch) as u32)
        * LAW_GENIUS_RESEARCH;
    mask
}

fn action_sha256(session_id: &str, parent_i0: &str, c: &PdSpineCompletion) -> [u8; 32] {
    let mut bytes = Vec::with_capacity(256);
    bytes.extend_from_slice(b"ESS_MAI_GCL_PD_ACTION_V160");
    bytes.extend_from_slice(session_id.as_bytes());
    bytes.extend_from_slice(parent_i0.as_bytes());
    bytes.extend_from_slice(&c.request.activation_id.to_le_bytes());
    bytes.extend_from_slice(&c.request.activation.contract_digest.to_le_bytes());
    bytes.extend_from_slice(&c.request.gcl.process_digest.to_le_bytes());
    bytes.extend_from_slice(&c.layer1.result_digest.to_le_bytes());
    bytes.extend_from_slice(&c.layer2.result_digest.to_le_bytes());
    bytes.extend_from_slice(&c.layer3.result_digest.to_le_bytes());
    bytes.extend_from_slice(&c.evidence_digest.to_le_bytes());
    bytes.extend_from_slice(&c.mpro_mass.raw().to_le_bytes());
    bytes.extend_from_slice(&c.completed_at.to_le_bytes());
    crate::sovereign::sha256(&bytes)
}

fn law_trace_sha256(action: [u8; 32], law_mask: u32, c: &PdSpineCompletion) -> [u8; 32] {
    let mut bytes = Vec::with_capacity(96);
    bytes.extend_from_slice(b"ESS_MAI_IS_GCL_LAW_TRACE_V160");
    bytes.extend_from_slice(&action);
    bytes.extend_from_slice(&law_mask.to_le_bytes());
    bytes.extend_from_slice(c.request.activation.mode.label().as_bytes());
    bytes.extend_from_slice(c.request.activation.origin.label().as_bytes());
    bytes.push(c.request.activation.required_layer_mask);
    bytes.extend_from_slice(&c.request.gcl.law_seal.to_le_bytes());
    bytes.extend_from_slice(&c.request.gcl.system_laws_seal.to_le_bytes());
    bytes.extend_from_slice(&c.request.gcl.process_digest.to_le_bytes());
    crate::sovereign::sha256(&bytes)
}

fn receipt_from_token(
    _token: &LgcToken, session_id: &str, parent_i0: &str, ts: u64,
    completion: &PdSpineCompletion, gcl: &GclActionAuthorizationToken,
) -> PdPreSealReceipt {
    let completion_digest = completion.completion_digest();
    let receipt_id = pd_id(&[
        session_id.as_bytes(), parent_i0.as_bytes(), &ts.to_le_bytes(),
        &completion.request.activation_id.to_le_bytes(), &completion_digest.to_le_bytes(),
        &[completion.layer_mask], &completion.mpro_mass.raw().to_le_bytes(),
        &gcl.action_sha256, &gcl.law_trace_sha256, &PD_SEAL.to_le_bytes(),
        b"GCL_PD_PRESEAL_V160",
    ]);
    PdPreSealReceipt {
        seal: PD_SEAL, session_id: session_id.to_string(), parent_i0: parent_i0.to_string(),
        receipt_id, prepared_at: ts, spine_activation_id: completion.request.activation_id,
        spine_completion_digest: completion_digest, layer_mask: completion.layer_mask,
        mpro_mass: completion.mpro_mass.raw(), action_sha256: gcl.action_sha256,
        law_trace_sha256: gcl.law_trace_sha256,
    }
}

fn pd_seal_error(err: SovErr) -> PdError { match err {
    SovErr::SealMismatch(delta) => PdError::SealInvalid(format!("delta=0x{delta:016X}")),
    SovErr::NonceMismatch { expected, actual } => PdError::SealInvalid(format!("nonce expected={expected} actual={actual}")),
    SovErr::AlreadyConsumed => PdError::SealInvalid("token already consumed".to_string()),
    SovErr::NotFound => PdError::SealInvalid("seal gate not found".to_string()),
}}
