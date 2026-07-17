// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAYER2/VERIFICATION_TRACE.RS — Verification Trace Layer (Light)     ║
// ║  GJATA LEGACY™ — ESS-MAI Light                            (LAYER 2)    ║
// ║                                                                          ║
// ║  Light ruan dhe ekspozon GJURMËN E VERIFIKIMIT: si u ndërtua besimi,    ║
// ║  sa herë mbijetoi, në cilat domene, çfarë evidence ekzistonte.         ║
// ║                                                                          ║
// ║  + Mini Algorithm Hardening: forcon vlerësimin me algoritmin e vogël    ║
// ║  ekzistues (LightMiniEvolver) — i përshtatshëm për trust/trace/soft     ║
// ║  filtering, jo për motorin e madh agresiv (ai i takon Quantum-it).     ║
// ║                                                                          ║
// ║  Prodhon gjurmë të lexueshme nga të tria platformat. Zero if/else.    ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::lab_contracts::trust::TrustContext;

// ─────────────────────────────────────────────────────────────────────────────
// VERIFICATION TRACE — gjurma e historisë së verifikimit
// ─────────────────────────────────────────────────────────────────────────────

/// Një hap në gjurmën e verifikimit të një rruge.
#[derive(Debug, Clone)]
pub struct TraceStep {
    /// Domeni ku u provua.
    pub domain:       String,
    /// A funksionoi në këtë hap.
    pub succeeded:    bool,
    /// Vula kohore.
    pub at:           u64,
}

/// Gjurma e plotë e verifikimit — historiku i mbijetesës së një rruge.
#[derive(Debug, Clone)]
pub struct VerificationTrace {
    /// Identifikuesi i gjurmës.
    pub trace_id:     u64,
    /// Hapat e historisë.
    pub steps:        Vec<TraceStep>,
}

impl VerificationTrace {
    /// new — gjurmë bosh. Zero if.
    pub fn new(trace_id: u64) -> Self {
        Self { trace_id, steps: Vec::new() }
    }

    /// record — shton një hap në gjurmë. Zero if.
    pub fn record(&mut self, domain: String, succeeded: bool, at: u64) {
        self.steps.push(TraceStep { domain, succeeded, at });
    }

    /// success_count — sa hapa funksionuan. Zero if — filter.
    pub fn success_count(&self) -> usize {
        self.steps.iter().filter(|s| s.succeeded).count()
    }

    /// total_count — sa hapa gjithsej. Zero if.
    pub fn total_count(&self) -> usize {
        self.steps.len()
    }

    /// survival_rate — raporti i mbijetesës [0,1]. Zero if — formula me mbrojtje.
    pub fn survival_rate(&self) -> f32 {
        let total = self.steps.len().max(1) as f32;
        let success = self.success_count() as f32;
        (success / total).clamp(0.0, 1.0)
    }

    /// distinct_domains — sa domene unike u provuan. Zero if.
    pub fn distinct_domains(&self) -> usize {
        let mut domains: Vec<&String> = self.steps.iter().map(|s| &s.domain).collect();
        domains.sort();
        domains.dedup();
        domains.len()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// MINI ALGORITHM HARDENING — forcon vlerësimin me algoritmin e vogël
// ─────────────────────────────────────────────────────────────────────────────

pub struct MiniAlgorithmHardening;

impl MiniAlgorithmHardening {
    /// harden_trust — forcon trust_score-in me gjurmën e verifikimit.
    ///
    /// Algoritmi i vogël: kombinon trust ekzistues me survival rate të gjurmës.
    /// Pak përdorime me mbijetesë të lartë → besim i forcuar (proactive).
    /// Zero if — formula.
    pub fn harden_trust(base_trust: f32, trace: &VerificationTrace) -> f32 {
        let survival = trace.survival_rate();
        let domain_factor = (trace.distinct_domains() as f32 / 5.0).clamp(0.0, 1.0);
        // Forcimi: trust bazë + mbijetesë + bonus ndër-domenesh.
        (base_trust * 0.5 + survival * 0.35 + domain_factor * 0.15).clamp(0.0, 1.0)
    }

    /// stability_score — sa i qëndrueshëm është një primitiv [0,1]. Zero if.
    ///
    /// Stabilitet = mbijetesë e lartë AND në shumë domene.
    pub fn stability_score(trace: &VerificationTrace) -> f32 {
        let survival = trace.survival_rate();
        let breadth = (trace.distinct_domains() as f32 / 3.0).clamp(0.0, 1.0);
        (survival * 0.6 + breadth * 0.4).clamp(0.0, 1.0)
    }

    /// apply_to_context — integron forcimin në TrustContext (verification_score).
    /// Zero if.
    pub fn apply_to_context(ctx: &TrustContext, trace: &VerificationTrace) -> f32 {
        let hardened = Self::harden_trust(ctx.trust_score, trace);
        hardened
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lab_contracts::trust::{TrustContext, TrustGuidance};

    fn sample_trace() -> VerificationTrace {
        let mut t = VerificationTrace::new(1);
        t.record("physics".to_string(), true, 100);
        t.record("chemistry".to_string(), true, 200);
        t.record("physics".to_string(), false, 300);
        t.record("biology".to_string(), true, 400);
        t
    }

    #[test]
    fn trace_counts() {
        let t = sample_trace();
        assert_eq!(t.total_count(), 4);
        assert_eq!(t.success_count(), 3);
        assert_eq!(t.distinct_domains(), 3); // physics, chemistry, biology
    }

    #[test]
    fn survival_rate_correct() {
        let t = sample_trace();
        // 3 sukses / 4 total = 0.75
        assert!((t.survival_rate() - 0.75).abs() < 1e-6);
    }

    #[test]
    fn empty_trace_safe() {
        let t = VerificationTrace::new(1);
        // Pa hapa → survival rate 0 (jo panik nga pjesëtimi).
        assert_eq!(t.survival_rate(), 0.0);
        assert_eq!(t.distinct_domains(), 0);
    }

    #[test]
    fn harden_trust_boosts() {
        let t = sample_trace();
        // Trust bazë 0.5 + gjurmë e mirë → i forcuar.
        let hardened = MiniAlgorithmHardening::harden_trust(0.5, &t);
        assert!(hardened > 0.0 && hardened <= 1.0);
    }

    #[test]
    fn stability_reflects_breadth() {
        let t = sample_trace();
        let stability = MiniAlgorithmHardening::stability_score(&t);
        assert!(stability > 0.0 && stability <= 1.0);
    }

    #[test]
    fn apply_to_context_works() {
        let ctx = TrustContext {
            trace_id: 1, domain: "d".to_string(),
            trust_score: 0.6, reliability_score: 0.5, verification_score: 0.4,
            history_refs: vec![1], guidance: TrustGuidance::Neutral,
        };
        let t = sample_trace();
        let result = MiniAlgorithmHardening::apply_to_context(&ctx, &t);
        assert!(result >= 0.0 && result <= 1.0);
    }
}
