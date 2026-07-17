// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAYER3/LAYER3_FLOW.RS — Orkestruesi i Shtresës 3 (Shadow)           ║
// ║  GJATA LEGACY™ — ESS-MAI Shadow                          (LAYER 3)    ║
// ║                                                                          ║
// ║  Hook i PASTËR që lidh Shtresën 3 të Shadow-it: sentineli i jashtëm +   ║
// ║  urtësia + magazina e përbashkët, përmes Layer3Packet (vula 500).      ║
// ║                                                                          ║
// ║  Rrjedha:                                                              ║
// ║    Layer3Packet (NightWatchReport) → verifiko vulën → night_watch       ║
// ║    gjykon 0/1 → shadow_wisdom regjistron lëvizjen → magazina e ruan.   ║
// ║                                                                          ║
// ║  KUFIRI: orkestron VETËM Shtresën 3; night_watch s'hyn te judiciary,    ║
// ║  wisdom s'është vendim. Komunikimi ndër-platformë me kontratë.        ║
// ║  Zero if/else.                                                        ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::lab_contracts_v11::watch::NightWatchReport;
use crate::lab_contracts_v11::packet::{Layer3Packet, Layer3PayloadKind};
use crate::lab_contracts_v11::wisdom::WisdomNote;
use crate::layer3::shadow_night_watch::{NightWatch, WatchVerdict};
use crate::layer3::shadow_wisdom::{ShadowWisdom, WisdomWarehouse};

// ─────────────────────────────────────────────────────────────────────────────
// LAYER3 FLOW RESULT — rezultati i orkestrimit
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Layer3FlowResult {
    /// Verdikti i sentinelit (0/1 mbi 7 shtresa).
    pub watch_verdict:  WatchVerdict,
    /// A ishte paketa hyrëse e vlefshme (vula 500 + lloji).
    pub packet_valid:   bool,
    /// Shënimi i urtësisë i regjistruar.
    pub wisdom_note:    WisdomNote,
}

// ─────────────────────────────────────────────────────────────────────────────
// LAYER3 FLOW — orkestron Shtresën 3 të Shadow-it
// ─────────────────────────────────────────────────────────────────────────────

pub struct Layer3Flow;

impl Layer3Flow {
    /// process_watch — pranon raport sentineli përmes Layer3Packet, gjykon,
    /// regjistron urtësinë në magazinë.
    ///
    /// Verifikon vulën 500 + llojin NightWatchReport, pastaj night_watch gjykon,
    /// pastaj shadow_wisdom regjistron lëvizjen. Zero if — match mbi vlefshmërinë.
    pub fn process_watch(
        packet: &Layer3Packet,
        report: &NightWatchReport,
        warehouse: &mut WisdomWarehouse,
        at: u64,
    ) -> Layer3FlowResult {
        // Verifiko zarfin: vula 500 + lloji i ngarkesës.
        let seal_ok = packet.seal_ok();
        let payload_ok = (packet.payload_kind as u8) == (Layer3PayloadKind::NightWatchReport as u8);
        let packet_valid = seal_ok & payload_ok;

        // Sentineli gjykon (gjithmonë; vlefshmëria regjistrohet veçmas).
        let watch_verdict = NightWatch::judge(report);

        // shadow_wisdom regjistron lëvizjen e gjykimit.
        let significance = watch_verdict.confidence;
        let description = format!(
            "night_watch gjykoi claim={} → bit={}",
            report.claim_id, watch_verdict.bit
        );
        let wisdom_note = ShadowWisdom::note_movement(
            report.claim_id,
            packet.context.territory.clone(),
            description,
            significance,
            at,
        );

        // Magazina e ruan (e përbashkët për të 3 platformat).
        warehouse.store(wisdom_note.clone());

        Layer3FlowResult {
            watch_verdict,
            packet_valid,
            wisdom_note,
        }
    }

