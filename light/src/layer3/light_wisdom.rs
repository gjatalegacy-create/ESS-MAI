// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAYER3/LIGHT_WISDOM.RS — Urtësia e Light-it (Shtresa 3)             ║
// ║  GJATA LEGACY™ — ESS-MAI Light                            (LAYER 3)    ║
// ║                                                                          ║
// ║  light_wisdom OBSERVON për besueshmërinë: kush input dhe output merr     ║
// ║  besueshmëri dhe SI e ka marrë në Light. Prodhon WisdomNote dhe ia       ║
// ║  dërgon magazinës së përbashkët. Pjesë e thelbit të Shtresës 3.        ║
// ║                                                                          ║
// ║  KUFIRI: light_wisdom NUK vendos — vetëm vëzhgon SI u formua besimi.    ║
// ║  Zero logjikë vendimi. Zero if/else.                                  ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::lab_contracts_v11::wisdom::{WisdomNote, WisdomSource, WisdomKind};

pub struct LightWisdom;

impl LightWisdom {
    /// observe_trust — vëzhgon SI një input/output mori besueshmëri. Zero if.
    ///
    /// trust_score: besueshmëria e marrë.
    /// basis: mbi çfarë u ndërtua (psh "3 suksese në 2 domene").
    pub fn observe_trust(
        ref_id: u64,
        trust_score: f32,
        basis: String,
        territory: String,
        observed_at: u64,
    ) -> WisdomNote {
        WisdomNote::new(
            ref_id,
            WisdomSource::Light,
            WisdomKind::TrustFormation,
            format!("besueshmëri {:.2} u formua nga: {}", trust_score, basis),
            territory,
            trust_score.clamp(0.0, 1.0),
            observed_at,
        )
    }

    /// observe_input_trust — vëzhgim specifik për një input. Zero if.
    pub fn observe_input_trust(
        input_id: u64,
        trust_score: f32,
        territory: String,
        at: u64,
    ) -> WisdomNote {
        Self::observe_trust(
            input_id, trust_score,
            "input — vlerësim historik".to_string(),
            territory, at,
        )
    }

    /// observe_output_trust — vëzhgim specifik për një output. Zero if.
    pub fn observe_output_trust(
        output_id: u64,
        trust_score: f32,
        territory: String,
        at: u64,
    ) -> WisdomNote {
        Self::observe_trust(
            output_id, trust_score,
            "output — pasuruar me besueshmëri".to_string(),
            territory, at,
        )
    }

    /// is_trusted_observation — a vëzhgon besueshmëri të lartë? Zero if.
    pub fn is_trusted_observation(note: &WisdomNote) -> bool {
        let is_trust = (note.kind as u8) == (WisdomKind::TrustFormation as u8);
        let significant = note.is_significant();
        is_trust & significant
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn observes_trust_formation() {
        let note = LightWisdom::observe_trust(
            1, 0.85, "4 suksese në 3 domene".to_string(), "physics".to_string(), 100,
        );
        assert_eq!(note.source, WisdomSource::Light);
        assert_eq!(note.kind, WisdomKind::TrustFormation);
        assert!(LightWisdom::is_trusted_observation(&note));
    }

    #[test]
    fn observes_input_trust() {
        let note = LightWisdom::observe_input_trust(1, 0.7, "d".to_string(), 100);
        assert!(note.observation.contains("input"));
        assert_eq!(note.source, WisdomSource::Light);
    }

    #[test]
    fn observes_output_trust() {
        let note = LightWisdom::observe_output_trust(2, 0.9, "d".to_string(), 200);
        assert!(note.observation.contains("output"));
    }

    #[test]
    fn low_trust_not_trusted_observation() {
        let note = LightWisdom::observe_trust(1, 0.3, "weak".to_string(), "d".to_string(), 100);
        // 0.3 < 0.5 → jo i besuar.
        assert!(!LightWisdom::is_trusted_observation(&note));
    }

    #[test]
    fn observation_only_no_decision() {
        // light_wisdom prodhon vetëm WisdomNote — asnjë vendim.
        let note = LightWisdom::observe_trust(1, 0.6, "b".to_string(), "d".to_string(), 100);
        assert_eq!(note.source, WisdomSource::Light);
        assert_eq!(note.kind, WisdomKind::TrustFormation);
    }
}
