// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  SHADOW_LAB.RS — Digital Lab në KARAKTERIN E SHADOW                   ║
// ║  Gjata Legacy™ | Arkitekt: Bledar Gjata                    (FAZA 8)    ║
// ║                                                                          ║
// ║  Te Quantum, Digital Lab MAT (prodhon evidencë për PIM).               ║
// ║  Te SHADOW, Digital Lab GJYKON — TRL bëhet rëndues i vendimit suprem.   ║
// ║                                                                          ║
// ║  SHKRIRJA me ligjin e Shadow: TRL nuk është thjesht matje — është një  ║
// ║  PORTË SUPREME shtesë që e bën vendimin MË TË RËNDË. Një input duhet të ║
// ║  kalojë jo vetëm judiciary + vula 500, por edhe pjekurinë TRL që       ║
// ║  Shadow e gjykon me autoritetin e tij të fundit.                       ║
// ║                                                                          ║
// ║  INVARIANTI (nga NASA TRL 1974, adaptuar suprem):                      ║
// ║   "asnjë promovim pa SHADOW_FILTER_GATE + TRL3 threshold"             ║
// ║                                                                          ║
// ║  Bazuar EKZAKT te kontrata e Quantum lab (TrlEvidence, THRESHOLD_TRL3   ║
// ║  = 0.70, FilterGate produkt-boolean), por më strikt sepse Shadow është ║
// ║  autoriteti suprem. Asnjë hamendësim — vetëm fusha që ekzistojnë.      ║
// ║  Zero if/else — match/formula/produkt boolean.                        ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::types::KnowledgeBand;

// ─────────────────────────────────────────────────────────────────────────────
// KONSTANTET — pragjet TRL supreme (identike me Quantum + rëndues Shadow)
// ─────────────────────────────────────────────────────────────────────────────

/// Threshold TRL3 — IDENTIK me Quantum (THRESHOLD_TRL3 = 0.70).
pub const SHADOW_THRESHOLD_TRL3: f32 = 0.70;

/// Niveli minimal TRL që Shadow kërkon për dije primitive/legacy.
/// (Më strikt se Quantum: Shadow gjykon me autoritetin suprem.)
pub const SHADOW_MIN_TRL: u8 = 3;

/// Pjekuria minimale TRL që një input të hyjë në bandën Scientific.
pub const SHADOW_SCIENTIFIC_TRL: u8 = 3;

// ─────────────────────────────────────────────────────────────────────────────
// TRL JUDGMENT — gjykimi TRL i Shadow-it (input për vendimin suprem)
// ─────────────────────────────────────────────────────────────────────────────

/// Hyrjet që Shadow gjykon me TRL — të mbledhura nga evidenca e Lab-it.
/// (Mirror i fushave të TrlEvidence te Quantum që kanë rëndësi për vendim.)
#[derive(Debug, Clone, Copy)]
pub struct TrlInput {
    pub trl_level:       u8,    // niveli TRL i arritur
    pub trl_passed:      bool,  // a kaloi validimin TRL3
    pub confidence:      f32,   // [0,1] besueshmëria e matjes
    pub reproducibility: f32,   // [0,1] riprodhueshmëria
    pub measurable:      bool,  // a është i matshëm (FilterGate)
    pub reproducible:    bool,  // a është i riprodhueshëm (FilterGate)
}

