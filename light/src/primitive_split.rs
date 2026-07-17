// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  PRIMITIVE_SPLIT.RS — ESS-MAI LIGHT: KOLAPSI I KOORDINIMIT               ║
// ║  Gjata Legacy™  |  Arkitekt: Bledar Gjata                                ║
// ║                                                                          ║
// ║  FRAGMENTI I FORMULËS I GDHENDUR KËTU:   i0 → PA → (Xi, Yi)              ║
// ║                                                                          ║
// ║  Light është KOORDINUESE: nuk arsyeton thellë, nuk vendos — KOORDINON.   ║
// ║  Split-i i i0 në dy hapësirat primitive bëhet me bashkëveprimin e:       ║
// ║                                                                          ║
// ║  • ALGORITMIT (roli semantik determinist):                               ║
// ║      – terma-ATRIBUT (cilësi/ngjyra/gjendje të njohura SQ+EN)  → Xi     ║
// ║      – terma-DOMEN  (përputhen me regjistrin kanonik të domeneve) → Xi  ║
// ║      – terma-BOSHT  (emrat bërthamë të mbetur)                  → Yi    ║
// ║      – Intent::Command → termat = kushte veprimi (Xi), objektivi → Yi   ║
// ║                                                                          ║
// ║  • MINI-ALGORITMIT (plotësuesi koordinu es — garanton invariantet):      ║
// ║      – Yi bosh & |Xi|≥2 → termi i FUNDIT i Xi kalon në Yi               ║
// ║        (boshti i përgjigjes = termi më specifik, zakonisht i fundit)     ║
// ║      – Xi bosh & |Yi|≥2 → termi i PARË i Yi kalon në Xi                 ║
// ║                                                                          ║
// ║  INVARIANTET (kontrata collapse::PrimitiveSplit):                        ║
// ║      Xi ∩ Yi = ∅        Xi ∪ Yi = domain_terms(PA)                       ║
// ║      asgjë s'shpiket — vetëm termat e tokenizuar nga lingua (I PAPREKUR)║
// ║                                                                          ║
// ║  LIGJE: zero if/else (vetëm match), zero clone i panevojshëm, zero deps.║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::lab_contracts::{PrimitiveSplit, normalize_domain_key};
use crate::lingua::{Intent, SystemQuery};

/// Terma-ATRIBUT: cilësi primitive (ngjyra, madhësi, gjendje) SQ + EN.
/// Këta përshkruajnë objektin hyrës → shkojnë në Xi (kushtet).
const ATTRIBUTE_TERMS: &[&str] = &[
    // ngjyra SQ
    "kuq", "kuqe", "zi", "zezë", "bardhë", "bardh", "jeshil", "gjelbër",
    "blu", "kaltër", "verdhë", "portokalli", "vjollcë", "gri", "kafe",
    // ngjyra EN
    "red", "black", "white", "green", "blue", "yellow", "orange",
    "purple", "gray", "grey", "brown",
    // madhësi / gjendje SQ
    "madh", "madhe", "vogël", "gjatë", "shkurtër", "ftohtë", "ngrohtë",
    "nxehtë", "lartë", "ulët", "shpejtë", "ngadaltë", "vjetër", "renë",
    // madhësi / gjendje EN
    "big", "large", "small", "long", "short", "cold", "warm", "hot",
    "high", "low", "fast", "slow", "old", "new",
];

