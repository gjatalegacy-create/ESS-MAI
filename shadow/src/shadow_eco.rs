// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  SHADOW_ECO.RS — Shadow Extreme Control Orchestrator                  ║
// ║  Gjata Legacy™ | Arkitekt: Bledar Gjata                    (FAZA 9)    ║
// ║                                                                          ║
// ║  Orkestratori EKSTREM i Shadow — truri që sistemon dijen sipas LLOJIT   ║
// ║  që gjykohet. Ky është "plus" për sistemin Shadow: merr verdiktet e     ║
// ║  brendshme dhe i organizon në hierarkinë epistemike supreme.           ║
// ║                                                                          ║
// ║  HIERARKIA E DIJES (sipas gjykimit):                                   ║
// ║                                                                          ║
// ║    NEGATIVE          → dështim i verifikuar (dije negative)            ║
// ║    HYPOTHESIS:                                                          ║
// ║      • POTENTIAL     → po formohet, ende pa TRL                        ║
// ║      • FACTUAL_CAND  → kandidat-faktik = TRL 3 (provë koncepti)        ║
// ║    KNOWLEDGE:                                                           ║
// ║      • SCIENTIFIC    → dije ekzistente e verifikuar                    ║
// ║      • EXACT_SCIENCE → TRL 4 (FAKTUAR nga bota — inovacion i prekshëm) ║
// ║                                                                          ║
// ║  ARKITEKTURË E FORTË: ky orkestrator është BAZË që mban edhe nën një    ║
// ║  ekosistem 20-30M+ rreshta. Klasifikimi është PASTËR (zero if/else,     ║
// ║  pa gjendje globale të fshehur), determinist, dhe i auditueshëm.       ║
// ║                                                                          ║
// ║  LIGJET E SHADOW & ESS-MAI të ruajtura:                               ║
// ║   • Shadow vendos — orkestratori organizon, s'mbishkruan vendimin.     ║
// ║   • Çdo klasifikim ka PROVË (TRL/dokumentacion) — jo supozim.          ║
// ║   • TRL 3 → TRL 4 kërkon faktim me dokumentacion (GeniusNovel).        ║
// ║  Zero if/else — match/formula/produkt boolean.                        ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::shadow_lab::{TrlInput, ShadowLab, ShadowLabOutcome};
use crate::shadow_genius_novel::{NovelEvidence, NovelVerdict, GeniusNovel, FactualInnovation, TRL4_FACTUAL};
use crate::knowledge_vault::StoreKind;

// ─────────────────────────────────────────────────────────────────────────────
// EPISTEMIC CLASS — klasa epistemike e gjykuar
// ─────────────────────────────────────────────────────────────────────────────

/// Klasifikimi suprem i dijes sipas gjykimit (hierarkia e plotë).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EpistemicClass {
    /// Dështim i verifikuar — dije negative.
    Negative,
    /// Hipotezë potenciale — po formohet, ende pa TRL.
    HypothesisPotential,
    /// Hipotezë kandidat-faktik — TRL 3 (provë koncepti).
    HypothesisFactualCandidate,
    /// Dije shkencore ekzistente e verifikuar.
    Scientific,
    /// Shkenca ekzakte / inovacion — TRL 4 (faktuar nga bota, i prekshëm).
    ExactScienceTrl4,
}

