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
    PaReceived      = 0,
    XiYiLoaded      = 1,
    Lim8Measured    = 2,
    ProSearched     = 3,
    NegativesMarked = 4,
    AproArgued      = 5,
    Mpro16Measured  = 6,
    Packaged        = 7,
    UltimatumIssued = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UclEvent {
    LoadXiYi,      // read_primitive_context(trace_id)
    MeasureLim8,   // lim.pre_pro_scan → 8 akset + epistemic_mass
    SearchPro,     // ProEngine::activate — 4 operatorët, i≈XY
    MarkNegatives, // NproEngine + mark_negative_spaces — (-Xi)+(-Yi)
    ArgueApro,     // Apro::argue — 4 argumentet
    MeasureMpro16, // Mpro::measure_sealed — 16 → faktik
    Package,       // PIM PassPackage + NPIM NegativeKnowledgePackage
    IssueUltimatum,// CollapseOutcome: Proposed(XY) | Refused — asnjë e tretë
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UclBreach {
    pub from:  UclState,
    pub event: UclEvent,
    pub text:  &'static str,
}

pub struct UltimatumCollapseLaw {
    state: UclState,
}

impl UltimatumCollapseLaw {
    pub fn commission(d: SupremeDirective) -> Result<Self, UclBreach> {
        match GjataCollapseLaw::verify(&d, CollapsePhase::Reasoning) {
            true  => Ok(Self { state: UclState::PaReceived }),
            false => Err(UclBreach {
                from:  UclState::PaReceived,
                event: UclEvent::LoadXiYi,
                text:  "direktivë e pavlefshme: muskuli i Quantum kërkon fazën Reasoning të vulosur",
            }),
        }
    }

    /// TABELA E LIGJIT — rendi i kodit real ekzekutiv, match TOTAL.
    fn transition(state: UclState, event: UclEvent) -> Result<UclState, UclBreach> {
        match (state, event) {
            (UclState::PaReceived,      UclEvent::LoadXiYi)       => Ok(UclState::XiYiLoaded),
            (UclState::XiYiLoaded,      UclEvent::MeasureLim8)    => Ok(UclState::Lim8Measured),
            (UclState::Lim8Measured,    UclEvent::SearchPro)      => Ok(UclState::ProSearched),
            (UclState::ProSearched,     UclEvent::MarkNegatives)  => Ok(UclState::NegativesMarked),
            (UclState::NegativesMarked, UclEvent::ArgueApro)      => Ok(UclState::AproArgued),
            (UclState::AproArgued,      UclEvent::MeasureMpro16)  => Ok(UclState::Mpro16Measured),
            (UclState::Mpro16Measured,  UclEvent::Package)        => Ok(UclState::Packaged),
            (UclState::Packaged,        UclEvent::IssueUltimatum) => Ok(UclState::UltimatumIssued),
            (from, event) => Err(UclBreach {
                from, event,
                text: "rend i shkelur: PA→Xi‖Yi→LIM8→PRO→(-Xi)(-Yi)→APRO→MPRO16→PIM/NPIM→Ultimatum",
            }),
        }
    }

    pub fn advance(&mut self, event: UclEvent) -> Result<UclState, UclBreach> {
        let next = Self::transition(self.state, event)?;
        self.state = next;
        Ok(next)
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
    use crate::lab_contracts::{GjataCollapseLaw as Law, CollapsePhase as Phase};

    #[test]
    fn full_lawful_reasoning_chain() {
        let mut m = UltimatumCollapseLaw::commission(Law::issue(Phase::Reasoning)).unwrap();
        for ev in [UclEvent::LoadXiYi, UclEvent::MeasureLim8, UclEvent::SearchPro,
                   UclEvent::MarkNegatives, UclEvent::ArgueApro, UclEvent::MeasureMpro16,
                   UclEvent::Package, UclEvent::IssueUltimatum] {
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
    fn wrong_phase_refused() {
        assert!(UltimatumCollapseLaw::commission(Law::issue(Phase::Verification)).is_err());
    }
}
