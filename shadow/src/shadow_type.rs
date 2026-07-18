// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  SHADOW_TYPE.RS — S.TYPE                                               ║
// ║  Gjata Legacy™  |  Arkitekt: Bledar Gjata                             ║
// ║  Verifikon gjendjen e paketimit të Quantum.                           ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::types::{ShadowNode, ShadowPassage, ShadowVerdict};

pub struct ShadowType;

impl ShadowType {
    /// Verifikon paketimin e gjendjes nga Quantum.
    /// Score = kombinim i IBE + causal_integrity + chain_coherence.
    /// ZERO if/else.
    pub fn verify(passage: &ShadowPassage, ts_ns: u64) -> ShadowVerdict {
        let pkg = &passage.package;

        // Packaging strength — sa mirë është paketuar arsyetimi
        let ibe_c     = pkg.ibe_score.clamp(0.0, 1.0);
        let causal_c  = pkg.causal_integrity.clamp(0.0, 1.0);
        let chain_c   = pkg.chain_coherence.clamp(0.0, 1.0);
        let pot_c     = pkg.potentiality_score.clamp(0.0, 1.0);

        // Tensor: vektor peshash × vektor skoresh
        let score = (ibe_c    * 0.30
                   + causal_c * 0.30
                   + chain_c  * 0.25
                   + pot_c    * 0.15)
                   .clamp(0.0, 1.0);

        // Konsistencë me verdiktin e Matrix
        let matrix_score = passage.verdicts.first()
            .map(|v| v.score)
            .unwrap_or(0.0);

        // Penalizim i devijimit nga Matrix — zero if/else
        let deviation  = (score - matrix_score).abs();
        let final_score = (score * (1.0 - deviation * 0.3)).clamp(0.0, 1.0);

        ShadowVerdict::from_score(ShadowNode::Type, final_score, 0, ts_ns)
    }
}
