//! Besa Nlight: subordinate enrichment muscle commanded by AKA.

use crate::alnur_karina_athar::{BesaAttestation, BesaCommand};
use crate::lab_contracts::{CollapsePhase, GjataCollapseLaw, SupremeDirective};
use shadow_contracts::{
    CandidateStatisticWire, KnowledgeSelectionRequestWire, KnowledgeSelectionResponseWire,
    SelectionRequester, SELECTION_MAX_CANDIDATES,
};
use std::marker::PhantomData;
use std::time::{SystemTime, UNIX_EPOCH};

const NEGATIVE_TERM_DOMAIN: &[u8] = b"ESSMAI/SELECTION/TERM/NEGATIVE";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NegativeAxisIndexWitness {
    pub xi_sha256: String,
    pub yi_sha256: String,
    pub combined_sha256: String,
}

/// Light nuk klasifikon dhe nuk rendit kandidatët negativë. Ai zgjon vetëm
/// adresat deterministe që Quantum përdor tashmë për të kërkuar −Xi/−Yi më
/// thellë. Renditja e termave ruhet byte-për-byte dhe hyn në provën GCL.
pub fn negative_axis_index_witness(
    xi: &[String],
    yi: &[String],
) -> NegativeAxisIndexWitness {
    fn axis_digest(axis: &[u8], terms: &[String]) -> String {
        let mut material = Vec::new();
        for term in terms {
            let address = shadow_contracts::sha256_hex(&[NEGATIVE_TERM_DOMAIN, term.as_bytes()]);
            material.extend_from_slice(&(term.len() as u64).to_le_bytes());
            material.extend_from_slice(term.as_bytes());
            material.extend_from_slice(address.as_bytes());
        }
        shadow_contracts::sha256_hex(&[
            b"ESSMAI/GCL/NEGATIVE-AXIS-INDEX",
            axis,
            material.as_slice(),
        ])
    }

    let xi_sha256 = axis_digest(b"-Xi", xi);
    let yi_sha256 = axis_digest(b"-Yi", yi);
    let combined_sha256 = shadow_contracts::sha256_hex(&[
        b"ESSMAI/GCL/NEGATIVE-AXIS-PAIR",
        xi_sha256.as_bytes(),
        yi_sha256.as_bytes(),
    ]);
    NegativeAxisIndexWitness {
        xi_sha256,
        yi_sha256,
        combined_sha256,
    }
}

pub(crate) fn selection_request_digest(request: &KnowledgeSelectionRequestWire) -> String {
    let request_bytes = shadow_contracts::encode_selection_request(request);
    shadow_contracts::sha256_hex(&[
        b"ESSMAI/SELECTION/REQUEST/V178",
        request_bytes.as_slice(),
    ])
}

fn append_candidate_material(material: &mut Vec<u8>, candidate: &CandidateStatisticWire) {
    material.extend_from_slice(&candidate.candidate_id.to_le_bytes());
    material.push(candidate.polarity);
    material.extend_from_slice(&(candidate.domain.len() as u64).to_le_bytes());
    material.extend_from_slice(candidate.domain.as_bytes());
    material.extend_from_slice(candidate.term_sha256.as_bytes());
    material.extend_from_slice(candidate.source_sha256.as_bytes());
    material.push(candidate.security_y);
    material.push(candidate.trust_x);
    material.extend_from_slice(&candidate.confidence_fixed.to_le_bytes());
    material.extend_from_slice(&candidate.frequency.to_le_bytes());
    material.extend_from_slice(&candidate.domain_count.to_le_bytes());
    material.extend_from_slice(&candidate.legacy_score_fixed.to_le_bytes());
}

pub(crate) fn selection_response_digest(response: &KnowledgeSelectionResponseWire) -> String {
    let mut material = Vec::new();
    for candidate in response.positive.iter().chain(response.negative.iter()) {
        append_candidate_material(&mut material, candidate);
    }
    shadow_contracts::sha256_hex(&[
        b"ESSMAI/SELECTION/CANDIDATES/V178",
        material.as_slice(),
    ])
}

