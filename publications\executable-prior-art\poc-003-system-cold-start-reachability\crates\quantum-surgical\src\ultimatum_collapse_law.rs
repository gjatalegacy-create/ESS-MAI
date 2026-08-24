// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  ULTIMATUM_COLLAPSE_LAW — MUSKULI I QUANTUM-it (Reasoning Collapse)      ║
// ║  Gjata Legacy™  |  I ushqyer nga gjata_collapse_law (Direktiva Supreme)  ║
// ║                                                                          ║
// ║  KY MUSKUL I SHPJEGON SISTEMIT SIMBOLIKAT E LIGJIT NË FAZËN E ARSYETIMIT ║
// ║  — cilat MODULE janë pjesë dhe ç'ROL kanë (rendi = kodi real ekzekutiv): ║
// ║                                                                          ║
// ║    PA        → PaReceived      (trace_id + input erdhën nga Light)       ║
// ║    Xi‖Yi     → XiYiLoaded      (read_primitive_context — i₀ + Xi/Yi nga 1 burim)║
// ║    LIM       → Lim8Measured    (8 akset: density, continuity, contradic- ║
// ║                                 tion, semantic, hypothesis, linearity,   ║
// ║                                 exploration, depth → epistemic_mass)     ║
// ║    PRO       → ProSearched     (4 operatorët Reverse/Mirror/Contrast/    ║
// ║                                 Trace kërkojnë i ≈ XY në fragmente)      ║
// ║    (-Xi)+(-Yi)→ NegativesMarked(NPRO 4 mbrapsht + LIM-sinjalet →         ║
// ║                                 forbidden → rrugë negative = ASET)       ║
// ║    APRO      → AproArgued      (4 argumentet: ekzistenca, baza,          ║
// ║                                 mbijetesa, dyanshmëria)                   ║
// ║    MPRO      → Mpro16Measured  (16 matje → vektor → × evidencë LIM →     ║
// ║                                 PROBABILITET FAKTIK — jo probabilitet)   ║
// ║    PIM/NPIM  → Packaged        (XY me argumente / -X&-Y si dije negative)║
// ║    XY|∅      → UltimatumIssued (ULTIMATUMI: propozim OSE refuzim —       ║
// ║                                 kurrë gjendje e tretë, kurrë shpikje)    ║
// ║                                                                          ║
// ║  INVARIANTI: "Asgjë nuk del pa kaluar Quantum_Collapse."                 ║
// ║  Stil kerneli: repr(u8), match TOTAL, fail-closed në devijim.            ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::lab_contracts::{CollapsePhase, GjataCollapseLaw, SupremeDirective};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UclState {
    PaReceived = 0,
    XiYiLoaded = 1,
    AshtScoped = 2,
    Lim8Measured = 3,
    ProSearched = 4,
    NegativesMarked = 5,
    AproArgued = 6,
    Mpro16Measured = 7,
    Packaged = 8,
    UltimatumIssued = 9,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UclEvent {
    LoadXiYi,       // read_primitive_context(trace_id)
    ScopeAsht,      // Asht Quantum: kandidatÃ« Shadow tÃ« lidhur me hold-in Besa
    MeasureLim8,    // lim.pre_pro_scan → 8 akset + epistemic_mass
    SearchPro,      // ProEngine::activate — 4 operatorët, i≈XY
    MarkNegatives,  // NproEngine + mark_negative_spaces — (-Xi)+(-Yi)
    ArgueApro,      // Apro::argue — 4 argumentet
    MeasureMpro16,  // Mpro::measure_sealed — 16 → faktik
    Package,        // PIM PassPackage + NPIM NegativeKnowledgePackage
    IssueUltimatum, // CollapseOutcome: Proposed(XY) | Refused — asnjë e tretë
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UclBreach {
    pub from: UclState,
    pub event: UclEvent,
    pub text: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AshtScopeEvidence {
    pub trace_id: String,
    pub split_sha256: String,
    pub besa_hold_sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AshtCommand {
    pub trace_id: String,
    pub split_sha256: String,
    pub besa_hold_sha256: String,
    pub law_seal: u64,
    pub system_laws_seal: u64,
}

/// Capability issued by Asht after Shadow has sealed the bounded evidence set.
/// NPRO and SRK accept this type instead of an unscoped boolean permission.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AshtOperatorDirective {
    pub trace_id: String,
    pub split_sha256: String,
    pub hold_sha256: String,
    pub selection_sha256: String,
    pub allowed_candidate_sha256: Vec<String>,
    pub allowed_operators: Vec<String>,
    pub directive_sha256: String,
}

impl AshtOperatorDirective {
    pub fn new(
        trace_id: String,
        split_sha256: String,
        hold_sha256: String,
        selection_sha256: String,
        allowed_candidate_sha256: Vec<String>,
    ) -> Self {
        let mut directive = Self {
            trace_id,
            split_sha256,
            hold_sha256,
            selection_sha256,
            allowed_candidate_sha256,
            allowed_operators: ["REVERSE", "MIRROR", "CONTRAST", "TRACE"]
                .into_iter()
                .map(str::to_string)
                .collect(),
            directive_sha256: String::new(),
        };
        directive.directive_sha256 = directive.recompute_sha256();
        directive
    }

    pub fn recompute_sha256(&self) -> String {
        let mut material = Vec::new();
        for value in &self.allowed_candidate_sha256 {
            material.extend_from_slice(&(value.len() as u64).to_le_bytes());
            material.extend_from_slice(value.as_bytes());
        }
        for value in &self.allowed_operators {
            material.extend_from_slice(&(value.len() as u64).to_le_bytes());
            material.extend_from_slice(value.as_bytes());
        }
        shadow_contracts::sha256_hex(&[
            b"ESSMAI/ASHT/OPERATOR-DIRECTIVE/V178",
            self.trace_id.as_bytes(),
            self.split_sha256.as_bytes(),
            self.hold_sha256.as_bytes(),
            self.selection_sha256.as_bytes(),
            material.as_slice(),
        ])
    }

    pub fn shape_is_valid(&self) -> bool {
        let required = ["REVERSE", "MIRROR", "CONTRAST", "TRACE"];
        !self.trace_id.is_empty()
            && shadow_contracts::is_canonical_sha256(&self.split_sha256)
            && shadow_contracts::is_canonical_sha256(&self.hold_sha256)
            && shadow_contracts::is_canonical_sha256(&self.selection_sha256)
            && self.allowed_candidate_sha256.len() <= 128
            && self
                .allowed_candidate_sha256
                .iter()
                .all(|value| shadow_contracts::is_canonical_sha256(value))
            && self.allowed_operators.len() == required.len()
            && self
                .allowed_operators
                .iter()
                .map(String::as_str)
                .eq(required)
            && self.directive_sha256 == self.recompute_sha256()
    }

    pub fn allows_operator(&self, name: &str) -> bool {
        self.allowed_operators.iter().any(|allowed| allowed == name)
    }
}

pub struct UltimatumCollapseLaw {
    state: UclState,
    law_seal: u64,
}

impl UltimatumCollapseLaw {
    pub fn commission(d: SupremeDirective) -> Result<Self, UclBreach> {
        match GjataCollapseLaw::verify(&d, CollapsePhase::Reasoning) {
            true => Ok(Self {
                state: UclState::PaReceived,
                law_seal: d.seal,
            }),
            false => Err(UclBreach {
                from: UclState::PaReceived,
                event: UclEvent::LoadXiYi,
                text: "direktivë e pavlefshme: muskuli i Quantum kërkon fazën Reasoning të vulosur",
            }),
        }
    }

    /// TABELA E LIGJIT — rendi i kodit real ekzekutiv, match TOTAL.
    fn transition(state: UclState, event: UclEvent) -> Result<UclState, UclBreach> {
        match (state, event) {
            (UclState::PaReceived, UclEvent::LoadXiYi) => Ok(UclState::XiYiLoaded),
            (UclState::XiYiLoaded, UclEvent::ScopeAsht) => Ok(UclState::AshtScoped),
            (UclState::AshtScoped, UclEvent::MeasureLim8) => Ok(UclState::Lim8Measured),
            (UclState::Lim8Measured, UclEvent::SearchPro) => Ok(UclState::ProSearched),
            (UclState::ProSearched, UclEvent::MarkNegatives) => Ok(UclState::NegativesMarked),
            (UclState::NegativesMarked, UclEvent::ArgueApro) => Ok(UclState::AproArgued),
            (UclState::AproArgued, UclEvent::MeasureMpro16) => Ok(UclState::Mpro16Measured),
            (UclState::Mpro16Measured, UclEvent::Package) => Ok(UclState::Packaged),
            (UclState::Packaged, UclEvent::IssueUltimatum) => Ok(UclState::UltimatumIssued),
            (from, event) => Err(UclBreach {
                from,
                event,
                text: "rend i shkelur: PA→Xi‖Yi→LIM8→PRO→(-Xi)(-Yi)→APRO→MPRO16→PIM/NPIM→Ultimatum",
            }),
        }
    }

    pub fn advance(&mut self, event: UclEvent) -> Result<UclState, UclBreach> {
        let next = Self::transition(self.state, event)?;
        self.state = next;
        Ok(next)
    }

    pub fn command_asht(&mut self, evidence: &AshtScopeEvidence) -> Result<AshtCommand, UclBreach> {
        let valid = !evidence.trace_id.is_empty()
            && shadow_contracts::is_canonical_sha256(&evidence.split_sha256)
            && shadow_contracts::is_canonical_sha256(&evidence.besa_hold_sha256);
        match (self.state, valid) {
            (UclState::XiYiLoaded, true) => {
                self.advance(UclEvent::ScopeAsht)?;
                Ok(AshtCommand {
                    trace_id: evidence.trace_id.clone(),
                    split_sha256: evidence.split_sha256.clone(),
                    besa_hold_sha256: evidence.besa_hold_sha256.clone(),
                    law_seal: self.law_seal,
                    system_laws_seal: crate::lab_contracts::system_laws_seal(),
                })
            }
            (from, _) => Err(UclBreach {
                from,
                event: UclEvent::ScopeAsht,
                text: "UCL cannot command Asht outside a valid XiYiLoaded state",
            }),
        }
    }

    pub fn state(&self) -> UclState {
        self.state
    }

    /// Invarianti: ultimatumi u lëshua — asgjë s'del pa Quantum_Collapse.
    pub fn is_fulfilled(&self) -> bool {
        matches!(self.state, UclState::UltimatumIssued)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lab_contracts::{CollapsePhase as Phase, GjataCollapseLaw as Law};

    #[test]
    fn full_lawful_reasoning_chain() {
        let mut m = UltimatumCollapseLaw::commission(Law::issue(Phase::Reasoning)).unwrap();
        for ev in [
            UclEvent::LoadXiYi,
            UclEvent::ScopeAsht,
            UclEvent::MeasureLim8,
            UclEvent::SearchPro,
            UclEvent::MarkNegatives,
            UclEvent::ArgueApro,
            UclEvent::MeasureMpro16,
            UclEvent::Package,
            UclEvent::IssueUltimatum,
        ] {
            m.advance(ev).unwrap();
        }
        assert!(m.is_fulfilled());
    }

    #[test]
    fn ultimatum_before_mpro_is_breach() {
        let mut m = UltimatumCollapseLaw::commission(Law::issue(Phase::Reasoning)).unwrap();
        m.advance(UclEvent::LoadXiYi).unwrap();
        let b = m.advance(UclEvent::IssueUltimatum).unwrap_err();
        assert!(b.text.contains("rend i shkelur"));
    }

    #[test]
    fn lim_measurement_before_asht_is_breach() {
        let mut m = UltimatumCollapseLaw::commission(Law::issue(Phase::Reasoning)).unwrap();
        m.advance(UclEvent::LoadXiYi).unwrap();
        let breach = m.advance(UclEvent::MeasureLim8).unwrap_err();
        assert_eq!(breach.from, UclState::XiYiLoaded);
    }

    #[test]
    fn wrong_phase_refused() {
        assert!(UltimatumCollapseLaw::commission(Law::issue(Phase::Verification)).is_err());
    }
}
