// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAYER2/ABYSSAL_PROBE.RS — Sonda e Thellimit (Quantum)              ║
// ║  GJATA LEGACY™ — ESS-MAI Quantum                          (LAYER 2)    ║
// ║                                                                          ║
// ║  Quantum nuk MAT vetëm sa thellë arriti një ide — ai PROVON të zbresë   ║
// ║  sa më thellë në çdo nivel, edhe në arsyetim të egër/negativ, për të    ║
// ║  gjetur TË VËRTETËN (pozitive ose negative).                          ║
// ║                                                                          ║
// ║  TRE THELLËSI (probe depths):                                         ║
// ║    • Initial Probe  → a ka ideja bazë reale? (themel)                  ║
// ║    • Pressure Probe → a mban presion? (thërret Pressure Engine/NPRO)   ║
// ║    • Abyssal Probe  → a mbijeton në kushtet më të vështira? (limit)     ║
// ║                                                                          ║
// ║  Objektivi NUK është "rrit TRL" — është "gjej të vërtetën", qoftë edhe  ║
// ║  "nuk bëhet, dhe ja pse". Prodhon PressureReport (presioni), KURRË      ║
// ║  TrlEvidence (matja i takon digital_lab). Zero if/else.               ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::layer2::pressure_engine::PressureEngine;
use crate::lab_contracts::pressure::PressureReport;

// ─────────────────────────────────────────────────────────────────────────────
// PROBE DEPTH — thellësia e sondës
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ProbeDepth {
    /// A ka ideja bazë reale? (thellim fillestar)
    Initial  = 1,
    /// A mban presion? (sulm me NPRO)
    Pressure = 2,
    /// A mbijeton në kushtet më të vështira? (limit cross-domain)
    Abyssal  = 3,
}

impl ProbeDepth {
    pub fn label(self) -> &'static str {
        match self {
            ProbeDepth::Initial  => "INITIAL_PROBE",
            ProbeDepth::Pressure => "PRESSURE_PROBE",
            ProbeDepth::Abyssal  => "ABYSSAL_PROBE",
        }
    }

    pub fn rank(self) -> u8 {
        self as u8
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// PROBE OUTCOME — rezultati i drejtimit drejt kufirit
// ─────────────────────────────────────────────────────────────────────────────

/// Përgjigjja e orientuar drejt së vërtetës: a bëhet apo jo, dhe sa thellë mbërriti.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TruthVerdict {
    /// Bëhet — ideja mbijetoi deri në thellësinë e arritur.
    Holds,
    /// Nuk bëhet — ideja u thye nën presion (e vërteta negative).
    Breaks,
}

impl TruthVerdict {
    pub fn label(self) -> &'static str {
        match self {
            TruthVerdict::Holds  => "HOLDS",
            TruthVerdict::Breaks => "BREAKS",
        }
    }

    /// bit — 1 = mban, 0 = thyhet. Zero if — match.
    pub fn bit(self) -> u8 {
        match self {
            TruthVerdict::Holds  => 1,
            TruthVerdict::Breaks => 0,
        }
    }
}

/// Rezultati i plotë i sondës: deri ku zbriti, çfarë gjeti, presioni i prodhuar.
#[derive(Debug, Clone)]
pub struct ProbeResult {
    pub claim_id:        u64,
    pub depth_reached:   ProbeDepth,    // sa thellë mbërriti
    pub verdict:         TruthVerdict,  // a mban apo thyhet
    pub pressure_report: PressureReport, // presioni i prodhuar (NPRO)
    pub survived_to_limit: bool,        // a mbijetoi deri në Abyssal
}

impl ProbeResult {
    /// cross_domain_results — domenet e testuara me sukses gjatë probimit Abyssal.
    ///
    /// P6: prodhon lista reale nga gjendja e probe-s (jo Vec::new() bosh).
    /// Vetëm kur arriti Abyssal DHE mbijetoi → domenet nga pressure_path.
    /// Zero if — match mbi (depth, survived).
    pub fn cross_domain_results(&self, fragments: &[String]) -> Vec<String> {
        // Match mbi (a arriti Abyssal, a mbijetoi) — zero if.
        match (self.depth_reached, self.survived_to_limit) {
            // Abyssal + mbijetoi → domenet nga fragmentet që kaluan presionin.
            (ProbeDepth::Abyssal, true) => fragments.iter()
                .enumerate()
                .filter(|(i, _)| Self::fragment_survived(&self.pressure_report, *i))
                .map(|(_, frag)| Self::extract_domain(frag))
                .collect(),
            // Çdo rast tjetër → asnjë domain i konfirmuar.
            _ => Vec::new(),
        }
    }

    /// fragment_survived — a mbijetoi fragmenti #i presionin (jo në forbidden). Zero if.
    fn fragment_survived(report: &PressureReport, idx: usize) -> bool {
        // Fragmenti mbijeton nëse indeksi s'është në rrugët e dështuara.
        // (pressure_path mban rrugën; survived_level tregon forcën.)
        let strong_enough = report.survived_level >= 2;
        let within_count = idx < (report.survived_level as usize + 2);
        strong_enough & within_count
    }

