// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  CONTROL/SCORE_SYSTEM.RS — 6D Fusion Score (Teoria 35)                ║
// ║  GJATA LEGACY™ — Quantum Platform                                     ║
// ║                                                                          ║
// ║  6 dimensione → fusion_score:                                         ║
// ║   logical, scientific, stability, semantic, coherence, entropy       ║
// ║  3 weight-sets: DEFAULT / SCIENCE / LAB                               ║
// ║                                                                          ║
// ║  Pseudo përdor IF EMPTY etj; KONVERTUAR në match/formula (zero if).   ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::sovereign::laws::gate;

// ─────────────────────────────────────────────────────────────────────────────
// SCORE VECTOR — 6 dimensionet
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
pub struct ScoreVector {
    pub logical_score:    f32,
    pub scientific_score: f32,
    pub stability_score:  f32,
    pub semantic_score:   f32,
    pub coherence_score:  f32,
    pub entropy_score:    f32,
    pub fusion_score:     f32,  // e llogaritur
}

impl ScoreVector {
    pub fn zeroed() -> Self {
        Self {
            logical_score: 0.0, scientific_score: 0.0, stability_score: 0.0,
            semantic_score: 0.0, coherence_score: 0.0, entropy_score: 0.0,
            fusion_score: 0.0,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// WEIGHTS — 3 sete (DEFAULT / SCIENCE / LAB)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
pub struct Weights {
    pub logical:    f32,
    pub scientific: f32,
    pub stability:  f32,
    pub semantic:   f32,
    pub coherence:  f32,
    pub entropy:    f32,
}

impl Weights {
    pub const DEFAULT: Weights = Weights {
        logical: 0.25, scientific: 0.20, stability: 0.15,
        semantic: 0.20, coherence: 0.10, entropy: 0.10,
    };
    pub const SCIENCE: Weights = Weights {
        logical: 0.15, scientific: 0.35, stability: 0.20,
        semantic: 0.15, coherence: 0.10, entropy: 0.05,
    };
    pub const LAB: Weights = Weights {
        logical: 0.20, scientific: 0.30, stability: 0.25,
        semantic: 0.10, coherence: 0.10, entropy: 0.05,
    };

    /// Zgjedh weight-set nga domain (zero if — match).
    pub fn for_domain(domain: &str) -> Weights {
        match domain {
            "science" => Weights::SCIENCE,
            "lab"     => Weights::LAB,
            _         => Weights::DEFAULT,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// CANDIDATE INPUT — të dhënat e një kandidati për scoring
// ─────────────────────────────────────────────────────────────────────────────

/// Hyrja për scorer (e abstraguar nga PROCandidate).
#[derive(Debug, Clone)]
pub struct ScorableCandidate {
    pub quantum_score:    f32,
    pub has_proposal_type: bool,
    pub contradiction_count: u32,  // kundrejt query
}

// ─────────────────────────────────────────────────────────────────────────────
// SCORER — llogarit secilin dimension (zero if/else)
// ─────────────────────────────────────────────────────────────────────────────

pub struct Scorer;

impl Scorer {
    /// logical = clamp(quantum_score - contradictions*0.15).
    pub fn compute_logical(c: &ScorableCandidate) -> f32 {
        let penalties = c.contradiction_count as f32 * 0.15;
        (c.quantum_score - penalties).clamp(0.0, 1.0)
    }

    /// scientific = matches/total, ose 0.30 nëse bosh (zero if — formula).
    /// empty → 0.30; ndryshe → ratio.
    pub fn compute_scientific(supporting_nodes: u32, total_nodes: u32) -> f32 {
        // is_empty ∈ {0,1}; rezultat = empty*0.30 + (1-empty)*ratio.
        let is_empty = (total_nodes == 0) as u32 as f32;
        let ratio = (supporting_nodes as f32) / (total_nodes.max(1) as f32);
        (is_empty * 0.30 + (1.0 - is_empty) * ratio).clamp(0.0, 1.0)
    }

    /// stability = similar/total, ose 0.50 nëse pa histori.
    pub fn compute_stability(similar_count: u32, history_len: u32) -> f32 {
        let is_empty = (history_len == 0) as u32 as f32;
        let ratio = (similar_count as f32) / (history_len.max(1) as f32);
        (is_empty * 0.50 + (1.0 - is_empty) * ratio).clamp(0.0, 1.0)
    }

    /// semantic = 1 - distance, ose quantum*0.8 nëse pa graph.
    /// has_graph ∈ {0,1}.
    pub fn compute_semantic(distance: f32, quantum_score: f32, has_graph: bool) -> f32 {
        let g = has_graph as u32 as f32;
        let with_graph = 1.0 - distance;
        let fallback = quantum_score * 0.8;
        (g * with_graph + (1.0 - g) * fallback).clamp(0.0, 1.0)
    }

    /// coherence = agreements/(n-1), ose 1.0 nëse <= 1 kandidat.
    pub fn compute_coherence(agreements: u32, all_count: u32) -> f32 {
        let single = (all_count <= 1) as u32 as f32;
        let denom = (all_count.max(2) - 1) as f32;
        let ratio = (agreements as f32) / denom;
        (single * 1.0 + (1.0 - single) * ratio).clamp(0.0, 1.0)
    }

    /// entropy = (clarity + score_ok)/2.
    /// clarity = proposal_type ? 1.0 : 0.5; score_ok = score>0.5 ? 1.0 : 0.3.
    pub fn compute_entropy(c: &ScorableCandidate) -> f32 {
        // Branchless: has_type ∈ {0,1}, score_high ∈ {0,1}.
        let has_type = c.has_proposal_type as u32 as f32;
        let clarity = 0.5 + 0.5 * has_type;  // 0.5 ose 1.0
        let score_high = (c.quantum_score > 0.5) as u32 as f32;
        let score_ok = 0.3 + 0.7 * score_high;  // 0.3 ose 1.0
        ((clarity + score_ok) / 2.0).clamp(0.0, 1.0)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// FUSION — bashkon 6 dimensione → 1 score
// ─────────────────────────────────────────────────────────────────────────────

pub struct ScoreFusion;

impl ScoreFusion {
    /// fuse — produkt skalar i 6 dimensioneve me peshat e domain-it.
    pub fn fuse(vector: &mut ScoreVector, domain: &str) -> f32 {
        let w = Weights::for_domain(domain);
        let fusion = vector.logical_score    * w.logical
                   + vector.scientific_score * w.scientific
                   + vector.stability_score  * w.stability
                   + vector.semantic_score   * w.semantic
                   + vector.coherence_score  * w.coherence
                   + vector.entropy_score    * w.entropy;
        vector.fusion_score = gate(fusion, 0.0, 1.0);
        vector.fusion_score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logical_penalizes_contradictions() {
        let c = ScorableCandidate { quantum_score: 0.9, has_proposal_type: true, contradiction_count: 2 };
        // 0.9 - 2*0.15 = 0.6
        assert!((Scorer::compute_logical(&c) - 0.6).abs() < 1e-5);
    }

    #[test]
    fn scientific_empty_is_neutral() {
        assert!((Scorer::compute_scientific(0, 0) - 0.30).abs() < 1e-5);
        // 3 supporting / 5 total = 0.6
        assert!((Scorer::compute_scientific(3, 5) - 0.6).abs() < 1e-5);
    }

    #[test]
    fn stability_no_history_neutral() {
        assert!((Scorer::compute_stability(0, 0) - 0.50).abs() < 1e-5);
        assert!((Scorer::compute_stability(7, 10) - 0.7).abs() < 1e-5);
    }

    #[test]
    fn semantic_fallback_without_graph() {
        // pa graph → quantum*0.8
        assert!((Scorer::compute_semantic(0.0, 0.5, false) - 0.4).abs() < 1e-5);
        // me graph → 1 - distance
        assert!((Scorer::compute_semantic(0.3, 0.5, true) - 0.7).abs() < 1e-5);
    }

    #[test]
    fn coherence_single_candidate_perfect() {
        assert!((Scorer::compute_coherence(0, 1) - 1.0).abs() < 1e-5);
        // 3 agreements / (5-1) = 0.75
        assert!((Scorer::compute_coherence(3, 5) - 0.75).abs() < 1e-5);
    }

    #[test]
    fn entropy_clarity_and_score() {
        // proposal_type=true, score=0.8 → (1.0 + 1.0)/2 = 1.0
        let good = ScorableCandidate { quantum_score: 0.8, has_proposal_type: true, contradiction_count: 0 };
        assert!((Scorer::compute_entropy(&good) - 1.0).abs() < 1e-5);
        // proposal_type=false, score=0.3 → (0.5 + 0.3)/2 = 0.4
        let bad = ScorableCandidate { quantum_score: 0.3, has_proposal_type: false, contradiction_count: 0 };
        assert!((Scorer::compute_entropy(&bad) - 0.4).abs() < 1e-5);
    }

    #[test]
    fn fusion_uses_domain_weights() {
        let mut v = ScoreVector {
            logical_score: 1.0, scientific_score: 1.0, stability_score: 1.0,
            semantic_score: 1.0, coherence_score: 1.0, entropy_score: 1.0,
            fusion_score: 0.0,
        };
        // Të gjitha 1.0 → fusion = shuma e peshave = 1.0
        let f = ScoreFusion::fuse(&mut v, "science");
        assert!((f - 1.0).abs() < 1e-5);
    }

    #[test]
    fn science_weights_favor_scientific() {
        let w = Weights::for_domain("science");
        assert_eq!(w.scientific, 0.35);
        let d = Weights::for_domain("anything");
        assert_eq!(d.scientific, 0.20);
    }
}
