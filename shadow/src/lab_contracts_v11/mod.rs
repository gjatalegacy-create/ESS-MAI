// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAB_CONTRACTS_V11/MOD.RS — Gjuha e Shtresës 3 (    )               ║
// ║  GJATA LEGACY™ — ESS-MAI (BYTE-FOR-BYTE IDENTIK në 3 platformat)      ║
// ║                                                                          ║
// ║  Zgjerimi i gjuhës për Shtresën 3, PA prekur lab_contracts (    ).     ║
// ║  Mban VETËM kontrata: kufij hardware, urtësi, raport sentineli, paketë. ║
// ║  ZERO logjikë platforme. ZERO presion. Byte-for-byte identik në 3.    ║
// ║                                                                          ║
// ║  KUFIRI ABSOLUT: ky skedar identik në Light, Quantum, Shadow.         ║
// ╚══════════════════════════════════════════════════════════════════════════╝

pub mod hardware;  // HardwareBounds + DeviceSnapshot (kufijtë fizikë)
pub mod wisdom;    // WisdomNote (vëzhgim, jo vendim)
pub mod watch;     // NightWatchReport (7 shtresa info nga web-via-device)
pub mod packet;    // Layer3Packet (gjuha e komunikimit)

// ── Ri-eksportim i sheshtë ──────────────────────────────────────────────────
pub use hardware::{HardwareBounds, DeviceSnapshot};
pub use wisdom::{WisdomNote, WisdomSource, WisdomKind};
pub use watch::{NightWatchReport, SevenLayerInfo};
pub use packet::{Layer3Packet, Layer3Context, Layer3PayloadKind};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn device_snapshot_idle_safe() {
        let d = DeviceSnapshot::idle();
        assert_eq!(d.energy_margin, 1.0);
        // Pa device real, ram_total=0 → ratio i mbrojtur.
        assert_eq!(d.ram_free_ratio(), 0.0);
    }

    #[test]
    fn device_snapshot_safety() {
        let d = DeviceSnapshot {
            ram_free_bytes: 4_000_000_000, ram_total_bytes: 8_000_000_000,
            cpu_load: 0.3, cores_active: 4, cores_total: 8,
            energy_margin: 0.8, on_battery: false,
            temp_c: 50.0, temp_limit_c: 90.0, timestamp: 100,
        };
        assert!((d.ram_free_ratio() - 0.5).abs() < 1e-6);
        assert!(d.is_safe_to_deepen()); // energ 0.8 + termik OK
    }

    #[test]
    fn device_unsafe_low_energy() {
        let mut d = DeviceSnapshot::idle();
        d.ram_total_bytes = 8_000_000_000;
        d.energy_margin = 0.1;  // nën 0.20
        d.temp_limit_c = 90.0;
        assert!(!d.is_safe_to_deepen());
    }

    #[test]
    fn hardware_bounds_minimal() {
        let b = HardwareBounds::minimal();
        assert!(!b.allows_deepening());
        assert_eq!(b.effective_parallel(), 1);
    }

    #[test]
    fn hardware_bounds_allows_deepening() {
        let b = HardwareBounds {
            ram_budget_bytes: 256_000_000, parallel_limit: 4,
            pressure_push: 0.7, deep_allowed: true, safety_margin: 0.2,
        };
        assert!(b.allows_deepening());
        assert_eq!(b.effective_parallel(), 4);
    }

    #[test]
    fn parallel_limit_clamped() {
        let b = HardwareBounds {
            ram_budget_bytes: 1, parallel_limit: 10, // mbi 4
            pressure_push: 0.5, deep_allowed: false, safety_margin: 0.5,
        };
        assert_eq!(b.effective_parallel(), 4); // clamp në 4
    }

    #[test]
    fn wisdom_note_birth_detection() {
        let note = WisdomNote::new(
            1, WisdomSource::Quantum, WisdomKind::PrimitiveBirth,
            "found via NReverse".to_string(), "physics".to_string(), 0.8, 100,
        );
        assert!(note.is_birth());
        assert!(note.is_significant());
        assert_eq!(note.source, WisdomSource::Quantum);
    }

    #[test]
    fn wisdom_note_trust_not_birth() {
        let note = WisdomNote::new(
            1, WisdomSource::Light, WisdomKind::TrustFormation,
            "trust built from 3 successes".to_string(), "d".to_string(), 0.3, 100,
        );
        assert!(!note.is_birth());
        assert!(!note.is_significant()); // 0.3 < 0.5
    }

    #[test]
    fn seven_layer_aggregate() {
        let info = SevenLayerInfo {
            policy_alignment: 0.8, literature_support: 0.7, source_consensus: 0.9,
            recency: 0.6, credibility: 0.85, contradiction_free: 0.75, cross_coverage: 0.7,
        };
        let agg = info.aggregate();
        assert!(agg > 0.0 && agg <= 1.0);
        assert!(info.all_layers_present());
    }

    #[test]
    fn empty_seven_layer_no_layers() {
        let info = SevenLayerInfo::empty();
        assert!(!info.all_layers_present());
        assert_eq!(info.aggregate(), 0.0);
    }

    #[test]
    fn night_watch_report_pending() {
        let r = NightWatchReport::pending(1);
        assert!(!r.is_complete()); // pa shtresa/burime
        assert!(r.via_device);     // gjithmonë via device
    }

    #[test]
    fn night_watch_complete() {
        let r = NightWatchReport {
            claim_id: 1,
            layers: SevenLayerInfo {
                policy_alignment: 0.8, literature_support: 0.7, source_consensus: 0.9,
                recency: 0.6, credibility: 0.85, contradiction_free: 0.75, cross_coverage: 0.7,
            },
            via_device: true, sources_count: 5, gathered_at: 100,
        };
        assert!(r.is_complete());
        assert!(r.confidence() > 0.0);
    }

    #[test]
    fn layer3_packet_seal() {
        let ctx = Layer3Context::new(1, 2, 3, "physics".to_string(), 4, 5);
        let pkt = Layer3Packet::new(ctx, Layer3PayloadKind::HardwareBounds, 1, 100);
        assert!(pkt.seal_ok());
        assert_eq!(pkt.seal, 0xA451);
        assert!(pkt.context.has_wisdom()); // wisdom_ref=5 != 0
    }

    #[test]
    fn layer3_context_no_wisdom() {
        let ctx = Layer3Context::new(1, 2, 3, "d".to_string(), 4, 0);
        assert!(!ctx.has_wisdom()); // wisdom_ref=0
    }
}
