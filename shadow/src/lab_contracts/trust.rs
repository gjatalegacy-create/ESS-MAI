// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAB_CONTRACTS/TRUST.RS — Kontrata e Kontekstit të Besimit            ║
// ║  GJATA LEGACY™ — ESS-MAI (IDENTIK në Light + Quantum + Shadow)        ║
// ║                                                                          ║
// ║  TrustContext është konteksti i besimit që LIGHT e prodhon e përditëson.║
// ║  Mban DY shtresa paralele:                                            ║
// ║    • trust_score        → sa shpesh një rrugë funksionon (intuitë)     ║
// ║    • verification_score → sa herë u provua/verifikua/mbijetoi (rigorozitet)
// ║                                                                          ║
// ║  Light nuk vendos — Light e thotë sa i besueshëm duket materiali,      ║
// ║  sa herë ka mbijetuar, a ka histori verifikimi. Zero logjikë, zero if. ║
// ╚══════════════════════════════════════════════════════════════════════════╝

/// Orientimi i butë i Light-it (JO vendim — vetëm shtytje drejt cilësisë).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrustGuidance {
    Recommended,  // histori e fortë suksesi
    Neutral,      // pa sinjal të qartë
    Risky,        // histori dështimesh
    Unknown,      // pa histori
}

impl TrustGuidance {
    pub fn label(self) -> &'static str {
        match self {
            TrustGuidance::Recommended => "RECOMMENDED",
            TrustGuidance::Neutral     => "NEUTRAL",
            TrustGuidance::Risky       => "RISKY",
            TrustGuidance::Unknown     => "UNKNOWN",
        }
    }
}

/// Konteksti i plotë i besimit — prodhuar nga Light, i lexueshëm nga të tria.
#[derive(Debug, Clone, PartialEq)]
pub struct TrustContext {
    /// Identifikuesi i gjurmës (lidh kontekstin me rrjedhën).
    pub trace_id:           u64,
    /// Domeni në të cilin u vlerësua.
    pub domain:             String,
    /// Sa shpesh kjo rrugë ka funksionuar [0,1] (intuitë historike).
    pub trust_score:        f32,
    /// Stabiliteti ndër-domenesh / në kohë [0,1].
    pub reliability_score:  f32,
    /// Sa herë u provua dhe mbijetoi nën kontroll [0,1] (rigorozitet).
    pub verification_score: f32,
    /// Referenca te historiku (id-të e verifikimeve/legacy të mëparshme).
    pub history_refs:       Vec<u64>,
    /// Orientimi i butë (jo vendim).
    pub guidance:           TrustGuidance,
}

impl TrustContext {
    /// empty — kontekst pa histori (input i ri, i panjohur). Zero if.
    pub fn empty(trace_id: u64, domain: String) -> Self {
        Self {
            trace_id,
            domain,
            trust_score:        0.0,
            reliability_score:  0.0,
            verification_score: 0.0,
            history_refs:       Vec::new(),
            guidance:           TrustGuidance::Unknown,
        }
    }

    /// has_history — a ekziston histori për këtë rrugë? Zero if.
    pub fn has_history(&self) -> bool {
        !self.history_refs.is_empty()
    }

    /// combined_strength — forca e përgjithshme [0,1]: peshë e dy shtresave.
    /// trust (intuitë) 0.40 + reliability 0.25 + verification (rigorozitet) 0.35.
    /// Zero if — formula.
    pub fn combined_strength(&self) -> f32 {
        let t = self.trust_score.clamp(0.0, 1.0);
        let r = self.reliability_score.clamp(0.0, 1.0);
        let v = self.verification_score.clamp(0.0, 1.0);
        (t * 0.40 + r * 0.25 + v * 0.35).clamp(0.0, 1.0)
    }
}
