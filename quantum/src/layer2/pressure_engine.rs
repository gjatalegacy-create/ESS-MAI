// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAYER2/PRESSURE_ENGINE.RS — Motori i Presionit (Quantum)            ║
// ║  GJATA LEGACY™ — ESS-MAI Quantum                          (LAYER 2)    ║
// ║                                                                          ║
// ║  Quantum është laboratori ku presioni është i HAPUR dhe i MATSHËM.      ║
// ║  Pressure Engine NUK e zëvendëson NPRO-në — e THËRRET dhe e FORCON,     ║
// ║  pastaj e përkthen rezultatin në PressureReport (kontrata e përbashkët).║
// ║                                                                          ║
// ║  Shkrirja e saktë (nga orientimi):                                    ║
// ║    • NPRO              → motori i presionit (sulmon, ekspozon dobësi)   ║
// ║    • npro_hardening    → forcuesi (consensus, severity, evidence)       ║
// ║    • PressureReport    → produkti i unifikuar (gjuha e përbashkët)      ║
// ║                                                                          ║
// ║  KUFIRI: presioni ≠ matja. Ky modul prodhon VETËM PressureReport,       ║
// ║  kurrë TrlEvidence (matja TRL i takon digital_lab/PIM). Zero if/else.  ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::npro::{NproEngine, NproResult, NegativeElimination};
use crate::npro_hardening::{WeaknessConsensus, SeverityEscalation, EvidenceWeighting};
use crate::lab_contracts::pressure::{PressureReport, PressureSource};

// ─────────────────────────────────────────────────────────────────────────────
// KONSTANTET — pragjet e nivelit të presionit
// ─────────────────────────────────────────────────────────────────────────────

/// Pragu i dobësisë mbi të cilin presioni quhet "i mbijetuar dobët".
pub const PRESSURE_WEAKNESS_HIGH: f32 = 0.65;
/// Pragu i kontradiktës mbi të cilin ideja konsiderohet e thyer.
pub const PRESSURE_CONTRADICTION_HIGH: f32 = 0.60;

// ─────────────────────────────────────────────────────────────────────────────
// PRESSURE ENGINE — forcon NPRO dhe prodhon PressureReport
// ─────────────────────────────────────────────────────────────────────────────

pub struct PressureEngine;

impl PressureEngine {
    /// apply_pressure — thërret NPRO, e forcon me hardening, prodhon PressureReport.
    ///
    /// Ky është "presioni i hapur agresiv" i Quantum-it. NPRO sulmon idenë;
    /// hardening e forcon vlerësimin (consensus + severity + evidence density);
    /// rezultati paketohet si PressureReport (gjuha e përbashkët).
    /// Zero if — formula + match.
    pub fn apply_pressure(
        claim_id: u64,
        fragments: &[String],
        signal: f32,
        causal: f32,
    ) -> PressureReport {
        // 1. NPRO sulmon (motori i presionit ekzistues — i pandryshuar).
        let npro: NproResult = NproEngine::activate(fragments, signal, causal);

        // 2. Forcimi me hardening (consensus + severity + evidence density).
        let consensus = WeaknessConsensus::consensus_strength(&npro.eliminations);
        let severity  = SeverityEscalation::severity_factor(&npro.eliminations);
        let density   = EvidenceWeighting::weighted_failure_density(&npro.eliminations);

        // 3. weakness_score i forcuar: dobësia mesatare e ngritur nga konsensusi.
        let base_weakness = npro.weakness_signal.avg_weakness;
        let weakness_score = (base_weakness * 0.5 + consensus * 0.3 + density * 0.2)
            .clamp(0.0, 1.0);

        // 4. contradiction_score: nga severity (sa të ashpra janë dobësitë).
        let contradiction_score = (severity).clamp(0.0, 1.0);

        // 5. failure_modes: shkaqet e eliminimeve me dobësi mbi prag.
        let failure_modes: Vec<String> = npro.eliminations.iter()
            .filter(|e| e.weakness_score > PRESSURE_WEAKNESS_HIGH)
            .map(|e| Self::failure_label(e))
            .collect();

        // 6. pressure_path: gjurma e operatorëve të aplikuar (rruga e shtyrjes).
        let pressure_path = Self::build_path(&npro.eliminations);

        // 7. survived_level: sa nivel presioni e mbijetoi ideja (0..=3).
        let survived_level = Self::compute_survived(weakness_score, contradiction_score);

        PressureReport {
            claim_id,
            source: PressureSource::QuantumNpro,
            contradiction_score,
            weakness_score,
            failure_modes,
            pressure_path,
            survived_level,
        }
    }

