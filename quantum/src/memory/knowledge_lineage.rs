// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  MEMORY/KNOWLEDGE_LINEAGE.RS — Knowledge Lineage Engine (Teoria 22)   ║
// ║  GJATA LEGACY™ — Quantum Platform                                     ║
// ║                                                                          ║
// ║  Lidh historinë e trace-ve me reasoning të ardhshëm.                 ║
// ║  3 burime: RAW_PENDING / VERIFIED_NEGATIVE / VERIFIED_OPTIMAL        ║
// ║   optimal_refs → BOOST ×1.15 për kandidatë që ndjekin rrugë optimale ║
// ║   failure_refs → PENALTY ×0.60 për pattern dështimi të njohura       ║
// ║                                                                          ║
// ║  KUJTESË: refs vijnë nga ato që SHADOW ka verifikuar (jashtë Quantum). ║
// ║  Quantum vetëm KONSULTON lineage — nuk vendos, nuk ruan persistent.    ║
// ║                                                                          ║
// ║  Pseudo përdor IF/FOR; KONVERTUAR në match/iterator (zero if/else).    ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::sovereign::laws::gate;
use crate::control::pressure_states::PressureState;
use crate::reasoning::elimination::ElimCandidate;

// ─────────────────────────────────────────────────────────────────────────────
// REFERENCAT — optimale & negative (nga Shadow-verifikimet)
// ─────────────────────────────────────────────────────────────────────────────

/// Rrugë e suksesshme e verifikuar nga Shadow.
#[derive(Debug, Clone)]
pub struct OptimalEntry {
    pub trace_id:          String,
    pub convergence_path:  Vec<String>,  // providers në rrugën fituese
}

/// Dështim i dokumentuar i verifikuar nga Shadow.
#[derive(Debug, Clone)]
pub struct NegativeEntry {
    pub cause:         String,
    pub proposal_type: String,
    pub consequence:   String,
}

// ─────────────────────────────────────────────────────────────────────────────
// MULTIPLIKUESIT — identike me pseudo
// ─────────────────────────────────────────────────────────────────────────────

pub const LINEAGE_BOOST:   f32 = 1.15;  // optimal path
pub const LINEAGE_PENALTY: f32 = 0.60;  // known failure

// ─────────────────────────────────────────────────────────────────────────────
// LINEAGE CONTEXT — pako që i jepet reasoning para ciklit
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct LineageContext {
    pub territory:             String,
    pub optimal_refs:          Vec<OptimalEntry>,
    pub negative_refs:         Vec<NegativeEntry>,
    pub known_failures:        Vec<String>,
    pub consequence_warnings:  Vec<String>,
    pub pressure_hint:         PressureState,
    pub lineage_depth:         usize,
}

// ─────────────────────────────────────────────────────────────────────────────
// KNOWLEDGE LINEAGE ENGINE
// ─────────────────────────────────────────────────────────────────────────────

pub struct KnowledgeLineage;

impl KnowledgeLineage {
    /// build — ndërton kontekstin nga refs të verifikuara (vijnë nga jashtë).
    /// optimal/negative refs prodhohen nga Shadow; Quantum i konsulton.
    pub fn build(
        territory: &str,
        optimal_refs: Vec<OptimalEntry>,
        negative_refs: Vec<NegativeEntry>,
    ) -> LineageContext {
        let known_failures: Vec<String> = negative_refs.iter()
            .map(|e| e.cause.clone())
            .collect();
        let consequence_warnings: Vec<String> = negative_refs.iter()
            .map(|e| e.consequence.clone())
            .filter(|c| !c.is_empty())
            .collect();

        let pressure_hint = Self::infer_pressure(&negative_refs, &optimal_refs);
        let lineage_depth = optimal_refs.len() + negative_refs.len();

        LineageContext {
            territory: territory.to_string(),
            optimal_refs,
            negative_refs,
            known_failures,
            consequence_warnings,
            pressure_hint,
            lineage_depth,
        }
    }

    /// infer_pressure — neg>5 → HIGH; opt>3 → LOW; default LOW.
    /// Zero if — match mbi tuple boolean.
    fn infer_pressure(neg: &[NegativeEntry], opt: &[OptimalEntry]) -> PressureState {
        let many_failures = neg.len() > 5;
        let many_optimal = opt.len() > 3;
        match (many_failures, many_optimal) {
            (true, _) => PressureState::High,
            (_, true) => PressureState::Low,
            _         => PressureState::Low,
        }
    }

    /// apply — boost optimal, penalty failures. Zero if — multiplikues nga formula.
    pub fn apply(
        candidates: Vec<ElimCandidate>,
        ctx: &LineageContext,
    ) -> Vec<ElimCandidate> {
        candidates.into_iter()
            .map(|mut c| {
                // matches_optimal ∈ {0,1}.
                let matches_optimal = Self::matches_optimal_path(&c, &ctx.optimal_refs) as u32 as f32;
                // is_known_failure ∈ {0,1}.
                let is_failure = Self::is_known_failure(&c.proposal_type, &ctx.negative_refs) as u32 as f32;

                // Multiplikuesi: boost nëse optimal, penalty nëse failure, ndryshe 1.0.
                // Aplikohen të dy në mënyrë multiplikative (branchless).
                let boost_factor = 1.0 + matches_optimal * (LINEAGE_BOOST - 1.0);
                let penalty_factor = 1.0 + is_failure * (LINEAGE_PENALTY - 1.0);

                c.quantum_score = gate(c.quantum_score * boost_factor * penalty_factor, 0.0, 1.0);
                c
            })
            .collect()
    }

