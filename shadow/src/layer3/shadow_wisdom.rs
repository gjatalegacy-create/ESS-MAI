// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAYER3/SHADOW_WISDOM.RS — Kujtesa Epistemike (Shadow)               ║
// ║  GJATA LEGACY™ — ESS-MAI Shadow                          (LAYER 3)    ║
// ║                                                                          ║
// ║  shadow_wisdom është SIDECAR — mban shënim çdo lëvizje në Shadow dhe e  ║
// ║  dërgon te magazina e përbashkët. Ndiqet SPECIFIKISHT për primitiv      ║
// ║  dhe legacy. E lidh çdo lëvizje me precedent dhe e ekspozon më vonë.    ║
// ║                                                                          ║
// ║  Kjo është MAGAZINA E PËRBASHKËT që të tria platformat e përdorin: çdo  ║
// ║  WisdomNote (nga light/quantum/shadow_wisdom) ruhet këtu; kushdo e      ║
// ║  lexon kur duhet të rifreskojë.                                        ║
// ║                                                                          ║
// ║  KUFIRI: shadow_wisdom NUK është vendim — vetëm lexues/regjistrues.    ║
// ║  Zero logjikë vendimi. Zero if/else.                                  ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::lab_contracts_v11::wisdom::{WisdomNote, WisdomSource, WisdomKind};

// ─────────────────────────────────────────────────────────────────────────────
// WISDOM WAREHOUSE — magazina e përbashkët (të 3 platformat e përdorin)
// ─────────────────────────────────────────────────────────────────────────────

/// Magazina e urtësisë — APPEND-ONLY, e përbashkët për të tria platformat.
/// Çdo WisdomNote (light/quantum/shadow) ruhet këtu; kushdo e lexon.
#[derive(Debug, Clone)]
pub struct WisdomWarehouse {
    notes: Vec<WisdomNote>,
}

impl WisdomWarehouse {
    /// new — magazinë bosh. Zero if.
    pub fn new() -> Self {
        Self { notes: Vec::new() }
    }

    /// store — ruan një shënim urtësie (nga çdo platformë). Zero if — append.
    pub fn store(&mut self, note: WisdomNote) {
        self.notes.push(note);
    }

    /// count — sa shënime ka magazina. Zero if.
    pub fn count(&self) -> usize {
        self.notes.len()
    }

    /// notes_for — të gjitha shënimet për një ref_id. Zero if — filter.
    pub fn notes_for(&self, ref_id: u64) -> Vec<&WisdomNote> {
        self.notes.iter().filter(|n| n.ref_id == ref_id).collect()
    }

    /// count_by_source — sa shënime nga një platformë. Zero if — filter.
    pub fn count_by_source(&self, source: WisdomSource) -> usize {
        self.notes.iter()
            .filter(|n| (n.source as u8) == (source as u8))
            .count()
    }

    /// birth_notes — të gjitha vëzhgimet e lindjeve (primitiv/legacy). Zero if.
    pub fn birth_notes(&self) -> Vec<&WisdomNote> {
        self.notes.iter().filter(|n| n.is_birth()).collect()
    }

