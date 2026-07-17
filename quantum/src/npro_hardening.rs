// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  NPRO_HARDENING.RS — Forcimi i NPRO + NPIM                            ║
// ║  GJATA LEGACY™ — Quantum Platform                                 ║
// ║                                                                          ║
// ║  E bën rrugën negative MË TË FORTË dhe MË TË SAKTË:                    ║
// ║                                                                          ║
// ║  NPRO:                                                                  ║
// ║   1. WeaknessConsensus — shumë operatorë dakord → eliminim më i sigurt║
// ║   2. AdaptiveCeiling — ceiling përshtatet sipas dobësisë mesatare     ║
// ║                                                                          ║
// ║  NPIM:                                                                  ║
// ║   3. EvidenceWeighting — eliminime me më shumë mbështetje peshohen më ║
// ║   4. SeverityEscalation — weakness ekstreme (>0.90) → peshë shtesë    ║
// ║                                                                          ║
// ║  Rezultat: forbidden_paths më të sakta, negativity_score më i fortë.   ║
// ║  Zero if/else — formula matematike.                                   ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::sovereign::laws::gate;
use crate::npro::{NproResult, NegativeElimination, WeaknessSignal};
use crate::npim::NegativityProfile;

// ─────────────────────────────────────────────────────────────────────────────
// 1. WEAKNESS CONSENSUS (NPRO) — dakordësi ndër operatorë
// ─────────────────────────────────────────────────────────────────────────────

pub struct WeaknessConsensus;

impl WeaknessConsensus {
    /// consensus_strength — sa operatorë bien dakord se ka dobësi.
    ///
    /// Kur shumë operatorë mbrapsht (REVERSE⁻¹, MIRROR⁻¹, ...) bien dakord
    /// se një rrugë është e dobët, besimi në eliminim rritet.
    /// Zero if — formula mbi numrin e operatorëve dakord.
    pub fn consensus_strength(eliminations: &[NegativeElimination]) -> f32 {
        let n = eliminations.len().max(1) as f32;
        // Sa eliminime kanë weakness të lartë (> 0.5).
        let agreeing = eliminations.iter()
            .filter(|e| e.weakness_score > 0.5)
            .count() as f32;
        (agreeing / n).clamp(0.0, 1.0)
    }

