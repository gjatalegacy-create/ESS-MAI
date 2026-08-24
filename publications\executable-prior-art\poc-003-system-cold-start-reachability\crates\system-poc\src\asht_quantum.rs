//! Asht Quantum: subordinate evidence-scoping muscle commanded by UCL.

use quantum_lim::candidate_weight::{
    candidate_mass, candidate_order, evaluate_parity, rank_candidates, ParityEvidence,
    ParityVerdict,
};
use quantum_lim::lab_contracts::{CollapsePhase, GjataCollapseLaw};
use quantum_lim::ultimatum_collapse_law::{AshtCommand, AshtOperatorDirective};
use sha2::{Digest, Sha256};
use shadow_contracts::{
    CandidateStatisticWire, KnowledgeSelectionRequestWire, KnowledgeSelectionResponseWire,
    MuscleEvidenceWire, SelectionRequester,
};
use std::marker::PhantomData;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BesaCompanion {
    pub session_id: String,
    pub parent_i0: String,
    pub primitive_anchor: u64,
    pub trace_id: String,
    pub split_sha256: String,
    pub hold_id: String,
    pub hold_sha256: String,
    pub selection_sha256: String,
    pub enrichment_sha256: String,
}

impl BesaCompanion {
    pub fn attestation_sha256(&self) -> String {
        shadow_contracts::sha256_hex(&[
            b"ESSMAI/BESA/ATTESTATION/V178",
            self.session_id.as_bytes(),
            self.parent_i0.as_bytes(),
            &self.primitive_anchor.to_le_bytes(),
            self.trace_id.as_bytes(),
            self.split_sha256.as_bytes(),
            self.hold_id.as_bytes(),
            self.hold_sha256.as_bytes(),
            self.selection_sha256.as_bytes(),
            self.enrichment_sha256.as_bytes(),
        ])
    }

    pub fn shape_is_valid(&self) -> bool {
        !self.session_id.is_empty()
            && shadow_contracts::is_canonical_sha256(&self.parent_i0)
            && self.primitive_anchor != 0
            && self.trace_id == self.session_id
            && shadow_contracts::is_canonical_sha256(&self.split_sha256)
            && !self.hold_id.is_empty()
            && shadow_contracts::is_canonical_sha256(&self.hold_sha256)
            && shadow_contracts::is_canonical_sha256(&self.selection_sha256)
            && shadow_contracts::is_canonical_sha256(&self.enrichment_sha256)
    }
}

pub fn read_besa_companion(
    path: &Path,
    session_id: &str,
    input_sha256: &str,
    primitive_anchor: u64,
    split: &quantum_lim::lab_contracts::PrimitiveSplit,
) -> Result<BesaCompanion, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|error| format!("Besa companion read failed: {error}"))?;
    let prefix = format!("{session_id}|");
    let line = content
        .lines()
        .filter(|candidate| candidate.starts_with(prefix.as_str()))
        .last()
        .ok_or_else(|| "Besa companion missing for session".to_string())?;
    match quantum_lim::lab_contracts::pa_wire::verify_line_generic(line, &[], 10) {
        quantum_lim::lab_contracts::pa_wire::PaWireVerdict::SealedOk => {}
        quantum_lim::lab_contracts::pa_wire::PaWireVerdict::LegacyOk => {
            return Err("Besa companion downgrade rejected".to_string())
        }
        quantum_lim::lab_contracts::pa_wire::PaWireVerdict::Corrupt(reason) => {
            return Err(format!("Besa companion corrupt: {reason}"))
        }
    }
    let body = line
        .rsplit_once("|c:")
        .map(|(value, _)| value)
        .ok_or_else(|| "Besa companion seal missing".to_string())?;
    let parts: Vec<&str> = body.split('|').collect();
    let parsed = match parts.as_slice() {
        [session, parent, pa_hex, trace, split_sha, hold_id, hold_sha, selection_sha, enrichment_sha] =>
        {
            let pa = u64::from_str_radix(pa_hex, 16)
                .map_err(|_| "Besa companion PA malformed".to_string())?;
            BesaCompanion {
                session_id: (*session).to_string(),
                parent_i0: (*parent).to_string(),
                primitive_anchor: pa,
                trace_id: (*trace).to_string(),
                split_sha256: (*split_sha).to_string(),
                hold_id: (*hold_id).to_string(),
                hold_sha256: (*hold_sha).to_string(),
                selection_sha256: (*selection_sha).to_string(),
                enrichment_sha256: (*enrichment_sha).to_string(),
            }
        }
        _ => return Err("Besa companion field count invalid".to_string()),
    };
    let expected_split_sha256 = lgc_split_sha256(split);
    match (
        parsed.shape_is_valid(),
        parsed.session_id == session_id,
        parsed.parent_i0 == input_sha256.to_ascii_lowercase(),
        parsed.primitive_anchor == primitive_anchor,
        parsed.split_sha256 == expected_split_sha256,
    ) {
        (true, true, true, true, true) => Ok(parsed),
        _ => Err("Besa companion lineage mismatch".to_string()),
    }
}