pub(crate) fn selection_hold_digest(response: &KnowledgeSelectionResponseWire) -> String {
    let requester = [response.requester as u8];
    let anchor = response.primitive_anchor.to_le_bytes();
    let law_seal = response.law_seal.to_le_bytes();
    let system_laws_seal = response.system_laws_seal.to_le_bytes();
    let expires_at_ns = response.expires_at_ns.to_le_bytes();
    shadow_contracts::sha256_hex(&[
        b"ESSMAI/SELECTION/HOLD/V178",
        &requester,
        response.session_id.as_bytes(),
        response.parent_i0.as_bytes(),
        &anchor,
        response.split_sha256.as_bytes(),
        response
            .upstream_hold_sha256
            .as_deref()
            .unwrap_or("")
            .as_bytes(),
        response.request_sha256.as_bytes(),
        response.selection_sha256.as_bytes(),
        response.hold_id.as_bytes(),
        &law_seal,
        &system_laws_seal,
        &expires_at_ns,
    ])
}

fn now_ns() -> Result<u64, String> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos().min(u128::from(u64::MAX)) as u64)
        .map_err(|error| format!("Besa selection clock failed: {error}"))
}

fn verify_complete_response(
    request: &KnowledgeSelectionRequestWire,
    response: &KnowledgeSelectionResponseWire,
) -> Result<(), String> {
    match selection_request_digest(request) == response.request_sha256 {
        true => {}
        false => {
            return Err("Besa rejected Shadow request SHA-256 drift".to_string())
        }
    }
    match selection_response_digest(response) == response.selection_sha256 {
        true => {}
        false => {
            return Err("Besa rejected Shadow candidate SHA-256 drift".to_string())
        }
    }
    match selection_hold_digest(response) == response.hold_sha256 {
        true => {}
        false => return Err("Besa rejected Shadow hold SHA-256 drift".to_string()),
    }
    match response.expires_at_ns > now_ns()? {
        true => {}
        false => {
            return Err("Besa rejected an expired Shadow selection hold".to_string())
        }
    }
    match response.positive.len() < usize::from(request.max_positive) {
        true => {}
        false => {
            return Err(
                "Besa cannot prove complete positive candidate enumeration".to_string(),
            )
        }
    }
    match response.negative.len() < usize::from(request.max_negative) {
        true => {}
        false => {
            return Err(
                "Besa cannot prove complete negative candidate enumeration".to_string(),
            )
        }
    }
    Ok(())
}

pub struct Commanded;
pub struct LawGated;
pub struct ShadowHeld;
pub struct Prepared;

pub struct BesaNlight<State> {
    command: BesaCommand,
    request: Option<KnowledgeSelectionRequestWire>,
    selection: Option<KnowledgeSelectionResponseWire>,
    negative_indices: Option<NegativeAxisIndexWitness>,
    enrichment_sha256: Option<String>,
    _state: PhantomData<State>,
}

impl BesaNlight<Commanded> {
    pub fn from_command(command: BesaCommand) -> Self {
        Self {
            command,
            request: None,
            selection: None,
            negative_indices: None,
            enrichment_sha256: None,
            _state: PhantomData,
        }
    }

    pub fn commission(self, directive: &SupremeDirective) -> Result<BesaNlight<LawGated>, String> {
        match (
            GjataCollapseLaw::verify(directive, CollapsePhase::Coordination),
            directive.seal == self.command.law_seal,
            self.command.system_laws_seal == crate::lab_contracts::system_laws_seal(),
        ) {
            (true, true, true) => Ok(BesaNlight {
                command: self.command,
                request: None,
                selection: None,
                negative_indices: None,
                enrichment_sha256: None,
                _state: PhantomData,
            }),
            (false, _, _) => Err("Besa rejected non-Coordination GCL directive".to_string()),
            (_, false, _) => Err("Besa rejected GCL law-seal drift".to_string()),
            (_, _, false) => Err("Besa rejected system-laws seal drift".to_string()),
        }
    }
}

