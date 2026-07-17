// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  SHADOW_MATRIX.RS — S.MATRIX                                           ║
// ║  Gjata Legacy™  |  Arkitekt: Bledar Gjata                             ║
// ║                                                                          ║
// ║  Sistematizon arsyetimet nga Quantum. Ndërton dosjen e çështjes.      ║
// ║  Konsulton KnowledgeVault për kontekst (lexon — NUK shkruan).         ║
// ║  ZERO if/else. Scoring vektorial i pastër.                            ║
// ║                                                                          ║
// ║  NUK shikon primitive_flags (500) — i kalon verbërisht në passage.   ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::knowledge_vault::KnowledgeVault;
use crate::types::{PassPackage, ShadowNode, ShadowVerdict};
use std::sync::Arc;

pub struct SystematizedCase {
    pub reasoning_strength:  f32,
    pub scientific_support:  f32,
    pub sovereign_conflict:  f32,
    pub negative_pattern:    f32,
    pub chain_coherence:     f32,
    pub candidate_top_score: f32,
    pub final_evidence_digest: u64,
    pub pd_gcl_process_digest: u64,
    pub spine_completion_digest: u64,
}

impl SystematizedCase {
    /// Score finale e dosjes — vektor i peshuar.
    pub fn case_score(&self) -> f32 {
        (self.reasoning_strength  * 0.30
       + self.scientific_support  * 0.25
       + self.chain_coherence     * 0.25
       + (1.0 - self.sovereign_conflict) * 0.10
       + (1.0 - self.negative_pattern)   * 0.10)
        .clamp(0.0, 1.0)
    }
}

pub struct ShadowMatrix;

impl ShadowMatrix {
    /// Sistematizon PassPackage me kontekst nga KnowledgeVault.
    /// ZERO if/else — çdo dimension është projektion numerik.
    pub fn systematize(
        pkg:   &PassPackage,
        vault: &Arc<KnowledgeVault>,
        ts_ns: u64,
    ) -> (SystematizedCase, ShadowVerdict) {

        // ── Konsulto vault (lexim) ────────────────────────────────────────
        let sci_support   = vault.query_scientific_similarity(&pkg.candidate_scores);
        let sov_conflict  = vault.check_sovereign_conflict(pkg.reasoning_strength());
        let neg_pattern   = vault.query_negative_pattern(pkg.epistemic_mass);

        // ── Reasoning strength nga PassPackage ────────────────────────────
        let r_strength = pkg.reasoning_strength();

        // ── Chain coherence ────────────────────────────────────────────────
        let chain_coh = pkg.chain_coherence.clamp(0.0, 1.0);

        // ── Top candidate score ────────────────────────────────────────────
        let top_score = pkg.candidate_scores.iter()
            .cloned()
            .fold(0.0f32, f32::max)
            .clamp(0.0, 1.0);

        let case = SystematizedCase {
            reasoning_strength:  r_strength,
            scientific_support:  sci_support,
            sovereign_conflict:  sov_conflict,
            negative_pattern:    neg_pattern,
            chain_coherence:     chain_coh,
            candidate_top_score: top_score,
            final_evidence_digest: pkg.final_evidence_digest,
            pd_gcl_process_digest: pkg.pd_gcl_process_digest,
            spine_completion_digest: pkg.spine_completion_digest,
        };

        let score = case.case_score();

        // Verdict: material i mjaftueshëm + nuk bie ndesh me sovereign
        // ZERO if/else — shumëzim binar
        let material_ok  = (r_strength > 0.30) as u32;
        let no_conflict  = (sov_conflict < 0.90) as u32;
        let verdict_u8   = (material_ok & no_conflict) as u8;

        let mut verdict = ShadowVerdict::from_score(
            ShadowNode::Matrix,
            score,
            0,
            ts_ns,
        );
        // Override me vendim të llogaritur — branchless AND
        verdict.verdict = verdict_u8;

        (case, verdict)
    }
}
