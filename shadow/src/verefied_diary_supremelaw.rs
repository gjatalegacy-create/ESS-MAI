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
use crate::lab_contracts::gjata_collapse_law::law_seal;

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

/// Forma e provës primitive që GCL/ESS-MAI i kërkon muskulit Shadow.
/// Nuk është verdict dhe nuk del në wire; lidh vetëm i₀/PA, Xi/Yi,
/// transformimin Quantum dhe vazhdimësinë e të njëjtit proces GCL.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GclPrimitiveMode {
    Bound = 1,
    Refused = 2,
    LegacyCompatible = 3,
    DirectTest = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GclPrimitiveVerification {
    pub mode: GclPrimitiveMode,
    pub state: VdsState,
    pub directive_seal: u64,
    pub primitive_anchor: u64,
    pub parent_i0_digest: u64,
    pub proposal_digest: u64,
    pub pd_binding_digest: u64,
    pub pd_continuum_activation_digest: u64,
    pub anchor_present: bool,
    pub parent_i0_matches: bool,
    pub xi_verified: bool,
    pub yi_verified: bool,
    pub proposal_received: bool,
    pub transformation_traced: bool,
    pub gcl_continuity: bool,
    pub structure_held: bool,
}

impl GclPrimitiveVerification {
    pub fn primitive_bit(&self) -> u8 {
        match self.mode {
            GclPrimitiveMode::Bound => (
                self.anchor_present
                    & self.parent_i0_matches
                    & self.xi_verified
                    & self.yi_verified
            ) as u8,
            GclPrimitiveMode::Refused => (
                self.anchor_present
                    & self.parent_i0_matches
            ) as u8,
            GclPrimitiveMode::LegacyCompatible | GclPrimitiveMode::DirectTest => 1,
        }
    }

    pub fn proposal_bit(&self) -> u8 {
        match self.mode {
            GclPrimitiveMode::Refused => 0,
            GclPrimitiveMode::Bound
            | GclPrimitiveMode::LegacyCompatible
            | GclPrimitiveMode::DirectTest => self.proposal_received as u8,
        }
    }

    pub fn transformation_bit(&self) -> u8 {
        match self.mode {
            GclPrimitiveMode::LegacyCompatible | GclPrimitiveMode::DirectTest => 1,
            GclPrimitiveMode::Bound | GclPrimitiveMode::Refused => (
                self.transformation_traced
                    & (self.parent_i0_digest != 0)
                    & (self.proposal_digest != 0)
                    & (self.pd_binding_digest != 0)
                    & (self.pd_continuum_activation_digest != 0)
            ) as u8,
        }
    }

    pub fn gcl_bit(&self) -> u8 {
        let directive_ok = self.directive_seal == law_seal();
        match self.mode {
            GclPrimitiveMode::LegacyCompatible | GclPrimitiveMode::DirectTest =>
                directive_ok as u8,
            GclPrimitiveMode::Bound | GclPrimitiveMode::Refused =>
                (directive_ok & self.gcl_continuity & self.structure_held) as u8,
        }
    }

    pub fn final_bit(&self) -> u8 {
        self.primitive_bit()
            & self.proposal_bit()
            & self.transformation_bit()
            & self.gcl_bit()
    }

    #[cfg(test)]
    pub fn direct_test(primitive_anchor: u64) -> Self {
        Self {
            mode: GclPrimitiveMode::DirectTest,
            state: VdsState::StructureHeld,
            directive_seal: law_seal(),
            primitive_anchor,
            parent_i0_digest: primitive_anchor.wrapping_add(1),
            proposal_digest: primitive_anchor.wrapping_add(2),
            pd_binding_digest: primitive_anchor.wrapping_add(3),
            pd_continuum_activation_digest: primitive_anchor.wrapping_add(4),
            anchor_present: primitive_anchor != 0,
            parent_i0_matches: true,
            xi_verified: true,
            yi_verified: true,
            proposal_received: true,
            transformation_traced: true,
            gcl_continuity: true,
            structure_held: true,
        }
    }
}

pub struct VerefiedDiarySupremelaw {
    state: VdsState,
    directive_seal: u64,
}

