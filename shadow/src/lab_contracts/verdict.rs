// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAB_CONTRACTS/VERDICT.RS — Kontrata e Gjendjes & Vendimit            ║
// ║  GJATA LEGACY™ — ESS-MAI (IDENTIK në Light + Quantum + Shadow)        ║
// ║                                                                          ║
// ║  Shadow nuk jep vetëm "po/jo" — mban GJENDJE verifikimi shkallë-shkallë.║
// ║  VerificationState = sa larg ka shkuar prova në shkallën e verifikimit. ║
// ║  LabVerdict = vendimi final i prodhuar nga autoriteti suprem (Shadow).  ║
// ║                                                                          ║
// ║  Vetëm Shadow e prodhon LabVerdict — Light/Quantum e lexojnë.          ║
// ║  Zero logjikë platforme. Zero if/else.                               ║
// ╚══════════════════════════════════════════════════════════════════════════╝

/// Gjendja e verifikimit — shkallë progresive (Verification First).
/// Sa më lart, aq më e vërtetuar dija. repr(u8) për krahasim/renditje.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum VerificationState {
    /// Pa verifikim — pretendim i papërpunuar.
    Unverified          = 0,
    /// Ka arsyetim, por pa presion/provë.
    Reasoned            = 1,
    /// Mbijetoi presionin (NPRO/adversarial).
    PressureTested      = 2,
    /// U verifikua ndër-domenesh.
    CrossDomainVerified = 3,
    /// Ka provë të verifikueshme (dokumentacion).
    EvidenceVerified    = 4,
    /// U demonstrua operacionalisht (live).
    OperationallyVerified = 5,
}

impl VerificationState {
    pub fn label(self) -> &'static str {
        match self {
            VerificationState::Unverified            => "UNVERIFIED",
            VerificationState::Reasoned              => "REASONED",
            VerificationState::PressureTested        => "PRESSURE_TESTED",
            VerificationState::CrossDomainVerified   => "CROSS_DOMAIN_VERIFIED",
            VerificationState::EvidenceVerified      => "EVIDENCE_VERIFIED",
            VerificationState::OperationallyVerified => "OPERATIONALLY_VERIFIED",
        }
    }

    /// rank — vlera numerike e gjendjes (për krahasim). Zero if.
    pub fn rank(self) -> u8 {
        self as u8
    }

    /// at_least — a e arrin (të paktën) një gjendje të kërkuar? Zero if.
    pub fn at_least(self, required: VerificationState) -> bool {
        (self as u8) >= (required as u8)
    }

    /// is_verified — a është të paktën EvidenceVerified (dije e vërtetuar)? Zero if.
    pub fn is_verified(self) -> bool {
        (self as u8) >= (VerificationState::EvidenceVerified as u8)
    }
}

/// Vendimi final i laboratorit — prodhuar VETËM nga Shadow (autoriteti suprem).
#[derive(Debug, Clone, PartialEq)]
pub enum LabVerdict {
    /// Pranuar në një nivel TRL, me arsye.
    Accepted {
        trl:    u8,
        state:  VerificationState,
        reason: String,
    },
    /// Refuzuar, me arsye + dobësitë e gjetura.
    Rejected {
        reason:     String,
        weaknesses: Vec<String>,
    },
    /// Nevojitet më shumë provë (lista e asaj që mungon).
    NeedsMoreEvidence {
        missing: Vec<String>,
    },
}

impl LabVerdict {
    /// is_accepted — a u pranua? Zero if — match.
    pub fn is_accepted(&self) -> bool {
        match self {
            LabVerdict::Accepted { .. } => true,
            LabVerdict::Rejected { .. } => false,
            LabVerdict::NeedsMoreEvidence { .. } => false,
        }
    }

    /// bit — biti 0/1 i vendimit (pranuar = 1). Zero if — match.
    pub fn bit(&self) -> u8 {
        match self {
            LabVerdict::Accepted { .. } => 1,
            LabVerdict::Rejected { .. } => 0,
            LabVerdict::NeedsMoreEvidence { .. } => 0,
        }
    }

    /// label — etiketa e shkurtër. Zero if — match.
    pub fn label(&self) -> &'static str {
        match self {
            LabVerdict::Accepted { .. } => "ACCEPTED",
            LabVerdict::Rejected { .. } => "REJECTED",
            LabVerdict::NeedsMoreEvidence { .. } => "NEEDS_MORE_EVIDENCE",
        }
    }
}