fn lgc_split_sha256(split: &quantum_lim::lab_contracts::PrimitiveSplit) -> String {
    let xi_sha = digest_terms("ESSMAI/LGC/XI/V2", &split.xi);
    let yi_sha = digest_terms("ESSMAI/LGC/YI/V2", &split.yi);
    domain_sha(
        "ESSMAI/LGC/SPLIT/V2",
        &[
            xi_sha.as_bytes(),
            yi_sha.as_bytes(),
            split.to_wire().as_bytes(),
        ],
    )
}

fn digest_terms(domain: &str, terms: &[String]) -> String {
    let mut canonical = Vec::new();
    for term in terms {
        canonical.extend_from_slice(&(term.len() as u64).to_le_bytes());
        canonical.extend_from_slice(term.as_bytes());
    }
    domain_sha(domain, &[canonical.as_slice()])
}

fn domain_sha(domain: &str, parts: &[&[u8]]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"ESSMAI-DOMAIN-SHA256-V1\0");
    hasher.update((domain.len() as u64).to_le_bytes());
    hasher.update(domain.as_bytes());
    for part in parts {
        hasher.update((part.len() as u64).to_le_bytes());
        hasher.update(part);
    }
    format!("{:x}", hasher.finalize())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AshtAttestation {
    pub hold_id: String,
    pub hold_sha256: String,
    pub selection_sha256: String,
    pub relevance_sha256: String,
    pub parity: ParityVerdict,
    pub positive_mass: u64,
    pub negative_mass: u64,
    pub ranked_negative_candidates: Vec<CandidateStatisticWire>,
    pub attestation_sha256: String,
    pub bounded_fragments: Vec<String>,
    pub operator_directive: AshtOperatorDirective,
}

impl AshtAttestation {
    pub fn parity_evidence(&self) -> ParityEvidence {
        ParityEvidence {
            verdict: self.parity,
            positive_mass: self.positive_mass,
            negative_mass: self.negative_mass,
        }
    }
}

pub struct Commanded;
pub struct LawGated;
pub struct ShadowHeld;

pub struct AshtQuantum<State> {
    command: AshtCommand,
    request: Option<KnowledgeSelectionRequestWire>,
    response: Option<KnowledgeSelectionResponseWire>,
    _state: PhantomData<State>,
}

impl AshtQuantum<Commanded> {
    pub fn from_command(command: AshtCommand) -> Self {
        Self {
            command,
            request: None,
            response: None,
            _state: PhantomData,
        }
    }

    pub fn commission(self) -> Result<AshtQuantum<LawGated>, String> {
        let directive = GjataCollapseLaw::issue(CollapsePhase::Reasoning);
        match (
            GjataCollapseLaw::verify(&directive, CollapsePhase::Reasoning),
            directive.seal == self.command.law_seal,
            self.command.system_laws_seal == quantum_lim::lab_contracts::system_laws_seal(),
        ) {
            (true, true, true) => Ok(AshtQuantum {
                command: self.command,
                request: None,
                response: None,
                _state: PhantomData,
            }),
            (false, _, _) => Err("Asht rejected non-Reasoning GCL directive".to_string()),
            (_, false, _) => Err("Asht rejected GCL law-seal drift".to_string()),
            (_, _, false) => Err("Asht rejected system-laws seal drift".to_string()),
        }
    }
}

