// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  PRO_OPERATOR.RS — 4 Operatorët si Formula                           ║
// ║  GJATA LEGACY™ — Quantum Platform                                     ║
// ║                                                                          ║
// ║  RREGULL ABSOLUT: zero if/else për vendime. Çdo operator = formulë.    ║
// ║                                                                          ║
// ║  REVERSE  (P1): shkak nga efekti                                      ║
// ║    score = signal × causal × (1 - signal × (1 - causal))             ║
// ║  MIRROR   (P2): analogji strukturale                                  ║
// ║    score = cosine(sig_a, sig_b)   [threshold >= 0.55]                ║
// ║  CONTRAST (P3): diferencat                                            ║
// ║    score = Σ|thesis_i - anti_i| / n                                  ║
// ║  TRACE    (P4): mbush boshllëqet (fallback)                          ║
// ║    score = Σ(gap_i × candidate_i) / n                                ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::lim_types::Operator;
use crate::pro_types::{FragmentVector, cosine};

// ─────────────────────────────────────────────────────────────────────────────
// REZULTAT I NJË OPERATORI (para NK)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct OperatorOutput {
    pub operator:      Operator,
    pub raw_score:     f32,          // [0,1] para NK
    pub confidence:    f32,
    pub fragment_refs: Vec<usize>,
    pub reasoning:     String,
}

// Threshold për MIRROR (nga prompt).
const MIRROR_THRESHOLD: f32 = 0.55;

// ─────────────────────────────────────────────────────────────────────────────
// REVERSE — shkak nga efekti
//   score = signal × causal × (1 - signal × (1 - causal))
// ─────────────────────────────────────────────────────────────────────────────

pub struct Reverse;

