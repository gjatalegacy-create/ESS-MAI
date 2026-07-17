// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  SHADOW_CONSENSUS.RS — S.CONSENSUS (deep only)                        ║
// ║  Gjata Legacy™  |  Arkitekt: Bledar Gjata                             ║
// ║                                                                          ║
// ║  Verifikon konsensusin ndër-cikël.                                     ║
// ║  — i portuar nga QuantumProposal → PassPackage/ShadowPassage.    ║
// ║    cross_cycle_agreement derivohet nga marrëveshja e candidate_scores: ║
// ║      agreement = (1 - dispersion) · mean_confidence                    ║
// ║  ZERO if/else për verdikt (from_score).                               ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::types::{ShadowNode, ShadowPassage, ShadowVerdict};

pub struct ShadowConsensus;

impl ShadowConsensus {
    pub fn verify(passage: &ShadowPassage, ts_ns: u64) -> ShadowVerdict {
        let pkg    = &passage.package;
        let scores = &pkg.candidate_scores;
        let n      = scores.len() as f32;

        // Marrëveshja ndër-cikël si komplement i shpërndarjes së kandidatëve,
        // e peshuar me besimin mesatar. Lista bosh → 0 (pa konsensus).
        let empty       = (scores.is_empty()) as u32 as f32;
        let safe_n      = n.max(1.0);
        let mean        = scores.iter().sum::<f32>() / safe_n;
        let variance    = scores.iter().map(|s| (s - mean) * (s - mean)).sum::<f32>() / safe_n;
        let dispersion  = variance.sqrt().clamp(0.0, 1.0);

        let agreement = ((1.0 - dispersion) * mean.max(0.0)).clamp(0.0, 1.0) * (1.0 - empty);

        ShadowVerdict::from_score(ShadowNode::Consensus, agreement, 0, ts_ns)
    }
}
