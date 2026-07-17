// ESS-MAI v1.6.0: Ky është LIGHT EMOTIONAL/INTERPRETATION SPINE për UI-në e
// vjetër. Nuk është PD Spine 9 i Quantum dhe nuk ekzekuton Layer 1/2/3 të PD.
// Ai pasqyron gjendjen e sistemit dhe mbetet paralel me Nura-n.

// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LIGHT_SPINE.RS — Kurrizi Vertikal i Light-it (PATCHIM)              ║
// ║  GJATA LEGACY™ — ESS-MAI Light                            (SPINE)       ║
// ║                                                                          ║
// ║  PATCHIM I THELLË: lidh shtresat e Light-it në NJË rrjedhë interpretimi. ║
// ║  Para patchimit, ui_channel (L3) dhe active_trust (L2) ishin të           ║
// ║  shkëputur — ky kurriz i bashkon: inputi nga UI merr gjurmim, kuptim     ║
// ║  dhe besim aktiv në një rrjedhë.                                       ║
// ║                                                                          ║
// ║  RRJEDHA E PLOTË E LIGHT-it (si një trup):                           ║
// ║    1. UI Channel (L3)      → input merr gjurmim në hyrje               ║
// ║    2. Neural Law (L3)      → kuptim i lëvizjes                         ║
// ║    3. Active Trust (L2)    → besim aktiv nga historiku                 ║
// ║    4. Verification Trace (L2) → gjurma e verifikimit                   ║
// ║                                                                          ║
// ║  KUFIRI: Light interpreton/transporton/observon — KURRË s'vendos.       ║
// ║  Kurrizi vetëm orkestron rrjedhën ekzistuese. Zero if/else.           ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::lab_contracts::trust::TrustContext;
use crate::layer2::active_trust::{ActiveTrustLayer, ReliabilitySignals};
use crate::layer3::ui_channel::{UiInput, TracedInput};
use crate::layer3::layer3_flow::{LightLayer3Flow, LightLayer3Result};

// ─────────────────────────────────────────────────────────────────────────────
// LIGHT SPINE RESULT — rezultati i rrjedhës vertikale
// ─────────────────────────────────────────────────────────────────────────────

/// Rezultati i plotë i kurrizit të Light-it — gjurmim + kuptim + besim.
#[derive(Debug, Clone)]
pub struct LightSpineResult {
    /// Inputi i gjurmuar me kuptim (L3).
    pub traced_input:  TracedInput,
    /// Konteksti i besimit aktiv (L2).
    pub trust_context: TrustContext,
    /// Prioriteti i butë i rrugës [0,1].
    pub soft_priority: f32,
}

// ─────────────────────────────────────────────────────────────────────────────
// LIGHT SPINE — orkestron rrjedhën vertikale L3→L3→L2→L2
// ─────────────────────────────────────────────────────────────────────────────

pub struct LightSpine;

impl LightSpine {
    /// interpret_full — rrjedha e plotë e Light-it nga UI te besimi aktiv.
    ///
    /// Lidh: ui_channel (gjurmim) → neural_law (kuptim, brenda layer3_flow) →
    /// active_trust (besim aktiv). Light interpreton, s'vendos. Zero if.
    pub fn interpret_full(
        input: UiInput,
        trace_id: u64,
        signals: &ReliabilitySignals,
        territory: String,
        history_refs: Vec<u64>,
        at: u64,
    ) -> LightSpineResult {
        // HAP 1+2 (L3): UI input merr gjurmim + kuptim (via layer3_flow).
        // Vlerësimi i besimit fillestar nga sinjalet.
        let trust_preview = signals.avg_success;
        let l3 = LightLayer3Flow::ingest_from_ui(
            input, trace_id, trust_preview, territory.clone(), at,
        );
        let traced_input = l3.traced_input;

        // HAP 3 (L2): Active Trust ndërton kontekstin e besimit nga historiku.
        let trust_context = ActiveTrustLayer::build_context(
            trace_id, territory, signals, history_refs,
        );

        // HAP 4 (L2): Prioriteti i butë (jo vendim — vetëm shtytje).
        let soft_priority = ActiveTrustLayer::soft_priority(&trust_context);

        LightSpineResult {
            traced_input,
            trust_context,
            soft_priority,
        }
    }

    /// apply_shadow_precedent — integron precedentin nga Shadow (cikli i mbyllur).
    ///
    /// Light merr sinjalin f32 nga Shadow (jo strukturën — crate të ndara) dhe
    /// e integron në besim. Sistemi mëson; roli i Light s'ndryshon. Zero if.
    pub fn apply_shadow_precedent(
        result: &LightSpineResult,
        precedent_signal: f32,
    ) -> f32 {
        // Besimi i ri = besim aktual + precedent i verifikuar nga Shadow.
        ActiveTrustLayer::integrate_precedent(
            result.trust_context.trust_score,
            precedent_signal,
        )
    }

    /// guidance_label — orientimi i butë për UI (jo vendim). Zero if.
    pub fn guidance_label(result: &LightSpineResult) -> &'static str {
        result.trust_context.guidance.label()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lab_contracts::trust::TrustGuidance;

    fn strong_signals() -> ReliabilitySignals {
        ReliabilitySignals {
            usage_count: 10, avg_success: 0.9, domain_count: 4, verification_count: 3,
        }
    }

    #[test]
    fn full_flow_connects_ui_and_trust() {
        let input = UiInput::new("explore with Nura".to_string(), "s1".to_string(), false);
        let result = LightSpine::interpret_full(
            input, 42, &strong_signals(), "physics".to_string(), vec![1, 2], 1000,
        );
        // L3 input u gjurmua.
        assert_eq!(result.traced_input.trace_id, 42);
        // L2 besimi u ndërtua.
        assert!(result.trust_context.trust_score > 0.0);
        assert_eq!(result.trust_context.guidance, TrustGuidance::Recommended);
        // Prioriteti i butë llogaritet.
        assert!(result.soft_priority > 0.0);
    }

    #[test]
    fn shadow_precedent_integrated() {
        let input = UiInput::new("q".to_string(), "s".to_string(), false);
        let result = LightSpine::interpret_full(
            input, 1, &strong_signals(), "d".to_string(), vec![1], 100,
        );
        // Precedent i fortë nga Shadow → besim i ngritur.
        let with_precedent = LightSpine::apply_shadow_precedent(&result, 0.9);
        let without = LightSpine::apply_shadow_precedent(&result, 0.0);
        assert!(with_precedent > without);
    }

    #[test]
    fn light_interprets_not_decides() {
        // Çdo gjë: gjurmim + kuptim + besim — asnjë vendim.
        let input = UiInput::new("test".to_string(), "s".to_string(), false);
        let result = LightSpine::interpret_full(
            input, 1, &strong_signals(), "d".to_string(), vec![1], 100,
        );
        // Rezultati është interpretim, jo verdikt.
        assert!(!result.traced_input.meaning.meaning.is_empty());
        let label = LightSpine::guidance_label(&result);
        assert!(!label.is_empty());
    }

    #[test]
    fn empty_history_unknown_guidance() {
        let input = UiInput::new("q".to_string(), "s".to_string(), false);
        let result = LightSpine::interpret_full(
            input, 1, &ReliabilitySignals::empty(), "d".to_string(), vec![], 100,
        );
        // Pa histori → Unknown (besim s'mund të ndërtohet ende).
        assert_eq!(result.trust_context.guidance, TrustGuidance::Unknown);
    }
}
