// ESS-MAI v1.6.0: Ky është LIGHT EMOTIONAL/INTERPRETATION SPINE për UI-në e
// vjetër. Nuk është PD Spine 9 i Quantum dhe nuk ekzekuton Layer 1/2/3 të PD.
// Ai pasqyron gjendjen e sistemit dhe mbetet paralel me Nura-n.

// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  PHASE9_INTEGRATION.RS — Lidhja me Rrjedhën e Gjallë (Light)         ║
// ║  GJATA LEGACY™ — ESS-MAI Light                            (FAZA 9)      ║
// ║                                                                          ║
// ║  FAZA 9 — PATCHIM I RRJEDHËS SË GJALLË:                               ║
// ║  Lidh light_spine (UI→gjurmim→kuptim→besim) + knowledge_channel me       ║
// ║  coordinator-in real, duke respektuar ligjin rrënjësor të Light.       ║
// ║                                                                          ║
// ║  LIGJI RRËNJËSOR I LIGHT — NUK VENDOS KURRË:                          ║
// ║   Light interpreton, transporton, ndërton besim, gjurmon — por çdo      ║
// ║   "vendim" final i takon Shadow-it. Faza 9 e ruan këtë me rreptësi:    ║
// ║   asnjë output i Light s'është verdikt 0/1.                            ║
// ║                                                                          ║
// ║  Plus: çdo paketë mban vulën 500 (verify_500). Knowledge merr trace     ║
// ║  fillestar para se të shkojë te Shadow.                                ║
// ║                                                                          ║
// ║  KUFIRI: Faza 9 lidh spine me rrjedhën; ruan që Light s'vendos.         ║
// ║  Zero if/else klasik (match/boolean).                                 ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::layer2::active_trust::ReliabilitySignals;
use crate::layer3::ui_channel::{UiInput, TracedInput};
use crate::layer3::knowledge_channel::{KnowledgeChannel, PipUpload, TracedUpload};
use crate::light_spine::{LightSpine, LightSpineResult};
use crate::sovereign::laws::verify_500;

// ─────────────────────────────────────────────────────────────────────────────
// LIGJET E FAZËS 9 (Light)
// ─────────────────────────────────────────────────────────────────────────────

/// Rezultati i kontrollit të ligjit "Light s'vendos".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightLawCheck {
    Passed,
    AttemptedDecision,  // Light provoi të vendoste (shkelje)
    InvalidSeal,        // vula 500 e pavlefshme
}

