//! Phase 1B.1: deterministic candidate mass and negative-parity closure.
//!
//! This module is Quantum-local pure computation. It changes no wire type,
//! performs no I/O, introduces no authority, and uses one polarity-symmetric
//! mass function over fields already bounded by Protocol V10.

use shadow_contracts::{CandidateStatisticWire, MASS_SCALE};
use std::cmp::Ordering;

/// Equal weighting is the least-assumption vector authorised for Phase 1B.1.
/// Any non-equal vector remains a sovereign ratification item.
pub const W_CONFIDENCE: u32 = 5_000;
pub const W_LEGACY: u32 = 5_000;

const _: () = assert!(W_CONFIDENCE + W_LEGACY == MASS_SCALE);

/// Result of the fail-closed parity gate.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParityVerdict {
    /// Refuting evidence is absent. Absence is not confirmation.
    NegativeEvidenceAbsent = 0,
    /// Refuting mass is greater than or equal to supporting mass.
    NegativeDominant = 1,
    /// Supporting mass strictly exceeds refuting mass.
    PositivePermitted = 2,
}

impl ParityVerdict {
    pub fn label(self) -> &'static str {
        match self {
            Self::NegativeEvidenceAbsent => "negative-evidence-absent",
            Self::NegativeDominant => "negative-dominant",
            Self::PositivePermitted => "positive-permitted",
        }
    }

    pub fn permits_positive(self) -> bool {
        matches!(self, Self::PositivePermitted)
    }
}

/// Canonical fixed-point parity evidence carried only inside Quantum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParityEvidence {
    pub verdict: ParityVerdict,
    pub positive_mass: u64,
    pub negative_mass: u64,
}

impl ParityEvidence {
    /// Verifies that the verdict and masses obey R1/R2.
    pub fn shape_is_valid(self, negative_count: usize) -> bool {
        match (self.verdict, negative_count) {
            (ParityVerdict::NegativeEvidenceAbsent, 0) => self.negative_mass == 0,
            (ParityVerdict::NegativeEvidenceAbsent, _) => false,
            (ParityVerdict::NegativeDominant, 0) => false,
            (ParityVerdict::NegativeDominant, _) => self.negative_mass >= self.positive_mass,
            (ParityVerdict::PositivePermitted, 0) => false,
            (ParityVerdict::PositivePermitted, _) => self.positive_mass > self.negative_mass,
        }
    }
}

/// GCL canonicity bracket:
/// `i0 ~(Y=1 -> X=1) // i0 ~(Y=0 -> X=0)`.
///
/// Only `(0,0)` and `(1,1)` are canonical. Invalid or non-canonical pairs
/// contribute zero mass.
#[inline]
fn canonical_bracket(candidate: &CandidateStatisticWire) -> u32 {
    u32::from(matches!(
        (candidate.trust_x, candidate.security_y),
        (0, 0) | (1, 1)
    ))
}

/// Deterministic, polarity-symmetric mass in `[0, MASS_SCALE]`.
///
/// Only `confidence_fixed` and `legacy_score_fixed` are used because they are
/// the only continuous statistics bounded by the existing contract. The
/// unbounded `frequency` and `domain_count` fields are deliberately excluded.
pub fn candidate_mass(candidate: &CandidateStatisticWire) -> u32 {
    let weighted = candidate
        .confidence_fixed
        .saturating_mul(W_CONFIDENCE)
        .saturating_add(
            candidate
                .legacy_score_fixed
                .saturating_mul(W_LEGACY),
        );
    let scaled = match MASS_SCALE {
        0 => 0,
        scale => weighted / scale,
    };
    scaled
        .min(MASS_SCALE)
        .saturating_mul(canonical_bracket(candidate))
}

/// Saturating aggregate polarity mass.
pub fn polarity_mass(candidates: &[CandidateStatisticWire]) -> u64 {
    candidates
        .iter()
        .map(candidate_mass)
        .fold(0u64, |acc, mass| acc.saturating_add(u64::from(mass)))
}

/// R1/R2 evaluation. Equality never affirms; absence never affirms.
pub fn evaluate_parity(
    positive: &[CandidateStatisticWire],
    negative: &[CandidateStatisticWire],
) -> ParityEvidence {
    let positive_mass = polarity_mass(positive);
    let negative_mass = polarity_mass(negative);
    let verdict = match negative.is_empty() {
        true => ParityVerdict::NegativeEvidenceAbsent,
        false => match negative_mass >= positive_mass {
            true => ParityVerdict::NegativeDominant,
            false => ParityVerdict::PositivePermitted,
        },
    };
    ParityEvidence {
        verdict,
        positive_mass,
        negative_mass,
    }
}

/// Corrected total order required by the Phase 0B audit:
///
/// mass descending -> polarity ascending (negative first on a tie)
/// -> candidate_id -> term_sha256 -> source_sha256 -> domain bytes
/// -> all remaining transported fields.
///
/// The final fields close collisions that the Protocol V10 shape rules do not
/// prohibit. Byte-identical records may compare equal because their ordering is
/// observationally indistinguishable.
pub fn candidate_order(
    left: &CandidateStatisticWire,
    right: &CandidateStatisticWire,
) -> Ordering {
    candidate_mass(right)
        .cmp(&candidate_mass(left))
        .then(left.polarity.cmp(&right.polarity))
        .then(left.candidate_id.cmp(&right.candidate_id))
        .then(left.term_sha256.as_bytes().cmp(right.term_sha256.as_bytes()))
        .then(
            left.source_sha256
                .as_bytes()
                .cmp(right.source_sha256.as_bytes()),
        )
        .then(left.domain.as_bytes().cmp(right.domain.as_bytes()))
        .then(left.security_y.cmp(&right.security_y))
        .then(left.trust_x.cmp(&right.trust_x))
        .then(left.confidence_fixed.cmp(&right.confidence_fixed))
        .then(left.frequency.cmp(&right.frequency))
        .then(left.domain_count.cmp(&right.domain_count))
        .then(left.legacy_score_fixed.cmp(&right.legacy_score_fixed))
}

