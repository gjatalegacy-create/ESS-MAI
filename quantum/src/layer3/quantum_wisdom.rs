// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAYER3/QUANTUM_WISDOM.RS — Urtësia e Quantum-it (Shtresa 3)         ║
// ║  GJATA LEGACY™ — ESS-MAI Quantum                          (LAYER 3)    ║
// ║                                                                          ║
// ║  quantum_wisdom VETËM OBSERVON si u gjet një primitiv ose legacy.       ║
// ║  Prodhon WisdomNote (gjuha e përbashkët) dhe ia dërgon magazinës së     ║
// ║  Shadow-it. Pjesë e thelbit të Shtresës 3 lab-to-lab.                  ║
// ║                                                                          ║
// ║  KUFIRI: quantum_wisdom NUK vendos, NUK gjykon — vetëm vëzhgon e        ║
// ║  regjistron SI ndodhi zbulimi. Zero logjikë vendimi. Zero if/else.    ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::lab_contracts_v11::wisdom::{WisdomNote, WisdomSource, WisdomKind};

pub struct QuantumWisdom;

impl QuantumWisdom {
    /// observe_primitive — vëzhgon si u gjet një primitiv. Zero if.
    ///
    /// operator: cili operator e gjeti (NReverse, NMirror, etj.).
    /// pressure_survived: sa presion mbijetoi (forca e zbulimit).
    pub fn observe_primitive(
        primitive_id: u64,
        operator: &str,
        territory: String,
        pressure_survived: f32,
        observed_at: u64,
    ) -> WisdomNote {
        WisdomNote::new(
            primitive_id,
            WisdomSource::Quantum,
            WisdomKind::PrimitiveBirth,
            format!("primitiv u gjet via {} (presion mbijetuar {:.2})", operator, pressure_survived),
            territory,
            pressure_survived.clamp(0.0, 1.0),
            observed_at,
        )
    }

    /// observe_legacy — vëzhgon si u krijua një legacy. Zero if.
    pub fn observe_legacy(
        legacy_id: u64,
        depth: u32,
        territory: String,
        maturity: f32,
        observed_at: u64,
    ) -> WisdomNote {
        WisdomNote::new(
            legacy_id,
            WisdomSource::Quantum,
            WisdomKind::LegacyBirth,
            format!("legacy u krijua në thellësi {} (pjekuri {:.2})", depth, maturity),
            territory,
            maturity.clamp(0.0, 1.0),
            observed_at,
        )
    }

    /// is_strong_discovery — a është zbulim i fortë (mbi prag)? Zero if.
    pub fn is_strong_discovery(note: &WisdomNote) -> bool {
        let is_birth = note.is_birth();
        let significant = note.is_significant();
        is_birth & significant
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn observes_primitive_birth() {
        let note = QuantumWisdom::observe_primitive(
            1, "NReverse", "physics".to_string(), 0.85, 100,
        );
        assert_eq!(note.source, WisdomSource::Quantum);
        assert_eq!(note.kind, WisdomKind::PrimitiveBirth);
        assert!(note.is_birth());
        assert!(QuantumWisdom::is_strong_discovery(&note));
    }

    #[test]
    fn observes_legacy_birth() {
        let note = QuantumWisdom::observe_legacy(
            2, 5, "chemistry".to_string(), 0.9, 200,
        );
        assert_eq!(note.kind, WisdomKind::LegacyBirth);
        assert!(note.is_birth());
    }

    #[test]
    fn weak_discovery_not_strong() {
        let note = QuantumWisdom::observe_primitive(
            1, "NTrace", "d".to_string(), 0.3, 100,
        );
        // Significance 0.3 < 0.5 → jo i fortë.
        assert!(!QuantumWisdom::is_strong_discovery(&note));
    }

    #[test]
    fn observation_only_no_decision() {
        // quantum_wisdom prodhon vetëm WisdomNote — asnjë verdikt.
        let note = QuantumWisdom::observe_primitive(1, "NMirror", "d".to_string(), 0.7, 100);
        // Është vetëm vëzhgim (source Quantum, kind birth).
        assert_eq!(note.source, WisdomSource::Quantum);
    }
}
