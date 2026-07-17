// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  NPRO_OPERATOR.RS — 4 Operatorët MBRAPSHT (Negative PRO)              ║
// ║  GJATA LEGACY™ — Quantum Platform                                     ║
// ║                                                                          ║
// ║  NPRO përdor TË NJËJTËT 4 operatorë si PRO, por të aplikuar MBRAPSHT:  ║
// ║  ku PRO kërkon forcën, NPRO kërkon dobësinë (eleminimin).             ║
// ║                                                                          ║
// ║   REVERSE⁻¹ = 1 − reverse_score  (sa dobët lidhet shkaku me efektin)  ║
// ║   MIRROR⁻¹  = 1 − cosine_max     (mungesa e analogjisë → false analogy)║
// ║   CONTRAST⁻¹= 1 − Σ|diff|/n      (mungesa e kontrastit → konfuzion)   ║
// ║   TRACE⁻¹   = 1 − trace_score    (mungesa e rrugës → dead-end)        ║
// ║                                                                          ║
// ║  weakness_score i lartë = rrugë e dobët → kandidat për eliminim.       ║
// ║  Zero if/else — vetëm formula matematike.                            ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::pro_types::{FragmentVector, cosine};
use crate::pro_operator::OperatorOutput;
use crate::lim_types::Operator;

// ─────────────────────────────────────────────────────────────────────────────
// REVERSE⁻¹ — sa dobët traversohet shkaku→efekti
// ─────────────────────────────────────────────────────────────────────────────

pub struct NReverse;

