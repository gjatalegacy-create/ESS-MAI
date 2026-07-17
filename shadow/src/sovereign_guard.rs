// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  SOVEREIGN_GUARD.RS — LIGJET SOVRANE TË QEVERISJES (të thjeshta)      ║
// ║  Gjata Legacy™  |  Arkitekt: Bledar Gjata                             ║
// ║                                                                          ║
// ║  enforce_sovereign_laws(&passage) -> Result<(), ShadowError>          ║
// ║                                                                          ║
// ║  5 LIGJET zbatohen si INVARIANTË STRUKTURORË. Shkelja → refuzim i      ║
// ║  fortë (Err), pa shkrim në vault, pa verdikt.                         ║
// ║    L1 — Ndalim i rekursionit të tepruar                                ║
// ║    L2 — Ndalim i anashkalimit të entiteteve të mbrojtura               ║
// ║    L3 — Pastërti arsyetimi (zinxhiri i gjykimit i paprekur)            ║
// ║    L4 — Integritet i konvergjencës (pipeline-i u plotësua)             ║
// ║    L5 — Autoritet i moduleve (asnjë nod jashtë rrugës)                 ║
// ║                                                                          ║
// ║  DALLIM: shkelja sovrane ≠ Dije Negative. Dija Negative (masë e ulët / ║
// ║  judiciary=0) regjistrohet si kufi i vlefshëm; shkelja sovrane është   ║
// ║  refuzim strukturor i sistemit.                                       ║
// ║                                                                          ║
// ║  LIGJI 0 (zero-copy): vlerëson VETËM përmes &passage; zero alokime.   ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::types::{ShadowError, ShadowNode, ShadowPassage, ShadowPath};

/// Gjatësia maksimale e proof_chain (mbrojtje nga rekursioni i tepruar).
const MAX_PROOF_CHAIN: usize = 64;
/// Numri maksimal i kandidatëve (shpërthim rekursiv).
const MAX_CANDIDATES:  usize = 64;
/// Vlera maksimale e lejuar e `suggested_verdict` (0/1/2).
const MAX_SUGGESTED:   u8    = 2;

/// A i përket nodi rrugës së zgjedhur? (autoriteti i moduleve, L5)
#[inline]
fn node_allowed(path: ShadowPath, node: ShadowNode) -> bool {
    match path {
        ShadowPath::Fast => matches!(
            node,
            ShadowNode::Router | ShadowNode::Matrix | ShadowNode::MultiVerification
                | ShadowNode::Judiciary
        ),
        ShadowPath::Standard => matches!(
            node,
            ShadowNode::Router | ShadowNode::Matrix | ShadowNode::Gen5
                | ShadowNode::Type | ShadowNode::Sovereign | ShadowNode::MultiVerification
                | ShadowNode::Judiciary
        ),
        ShadowPath::Deep => true, /* Deep lejon të 9 nodat */
    }
}

