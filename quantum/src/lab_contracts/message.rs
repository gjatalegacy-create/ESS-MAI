// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAB_CONTRACTS/MESSAGE.RS — Gjuha e Komunikimit Lab-to-Lab            ║
// ║  GJATA LEGACY™ — ESS-MAI (IDENTIK në Light + Quantum + Shadow)        ║
// ║                                                                          ║
// ║  LabMessage është zarfi i përbashkët me të cilin tre laboratorët flasin.║
// ║  Kur Quantum flet, Shadow e kupton. Kur Shadow gjykon, Light e kupton.  ║
// ║  Pa përkthime. Pa adaptera. Pa shtresa të ndërmjetme.                  ║
// ║                                                                          ║
// ║  Zero logjikë platforme. Zero if/else.                               ║
// ╚══════════════════════════════════════════════════════════════════════════╝

/// Cila platformë-laborator e dërgon/pranon mesazhin.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabNode {
    Light,
    Quantum,
    Shadow,
}

impl LabNode {
    pub fn label(self) -> &'static str {
        match self {
            LabNode::Light   => "LIGHT",
            LabNode::Quantum => "QUANTUM",
            LabNode::Shadow  => "SHADOW",
        }
    }
}

/// Lloji i ngarkesës që mesazhi mbart (cila kontratë).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabPayloadKind {
    TrustContext,
    TrlEvidence,
    PressureReport,
    EvidencePackage,
    Verdict,
    LedgerEntry,
    NegativeKnowledge,
}

impl LabPayloadKind {
    pub fn label(self) -> &'static str {
        match self {
            LabPayloadKind::TrustContext      => "TRUST_CONTEXT",
            LabPayloadKind::TrlEvidence       => "TRL_EVIDENCE",
            LabPayloadKind::PressureReport    => "PRESSURE_REPORT",
            LabPayloadKind::EvidencePackage   => "EVIDENCE_PACKAGE",
            LabPayloadKind::Verdict           => "VERDICT",
            LabPayloadKind::LedgerEntry       => "LEDGER_ENTRY",
            LabPayloadKind::NegativeKnowledge => "NEGATIVE_KNOWLEDGE",
        }
    }
}

/// Zarfi i komunikimit lab-to-lab — metadata e përbashkët e çdo transferimi.
///
/// Vetë ngarkesa (kontrata konkrete) referohet me id + lloj + vulë; zarfi
/// mban rrugën (kush→kujt), sekuencën dhe vulën 500 (konsistencë me sistemin).
#[derive(Debug, Clone, PartialEq)]
pub struct LabMessage {
    /// Identifikuesi i pretendimit që mesazhi i përket.
    pub claim_id:      u64,
    /// Kush e dërgon.
    pub from:          LabNode,
    /// Kujt i shkon.
    pub to:            LabNode,
    /// Çfarë lloj ngarkese mbart.
    pub payload_kind:  LabPayloadKind,
    /// Numri sekuencial në rrjedhë (rendit mesazhet).
    pub sequence:      u64,
    /// Vula 500 e maskuar (0xA451) — konsistencë me të gjithë sistemin.
    pub seal:          u32,
    /// Vula kohore (ns).
    pub timestamp:     u64,
}

impl LabMessage {
    /// Vula 500 e maskuar siç udhëton kudo në sistem.
    pub const SEAL_VALID: u32 = 0xA451;

    /// new — krijon një mesazh të vulosur me vulën 500. Zero if.
    pub fn new(
        claim_id: u64,
        from: LabNode,
        to: LabNode,
        payload_kind: LabPayloadKind,
        sequence: u64,
        timestamp: u64,
    ) -> Self {
        Self {
            claim_id,
            from,
            to,
            payload_kind,
            sequence,
            seal: Self::SEAL_VALID,
            timestamp,
        }
    }

    /// seal_ok — a mban vulën 500 të vlefshme? Zero if.
    /// (flags & 0xFFFF) ^ 0xA5A5 == 500 — identike me gjithë ekosistemin.
    pub fn seal_ok(&self) -> bool {
        ((self.seal & 0xFFFF) ^ 0xA5A5) == 500
    }

    /// is_valid_route — a është rrugë e ndryshme (from != to)? Zero if.
    pub fn is_valid_route(&self) -> bool {
        (self.from as u8) != (self.to as u8)
    }
}