/// Kolapsi i koordinimit: PA (termat e i0-s) → (Xi, Yi).
///
/// Merr SystemQuery-n e gjallë (dalja e lingua-s — tokenizimi I PAPREKUR)
/// dhe koordinon çdo term primitiv në NJËRËN hapësirë. Determinist i plotë:
/// i njëjti input → i njëjti split, gjithmonë.
pub fn split_primitive(query: &SystemQuery) -> PrimitiveSplit {
    let mut xi: Vec<String> = Vec::new();
    let mut yi: Vec<String> = Vec::new();

    // ── ALGORITMI: caktimi i rolit semantik, term-për-term ────────────────
    for term in query.domain_terms.iter() {
        let is_attribute = ATTRIBUTE_TERMS.contains(&term.as_str());
        let is_domain    = normalize_domain_key(term).is_some();
        let route_to_xi  = match query.intent {
            // Komanda: termat janë kushte veprimi (Xi); boshti caktohet
            // nga mini-algoritmi (termi i fundit → objektivi Yi).
            Intent::Command => true,
            // Query/Hypothesis/Statement: atributet e domenet përshkruajnë
            // objektin hyrës (Xi); bërthamat emërore janë boshte dalëse (Yi).
            _ => is_attribute || is_domain,
        };
        match route_to_xi {
            true  => xi.push(term.clone()),
            false => yi.push(term.clone()),
        }
    }

    // ── MINI-ALGORITMI: plotësuesi koordinues (garancitë) ─────────────────
    // Yi bosh & Xi ka ≥2 → boshti i përgjigjes = termi i fundit i Xi.
    match (yi.is_empty(), xi.len() >= 2) {
        (true, true) => {
            match xi.pop() {
                Some(last) => yi.push(last),
                None => {}
            }
        }
        _ => {}
    }
    // Xi bosh & Yi ka ≥2 → konteksti hyrës = termi i parë i Yi.
    match (xi.is_empty(), yi.len() >= 2) {
        (true, true) => {
            let first = yi.remove(0);
            xi.push(first);
        }
        _ => {}
    }

    PrimitiveSplit { xi, yi }
}

// ════════════════════════════════════════════════════════════════════════════
// TESTE — shembujt e Arkitektit + invariantet e kontratës
// ════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lingua::LinguaCore;

    fn split_of(text: &str) -> PrimitiveSplit {
        let core = LinguaCore::new();
        let q = core.parse_input(text);
        split_primitive(&q)
    }

    #[test]
    fn shqiponja_example_attributes_to_xi_axis_to_yi() {
        // Shembulli i Arkitektit: flamur, kuq, zi, shqiponjë
        let s = split_of("Çfarë është flamuri kuq e zi me shqiponjë?");
        // atributet (kuq, zi — pas tokenizimit len>2) → Xi
        assert!(s.xi.iter().any(|t| t == "kuq"));
        // bërthamat emërore (flamuri…/shqiponjë) → Yi (boshti i përgjigjes)
        assert!(s.yi.iter().any(|t| t.contains("shqiponj") || t.contains("flamur")));
        assert!(s.has_material());
    }

    #[test]
    fn invariants_disjoint_and_complete() {
        let core = LinguaCore::new();
        let q = core.parse_input("What is the large red industrial motor sensor?");
        let s = split_primitive(&q);
        // Xi ∩ Yi = ∅
        for t in s.xi.iter() {
            assert!(!s.yi.contains(t), "term në të dy hapësirat: {}", t);
        }
        // Xi ∪ Yi = domain_terms (asgjë s'humbet, asgjë s'shpiket)
        let mut all: Vec<String> = s.xi.iter().chain(s.yi.iter()).cloned().collect();
        all.sort();
        let mut want = q.domain_terms.clone();
        want.sort();
        assert_eq!(all, want);
    }

    #[test]
    fn mini_algorithm_guarantees_axis_when_all_attributes() {
        // Vetëm atribute → algoritmi i çon të gjitha në Xi;
        // mini-algoritmi transferon të fundit në Yi (boshti).
        let s = split_of("kuq zi bardhë jeshil");
        assert!(!s.xi.is_empty());
        assert!(!s.yi.is_empty());
    }

    #[test]
    fn command_intent_last_term_becomes_target() {
        let s = split_of("Krijo raportin final laboratori");
        // Command: kushtet → Xi; mini-algoritmi → objektivi (i fundit) në Yi
        assert!(!s.yi.is_empty());
        assert!(s.has_material());
    }

    #[test]
    fn deterministic_same_input_same_split() {
        let a = split_of("fizikë kuantike matje ftohtë sensor");
        let b = split_of("fizikë kuantike matje ftohtë sensor");
        assert_eq!(a, b);
    }

    #[test]
    fn empty_terms_no_material_honest_refusal_path() {
        // Vetëm stopwords → 0 terma → pa material (→ refuzim i ndershëm më vonë)
        let s = split_of("dhe ose por");
        assert!(!s.has_material());
    }
}
