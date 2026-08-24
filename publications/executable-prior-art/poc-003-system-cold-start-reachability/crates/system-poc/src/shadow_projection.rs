//! Surgical Shadow projection for the generation-zero branch only.
//!
//! `KnowledgeVault::select_bounded_candidates` is copied byte-for-byte from
//! the v1.8.9 production function. The reduced store shell around it is new POC
//! glue. No writer, transaction, WAL, verdict, or private Shadow core is present.

use crate::knowledge_lineage::PrimitiveTrace;
use std::sync::RwLock;

#[derive(Debug)]
pub(crate) enum ShadowError {
    SealInvalid(String),
}

impl std::fmt::Display for ShadowError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SealInvalid(reason) => formatter.write_str(reason),
        }
    }
}

#[derive(Debug, Clone)]
struct PrimitiveEntry {
    input_id: u64,
    raw_bytes: Vec<u8>,
    vector_points: f32,
    real_hits: u32,
    temporal_score: f32,
    primitive_chain: Vec<PrimitiveTrace>,
    legacy_ready: bool,
}

impl PrimitiveEntry {
    fn legacy_score(&self) -> f32 {
        (self.vector_points * 0.40
            + self.real_hits as f32 * 0.35 / 100.0
            + self.temporal_score * 0.25)
            .clamp(0.0, 1.0)
    }
}

#[derive(Debug, Clone)]
struct LegacyEntry {
    origin_primitive_id: u64,
    raw_bytes: Vec<u8>,
    legacy_score: f32,
}

#[derive(Debug, Clone)]
struct NegativeEntry {
    input_id: u64,
    failure_reason_code: u32,
    process: String,
    mass_at_fail: f32,
    frequency: u32,
    raw_bytes: Vec<u8>,
}

/// Harness-only empty shell with the same three read stores used by the exact
/// production projection. It deliberately exposes no insertion method.
#[derive(Debug, Default)]
struct KnowledgeVault {
    primitive: RwLock<Vec<PrimitiveEntry>>,
    legacy: RwLock<Vec<LegacyEntry>>,
    negative: RwLock<Vec<NegativeEntry>>,
}

impl KnowledgeVault {
    /// Read-only, bounded projection for the mediated Besa/Asht selection
    /// port. No vault entry, access counter, trust score, or verdict is
    /// mutated. Only digests and bounded statistics cross the process border.
    pub(crate) fn select_bounded_candidates(
        &self,
        max_positive: u16,
        max_negative: u16,
    ) -> Result<
        (
            Vec<shadow_contracts::CandidateStatisticWire>,
            Vec<shadow_contracts::CandidateStatisticWire>,
        ),
        ShadowError,
    > {
        let primitives = self.primitive.read().unwrap_or_else(|p| p.into_inner());
        let legacies = self.legacy.read().unwrap_or_else(|p| p.into_inner());
        let negatives = self.negative.read().unwrap_or_else(|p| p.into_inner());

        let mut positive = Vec::with_capacity(usize::from(max_positive));
        for entry in primitives.iter().take(usize::from(max_positive)) {
            let confidence_fixed = match shadow_contracts::fixed_mass(entry.vector_points) {
                Some(value) => value,
                None => {
                    return Err(ShadowError::SealInvalid(
                        "selection primitive vector_points is non-finite".to_string(),
                    ))
                }
            };
            let legacy_score_fixed = match shadow_contracts::fixed_mass(entry.legacy_score()) {
                Some(value) => value,
                None => {
                    return Err(ShadowError::SealInvalid(
                        "selection primitive legacy_score is non-finite".to_string(),
                    ))
                }
            };
            positive.push(shadow_contracts::CandidateStatisticWire {
                candidate_id: entry.input_id,
                polarity: 1,
                domain: "primitive".to_string(),
                term_sha256: shadow_contracts::sha256_hex(&[
                    b"ESSMAI/SELECTION/TERM/PRIMITIVE",
                    entry.raw_bytes.as_slice(),
                ]),
                source_sha256: shadow_contracts::sha256_hex(&[
                    b"ESSMAI/SELECTION/SOURCE/PRIMITIVE",
                    &entry.input_id.to_le_bytes(),
                    entry.raw_bytes.as_slice(),
                ]),
                security_y: 1,
                trust_x: u8::from(entry.legacy_ready),
                confidence_fixed,
                frequency: entry.real_hits,
                domain_count: crate::knowledge_lineage::unique_domains(&entry.primitive_chain)
                    .min(usize::from(u16::MAX)) as u16,
                legacy_score_fixed,
            });
        }
        let remaining = usize::from(max_positive).saturating_sub(positive.len());
        for entry in legacies.iter().take(remaining) {
            let legacy_score_fixed = match shadow_contracts::fixed_mass(entry.legacy_score) {
                Some(value) => value,
                None => {
                    return Err(ShadowError::SealInvalid(
                        "selection legacy score is non-finite".to_string(),
                    ))
                }
            };
            positive.push(shadow_contracts::CandidateStatisticWire {
                candidate_id: entry.origin_primitive_id,
                polarity: 1,
                domain: "legacy".to_string(),
                term_sha256: shadow_contracts::sha256_hex(&[
                    b"ESSMAI/SELECTION/TERM/LEGACY",
                    entry.raw_bytes.as_slice(),
                ]),
                source_sha256: shadow_contracts::sha256_hex(&[
                    b"ESSMAI/SELECTION/SOURCE/LEGACY",
                    &entry.origin_primitive_id.to_le_bytes(),
                    entry.raw_bytes.as_slice(),
                ]),
                security_y: 1,
                trust_x: 1,
                confidence_fixed: legacy_score_fixed,
                frequency: 1,
                domain_count: 1,
                legacy_score_fixed,
            });
        }

        let mut negative = Vec::with_capacity(usize::from(max_negative));
        for entry in negatives.iter().take(usize::from(max_negative)) {
            let mass_fixed = match shadow_contracts::fixed_mass(entry.mass_at_fail) {
                Some(value) => value,
                None => {
                    return Err(ShadowError::SealInvalid(
                        "selection negative mass is non-finite".to_string(),
                    ))
                }
            };
            negative.push(shadow_contracts::CandidateStatisticWire {
                candidate_id: entry.input_id,
                polarity: 0,
                domain: entry.process.clone(),
                term_sha256: shadow_contracts::sha256_hex(&[
                    b"ESSMAI/SELECTION/TERM/NEGATIVE",
                    entry.raw_bytes.as_slice(),
                ]),
                source_sha256: shadow_contracts::sha256_hex(&[
                    b"ESSMAI/SELECTION/SOURCE/NEGATIVE",
                    &entry.input_id.to_le_bytes(),
                    &entry.failure_reason_code.to_le_bytes(),
                    entry.raw_bytes.as_slice(),
                ]),
                security_y: 0,
                trust_x: 0,
                confidence_fixed: shadow_contracts::MASS_SCALE.saturating_sub(mass_fixed),
                frequency: entry.frequency,
                domain_count: 1,
                legacy_score_fixed: 0,
            });
        }
        Ok((positive, negative))
    }
}

pub(crate) fn project_generation_zero(
    max_positive: u16,
    max_negative: u16,
) -> Result<
    (
        Vec<shadow_contracts::CandidateStatisticWire>,
        Vec<shadow_contracts::CandidateStatisticWire>,
    ),
    String,
> {
    KnowledgeVault::default()
        .select_bounded_candidates(max_positive, max_negative)
        .map_err(|error| error.to_string())
}

