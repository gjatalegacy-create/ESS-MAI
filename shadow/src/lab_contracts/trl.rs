// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAB_CONTRACTS/TRL.RS — Kontrata e Dëshmisë TRL (e re, minimale)      ║
// ║  GJATA LEGACY™ — ESS-MAI (IDENTIK në Light + Quantum + Shadow)        ║
// ║                                                                          ║
// ║  Kjo kontratë është GJUHA E PËRBASHKËT e matjes TRL — jo presioni.      ║
// ║  Presioni jeton te PressureReport (pressure.rs), JO këtu.             ║
// ║                                                                          ║
// ║  TrlEvidence tregon:                                                   ║
// ║    • çfarë u pa / mat / provua                                        ║
// ║    • sa thellë u testua                                               ║
// ║    • a u kalua cross-domain                                           ║
// ║    • a u ndikua nga presioni (vetëm flamur, JO raporti)              ║
// ║                                                                          ║
// ║  E pavarur nga strukturat e vjetra të secilës platformë: minimale,    ║
// ║  universale, e transferueshme. Zero logjikë platforme. Zero if/else.  ║
// ╚══════════════════════════════════════════════════════════════════════════╝

/// Niveli TRL — shkallë epistemike (jo teknologjike), 0..=4.
/// 0 = pa bazë | 1 = bazë fillestare | 2 = mban presion | 3 = kandidat-faktik
/// 4 = faktuar nga bota (inovacion i prekshëm).
pub type TrlLevel = u8;

/// Dëshmia e matjes TRL — kontratë e pastër, e barabartë për 3 platformat.
///
/// Fushat janë vetëm MATJE dhe GJENDJE — asnjë logjikë, asnjë presion.
/// Çdo platformë e prodhon nga strukturat e veta dhe e konsumon njëlloj.
#[derive(Debug, Clone, PartialEq)]
pub struct TrlEvidence {
    /// Identifikuesi i pretendimit që kjo dëshmi mbështet.
    pub claim_id:           u64,
    /// Niveli TRL i arritur (0..=4).
    pub trl_level:          TrlLevel,
    /// A u kalua validimi në këtë nivel.
    pub trl_passed:         bool,
    /// Sa thellë u testua arsyetimi (numër ciklesh/shkallësh).
    pub reasoning_depth:    u32,
    /// A u testua ndër-domenesh (cross-domain).
    pub cross_domain_tested: bool,
    /// A u ndikua nga presioni (VETËM flamur — raporti shkon te PressureReport).
    pub pressure_applied:   bool,
    /// Besueshmëria e matjes [0,1].
    pub confidence:         f32,
    /// Riprodhueshmëria [0,1].
    pub reproducibility:    f32,
    /// Hash i dëshmisë (gjurmë integriteti) — 32 bajt (SHA-256-class).
    pub evidence_hash:      [u8; 32],
    /// Vula kohore (ns).
    pub timestamp:          u64,
}

impl TrlEvidence {
    /// pending — kur Lab s'u konsultua ende (dëshmia mungon, e ditur).
    /// Zero if — vlera fikse.
    pub fn pending(claim_id: u64) -> Self {
        Self {
            claim_id,
            trl_level:           0,
            trl_passed:          false,
            reasoning_depth:     0,
            cross_domain_tested: false,
            pressure_applied:    false,
            confidence:          0.0,
            reproducibility:     0.0,
            evidence_hash:       [0u8; 32],
            timestamp:           0,
        }
    }

    /// is_pending — a është dëshmi e pakonsultuar? Zero if.
    pub fn is_pending(&self) -> bool {
        let zero_level = self.trl_level == 0;
        let zero_conf  = self.confidence == 0.0;
        let zero_ts    = self.timestamp == 0;
        zero_level & zero_conf & zero_ts
    }

    /// meets_level — a e arrin (të paktën) një nivel TRL të kërkuar? Zero if.
    pub fn meets_level(&self, required: TrlLevel) -> bool {
        let level_ok  = self.trl_level >= required;
        let passed_ok = self.trl_passed;
        level_ok & passed_ok
    }
}

// ════════════════════════════════════════════════════════════════════════════
// TRL-VEKTOR (v1.4.4) — SHTYLLA DEEPTECH E ESS-MAI
// Deeptech matet me TRL. ESS-MAI e mban si VEKTOR tre-dimensional: hyrja
// (Light), arsyetimi (Quantum), verifikimi (Shadow) — secila platformë mban
// dimensionin e vet; pjekuria e sistemit = minimumi (zinxhiri i dobët flet).
// Kjo është ura drejt laboratorëve realë (DigitalLab TRL1–TRL3 në Quantum,
// TrlVerdict deri TRL4 në Shadow) dhe drejt gjuhës së EIC/Startup Albania.
// ════════════════════════════════════════════════════════════════════════════

