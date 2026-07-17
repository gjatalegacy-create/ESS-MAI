// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAYER3/MPRO.RS — Max Primitive Reasoning Origin (Layer 3)           ║
// ║  GJATA LEGACY™ — ESS-MAI Quantum                          (LAYER 3)    ║
// ║                                                                          ║
// ║  MPRO është MATES DETERMINIST — JO vendimmarrës.                      ║
// ║  Merr 16 matje (PRO 4 + NPRO 4 + HPRO 4 + APRO 4) dhe prodhon një        ║
// ║  SCORE VECTOR (score besimi, JO probabilitet).                        ║
// ║                                                                          ║
// ║  RREGULL ZYRTAR (Hapi 5):                                             ║
// ║   • MPRO NUK ELIMINON. Prodhon Score Vector.                          ║
// ║   • Score-i LIDHET me TrustContext EKZISTUES (jo strukturë paralele).   ║
// ║   • history_refs e TrustContext = akumulimi kohor (input-output).      ║
// ║   • Output = update i TrustContext (rrit/ul trust_score, shton ref),    ║
// ║     JO vendim Accept/Reject.                                          ║
// ║                                                                          ║
// ║  PSE SCORE BESIMI, JO PROBABILITET:                                   ║
// ║   9/16 = 0.56 → NUK është 56%. Është Score Vector — sa besim merr ai     ║
// ║   input/output. Akumulohet me kohën (sidomos output).                  ║
// ║                                                                          ║
// ║  KERNEL: 0-copy, FFI gate, HatCap compatible — i pandryshuar.          ║
// ║  TrustContext: IMPORT DIREKT nga lab_contracts (jo mirror).            ║
// ║  KUFIRI: MPRO mat, s'vendos (verdiktin 0/1 e jep Shadow).             ║
// ║  Zero if/else klasik.                                                 ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::lab_contracts::trust::{TrustContext, TrustGuidance};
use crate::pd_spine_contract::{FixedMass, MASS_SCALE};

// Token seal për matjet e MPRO (FNV i modulit).
const S_MPRO: u64 = 0x4D50_524F_0000_0001; // "MPRO" + version

// ─────────────────────────────────────────────────────────────────────────────
// MEASUREMENT INPUT — 16 matjet hyrëse (0/1 secila)
// ─────────────────────────────────────────────────────────────────────────────

/// 16 matjet nga 4 familjet (secila 4 operatorë, output 0/1).
/// MPRO i merr këto si HYRJE — nuk i prodhon vetë.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MeasurementSet {
    /// PRO: 4 matje (reverse, mirror, contrast, trace).
    pub pro:  [u8; 4],
    /// NPRO: 4 matje (nreverse, nmirror, ncontrast, ntrace).
    pub npro: [u8; 4],
    /// HPRO: 4 matje (thellësi/territor).
    pub hpro: [u8; 4],
    /// APRO: 4 matje (science, hypothesis, argument, historical-philosophy).
    pub apro: [u8; 4],
}

impl MeasurementSet {
    /// new — krijon nga 4 grupet. Zero if.
    pub fn new(pro: [u8; 4], npro: [u8; 4], hpro: [u8; 4], apro: [u8; 4]) -> Self {
        Self { pro, npro, hpro, apro }
    }

    /// positives — sa nga 16 matjet janë 1. Zero if — sum mbi clamp.
    pub fn positives(&self) -> u32 {
        let count_group = |g: &[u8; 4]| -> u32 {
            g.iter().map(|&m| (m != 0) as u32).sum()
        };
        count_group(&self.pro) + count_group(&self.npro)
            + count_group(&self.hpro) + count_group(&self.apro)
    }

