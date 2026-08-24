//! Phase 1B.0: exact request-bound candidate relevance for Quantum.
//!
//! Shadow remains an unranked, read-only projector. Quantum accepts candidate
//! statistics only when the response proves complete bounded enumeration and a
//! candidate term digest matches an Xi/Yi term byte-for-byte under the same
//! domain-separated hash used by the current Shadow producer.

use shadow_contracts::{
    CandidateStatisticWire, KnowledgeSelectionRequestWire, KnowledgeSelectionResponseWire,
    SELECTION_MAX_CANDIDATES,
};
use std::collections::BTreeSet;

const TERM_PRIMITIVE_DOMAIN: &[u8] = b"ESSMAI/SELECTION/TERM/PRIMITIVE";
const TERM_LEGACY_DOMAIN: &[u8] = b"ESSMAI/SELECTION/TERM/LEGACY";
const TERM_NEGATIVE_DOMAIN: &[u8] = b"ESSMAI/SELECTION/TERM/NEGATIVE";
const RELEVANCE_DOMAIN: &[u8] = b"ESSMAI/ASHT/REQUEST-BOUND-RELEVANCE/V1";
const DIRECTIVE_CANDIDATE_LIMIT: usize = 128;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelevanceAxis {
    Xi = 1,
    Yi = 2,
}

impl RelevanceAxis {
    pub fn label(self) -> &'static str {
        match self {
            Self::Xi => "Xi",
            Self::Yi => "Yi",
        }
    }
}

#[derive(Debug)]
pub struct RelevantCandidate<'a> {
    pub candidate: &'a CandidateStatisticWire,
    pub axis: RelevanceAxis,
}

#[derive(Debug)]
pub struct RequestBoundProjection<'a> {
    pub positive: Vec<RelevantCandidate<'a>>,
    pub negative: Vec<RelevantCandidate<'a>>,
    pub relevance_sha256: String,
}

impl RequestBoundProjection<'_> {
    pub fn total_len(&self) -> usize {
        self.positive.len().saturating_add(self.negative.len())
    }
}

#[derive(Debug)]
struct AxisHashes {
    primitive: BTreeSet<String>,
    legacy: BTreeSet<String>,
    negative: BTreeSet<String>,
}

impl AxisHashes {
    fn from_terms(terms: &[String]) -> Self {
        Self {
            primitive: hash_terms(terms, TERM_PRIMITIVE_DOMAIN),
            legacy: hash_terms(terms, TERM_LEGACY_DOMAIN),
            negative: hash_terms(terms, TERM_NEGATIVE_DOMAIN),
        }
    }

    fn overlaps(&self, other: &Self) -> bool {
        !self.primitive.is_disjoint(&other.primitive)
            || !self.legacy.is_disjoint(&other.legacy)
            || !self.negative.is_disjoint(&other.negative)
    }
}

fn hash_terms(terms: &[String], domain: &[u8]) -> BTreeSet<String> {
    terms
        .iter()
        .map(|term| shadow_contracts::sha256_hex(&[domain, term.as_bytes()]))
        .collect()
}

fn axis_for_hash(
    term_sha256: &str,
    xi_hashes: &BTreeSet<String>,
    yi_hashes: &BTreeSet<String>,
) -> Result<Option<RelevanceAxis>, String> {
    match (
        xi_hashes.contains(term_sha256),
        yi_hashes.contains(term_sha256),
    ) {
        (true, false) => Ok(Some(RelevanceAxis::Xi)),
        (false, true) => Ok(Some(RelevanceAxis::Yi)),
        (false, false) => Ok(None),
        (true, true) => Err("request-bound relevance rejected Xi/Yi hash overlap".to_string()),
    }
}

fn positive_axis(
    candidate: &CandidateStatisticWire,
    xi: &AxisHashes,
    yi: &AxisHashes,
) -> Result<Option<RelevanceAxis>, String> {
    match candidate.domain.as_str() {
        "primitive" => axis_for_hash(
            candidate.term_sha256.as_str(),
            &xi.primitive,
            &yi.primitive,
        ),
        "legacy" => axis_for_hash(candidate.term_sha256.as_str(), &xi.legacy, &yi.legacy),
        _ => Err("request-bound relevance rejected an unknown positive domain".to_string()),
    }
}

fn negative_axis(
    candidate: &CandidateStatisticWire,
    xi: &AxisHashes,
    yi: &AxisHashes,
) -> Result<Option<RelevanceAxis>, String> {
    axis_for_hash(
        candidate.term_sha256.as_str(),
        &xi.negative,
        &yi.negative,
    )
}

