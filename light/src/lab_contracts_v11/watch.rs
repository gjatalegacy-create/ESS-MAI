// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAB_CONTRACTS_V11/WATCH.RS — Raporti i Sentinelit (Shtresa 3)       ║
// ║  GJATA LEGACY™ — ESS-MAI (IDENTIK në Light + Quantum + Shadow)        ║
// ║                                                                          ║
// ║  NightWatchReport është gjuha e përbashkët e sinjaleve që sentineli i   ║
// ║  jashtëm (shadow_night_watch) mbledh — 7 shtresa info të gjera e të     ║
// ║  plota nga web-i NËPËRMJET device (kurrë duke prekur web-in drejtpërdrejt).
// ║                                                                          ║
// ║  KUFIRI: ky raport është SINJAL i strukturuar — sentineli e prodhon,    ║
// ║  por gjykimi 0/1 i takon vetë night_watch (gjykatës i ndarë), JO këtu.  ║
// ║  Kontrata mban vetëm formën. Zero logjikë platforme. Zero if/else.    ║
// ╚══════════════════════════════════════════════════════════════════════════╝

/// 7 shtresat e informacionit që sentineli mbledh (nga web-via-device).
/// Secila është një skor [0,1] besueshmërie/mbështetjeje.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SevenLayerInfo {
    /// 1. Përditësimet e politikave (policy).
    pub policy_alignment:   f32,
    /// 2. Mbështetja literaturore/akademike.
    pub literature_support: f32,
    /// 3. Konsensusi i burimeve.
    pub source_consensus:   f32,
    /// 4. Freskia e informacionit (sa i ri).
    pub recency:            f32,
    /// 5. Kredibiliteti i burimeve.
    pub credibility:        f32,
    /// 6. Kontradiktat e gjetura (i invertuar — më pak = më mirë).
    pub contradiction_free: f32,
    /// 7. Mbulimi ndër-domenesh.
    pub cross_coverage:     f32,
}

impl SevenLayerInfo {
    /// empty — pa info (sentineli s'mblodhi ende). Zero if.
    pub fn empty() -> Self {
        Self {
            policy_alignment:   0.0,
            literature_support: 0.0,
            source_consensus:   0.0,
            recency:            0.0,
            credibility:        0.0,
            contradiction_free: 0.0,
            cross_coverage:     0.0,
        }
    }

    /// aggregate — skori i agreguar i 7 shtresave [0,1]. Zero if — formulë.
    /// Peshim i barabartë (secila shtresë 1/7).
    pub fn aggregate(&self) -> f32 {
        let sum = self.policy_alignment
            + self.literature_support
            + self.source_consensus
            + self.recency
            + self.credibility
            + self.contradiction_free
            + self.cross_coverage;
        (sum / 7.0).clamp(0.0, 1.0)
    }

    /// all_layers_present — a kanë të 7 shtresat sinjal (jo zero)? Zero if.
    pub fn all_layers_present(&self) -> bool {
        let l1 = self.policy_alignment > 0.0;
        let l2 = self.literature_support > 0.0;
        let l3 = self.source_consensus > 0.0;
        let l4 = self.recency > 0.0;
        let l5 = self.credibility > 0.0;
        let l6 = self.contradiction_free > 0.0;
        let l7 = self.cross_coverage > 0.0;
        l1 & l2 & l3 & l4 & l5 & l6 & l7
    }
}

/// Raporti i plotë i sentinelit — sinjalet që night_watch do gjykojë.
#[derive(Debug, Clone, PartialEq)]
pub struct NightWatchReport {
    /// Identifikuesi i pretendimit nën vëzhgim.
    pub claim_id:      u64,
    /// 7 shtresat e informacionit.
    pub layers:        SevenLayerInfo,
    /// A u mblodhën sinjalet nëpërmjet device (kurrë web i drejtpërdrejtë).
    pub via_device:    bool,
    /// Numri i burimeve të konsultuara.
    pub sources_count: u32,
    /// Vula kohore (ns).
    pub gathered_at:   u64,
}

impl NightWatchReport {
    /// pending — raport i pakonsultuar ende. Zero if.
    pub fn pending(claim_id: u64) -> Self {
        Self {
            claim_id,
            layers:        SevenLayerInfo::empty(),
            via_device:    true,  // gjithmonë nëpërmjet device (parim)
            sources_count: 0,
            gathered_at:   0,
        }
    }

    /// is_complete — a është raporti i plotë (7 shtresa + burime + via device)? Zero if.
    pub fn is_complete(&self) -> bool {
        let layers_ok  = self.layers.all_layers_present();
        let sources_ok = self.sources_count > 0;
        let device_ok  = self.via_device;
        layers_ok & sources_ok & device_ok
    }

    /// confidence — besueshmëria e raportit [0,1]. Zero if.
    pub fn confidence(&self) -> f32 {
        self.layers.aggregate()
    }
}