    /// boost_weakness — rrit weakness-in kur ka konsensus.
    /// Eliminime me dakordësi të lartë → weakness i konfirmuar.
    pub fn boost_weakness(base_weakness: f32, consensus: f32) -> f32 {
        // Boost proporcional me konsensusin (max +0.20 kur konsensus i plotë).
        let boost = consensus * 0.20;
        gate(base_weakness + boost, 0.0, 1.0)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 2. ADAPTIVE CEILING (NPRO) — ceiling dinamik
// ─────────────────────────────────────────────────────────────────────────────

pub struct AdaptiveCeiling;

impl AdaptiveCeiling {
    /// compute_ceiling — ceiling përshtatet sipas dobësisë mesatare.
    ///
    /// Kur dobësia mesatare është e lartë, ceiling-u ulet (më shumë rrugë
    /// ndalohen). Kur është e ulët, ceiling-u ngrihet (më pak ndalohen).
    /// Bazë 0.65, përshtatet ±0.10.
    /// Zero if — formula lineare.
    pub fn compute_ceiling(avg_weakness: f32) -> f32 {
        // Sa më i lartë avg_weakness, aq më i ulët ceiling (më rreptë).
        // ceiling = 0.65 - (avg_weakness - 0.5) * 0.20, clamp [0.55, 0.75].
        let adjustment = (avg_weakness - 0.5) * 0.20;
        (0.65 - adjustment).clamp(0.55, 0.75)
    }

    /// recompute_forbidden — rindërton forbidden_paths me ceiling adaptiv.
    pub fn recompute_forbidden(
        eliminations: &[NegativeElimination],
        adaptive_ceiling: f32,
    ) -> Vec<String> {
        eliminations.iter()
            .filter(|e| e.weakness_score > adaptive_ceiling)
            .map(|e| e.operator.clone())
            .collect()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 3. EVIDENCE WEIGHTING (NPIM) — peshim sipas mbështetjes
// ─────────────────────────────────────────────────────────────────────────────

pub struct EvidenceWeighting;

impl EvidenceWeighting {
    /// weighted_failure_density — peshon eliminimet sipas fragment_refs.
    ///
    /// Eliminime me më shumë fragment references (më shumë evidencë) peshohen
    /// më rëndë në failure_density.
    /// Zero if — formula me peshim.
    pub fn weighted_failure_density(eliminations: &[NegativeElimination]) -> f32 {
        let n = eliminations.len().max(1) as f32;

        // Shuma e peshuar: weakness × (1 + ref_count_factor).
        let weighted_sum: f32 = eliminations.iter()
            .map(|e| {
                let ref_factor = (e.fragment_refs.len() as f32 / 4.0).clamp(0.0, 1.0);
                let high = (e.weakness_score > 0.5) as u32 as f32;
                high * (1.0 + ref_factor * 0.5)
            })
            .sum();

        // Normalizo me peshën maksimale të mundshme (n × 1.5).
        (weighted_sum / (n * 1.5)).clamp(0.0, 1.0)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 4. SEVERITY ESCALATION (NPIM) — weakness ekstreme
// ─────────────────────────────────────────────────────────────────────────────

pub struct SeverityEscalation;

impl SeverityEscalation {
    /// severity_factor — eliminime me weakness ekstreme marrin peshë shtesë.
    ///
    /// weakness > 0.90 → "kritike" (dështim i sigurt). Kjo ngre besimin që
    /// kjo është vërtet negative knowledge.
    /// Zero if — formula mbi numrin e kritikeve.
    pub fn severity_factor(eliminations: &[NegativeElimination]) -> f32 {
        let n = eliminations.len().max(1) as f32;
        // Sa eliminime janë kritike (weakness > 0.90).
        let critical = eliminations.iter()
            .filter(|e| e.weakness_score > 0.90)
            .count() as f32;
        (critical / n).clamp(0.0, 1.0)
    }

    /// escalate_negativity — ngre negativity_score me severitetin.
    pub fn escalate_negativity(base_negativity: f32, severity: f32) -> f32 {
        // Escalation proporcional (max +0.15 kur të gjitha kritike).
        let escalation = severity * 0.15;
        gate(base_negativity + escalation, 0.0, 1.0)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// NPRO HARDENING — aplikon forcimet mbi NproResult
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct NproHardeningReport {
    pub consensus_strength:  f32,
    pub adaptive_ceiling:    f32,
    pub original_ceiling:    f32,
    pub forbidden_before:    usize,
    pub forbidden_after:     usize,
}

pub struct NproHardening;

impl NproHardening {
    /// strengthen_npro — forcon NproResult me konsensus + ceiling adaptiv.
    ///
    /// 1. Llogarit konsensusin → boost weakness te çdo eliminim
    /// 2. Llogarit ceiling adaptiv → rindërto forbidden_paths
    pub fn strengthen_npro(mut npro: NproResult) -> (NproResult, NproHardeningReport) {
        let consensus = WeaknessConsensus::consensus_strength(&npro.eliminations);
        let original_ceiling = npro.weakness_signal.weakness_ceiling;
        let forbidden_before = npro.weakness_signal.forbidden_paths.len();

        // 1. Boost weakness me konsensus.
        let boosted: Vec<NegativeElimination> = npro.eliminations.into_iter()
            .map(|mut e| {
                e.weakness_score = WeaknessConsensus::boost_weakness(e.weakness_score, consensus);
                e
            })
            .collect();

        // 2. Ceiling adaptiv nga dobësia mesatare e re.
        let sum: f32 = boosted.iter().map(|e| e.weakness_score).sum();
        let avg_weakness = sum / boosted.len().max(1) as f32;
        let adaptive_ceiling = AdaptiveCeiling::compute_ceiling(avg_weakness);

        // Rindërto forbidden_paths.
        let forbidden = AdaptiveCeiling::recompute_forbidden(&boosted, adaptive_ceiling);
        let forbidden_after = forbidden.len();

        npro.eliminations = boosted;
        npro.weakness_signal = WeaknessSignal {
            forbidden_paths: forbidden,
            weakness_ceiling: adaptive_ceiling,
            avg_weakness,
        };

        let report = NproHardeningReport {
            consensus_strength: consensus,
            adaptive_ceiling,
            original_ceiling,
            forbidden_before,
            forbidden_after,
        };

        (npro, report)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// NPIM HARDENING — forcon NegativityProfile
// ─────────────────────────────────────────────────────────────────────────────

pub struct NpimHardening;

impl NpimHardening {
    /// strengthen_negativity — forcon profilin me evidence weighting + severity.
    ///
    /// 1. failure_density → i peshuar sipas fragment_refs
    /// 2. negativity_score → eskaluar me severitetin (weakness ekstreme)
    pub fn strengthen_negativity(
        mut profile: NegativityProfile,
        eliminations: &[NegativeElimination],
    ) -> NegativityProfile {
        // 1. Evidence-weighted failure density.
        let weighted_fd = EvidenceWeighting::weighted_failure_density(eliminations);
        // Përziej: 60% i peshuar + 40% origjinal.
        profile.failure_density = gate(
            weighted_fd * 0.6 + profile.failure_density * 0.4, 0.0, 1.0
        );

        // 2. Severity escalation.
        let severity = SeverityEscalation::severity_factor(eliminations);
        let base_neg = profile.negativity_score;
        profile.negativity_score = SeverityEscalation::escalate_negativity(base_neg, severity);

        profile
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::npro::NproEngine;

    fn elim(operator: &str, weakness: f32, refs: usize) -> NegativeElimination {
        NegativeElimination {
            elimination_id: format!("E_{}", operator),
            operator: operator.to_string(),
            weakness_score: weakness,
            cause: "test".to_string(),
            fragment_refs: vec![0; refs],
        }
    }

    #[test]
    fn consensus_high_when_all_weak() {
        let elims = vec![elim("A", 0.8, 1), elim("B", 0.7, 1), elim("C", 0.9, 1)];
        let consensus = WeaknessConsensus::consensus_strength(&elims);
        assert!(consensus > 0.9); // të gjithë dakord
    }

    #[test]
    fn consensus_low_when_mixed() {
        let elims = vec![elim("A", 0.8, 1), elim("B", 0.2, 1), elim("C", 0.1, 1)];
        let consensus = WeaknessConsensus::consensus_strength(&elims);
        assert!(consensus < 0.5);
    }

    #[test]
    fn boost_increases_with_consensus() {
        let boosted = WeaknessConsensus::boost_weakness(0.6, 1.0);
        assert!(boosted > 0.6);
        let no_boost = WeaknessConsensus::boost_weakness(0.6, 0.0);
        assert!((no_boost - 0.6).abs() < 1e-5);
    }

    #[test]
    fn adaptive_ceiling_lowers_with_high_weakness() {
        // Dobësi e lartë → ceiling i ulët (më rreptë).
        let strict = AdaptiveCeiling::compute_ceiling(0.9);
        assert!(strict < 0.65);
        // Dobësi e ulët → ceiling i lartë.
        let lenient = AdaptiveCeiling::compute_ceiling(0.2);
        assert!(lenient > 0.65);
    }

    #[test]
    fn ceiling_stays_in_bounds() {
        // Edhe ekstreme → clamp [0.55, 0.75].
        assert!(AdaptiveCeiling::compute_ceiling(1.0) >= 0.55);
        assert!(AdaptiveCeiling::compute_ceiling(0.0) <= 0.75);
    }

    #[test]
    fn evidence_weighting_favors_more_refs() {
        // Eliminim me shumë refs peshohet më rëndë.
        let many_refs = vec![elim("A", 0.8, 4)];
        let few_refs = vec![elim("A", 0.8, 0)];
        let wd_many = EvidenceWeighting::weighted_failure_density(&many_refs);
        let wd_few = EvidenceWeighting::weighted_failure_density(&few_refs);
        assert!(wd_many > wd_few);
    }

    #[test]
    fn severity_detects_critical() {
        // Weakness ekstreme (>0.90) → severitet.
        let critical = vec![elim("A", 0.95, 1), elim("B", 0.92, 1)];
        let severity = SeverityEscalation::severity_factor(&critical);
        assert!(severity > 0.9);
        // Weakness normale → pa severitet.
        let normal = vec![elim("A", 0.6, 1)];
        assert_eq!(SeverityEscalation::severity_factor(&normal), 0.0);
    }

    #[test]
    fn escalation_raises_negativity() {
        let escalated = SeverityEscalation::escalate_negativity(0.7, 1.0);
        assert!(escalated > 0.7);
    }

    #[test]
    fn strengthen_npro_full() {
        let frags = vec!["single weak fragment".to_string()];
        let npro = NproEngine::activate(&frags, 0.3, 0.3);
        let (strengthened, report) = NproHardening::strengthen_npro(npro);
        // Konsensusi u llogarit.
        assert!((0.0..=1.0).contains(&report.consensus_strength));
        // Ceiling adaptiv në kufij.
        assert!(report.adaptive_ceiling >= 0.55 && report.adaptive_ceiling <= 0.75);
        // Eliminimet ekzistojnë ende.
        assert_eq!(strengthened.eliminations.len(), 4);
    }

    #[test]
    fn strengthen_negativity_full() {
        let profile = NegativityProfile {
            failure_density: 0.5,
            contradiction_strength: 0.6,
            elimination_certainty: 0.7,
            negativity_score: 0.6,
        };
        let elims = vec![elim("A", 0.95, 4), elim("B", 0.92, 3)];
        let strengthened = NpimHardening::strengthen_negativity(profile, &elims);
        // Severiteti i lartë → negativity u ngrit.
        assert!(strengthened.negativity_score >= 0.6);
        assert!((0.0..=1.0).contains(&strengthened.failure_density));
    }
}