impl VerefiedDiarySupremelaw {
    pub fn commission(d: SupremeDirective) -> Result<Self, VdsBreach> {
        match GjataCollapseLaw::verify(&d, CollapsePhase::Verification) {
            true  => Ok(Self { state: VdsState::AwaitingAnchor, directive_seal: d.seal }),
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


    /// Pajtueshmëria historike mbetet e deklaruar, por lind vetëm nga një
    /// muskul VDS i komisionuar dhe i vulosur realisht nga GCL/ESS-MAI.
    pub fn attest_legacy_compatible(&self) -> GclPrimitiveVerification {
        GclPrimitiveVerification {
            mode: GclPrimitiveMode::LegacyCompatible,
            state: self.state,
            directive_seal: self.directive_seal,
            primitive_anchor: 0,
            parent_i0_digest: 0,
            proposal_digest: 0,
            pd_binding_digest: 0,
            pd_continuum_activation_digest: 0,
            anchor_present: false,
            parent_i0_matches: true,
            xi_verified: false,
            yi_verified: false,
            proposal_received: true,
            transformation_traced: true,
            gcl_continuity: true,
            structure_held: true,
        }
    }

    /// Krijon provën primitive vetëm pasi procesi është gjykuar dhe struktura
    /// është mbajtur. Kjo provë ushqen multi-verifikimin si forcë e GCL-së.
    pub fn attest_structure(
        &self,
        mode: GclPrimitiveMode,
        primitive_anchor: u64,
        parent_i0_digest: u64,
        proposal_digest: u64,
        pd_binding_digest: u64,
        pd_continuum_activation_digest: u64,
        anchor_present: bool,
        parent_i0_matches: bool,
        xi_verified: bool,
        yi_verified: bool,
        proposal_received: bool,
        transformation_traced: bool,
        gcl_continuity: bool,
    ) -> Result<GclPrimitiveVerification, VdsBreach> {
        match self.state {
            VdsState::StructureHeld => Ok(GclPrimitiveVerification {
                mode,
                state: self.state,
                directive_seal: self.directive_seal,
                primitive_anchor,
                parent_i0_digest,
                proposal_digest,
                pd_binding_digest,
                pd_continuum_activation_digest,
                anchor_present,
                parent_i0_matches,
                xi_verified,
                yi_verified,
                proposal_received,
                transformation_traced,
                gcl_continuity,
                structure_held: true,
            }),
            from => Err(VdsBreach {
                from,
                event: VdsEvent::HoldStructure,
                text: "prova primitive kërkon ProcessJudged→StructureHeld nën direktivën GCL",
            }),
        }
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

    #[test]
    fn bound_primitive_proof_requires_full_gcl_state_chain() {
        let mut m = VerefiedDiarySupremelaw::commission(
            Law::issue(Phase::Verification),
        ).unwrap();
        for event in [
            VdsEvent::TrustAnchor,
            VdsEvent::ReceiveProposal,
            VdsEvent::JudgeProcess,
            VdsEvent::HoldStructure,
        ] {
            m.advance(event).unwrap();
        }
        let proof = m.attest_structure(
            GclPrimitiveMode::Bound,
            0xA451,
            0xA452,
            0xA453,
            0xA454,
            0xA455,
            true,
            true,
            true,
            true,
            true,
            true,
            true,
        ).unwrap();
        assert_eq!(proof.state, VdsState::StructureHeld);
        assert_eq!(proof.final_bit(), 1);
    }

    #[test]
    fn refusal_is_processed_but_cannot_become_positive_proposal() {
        let mut m = VerefiedDiarySupremelaw::commission(
            Law::issue(Phase::Verification),
        ).unwrap();
        for event in [
            VdsEvent::TrustAnchor,
            VdsEvent::ReceiveProposal,
            VdsEvent::JudgeProcess,
            VdsEvent::HoldStructure,
        ] {
            m.advance(event).unwrap();
        }
        let proof = m.attest_structure(
            GclPrimitiveMode::Refused,
            0xA451,
            0xA452,
            0xA453,
            0xA454,
            0xA455,
            true,
            true,
            false,
            false,
            true,
            true,
            true,
        ).unwrap();
        assert_eq!(proof.primitive_bit(), 1);
        assert_eq!(proof.proposal_bit(), 0);
        assert_eq!(proof.final_bit(), 0);
    }
}
