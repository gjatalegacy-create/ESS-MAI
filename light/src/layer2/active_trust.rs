// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAYER2/ACTIVE_TRUST.RS — Active Trust Layer (Light)                ║
// ║  GJATA LEGACY™ — ESS-MAI Light                            (LAYER 2)    ║
// ║                                                                          ║
// ║  Light mbetet veshi, goja dhe ndërtuesi i besimit — por Layer 2 e bën   ║
// ║  AKTIV: nuk pret Shadow-in për të ndërtuar besim. Parashikon, peshon,    ║
// ║  pasuron output-in me histori, orienton drejt rrugëve më të besueshme.  ║
// ║                                                                          ║
// ║  PRESIONI I LIGHT-IT është i BUTË (jo sulm si NPRO):                   ║
// ║    • nuk shtrydh idenë                                                 ║
// ║    • e ngarkon me histori, e bën të gjurmueshme                        ║
// ║    • e forcon me mini algoritmin (LightMiniEvolver ekzistues)          ║
// ║    • e orienton (Recommended/Neutral/Risky/Unknown)                    ║
// ║                                                                          ║
// ║  Prodhon TrustContext (kontrata e përbashkët). KUFIRI: Light NUK        ║
// ║  vendos, NUK bën presion të hapur, NUK gjykon. Zero if/else.          ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::lab_contracts::trust::{TrustContext, TrustGuidance};

// ─────────────────────────────────────────────────────────────────────────────
// KONSTANTET — pragjet e orientimit të butë
// ─────────────────────────────────────────────────────────────────────────────

/// Mbi këtë trust score → Recommended.
pub const TRUST_RECOMMENDED: f32 = 0.70;
/// Nën këtë trust score (me histori) → Risky.
pub const TRUST_RISKY: f32 = 0.35;

// ─────────────────────────────────────────────────────────────────────────────
// RELIABILITY METRICS — sinjalet e besueshmërisë nga historiku
// ─────────────────────────────────────────────────────────────────────────────

/// Sinjalet që Light mbledh nga historiku i një primitivi/rruge.
#[derive(Debug, Clone, Copy)]
pub struct ReliabilitySignals {
    /// Sa herë është përdorur (nga LightMiniEvolver.get_usage_count).
    pub usage_count:     u32,
    /// Skori mesatar i suksesit [0,1] (nga get_average_vector_score).
    pub avg_success:     f32,
    /// Në sa domene ka qenë i qëndrueshëm.
    pub domain_count:    u32,
    /// Sa herë është verifikuar nga Shadow (histori verifikimi).
    pub verification_count: u32,
}

impl ReliabilitySignals {
    /// empty — pa histori (primitiv i ri). Zero if.
    pub fn empty() -> Self {
        Self {
            usage_count: 0,
            avg_success: 0.0,
            domain_count: 0,
            verification_count: 0,
        }
    }