    /// absorb_external_wisdom — magazina thith urtësi nga platformat e tjera.
    ///
    /// light_wisdom dhe quantum_wisdom dërgojnë WisdomNote këtu; magazina
    /// e Shadow-it i ruan (pika ku të 3 takohen). Zero if.
    pub fn absorb_external_wisdom(warehouse: &mut WisdomWarehouse, note: WisdomNote) {
        warehouse.store(note);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lab_contracts_v11::watch::SevenLayerInfo;
    use crate::lab_contracts_v11::packet::Layer3Context;
    use crate::lab_contracts_v11::wisdom::{WisdomSource, WisdomKind};

    fn full_report() -> NightWatchReport {
        NightWatchReport {
            claim_id: 1,
            layers: SevenLayerInfo {
                policy_alignment: 0.8, literature_support: 0.7, source_consensus: 0.9,
                recency: 0.6, credibility: 0.85, contradiction_free: 0.75, cross_coverage: 0.7,
            },
            via_device: true, sources_count: 5, gathered_at: 100,
        }
    }

    fn watch_packet() -> Layer3Packet {
        let ctx = Layer3Context::new(1, 1, 1, "physics".to_string(), 1, 0);
        Layer3Packet::new(ctx, Layer3PayloadKind::NightWatchReport, 1, 100)
    }

    #[test]
    fn processes_valid_watch_packet() {
        let mut wh = WisdomWarehouse::new();
        let result = Layer3Flow::process_watch(&watch_packet(), &full_report(), &mut wh, 1000);
        assert!(result.packet_valid);
        assert_eq!(result.watch_verdict.bit, 1); // raport i fortë
        // Urtësia u regjistrua në magazinë.
        assert_eq!(wh.count(), 1);
    }

    #[test]
    fn wrong_payload_invalid_but_judges() {
        let ctx = Layer3Context::new(1, 1, 1, "d".to_string(), 1, 0);
        let wrong = Layer3Packet::new(ctx, Layer3PayloadKind::WisdomNote, 1, 100);
        let mut wh = WisdomWarehouse::new();
        let result = Layer3Flow::process_watch(&wrong, &full_report(), &mut wh, 1000);
        assert!(!result.packet_valid); // lloj i gabuar
        // Por gjykimi prapë kryhet + urtësia regjistrohet.
        assert_eq!(wh.count(), 1);
    }

    #[test]
    fn warehouse_absorbs_external() {
        let mut wh = WisdomWarehouse::new();
        // Urtësi nga Light.
        let light_note = WisdomNote::new(1, WisdomSource::Light, WisdomKind::TrustFormation, "t".to_string(), "d".to_string(), 0.7, 100);
        Layer3Flow::absorb_external_wisdom(&mut wh, light_note);
        // Urtësi nga Quantum.
        let quantum_note = WisdomNote::new(2, WisdomSource::Quantum, WisdomKind::PrimitiveBirth, "p".to_string(), "d".to_string(), 0.8, 100);
        Layer3Flow::absorb_external_wisdom(&mut wh, quantum_note);
        assert_eq!(wh.count(), 2);
        assert_eq!(wh.count_by_source(WisdomSource::Light), 1);
        assert_eq!(wh.count_by_source(WisdomSource::Quantum), 1);
    }

    #[test]
    fn three_platforms_meet_in_warehouse() {
        // Pika ku të 3 takohen: magazina e përbashkët.
        let mut wh = WisdomWarehouse::new();
        Layer3Flow::process_watch(&watch_packet(), &full_report(), &mut wh, 1000); // Shadow
        Layer3Flow::absorb_external_wisdom(&mut wh,
            WisdomNote::new(2, WisdomSource::Light, WisdomKind::TrustFormation, "t".to_string(), "d".to_string(), 0.7, 100));
        Layer3Flow::absorb_external_wisdom(&mut wh,
            WisdomNote::new(3, WisdomSource::Quantum, WisdomKind::PrimitiveBirth, "p".to_string(), "d".to_string(), 0.8, 100));
        assert_eq!(wh.count(), 3);
    }
}
