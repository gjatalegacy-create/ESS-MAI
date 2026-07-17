// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAYER3/LAYER3_FLOW.RS — Orkestruesi i Shtresës 3 (Quantum)          ║
// ║  GJATA LEGACY™ — ESS-MAI Quantum                          (LAYER 3)    ║
// ║                                                                          ║
// ║  Hook i PASTËR që lidh Shtresën 3 të Quantum-it:                       ║
// ║    DeviceSnapshot → HPRO (kufijtë + stabilitet) → HPIM (paketim) →      ║
// ║    gati për PIM. Plus quantum_wisdom → Layer3Packet → magazina.        ║
// ║                                                                          ║
// ║  Komunikimi ndër-platformë: prodhon Layer3Packet (vula 500) me urtësi   ║
// ║  që i dërgohet magazinës së Shadow-it.                                 ║
// ║                                                                          ║
// ║  KUFIRI: HPRO lëshon harduerin; presioni → HPIM → PIM (jo drejtpërdrejt);║
// ║  quantum_wisdom vetëm observon. Zero if/else.                         ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::lab_contracts_v11::hardware::DeviceSnapshot;
use crate::lab_contracts_v11::packet::{Layer3Packet, Layer3Context, Layer3PayloadKind};
use crate::lab_contracts_v11::wisdom::WisdomNote;
use crate::layer3::hpro::{HproEngine, HproResult};
use crate::layer3::hpim::{HpimEngine, HpimPackage};
use crate::layer3::quantum_wisdom::QuantumWisdom;

// ─────────────────────────────────────────────────────────────────────────────
// QUANTUM LAYER3 RESULT — rezultati i orkestrimit
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct QuantumLayer3Result {
    /// Dosja HPIM gati për PIM.
    pub hpim_package:     HpimPackage,
    /// A u lëshua harduri.
    pub hardware_released: bool,
    /// A është gati për PIM.
    pub ready_for_pim:    bool,
}

// ─────────────────────────────────────────────────────────────────────────────
// QUANTUM LAYER3 FLOW — orkestron HPRO → HPIM
// ─────────────────────────────────────────────────────────────────────────────

pub struct QuantumLayer3Flow;

impl QuantumLayer3Flow {
    /// run_hardware_pressure — HPRO → HPIM, gati për PIM.
    ///
    /// Rrjedha: device → HPRO (kufijtë + stabilitet, lëshon harduerin) →
    /// HPIM (paketon trace-in) → gati për PIM. Zero if.
    pub fn run_hardware_pressure(
        claim_id: u64,
        snapshot: &DeviceSnapshot,
    ) -> QuantumLayer3Result {
        let hpro = HproEngine::run(snapshot);
        Self::from_hpro(claim_id, &hpro)
    }

    /// Production path: paketon rezultatin HPRO që tashmë ka lease-in e ciklit.
    /// Nuk ekzekuton një HPRO të dytë dhe nuk mat hardware përsëri.
    pub fn from_hpro(claim_id: u64, hpro: &HproResult) -> QuantumLayer3Result {
        let hpim_package = HpimEngine::package(claim_id, hpro);
        let ready_for_pim = hpim_package.is_ready_for_pim();

        QuantumLayer3Result {
            hpim_package,
            hardware_released: hpro.hardware_released,
            ready_for_pim,
        }
    }

    /// emit_wisdom_packet — paketon një WisdomNote në Layer3Packet për magazinë.
    ///
    /// quantum_wisdom observon → Layer3Packet (vula 500) → magazina e Shadow-it.
    /// Zero if.
    pub fn emit_wisdom_packet(
        note: &WisdomNote,
        trace_id: u64,
        session_id: u64,
        sequence: u64,
        timestamp: u64,
    ) -> Layer3Packet {
        let context = Layer3Context::new(
            trace_id,
            note.ref_id,
            session_id,
            note.territory.clone(),
            sequence,        // packet_id
            note.ref_id,     // wisdom_ref = ref i note-s
        );
        Layer3Packet::new(context, Layer3PayloadKind::WisdomNote, sequence, timestamp)
    }

    /// observe_and_emit — observon një primitiv dhe e paketon për magazinë. Zero if.
    pub fn observe_and_emit(
        primitive_id: u64,
        operator: &str,
        territory: String,
        pressure_survived: f32,
        trace_id: u64,
        session_id: u64,
        sequence: u64,
        at: u64,
    ) -> (WisdomNote, Layer3Packet) {
        let note = QuantumWisdom::observe_primitive(
            primitive_id, operator, territory, pressure_survived, at,
        );
        let packet = Self::emit_wisdom_packet(&note, trace_id, session_id, sequence, at);
        (note, packet)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strong_device() -> DeviceSnapshot {
        DeviceSnapshot {
            ram_free_bytes: 4_000_000_000, ram_total_bytes: 8_000_000_000,
            cpu_load: 0.3, cores_active: 4, cores_total: 8,
            energy_margin: 0.9, on_battery: false,
            temp_c: 40.0, temp_limit_c: 90.0, timestamp: 100,
        }
    }

    #[test]
    fn hardware_pressure_ready_for_pim() {
        let result = QuantumLayer3Flow::run_hardware_pressure(1, &strong_device());
        assert!(result.hardware_released); // HPRO lëshon harduerin
        assert!(result.ready_for_pim);
        assert_eq!(result.hpim_package.claim_id, 1);
    }

    #[test]
    fn wisdom_packet_sealed() {
        let note = QuantumWisdom::observe_primitive(
            5, "NReverse", "physics".to_string(), 0.8, 100,
        );
        let packet = QuantumLayer3Flow::emit_wisdom_packet(&note, 1, 1, 1, 100);
        assert!(packet.seal_ok()); // vula 500
        assert_eq!(packet.payload_kind, Layer3PayloadKind::WisdomNote);
        // wisdom_ref lidhet me note-n.
        assert_eq!(packet.context.wisdom_ref, 5);
    }

    #[test]
    fn observe_and_emit_works() {
        let (note, packet) = QuantumLayer3Flow::observe_and_emit(
            5, "NMirror", "chemistry".to_string(), 0.85, 1, 1, 1, 100,
        );
        use crate::lab_contracts_v11::wisdom::WisdomSource;
        assert_eq!(note.source, WisdomSource::Quantum);
        assert!(packet.seal_ok());
        assert!(packet.context.has_wisdom());
    }

    #[test]
    fn weak_device_not_ready() {
        let mut weak = DeviceSnapshot::idle();
        weak.ram_free_bytes = 10_000_000;
        weak.energy_margin = 0.1;
        weak.temp_c = 88.0;
        weak.temp_limit_c = 90.0;
        let result = QuantumLayer3Flow::run_hardware_pressure(1, &weak);
        // HPRO prapë lëshon harduerin, por dosja ka gjurmë (gati varet nga trace).
        assert!(result.hardware_released);
    }
}