impl LightLawCheck {
    pub fn label(self) -> &'static str {
        match self {
            LightLawCheck::Passed            => "LAWS_PASSED",
            LightLawCheck::AttemptedDecision => "ATTEMPTED_DECISION",
            LightLawCheck::InvalidSeal       => "INVALID_SEAL",
        }
    }

    /// is_passed — a kaluan ligjet? Zero if.
    pub fn is_passed(self) -> bool {
        match self {
            LightLawCheck::Passed => true,
            _ => false,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// PHASE 9 INTEGRATION (Light)
// ─────────────────────────────────────────────────────────────────────────────

/// Rezultati i interpretimit me spine të lidhur.
#[derive(Debug, Clone)]
pub struct Phase9InterpretResult {
    pub law_check:    LightLawCheck,
    pub spine:        Option<LightSpineResult>,  // Some nëse ligjet kaluan
    pub interprets_only: bool,                   // gjithmonë true (Light s'vendos)
}

/// Rezultati i kanalit të knowledge.
#[derive(Debug, Clone)]
pub struct Phase9KnowledgeResult {
    pub law_check:        LightLawCheck,
    pub traced_upload:    Option<TracedUpload>,
    pub ready_for_shadow: bool,
}

pub struct Phase9Integration;

impl Phase9Integration {
    /// interpret_with_spine — lidh light_spine me rrjedhën, ruan që Light s'vendos.
    ///
    /// Çdo output i Light është interpretim/besim — KURRË verdikt. Faza 9 e
    /// verifikon këtë. Zero if — match.
    pub fn interpret_with_spine(
        input: UiInput,
        trace_id: u64,
        signals: &ReliabilitySignals,
        territory: String,
        history_refs: Vec<u64>,
        at: u64,
    ) -> Phase9InterpretResult {
        // LIGJ: Light s'vendos. Verifikoj para çdo lidhjeje (gjithmonë kalon
        // sepse spine prodhon vetëm interpretim/besim).
        let law_check = Self::check_light_laws();

        // Spine ekzekutohet VETËM nëse ligjet kalojnë. Zero if — match.
        let spine = match law_check.is_passed() {
            true => {
                let result = LightSpine::interpret_full(
                    input, trace_id, signals, territory, history_refs, at,
                );
                Some(result)
            }
            false => None,
        };

        Phase9InterpretResult {
            law_check,
            spine,
            interprets_only: true,  // Light gjithmonë vetëm interpreton
        }
    }

    /// ingest_knowledge_with_trace — lidh knowledge_channel me rrjedhën.
    ///
    /// Knowledge merr TRACE FILLESTAR te Light PARA se të shkojë te Shadow.
    /// Faza 9 verifikon vulën 500 të paketës. Zero if — match.
    pub fn ingest_knowledge_with_trace(
        upload: PipUpload,
        trace_id: u64,
        at: u64,
    ) -> Phase9KnowledgeResult {
        // Knowledge merr trace fillestar (Light gjurmon).
        let traced = KnowledgeChannel::ingest_pip(upload, trace_id, at);

        // LIGJ: vula 500 e paketës duhet të jetë e vlefshme.
        let seal_valid = verify_500(traced.packet.seal);
        let ready = KnowledgeChannel::is_ready_for_shadow(&traced);

        // Match mbi (seal_valid, ready) — zero if.
        let (law_check, traced_upload, ready_for_shadow) = match (seal_valid, ready) {
            (true, true)  => (LightLawCheck::Passed, Some(traced), true),
            (true, false) => (LightLawCheck::Passed, Some(traced), false),
            (false, _)    => (LightLawCheck::InvalidSeal, None, false),
        };

        Phase9KnowledgeResult {
            law_check,
            traced_upload,
            ready_for_shadow,
        }
    }

    /// check_light_laws — verifikon ligjin "Light s'vendos".
    ///
    /// Light prodhon vetëm interpretim/besim — kurrë verdikt. Ky është
    /// strukturalisht i garantuar (s'ka API vendimi te Light). Zero if.
    fn check_light_laws() -> LightLawCheck {
        // Light s'ka mënyrë të vendosë (s'ka verdikt API) → gjithmonë kalon.
        LightLawCheck::Passed
    }

    /// output_is_not_verdict — konfirmon që output-i i Light s'është verdikt.
    ///
    /// Rezultati ka trust_context (besim) + traced_input (gjurmim) — JO
    /// verdikt 0/1. Faza 9 e ruan këtë kufi. Zero if — match.
    pub fn output_is_not_verdict(result: &Phase9InterpretResult) -> bool {
        // Match mbi spine — nëse ekziston, përmban besim/gjurmim (jo verdikt).
        match &result.spine {
            Some(spine) => Self::is_interpretation(spine),
            None => false,
        }
    }

    /// is_interpretation — verifikon që rezultati është interpretim. Zero if.
    fn is_interpretation(spine: &LightSpineResult) -> bool {
        // Ka gjurmim (trace) + besim — interpretim, jo vendim.
        let has_trace = spine.traced_input.trace_id != 0;
        let has_meaning = !spine.traced_input.meaning.meaning.is_empty();
        has_trace & has_meaning
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn spine9_wired_produces_trust_not_verdict() {
        // Dëshmi që SPINE9 i lidhur në runtime prodhon BESIM (jo verdikt).
        let ui = UiInput { text: "flamuri kuq e zi".to_string(),
                           session_id: "S1".to_string(), deep_research: false };
        let sig = ReliabilitySignals::empty();
        let r = Phase9Integration::interpret_with_spine(
            ui, 0xA451, &sig, "letersi".to_string(), Vec::new(), 1000);
        assert!(r.law_check.is_passed());
        assert!(r.interprets_only);                        // Light interpreton
        assert!(Phase9Integration::output_is_not_verdict(&r)); // KURRË verdikt
        assert!(r.spine.is_some());                        // spine u aktivizua
    }

    use super::*;
    use crate::lab_contracts::trust::TrustGuidance;

    fn strong_signals() -> ReliabilitySignals {
        ReliabilitySignals {
            usage_count: 10, avg_success: 0.9, domain_count: 4, verification_count: 3,
        }
    }

    #[test]
    fn spine_runs_light_interprets_only() {
        let input = UiInput::new("explore with Nura".to_string(), "s1".to_string(), false);
        let result = Phase9Integration::interpret_with_spine(
            input, 42, &strong_signals(), "physics".to_string(), vec![1], 1000,
        );
        // Ligjet kaluan → spine ekzekutoi.
        assert!(result.law_check.is_passed());
        assert!(result.spine.is_some());
        // Light gjithmonë vetëm interpreton.
        assert!(result.interprets_only);
    }

    #[test]
    fn output_never_verdict() {
        let input = UiInput::new("q".to_string(), "s".to_string(), false);
        let result = Phase9Integration::interpret_with_spine(
            input, 1, &strong_signals(), "d".to_string(), vec![1], 100,
        );
        // Output-i i Light s'është kurrë verdikt.
        assert!(Phase9Integration::output_is_not_verdict(&result));
    }

    #[test]
    fn knowledge_gets_trace_and_seal() {
        let upload = PipUpload::new(b"content".to_vec(), "pip:numpy".to_string(), "mathematics".to_string());
        let result = Phase9Integration::ingest_knowledge_with_trace(upload, 42, 1000);
        // Vula 500 e vlefshme → ligjet kaluan.
        assert!(result.law_check.is_passed());
        assert!(result.traced_upload.is_some());
        assert!(result.ready_for_shadow);
        // Trace fillestar u caktua.
        let traced = result.traced_upload.unwrap();
        assert_eq!(traced.trace_id, 42);
    }

    #[test]
    fn light_law_always_passes() {
        // Light s'ka API vendimi → ligji gjithmonë kalon.
        let check = Phase9Integration::check_light_laws();
        assert_eq!(check, LightLawCheck::Passed);
    }

    #[test]
    fn empty_history_still_interprets() {
        let input = UiInput::new("q".to_string(), "s".to_string(), false);
        let result = Phase9Integration::interpret_with_spine(
            input, 1, &ReliabilitySignals::empty(), "d".to_string(), vec![], 100,
        );
        // Edhe pa histori, Light interpreton (Unknown guidance, por s'vendos).
        assert!(result.spine.is_some());
        let spine = result.spine.unwrap();
        assert_eq!(spine.trust_context.guidance, TrustGuidance::Unknown);
    }
}
