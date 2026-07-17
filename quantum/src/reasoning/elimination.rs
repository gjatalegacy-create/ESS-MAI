// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  REASONING/ELIMINATION.RS — Pipeline 3-Nivel (Teoria 27)             ║
// ║  GJATA LEGACY™ — Quantum Platform                                     ║
// ║                                                                          ║
// ║  LOCAL (0.30) → REGIONAL (0.50) → GLOBAL (0.70).                      ║
// ║  Vetëm strongest survival. N sipas ShadowState.                       ║
// ║                                                                          ║
// ║  Pseudo përdor IF/ELSE; KONVERTUAR plotësisht në match/filter/formula.║
// ╚══════════════════════════════════════════════════════════════════════════╝

use std::collections::HashSet;
use crate::control::pressure_states::ShadowState;

// ─────────────────────────────────────────────────────────────────────────────
// THRESHOLDS — identike me pseudo
// ─────────────────────────────────────────────────────────────────────────────

pub const THRESHOLD_LOCAL:    f32 = 0.30;
pub const THRESHOLD_REGIONAL: f32 = 0.50;
pub const THRESHOLD_GLOBAL:   f32 = 0.70;

// ─────────────────────────────────────────────────────────────────────────────
// ELIM CANDIDATE — kandidati për eliminim
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ElimCandidate {
    pub provider:        String,
    pub provider_domain: String,
    pub proposal_type:   String,
    pub quantum_score:   f32,
    pub has_proposal:    bool,
    pub has_signature:   bool,
}

// ─────────────────────────────────────────────────────────────────────────────
// ELIMINATION PIPELINE
// ─────────────────────────────────────────────────────────────────────────────

pub struct Elimination;

impl Elimination {
    // ── NIVEL 1: LOCAL ─────────────────────────────────────────────────────

    /// local_eliminate: heq kandidatë inkoherentë.
    /// valid = score > 0.30 AND has_proposal AND has_signature.
    /// Zero if — filter me predikat boolean.
    pub fn local_eliminate(candidates: Vec<ElimCandidate>) -> Vec<ElimCandidate> {
        candidates.into_iter()
            .filter(|c| {
                c.quantum_score > THRESHOLD_LOCAL
                    && c.has_proposal
                    && c.has_signature
                    && !c.provider.is_empty()
            })
            .collect()
    }

    // ── NIVEL 2: REGIONAL ──────────────────────────────────────────────────

    /// regional_eliminate: grupon per domain, heq kontradiktat, mban strongest.
    /// Zero if — përdor remove_contradictions + filter.
    pub fn regional_eliminate(candidates: Vec<ElimCandidate>) -> Vec<ElimCandidate> {
        // Hiq kontradiktat (mban strongest per domain:proposal_type).
        let consistent = Self::remove_contradictions(candidates);

        // Mban vetëm ata >= REGIONAL threshold.
        consistent.into_iter()
            .filter(|c| c.quantum_score >= THRESHOLD_REGIONAL)
            .collect()
    }

