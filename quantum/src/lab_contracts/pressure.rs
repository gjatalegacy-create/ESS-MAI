// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAB_CONTRACTS/PRESSURE.RS — Kontrata e Presionit                     ║
// ║  GJATA LEGACY™ — ESS-MAI (IDENTIK në Light + Quantum + Shadow)        ║
// ║                                                                          ║
// ║  PressureReport është KONTRATA E PRESIONIT — këtu jeton presioni,       ║
// ║  JO te TrlEvidence. Çdo platformë e mbush sipas motorit të vet:        ║
// ║    • Quantum → NPRO / NPIM / Abyssal Probe (presion i hapur agresiv)   ║
// ║    • Light   → trust pressure / trace hardening (presion i butë)       ║
// ║    • Shadow  → adversarial / evidence gate (presion sovran)            ║
// ║                                                                          ║
// ║  Vetë raporti është gjuhë e përbashkët: kontradikta, dobësi, dështime, ║
// ║  rruga e presionit. Zero logjikë platforme. Zero if/else.             ║
// ╚══════════════════════════════════════════════════════════════════════════╝

/// Burimi i presionit — cila platformë/motor e aplikoi.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PressureSource {
    /// Presion i hapur agresiv (Quantum NPRO/NPIM/Abyssal).
    QuantumNpro,
    /// Presion i butë para-verifikimi (Light trace/trust hardening).
    LightSoft,
    /// Presion sovran verifikues (Shadow adversarial/evidence).
    ShadowSovereign,
}

impl PressureSource {
    pub fn label(self) -> &'static str {
        match self {
            PressureSource::QuantumNpro     => "QUANTUM_NPRO",
            PressureSource::LightSoft       => "LIGHT_SOFT",
            PressureSource::ShadowSovereign => "SHADOW_SOVEREIGN",
        }
    }
}

/// Raporti i presionit — rezultati i shtyrjes së një ideje drejt kufirit.
#[derive(Debug, Clone, PartialEq)]
pub struct PressureReport {
    /// Pretendimi që u vu nën presion.
    pub claim_id:           u64,
    /// Kush e aplikoi presionin.
    pub source:             PressureSource,
    /// Sa kontradikta u gjetën [0,1] (dendësi kontradiktash).
    pub contradiction_score: f32,
    /// Sa dobësi u ekspozuan [0,1].
    pub weakness_score:     f32,
    /// Mënyrat e dështimit të zbuluara (përshkrime të shkurtra).
    pub failure_modes:      Vec<String>,
    /// Rruga e presionit që sistemi ndoqi (gjurmë e shtyrjes).
    pub pressure_path:      String,
    /// Sa nivel presioni e mbijetoi ideja (0..=3: jo / lehtë / mesatar / ekstrem).
    pub survived_level:     u8,
}

impl PressureReport {
    /// none — pa presion të aplikuar ende (gjendje fillestare). Zero if.
    pub fn none(claim_id: u64, source: PressureSource) -> Self {
        Self {
            claim_id,
            source,
            contradiction_score: 0.0,
            weakness_score:      0.0,
            failure_modes:       Vec::new(),
            pressure_path:       String::new(),
            survived_level:      0,
        }
    }

    /// withstood — a e mbijetoi ideja presionin pa dobësi të mëdha? Zero if.
    ///
    /// Mbijetesë = pak kontradikta AND pak dobësi AND asnjë mënyrë dështimi.
    /// (Pragjet janë universale; gjykimi final i takon Shadow-it.)
    pub fn withstood(&self) -> bool {
        let low_contradiction = self.contradiction_score <= 0.30;
        let low_weakness      = self.weakness_score <= 0.30;
        let no_failures       = self.failure_modes.is_empty();
        low_contradiction & low_weakness & no_failures
    }

    /// stress_index — sa "i sforcuar" ishte presioni [0,1]: kombinim i dy skoreve.
    /// Zero if — formula.
    pub fn stress_index(&self) -> f32 {
        let c = self.contradiction_score.clamp(0.0, 1.0);
        let w = self.weakness_score.clamp(0.0, 1.0);
        ((c + w) * 0.5).clamp(0.0, 1.0)
    }
}
