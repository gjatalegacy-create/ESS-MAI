// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  SHADOW_TEMPORAL.RS — S.TEMPORAL (deep only)                          ║
// ║  Gjata Legacy™  |  Arkitekt: Bledar Gjata                             ║
// ║                                                                          ║
// ║  Verifikon validitetin KOHOR. Decay sipas KnowledgeBand + resonance.  ║
// ║  — i portuar nga QuantumProposal → PassPackage/ShadowPassage.    ║
// ║         age/stabilitet kohor merren nga reproducibility + proof_chain. ║
// ║  ZERO if/else për verdikt (from_score).                               ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::types::{KnowledgeBand, ShadowNode, ShadowPassage, ShadowVerdict};

pub struct ShadowTemporal;

impl ShadowTemporal {
    /// Ndjeshmëria ndaj decay-t sipas bandës (Scientific = e përjetshme).
    #[inline]
    fn band_sensitivity(band: KnowledgeBand) -> f32 {
        match band {
            KnowledgeBand::Scientific => 0.00,
            KnowledgeBand::Fact       => 0.05,
            KnowledgeBand::Hypothesis => 0.20,
            KnowledgeBand::Negative   => 0.40,
        }
    }

    /// Verifikon shëndetin kohor të paketës.
    ///
    /// temporal_health = (1 - decay)·0.45 + resonance·0.35 + chain_maturity·0.20
    ///   decay          = (1 - time_stability) · band_sensitivity
    ///   time_stability ← reproducibility (proxy i qëndrueshmërisë kohore)
    ///   resonance      ← convergence_strength
    ///   chain_maturity ← gjatësia e proof_chain (nga origjina Light) / 5
    pub fn verify(passage: &ShadowPassage, ts_ns: u64) -> ShadowVerdict {
        let pkg  = &passage.package;
        let band = pkg.knowledge_band();
        let sens = Self::band_sensitivity(band);

        let time_stability = pkg.reproducibility.clamp(0.0, 1.0);
        let resonance      = pkg.convergence_strength.clamp(0.0, 1.0);
        let chain_len      = passage.light.proof_chain.len() as f32;
        let chain_maturity = (chain_len / 5.0).clamp(0.0, 1.0);

        let decay = ((1.0 - time_stability) * sens).clamp(0.0, 1.0);

        let temporal_health = ((1.0 - decay) * 0.45
                             + resonance       * 0.35
                             + chain_maturity  * 0.20)
                             .clamp(0.0, 1.0);

        ShadowVerdict::from_score(ShadowNode::Temporal, temporal_health, 0, ts_ns)
    }
}