impl AshtQuantum<LawGated> {
    pub fn request_evidence_candidates(
        self,
        request: KnowledgeSelectionRequestWire,
    ) -> Result<AshtQuantum<ShadowHeld>, String> {
        match (
            request.requester,
            request.shape_is_valid(),
            request.trace_id == self.command.trace_id,
            request.split_sha256 == self.command.split_sha256,
            request.upstream_hold_sha256.as_deref() == Some(self.command.besa_hold_sha256.as_str()),
        ) {
            (SelectionRequester::AshtQuantum, true, true, true, true) => {}
            (SelectionRequester::BesaNlight, _, _, _, _) => {
                return Err("Asht cannot issue a Besa selection request".to_string())
            }
            (_, false, _, _, _) => return Err("Asht selection shape invalid".to_string()),
            (_, _, false, _, _) => return Err("Asht trace identity mismatch".to_string()),
            (_, _, _, false, _) => return Err("Asht split identity mismatch".to_string()),
            (_, _, _, _, false) => return Err("Asht lost the upstream Besa hold".to_string()),
        }
        let response = crate::shadow_process_bridge::execute_selection(&request)?;
        match response.shape_is_valid_for(&request) {
            true => Ok(AshtQuantum {
                command: self.command,
                request: Some(request),
                response: Some(response),
                _state: PhantomData,
            }),
            false => Err("Asht rejected an unbound Shadow selection response".to_string()),
        }
    }
}