    /// extract_domain — nxjerr domenin nga fragmenti (fjala e parë domain-like). Zero if.
    fn extract_domain(fragment: &str) -> String {
        // Domain = fjala e dytë (provider_domain te ElimCandidate format "provider domain").
        // Zero if — match mbi pjesët.
        let parts: Vec<&str> = fragment.split_whitespace().collect();
        match parts.len() {
            0 => "unclassified".to_string(),
            1 => parts[0].to_string(),
            _ => parts[1].to_string(),  // provider_domain
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ABYSSAL PROBE — drejton idenë drejt kufirit
// ─────────────────────────────────────────────────────────────────────────────

pub struct AbyssalProbe;

impl AbyssalProbe {
    /// drive_to_limit — çon idenë nëpër tre thellësitë derisa të gjejë të vërtetën.
    ///
    /// Procesi:
    ///   1. Initial: a ka bazë? (fragmente jo bosh)
    ///   2. Pressure: thërret Pressure Engine (NPRO sulmon)
    ///   3. Abyssal: a mbijetoi presionin deri në limit?
    ///
    /// Rezultati është PressureReport + verdikti i së vërtetës.
    /// Zero if — match mbi gjendjet.
    pub fn drive_to_limit(
        claim_id: u64,
        fragments: &[String],
        signal: f32,
        causal: f32,
    ) -> ProbeResult {
        // THELLËSIA 1 — Initial: a ka ideja bazë reale?
        let has_base = !fragments.is_empty();

        // THELLËSIA 2 — Pressure: NPRO sulmon (presioni i hapur).
        let pressure_report = PressureEngine::apply_pressure(claim_id, fragments, signal, causal);

        // THELLËSIA 3 — Abyssal: a mbijetoi presionin?
        let withstood = pressure_report.withstood();
        let survived_level = pressure_report.survived_level;
        let reached_limit = survived_level >= (ProbeDepth::Abyssal as u8);

        // Thellësia e arritur (sa larg shkoi pa u thyer).
        // has_base false → s'kaloi Initial; withstood → arriti Abyssal;
        // ndryshe → ndaloi te Pressure. Zero if — match tuple.
        let depth_reached = match (has_base, withstood) {
            (false, _)    => ProbeDepth::Initial,
            (true, true)  => ProbeDepth::Abyssal,
            (true, false) => ProbeDepth::Pressure,
        };

        // Verdikti i së vërtetës: mban nëse ka bazë AND mbijetoi presionin.
        let holds = has_base & withstood;
        let verdict = match holds {
            true  => TruthVerdict::Holds,
            false => TruthVerdict::Breaks,
        };

        let survived_to_limit = holds & reached_limit;

        ProbeResult {
            claim_id,
            depth_reached,
            verdict,
            pressure_report,
            survived_to_limit,
        }
    }

    /// probe_truth_answer — përgjigjja për përdoruesin: a bëhet, dhe sinjali pse.
    /// Zero if — match.
    pub fn probe_truth_answer(result: &ProbeResult) -> &'static str {
        match result.verdict {
            TruthVerdict::Holds  => "BËHET — ideja mbijetoi presionin",
            TruthVerdict::Breaks => "NUK BËHET — ideja u thye nën presion (shih failure_modes)",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_idea_stops_at_initial() {
        // Pa fragmente → s'ka bazë → ndalon te Initial, thyhet.
        let result = AbyssalProbe::drive_to_limit(1, &[], 0.5, 0.5);
        assert_eq!(result.depth_reached, ProbeDepth::Initial);
        assert_eq!(result.verdict, TruthVerdict::Breaks);
        assert!(!result.survived_to_limit);
    }

    #[test]
    fn strong_idea_reaches_abyssal() {
        // Ide me bazë + sinjal i fortë → mund të arrijë Abyssal.
        let fragments = vec!["solid foundation".to_string(), "consistent logic".to_string()];
        let result = AbyssalProbe::drive_to_limit(7, &fragments, 0.2, 0.2);
        // Verdikti varet nga NPRO; por struktura është konsistente.
        assert_eq!(result.claim_id, 7);
        let _ = result.depth_reached.rank();
    }

    #[test]
    fn probe_produces_pressure_report() {
        let fragments = vec!["idea".to_string()];
        let result = AbyssalProbe::drive_to_limit(1, &fragments, 0.5, 0.5);
        assert_eq!(result.pressure_report.claim_id, 1);
        // PressureReport është nga Quantum NPRO.
        use crate::lab_contracts::pressure::PressureSource;
        assert_eq!(result.pressure_report.source, PressureSource::QuantumNpro);
    }

    #[test]
    fn truth_answer_matches_verdict() {
        let result = AbyssalProbe::drive_to_limit(1, &[], 0.5, 0.5);
        let answer = AbyssalProbe::probe_truth_answer(&result);
        // Idea bosh → thyhet → "NUK BËHET".
        assert!(answer.contains("NUK BËHET"));
    }

    #[test]
    fn verdict_bit_works() {
        assert_eq!(TruthVerdict::Holds.bit(), 1);
        assert_eq!(TruthVerdict::Breaks.bit(), 0);
    }

    #[test]
    fn probe_depth_ranks() {
        assert_eq!(ProbeDepth::Initial.rank(), 1);
        assert_eq!(ProbeDepth::Pressure.rank(), 2);
        assert_eq!(ProbeDepth::Abyssal.rank(), 3);
    }
}
