// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  SHADOW_EMERGENCE.RS — S.EMERGENCE (deep only)                        ║
// ║  Gjata Legacy™  |  Arkitekt: Bledar Gjata                             ║
// ║                                                                          ║
// ║  Verifikon tranzicionin (transition_ready = produkt binar).           ║
// ║  — i portuar nga QuantumProposal → PassPackage/ShadowPassage.    ║
// ║    chain_ok ← proof_chain.len() >= 3   (nga origjina Light)           ║
// ║    dag_ok   ← chain_coherence >= 0.50  (proxy i DAG-ut të plotë)      ║
// ║    cons_ok  ← conservation_ok                                          ║
// ║  ZERO if/else — produkt binar.                                        ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::types::{ShadowNode, ShadowPassage, ShadowVerdict};

pub struct ShadowEmergence;

impl ShadowEmergence {
    pub fn verify(passage: &ShadowPassage, ts_ns: u64) -> ShadowVerdict {
        let pkg = &passage.package;

        let chain_ok = (passage.light.proof_chain.len() >= 3) as u32;
        let dag_ok   = (pkg.chain_coherence >= 0.50) as u32;
        let cons_ok  = pkg.conservation_ok as u32;

        // transition_ready — produkt binar (zero if/else)
        let product = chain_ok & dag_ok & cons_ok;

        // Score i butë: gatishmëria binare e peshuar me koherencën (për Judiciary).
        let score = (product as f32) * (0.5 + 0.5 * pkg.chain_coherence.clamp(0.0, 1.0));

        ShadowVerdict::from_score(
            ShadowNode::Emergence,
            score.clamp(0.0, 1.0),
            product as u8,
            ts_ns,
        )
    }
}