impl TrlInput {
    /// pending — kur Lab s'u konsultua (Shadow e di që mungon evidenca TRL).
    pub fn pending() -> Self {
        Self {
            trl_level: 0,
            trl_passed: false,
            confidence: 0.0,
            reproducibility: 0.0,
            measurable: false,
            reproducible: false,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// SHADOW FILTER GATE — porta supreme (më strikt se Quantum)
// ─────────────────────────────────────────────────────────────────────────────

/// FilterGate në karakterin e Shadow: "asnjë promovim pa SHADOW_FILTER_GATE".
/// Mirror i FilterGate të Quantum (produkt boolean) + rëndues suprem TRL.
pub struct ShadowFilterGate;

impl ShadowFilterGate {
    /// check — invariantët supreme që duhen para çdo promovimi me TRL.
    ///
    /// measurable AND reproducible AND confidence_valid AND trl_sufficient.
    /// Më strikt se Quantum: kërkon EDHE trl_level >= SHADOW_MIN_TRL.
    /// Zero if — produkt boolean.
    pub fn check(input: &TrlInput) -> bool {
        let measurable    = input.measurable as u32;
        let reproducible  = input.reproducible as u32;
        let conf_valid    = ((0.0..=1.0).contains(&input.confidence)) as u32;
        let repro_valid   = ((0.0..=1.0).contains(&input.reproducibility)) as u32;
        let trl_ok        = (input.trl_level >= SHADOW_MIN_TRL) as u32;

        // Të GJITHA duhet 1 (produkt boolean — invariant suprem).
        (measurable * reproducible * conf_valid * repro_valid * trl_ok) == 1
    }

    /// gate_reason — pse refuzoi porta supreme (informativ, për DSL).
    pub fn gate_reason(input: &TrlInput) -> &'static str {
        let not_measurable   = !input.measurable;
        let not_reproducible = !input.reproducible;
        let trl_low          = input.trl_level < SHADOW_MIN_TRL;
        // Prioritet: invarianti i parë i thyer.
        match (not_measurable, not_reproducible, trl_low) {
            (true, _, _) => "jo measurable (shadow gate)",
            (_, true, _) => "jo reproducible (shadow gate)",
            (_, _, true) => "trl < 3 (shadow gate)",
            _            => "OK",
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// TRL VERDICT — gjykimi suprem TRL që RËNDON vendimin
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TrlRejection {
    None,             // kaloi
    FilterGate,       // porta supreme refuzoi
    ScoreLow,         // confidence < threshold 0.70
    PrerequisiteFail, // trl_passed == false
}

impl TrlRejection {
    pub fn label(self) -> &'static str {
        match self {
            TrlRejection::None             => "TRL_NONE",
            TrlRejection::FilterGate       => "TRL_FILTER_GATE",
            TrlRejection::ScoreLow         => "TRL_SCORE_LOW",
            TrlRejection::PrerequisiteFail => "TRL_PREREQUISITE_FAIL",
        }
    }
}

/// Verdikti TRL i Shadow-it — a e mbështet TRL-ja vendimin suprem?
#[derive(Debug, Clone, Copy)]
pub struct TrlVerdict {
    pub trl_supports:    bool,         // a e mbështet TRL promovimin
    pub trl_score:       f32,          // [0,1] forca e mbështetjes TRL
    pub gate_passed:     bool,         // a kaloi SHADOW_FILTER_GATE
    pub rejection:       TrlRejection, // arsyeja nëse s'mbështet
}

impl TrlVerdict {
    /// judge — Shadow gjykon TRL-në me autoritetin suprem.
    ///
    /// trl_supports = gate_passed AND score_ok AND prerequisite_ok.
    /// Prioritet refuzimi (match): gate → prerequisite → threshold.
    /// Zero if — produkt boolean + match shterues.
    pub fn judge(input: &TrlInput) -> Self {
        let gate_passed = ShadowFilterGate::check(input);
        // score_ok: confidence mbi threshold-in TRL3.
        let score_ok = input.confidence >= SHADOW_THRESHOLD_TRL3;
        // prerequisite: TRL3 duhet të jetë kaluar realisht.
        let prereq_ok = input.trl_passed;

        // Vendimi + arsyeja (prioritet: gate > prerequisite > score).
        let (supports, rejection) = match (gate_passed, prereq_ok, score_ok) {
            (false, _, _) => (false, TrlRejection::FilterGate),
            (true, false, _) => (false, TrlRejection::PrerequisiteFail),
            (true, true, false) => (false, TrlRejection::ScoreLow),
            (true, true, true) => (true, TrlRejection::None),
        };

        // trl_score: forca e mbështetjes (confidence × reproducibility × support).
        let support_factor = supports as u32 as f32;
        let trl_score = (support_factor
            * input.confidence
            * input.reproducibility)
            .clamp(0.0, 1.0);

        Self {
            trl_supports: supports,
            trl_score,
            gate_passed,
            rejection,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// SHADOW LAB — gjykimi suprem që RËNDON vendimin 0/1
// ─────────────────────────────────────────────────────────────────────────────

/// Rezultati i shkrirjes TRL me vendimin suprem.
/// TRL e bën vendimin më të rëndë: edhe nëse judiciary+vula kalojnë, banda
/// e dijes kufizohet nga pjekuria TRL.
#[derive(Debug, Clone, Copy)]
pub struct ShadowLabOutcome {
    pub trl_verdict:     TrlVerdict,
    pub band_ceiling:    KnowledgeBand,  // banda maksimale që TRL lejon
    pub weight_factor:   f32,            // rëndues [0,1] për legacy_score
}

pub struct ShadowLab;

impl ShadowLab {
    /// adjudicate — Shadow gjykon me TRL dhe RËNDON vendimin.
    ///
    /// Prodhon:
    ///  • band_ceiling: banda maksimale e dijes që pjekuria TRL lejon.
    ///  • weight_factor: rëndues që modifikon legacy_score (TRL i fortë → më i rëndë).
    /// Zero if — match/formula.
    pub fn adjudicate(input: &TrlInput) -> ShadowLabOutcome {
        let verdict = TrlVerdict::judge(input);

        // band_ceiling: pa mbështetje TRL → maksimumi Hypothesis (s'arrin Scientific).
        // Me mbështetje + TRL >= SHADOW_SCIENTIFIC_TRL → lejon Scientific.
        let trl_high = (input.trl_level >= SHADOW_SCIENTIFIC_TRL) as u32;
        let supports = verdict.trl_supports as u32;
        // idx: 0 → Hypothesis (ceiling i ulët); 1 → Scientific (ceiling i lartë).
        let scientific_allowed = supports * trl_high;
        let band_ceiling = match scientific_allowed {
            1 => KnowledgeBand::Scientific,
            _ => KnowledgeBand::Hypothesis,
        };

        // weight_factor: TRL i fortë e bën vendimin më të rëndë.
        // Pa mbështetje → 0.5 (neutral, s'ndihmon); me mbështetje → shkallëzohet me trl_score.
        let base = 0.5f32;
        let bonus = verdict.trl_supports as u32 as f32 * verdict.trl_score * 0.5;
        let weight_factor = (base + bonus).clamp(0.0, 1.0);

        ShadowLabOutcome {
            trl_verdict: verdict,
            band_ceiling,
            weight_factor,
        }
    }

    /// weigh_legacy_score — RËNDON legacy_score ekzistues me gjykimin TRL.
    ///
    /// Shadow e bën vendimin më të rëndë: legacy_score modifikohet nga sa
    /// fort e mbështet TRL-ja. TRL i dobët → ul peshën; TRL i fortë → ruan/rrit.
    /// Zero if — shumëzim me weight_factor.
    pub fn weigh_legacy_score(base_score: f32, outcome: &ShadowLabOutcome) -> f32 {
        // Vendimi më i rëndë: score × (0.5 + 0.5×weight) → TRL modulon final-in.
        let modulation = 0.5 + 0.5 * outcome.weight_factor;
        (base_score * modulation).clamp(0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strong_trl() -> TrlInput {
        TrlInput {
            trl_level: 3,
            trl_passed: true,
            confidence: 0.85,
            reproducibility: 0.9,
            measurable: true,
            reproducible: true,
        }
    }

    fn weak_trl() -> TrlInput {
        TrlInput {
            trl_level: 1,
            trl_passed: false,
            confidence: 0.3,
            reproducibility: 0.2,
            measurable: false,
            reproducible: false,
        }
    }

    #[test]
    fn threshold_matches_quantum() {
        // IDENTIK me Quantum: THRESHOLD_TRL3 = 0.70.
        assert_eq!(SHADOW_THRESHOLD_TRL3, 0.70);
    }

    #[test]
    fn filter_gate_passes_strong() {
        assert!(ShadowFilterGate::check(&strong_trl()));
    }

    #[test]
    fn filter_gate_blocks_weak() {
        assert!(!ShadowFilterGate::check(&weak_trl()));
    }

    #[test]
    fn filter_gate_blocks_low_trl() {
        // TRL 2 < SHADOW_MIN_TRL (3) → refuzohet edhe me masat e tjera OK.
        let mut input = strong_trl();
        input.trl_level = 2;
        assert!(!ShadowFilterGate::check(&input));
        assert_eq!(ShadowFilterGate::gate_reason(&input), "trl < 3 (shadow gate)");
    }

    #[test]
    fn trl_verdict_supports_strong() {
        let v = TrlVerdict::judge(&strong_trl());
        assert!(v.trl_supports);
        assert!(v.gate_passed);
        assert_eq!(v.rejection, TrlRejection::None);
        assert!(v.trl_score > 0.7);
    }

    #[test]
    fn trl_verdict_rejects_weak_at_gate() {
        let v = TrlVerdict::judge(&weak_trl());
        assert!(!v.trl_supports);
        assert_eq!(v.rejection, TrlRejection::FilterGate);
        assert_eq!(v.trl_score, 0.0);
    }

    #[test]
    fn trl_verdict_prerequisite_fail() {
        // Gate kalon (measurable/repro/trl OK) por trl_passed == false.
        let mut input = strong_trl();
        input.trl_passed = false;
        let v = TrlVerdict::judge(&input);
        assert!(!v.trl_supports);
        assert_eq!(v.rejection, TrlRejection::PrerequisiteFail);
    }

    #[test]
    fn trl_verdict_score_low() {
        // Gate + prerequisite OK por confidence < 0.70.
        let mut input = strong_trl();
        input.confidence = 0.5;
        let v = TrlVerdict::judge(&input);
        assert!(!v.trl_supports);
        assert_eq!(v.rejection, TrlRejection::ScoreLow);
    }

    #[test]
    fn adjudicate_strong_allows_scientific() {
        let outcome = ShadowLab::adjudicate(&strong_trl());
        assert!(outcome.trl_verdict.trl_supports);
        // TRL i fortë → lejon bandën Scientific.
        assert_eq!(outcome.band_ceiling, KnowledgeBand::Scientific);
        assert!(outcome.weight_factor > 0.5);
    }

    #[test]
    fn adjudicate_weak_caps_at_hypothesis() {
        let outcome = ShadowLab::adjudicate(&weak_trl());
        assert!(!outcome.trl_verdict.trl_supports);
        // TRL i dobët → maksimumi Hypothesis (s'arrin Scientific).
        assert_eq!(outcome.band_ceiling, KnowledgeBand::Hypothesis);
        // weight_factor neutral (0.5) — s'ndihmon promovimin.
        assert!((outcome.weight_factor - 0.5).abs() < 1e-6);
    }

    #[test]
    fn weigh_makes_decision_heavier() {
        // TRL i fortë RUAN më shumë nga score-i; TRL i dobët e ul.
        let strong = ShadowLab::adjudicate(&strong_trl());
        let weak = ShadowLab::adjudicate(&weak_trl());

        let base = 0.8f32;
        let strong_weighted = ShadowLab::weigh_legacy_score(base, &strong);
        let weak_weighted = ShadowLab::weigh_legacy_score(base, &weak);

        // TRL i fortë → score më i lartë se TRL i dobët (vendim më i rëndë).
        assert!(strong_weighted > weak_weighted);
        // TRL i dobët e ul score-in (s'mbështet).
        assert!(weak_weighted < base);
    }

    #[test]
    fn pending_trl_does_not_support() {
        let v = TrlVerdict::judge(&TrlInput::pending());
        assert!(!v.trl_supports);
        // Pending → gate dështon (s'ka asgjë measurable).
        assert_eq!(v.rejection, TrlRejection::FilterGate);
    }
}
