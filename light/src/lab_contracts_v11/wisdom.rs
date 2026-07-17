// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAB_CONTRACTS_V11/WISDOM.RS — Urtësia e Vëzhguar (Shtresa 3)        ║
// ║  GJATA LEGACY™ — ESS-MAI (IDENTIK në Light + Quantum + Shadow)        ║
// ║                                                                          ║
// ║  WisdomNote është gjuha e përbashkët e VËZHGIMIT — jo vendimi.         ║
// ║  Të tre wisdom-modulet prodhojnë WisdomNote dhe magazina e Shadow-it     ║
// ║  i ruan; të tria platformat e lexojnë kur duhet të rifreskojnë.        ║
// ║                                                                          ║
// ║  • light_wisdom   → observon SI një input/output mori besueshmëri      ║
// ║  • quantum_wisdom  → observon SI u gjet një primitiv/legacy            ║
// ║  • shadow_wisdom   → observon çdo lëvizje (primitiv + legacy)          ║
// ║                                                                          ║
// ║  KUFIRI: WisdomNote NUK vendos, NUK gjykon — vetëm regjistron çfarë     ║
// ║  ndodhi dhe pse. Zero logjikë platforme. Zero if/else.               ║
// ╚══════════════════════════════════════════════════════════════════════════╝

/// Burimi i urtësisë — cila platformë e vëzhgoi.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WisdomSource {
    Light,    // besueshmëri (input/output)
    Quantum,  // si u gjet primitiv/legacy
    Shadow,   // çdo lëvizje (primitiv + legacy)
}

impl WisdomSource {
    pub fn label(self) -> &'static str {
        match self {
            WisdomSource::Light   => "LIGHT_WISDOM",
            WisdomSource::Quantum => "QUANTUM_WISDOM",
            WisdomSource::Shadow  => "SHADOW_WISDOM",
        }
    }
}

/// Çfarë lloj urtësie u vëzhgua.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WisdomKind {
    /// Si një input/output mori besueshmëri (Light).
    TrustFormation,
    /// Si u gjet një primitiv (Quantum/Shadow).
    PrimitiveBirth,
    /// Si u krijua një legacy (Quantum/Shadow).
    LegacyBirth,
    /// Një lëvizje e përgjithshme në sistem.
    Movement,
}

impl WisdomKind {
    pub fn label(self) -> &'static str {
        match self {
            WisdomKind::TrustFormation => "TRUST_FORMATION",
            WisdomKind::PrimitiveBirth => "PRIMITIVE_BIRTH",
            WisdomKind::LegacyBirth    => "LEGACY_BIRTH",
            WisdomKind::Movement       => "MOVEMENT",
        }
    }
}

/// Një shënim urtësie — vëzhgim i pandryshueshëm i një ngjarjeje në sistem.
#[derive(Debug, Clone, PartialEq)]
pub struct WisdomNote {
    /// Identifikuesi i pretendimit/entitetit të vëzhguar.
    pub ref_id:       u64,
    /// Kush e vëzhgoi.
    pub source:       WisdomSource,
    /// Çfarë lloji urtësie.
    pub kind:         WisdomKind,
    /// Përshkrimi i SI/PSE ndodhi (gjurma e vëzhgimit).
    pub observation:  String,
    /// Domeni/territori ku ndodhi.
    pub territory:    String,
    /// Forca/rëndësia e vëzhgimit [0,1].
    pub significance: f32,
    /// Vula kohore (ns).
    pub observed_at:  u64,
}

impl WisdomNote {
    /// new — krijon një shënim urtësie. Zero if.
    pub fn new(
        ref_id: u64,
        source: WisdomSource,
        kind: WisdomKind,
        observation: String,
        territory: String,
        significance: f32,
        observed_at: u64,
    ) -> Self {
        Self {
            ref_id,
            source,
            kind,
            observation,
            territory,
            significance: significance.clamp(0.0, 1.0),
            observed_at,
        }
    }

    /// is_birth — a vëzhgon lindjen e një primitivi/legacy? Zero if.
    pub fn is_birth(&self) -> bool {
        let primitive = (self.kind as u8) == (WisdomKind::PrimitiveBirth as u8);
        let legacy    = (self.kind as u8) == (WisdomKind::LegacyBirth as u8);
        primitive | legacy
    }

    /// is_significant — a është vëzhgim i rëndësishëm (mbi prag)? Zero if.
    pub fn is_significant(&self) -> bool {
        self.significance >= 0.50
    }
}