impl AshtQuantum<ShadowHeld> {
    pub fn attest(self) -> Result<AshtAttestation, String> {
        let request = self
            .request
            .ok_or_else(|| "Asht request missing at attestation".to_string())?;
        let response = self
            .response
            .ok_or_else(|| "Asht selection missing at attestation".to_string())?;

        let projection = quantum_lim::request_bound_relevance::project_request_bound(
            &request,
            &response,
        )?;
        let relevance_sha256 = projection.relevance_sha256.clone();

        let positive_candidates = projection
            .positive
            .iter()
            .map(|relevant| relevant.candidate.clone())
            .collect::<Vec<_>>();
        let mut ranked_negative_candidates = projection
            .negative
            .iter()
            .map(|relevant| relevant.candidate.clone())
            .collect::<Vec<_>>();
        rank_candidates(&mut ranked_negative_candidates);

        let parity_evidence = evaluate_parity(&positive_candidates, &ranked_negative_candidates);
        match parity_evidence.shape_is_valid(ranked_negative_candidates.len()) {
            true => {}
            false => return Err("Asht produced non-canonical negative parity evidence".to_string()),
        }

        let mut ranked_relevant = projection
            .positive
            .iter()
            .chain(projection.negative.iter())
            .collect::<Vec<_>>();
        ranked_relevant.sort_by(|left, right| {
            candidate_order(left.candidate, right.candidate)
                .then((left.axis as u8).cmp(&(right.axis as u8)))
        });

        let mut candidate_material = Vec::new();
        let mut candidate_sha256 = Vec::new();
        let mut bounded_fragments = Vec::new();

        for relevant in ranked_relevant {
            let candidate = relevant.candidate;
            let mass = candidate_mass(candidate);
            candidate_material.push(relevant.axis as u8);
            candidate_material.extend_from_slice(&candidate.candidate_id.to_le_bytes());
            candidate_material.push(candidate.polarity);
            candidate_material.extend_from_slice(&(candidate.domain.len() as u64).to_le_bytes());
            candidate_material.extend_from_slice(candidate.domain.as_bytes());
            candidate_material.extend_from_slice(candidate.term_sha256.as_bytes());
            candidate_material.extend_from_slice(candidate.source_sha256.as_bytes());
            candidate_material.push(candidate.security_y);
            candidate_material.push(candidate.trust_x);
            candidate_material.extend_from_slice(&candidate.confidence_fixed.to_le_bytes());
            candidate_material.extend_from_slice(&candidate.frequency.to_le_bytes());
            candidate_material.extend_from_slice(&candidate.domain_count.to_le_bytes());
            candidate_material.extend_from_slice(&candidate.legacy_score_fixed.to_le_bytes());
            candidate_material.extend_from_slice(&mass.to_le_bytes());

            let reference_sha256 = shadow_contracts::sha256_hex(&[
                b"ESSMAI/ASHT/CANDIDATE-REFERENCE/V1B1",
                &[relevant.axis as u8],
                &candidate.candidate_id.to_le_bytes(),
                &[candidate.polarity],
                candidate.domain.as_bytes(),
                candidate.term_sha256.as_bytes(),
                candidate.source_sha256.as_bytes(),
                &[candidate.security_y],
                &[candidate.trust_x],
                &candidate.confidence_fixed.to_le_bytes(),
                &candidate.frequency.to_le_bytes(),
                &candidate.domain_count.to_le_bytes(),
                &candidate.legacy_score_fixed.to_le_bytes(),
                &mass.to_le_bytes(),
            ]);
            candidate_sha256.push(reference_sha256.clone());
            bounded_fragments.push(format!(
                "ASHT request-bound evidence axis={} domain={} polarity={} mass_fixed={} confidence_fixed={} candidate_sha256={}",
                relevant.axis.label(),
                candidate.domain,
                candidate.polarity,
                mass,
                candidate.confidence_fixed,
                reference_sha256,
            ));
        }
        drop(projection);

        let operator_directive = AshtOperatorDirective::new(
            self.command.trace_id.clone(),
            self.command.split_sha256.clone(),
            response.hold_sha256.clone(),
            response.selection_sha256.clone(),
            candidate_sha256,
        );
        match operator_directive.shape_is_valid() {
            true => {}
            false => return Err("Asht produced an invalid operator directive".to_string()),
        }
        let attestation_sha256 = shadow_contracts::sha256_hex(&[
            b"ESSMAI/ASHT/ATTESTATION/V1B1",
            self.command.trace_id.as_bytes(),
            self.command.split_sha256.as_bytes(),
            self.command.besa_hold_sha256.as_bytes(),
            response.hold_id.as_bytes(),
            response.hold_sha256.as_bytes(),
            response.selection_sha256.as_bytes(),
            relevance_sha256.as_bytes(),
            &[parity_evidence.verdict as u8],
            &parity_evidence.positive_mass.to_le_bytes(),
            &parity_evidence.negative_mass.to_le_bytes(),
            candidate_material.as_slice(),
            operator_directive.directive_sha256.as_bytes(),
        ]);
        Ok(AshtAttestation {
            hold_id: response.hold_id,
            hold_sha256: response.hold_sha256,
            selection_sha256: response.selection_sha256,
            relevance_sha256,
            parity: parity_evidence.verdict,
            positive_mass: parity_evidence.positive_mass,
            negative_mass: parity_evidence.negative_mass,
            ranked_negative_candidates,
            attestation_sha256,
            bounded_fragments,
            operator_directive,
        })
    }
}

pub fn combine_muscle_evidence(besa: &BesaCompanion, asht: &AshtAttestation) -> MuscleEvidenceWire {
    let mut evidence = MuscleEvidenceWire {
        besa_hold_id: besa.hold_id.clone(),
        besa_hold_sha256: besa.hold_sha256.clone(),
        besa_selection_sha256: besa.selection_sha256.clone(),
        besa_attestation_sha256: besa.attestation_sha256(),
        asht_hold_id: asht.hold_id.clone(),
        asht_hold_sha256: asht.hold_sha256.clone(),
        asht_selection_sha256: asht.selection_sha256.clone(),
        asht_attestation_sha256: asht.attestation_sha256.clone(),
        split_sha256: besa.split_sha256.clone(),
        combined_sha256: String::new(),
    };
    evidence.combined_sha256 = evidence.recompute_combined_sha256();
    evidence
}
