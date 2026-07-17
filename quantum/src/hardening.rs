// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  HARDENING.RS — Patch Përforcues mbi Quantum                          ║
// ║  GJATA LEGACY™ — Quantum Platform                                 ║
// ║                                                                          ║
// ║  E bën Quantum-in MË TË FORTË dhe MË TË SAKTË përmes:                  ║
// ║                                                                          ║
// ║   1. CrossOperatorConsensus — konsensus ndër 4 operatorët             ║
// ║      (saktësi: kandidatë me dakordësi të lartë ngrihen)              ║
// ║   2. ConfidenceCalibration — redukton mbi-sigurinë                    ║
// ║      (saktësi: score afër 1.0 me pak evidencë ulet)                  ║
// ║   3. ProNproConflict — detekton konflikt PRO↔NPRO                     ║
// ║      (forcë: nëse PRO pranon ç'ka NPRO eliminon → flag)             ║
// ║   4. StabilityMargin — kontroll i qëndrueshmërisë së score-ve         ║
// ║      (forcë: varianca e lartë → besim i ulët)                        ║
// ║                                                                          ║
// ║  Aplikohet PAS eliminimit, PARA convergence. Quantum s'vendos —        ║
// ║  vetëm përmirëson saktësinë e sinjalit që shkon te Shadow.            ║
// ║  Zero if/else — formula matematike.                                   ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::sovereign::laws::gate;
use crate::reasoning::elimination::ElimCandidate;
use crate::npro::WeaknessSignal;

// ─────────────────────────────────────────────────────────────────────────────
// 1. CROSS-OPERATOR CONSENSUS — saktësi përmes dakordësisë
// ─────────────────────────────────────────────────────────────────────────────

pub struct CrossOperatorConsensus;

impl CrossOperatorConsensus {
    /// consensus_boost — kandidatë me shumë operatorë dakord ngrihen.
    ///
    /// operator_scores: score nga secili operator që e mbështeti kandidatin.
    /// Sa më shumë operatorë me score të lartë → boost; pak → pa boost.
    /// Zero if — formula mbi numrin e operatorëve dakord.
    pub fn consensus_boost(base_score: f32, operator_scores: &[f32]) -> f32 {
        let n = operator_scores.len().max(1) as f32;
        // Sa operatorë janë "dakord" (score > 0.5).
        let agreeing = operator_scores.iter().filter(|s| **s > 0.5).count() as f32;
        let agreement_ratio = agreeing / n;

        // Boost proporcional me dakordësinë (max +0.15 kur të gjithë dakord).
        let boost = agreement_ratio * 0.15;
        gate(base_score + boost, 0.0, 1.0)
    }

