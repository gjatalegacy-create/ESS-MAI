// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  NPRO.RS — Negative PRO (Eliminimi Paralel)                          ║
// ║  GJATA LEGACY™ — Quantum Platform                                     ║
// ║                                                                          ║
// ║  NPRO punon PARALEL me PRO. Ndërsa PRO kërkon primitivin, NPRO         ║
// ║  kalkulon rrugët e dobëta (eleminimet) me 4 operatorët MBRAPSHT.       ║
// ║                                                                          ║
// ║  DY OUTPUTE:                                                           ║
// ║   1. eliminations → NPIM (paketon me argumenta për Shadow)           ║
// ║   2. WeaknessSignal → LIM (forbidden_paths për kufijtë e PRO)        ║
// ║                                                                          ║
// ║  NPRO NUK vendos, NUK dërgon vetë te Shadow — vetëm kalkulon.          ║
// ║  Zero if/else — formula matematike.                                   ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::pro_types::{FragmentVector, vectorize_fragments};
use crate::npro_operator::{NReverse, NMirror, NContrast, NTrace};

// ─────────────────────────────────────────────────────────────────────────────
// ELIMINATION — një rrugë e dobët e identifikuar nga NPRO
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct NegativeElimination {
    pub elimination_id: String,
    pub operator:       String,    // cili operator mbrapsht e gjeti
    pub weakness_score: f32,       // [0,1] sa e dobët
    pub cause:          String,    // pse u eliminua
    pub fragment_refs:  Vec<usize>,
}

// ─────────────────────────────────────────────────────────────────────────────
// WEAKNESS SIGNAL — sinjali që NPRO i dërgon LIM-it
// ─────────────────────────────────────────────────────────────────────────────

/// Sinjali që vendos kufij për PRO-n (forbidden paths).
/// LIM e merr → PRO s'humbet cikle në rrugë të dobëta.
#[derive(Debug, Clone)]
pub struct WeaknessSignal {
    pub forbidden_paths:   Vec<String>,  // operatorët/rrugët e ndaluara
    pub weakness_ceiling:  f32,          // mbi këtë score → e ndaluar
    pub avg_weakness:      f32,          // dobësia mesatare e detektuar
}

impl WeaknessSignal {
    /// A është një rrugë e ndaluar? Zero if — contains() boolean.
    pub fn is_forbidden(&self, path: &str) -> bool {
        self.forbidden_paths.iter().any(|p| p == path)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// NPRO RESULT
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct NproResult {
    pub eliminations:    Vec<NegativeElimination>,   // → NPIM
    pub weakness_signal: WeaknessSignal,     // → LIM
    pub operators_run:   usize,
}

// ─────────────────────────────────────────────────────────────────────────────
// NPRO ENGINE
// ─────────────────────────────────────────────────────────────────────────────

/// Pragu mbi të cilin një rrugë konsiderohet e dobët (forbidden).
pub const WEAKNESS_CEILING: f32 = 0.65;

pub struct NproEngine;

impl NproEngine {
    /// activate — kalkulon eliminimet me 4 operatorët mbrapsht.
    ///
    /// signal/causal vijnë nga scaffold-i (si te PRO), por interpretohen
    /// për dobësi. Prodhon eliminations (→NPIM) + WeaknessSignal (→LIM).
    pub fn activate(
        fragments: &[String],
        signal: f32,
        causal: f32,
    ) -> NproResult {
        let vectors: Vec<FragmentVector> = vectorize_fragments(fragments);

        // Aplikon 4 operatorët MBRAPSHT.
        let n_reverse = NReverse::compute(signal, causal, vec![0]);
        let n_mirror = NMirror::compute(&vectors);
        let n_contrast = NContrast::compute(&vectors);
        let n_trace = NTrace::compute(&vectors);

        let outputs = [n_reverse, n_mirror, n_contrast, n_trace];

        // Ndërto eliminimet (një per operator).
        let eliminations: Vec<NegativeElimination> = outputs.iter().enumerate()
            .map(|(i, out)| NegativeElimination {
                elimination_id: format!("NELIM_{}", i),
                operator: format!("{:?}", out.operator),
                weakness_score: out.raw_score,
                cause: out.reasoning.clone(),
                fragment_refs: out.fragment_refs.clone(),
            })
            .collect();

        // Forbidden paths: operatorët me weakness > ceiling.
        let forbidden_paths: Vec<String> = eliminations.iter()
            .filter(|e| e.weakness_score > WEAKNESS_CEILING)
            .map(|e| e.operator.clone())
            .collect();

        // Dobësia mesatare.
        let sum: f32 = eliminations.iter().map(|e| e.weakness_score).sum();
        let avg_weakness = sum / eliminations.len().max(1) as f32;

        let weakness_signal = WeaknessSignal {
            forbidden_paths,
            weakness_ceiling: WEAKNESS_CEILING,
            avg_weakness,
        };

        NproResult {
            eliminations,
            weakness_signal,
            operators_run: 4,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn npro_produces_four_eliminations() {
        let frags = vec![
            "the reactor is stable under load".to_string(),
            "the reactor is not stable under load".to_string(),
        ];
        let result = NproEngine::activate(&frags, 0.5, 0.5);
        assert_eq!(result.eliminations.len(), 4);
        assert_eq!(result.operators_run, 4);
    }

    #[test]
    fn weak_causal_raises_weakness() {
        let frags = vec!["a".to_string(), "b".to_string()];
        // Lidhje e dobët → dobësi e lartë te REVERSE⁻¹.
        let weak = NproEngine::activate(&frags, 0.1, 0.1);
        let reverse_elim = weak.eliminations.iter()
            .find(|e| e.operator.contains("Reverse")).unwrap();
        assert!(reverse_elim.weakness_score > 0.5);
    }

    #[test]
    fn forbidden_paths_above_ceiling() {
        // Fragment i vetëm → CONTRAST⁻¹ dhe TRACE⁻¹ dobësi të larta.
        let frags = vec!["single fragment only".to_string()];
        let result = NproEngine::activate(&frags, 0.5, 0.5);
        // Të paktën një rrugë e ndaluar (contrast=1.0 > 0.65).
        assert!(!result.weakness_signal.forbidden_paths.is_empty());
    }

    #[test]
    fn weakness_signal_detects_forbidden() {
        let frags = vec!["x".to_string()];
        let result = NproEngine::activate(&frags, 0.5, 0.5);
        // CONTRAST⁻¹ jep 1.0 për fragment të vetëm → forbidden.
        assert!(result.weakness_signal.is_forbidden("Contrast"));
    }

    #[test]
    fn avg_weakness_computed() {
        let frags = vec!["a".to_string(), "b".to_string()];
        let result = NproEngine::activate(&frags, 0.5, 0.5);
        assert!((0.0..=1.0).contains(&result.weakness_signal.avg_weakness));
    }

    #[test]
    fn strong_input_few_forbidden() {
        // Input i fortë → pak forbidden paths.
        let frags = vec![
            "deterministic elimination produces verifiable knowledge".to_string(),
            "each candidate passes through filters before acceptance".to_string(),
            "surviving knowledge becomes primitive over time".to_string(),
        ];
        let result = NproEngine::activate(&frags, 0.9, 0.9);
        // Me input të fortë, forbidden paths < të gjithë.
        assert!(result.weakness_signal.forbidden_paths.len() < 4);
    }
}