    /// matches_optimal_path — provider në ndonjë convergence_path.
    /// Zero if — any() mbi iterator.
    fn matches_optimal_path(candidate: &ElimCandidate, optimal_refs: &[OptimalEntry]) -> bool {
        optimal_refs.iter().any(|opt| {
            opt.convergence_path.iter().any(|p| *p == candidate.provider)
        })
    }

    /// is_known_failure — proposal_type në negative refs.
    /// Zero if — any() mbi iterator.
    fn is_known_failure(proposal_type: &str, negative_refs: &[NegativeEntry]) -> bool {
        negative_refs.iter().any(|e| e.proposal_type == proposal_type)
    }

    /// link_trace_ref — kthen parent-in (trace_id i optimal[0]) ose bosh.
    /// Zero if — map + unwrap_or.
    pub fn link_trace_ref(ctx: &LineageContext) -> String {
        ctx.optimal_refs.first()
            .map(|o| o.trace_id.clone())
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cand(provider: &str, ptype: &str, score: f32) -> ElimCandidate {
        ElimCandidate {
            provider: provider.to_string(),
            provider_domain: "sci".to_string(),
            proposal_type: ptype.to_string(),
            quantum_score: score,
            has_proposal: true,
            has_signature: true,
        }
    }

    #[test]
    fn build_creates_context() {
        let opt = vec![OptimalEntry { trace_id: "CT_1".to_string(), convergence_path: vec!["PRO".to_string()] }];
        let neg = vec![NegativeEntry { cause: "bad path".to_string(), proposal_type: "t_bad".to_string(), consequence: "instability".to_string() }];
        let ctx = KnowledgeLineage::build("science", opt, neg);
        assert_eq!(ctx.lineage_depth, 2);
        assert_eq!(ctx.known_failures, vec!["bad path"]);
        assert_eq!(ctx.consequence_warnings, vec!["instability"]);
    }

    #[test]
    fn optimal_path_boosts() {
        let opt = vec![OptimalEntry { trace_id: "CT_1".to_string(), convergence_path: vec!["PRO".to_string()] }];
        let ctx = KnowledgeLineage::build("sci", opt, vec![]);
        let cands = vec![cand("PRO", "t1", 0.6)];
        let result = KnowledgeLineage::apply(cands, &ctx);
        // 0.6 * 1.15 = 0.69
        assert!((result[0].quantum_score - 0.69).abs() < 1e-4);
    }

    #[test]
    fn known_failure_penalizes() {
        let neg = vec![NegativeEntry { cause: "c".to_string(), proposal_type: "t_bad".to_string(), consequence: "".to_string() }];
        let ctx = KnowledgeLineage::build("sci", vec![], neg);
        let cands = vec![cand("X", "t_bad", 0.8)];
        let result = KnowledgeLineage::apply(cands, &ctx);
        // 0.8 * 0.60 = 0.48
        assert!((result[0].quantum_score - 0.48).abs() < 1e-4);
    }

    #[test]
    fn boost_and_penalty_combine() {
        // Provider në optimal path POR proposal_type është failure.
        let opt = vec![OptimalEntry { trace_id: "CT_1".to_string(), convergence_path: vec!["PRO".to_string()] }];
        let neg = vec![NegativeEntry { cause: "c".to_string(), proposal_type: "t_bad".to_string(), consequence: "".to_string() }];
        let ctx = KnowledgeLineage::build("sci", opt, neg);
        let cands = vec![cand("PRO", "t_bad", 0.6)];
        let result = KnowledgeLineage::apply(cands, &ctx);
        // 0.6 * 1.15 * 0.60 = 0.414
        assert!((result[0].quantum_score - 0.414).abs() < 1e-4);
    }

    #[test]
    fn no_match_unchanged() {
        let ctx = KnowledgeLineage::build("sci", vec![], vec![]);
        let cands = vec![cand("Z", "t_new", 0.7)];
        let result = KnowledgeLineage::apply(cands, &ctx);
        // pa boost/penalty → e pandryshuar
        assert!((result[0].quantum_score - 0.7).abs() < 1e-5);
    }

    #[test]
    fn pressure_hint_from_failures() {
        // > 5 failures → HIGH
        let neg: Vec<NegativeEntry> = (0..6).map(|i| NegativeEntry {
            cause: format!("c{}", i), proposal_type: format!("t{}", i), consequence: "".to_string(),
        }).collect();
        let ctx = KnowledgeLineage::build("sci", vec![], neg);
        assert_eq!(ctx.pressure_hint, PressureState::High);
    }

    #[test]
    fn link_trace_ref_uses_best_optimal() {
        let opt = vec![OptimalEntry { trace_id: "CT_99".to_string(), convergence_path: vec![] }];
        let ctx = KnowledgeLineage::build("sci", opt, vec![]);
        assert_eq!(KnowledgeLineage::link_trace_ref(&ctx), "CT_99");
        // Pa optimal → bosh.
        let empty = KnowledgeLineage::build("sci", vec![], vec![]);
        assert_eq!(KnowledgeLineage::link_trace_ref(&empty), "");
    }
}