    /// remove_contradictions: rendit zbritës, mban të parin per (domain:type).
    /// Zero if — HashSet insert kthen bool, filter mbi të.
    pub fn remove_contradictions(mut items: Vec<ElimCandidate>) -> Vec<ElimCandidate> {
        // Rendit zbritës sipas score (strongest i pari).
        items.sort_by(|a, b| {
            b.quantum_score.partial_cmp(&a.quantum_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let mut seen: HashSet<String> = HashSet::new();
        let mut clean = Vec::new();
        for item in items {
            let key = format!("{}:{}", item.provider_domain, item.proposal_type);
            // insert kthen true nëse i ri — push vetëm atëherë (zero if: match).
            match seen.insert(key) {
                true  => clean.push(item),
                false => {}
            }
        }
        clean
    }

    // ── NIVEL 3: GLOBAL ────────────────────────────────────────────────────

    /// global_eliminate: rendit me score, merr top-N sipas ShadowState.
    /// Zero if — sort + N nga match + truncate.
    pub fn global_eliminate(
        mut candidates: Vec<ElimCandidate>,
        shadow_state: ShadowState,
    ) -> Vec<ElimCandidate> {
        // Rendit zbritës.
        candidates.sort_by(|a, b| {
            b.quantum_score.partial_cmp(&a.quantum_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // N sipas ShadowState (match — zero if).
        let n = Self::n_for_state(shadow_state);
        candidates.truncate(n);
        candidates
    }

    /// N sipas ShadowState — identike me pseudo.
    fn n_for_state(state: ShadowState) -> usize {
        match state {
            ShadowState::FastPath  => 1,
            ShadowState::Light     => 3,
            ShadowState::Pressured => 2,
            ShadowState::Absorbing => 1,
        }
    }

    // ── RUN ALL — pipeline i plotë ─────────────────────────────────────────

    /// run_all: local → regional → global.
    pub fn run_all(
        candidates: Vec<ElimCandidate>,
        shadow_state: ShadowState,
    ) -> Vec<ElimCandidate> {
        let after_local = Self::local_eliminate(candidates);
        let after_regional = Self::regional_eliminate(after_local);
        Self::global_eliminate(after_regional, shadow_state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cand(provider: &str, domain: &str, ptype: &str, score: f32) -> ElimCandidate {
        ElimCandidate {
            provider: provider.to_string(),
            provider_domain: domain.to_string(),
            proposal_type: ptype.to_string(),
            quantum_score: score,
            has_proposal: true,
            has_signature: true,
        }
    }

    #[test]
    fn local_removes_below_threshold() {
        let cands = vec![
            cand("A", "sci", "t1", 0.5),  // > 0.30 OK
            cand("B", "sci", "t2", 0.2),  // < 0.30 FAIL
        ];
        let passed = Elimination::local_eliminate(cands);
        assert_eq!(passed.len(), 1);
        assert_eq!(passed[0].provider, "A");
    }

    #[test]
    fn local_removes_no_signature() {
        let mut c = cand("A", "sci", "t1", 0.8);
        c.has_signature = false;
        let passed = Elimination::local_eliminate(vec![c]);
        assert_eq!(passed.len(), 0);
    }

    #[test]
    fn remove_contradictions_keeps_strongest() {
        let cands = vec![
            cand("A", "sci", "t1", 0.6),
            cand("B", "sci", "t1", 0.9),  // i njëjti domain:type, score më i lartë
        ];
        let clean = Elimination::remove_contradictions(cands);
        assert_eq!(clean.len(), 1);
        assert_eq!(clean[0].provider, "B"); // strongest
    }

    #[test]
    fn regional_threshold_filters() {
        let cands = vec![
            cand("A", "sci", "t1", 0.7),  // >= 0.50 OK
            cand("B", "ind", "t2", 0.4),  // < 0.50 FAIL
        ];
        let regional = Elimination::regional_eliminate(cands);
        assert_eq!(regional.len(), 1);
        assert_eq!(regional[0].provider, "A");
    }

    #[test]
    fn global_n_by_shadow_state() {
        let cands = vec![
            cand("A", "sci", "t1", 0.9),
            cand("B", "ind", "t2", 0.8),
            cand("C", "sem", "t3", 0.7),
        ];
        // FastPath → 1
        let fast = Elimination::global_eliminate(cands.clone(), ShadowState::FastPath);
        assert_eq!(fast.len(), 1);
        assert_eq!(fast[0].provider, "A"); // më i larti
        // Light → 3
        let light = Elimination::global_eliminate(cands.clone(), ShadowState::Light);
        assert_eq!(light.len(), 3);
        // Pressured → 2
        let pressured = Elimination::global_eliminate(cands, ShadowState::Pressured);
        assert_eq!(pressured.len(), 2);
    }

    #[test]
    fn run_all_full_pipeline() {
        let cands = vec![
            cand("A", "sci", "t1", 0.9),  // kalon të gjitha
            cand("B", "sci", "t1", 0.6),  // bie te contradictions (duplikat domain:type)
            cand("C", "ind", "t2", 0.2),  // bie te local
        ];
        let result = Elimination::run_all(cands, ShadowState::Light);
        // A mbijeton (B hiqet si kontradiktë, C te local).
        assert!(result.iter().any(|c| c.provider == "A"));
        assert!(!result.iter().any(|c| c.provider == "C"));
    }
}
