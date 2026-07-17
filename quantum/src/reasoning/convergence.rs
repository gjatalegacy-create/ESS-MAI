// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  REASONING/CONVERGENCE.RS — ShadowUltraInstinct (Teoria 31)           ║
// ║  GJATA LEGACY™ — Quantum Platform                                     ║
// ║                                                                          ║
// ║  fuse() → final selection (ShadowUltraInstinct lazy, fallback [0]).    ║
// ║  deliver() → LGC seal + FinalOutput.                                   ║
// ║                                                                          ║
// ║  Pseudo përdor IF EMPTY/ACTIVE; KONVERTUAR në match (zero if/else).    ║
// ║  KUJTESË: convergence NUK vendos verdiktin — vetëm zgjedh best dhe     ║
// ║  vulos për Shadow. Shadow vendos 0/1.                                  ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::reasoning::elimination::ElimCandidate;

// ─────────────────────────────────────────────────────────────────────────────
// BEST CANDIDATE — rezultati i fusion
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct BestCandidate {
    pub provider:      String,
    pub quantum_score: f32,
    pub selected_by:   SelectionMethod,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectionMethod {
    UltraInstinct,  // ShadowUltraInstinct aktiv
    Fallback,       // kandidati i parë
    Empty,          // asnjë kandidat
}

// ─────────────────────────────────────────────────────────────────────────────
// CONVERGENCE
// ─────────────────────────────────────────────────────────────────────────────

pub struct Convergence;

impl Convergence {
    /// fuse — final selection pas 3 niveleve eliminimi.
    ///
    /// ShadowUltraInstinct (nëse aktiv) → process; ndryshe fallback te [0].
    /// candidates bosh → None.
    /// Zero if — match mbi (is_empty, ultra_active).
    pub fn fuse(
        candidates: &[ElimCandidate],
        ultra_active: bool,
    ) -> Option<BestCandidate> {
        // Match mbi gjendjen: bosh, ose ultra aktiv, ose fallback.
        match (candidates.is_empty(), ultra_active) {
            // Bosh → None (STATE_MACHINE.reset() do thirret jashtë).
            (true, _) => None,
            // Jo bosh + ultra aktiv → UltraInstinct mbi [0].
            (false, true) => {
                let c = &candidates[0];
                Some(BestCandidate {
                    provider: c.provider.clone(),
                    quantum_score: c.quantum_score,
                    selected_by: SelectionMethod::UltraInstinct,
                })
            }
            // Jo bosh + ultra jo aktiv → fallback te [0].
            (false, false) => {
                let c = &candidates[0];
                Some(BestCandidate {
                    provider: c.provider.clone(),
                    quantum_score: c.quantum_score,
                    selected_by: SelectionMethod::Fallback,
                })
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// TRANSFER — deliver me LGC seal
// ─────────────────────────────────────────────────────────────────────────────

/// Output-i final i Quantum (para dorëzimit te Shadow/Light).
#[derive(Debug, Clone)]
pub struct FinalOutput {
    pub provider:        String,
    pub quantum_score:   f32,
    pub lgc_seal:        String,
    pub confidence:      f32,
    pub selection:       SelectionMethod,
}

pub struct Transfer;

impl Transfer {
    /// deliver — vulos best me LGC seal → FinalOutput.
    /// confidence = quantum_score (e mbartur). seal nga provider+score.
    pub fn deliver(best: &BestCandidate, trace_id: &str) -> FinalOutput {
        let lgc_seal = format!("LGC::{}|{}", trace_id, best.provider);
        FinalOutput {
            provider: best.provider.clone(),
            quantum_score: best.quantum_score,
            lgc_seal,
            confidence: best.quantum_score.clamp(0.0, 1.0),
            selection: best.selected_by,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cand(provider: &str, score: f32) -> ElimCandidate {
        ElimCandidate {
            provider: provider.to_string(),
            provider_domain: "sci".to_string(),
            proposal_type: "t1".to_string(),
            quantum_score: score,
            has_proposal: true,
            has_signature: true,
        }
    }

    #[test]
    fn fuse_empty_returns_none() {
        let result = Convergence::fuse(&[], false);
        assert!(result.is_none());
    }

    #[test]
    fn fuse_ultra_active_selects() {
        let cands = vec![cand("A", 0.9), cand("B", 0.8)];
        let result = Convergence::fuse(&cands, true).unwrap();
        assert_eq!(result.provider, "A");
        assert_eq!(result.selected_by, SelectionMethod::UltraInstinct);
    }

    #[test]
    fn fuse_fallback_selects_first() {
        let cands = vec![cand("A", 0.9), cand("B", 0.8)];
        let result = Convergence::fuse(&cands, false).unwrap();
        assert_eq!(result.provider, "A");
        assert_eq!(result.selected_by, SelectionMethod::Fallback);
    }

    #[test]
    fn deliver_seals_output() {
        let best = BestCandidate {
            provider: "A".to_string(), quantum_score: 0.85,
            selected_by: SelectionMethod::Fallback,
        };
        let output = Transfer::deliver(&best, "T001");
        assert!(output.lgc_seal.contains("T001"));
        assert!(output.lgc_seal.contains("LGC::"));
        assert!((output.confidence - 0.85).abs() < 1e-5);
    }

    #[test]
    fn deliver_clamps_confidence() {
        let best = BestCandidate {
            provider: "A".to_string(), quantum_score: 1.5,  // jashtë
            selected_by: SelectionMethod::UltraInstinct,
        };
        let output = Transfer::deliver(&best, "T002");
        assert_eq!(output.confidence, 1.0);
    }
}