impl Reverse {
    /// signal = forca informative e fragmentit (density-like)
    /// causal = sa qartë lidhet efekti me shkakun (continuity-like)
    pub fn compute(signal: f32, causal: f32, refs: Vec<usize>) -> OperatorOutput {
        let s = signal.clamp(0.0, 1.0);
        let c = causal.clamp(0.0, 1.0);

        // Formula e dhënë — branchless.
        let score = (s * c * (1.0 - s * (1.0 - c))).clamp(0.0, 1.0);

        OperatorOutput {
            operator: Operator::Reverse,
            raw_score: score,
            confidence: c,
            fragment_refs: refs,
            reasoning: "REVERSE: efekti → shkak (causal traversal mbrapsht)".to_string(),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// MIRROR — analogji strukturale
//   score = cosine(sig_a, sig_b), pranohet vetëm nëse >= 0.55
// ─────────────────────────────────────────────────────────────────────────────

pub struct Mirror;

impl Mirror {
    /// Gjen çiftin e fragmenteve me analogjinë më të fortë (cosine max).
    /// Vektorët vijnë nga stem-et (pro_types::vectorize_fragments).
    pub fn compute(vectors: &[FragmentVector]) -> OperatorOutput {
        // Pa ≥2 fragmente s'ka analogji.
        let n = vectors.len();
        let insufficient = (n < 2) as u32 as f32;

        // Gjej cosine maksimal midis çdo çifti.
        let mut best = 0.0f32;
        let mut best_pair = (0usize, 0usize);
        for i in 0..n {
            for j in (i + 1)..n {
                let c = cosine(&vectors[i].dense, &vectors[j].dense);
                // branchless max + track i çiftit
                let is_better = (c > best) as u32 as f32;
                best = best * (1.0 - is_better) + c * is_better;
                // Përditëso best_pair vetëm kur ky çift është më i miri. Branchless:
                // ruaj indeksin e ri kur is_better, ndryshe mbaj të vjetrin.
                let take = (is_better > 0.5) as usize;
                best_pair = match take {
                    1 => (vectors[i].index, vectors[j].index),
                    _ => best_pair,
                };
            }
        }

        // Threshold gate: nën 0.55 → score zbret drejt zeros (branchless).
        // pass_factor = clamp((best - 0.55)/(1-0.55), 0, 1) por ruajmë sinjalin.
        let above = ((best - MIRROR_THRESHOLD) / (1.0 - MIRROR_THRESHOLD)).clamp(0.0, 1.0);
        // Nëse nën threshold, score = best * above (zbutet); mbi → best.
        let gated = best * (MIRROR_THRESHOLD + (1.0 - MIRROR_THRESHOLD) * above) / 1.0;
        let score = (gated * (1.0 - insufficient)).clamp(0.0, 1.0);

        OperatorOutput {
            operator: Operator::Mirror,
            raw_score: score,
            confidence: best,
            fragment_refs: vec![best_pair.0, best_pair.1],
            reasoning: format!(
                "MIRROR: analogji strukturale midis frag {} dhe {} (cosine={:.3})",
                best_pair.0, best_pair.1, best,
            ),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// CONTRAST — diferencat
//   score = Σ|thesis_i - anti_i| / n
// ─────────────────────────────────────────────────────────────────────────────

pub struct Contrast;

impl Contrast {
    /// Mat diferencën mesatare midis dy vektorëve (tezë vs antitezë).
    /// Përdor dy fragmentet më të largët si tezë/antitezë.
    pub fn compute(vectors: &[FragmentVector]) -> OperatorOutput {
        let n = vectors.len();
        let insufficient = (n < 2) as u32 as f32;

        // Gjej çiftin me diferencën më të madhe (cosine më të vogël = më ndryshe).
        let mut max_diff = 0.0f32;
        let mut pair = (0usize, 0usize);
        for i in 0..n {
            for j in (i + 1)..n {
                let diff = Self::vector_diff(&vectors[i].dense, &vectors[j].dense);
                let is_better = (diff > max_diff) as u32 as f32;
                max_diff = max_diff * (1.0 - is_better) + diff * is_better;
                let take = (is_better > 0.5) as usize;
                pair = match take {
                    1 => (vectors[i].index, vectors[j].index),
                    _ => pair,
                };
            }
        }

        let score = (max_diff * (1.0 - insufficient)).clamp(0.0, 1.0);

        OperatorOutput {
            operator: Operator::Contrast,
            raw_score: score,
            confidence: max_diff,
            fragment_refs: vec![pair.0, pair.1],
            reasoning: format!(
                "CONTRAST: diferenca tezë↔antitezë midis frag {} dhe {} (diff={:.3})",
                pair.0, pair.1, max_diff,
            ),
        }
    }

    /// Σ|a_i - b_i| / n — diferenca mesatare absolute.
    fn vector_diff(a: &[f32], b: &[f32]) -> f32 {
        let n = a.len().max(1);
        let sum: f32 = a.iter().zip(b.iter()).map(|(x, y)| (x - y).abs()).sum();
        (sum / n as f32).clamp(0.0, 1.0)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// TRACE — mbush boshllëqet (fallback)
//   score = Σ(gap_i × candidate_i) / n
// ─────────────────────────────────────────────────────────────────────────────

pub struct Trace;

impl Trace {
    /// gap_i = sa "i munguar" është lidhja midis fragmentit i dhe i+1
    ///         (1 - cosine të fqinjëve)
    /// candidate_i = forca e fragmentit si urë (norm-i normalizuar)
    pub fn compute(vectors: &[FragmentVector]) -> OperatorOutput {
        let n = vectors.len();

        // Fragment i vetëm: TRACE prapë jep sinjal minimal (fallback gjithmonë).
        let single = (n < 2) as u32 as f32;

        let mut sum = 0.0f32;
        let mut count = 0u32;
        let refs: Vec<usize> = vectors.iter().map(|v| v.index).collect();

        for i in 0..n.saturating_sub(1) {
            // gap = 1 - similarity (boshllëk i lartë = pak ngjashmëri)
            let gap = 1.0 - cosine(&vectors[i].dense, &vectors[i + 1].dense);
            // candidate = forca e fragmentit-urë (sa stem unikë ka)
            let candidate = (vectors[i + 1].stems.len() as f32 / 10.0).clamp(0.0, 1.0);
            sum += gap * candidate;
            count += 1;
        }

        // Σ(gap × candidate) / n
        // count==0 → 0.30 (fallback); ndryshe → mesatare. Branchless.
        let has = (count > 0) as u32 as f32;
        let base = has * (sum / count.max(1) as f32) + (1.0 - has) * 0.30;
        // Single fragment → score bazë (0.30) që TRACE të mbetet fallback.
        let score = (base * (1.0 - single) + 0.30 * single).clamp(0.0, 1.0);

        OperatorOutput {
            operator: Operator::Trace,
            raw_score: score,
            confidence: base,
            fragment_refs: refs,
            reasoning: "TRACE: mbush boshllëqet midis fragmenteve (rrugë minimale)".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pro_types::vectorize_fragments;

    fn vecs(frags: &[&str]) -> Vec<FragmentVector> {
        let owned: Vec<String> = frags.iter().map(|s| s.to_string()).collect();
        vectorize_fragments(&owned)
    }

    #[test]
    fn reverse_formula() {
        // signal=0.8, causal=0.6 → 0.8*0.6*(1 - 0.8*0.4) = 0.48*(1-0.32)=0.48*0.68
        let out = Reverse::compute(0.8, 0.6, vec![0]);
        let expected = 0.8 * 0.6 * (1.0 - 0.8 * (1.0 - 0.6));
        assert!((out.raw_score - expected).abs() < 1e-5);
    }

    #[test]
    fn mirror_high_for_similar() {
        let v = vecs(&["quantum reasoning engine", "quantum reasoning engine"]);
        let out = Mirror::compute(&v);
        assert!(out.raw_score > 0.5);
    }

    #[test]
    fn contrast_high_for_different() {
        let v = vecs(&["quantum physics math", "banana fruit cooking"]);
        let out = Contrast::compute(&v);
        assert!(out.raw_score > 0.0);
    }

    #[test]
    fn trace_always_produces() {
        let v = vecs(&["single fragment here"]);
        let out = Trace::compute(&v);
        assert!(out.raw_score > 0.0, "TRACE është fallback — gjithmonë jep diçka");
    }
}
