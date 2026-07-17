// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAB_CONTRACTS/EVIDENCE.RS — Kontrata e Paketës së Provës             ║
// ║  GJATA LEGACY™ — ESS-MAI (IDENTIK në Light + Quantum + Shadow)        ║
// ║                                                                          ║
// ║  EvidencePackage është DOSJA E VERIFIKIMIT që Quantum dërgon te Shadow. ║
// ║  Quantum nuk dërgon një supozim — dërgon një paketë të plotë prove.    ║
// ║                                                                          ║
// ║  Mban: pretendimin, arsyetimin, dëshminë TRL, kontekstin e besimit,    ║
// ║  zinxhirin e provës, kontrollet ndër-domenesh. Presioni vjen veçmas    ║
// ║  (PressureReport). Zero logjikë platforme. Zero if/else.             ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::lab_contracts::trl::TrlEvidence;
use crate::lab_contracts::trust::TrustContext;
use crate::lab_contracts::pressure::PressureReport;

/// Kategoria e provës fizike (për TRL 4 — faktim nga bota).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceKind {
    /// Vetëm arsyetim/simulim (TRL 1-3).
    Reasoned,
    /// Provë dokumentare (foto/video/dokumente — TRL 4).
    Documented,
    /// Demonstrim operacional (live — niveli më i lartë).
    Operational,
}

impl EvidenceKind {
    pub fn label(self) -> &'static str {
        match self {
            EvidenceKind::Reasoned    => "REASONED",
            EvidenceKind::Documented  => "DOCUMENTED",
            EvidenceKind::Operational => "OPERATIONAL",
        }
    }
}

/// Paketa e plotë e provës — dosja që Shadow gjykon.
#[derive(Debug, Clone, PartialEq)]
pub struct EvidencePackage {
    /// Identifikuesi i pretendimit.
    pub claim_id:          u64,
    /// Vetë pretendimi (çfarë po pretendohet).
    pub claim:             String,
    /// Arsyetimi mbështetës.
    pub reasoning:         String,
    /// Dëshmia TRL (matja).
    pub trl_evidence:      TrlEvidence,
    /// Konteksti i besimit (nga Light).
    pub trust_context:     TrustContext,
    /// Raporti i presionit (nga Quantum NPRO/NPIM).
    pub pressure_report:   PressureReport,
    /// Zinxhiri i referencave të provës (hash/id të dokumentacionit).
    pub evidence_chain:    Vec<String>,
    /// Kontrollet ndër-domenesh të kaluara.
    pub cross_domain_checks: Vec<String>,
    /// Lloji i provës.
    pub kind:              EvidenceKind,
}

impl EvidencePackage {
    /// has_physical_evidence — a ka provë fizike (jo vetëm arsyetim)? Zero if.
    pub fn has_physical_evidence(&self) -> bool {
        let documented  = (self.kind as u8) == (EvidenceKind::Documented as u8);
        let operational = (self.kind as u8) == (EvidenceKind::Operational as u8);
        let chain_nonempty = !self.evidence_chain.is_empty();
        (documented | operational) & chain_nonempty
    }

    /// cross_domain_count — sa kontrolle ndër-domenesh u kaluan. Zero if.
    pub fn cross_domain_count(&self) -> usize {
        self.cross_domain_checks.len()
    }

    /// completeness — sa e plotë është paketa [0,1]: pretendim + arsyetim +
    /// TRL jo-pending + prova + cross-domain. Zero if — numërim boolean.
    pub fn completeness(&self) -> f32 {
        let has_claim     = (!self.claim.is_empty()) as u32;
        let has_reasoning = (!self.reasoning.is_empty()) as u32;
        let trl_ready     = (!self.trl_evidence.is_pending()) as u32;
        let has_evidence  = (!self.evidence_chain.is_empty()) as u32;
        let has_cross     = (!self.cross_domain_checks.is_empty()) as u32;
        let sum = has_claim + has_reasoning + trl_ready + has_evidence + has_cross;
        (sum as f32 / 5.0).clamp(0.0, 1.0)
    }
}