impl BesaNlight<LawGated> {
    pub fn request_xi_yi_statistics(
        self,
        request: KnowledgeSelectionRequestWire,
    ) -> Result<BesaNlight<ShadowHeld>, String> {
        match (
            request.requester,
            request.shape_is_valid(),
            request.trace_id == self.command.trace_id,
            request.split_sha256 == self.command.split_sha256,
            request.max_positive == SELECTION_MAX_CANDIDATES,
            request.max_negative == SELECTION_MAX_CANDIDATES,
        ) {
            (SelectionRequester::BesaNlight, true, true, true, true, true) => {}
            (SelectionRequester::AshtQuantum, _, _, _, _, _) => {
                return Err("Besa cannot issue an Asht selection request".to_string())
            }
            (_, false, _, _, _, _) => {
                return Err("Besa selection request shape invalid".to_string())
            }
            (_, _, false, _, _, _) => return Err("Besa trace identity mismatch".to_string()),
            (_, _, _, false, _, _) => return Err("Besa split identity mismatch".to_string()),
            (_, _, _, _, false, _) => {
                return Err("Besa positive candidate limit is incomplete".to_string())
            }
            (_, _, _, _, _, false) => {
                return Err("Besa negative candidate limit is incomplete".to_string())
            }
        }
        let response = crate::shadow_selection_bridge::execute_selection(&request)?;
        verify_complete_response(&request, &response)?;
        match (
            response.shape_is_valid_for(&request),
            response.law_seal == self.command.law_seal,
            response.system_laws_seal == self.command.system_laws_seal,
        ) {
            (true, true, true) => Ok(BesaNlight {
                command: self.command,
                negative_indices: Some(negative_axis_index_witness(&request.xi, &request.yi)),
                request: Some(request),
                selection: Some(response),
                enrichment_sha256: None,
                _state: PhantomData,
            }),
            (false, _, _) => {
                Err("Besa rejected an unbound Shadow selection response".to_string())
            }
            (_, false, _) => Err("Besa rejected Shadow GCL law-seal drift".to_string()),
            (_, _, false) => {
                Err("Besa rejected Shadow system-laws seal drift".to_string())
            }
        }
    }
}

impl BesaNlight<ShadowHeld> {
    pub fn weave_context(self) -> Result<BesaNlight<Prepared>, String> {
        let request = self
            .request
            .ok_or_else(|| "Besa cannot weave without its GCL request".to_string())?;
        let selection = self
            .selection
            .ok_or_else(|| "Besa cannot weave without a Shadow hold".to_string())?;
        let negative_indices = self
            .negative_indices
            .ok_or_else(|| "Besa cannot weave without −Xi/−Yi indices".to_string())?;
        let mut candidate_material = Vec::new();
        for candidate in selection.positive.iter().chain(selection.negative.iter()) {
            append_candidate_material(&mut candidate_material, candidate);
        }
        let positive_count = (selection.positive.len() as u64).to_le_bytes();
        let negative_count = (selection.negative.len() as u64).to_le_bytes();
        let law_seal = selection.law_seal.to_le_bytes();
        let system_laws_seal = selection.system_laws_seal.to_le_bytes();
        let expires_at_ns = selection.expires_at_ns.to_le_bytes();
        let enrichment_sha256 = shadow_contracts::sha256_hex(&[
            b"ESSMAI/BESA/ENRICHMENT/V178",
            self.command.trace_id.as_bytes(),
            self.command.split_sha256.as_bytes(),
            request.evidence_sha256.as_bytes(),
            selection.request_sha256.as_bytes(),
            selection.hold_sha256.as_bytes(),
            selection.selection_sha256.as_bytes(),
            negative_indices.xi_sha256.as_bytes(),
            negative_indices.yi_sha256.as_bytes(),
            negative_indices.combined_sha256.as_bytes(),
            &positive_count,
            &negative_count,
            &law_seal,
            &system_laws_seal,
            &expires_at_ns,
            candidate_material.as_slice(),
        ]);
        Ok(BesaNlight {
            command: self.command,
            request: Some(request),
            selection: Some(selection),
            negative_indices: Some(negative_indices),
            enrichment_sha256: Some(enrichment_sha256),
            _state: PhantomData,
        })
    }
}