    /// total — gjithmonë 16 (4 familje × 4 operatorë). Zero if.
    pub fn total(&self) -> u32 {
        16
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// SCORE VECTOR — rezultati i MPRO (score besimi, JO probabilitet)
// ─────────────────────────────────────────────────────────────────────────────

/// Score Vector — sa besim merr ai input/output. JO probabilitet.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScoreVector {
    /// Matjet pozitive (nga 16).
    pub positives:    u32,
    /// Totali (16).
    pub total:        u32,
    /// Masë vektoriale fixed-point [0..10_000] = positives/total.
    /// 8/16 = 5_000 = 0.5000. Runtime-i nuk ruan f32.
    pub mass:         u32,
    /// A u vulos matja (token mint+burn).
    pub sealed:       bool,
}

impl ScoreVector {
    /// is_strong — a është score-i i fortë (≥0.50)? Zero if.
    /// (Vetëm informativ — MPRO s'eliminon, vetëm mat.)
    pub fn is_strong(&self) -> bool {
        self.mass >= MASS_SCALE / 2
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// MPRO — mates determinist (kernel i pastër)
// ─────────────────────────────────────────────────────────────────────────────

pub struct Mpro;

impl Mpro {
    /// measure — prodhon Score Vector nga 16 matjet. MATES, jo vendimmarrës.
    ///
    /// 9/16 = 0.56 → Score Vector (jo 56%). Zero if — formula.
    /// Kernel i pastër: zero alokime, zero copy të panevojshme.
    pub fn measure(measurements: &MeasurementSet) -> ScoreVector {
        let positives = measurements.positives();
        let total = measurements.total();

        // Masë fikse = positives/16 në shkallën 10_000. Pa float.
        let mass = FixedMass::from_ratio(positives, total).raw();

        ScoreVector {
            positives,
            total,
            mass,
            sealed: false,  // vuloset te measure_sealed
        }
    }

    /// measure_sealed — prodhon Score Vector dhe e vulos me token (FFI gate).
    ///
    /// Përdor SovereignGate-style seal për matje të verifikueshme. Zero if — match.
    pub fn measure_sealed(measurements: &MeasurementSet, seal: u64) -> ScoreVector {
        let mut vector = Self::measure(measurements);
        // Vula: seal duhet të përputhet me S_MPRO (matje e autorizuar). Zero if.
        let valid_seal = seal == S_MPRO;
        let sealed = match valid_seal {
            true  => true,
            false => false,
        };
        vector.sealed = sealed;
        vector
    }

    /// mpro_seal — kthen seal-in e autorizuar të MPRO. Zero if.
    pub fn mpro_seal() -> u64 {
        S_MPRO
    }

    /// apply_to_trust — LIDH Score Vector me TrustContext ekzistues.
    ///
    /// Output = update i TrustContext (rrit/ul trust_score + shton history_ref),
    /// JO vendim Accept/Reject. Ky është akumulimi kohor (input-output).
    /// Zero if — formula + match.
    pub fn apply_to_trust(
        score: &ScoreVector,
        context: &TrustContext,
        measurement_ref: u64,
    ) -> TrustContext {
        // Score Vector lëviz trust_score drejt vetes (mesatare e peshuar).
        // Akumulim: besimi i ri = 60% i vjetri + 40% matja e re. Zero if.
        // TrustContext është kontratë legacy f32; konvertimi ndodh vetëm në këtë kufi.
        let score_legacy = score.mass as f32 / MASS_SCALE as f32;
        let new_trust = (context.trust_score * 0.60 + score_legacy * 0.40).clamp(0.0, 1.0);

        // history_refs rritet me referencën e re (akumulim kohor).
        let mut new_refs = context.history_refs.clone();
        new_refs.push(measurement_ref);

        // guidance rikalibrohet nga trust i ri (jo Accept/Reject binar). Zero if — match.
        let new_guidance = Self::guidance_from_trust(new_trust, new_refs.len());

        TrustContext {
            trace_id:           context.trace_id,
            domain:             context.domain.clone(),
            trust_score:        new_trust,
            reliability_score:  context.reliability_score,
            verification_score: context.verification_score,
            history_refs:       new_refs,
            guidance:           new_guidance,
        }
    }

    /// guidance_from_trust — rikalibron orientimin nga trust + histori. Zero if — match.
    ///
    /// Recommended/Neutral/Risky/Unknown — JO Accept/Reject (filozofia "mat").
    fn guidance_from_trust(trust: f32, history_count: usize) -> TrustGuidance {
        let has_history = history_count > 0;
        let strong = trust >= 0.70;
        let weak = trust < 0.35;

        // Match mbi (ka_histori, fort, dobët) — zero if.
        match (has_history, strong, weak) {
            (false, _, _)        => TrustGuidance::Unknown,    // pa histori
            (true, true, _)      => TrustGuidance::Recommended, // fort
            (true, false, true)  => TrustGuidance::Risky,       // dobët
            (true, false, false) => TrustGuidance::Neutral,     // mes
        }
    }

    /// accumulate_output — akumulim i veçantë për OUTPUT (sidomos output).
    ///
    /// Output-i merr peshë më të madhe në akumulim (50/50) sepse output-i
    /// verifikuar nga Shadow është më i vlefshëm. Zero if.
    pub fn accumulate_output(
        score: &ScoreVector,
        context: &TrustContext,
        output_ref: u64,
    ) -> TrustContext {
        // Output: 50% i vjetri + 50% matja (më shumë peshë se input). Zero if.
        let score_legacy = score.mass as f32 / MASS_SCALE as f32;
        let new_trust = (context.trust_score * 0.50 + score_legacy * 0.50).clamp(0.0, 1.0);
        let mut new_refs = context.history_refs.clone();
        new_refs.push(output_ref);
        let new_guidance = Self::guidance_from_trust(new_trust, new_refs.len());

        TrustContext {
            trace_id:           context.trace_id,
            domain:             context.domain.clone(),
            trust_score:        new_trust,
            reliability_score:  context.reliability_score,
            verification_score: context.verification_score,
            history_refs:       new_refs,
            guidance:           new_guidance,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_measurements() -> MeasurementSet {
        // PRO=3, NPRO=2, HPRO=2, APRO=2 → 9 pozitive (si shembulli i dokumentit).
        MeasurementSet::new(
            [1, 1, 1, 0],  // PRO: 3
            [1, 1, 0, 0],  // NPRO: 2
            [1, 1, 0, 0],  // HPRO: 2
            [1, 1, 0, 0],  // APRO: 2
        )
    }

    #[test]
    fn measures_nine_of_sixteen() {
        let m = sample_measurements();
        assert_eq!(m.positives(), 9);
        assert_eq!(m.total(), 16);
    }

    #[test]
    fn score_vector_is_confidence_not_probability() {
        let m = sample_measurements();
        let score = Mpro::measure(&m);
        // 9/16 = 0.5625 → Score Vector (jo 56%).
        assert_eq!(score.mass, 5_625);
        assert_eq!(score.positives, 9);
    }

    #[test]
    fn mpro_does_not_eliminate() {
        // MPRO prodhon score edhe për matje të dobëta — s'eliminon.
        let weak = MeasurementSet::new([0,0,0,0], [0,0,0,0], [1,0,0,0], [0,0,0,0]);
        let score = Mpro::measure(&weak);
        // 1/16 = 0.0625 → prapë prodhon score (s'fshin kandidatin).
        assert!(score.mass > 0);
        assert_eq!(score.positives, 1);
    }

    #[test]
    fn seal_validates_authorized_measurement() {
        let m = sample_measurements();
        let score = Mpro::measure_sealed(&m, Mpro::mpro_seal());
        assert!(score.sealed); // seal i saktë
        // Seal i gabuar → s'vuloset.
        let bad = Mpro::measure_sealed(&m, 0xDEAD);
        assert!(!bad.sealed);
    }

    #[test]
    fn applies_to_trust_context() {
        let m = sample_measurements();
        let score = Mpro::measure(&m);
        let ctx = TrustContext::empty(1, "physics".to_string());
        let updated = Mpro::apply_to_trust(&score, &ctx, 100);
        // trust_score lëvizi drejt score-it (akumulim).
        assert!(updated.trust_score > 0.0);
        // history_ref u shtua (akumulim kohor).
        assert_eq!(updated.history_refs.len(), 1);
        assert_eq!(updated.history_refs[0], 100);
    }

    #[test]
    fn output_accumulates_more_weight() {
        let m = sample_measurements();
        let score = Mpro::measure(&m); // 0.5625
        let ctx = TrustContext::empty(1, "d".to_string()); // trust=0.0
        // Input: 60/40 → 0.225; Output: 50/50 → 0.281.
        let input_upd = Mpro::apply_to_trust(&score, &ctx, 1);
        let output_upd = Mpro::accumulate_output(&score, &ctx, 1);
        // Output merr më shumë peshë.
        assert!(output_upd.trust_score > input_upd.trust_score);
    }

    #[test]
    fn accumulation_over_time() {
        let m = sample_measurements();
        let score = Mpro::measure(&m);
        let mut ctx = TrustContext::empty(1, "d".to_string());
        // 3 matje radhazi → trust konvergon, history rritet.
        for r in 0..3 {
            ctx = Mpro::apply_to_trust(&score, &ctx, r);
        }
        assert_eq!(ctx.history_refs.len(), 3); // akumulim
        assert!(ctx.trust_score > 0.3); // konvergjim drejt 0.56
    }

    #[test]
    fn guidance_not_accept_reject() {
        // Guidance është Recommended/Neutral/Risky/Unknown — jo binar.
        let m = MeasurementSet::new([1,1,1,1], [1,1,1,1], [1,1,1,1], [1,1,1,0]); // 15/16
        let score = Mpro::measure(&m);
        let ctx = TrustContext::empty(1, "d".to_string());
        // Disa akumulime → trust i lartë → Recommended (jo "Accept").
        let mut c = ctx;
        for r in 0..5 { c = Mpro::accumulate_output(&score, &c, r); }
        assert_eq!(c.guidance, TrustGuidance::Recommended);
    }
}