impl NReverse {
    /// weakness = 1 − (signal × causal × (1 − signal×(1−causal))).
    /// Sa më e ulët lidhja kauzale, aq më e lartë dobësia.
    pub fn compute(signal: f32, causal: f32, refs: Vec<usize>) -> OperatorOutput {
        let s = signal.clamp(0.0, 1.0);
        let c = causal.clamp(0.0, 1.0);
        let strength = (s * c * (1.0 - s * (1.0 - c))).clamp(0.0, 1.0);
        let weakness = 1.0 - strength;
        OperatorOutput {
            operator: Operator::Reverse,
            raw_score: weakness,
            confidence: weakness,
            fragment_refs: refs,
            reasoning: "REVERSE⁻¹: dobësi e lidhjes kauzale (forbidden path candidate)".to_string(),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// MIRROR⁻¹ — mungesa e analogjisë (false analogy detection)
// ─────────────────────────────────────────────────────────────────────────────

pub struct NMirror;

impl NMirror {
    /// weakness = 1 − cosine_max. Çiftet pa analogji → analogji të rreme.
    pub fn compute(vectors: &[FragmentVector]) -> OperatorOutput {
        // Gjej cosine maksimal (analogjinë më të fortë që ekziston).
        let mut max_cos = 0.0f32;
        let n = vectors.len();
        let mut i = 0usize;
        while i < n {
            let mut j = i + 1;
            while j < n {
                let c = cosine(&vectors[i].dense, &vectors[j].dense);
                max_cos = max_cos.max(c);
                j += 1;
            }
            i += 1;
        }
        // weakness = 1 − max_cos. Pak analogji → false analogy risk.
        let weakness = (1.0 - max_cos).clamp(0.0, 1.0);
        OperatorOutput {
            operator: Operator::Mirror,
            raw_score: weakness,
            confidence: weakness,
            fragment_refs: vec![],
            reasoning: "MIRROR⁻¹: mungesë analogjie (false analogy)".to_string(),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// CONTRAST⁻¹ — mungesa e kontrastit (konfuzion semantik)
// ─────────────────────────────────────────────────────────────────────────────

pub struct NContrast;

impl NContrast {
    /// weakness = 1 − max_diff. Mungesa e kontrastit → konfuzion (s'dallon dot).
    pub fn compute(vectors: &[FragmentVector]) -> OperatorOutput {
        let n = vectors.len();
        // single ∈ {0,1}: nëse < 2 fragmente, s'ka kontrast → dobësi e plotë.
        let single = (n < 2) as u32 as f32;

        let mut max_diff = 0.0f32;
        let mut i = 0usize;
        while i < n {
            let mut j = i + 1;
            while j < n {
                let d = Self::mean_abs_diff(&vectors[i].dense, &vectors[j].dense);
                max_diff = max_diff.max(d);
                j += 1;
            }
            i += 1;
        }
        // weakness = 1 − max_diff (kur ka çifte); single → 1.0.
        let weakness = (single * 1.0 + (1.0 - single) * (1.0 - max_diff)).clamp(0.0, 1.0);
        OperatorOutput {
            operator: Operator::Contrast,
            raw_score: weakness,
            confidence: weakness,
            fragment_refs: vec![],
            reasoning: "CONTRAST⁻¹: mungesë kontrasti (konfuzion semantik)".to_string(),
        }
    }

    fn mean_abs_diff(a: &[f32], b: &[f32]) -> f32 {
        let n = a.len().min(b.len()).max(1) as f32;
        let sum: f32 = a.iter().zip(b.iter()).map(|(x, y)| (x - y).abs()).sum();
        sum / n
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// TRACE⁻¹ — mungesa e rrugës (dead-end detection)
// ─────────────────────────────────────────────────────────────────────────────

pub struct NTrace;

impl NTrace {
    /// weakness = 1 − trace_score. Pak rrugë valid → dead-end.
    pub fn compute(vectors: &[FragmentVector]) -> OperatorOutput {
        let n = vectors.len();
        let single = (n < 2) as u32 as f32;

        // base = mesatarja e ngjashmërisë me fragmentin e parë (gap-fill potential).
        let mut sum = 0.0f32;
        let mut count = 0.0f32;
        let mut i = 1usize;
        while i < n {
            sum += cosine(&vectors[0].dense, &vectors[i].dense);
            count += 1.0;
            i += 1;
        }
        let base = sum / count.max(1.0);
        // strength = base (kur ka shumë); single → 0.30 (default i ulët).
        let strength = (base * (1.0 - single) + 0.30 * single).clamp(0.0, 1.0);
        let weakness = 1.0 - strength;
        OperatorOutput {
            operator: Operator::Trace,
            raw_score: weakness,
            confidence: weakness,
            fragment_refs: vec![],
            reasoning: "TRACE⁻¹: mungesë rruge valide (dead-end)".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fv(dense: Vec<f32>) -> FragmentVector {
        FragmentVector { index: 0, stems: std::collections::HashSet::new(), dense }
    }

    #[test]
    fn nreverse_high_when_weak_causal() {
        // Lidhje e fortë → weakness e ulët.
        let strong = NReverse::compute(0.9, 0.9, vec![]);
        // Lidhje e dobët → weakness e lartë.
        let weak = NReverse::compute(0.1, 0.1, vec![]);
        assert!(weak.raw_score > strong.raw_score);
    }

    #[test]
    fn nreverse_is_complement_of_reverse() {
        // weakness = 1 - strength. signal=0.5, causal=0.5.
        let s = 0.5f32; let c = 0.5f32;
        let strength = s * c * (1.0 - s * (1.0 - c));
        let out = NReverse::compute(0.5, 0.5, vec![]);
        assert!((out.raw_score - (1.0 - strength)).abs() < 1e-5);
    }

    #[test]
    fn nmirror_high_without_analogy() {
        // Vektorë identikë → cosine 1.0 → weakness 0.0.
        let same = vec![fv(vec![1.0, 0.0]), fv(vec![1.0, 0.0])];
        let out_same = NMirror::compute(&same);
        assert!(out_same.raw_score < 0.1);
        // Vektorë ortogonalë → cosine 0.0 → weakness 1.0 (false analogy).
        let ortho = vec![fv(vec![1.0, 0.0]), fv(vec![0.0, 1.0])];
        let out_ortho = NMirror::compute(&ortho);
        assert!(out_ortho.raw_score > 0.9);
    }

    #[test]
    fn ncontrast_single_fragment_max_weakness() {
        let one = vec![fv(vec![1.0, 0.0])];
        let out = NContrast::compute(&one);
        // S'ka kontrast → dobësi e plotë.
        assert!((out.raw_score - 1.0).abs() < 1e-5);
    }

    #[test]
    fn ntrace_dead_end_high() {
        // Fragment i vetëm → strength 0.30 → weakness 0.70.
        let one = vec![fv(vec![1.0, 0.0])];
        let out = NTrace::compute(&one);
        assert!((out.raw_score - 0.70).abs() < 1e-5);
    }

    #[test]
    fn all_weakness_in_range() {
        let v = vec![fv(vec![0.5, 0.5]), fv(vec![0.3, 0.7])];
        assert!((0.0..=1.0).contains(&NReverse::compute(0.5, 0.5, vec![]).raw_score));
        assert!((0.0..=1.0).contains(&NMirror::compute(&v).raw_score));
        assert!((0.0..=1.0).contains(&NContrast::compute(&v).raw_score));
        assert!((0.0..=1.0).contains(&NTrace::compute(&v).raw_score));
    }
}