    /// failure_label — përshkrim i shkurtër i një mënyre dështimi. Zero if.
    fn failure_label(e: &NegativeElimination) -> String {
        format!("{}: {}", e.operator, e.cause)
    }

    /// build_path — gjurma e rrugës së presionit (operatorët e ndjekur). Zero if.
    fn build_path(eliminations: &[NegativeElimination]) -> String {
        let ops: Vec<String> = eliminations.iter()
            .map(|e| e.operator.clone())
            .collect();
        ops.join("->")
    }

    /// compute_survived — sa nivel presioni e mbijetoi ideja (0..=3).
    ///
    /// 3 = mbijetoi ekstremin (dobësi+kontradikta të ulëta)
    /// 2 = mesatar | 1 = lehtë | 0 = u thye (dobësi/kontradikta të larta)
    /// Zero if — match tuple mbi pragjet.
    fn compute_survived(weakness: f32, contradiction: f32) -> u8 {
        let high_weak = weakness >= PRESSURE_WEAKNESS_HIGH;
        let high_contra = contradiction >= PRESSURE_CONTRADICTION_HIGH;
        let mid_weak = weakness >= 0.40;
        let mid_contra = contradiction >= 0.35;

        // Prioritet: u thye → lehtë → mesatar → ekstrem (mbijetoi plotësisht).
        match (high_weak | high_contra, mid_weak | mid_contra) {
            (true, _)  => 0,   // dobësi/kontradikta të larta → u thye
            (false, true) => 2, // presion mesatar i mbijetuar
            (false, false) => 3, // pa dobësi/kontradikta → mbijetoi ekstremin
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn produces_pressure_report_with_quantum_source() {
        let fragments = vec!["idea fragment one".to_string(), "fragment two".to_string()];
        let report = PressureEngine::apply_pressure(42, &fragments, 0.7, 0.6);
        assert_eq!(report.claim_id, 42);
        assert_eq!(report.source, PressureSource::QuantumNpro);
    }

    #[test]
    fn pressure_scores_are_bounded() {
        let fragments = vec!["test".to_string()];
        let report = PressureEngine::apply_pressure(1, &fragments, 0.5, 0.5);
        assert!(report.contradiction_score >= 0.0 && report.contradiction_score <= 1.0);
        assert!(report.weakness_score >= 0.0 && report.weakness_score <= 1.0);
    }

    #[test]
    fn survived_level_in_range() {
        let fragments = vec!["strong idea".to_string()];
        let report = PressureEngine::apply_pressure(1, &fragments, 0.9, 0.9);
        assert!(report.survived_level <= 3);
    }

    #[test]
    fn compute_survived_logic() {
        // Dobësi e lartë → u thye (0).
        assert_eq!(PressureEngine::compute_survived(0.8, 0.1), 0);
        // Kontradikta e lartë → u thye (0).
        assert_eq!(PressureEngine::compute_survived(0.1, 0.7), 0);
        // Mesatar → 2.
        assert_eq!(PressureEngine::compute_survived(0.45, 0.1), 2);
        // I ulët → mbijetoi ekstremin (3).
        assert_eq!(PressureEngine::compute_survived(0.1, 0.1), 3);
    }

    #[test]
    fn pressure_path_is_built() {
        let fragments = vec!["fragment".to_string()];
        let report = PressureEngine::apply_pressure(1, &fragments, 0.5, 0.5);
        // Rruga përmban operatorë (NPRO ekzekuton 4).
        assert!(!report.pressure_path.is_empty());
    }

    #[test]
    fn report_is_lab_contract_compatible() {
        // PressureReport ka metodat e kontratës (withstood, stress_index).
        let fragments = vec!["x".to_string()];
        let report = PressureEngine::apply_pressure(1, &fragments, 0.5, 0.5);
        let _ = report.withstood();
        let stress = report.stress_index();
        assert!(stress >= 0.0 && stress <= 1.0);
    }
}
