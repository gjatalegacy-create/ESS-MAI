// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  SHADOW_SOVEREIGN.RS — S.SOVEREIGN                                     ║
// ║  Gjata Legacy™  |  Arkitekt: Bledar Gjata                             ║
// ║  5 ligjet sovrane. Verifikon evolimin e gjendjes.                     ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::types::{ShadowNode, ShadowPassage, ShadowVerdict};

/// 5 Ligjet Sovrane — bitmask verifikimi
pub struct SovereignLaws {
    pub law1_primitive:    f32,  /* input i padividuar — fortësia rrënjore  */
    pub law2_verified:     f32,  /* input i vërtetë — prova ekziston        */
    pub law3_legacy_path:  f32,  /* rruga drejt legacy — akumulo pikë       */
    pub law4_immutable:    f32,  /* legacy = i paprekshëm                   */
    pub law5_negative:     f32,  /* negative = dije e vlefshme              */
}

impl SovereignLaws {
    /// Score sovrane — dot product i 5 ligjeve me peshat
    pub fn score(&self) -> f32 {
        (self.law1_primitive   * 0.30
       + self.law2_verified    * 0.25
       + self.law3_legacy_path * 0.20
       + self.law4_immutable   * 0.15
       + self.law5_negative    * 0.10)
        .clamp(0.0, 1.0)
    }
}

pub struct ShadowSovereign;

impl ShadowSovereign {
    /// Verifikon 5 ligjet nga gjendja e akumuluar e passage.
    /// ZERO if/else — çdo ligj = projektion numerik.
    pub fn verify(passage: &ShadowPassage, ts_ns: u64) -> ShadowVerdict {
        let pkg          = &passage.package;
        let agg_score    = passage.aggregate_score();
        let potentiality = pkg.potentiality_score.clamp(0.0, 1.0);

        // LAW 1: Fortësia primitive — sa i qëndrueshëm është inputi
        let law1 = (potentiality * 0.5 + pkg.causal_integrity * 0.5).clamp(0.0, 1.0);

        // LAW 2: Verifikimi — prova e arsyetimit
        let law2 = (pkg.ibe_score * 0.6 + pkg.chain_coherence * 0.4).clamp(0.0, 1.0);

        // LAW 3: Rruga legacy — potencial akumulimi
        let law3 = (potentiality * 0.7 + agg_score * 0.3).clamp(0.0, 1.0);

        // LAW 4: Paprekueshmëria — konsistencë gjatë gjithë passage
        let law4 = passage.verdicts.iter()
            .map(|v| v.score)
            .fold(1.0f32, |acc, s| acc * s.clamp(0.0, 1.0))
            .clamp(0.0, 1.0);

        // LAW 5: Dija negative si vlerë — NegativeKnowledge = dije
        let law5 = (1.0 - pkg.epistemic_mass * 0.5).clamp(0.0, 1.0);

        let laws = SovereignLaws {
            law1_primitive: law1,
            law2_verified: law2,
            law3_legacy_path: law3,
            law4_immutable: law4,
            law5_negative: law5,
        };

        ShadowVerdict::from_score(ShadowNode::Sovereign, laws.score(), 0, ts_ns)
    }
}