/// Vektori TRL i një prove — tre dimensione, zero if/else.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrlVector {
    /// TRL i hyrjes (Light: sa e pjekur është indicia i₀ / burimi).
    pub input_trl:        u8,
    /// TRL i arsyetimit (Quantum: evidence.trl_level i LIM/DigitalLab).
    pub reasoning_trl:    u8,
    /// TRL i verifikimit (Shadow: TrlVerdict — prova faktike).
    pub verification_trl: u8,
}

impl TrlVector {
    /// Vektor i ri — dimensionet vendosen nga platformat përkatëse.
    pub fn new(input_trl: u8, reasoning_trl: u8, verification_trl: u8) -> Self {
        Self { input_trl, reasoning_trl, verification_trl }
    }

    /// Pjekuria e sistemit = MINIMUMI i dimensioneve (zinxhiri i dobët).
    pub fn system_trl(&self) -> u8 {
        self.input_trl.min(self.reasoning_trl).min(self.verification_trl)
    }

    /// A ka folur çdo dimension (asnjë zero — vektor i plotë deeptech)?
    pub fn is_complete(&self) -> bool {
        match (self.input_trl, self.reasoning_trl, self.verification_trl) {
            (0, _, _) | (_, 0, _) | (_, _, 0) => false,
            _                                 => true,
        }
    }

    /// Etiketa për printim uniform në të tria platformat.
    pub fn label(&self) -> String {
        format!("in:{} rsn:{} ver:{} → sys:{}",
            self.input_trl, self.reasoning_trl, self.verification_trl, self.system_trl())
    }

    /// v1.4.5 (audit p.4): TRL nga një besueshmëri [0,1] → nivel 0..4.
    /// Ligji i pragjeve (deeptech: TRL rritet me provën): zero if — match
    /// mbi krahasime të renditura. 0 = pa sinjal; 4 = provë e fortë.
    pub fn trl_from_confidence(conf: f32) -> u8 {
        let c = match conf.is_finite() { true => conf, false => 0.0 };
        match (c >= 0.90, c >= 0.75, c >= 0.50, c >= 0.25) {
            (true, _, _, _)        => 4,
            (false, true, _, _)    => 3,
            (false, false, true, _) => 2,
            (false, false, false, true) => 1,
            _                      => 0,
        }
    }

    /// Ndërton vektorin e plotë nga tre burime reale (Light→Quantum→Shadow).
    /// input nga besueshmëria e lingua-s; reasoning nga evidence.trl_level;
    /// verification nga trl_score i Shadow-t. Zinxhiri i plotë deeptech.
    pub fn assemble(input_conf: f32, reasoning_trl: u8, verification_score: f32) -> Self {
        Self {
            input_trl:        Self::trl_from_confidence(input_conf),
            reasoning_trl,
            verification_trl: Self::trl_from_confidence(verification_score),
        }
    }
}

#[cfg(test)]
mod trl_vector_tests {
    use super::TrlVector;

    #[test]
    fn system_trl_is_the_weakest_link() {
        let v = TrlVector::new(4, 3, 2);
        assert_eq!(v.system_trl(), 2, "zinxhiri i dobët flet");
        assert!(v.is_complete());
    }

    #[test]
    fn trl_from_confidence_follows_thresholds() {
        assert_eq!(TrlVector::trl_from_confidence(0.95), 4);
        assert_eq!(TrlVector::trl_from_confidence(0.80), 3);
        assert_eq!(TrlVector::trl_from_confidence(0.60), 2);
        assert_eq!(TrlVector::trl_from_confidence(0.30), 1);
        assert_eq!(TrlVector::trl_from_confidence(0.10), 0);
        assert_eq!(TrlVector::trl_from_confidence(f32::NAN), 0);
    }

    #[test]
    fn assemble_builds_full_chain_from_real_sources() {
        // Light conf 0.8 → in:3; Quantum evidence 2 → rsn:2; Shadow 0.95 → ver:4.
        let v = TrlVector::assemble(0.80, 2, 0.95);
        assert_eq!((v.input_trl, v.reasoning_trl, v.verification_trl), (3, 2, 4));
        assert_eq!(v.system_trl(), 2, "zinxhiri i dobët = Quantum këtu");
        assert!(v.is_complete(), "të tria dimensionet folën");
    }

    #[test]
    fn zero_dimension_means_incomplete_vector() {
        assert!(!TrlVector::new(3, 0, 2).is_complete());
        assert!(!TrlVector::new(0, 1, 1).is_complete());
        assert_eq!(TrlVector::new(3, 0, 2).system_trl(), 0);
        assert!(TrlVector::new(1, 1, 1).is_complete());
    }
}