impl EpistemicClass {
    pub fn label(self) -> &'static str {
        match self {
            EpistemicClass::Negative                   => "NEGATIVE",
            EpistemicClass::HypothesisPotential        => "HYPOTHESIS_POTENTIAL",
            EpistemicClass::HypothesisFactualCandidate => "HYPOTHESIS_FACTUAL_CANDIDATE_TRL3",
            EpistemicClass::Scientific                 => "SCIENTIFIC",
            EpistemicClass::ExactScienceTrl4           => "EXACT_SCIENCE_TRL4_FACTUAL",
        }
    }

    /// trl_of — niveli TRL që i korrespondon klasës. Zero if — match.
    pub fn trl_of(self) -> u8 {
        match self {
            EpistemicClass::Negative                   => 0,
            EpistemicClass::HypothesisPotential        => 1,
            EpistemicClass::HypothesisFactualCandidate => 3,
            EpistemicClass::Scientific                 => 3,
            EpistemicClass::ExactScienceTrl4           => 4,
        }
    }

    /// store_kind — te cili store i vault-it i përket kjo klasë. Zero if — match.
    ///
    /// Lidhja me sistemin e ruajtjes ekzistues:
    ///   Negative → Negative; Potential/Candidate → Hypothesis;
    ///   Scientific → Scientific; ExactScienceTrl4 → Fact (i prekshëm).
    pub fn store_kind(self) -> StoreKind {
        match self {
            EpistemicClass::Negative                   => StoreKind::Negative,
            EpistemicClass::HypothesisPotential        => StoreKind::Hypothesis,
            EpistemicClass::HypothesisFactualCandidate => StoreKind::Hypothesis,
            EpistemicClass::Scientific                 => StoreKind::Scientific,
            EpistemicClass::ExactScienceTrl4           => StoreKind::Fact,
        }
    }

    /// is_knowledge — a është dije e konsoliduar (jo hipotezë/dështim)? Zero if.
    pub fn is_knowledge(self) -> bool {
        let scientific = (self as u8) == (EpistemicClass::Scientific as u8);
        let exact      = (self as u8) == (EpistemicClass::ExactScienceTrl4 as u8);
        scientific | exact
    }

    /// is_hypothesis — a është hipotezë (potenciale ose kandidat)? Zero if.
    pub fn is_hypothesis(self) -> bool {
        let potential = (self as u8) == (EpistemicClass::HypothesisPotential as u8);
        let candidate = (self as u8) == (EpistemicClass::HypothesisFactualCandidate as u8);
        potential | candidate
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// CLASSIFICATION INPUT — sinjalet e gjykimit
// ─────────────────────────────────────────────────────────────────────────────

/// Sinjalet që orkestratori përdor për të klasifikuar (nga vendimi suprem).
#[derive(Debug, Clone, Copy)]
pub struct ClassificationInput {
    /// 0/1 — a kaloi judiciary + ligjet sovrane.
    pub verified:    u8,
    /// 0/1 — a është primitiv (vula 500).
    pub primitive:   u8,
    /// Evidenca TRL nga laboratorët (Faza 8).
    pub trl:         TrlInput,
    /// A ekziston dije shkencore ekzistente për këtë (banda Scientific).
    pub is_existing_science: bool,
}

// ─────────────────────────────────────────────────────────────────────────────
// CLASSIFICATION RESULT — verdikti i orkestrimit
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
pub struct ClassificationResult {
    pub class:          EpistemicClass,
    pub trl_level:      u8,
    pub lab_outcome:    ShadowLabOutcome,  // gjykimi TRL i Fazës 8
    pub store_kind:     StoreKind,         // ku regjistrohet
}

// ─────────────────────────────────────────────────────────────────────────────
// SHADOW ECO — orkestratori ekstrem
// ─────────────────────────────────────────────────────────────────────────────

pub struct ShadowEco;

impl ShadowEco {
    /// classify — gjykon llojin e dijes nga sinjalet supreme.
    ///
    /// Logjika hierarkike (prioritet nga lart-poshtë, zero if — match tuple):
    ///   1. verified==0           → Negative (dështim i verifikuar)
    ///   2. is_existing_science   → Scientific (dije ekzistente)
    ///   3. trl_supports & TRL>=3 → HypothesisFactualCandidate (TRL 3)
    ///   4. ndryshe               → HypothesisPotential (po formohet)
    ///
    /// VINI RE: ExactScienceTrl4 NUK arrihet nga classify — arrihet VETËM
    /// përmes faktimit me dokumentacion (classify_with_factualization).
    /// Kjo ruan ligjin: TRL 4 = faktuar nga bota, jo arsyetuar.
    pub fn classify(input: &ClassificationInput) -> ClassificationResult {
        let lab_outcome = ShadowLab::adjudicate(&input.trl);
        let trl_supports = lab_outcome.trl_verdict.trl_supports;

        let verified_ok = input.verified == 1;
        let trl3_ok = trl_supports & (input.trl.trl_level >= 3);

        // Klasifikimi (prioritet i fiksuar via match tuple).
        let class = match (verified_ok, input.is_existing_science, trl3_ok) {
            (false, _, _)    => EpistemicClass::Negative,
            (true, true, _)  => EpistemicClass::Scientific,
            (true, false, true)  => EpistemicClass::HypothesisFactualCandidate,
            (true, false, false) => EpistemicClass::HypothesisPotential,
        };

        ClassificationResult {
            class,
            trl_level: class.trl_of(),
            lab_outcome,
            store_kind: class.store_kind(),
        }
    }

    /// classify_with_factualization — gjykimi i PLOTË me faktim TRL 4.
    ///
    /// Kur ka dokumentacion (NovelEvidence), orkestratori provon ngritjen
    /// në TRL 4. Nëse faktohet → ExactScienceTrl4 + FactualInnovation.
    /// Nëse jo → kthehet te klasifikimi normal (mbetet TRL 3 ose më ulët).
    ///
    /// Kjo është LIDHJA e tre laboratorëve: evidenca TRL (Quantum/Light/Shadow)
    /// + dokumentacioni real → faktim suprem.
    /// Zero if — match mbi faktimin.
    pub fn classify_with_factualization(
        input: &ClassificationInput,
        evidence: &NovelEvidence,
        registered_at: u64,
    ) -> (ClassificationResult, NovelVerdict, Option<FactualInnovation>) {
        // Gjykimi i faktimit (GeniusNovel, Faza 9).
        let novel_verdict = GeniusNovel::adjudicate(evidence);

        // Klasifikimi bazë (pa faktim).
        let base = Self::classify(input);

        // Nëse faktohet → ngri në ExactScienceTrl4 + prodho FactualInnovation.
        // Zero if — match mbi factualized.
        let (result, innovation) = match novel_verdict.factualized {
            true => {
                let exact = EpistemicClass::ExactScienceTrl4;
                let innovation = GeniusNovel::register(evidence, &novel_verdict, registered_at);
                (
                    ClassificationResult {
                        class: exact,
                        trl_level: exact.trl_of(),
                        lab_outcome: base.lab_outcome,
                        store_kind: exact.store_kind(),
                    },
                    Some(innovation),
                )
            }
            false => (base, None),
        };

        (result, novel_verdict, innovation)
    }

    /// is_factual_trl4 — a është rezultati një inovacion faktik TRL 4? Zero if.
    pub fn is_factual_trl4(result: &ClassificationResult) -> bool {
        let class_ok = (result.class as u8) == (EpistemicClass::ExactScienceTrl4 as u8);
        let trl_ok   = result.trl_level == TRL4_FACTUAL;
        class_ok & trl_ok
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strong_trl() -> TrlInput {
        TrlInput {
            trl_level: 3, trl_passed: true, confidence: 0.85,
            reproducibility: 0.9, measurable: true, reproducible: true,
        }
    }
    fn weak_trl() -> TrlInput {
        TrlInput {
            trl_level: 1, trl_passed: false, confidence: 0.3,
            reproducibility: 0.2, measurable: false, reproducible: false,
        }
    }

    fn input(verified: u8, trl: TrlInput, existing: bool) -> ClassificationInput {
        ClassificationInput {
            verified, primitive: verified, trl, is_existing_science: existing,
        }
    }

    #[test]
    fn class_trl_mapping() {
        assert_eq!(EpistemicClass::Negative.trl_of(), 0);
        assert_eq!(EpistemicClass::HypothesisPotential.trl_of(), 1);
        assert_eq!(EpistemicClass::HypothesisFactualCandidate.trl_of(), 3);
        assert_eq!(EpistemicClass::ExactScienceTrl4.trl_of(), 4);
    }

    #[test]
    fn class_store_mapping() {
        assert_eq!(EpistemicClass::Negative.store_kind(), StoreKind::Negative);
        assert_eq!(EpistemicClass::HypothesisFactualCandidate.store_kind(), StoreKind::Hypothesis);
        assert_eq!(EpistemicClass::Scientific.store_kind(), StoreKind::Scientific);
        assert_eq!(EpistemicClass::ExactScienceTrl4.store_kind(), StoreKind::Fact);
    }

    #[test]
    fn unverified_is_negative() {
        let r = ShadowEco::classify(&input(0, strong_trl(), false));
        assert_eq!(r.class, EpistemicClass::Negative);
        assert_eq!(r.trl_level, 0);
    }

    #[test]
    fn existing_science_is_scientific() {
        let r = ShadowEco::classify(&input(1, strong_trl(), true));
        assert_eq!(r.class, EpistemicClass::Scientific);
        assert!(r.class.is_knowledge());
    }

    #[test]
    fn strong_trl_is_factual_candidate() {
        // Verified + TRL i fortë (>=3) + jo ekzistuese → kandidat-faktik TRL 3.
        let r = ShadowEco::classify(&input(1, strong_trl(), false));
        assert_eq!(r.class, EpistemicClass::HypothesisFactualCandidate);
        assert_eq!(r.trl_level, 3);
        assert!(r.class.is_hypothesis());
    }

    #[test]
    fn weak_trl_is_potential() {
        // Verified por TRL i dobët → hipotezë potenciale.
        let r = ShadowEco::classify(&input(1, weak_trl(), false));
        assert_eq!(r.class, EpistemicClass::HypothesisPotential);
        assert_eq!(r.trl_level, 1);
    }

    #[test]
    fn classify_never_reaches_trl4_alone() {
        // KRITIKE: classify (pa dokumentacion) s'arrin KURRË TRL 4.
        let r = ShadowEco::classify(&input(1, strong_trl(), false));
        assert_ne!(r.class, EpistemicClass::ExactScienceTrl4);
    }

    fn full_evidence(pid: u64) -> NovelEvidence {
        NovelEvidence {
            project_id: pid,
            files: vec![
                crate::shadow_genius_novel::EvidenceFile::from_bytes(
                    vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A]),  // PNG
                crate::shadow_genius_novel::EvidenceFile::from_bytes(
                    vec![0x00, 0x00, 0x00, 0x18, 0x66, 0x74, 0x79, 0x70]),  // MP4
            ],
            description: "How the innovation was produced: built and tested the working POC.".to_string(),
            trl: strong_trl(),
        }
    }

    #[test]
    fn factualization_raises_to_trl4() {
        // Me dokumentacion të plotë → ngrihet në TRL 4 ExactScience.
        let (result, verdict, innovation) =
            ShadowEco::classify_with_factualization(&input(1, strong_trl(), false), &full_evidence(7), 9999);
        assert!(verdict.factualized);
        assert_eq!(result.class, EpistemicClass::ExactScienceTrl4);
        assert_eq!(result.trl_level, 4);
        assert_eq!(result.store_kind, StoreKind::Fact);
        // Inovacioni faktik u prodhua.
        assert!(innovation.is_some());
        assert_eq!(innovation.unwrap().project_id, 7);
    }

    #[test]
    fn no_documentation_stays_candidate() {
        // Pa provë vizuale → mbetet kandidat-faktik TRL 3 (nuk ngrihet).
        let weak_evidence = NovelEvidence {
            project_id: 8,
            files: vec![],  // pa skedarë
            description: "A description long enough but no visual proof here.".to_string(),
            trl: strong_trl(),
        };
        let (result, verdict, innovation) =
            ShadowEco::classify_with_factualization(&input(1, strong_trl(), false), &weak_evidence, 1000);
        assert!(!verdict.factualized);
        // Mbetet klasifikimi bazë (kandidat-faktik TRL 3).
        assert_eq!(result.class, EpistemicClass::HypothesisFactualCandidate);
        assert!(innovation.is_none());
    }

    #[test]
    fn is_factual_trl4_detector() {
        let (result, _, _) =
            ShadowEco::classify_with_factualization(&input(1, strong_trl(), false), &full_evidence(9), 500);
        assert!(ShadowEco::is_factual_trl4(&result));
        // Klasifikim normal → jo TRL 4.
        let normal = ShadowEco::classify(&input(1, strong_trl(), false));
        assert!(!ShadowEco::is_factual_trl4(&normal));
    }
}