    /// significant_count — sa shënime të rëndësishme. Zero if — filter.
    pub fn significant_count(&self) -> usize {
        self.notes.iter().filter(|n| n.is_significant()).count()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// SHADOW WISDOM — sidecar që mban shënim lëvizjet
// ─────────────────────────────────────────────────────────────────────────────

pub struct ShadowWisdom;

impl ShadowWisdom {
    /// note_movement — mban shënim një lëvizje të përgjithshme në Shadow. Zero if.
    pub fn note_movement(
        ref_id: u64,
        territory: String,
        description: String,
        significance: f32,
        at: u64,
    ) -> WisdomNote {
        WisdomNote::new(
            ref_id,
            WisdomSource::Shadow,
            WisdomKind::Movement,
            description,
            territory,
            significance.clamp(0.0, 1.0),
            at,
        )
    }

    /// note_primitive — mban shënim specifikisht për një primitiv. Zero if.
    pub fn note_primitive(
        primitive_id: u64,
        territory: String,
        at: u64,
    ) -> WisdomNote {
        WisdomNote::new(
            primitive_id,
            WisdomSource::Shadow,
            WisdomKind::PrimitiveBirth,
            "primitiv u verifikua nga Shadow".to_string(),
            territory,
            0.8,
            at,
        )
    }

    /// note_legacy — mban shënim specifikisht për një legacy. Zero if.
    pub fn note_legacy(
        legacy_id: u64,
        territory: String,
        at: u64,
    ) -> WisdomNote {
        WisdomNote::new(
            legacy_id,
            WisdomSource::Shadow,
            WisdomKind::LegacyBirth,
            "legacy u kristalizua nga Shadow".to_string(),
            territory,
            0.9,
            at,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn warehouse_stores_and_counts() {
        let mut wh = WisdomWarehouse::new();
        assert_eq!(wh.count(), 0);
        wh.store(ShadowWisdom::note_primitive(1, "physics".to_string(), 100));
        wh.store(ShadowWisdom::note_legacy(2, "chemistry".to_string(), 200));
        assert_eq!(wh.count(), 2);
    }

    #[test]
    fn warehouse_shared_across_sources() {
        // Magazina pranon nga të 3 platformat.
        let mut wh = WisdomWarehouse::new();
        wh.store(WisdomNote::new(1, WisdomSource::Light, WisdomKind::TrustFormation, "t".to_string(), "d".to_string(), 0.6, 100));
        wh.store(WisdomNote::new(1, WisdomSource::Quantum, WisdomKind::PrimitiveBirth, "p".to_string(), "d".to_string(), 0.8, 100));
        wh.store(WisdomNote::new(1, WisdomSource::Shadow, WisdomKind::Movement, "m".to_string(), "d".to_string(), 0.5, 100));
        assert_eq!(wh.count_by_source(WisdomSource::Light), 1);
        assert_eq!(wh.count_by_source(WisdomSource::Quantum), 1);
        assert_eq!(wh.count_by_source(WisdomSource::Shadow), 1);
    }

    #[test]
    fn notes_for_ref() {
        let mut wh = WisdomWarehouse::new();
        wh.store(ShadowWisdom::note_primitive(5, "d".to_string(), 100));
        wh.store(ShadowWisdom::note_primitive(5, "d".to_string(), 200));
        wh.store(ShadowWisdom::note_primitive(9, "d".to_string(), 300));
        assert_eq!(wh.notes_for(5).len(), 2);
        assert_eq!(wh.notes_for(9).len(), 1);
    }

    #[test]
    fn birth_notes_filtered() {
        let mut wh = WisdomWarehouse::new();
        wh.store(ShadowWisdom::note_primitive(1, "d".to_string(), 100));
        wh.store(ShadowWisdom::note_movement(2, "d".to_string(), "m".to_string(), 0.5, 200));
        // Vetëm primitivi është birth.
        assert_eq!(wh.birth_notes().len(), 1);
    }

    #[test]
    fn shadow_notes_primitive_and_legacy() {
        let prim = ShadowWisdom::note_primitive(1, "physics".to_string(), 100);
        assert_eq!(prim.kind, WisdomKind::PrimitiveBirth);
        assert_eq!(prim.source, WisdomSource::Shadow);
        let leg = ShadowWisdom::note_legacy(2, "physics".to_string(), 200);
        assert_eq!(leg.kind, WisdomKind::LegacyBirth);
    }

    #[test]
    fn significant_count_works() {
        let mut wh = WisdomWarehouse::new();
        wh.store(ShadowWisdom::note_primitive(1, "d".to_string(), 100)); // 0.8 significant
        wh.store(ShadowWisdom::note_movement(2, "d".to_string(), "m".to_string(), 0.3, 200)); // not
        assert_eq!(wh.significant_count(), 1);
    }
}