/// Deterministic in-place ordering.
pub fn rank_candidates(candidates: &mut [CandidateStatisticWire]) {
    candidates.sort_by(candidate_order);
}

/// Verifies the exact canonical ordering without mutating the input.
pub fn is_ranked(candidates: &[CandidateStatisticWire]) -> bool {
    candidates
        .windows(2)
        .all(|pair| candidate_order(&pair[0], &pair[1]) != Ordering::Greater)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hash(fill: char) -> String {
        std::iter::repeat(fill).take(64).collect()
    }

    fn candidate(id: u64, polarity: u8, trust_x: u8, security_y: u8) -> CandidateStatisticWire {
        CandidateStatisticWire {
            candidate_id: id,
            polarity,
            domain: match polarity {
                0 => "negative".to_string(),
                _ => "primitive".to_string(),
            },
            term_sha256: hash(char::from_digit((id % 10) as u32, 10).unwrap_or('a')),
            source_sha256: hash('f'),
            security_y,
            trust_x,
            confidence_fixed: 8_000,
            frequency: 1,
            domain_count: 1,
            legacy_score_fixed: 6_000,
        }
    }

    #[test]
    fn mass_is_polarity_symmetric() {
        let positive = candidate(1, 1, 1, 1);
        let mut negative = positive.clone();
        negative.polarity = 0;
        negative.domain = "negative".to_string();
        assert_eq!(candidate_mass(&positive), candidate_mass(&negative));
    }

    #[test]
    fn non_canonical_pair_contributes_zero_mass() {
        assert_eq!(candidate_mass(&candidate(1, 1, 1, 0)), 0);
        assert_eq!(candidate_mass(&candidate(2, 0, 0, 1)), 0);
    }

    #[test]
    fn canonical_zero_pair_retains_mass() {
        assert!(candidate_mass(&candidate(1, 0, 0, 0)) > 0);
    }

    #[test]
    fn ranking_is_deterministic_on_tie() {
        let mut values = vec![candidate(9, 1, 1, 1), candidate(2, 1, 1, 1)];
        rank_candidates(&mut values);
        assert_eq!(values[0].candidate_id, 2);
        assert_eq!(values[1].candidate_id, 9);
        assert!(is_ranked(&values));
    }

    #[test]
    fn duplicate_ids_use_hash_and_domain_tiebreakers() {
        let mut alpha = candidate(7, 1, 1, 1);
        alpha.term_sha256 = hash('a');
        alpha.source_sha256 = hash('c');
        alpha.domain = "legacy".to_string();
        let mut beta = alpha.clone();
        beta.term_sha256 = hash('b');
        let mut values = vec![beta, alpha.clone()];
        rank_candidates(&mut values);
        assert_eq!(values[0].term_sha256, alpha.term_sha256);
        assert!(is_ranked(&values));
    }

    #[test]
    fn equal_masses_do_not_affirm() {
        let positive = vec![candidate(1, 1, 1, 1)];
        let negative = vec![candidate(2, 0, 0, 0)];
        let evidence = evaluate_parity(&positive, &negative);
        assert_eq!(evidence.positive_mass, evidence.negative_mass);
        assert_eq!(evidence.verdict, ParityVerdict::NegativeDominant);
        assert!(evidence.shape_is_valid(negative.len()));
    }

    #[test]
    fn absent_negative_is_undetermined_not_affirmed() {
        let positive = vec![candidate(1, 1, 1, 1)];
        let evidence = evaluate_parity(&positive, &[]);
        assert_eq!(evidence.verdict, ParityVerdict::NegativeEvidenceAbsent);
        assert!(!evidence.verdict.permits_positive());
        assert!(evidence.shape_is_valid(0));
    }

    #[test]
    fn saturating_mass_never_panics_and_fails_bounded() {
        let mut value = candidate(1, 1, 255, 255);
        value.confidence_fixed = u32::MAX;
        value.legacy_score_fixed = u32::MAX;
        value.frequency = u32::MAX;
        value.domain_count = u16::MAX;
        assert_eq!(candidate_mass(&value), 0);
    }

    #[test]
    fn mass_is_bounded_by_mass_scale() {
        for confidence in [0, 1, 5_000, MASS_SCALE] {
            for legacy in [0, 1, 5_000, MASS_SCALE] {
                let mut value = candidate(1, 1, 1, 1);
                value.confidence_fixed = confidence;
                value.legacy_score_fixed = legacy;
                assert!(candidate_mass(&value) <= MASS_SCALE);
            }
        }
    }

    #[test]
    fn unbounded_fields_do_not_affect_mass() {
        let base = candidate(1, 1, 1, 1);
        let expected = candidate_mass(&base);
        let mut extreme = base;
        extreme.frequency = u32::MAX;
        extreme.domain_count = u16::MAX;
        assert_eq!(candidate_mass(&extreme), expected);
    }

    #[test]
    fn positive_strictly_above_negative_is_permitted() {
        let mut positive = candidate(1, 1, 1, 1);
        positive.confidence_fixed = MASS_SCALE;
        positive.legacy_score_fixed = MASS_SCALE;
        let mut negative = candidate(2, 0, 0, 0);
        negative.confidence_fixed = 1_000;
        negative.legacy_score_fixed = 0;
        let evidence = evaluate_parity(&[positive], &[negative]);
        assert_eq!(evidence.verdict, ParityVerdict::PositivePermitted);
        assert!(evidence.shape_is_valid(1));
    }
}
