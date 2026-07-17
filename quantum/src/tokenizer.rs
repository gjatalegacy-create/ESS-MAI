// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  TOKENIZER.RS — LIM Rruga B: tokenizim + stemming                     ║
// ║  GJATA LEGACY™ — Quantum Platform / Linear Info Metric                ║
// ║                                                                          ║
// ║  Rruga B: unicode-segmentation + rust-stemmers KUR janë të pranishme.  ║
// ║  Fallback pure-Rust (zero varësi) kur s'janë — gjithmonë ekzekutues.   ║
// ║                                                                          ║
// ║  APROKSIM (Rruga C: zëvendëso këtu me embeddings):                     ║
// ║    cosine_similarity(embedding) → overlap stem-esh (Jaccard)          ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use std::collections::HashSet;

// ─────────────────────────────────────────────────────────────────────────────
// TOKEN — njësia bazë
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Token {
    /// Forma origjinale (lowercased).
    pub surface: String,
    /// Rrënja (stem) — për krahasim leksikal.
    pub stem: String,
    /// A është fjalë funksionale (stopword)?
    pub is_stopword: bool,
}

impl Token {
    pub fn new(surface: &str) -> Self {
        let lower = surface.to_lowercase();
        let stem = stem_word(&lower);
        let is_stopword = is_stopword(&lower);
        Self { surface: lower, stem, is_stopword }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// TOKENIZIM — fjalë nga tekst
// ─────────────────────────────────────────────────────────────────────────────

/// Ndan tekstin në token-e. Rruga B: unicode word boundaries.
/// Fallback: ndarje me hapësira + heqje pikësimi.
pub fn tokenize(text: &str) -> Vec<Token> {
    #[cfg(feature = "nlp")]
    {
        use unicode_segmentation::UnicodeSegmentation;
        return text
            .unicode_words()
            .map(Token::new)
            .collect();
    }
    #[cfg(not(feature = "nlp"))]
    {
        tokenize_fallback(text)
    }
}

/// Fallback pure-Rust: ndan me jo-alfanumerikë, ruan shkronjat shqipe.
#[cfg(not(feature = "nlp"))]
fn tokenize_fallback(text: &str) -> Vec<Token> {
    text.split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .map(Token::new)
        .collect()
}

// ─────────────────────────────────────────────────────────────────────────────
// STEMMING — rrënja e fjalës
// ─────────────────────────────────────────────────────────────────────────────

/// Stem i fjalës. Rruga B: rust-stemmers (Snowball English).
/// Fallback: heqje suffikesh të zakonshëm (en + sq).
pub fn stem_word(word: &str) -> String {
    #[cfg(feature = "nlp")]
    {
        use rust_stemmers::{Algorithm, Stemmer};
        let stemmer = Stemmer::create(Algorithm::English);
        return stemmer.stem(word).to_string();
    }
    #[cfg(not(feature = "nlp"))]
    {
        stem_fallback(word)
    }
}

/// Stemming heuristik pure-Rust — heq suffikset më të zakonshme.
/// Mjafton për overlap leksikal (krahasim midis fragmenteve).
#[cfg(not(feature = "nlp"))]
fn stem_fallback(word: &str) -> String {
    // Suffikse të renditura nga më i gjati te më i shkurtri.
    const SUFFIXES: &[&str] = &[
        // English
        "ization", "ational", "fulness", "ousness", "iveness",
        "ing", "ied", "ies", "ied", "ment", "ness", "tion", "sion",
        "able", "ible", "ance", "ence", "ed", "es", "s", "ly", "er", "est",
        // Shqip
        "imit", "imet", "imin", "ueshëm", "shëm", "imi", "eve", "ave",
        "ojnë", "uar", "ut", "it", "in", "ës", "ëve", "të",
    ];
    let w = word;
    // Mos shkurto fjalë shumë të shkurtra.
    match w.chars().count() <= 4 {
        true => return w.to_string(),
        false => {}
    }
    for suf in SUFFIXES {
        match w.ends_with(suf) && w.len() - suf.len() >= 3 {
            true => return w[..w.len() - suf.len()].to_string(),
            false => {}
        }
    }
    w.to_string()
}

// ─────────────────────────────────────────────────────────────────────────────
// STOPWORDS — fjalë funksionale (en + sq)
// ─────────────────────────────────────────────────────────────────────────────

/// A është fjalë funksionale që s'mbart informacion semantik?
pub fn is_stopword(word: &str) -> bool {
    const STOPWORDS: &[&str] = &[
        // English
        "the", "a", "an", "and", "or", "but", "is", "are", "was", "were",
        "be", "been", "being", "to", "of", "in", "on", "at", "for", "with",
        "as", "by", "that", "this", "it", "i", "you", "he", "she", "we", "they",
        "do", "does", "did", "have", "has", "had", "will", "would", "can", "could",
        // Shqip
        "dhe", "ose", "por", "është", "janë", "ishte", "të", "e", "i", "një",
        "me", "në", "nga", "për", "si", "që", "ky", "kjo", "ai", "ajo", "ne", "ju",
        "do", "ka", "kanë", "kishte", "mund", "u", "se", "këtë", "atë",
    ];
    STOPWORDS.contains(&word)
}

// ─────────────────────────────────────────────────────────────────────────────
// NDIHMËS LEKSIKALË — për krahasim midis fragmenteve (zemra e Grupit 2)
// ─────────────────────────────────────────────────────────────────────────────

/// Set i stem-eve semantike (jo-stopword) të një teksti.
pub fn semantic_stems(text: &str) -> HashSet<String> {
    tokenize(text)
        .into_iter()
        .filter(|t| !t.is_stopword)
        .map(|t| t.stem)
        .collect()
}

/// Jaccard similarity midis dy seteve stem-esh ∈ [0,1].
/// APROKSIM i cosine_similarity(embeddings) — Rruga C zëvendëson këtu.
pub fn jaccard(a: &HashSet<String>, b: &HashSet<String>) -> f32 {
    match a.is_empty() && b.is_empty() {
        true => return 1.0,
        false => {}
    }
    let inter = a.intersection(b).count() as f32;
    let union = a.union(b).count() as f32;
    let nonzero = (union != 0.0) as u32 as f32;
    nonzero * (inter / union.max(1.0))
}

/// Overlap leksikal i drejtuar: sa nga `b` ekziston te `a` ∈ [0,1].
/// Përdoret për "sa i ri është fragmenti i ri vs të mëparshmit".
pub fn overlap_ratio(prev: &HashSet<String>, new: &HashSet<String>) -> f32 {
    match new.is_empty() {
        true => return 1.0,
        false => {}
    }
    let shared = new.intersection(prev).count() as f32;
    shared / new.len() as f32
}

// ─────────────────────────────────────────────────────────────────────────────
// NDIHMËS STRUKTURORË
// ─────────────────────────────────────────────────────────────────────────────

/// Ndan tekstin në fjali (me . ! ? dhe newline).
pub fn split_sentences(text: &str) -> Vec<String> {
    text.split(|c| c == '.' || c == '!' || c == '?' || c == '\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

/// A përmban teksti ndonjë nga shenjuesit (case-insensitive)?
pub fn contains_any(text: &str, markers: &[&str]) -> bool {
    let lower = text.to_lowercase();
    markers.iter().any(|m| lower.contains(m))
}

/// Numëron sa nga shenjuesit shfaqen.
pub fn count_markers(text: &str, markers: &[&str]) -> usize {
    let lower = text.to_lowercase();
    markers.iter().filter(|m| lower.contains(*m)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_basic() {
        let toks = tokenize("The cat sleeps");
        assert_eq!(toks.len(), 3);
        assert!(toks[0].is_stopword); // "the"
        assert!(!toks[1].is_stopword); // "cat"
    }

    #[test]
    fn jaccard_identical() {
        let a = semantic_stems("quantum reasoning system");
        let b = semantic_stems("quantum reasoning system");
        assert!((jaccard(&a, &b) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn jaccard_disjoint() {
        let a = semantic_stems("quantum physics");
        let b = semantic_stems("banana fruit");
        assert!(jaccard(&a, &b) < 0.1);
    }

    #[test]
    fn overlap_new_fragment() {
        let prev = semantic_stems("reasoning system architecture");
        let new = semantic_stems("reasoning system novel approach");
        // 2 shared (reasoning, system) nga ~4 te new
        let o = overlap_ratio(&prev, &new);
        assert!(o > 0.3 && o < 0.8);
    }
}