fn append_candidate_material(
    material: &mut Vec<u8>,
    relevant: &RelevantCandidate<'_>,
) {
    let candidate = relevant.candidate;
    material.push(relevant.axis as u8);
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

fn selection_digest(response: &KnowledgeSelectionResponseWire) -> String {
    let mut material = Vec::new();
    for candidate in response.positive.iter().chain(response.negative.iter()) {
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
    shadow_contracts::sha256_hex(&[
        b"ESSMAI/SELECTION/CANDIDATES/V178",
        material.as_slice(),
    ])
}

fn response_is_bound(
    request: &KnowledgeSelectionRequestWire,
    response: &KnowledgeSelectionResponseWire,
) -> bool {
    let request_bytes = shadow_contracts::encode_selection_request(request);
    let request_sha256 = shadow_contracts::sha256_hex(&[
        b"ESSMAI/SELECTION/REQUEST/V178",
        request_bytes.as_slice(),
    ]);
    request_sha256 == response.request_sha256
        && selection_digest(response) == response.selection_sha256
}

/// Produces the complete, exact-byte request-relevant subset of the Shadow
/// projection. The function is deliberately conservative:
///
/// - both request limits must be the protocol maximum;
/// - a response length equal to its limit is treated as potentially truncated;
/// - Xi/Yi terms are never normalized or semantically guessed;
/// - both polarities must contain request-bound material;
/// - the relevant set must fit the existing Asht directive capacity.
///
/// Any unprovable condition is terminal and returns `Err`.
pub fn project_request_bound<'a>(
    request: &KnowledgeSelectionRequestWire,
    response: &'a KnowledgeSelectionResponseWire,
) -> Result<RequestBoundProjection<'a>, String> {
    match (
        request.shape_is_valid(),
        response.shape_is_valid_for(request),
        response_is_bound(request, response),
    ) {
        (true, true, true) => {}
        (false, _, _) => return Err("request-bound relevance rejected request shape".to_string()),
        (_, false, _) => return Err("request-bound relevance rejected response shape".to_string()),
        (_, _, false) => {
            return Err("request-bound relevance rejected response digest binding".to_string())
        }
    }

    match (
        request.max_positive == SELECTION_MAX_CANDIDATES,
        request.max_negative == SELECTION_MAX_CANDIDATES,
    ) {
        (true, true) => {}
        _ => {
            return Err(
                "request-bound relevance requires the complete protocol candidate limits"
                    .to_string(),
            )
        }
    }

    match (
        response.positive.len() < usize::from(request.max_positive),
        response.negative.len() < usize::from(request.max_negative),
    ) {
        (true, true) => {}
        (false, _) => {
            return Err(
                "request-bound relevance cannot prove complete positive enumeration".to_string(),
            )
        }
        (_, false) => {
            return Err(
                "request-bound relevance cannot prove complete negative enumeration".to_string(),
            )
        }
    }

    let xi = AxisHashes::from_terms(&request.xi);
    let yi = AxisHashes::from_terms(&request.yi);
    match xi.overlaps(&yi) {
        true => return Err("request-bound relevance rejected non-disjoint Xi/Yi terms".to_string()),
        false => {}
    }

    let mut positive = Vec::new();
    for candidate in &response.positive {
        match positive_axis(candidate, &xi, &yi)? {
            Some(axis) => positive.push(RelevantCandidate { candidate, axis }),
            None => {}
        }
    }

    let mut negative = Vec::new();
    for candidate in &response.negative {
        match negative_axis(candidate, &xi, &yi)? {
            Some(axis) => negative.push(RelevantCandidate { candidate, axis }),
            None => {}
        }
    }

    match (positive.is_empty(), negative.is_empty()) {
        (false, false) => {}
        (true, _) => {
            return Err("request-bound relevance found no exact positive candidate".to_string())
        }
        (_, true) => {
            return Err("request-bound relevance found no exact negative candidate".to_string())
        }
    }

    let total = positive.len().saturating_add(negative.len());
    match total <= DIRECTIVE_CANDIDATE_LIMIT {
        true => {}
        false => {
            return Err(
                "request-bound relevance exceeds the existing Asht directive capacity"
                    .to_string(),
            )
        }
    }

    let mut material = Vec::new();
    for relevant in positive.iter().chain(negative.iter()) {
        append_candidate_material(&mut material, relevant);
    }
    let relevance_sha256 = shadow_contracts::sha256_hex(&[
        RELEVANCE_DOMAIN,
        request.evidence_sha256.as_bytes(),
        request.split_sha256.as_bytes(),
        response.request_sha256.as_bytes(),
        response.selection_sha256.as_bytes(),
        material.as_slice(),
    ]);

    Ok(RequestBoundProjection {
        positive,
        negative,
        relevance_sha256,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use shadow_contracts::{SelectionPurpose, SelectionRequester, PROTOCOL_VERSION};

    fn hash(fill: char) -> String {
        std::iter::repeat(fill).take(64).collect()
    }

    fn request(xi: &[&str], yi: &[&str]) -> KnowledgeSelectionRequestWire {
        KnowledgeSelectionRequestWire {
            version: PROTOCOL_VERSION,
            requester: SelectionRequester::AshtQuantum,
            purpose: SelectionPurpose::EvidenceCandidates,
            session_id: "T1".to_string(),
            parent_i0: hash('a'),
            primitive_anchor: 7,
            trace_id: "T1".to_string(),
            split_sha256: hash('b'),
            xi: xi.iter().map(|value| (*value).to_string()).collect(),
            yi: yi.iter().map(|value| (*value).to_string()).collect(),
            upstream_hold_sha256: Some(hash('c')),
            evidence_sha256: hash('d'),
            max_positive: SELECTION_MAX_CANDIDATES,
            max_negative: SELECTION_MAX_CANDIDATES,
        }
    }

    fn candidate(
        id: u64,
        polarity: u8,
        domain: &str,
        raw_term: &str,
    ) -> CandidateStatisticWire {
        let term_domain = match (polarity, domain) {
            (1, "primitive") => TERM_PRIMITIVE_DOMAIN,
            (1, "legacy") => TERM_LEGACY_DOMAIN,
            (0, _) => TERM_NEGATIVE_DOMAIN,
            _ => TERM_PRIMITIVE_DOMAIN,
        };
        CandidateStatisticWire {
            candidate_id: id,
            polarity,
            domain: domain.to_string(),
            term_sha256: shadow_contracts::sha256_hex(&[term_domain, raw_term.as_bytes()]),
            source_sha256: hash('e'),
            security_y: u8::from(polarity == 1),
            trust_x: u8::from(polarity == 1),
            confidence_fixed: 7_500,
            frequency: 2,
            domain_count: 1,
            legacy_score_fixed: 5_000,
        }
    }

    fn response(
        request: &KnowledgeSelectionRequestWire,
        positive: Vec<CandidateStatisticWire>,
        negative: Vec<CandidateStatisticWire>,
    ) -> KnowledgeSelectionResponseWire {
        let request_bytes = shadow_contracts::encode_selection_request(request);
        let request_sha256 = shadow_contracts::sha256_hex(&[
            b"ESSMAI/SELECTION/REQUEST/V178",
            request_bytes.as_slice(),
        ]);
        let mut response = KnowledgeSelectionResponseWire {
            version: PROTOCOL_VERSION,
            requester: request.requester,
            session_id: request.session_id.clone(),
            parent_i0: request.parent_i0.clone(),
            primitive_anchor: request.primitive_anchor,
            split_sha256: request.split_sha256.clone(),
            upstream_hold_sha256: request.upstream_hold_sha256.clone(),
            request_sha256,
            selection_sha256: String::new(),
            hold_id: "hold-1".to_string(),
            hold_sha256: hash('2'),
            positive,
            negative,
            law_seal: 11,
            system_laws_seal: 12,
            expires_at_ns: 13,
        };
        response.selection_sha256 = selection_digest(&response);
        response
    }

    #[test]
    fn exact_xi_yi_projection_is_accepted() {
        let request = request(&["alpha"], &["beta"]);
        let response = response(
            &request,
            vec![candidate(1, 1, "primitive", "alpha")],
            vec![candidate(2, 0, "negative-process", "beta")],
        );
        let projection = project_request_bound(&request, &response);
        match projection {
            Ok(value) => {
                assert_eq!(value.positive.len(), 1);
                assert_eq!(value.negative.len(), 1);
                assert_eq!(value.positive[0].axis, RelevanceAxis::Xi);
                assert_eq!(value.negative[0].axis, RelevanceAxis::Yi);
                assert!(shadow_contracts::is_canonical_sha256(
                    &value.relevance_sha256
                ));
            }
            Err(error) => panic!("unexpected relevance error: {error}"),
        }
    }

    #[test]
    fn unrelated_candidates_are_not_admitted() {
        let request = request(&["alpha"], &["beta"]);
        let response = response(
            &request,
            vec![candidate(1, 1, "primitive", "unrelated")],
            vec![candidate(2, 0, "negative-process", "beta")],
        );
        assert!(project_request_bound(&request, &response).is_err());
    }

    #[test]
    fn exact_bytes_are_case_sensitive() {
        let request = request(&["Alpha"], &["beta"]);
        let response = response(
            &request,
            vec![candidate(1, 1, "primitive", "alpha")],
            vec![candidate(2, 0, "negative-process", "beta")],
        );
        assert!(project_request_bound(&request, &response).is_err());
    }

    #[test]
    fn non_disjoint_xi_yi_is_rejected() {
        let request = request(&["same"], &["same"]);
        let response = response(
            &request,
            vec![candidate(1, 1, "primitive", "same")],
            vec![candidate(2, 0, "negative-process", "same")],
        );
        assert!(project_request_bound(&request, &response).is_err());
    }

    #[test]
    fn unknown_positive_domain_is_rejected() {
        let request = request(&["alpha"], &["beta"]);
        let response = response(
            &request,
            vec![candidate(1, 1, "scientific", "alpha")],
            vec![candidate(2, 0, "negative-process", "beta")],
        );
        assert!(project_request_bound(&request, &response).is_err());
    }

    #[test]
    fn positive_boundary_equal_to_limit_is_rejected() {
        let request = request(&["alpha"], &["beta"]);
        let positive = (0..SELECTION_MAX_CANDIDATES)
            .map(|index| candidate(u64::from(index).saturating_add(1), 1, "primitive", "alpha"))
            .collect();
        let response = response(
            &request,
            positive,
            vec![candidate(500, 0, "negative-process", "beta")],
        );
        assert!(project_request_bound(&request, &response).is_err());
    }

    #[test]
    fn negative_boundary_equal_to_limit_is_rejected() {
        let request = request(&["alpha"], &["beta"]);
        let negative = (0..SELECTION_MAX_CANDIDATES)
            .map(|index| {
                candidate(
                    u64::from(index).saturating_add(1),
                    0,
                    "negative-process",
                    "beta",
                )
            })
            .collect();
        let response = response(
            &request,
            vec![candidate(500, 1, "primitive", "alpha")],
            negative,
        );
        assert!(project_request_bound(&request, &response).is_err());
    }

    #[test]
    fn protocol_maximum_limits_are_required() {
        let mut request = request(&["alpha"], &["beta"]);
        request.max_positive = 64;
        request.max_negative = 64;
        let response = response(
            &request,
            vec![candidate(1, 1, "primitive", "alpha")],
            vec![candidate(2, 0, "negative-process", "beta")],
        );
        assert!(project_request_bound(&request, &response).is_err());
    }

    #[test]
    fn missing_exact_negative_candidate_is_rejected() {
        let request = request(&["alpha"], &["beta"]);
        let response = response(
            &request,
            vec![candidate(1, 1, "primitive", "alpha")],
            vec![candidate(2, 0, "negative-process", "unrelated")],
        );
        assert!(project_request_bound(&request, &response).is_err());
    }

    #[test]
    fn relevant_set_over_directive_capacity_is_rejected() {
        let request = request(&["alpha"], &["beta"]);
        let positive = (0..64u16)
            .map(|index| candidate(u64::from(index).saturating_add(1), 1, "primitive", "alpha"))
            .collect();
        let negative = (0..65u16)
            .map(|index| {
                candidate(
                    u64::from(index).saturating_add(1000),
                    0,
                    "negative-process",
                    "beta",
                )
            })
            .collect();
        let response = response(&request, positive, negative);
        assert!(project_request_bound(&request, &response).is_err());
    }

    #[test]
    fn tampered_request_digest_is_rejected() {
        let request = request(&["alpha"], &["beta"]);
        let mut response = response(
            &request,
            vec![candidate(1, 1, "primitive", "alpha")],
            vec![candidate(2, 0, "negative-process", "beta")],
        );
        response.request_sha256 = hash('8');
        assert!(project_request_bound(&request, &response).is_err());
    }

    #[test]
    fn tampered_selection_digest_is_rejected() {
        let request = request(&["alpha"], &["beta"]);
        let mut response = response(
            &request,
            vec![candidate(1, 1, "primitive", "alpha")],
            vec![candidate(2, 0, "negative-process", "beta")],
        );
        response.selection_sha256 = hash('9');
        assert!(project_request_bound(&request, &response).is_err());
    }

    #[test]
    fn relevance_digest_is_deterministic() {
        let request = request(&["alpha"], &["beta"]);
        let response = response(
            &request,
            vec![candidate(1, 1, "primitive", "alpha")],
            vec![candidate(2, 0, "negative-process", "beta")],
        );
        let first = project_request_bound(&request, &response);
        let second = project_request_bound(&request, &response);
        match (first, second) {
            (Ok(left), Ok(right)) => assert_eq!(left.relevance_sha256, right.relevance_sha256),
            (Err(left), Err(right)) => panic!("unexpected relevance errors: {left}; {right}"),
            (Err(error), _) | (_, Err(error)) => panic!("unexpected relevance error: {error}"),
        }
    }
}
