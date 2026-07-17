// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAYER3/LAYER3_FLOW.RS — Orkestruesi i Shtresës 3 (Light)            ║
// ║  GJATA LEGACY™ — ESS-MAI Light                            (LAYER 3)    ║
// ║                                                                          ║
// ║  Hook i PASTËR që lidh Shtresën 3 të Light-it:                         ║
// ║    UI input → ui_channel (gjurmim) → neural_law (kuptim) →             ║
// ║    light_wisdom (observim besueshmërie) → Layer3Packet → magazina.     ║
// ║                                                                          ║
// ║  Komunikimi ndër-platformë: prodhon Layer3Packet (vula 500) me urtësinë ║
// ║  e besueshmërisë që i dërgohet magazinës së Shadow-it.                 ║
// ║                                                                          ║
// ║  KUFIRI: Light interpreton dhe transporton — kurrë s'vendos. Çdo gjë    ║
// ║  këtu është gjurmim/kuptim/observim. Zero if/else.                    ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::lab_contracts_v11::packet::{Layer3Packet, Layer3Context, Layer3PayloadKind};
use crate::lab_contracts_v11::wisdom::WisdomNote;
use crate::layer3::ui_channel::{UiChannel, UiInput, TracedInput};
use crate::layer3::light_wisdom::LightWisdom;

// ─────────────────────────────────────────────────────────────────────────────
// LIGHT LAYER3 RESULT — rezultati i orkestrimit në hyrje
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct LightLayer3Result {
    /// Inputi i gjurmuar (me kuptim nga neural_law).
    pub traced_input:  TracedInput,
    /// Shënimi i urtësisë së besueshmërisë.
    pub wisdom_note:   WisdomNote,
    /// Paketa për magazinë (vula 500).
    pub wisdom_packet: Layer3Packet,
}

// ─────────────────────────────────────────────────────────────────────────────
// LIGHT LAYER3 FLOW — orkestron hyrjen UI → gjurmim → urtësi
// ─────────────────────────────────────────────────────────────────────────────

pub struct LightLayer3Flow;

impl LightLayer3Flow {
    /// ingest_from_ui — pranon input nga UI, e gjurmon, observon besueshmërinë,
    /// e paketon për magazinë.
    ///
    /// Rrjedha: UI input → ui_channel (gjurmim + neural_law kuptim) →
    /// light_wisdom (observim besueshmërie) → Layer3Packet për magazinë.
    /// Zero if.
    pub fn ingest_from_ui(
        input: UiInput,
        trace_id: u64,
        trust_score: f32,
        territory: String,
        at: u64,
    ) -> LightLayer3Result {
        // 1. Inputi merr gjurmim menjëherë (aty ku futet, lind gjurma).
        let traced_input = UiChannel::receive_input(input, trace_id, at);

        // 2. light_wisdom observon SI u formua besueshmëria e inputit.
        let wisdom_note = LightWisdom::observe_input_trust(
            trace_id, trust_score, territory.clone(), at,
        );

        // 3. Paketohet për magazinë (vula 500).
        let context = Layer3Context::new(
            trace_id,
            0,            // claim_id ende i pacaktuar
            0,            // session
            territory,
            trace_id,     // packet_id
            trace_id,     // wisdom_ref
        );
        let wisdom_packet = Layer3Packet::new(
            context, Layer3PayloadKind::WisdomNote, trace_id, at,
        );

        LightLayer3Result {
            traced_input,
            wisdom_note,
            wisdom_packet,
        }
    }

    /// emit_output_wisdom — observon besueshmërinë e një outputi dhe e paketon.
    /// Zero if.
    pub fn emit_output_wisdom(
        output_id: u64,
        trust_score: f32,
        territory: String,
        trace_id: u64,
        at: u64,
    ) -> (WisdomNote, Layer3Packet) {
        let note = LightWisdom::observe_output_trust(output_id, trust_score, territory.clone(), at);
        let context = Layer3Context::new(trace_id, output_id, 0, territory, trace_id, output_id);
        let packet = Layer3Packet::new(context, Layer3PayloadKind::WisdomNote, trace_id, at);
        (note, packet)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lab_contracts_v11::wisdom::{WisdomSource, WisdomKind};

    #[test]
    fn ingests_ui_input_with_trace() {
        let input = UiInput::new("explore with Nura".to_string(), "s1".to_string(), false);
        let result = LightLayer3Flow::ingest_from_ui(input, 42, 0.7, "physics".to_string(), 1000);
        // Inputi u gjurmua.
        assert_eq!(result.traced_input.trace_id, 42);
        // Urtësia e besueshmërisë u krijua.
        assert_eq!(result.wisdom_note.source, WisdomSource::Light);
        assert_eq!(result.wisdom_note.kind, WisdomKind::TrustFormation);
        // Paketa e vulosur.
        assert!(result.wisdom_packet.seal_ok());
    }

    #[test]
    fn output_wisdom_emitted() {
        let (note, packet) = LightLayer3Flow::emit_output_wisdom(
            5, 0.9, "chemistry".to_string(), 1, 100,
        );
        assert_eq!(note.source, WisdomSource::Light);
        assert!(note.observation.contains("output"));
        assert!(packet.seal_ok());
        assert!(packet.context.has_wisdom());
    }

    #[test]
    fn trace_born_at_input() {
        // Aty ku futet inputi, lind gjurma (kanali UI).
        let input = UiInput::new("test".to_string(), "".to_string(), true);
        let result = LightLayer3Flow::ingest_from_ui(input, 7, 0.5, "d".to_string(), 100);
        assert_eq!(result.traced_input.trace_id, 7);
        assert!(result.traced_input.input.deep_research);
    }

    #[test]
    fn light_only_interprets_not_decides() {
        // Çdo gjë këtu është gjurmim/kuptim/observim — asnjë vendim.
        let input = UiInput::new("q".to_string(), "s".to_string(), false);
        let result = LightLayer3Flow::ingest_from_ui(input, 1, 0.6, "d".to_string(), 100);
        // Rezultati: gjurmë + kuptim + urtësi, jo verdikt.
        assert!(!result.traced_input.meaning.meaning.is_empty());
        assert_eq!(result.wisdom_note.source, WisdomSource::Light);
    }
}