impl BesaNlight<Prepared> {
    pub fn attest(self) -> Result<BesaAttestation, String> {
        let request = self
            .request
            .ok_or_else(|| "Besa request missing at attestation".to_string())?;
        let selection = self
            .selection
            .ok_or_else(|| "Besa selection missing at attestation".to_string())?;
        let negative_indices = self
            .negative_indices
            .ok_or_else(|| "Besa −Xi/−Yi indices missing at attestation".to_string())?;
        let enrichment_sha256 = self
            .enrichment_sha256
            .ok_or_else(|| "Besa enrichment missing at attestation".to_string())?;
        Ok(BesaAttestation {
            trace_id: self.command.trace_id,
            split_sha256: self.command.split_sha256,
            request_sha256: selection.request_sha256,
            hold_id: selection.hold_id,
            hold_sha256: selection.hold_sha256,
            selection_sha256: selection.selection_sha256,
            xi_negative_index_sha256: negative_indices.xi_sha256,
            yi_negative_index_sha256: negative_indices.yi_sha256,
            negative_index_sha256: negative_indices.combined_sha256,
            positive_count: selection.positive.len(),
            negative_count: selection.negative.len(),
            max_positive: request.max_positive,
            max_negative: request.max_negative,
            law_seal: selection.law_seal,
            system_laws_seal: selection.system_laws_seal,
            expires_at_ns: selection.expires_at_ns,
            enrichment_sha256,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shadow_contracts::{SelectionPurpose, PROTOCOL_VERSION};

    fn request() -> KnowledgeSelectionRequestWire {
        KnowledgeSelectionRequestWire {
            version: PROTOCOL_VERSION,
            requester: SelectionRequester::BesaNlight,
            purpose: SelectionPurpose::XiYiStatistics,
            session_id: "T1_besa".to_string(),
            parent_i0: "11".repeat(32),
            primitive_anchor: 7,
            trace_id: "T1_besa".to_string(),
            split_sha256: "22".repeat(32),
            xi: vec!["motor".to_string()],
            yi: vec!["termik".to_string()],
            upstream_hold_sha256: None,
            evidence_sha256: "33".repeat(32),
            max_positive: SELECTION_MAX_CANDIDATES,
            max_negative: SELECTION_MAX_CANDIDATES,
        }
    }

    #[test]
    fn negative_axis_addresses_are_deterministic_and_axis_bound() {
        let first = negative_axis_index_witness(
            &["motor".to_string()],
            &["termik".to_string()],
        );
        let second = negative_axis_index_witness(
            &["motor".to_string()],
            &["termik".to_string()],
        );
        assert_eq!(first, second);
        assert_ne!(first.xi_sha256, first.yi_sha256);
        assert!(shadow_contracts::is_canonical_sha256(
            &first.combined_sha256
        ));
    }

    #[test]
    fn selection_binding_rejects_digest_expiry_and_truncation_drift() {
        let request = request();
        let response = crate::shadow_selection_bridge::execute_selection(&request)
            .expect("canonical test response");
        verify_complete_response(&request, &response).expect("complete response");

        let mut request_drift = response.clone();
        request_drift.request_sha256 = "00".repeat(32);
        assert!(verify_complete_response(&request, &request_drift).is_err());

        let mut expired = response.clone();
        expired.expires_at_ns = 1;
        expired.hold_sha256 = selection_hold_digest(&expired);
        assert!(verify_complete_response(&request, &expired).is_err());

        let mut truncated = response.clone();
        truncated.positive = vec![
            response
                .positive
                .first()
                .expect("positive fixture")
                .clone();
            usize::from(SELECTION_MAX_CANDIDATES)
        ];
        truncated.selection_sha256 = selection_response_digest(&truncated);
        truncated.hold_sha256 = selection_hold_digest(&truncated);
        assert!(verify_complete_response(&request, &truncated).is_err());
    }
}
