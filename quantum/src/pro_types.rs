// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  PRO_TYPES.RS — Primitive Reasoning Origin: tipet                     ║
// ║  GJATA LEGACY™ — Quantum Platform                                     ║
// ║                                                                          ║
// ║  PROCandidate, ProResult, dhe vektorizimi i fragmenteve.              ║
// ║  Vektorët (për MIRROR dot product) vijnë nga stem-et e fragmenteve.    ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::lim_types::Operator;
use std::collections::HashSet;

// ─────────────────────────────────────────────────────────────────────────────
// PRO CANDIDATE — output i një operatori (max 1 për operator)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct PROCandidate {
    pub candidate_id:  String,
    pub operator:      Operator,
    pub score:         f32,          // [0,1] pas NK
    pub confidence:    f32,          // [0,1]
    pub reasoning:     String,       // shpjegimi
    pub fragment_refs: Vec<usize>,   // indekset e fragmenteve
}

impl PROCandidate {
    /// candidate_id nga operatori + fragmentet (jo SHA256 ende).
    pub fn make_id(operator: Operator, refs: &[usize]) -> String {
        let refs_str: Vec<String> = refs.iter().map(|r| r.to_string()).collect();
        format!("{}_{}", operator.name(), refs_str.join("-"))
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// PRO RESULT — output i plotë i PRO (max 5 kandidatë)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ProResult {
    /// Kandidatët, renditur sipas score (top 5).
    pub candidates: Vec<PROCandidate>,
    /// Sa operatorë u ekzekutuan.
    pub operators_run: usize,
    /// Sa kandidatë u bllokuan nga NK (HARD).
    pub nk_blocked: usize,
    /// epistemic_mass që erdhi nga LIM.
    pub source_mass: f32,
    /// Sa operatorë paralel lejoi hardware.
    pub hw_parallel: u8,
    /// Sa worker-a scoped u bashkuan (përfshin worker që dështoi/panicked).
    pub workers_joined: usize,
    /// Sa fragmente u pranuan realisht nga lease-i.
    pub fragments_admitted: usize,
    /// Working-set i vlerësuar që u pranua nga lease-i.
    pub working_set_bytes: u64,
}

impl ProResult {
    pub fn best(&self) -> Option<&PROCandidate> {
        self.candidates.first()
    }

    pub fn report(&self) -> String {
        let mut out = format!(
            "ProResult: {} kandidatë | operatorë={} | workers_joined={} | fragmente={} | working_set={}B | NK_blocked={} | mass={:.3} | hw_parallel={}\n",
            self.candidates.len(), self.operators_run, self.workers_joined,
            self.fragments_admitted, self.working_set_bytes, self.nk_blocked,
            self.source_mass, self.hw_parallel,
        );
        for (i, c) in self.candidates.iter().enumerate() {
            out += &format!(
                "  {}. [{}] score={:.3} conf={:.3} refs={:?}\n     {}\n",
                i + 1, c.operator.name(), c.score, c.confidence,
                c.fragment_refs, c.reasoning,
            );
        }
        out
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// FRAGMENT VECTOR — vektorizimi për operatorët (MIRROR dot product)
// ─────────────────────────────────────────────────────────────────────────────

/// Vektor numerik i një fragmenti, ndërtuar nga stem-et.
/// Përdoret nga MIRROR (cosine) dhe operatorët e tjerë.
#[derive(Debug, Clone)]
pub struct FragmentVector {
    /// Indeksi i fragmentit në grumbullim.
    pub index: usize,
    /// Set i stem-eve (për Jaccard/overlap).
    pub stems: HashSet<String>,
    /// Vektor i dendur mbi fjalor të përbashkët (për dot product).
    pub dense: Vec<f32>,
}

/// Ndërton vektorë të dendur për një grup fragmentesh mbi fjalor të përbashkët.
/// Çdo dimension = një stem unik; vlera = frekuenca normalizuar (TF).
pub fn vectorize_fragments(fragments: &[String]) -> Vec<FragmentVector> {
    use crate::tokenizer::{tokenize};

    // 1. Ndërto fjalorin e përbashkët (të gjitha stem-et jo-stopword).
    let mut vocab: Vec<String> = Vec::new();
    let mut vocab_set: HashSet<String> = HashSet::new();
    let mut per_frag_stems: Vec<Vec<String>> = Vec::new();

    for frag in fragments {
        let stems: Vec<String> = tokenize(frag)
            .into_iter()
            .filter(|t| !t.is_stopword)
            .map(|t| t.stem)
            .collect();
        for s in &stems {
            // insert kthen true nëse i ri → push vetëm atëherë.
            match vocab_set.insert(s.clone()) {
                true => vocab.push(s.clone()),
                false => {}
            }
        }
        per_frag_stems.push(stems);
    }

    // 2. Për çdo fragment, ndërto vektorin TF mbi fjalorin.
    let dim = vocab.len().max(1);
    let mut vectors = Vec::new();

    for (i, stems) in per_frag_stems.iter().enumerate() {
        let mut dense = vec![0.0f32; dim];
        let stem_set: HashSet<String> = stems.iter().cloned().collect();

        match stems.is_empty() {
            false => {
                for (d, vocab_stem) in vocab.iter().enumerate() {
                    let count = stems.iter().filter(|s| *s == vocab_stem).count();
                    dense[d] = count as f32 / stems.len() as f32; // TF normalizuar
                }
            }
            true => {}
        }

        vectors.push(FragmentVector {
            index: i,
            stems: stem_set,
            dense,
        });
    }

    vectors
}

// ─────────────────────────────────────────────────────────────────────────────
// OPERACIONE VEKTORIALE — për operatorët
// ─────────────────────────────────────────────────────────────────────────────

/// Dot product i dy vektorëve.
pub fn dot(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

/// Norma (gjatësia) e një vektori.
pub fn norm(a: &[f32]) -> f32 {
    dot(a, a).sqrt()
}

/// Cosine similarity ∈ [0,1] për vektorë jo-negativë (TF).
/// MIRROR: score = dot(a,b) / (|a| × |b|).
pub fn cosine(a: &[f32], b: &[f32]) -> f32 {
    let na = norm(a);
    let nb = norm(b);
    match na == 0.0 || nb == 0.0 {
        true => return 0.0,
        false => {}
    }
    (dot(a, b) / (na * nb)).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vectorize_produces_aligned_vectors() {
        let frags = vec![
            "quantum reasoning system".to_string(),
            "quantum neural network".to_string(),
        ];
        let vecs = vectorize_fragments(&frags);
        assert_eq!(vecs.len(), 2);
        // Të dy vektorët kanë të njëjtën dimension (fjalor i përbashkët).
        assert_eq!(vecs[0].dense.len(), vecs[1].dense.len());
    }

    #[test]
    fn cosine_identical_high() {
        let frags = vec![
            "deterministic elimination engine".to_string(),
            "deterministic elimination engine".to_string(),
        ];
        let v = vectorize_fragments(&frags);
        let c = cosine(&v[0].dense, &v[1].dense);
        assert!((c - 1.0).abs() < 1e-5);
    }

    #[test]
    fn cosine_disjoint_low() {
        let frags = vec![
            "quantum physics".to_string(),
            "banana cooking".to_string(),
        ];
        let v = vectorize_fragments(&frags);
        let c = cosine(&v[0].dense, &v[1].dense);
        assert!(c < 0.1);
    }
}
