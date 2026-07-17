// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  VEREFIED_DIARY_SUPREMELAW — MUSKULI I SHADOW-it (Verification Collapse) ║
// ║  Gjata Legacy™  |  I ushqyer nga gjata_collapse_law (Direktiva Supreme)  ║
// ║                                                                          ║
// ║  KY MUSKUL I SHPJEGON SISTEMIT SE ÇFARË GJYKOHET — NË GJUHË KODI:        ║
// ║  JO vetëm rrjedha Y (outputi). Në bazë të i0 gjykohet PROCESI i          ║
// ║  ultimatum_collapse_law: propozimi i Quantum kundrejt indicies së        ║
// ║  Light-it. Struktura NUK duhet të bjerë:                                 ║
// ║                                                                          ║
// ║    X (i0/PA+Xi‖Yi nga Light) → i besueshëm?   [inputi gjykohet]          ║
// ║    PROCESI (ultimatumi: XY|refuzim, faktik, negativat) → i ligjshëm?     ║
// ║    Y (outputi siç e ka rrjedha) → i vulosur?  [rrjedha ekzistuese]       ║
// ║                                                                          ║
// ║  GJENDJET (simbolet e fazës Verification):                               ║
// ║    AwaitingAnchor   — ditari pret sinjalin e Light                       ║
// ║    AnchorTrusted    — X/i0: PA+(Xi,Yi) u besua si indicie                ║
// ║    ProposalReceived — Y-propozimi: XY (ose refuzimi) i Quantum erdhi     ║
// ║    ProcessJudged    — PROCESI u gjykua mbi i0 (jo vetëm Y)               ║
// ║    StructureHeld    — struktura QËNDROI (negativat u vulosën si aset)    ║
// ║    DiaryVerdict     — D ∈ {0,1}: ditari i verifikuar vulosi              ║
// ║                                                                          ║
// ║  INVARIANTI: "Asgjë nuk pranohet pa Sovereign_Verification_Collapse."    ║
// ║  Shadow s'krijon ide, s'arsyeton nga e para — PROVON ose RRËZON.        ║
// ║  Stil kerneli: repr(u8), match TOTAL, ditar i shkruar (white-box).       ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::lab_contracts::{CollapsePhase, GjataCollapseLaw, SupremeDirective};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VdsState {
    AwaitingAnchor   = 0,
    AnchorTrusted    = 1,
    ProposalReceived = 2,
    ProcessJudged    = 3,
    StructureHeld    = 4,
    DiaryVerdict     = 5,
}

impl VdsState {
    pub fn label(&self) -> &'static str {
        match self {
            Self::AwaitingAnchor   => "AWAITING_ANCHOR",
            Self::AnchorTrusted    => "ANCHOR_TRUSTED(X/i0)",
            Self::ProposalReceived => "PROPOSAL_RECEIVED(Y)",
            Self::ProcessJudged    => "PROCESS_JUDGED(ultimatum⊢i0)",
            Self::StructureHeld    => "STRUCTURE_HELD",
            Self::DiaryVerdict     => "DIARY_VERDICT(D)",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VdsEvent {
    TrustAnchor,     // PA+(Xi,Yi) nga Light — inputi X/i0 u besua
    ReceiveProposal, // XY | refuzimi i mbartur — Y-propozimi erdhi
    JudgeProcess,    // gjykimi i PROCESIT të ultimatum-it mbi i0
    HoldStructure,   // negativat u vulosën; struktura s'ra
    SealDiary,       // verdikti sovran doli — ditari vulos D
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VdsBreach {
    pub from:  VdsState,
    pub event: VdsEvent,
    pub text:  &'static str,
}

pub struct VerefiedDiarySupremelaw {
    state: VdsState,
}

impl VerefiedDiarySupremelaw {
    pub fn commission(d: SupremeDirective) -> Result<Self, VdsBreach> {
        match GjataCollapseLaw::verify(&d, CollapsePhase::Verification) {
            true  => Ok(Self { state: VdsState::AwaitingAnchor }),
            false => Err(VdsBreach {
                from:  VdsState::AwaitingAnchor,
                event: VdsEvent::TrustAnchor,
                text:  "direktivë e pavlefshme: muskuli i Shadow kërkon fazën Verification të vulosur",
            }),
        }
    }

    /// TABELA E LIGJIT — gjykimi strukturor, match TOTAL.
    fn transition(state: VdsState, event: VdsEvent) -> Result<VdsState, VdsBreach> {
        match (state, event) {
            (VdsState::AwaitingAnchor,   VdsEvent::TrustAnchor)     => Ok(VdsState::AnchorTrusted),
            (VdsState::AnchorTrusted,    VdsEvent::ReceiveProposal) => Ok(VdsState::ProposalReceived),
            (VdsState::ProposalReceived, VdsEvent::JudgeProcess)    => Ok(VdsState::ProcessJudged),
            (VdsState::ProcessJudged,    VdsEvent::HoldStructure)   => Ok(VdsState::StructureHeld),
            (VdsState::StructureHeld,    VdsEvent::SealDiary)       => Ok(VdsState::DiaryVerdict),
            (from, event) => Err(VdsBreach {
                from, event,
                text: "rend i shkelur: X-besim→Y-propozim→gjykim-procesi→strukturë→ditar-D",
            }),
        }
    }

    pub fn advance(&mut self, event: VdsEvent) -> Result<VdsState, VdsBreach> {
        let next = Self::transition(self.state, event)?;
        self.state = next;
        Ok(next)
    }

    pub fn state(&self) -> VdsState {
        self.state
    }

    pub fn is_fulfilled(&self) -> bool {
        matches!(self.state, VdsState::DiaryVerdict)
    }

    /// Rreshti i ditarit — dëshmia e shkruar e gjykimit (white-box).
    pub fn diary_line(&self, session_id: &str) -> String {
        format!("[VDS-DIARY] session={} gjendja={}", session_id, self.state.label())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lab_contracts::{GjataCollapseLaw as Law, CollapsePhase as Phase};

    #[test]
    fn full_structural_judgement_chain() {
        let mut m = VerefiedDiarySupremelaw::commission(Law::issue(Phase::Verification)).unwrap();
        for ev in [VdsEvent::TrustAnchor, VdsEvent::ReceiveProposal,
                   VdsEvent::JudgeProcess, VdsEvent::HoldStructure, VdsEvent::SealDiary] {
            m.advance(ev).unwrap();
        }
        assert!(m.is_fulfilled());
        assert!(m.diary_line("S1").contains("DIARY_VERDICT"));
    }

    #[test]
    fn judging_y_without_trusted_x_is_breach() {
        // Gjykimi VETËM i Y-së (pa X/i0 të besuar) është shkelje — thelbi i ligjit.
        let mut m = VerefiedDiarySupremelaw::commission(Law::issue(Phase::Verification)).unwrap();
        let b = m.advance(VdsEvent::ReceiveProposal).unwrap_err();
        assert_eq!(b.from, VdsState::AwaitingAnchor);
        assert!(b.text.contains("rend i shkelur"));
    }

    #[test]
    fn wrong_phase_refused() {
        assert!(VerefiedDiarySupremelaw::commission(Law::issue(Phase::Coordination)).is_err());
    }
}