/// Zbaton 5 ligjet sovrane mbi passage. Kthen Err në shkeljen e parë.
pub fn enforce_sovereign_laws(passage: &ShadowPassage) -> Result<(), ShadowError> {
    // ── L1 — Ndalim i rekursionit të tepruar ──────────────────────────────
    let chain_len = passage.light.proof_chain.len();
    let cand_len  = passage.package.candidate_scores.len();
    match chain_len > MAX_PROOF_CHAIN || cand_len > MAX_CANDIDATES {
        true => return Err(ShadowError::SovereignViolation(format!(
            "L1 recursion: proof_chain={} (max {}), candidates={} (max {})",
            chain_len, MAX_PROOF_CHAIN, cand_len, MAX_CANDIDATES
        ))),
        false => {}
    }

    // ── L2 — Ndalim i anashkalimit të entiteteve të mbrojtura ─────────────
    // Pretendim i pavlefshëm (jashtë {0,1,2}) = përpjekje për të anashkaluar
    // klasifikimin sovran.
    match passage.package.suggested_verdict > MAX_SUGGESTED {
        true => return Err(ShadowError::SovereignViolation(format!(
            "L2 protected: suggested_verdict={} jashtë {{0,1,2}}",
            passage.package.suggested_verdict
        ))),
        false => {}
    }

    // ── L3 — Pastërti arsyetimi ───────────────────────────────────────────
    // Vendimi persistent bëhet VETËM nga shadow_gj_legacy PAS gjykatës.
    // Mungesa e një verdikti Judiciary thyen pastërtinë e zinxhirit.
    match passage.has_judiciary() {
        false => return Err(ShadowError::SovereignViolation(
            "L3 purity: mungon verdikti S.Judiciary".to_string(),
        )),
        true => {}
    }

    match passage.has_multi_verification() {
        false => return Err(ShadowError::SovereignViolation(
            "L3 purity: mungon porta S.MULTI_VERIFY".to_string(),
        )),
        true => {}
    }

    // ── L4 — Integritet i konvergjencës ───────────────────────────────────
    // Pipeline-i duhet të jetë plotësuar (të paktën nodat minimale të rrugës).
    let min_nodes = passage.path.min_nodes();
    match passage.verdicts.len() < min_nodes {
        true => return Err(ShadowError::SovereignViolation(format!(
            "L4 convergence: {} noda, kërkohen ≥{} për rrugën {}",
            passage.verdicts.len(), min_nodes, passage.path.label()
        ))),
        false => {}
    }

    // ── L5 — Autoritet i moduleve ─────────────────────────────────────────
    // Asnjë nod jashtë rrugës së zgjedhur (asnjë modul nuk vepron pa autoritet).
    match passage.verdicts.iter().find(|v| !node_allowed(passage.path, v.node)) {
        Some(v) => {
        return Err(ShadowError::SovereignViolation(format!(
            "L5 authority: nodi {} jashtë rrugës {}",
            v.node.label(), passage.path.label()
        )));
    }
        None => {}
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{LightEnvelope, PassPackage, ShadowVerdict};

    fn build(path: ShadowPath, nodes: &[ShadowNode]) -> ShadowPassage {
        let pkg = PassPackage {
            suggested_verdict: 1,
            candidate_scores: vec![0.8, 0.7],
            ..Default::default()
        };
        let light = LightEnvelope { proof_chain: vec![1, 2, 3], ..Default::default() };
        let mut p = ShadowPassage::new(1, pkg, light, path);
        for (i, n) in nodes.iter().enumerate() {
            p.push_verdict(ShadowVerdict::from_score(*n, 0.9, 0, i as u64));
        }
        p
    }

    #[test]
    fn valid_fast_passes() {
        let p = build(ShadowPath::Fast,
            &[ShadowNode::Router, ShadowNode::Matrix, ShadowNode::MultiVerification, ShadowNode::Judiciary]);
        assert!(enforce_sovereign_laws(&p).is_ok());
    }

    #[test]
    fn missing_judiciary_violates_l3() {
        let p = build(ShadowPath::Fast,
            &[ShadowNode::Router, ShadowNode::Matrix]);
        let e = enforce_sovereign_laws(&p).unwrap_err();
        assert!(matches!(e, ShadowError::SovereignViolation(_)));
    }

    #[test]
    fn foreign_node_violates_l5() {
        // Gen5 nuk lejohet në Fast.
        let p = build(ShadowPath::Fast,
            &[ShadowNode::Router, ShadowNode::Matrix, ShadowNode::Gen5, ShadowNode::MultiVerification, ShadowNode::Judiciary]);
        let e = enforce_sovereign_laws(&p).unwrap_err();
        assert!(matches!(e, ShadowError::SovereignViolation(_)));
    }

    #[test]
    fn oversized_chain_violates_l1() {
        let pkg = PassPackage { suggested_verdict: 1, ..Default::default() };
        let light = LightEnvelope { proof_chain: (0..100).collect(), ..Default::default() };
        let mut p = ShadowPassage::new(1, pkg, light, ShadowPath::Fast);
        p.push_verdict(ShadowVerdict::from_score(ShadowNode::Router, 0.9, 0, 0));
        p.push_verdict(ShadowVerdict::from_score(ShadowNode::Matrix, 0.9, 0, 1));
        p.push_verdict(ShadowVerdict::from_score(ShadowNode::Judiciary, 0.9, 0, 2));
        let e = enforce_sovereign_laws(&p).unwrap_err();
        assert!(matches!(e, ShadowError::SovereignViolation(_)));
    }
}
