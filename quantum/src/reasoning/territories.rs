// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  REASONING/TERRITORIES.RS — Disiplinë per Domain (Teoria 34)          ║
// ║  GJATA LEGACY™ — Quantum Platform                                     ║
// ║                                                                          ║
// ║  Çdo domain arsyeton ndryshe — jo algoritëm universal.                ║
// ║   science    → CONSERVATIVE, min=5, scientific_weight=0.40            ║
// ║   industrial → AGGRESSIVE, min=2, logical_weight=0.35                 ║
// ║   semantic   → DIVERSE, min=3, semantic_weight=0.40                   ║
// ║                                                                          ║
// ║  Pseudo përdor IF NULL; KONVERTUAR në match (zero if/else).           ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::control::score_system::Weights;

// ─────────────────────────────────────────────────────────────────────────────
// ELIMINATION MODE + CONTRADICTION POLICY
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EliminationMode {
    Conservative,  // ruan diversitet
    Aggressive,    // duhet 1 zgjidhje e qartë
    Diverse,       // ruan variantet
}

impl EliminationMode {
    pub fn label(self) -> &'static str {
        match self {
            EliminationMode::Conservative => "CONSERVATIVE",
            EliminationMode::Aggressive   => "AGGRESSIVE",
            EliminationMode::Diverse      => "DIVERSE",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ContradictionPolicy {
    Strict,            // refuzon kontradikta
    Reject,            // hedh kontradiktat
    AllowWithScore,    // lejon me score
    Warn,              // paralajmëron
}

impl ContradictionPolicy {
    pub fn label(self) -> &'static str {
        match self {
            ContradictionPolicy::Strict         => "STRICT",
            ContradictionPolicy::Reject         => "REJECT",
            ContradictionPolicy::AllowWithScore => "ALLOW_WITH_SCORE",
            ContradictionPolicy::Warn           => "WARN",
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// TERRITORY — disiplina e një domaini
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
pub struct Territory {
    pub name:                 &'static str,
    pub elimination_mode:     EliminationMode,
    pub min_candidates:       u32,
    pub weights:              Weights,
    pub contradiction_policy: ContradictionPolicy,
    pub scientific_required:  bool,
    pub lineage_depth:        u32,
}

// ─────────────────────────────────────────────────────────────────────────────
// TERRITORIES — regjistri (match mbi domain, zero if)
// ─────────────────────────────────────────────────────────────────────────────

pub struct ReasoningTerritories;

impl ReasoningTerritories {
    /// get — kthen territory-n për domain (ose default).
    /// Zero if — match i plotë.
    pub fn get(domain: &str) -> Territory {
        match domain {
            "science" => Territory {
                name: "science",
                elimination_mode: EliminationMode::Conservative,
                min_candidates: 5,
                weights: Weights {
                    logical: 0.25, scientific: 0.40, stability: 0.20,
                    semantic: 0.10, coherence: 0.03, entropy: 0.02,
                },
                contradiction_policy: ContradictionPolicy::Strict,
                scientific_required: true,
                lineage_depth: 10,
            },
            "industrial" => Territory {
                name: "industrial",
                elimination_mode: EliminationMode::Aggressive,
                min_candidates: 2,
                weights: Weights {
                    logical: 0.35, scientific: 0.15, stability: 0.30,
                    semantic: 0.10, coherence: 0.07, entropy: 0.03,
                },
                contradiction_policy: ContradictionPolicy::Reject,
                scientific_required: false,
                lineage_depth: 5,
            },
            "semantic" => Territory {
                name: "semantic",
                elimination_mode: EliminationMode::Diverse,
                min_candidates: 3,
                weights: Weights {
                    logical: 0.15, scientific: 0.10, stability: 0.10,
                    semantic: 0.40, coherence: 0.25, entropy: 0.00,
                },
                contradiction_policy: ContradictionPolicy::AllowWithScore,
                scientific_required: false,
                lineage_depth: 5,
            },
            "logistics" => Territory {
                name: "logistics",
                elimination_mode: EliminationMode::Conservative,
                min_candidates: 3,
                weights: Weights {
                    logical: 0.30, scientific: 0.20, stability: 0.30,
                    semantic: 0.07, coherence: 0.10, entropy: 0.03,
                },
                contradiction_policy: ContradictionPolicy::Warn,
                scientific_required: false,
                lineage_depth: 5,
            },
            // default — për çdo domain të panjohur.
            _ => Territory {
                name: "default",
                elimination_mode: EliminationMode::Conservative,
                min_candidates: 3,
                weights: Weights::DEFAULT,
                contradiction_policy: ContradictionPolicy::Warn,
                scientific_required: false,
                lineage_depth: 5,
            },
        }
    }

    pub fn elimination_mode(domain: &str) -> EliminationMode {
        Self::get(domain).elimination_mode
    }

    pub fn min_candidates(domain: &str) -> u32 {
        Self::get(domain).min_candidates
    }

    pub fn requires_science(domain: &str) -> bool {
        Self::get(domain).scientific_required
    }

    pub fn weights(domain: &str) -> Weights {
        Self::get(domain).weights
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn science_is_conservative_scientific() {
        let t = ReasoningTerritories::get("science");
        assert_eq!(t.elimination_mode, EliminationMode::Conservative);
        assert_eq!(t.min_candidates, 5);
        assert_eq!(t.weights.scientific, 0.40);
        assert!(t.scientific_required);
        assert_eq!(t.contradiction_policy, ContradictionPolicy::Strict);
    }

    #[test]
    fn industrial_is_aggressive() {
        let t = ReasoningTerritories::get("industrial");
        assert_eq!(t.elimination_mode, EliminationMode::Aggressive);
        assert_eq!(t.min_candidates, 2);
        assert_eq!(t.weights.logical, 0.35);
        assert_eq!(t.contradiction_policy, ContradictionPolicy::Reject);
    }

    #[test]
    fn semantic_is_diverse() {
        let t = ReasoningTerritories::get("semantic");
        assert_eq!(t.elimination_mode, EliminationMode::Diverse);
        assert_eq!(t.weights.semantic, 0.40);
        assert_eq!(t.contradiction_policy, ContradictionPolicy::AllowWithScore);
    }

    #[test]
    fn unknown_domain_gets_default() {
        let t = ReasoningTerritories::get("quantum_physics_xyz");
        assert_eq!(t.name, "default");
        assert_eq!(t.min_candidates, 3);
    }

    #[test]
    fn helper_accessors() {
        assert_eq!(ReasoningTerritories::min_candidates("science"), 5);
        assert!(ReasoningTerritories::requires_science("science"));
        assert!(!ReasoningTerritories::requires_science("industrial"));
        assert_eq!(
            ReasoningTerritories::elimination_mode("industrial"),
            EliminationMode::Aggressive
        );
    }
}