    /// agreement_strength — sa të afërta janë score-t (1=identike, 0=larg).
    pub fn agreement_strength(operator_scores: &[f32]) -> f32 {
        let n = operator_scores.len().max(1) as f32;
        let mean = operator_scores.iter().sum::<f32>() / n;
        let variance = operator_scores.iter()
            .map(|s| (s - mean).powi(2))
            .sum::<f32>() / n;
        // Dakordësi e lartë = variancë e ulët. strength = 1 - sqrt(variance).
        (1.0 - variance.sqrt()).clamp(0.0, 1.0)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 2. CONFIDENCE CALIBRATION — redukton mbi-sigurinë
// ─────────────────────────────────────────────────────────────────────────────

pub struct ConfidenceCalibration;

impl ConfidenceCalibration {
    /// calibrate — ul score-t tepër të larta me pak evidencë (anti-hallucination).
    ///
    /// score afër 1.0 POR evidence_count i ulët → ulet drejt një vlere reale.
    /// Kjo redukton mbi-sigurinë (problemi klasik i sistemeve gjeneruese).
    /// Zero if — formula me penalty proporcional.
    pub fn calibrate(raw_score: f32, evidence_count: u32) -> f32 {
        // Faktori i evidencës: 0 evidencë → 0.5; shumë → 1.0 (saturim).
        // evidence_factor = 1 - exp(-count/3) afërsisht, por branchless me clamp.
        let ev = evidence_count as f32;
        let evidence_factor = (ev / (ev + 3.0)).clamp(0.0, 1.0);

        // Sa "e rrezikshme" është mbi-siguria: score i lartë × pak evidencë.
        let overconfidence_risk = raw_score * (1.0 - evidence_factor);

        // Penalty proporcional me rrezikun (max ul ~0.30).
        let penalty = overconfidence_risk * 0.30;
        gate(raw_score - penalty, 0.0, 1.0)
    }

    /// is_overconfident — flag kur score > 0.90 me < 2 evidenca (informativ).
    pub fn overconfidence_level(raw_score: f32, evidence_count: u32) -> f32 {
        // high_score ∈ {0,1}, low_evidence ∈ {0,1}.
        let high_score = (raw_score > 0.90) as u32 as f32;
        let low_evidence = (evidence_count < 2) as u32 as f32;
        high_score * low_evidence
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 3. PRO↔NPRO CONFLICT — forcë përmes kontrollit të dyfishtë
// ─────────────────────────────────────────────────────────────────────────────

pub struct PronoConflict;

impl PronoConflict {
    /// conflict_penalty — nëse PRO pranon një rrugë që NPRO e ka eliminuar.
    ///
    /// Kjo është forcë: dy sisteme të pavarura (PRO pozitiv, NPRO negativ)
    /// duhet të pajtohen. Konflikti → penalty (besim i ulët në kandidat).
    /// Zero if — formula mbi forbidden_paths.
    pub fn conflict_penalty(candidate: &ElimCandidate, weakness: &WeaknessSignal) -> f32 {
        // A është provider-i i kandidatit në rrugët e ndaluara nga NPRO?
        let in_forbidden = weakness.forbidden_paths.iter()
            .any(|p| candidate.provider.contains(p)) as u32 as f32;

        // Konflikt → penalty 0.40 (ul ndjeshëm besimin); pa konflikt → 0.
        in_forbidden * 0.40
    }

    /// apply_conflict_check — ul score-t e kandidatëve në konflikt me NPRO.
    /// Zero if — map me formula.
    pub fn apply(candidates: Vec<ElimCandidate>, weakness: &WeaknessSignal) -> Vec<ElimCandidate> {
        candidates.into_iter()
            .map(|mut c| {
                let penalty = Self::conflict_penalty(&c, weakness);
                c.quantum_score = gate(c.quantum_score * (1.0 - penalty), 0.0, 1.0);
                c
            })
            .collect()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 4. STABILITY MARGIN — forcë përmes kontrollit të qëndrueshmërisë
// ─────────────────────────────────────────────────────────────────────────────

pub struct StabilityMargin;

impl StabilityMargin {
    /// stability_factor — varianca e ulët mes score-ve = qëndrueshmëri e lartë.
    ///
    /// Nëse kandidatët kanë score shumë të ndryshme → sistemi s'është i sigurt
    /// → besim i ulët në rezultatin final.
    /// Zero if — formula statistikore.
    pub fn stability_factor(scores: &[f32]) -> f32 {
        let n = scores.len().max(1) as f32;
        let mean = scores.iter().sum::<f32>() / n;
        let variance = scores.iter()
            .map(|s| (s - mean).powi(2))
            .sum::<f32>() / n;
        // Qëndrueshmëri = 1 - 2*sqrt(variance), clamp.
        (1.0 - 2.0 * variance.sqrt()).clamp(0.0, 1.0)
    }

    /// apply_stability — modulon score final me faktorin e qëndrueshmërisë.
    pub fn apply_stability(final_score: f32, all_scores: &[f32]) -> f32 {
        let stability = Self::stability_factor(all_scores);
        // Përziej: 70% score original + 30% i moduluar nga qëndrueshmëria.
        gate(final_score * (0.70 + 0.30 * stability), 0.0, 1.0)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// HARDENING PIPELINE — aplikon të 4 përforcimet në rend
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct HardeningReport {
    pub candidates_strengthened: usize,
    pub avg_score_before:        f32,
    pub avg_score_after:         f32,
    pub stability:               f32,
    pub conflicts_detected:      usize,
}

pub struct Hardening;

impl Hardening {
    /// strengthen — aplikon të 4 përforcimet mbi kandidatët.
    ///
    /// Rendi: conflict check → calibration → consensus → stability.
    /// Rezultat: kandidatë me score më të saktë e më të qëndrueshëm.
    pub fn strengthen(
        candidates: Vec<ElimCandidate>,
        weakness: &WeaknessSignal,
        evidence_count: u32,
    ) -> (Vec<ElimCandidate>, HardeningReport) {
        let count_before = candidates.len();
        let scores_before: Vec<f32> = candidates.iter().map(|c| c.quantum_score).collect();
        let avg_before = Self::mean(&scores_before);

        // Numëro konfliktet PRO↔NPRO para aplikimit.
        let conflicts = candidates.iter()
            .filter(|c| PronoConflict::conflict_penalty(c, weakness) > 0.0)
            .count();

        // 1. PRO↔NPRO conflict check.
        let after_conflict = PronoConflict::apply(candidates, weakness);

        // 2. Confidence calibration (anti mbi-siguri) + 3. consensus.
        let strengthened: Vec<ElimCandidate> = after_conflict.into_iter()
            .map(|mut c| {
                // Kalibrim besimi.
                let calibrated = ConfidenceCalibration::calibrate(c.quantum_score, evidence_count);
                // Konsensus (vetë-score si proxy për dakordësi minimale).
                let with_consensus = CrossOperatorConsensus::consensus_boost(
                    calibrated, &[c.quantum_score, calibrated]
                );
                c.quantum_score = with_consensus;
                c
            })
            .collect();

        // 4. Stability margin mbi të gjithë.
        let scores_after: Vec<f32> = strengthened.iter().map(|c| c.quantum_score).collect();
        let stability = StabilityMargin::stability_factor(&scores_after);
        let avg_after = Self::mean(&scores_after);

        let report = HardeningReport {
            candidates_strengthened: count_before,
            avg_score_before: avg_before,
            avg_score_after: avg_after,
            stability,
            conflicts_detected: conflicts,
        };

        (strengthened, report)
    }

    fn mean(xs: &[f32]) -> f32 {
        let is_empty = xs.is_empty() as u32 as f32;
        let sum = xs.iter().sum::<f32>();
        (1.0 - is_empty) * (sum / xs.len().max(1) as f32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cand(provider: &str, score: f32) -> ElimCandidate {
        ElimCandidate {
            provider: provider.to_string(),
            provider_domain: "science".to_string(),
            proposal_type: "t1".to_string(),
            quantum_score: score,
            has_proposal: true,
            has_signature: true,
        }
    }

    fn empty_weakness() -> WeaknessSignal {
        WeaknessSignal { forbidden_paths: vec![], weakness_ceiling: 0.65, avg_weakness: 0.2 }
    }

    #[test]
    fn consensus_boosts_agreement() {
        // Të gjithë dakord (score > 0.5) → boost.
        let boosted = CrossOperatorConsensus::consensus_boost(0.6, &[0.7, 0.8, 0.9, 0.6]);
        assert!(boosted > 0.6);
        // Asnjë dakord → pa boost.
        let no_boost = CrossOperatorConsensus::consensus_boost(0.4, &[0.1, 0.2, 0.3]);
        assert!((no_boost - 0.4).abs() < 1e-5);
    }

    #[test]
    fn agreement_strength_high_when_similar() {
        // Score identike → strength e lartë.
        let high = CrossOperatorConsensus::agreement_strength(&[0.8, 0.8, 0.8]);
        assert!(high > 0.9);
        // Score të ndryshme → strength e ulët.
        let low = CrossOperatorConsensus::agreement_strength(&[0.1, 0.9, 0.5]);
        assert!(low < high);
    }

    #[test]
    fn calibration_reduces_overconfidence() {
        // Score i lartë me pak evidencë → ulet.
        let calibrated = ConfidenceCalibration::calibrate(0.95, 0);
        assert!(calibrated < 0.95);
        // Score i lartë me shumë evidencë → mbahet afër.
        let stable = ConfidenceCalibration::calibrate(0.95, 20);
        assert!(stable > calibrated);
    }

    #[test]
    fn overconfidence_flagged() {
        // score > 0.90 me < 2 evidenca → flag 1.0.
        assert_eq!(ConfidenceCalibration::overconfidence_level(0.95, 1), 1.0);
        // score normal → 0.
        assert_eq!(ConfidenceCalibration::overconfidence_level(0.7, 5), 0.0);
    }

    #[test]
    fn conflict_penalizes_forbidden() {
        let weakness = WeaknessSignal {
            forbidden_paths: vec!["REVERSE".to_string()],
            weakness_ceiling: 0.65, avg_weakness: 0.7,
        };
        // Provider që përmban "REVERSE" → penalty.
        let c = cand("REVERSE_op", 0.8);
        let penalty = PronoConflict::conflict_penalty(&c, &weakness);
        assert!(penalty > 0.0);
        // Provider tjetër → pa penalty.
        let safe = cand("MIRROR_op", 0.8);
        assert_eq!(PronoConflict::conflict_penalty(&safe, &weakness), 0.0);
    }

    #[test]
    fn stability_high_when_consistent() {
        // Score të ngjashme → qëndrueshmëri e lartë.
        let stable = StabilityMargin::stability_factor(&[0.7, 0.72, 0.68]);
        assert!(stable > 0.7);
        // Score të shpërndara → qëndrueshmëri e ulët.
        let unstable = StabilityMargin::stability_factor(&[0.1, 0.9, 0.5]);
        assert!(unstable < stable);
    }

    #[test]
    fn strengthen_full_pipeline() {
        let candidates = vec![cand("PRO_A", 0.9), cand("PRO_B", 0.7)];
        let (strengthened, report) = Hardening::strengthen(candidates, &empty_weakness(), 5);
        assert_eq!(strengthened.len(), 2);
        assert_eq!(report.candidates_strengthened, 2);
        assert!((0.0..=1.0).contains(&report.stability));
    }

    #[test]
    fn strengthen_detects_conflicts() {
        let weakness = WeaknessSignal {
            forbidden_paths: vec!["BAD".to_string()],
            weakness_ceiling: 0.65, avg_weakness: 0.7,
        };
        let candidates = vec![cand("BAD_provider", 0.9), cand("GOOD_provider", 0.8)];
        let (_strengthened, report) = Hardening::strengthen(candidates, &weakness, 5);
        // Një konflikt u detektua (BAD_provider).
        assert_eq!(report.conflicts_detected, 1);
    }

    #[test]
    fn calibration_anti_hallucination() {
        // Demonstrim: mbi-siguria pa evidencë ulet ndjeshëm.
        let hallucinated = ConfidenceCalibration::calibrate(1.0, 0);
        // 1.0 me 0 evidencë → duhet të ulet dukshëm (anti-hallucination).
        assert!(hallucinated < 0.80);
    }
}