    /// has_history — a ka ndonjë histori përdorimi? Zero if.
    pub fn has_history(&self) -> bool {
        self.usage_count > 0
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ACTIVE TRUST LAYER — prodhon TrustContext
// ─────────────────────────────────────────────────────────────────────────────

pub struct ActiveTrustLayer;

impl ActiveTrustLayer {
    /// build_context — ndërton TrustContext nga sinjalet e historikut.
    ///
    /// DY shtresa paralele (sipas orientimit):
    ///   • trust_score        → sa shpesh funksionon (nga avg_success)
    ///   • verification_score → sa herë u verifikua/mbijetoi (nga verif_count)
    /// Zero if — formula + match.
    pub fn build_context(
        trace_id: u64,
        domain: String,
        signals: &ReliabilitySignals,
        history_refs: Vec<u64>,
    ) -> TrustContext {
        // trust_score: sukses i peshuar nga vëllimi i përdorimit.
        // Pak përdorime me sukses të fortë ende vlejnë (proactive trust building).
        let usage_factor = (signals.usage_count as f32 / 5.0).clamp(0.0, 1.0);
        let trust_score = (signals.avg_success * 0.7 + signals.avg_success * usage_factor * 0.3)
            .clamp(0.0, 1.0);

        // reliability_score: stabiliteti ndër-domenesh.
        let reliability_score = (signals.domain_count as f32 / 5.0).clamp(0.0, 1.0);

        // verification_score: sa herë mbijetoi verifikimin (rigorozitet).
        let verification_score = (signals.verification_count as f32 / 3.0).clamp(0.0, 1.0);

        // Orientimi i butë (jo vendim).
        let guidance = Self::orient(trust_score, signals.has_history());

        TrustContext {
            trace_id,
            domain,
            trust_score,
            reliability_score,
            verification_score,
            history_refs,
            guidance,
        }
    }

    /// orient — orientimi i butë nga trust score + prania e historisë.
    /// Zero if — match tuple.
    ///
    /// Pa histori → Unknown. Me histori: i lartë → Recommended,
    /// i ulët → Risky, mes → Neutral.
    fn orient(trust_score: f32, has_history: bool) -> TrustGuidance {
        let high = trust_score >= TRUST_RECOMMENDED;
        let low  = trust_score < TRUST_RISKY;

        match (has_history, high, low) {
            (false, _, _)     => TrustGuidance::Unknown,
            (true, true, _)   => TrustGuidance::Recommended,
            (true, false, true) => TrustGuidance::Risky,
            (true, false, false) => TrustGuidance::Neutral,
        }
    }

    /// enrich_output — pasuron një output me sinjal besueshmërie (string).
    /// "Output Enrichment": output + histori suksesi. Zero if.
    pub fn enrich_output(ctx: &TrustContext) -> String {
        format!(
            "[trust={:.0}% | reliability={:.0}% | verification={:.0}% | {}]",
            ctx.trust_score * 100.0,
            ctx.reliability_score * 100.0,
            ctx.verification_score * 100.0,
            ctx.guidance.label()
        )
    }

    /// soft_priority — prioriteti i butë [0,1] (jo vendim — vetëm shtytje).
    /// Rrugët me besueshmëri më të lartë marrin prioritet më të lartë. Zero if.
    pub fn soft_priority(ctx: &TrustContext) -> f32 {
        ctx.combined_strength()
    }

    /// integrate_precedent — mbyll ciklin vetë-përmirësues: Light lexon
    /// precedentin nga kujtesa e verifikimit të Shadow-it (si sinjal f32) dhe
    /// e integron në besim. Sistemi mëson nga historia me kalimin e kohës.
    ///
    /// Light NUK e merr strukturën e Shadow-it (crate të ndara) — merr vetëm
    /// sinjalin numerik. Roli i Light s'ndryshon: ai ndërton besim, tani më
    /// të zgjuar sepse di çfarë ka kaluar/dështuar më parë. Zero if — formula.
    ///
    /// precedent_signal: [0,1] nga Shadow VerificationMemoryIndex::trust_signal.
    pub fn integrate_precedent(base_trust: f32, precedent_signal: f32) -> f32 {
        let base = base_trust.clamp(0.0, 1.0);
        let prec = precedent_signal.clamp(0.0, 1.0);
        // Besimi i ri: bazë + dëshmi historike e verifikuar nga Shadow.
        // Precedenti i fortë e ngre besimin; mungesa e tij e lë te baza.
        (base * 0.6 + prec * 0.4).clamp(0.0, 1.0)
    }

    /// caution_factor — sa kujdes duhet për një rrugë të njohur si dështim.
    /// known_failure nga Shadow → më shumë kujdes (prioritet i ulur). Zero if.
    pub fn caution_factor(base_priority: f32, known_failure: bool) -> f32 {
        // known_failure true → zbut prioritetin (filtrim i butë, jo bllokim).
        let penalty = (known_failure as u32 as f32) * 0.5;
        (base_priority * (1.0 - penalty)).clamp(0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strong_signals() -> ReliabilitySignals {
        ReliabilitySignals {
            usage_count: 10,
            avg_success: 0.9,
            domain_count: 4,
            verification_count: 3,
        }
    }

    #[test]
    fn empty_signals_unknown_guidance() {
        let ctx = ActiveTrustLayer::build_context(
            1, "physics".to_string(), &ReliabilitySignals::empty(), vec![],
        );
        assert_eq!(ctx.guidance, TrustGuidance::Unknown);
        assert_eq!(ctx.trust_score, 0.0);
    }

    #[test]
    fn strong_signals_recommended() {
        let ctx = ActiveTrustLayer::build_context(
            1, "physics".to_string(), &strong_signals(), vec![1, 2],
        );
        assert_eq!(ctx.guidance, TrustGuidance::Recommended);
        assert!(ctx.trust_score >= TRUST_RECOMMENDED);
        assert!(ctx.has_history());
    }

    #[test]
    fn risky_when_low_success_with_history() {
        let signals = ReliabilitySignals {
            usage_count: 5, avg_success: 0.2, domain_count: 1, verification_count: 0,
        };
        let ctx = ActiveTrustLayer::build_context(
            1, "d".to_string(), &signals, vec![1],
        );
        assert_eq!(ctx.guidance, TrustGuidance::Risky);
    }

    #[test]
    fn neutral_when_mid_success() {
        let signals = ReliabilitySignals {
            usage_count: 5, avg_success: 0.5, domain_count: 2, verification_count: 1,
        };
        let ctx = ActiveTrustLayer::build_context(
            1, "d".to_string(), &signals, vec![1],
        );
        assert_eq!(ctx.guidance, TrustGuidance::Neutral);
    }

    #[test]
    fn two_layers_distinct() {
        // trust_score (sukses) dhe verification_score (rigorozitet) janë të ndara.
        let ctx = ActiveTrustLayer::build_context(
            1, "d".to_string(), &strong_signals(), vec![1],
        );
        // Të dyja jo-zero, por të llogaritura veçmas.
        assert!(ctx.trust_score > 0.0);
        assert!(ctx.verification_score > 0.0);
    }

    #[test]
    fn enrich_output_contains_scores() {
        let ctx = ActiveTrustLayer::build_context(
            1, "d".to_string(), &strong_signals(), vec![1],
        );
        let enriched = ActiveTrustLayer::enrich_output(&ctx);
        assert!(enriched.contains("trust="));
        assert!(enriched.contains("RECOMMENDED"));
    }

    #[test]
    fn soft_priority_reflects_strength() {
        let strong = ActiveTrustLayer::build_context(1, "d".to_string(), &strong_signals(), vec![1]);
        let weak = ActiveTrustLayer::build_context(1, "d".to_string(), &ReliabilitySignals::empty(), vec![]);
        assert!(ActiveTrustLayer::soft_priority(&strong) > ActiveTrustLayer::soft_priority(&weak));
    }

    #[test]
    fn integrate_precedent_raises_trust() {
        // Precedent i fortë nga Shadow → besim i ngritur (cikli vetë-përmirësues).
        let with_precedent = ActiveTrustLayer::integrate_precedent(0.5, 0.9);
        let without = ActiveTrustLayer::integrate_precedent(0.5, 0.0);
        assert!(with_precedent > without);
        assert!(with_precedent >= 0.0 && with_precedent <= 1.0);
    }

    #[test]
    fn caution_factor_penalizes_known_failure() {
        // Dështim i njohur nga Shadow → prioritet i zbutur (filtrim i butë).
        let normal = ActiveTrustLayer::caution_factor(0.8, false);
        let cautious = ActiveTrustLayer::caution_factor(0.8, true);
        assert!(cautious < normal);
        // Por jo bllokim total (Light nuk vendos).
        assert!(cautious > 0.0);
    }
}
