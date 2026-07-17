// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAB_CONTRACTS_V11/PACKET.RS — Gjuha e Shtresës 3 (Layer3Packet)     ║
// ║  GJATA LEGACY™ — ESS-MAI (IDENTIK në Light + Quantum + Shadow)        ║
// ║                                                                          ║
// ║  Layer3Packet është zarfi me të cilin Shtresa 3 komunikon nëpër tri     ║
// ║  platformat — boshti i përbashkët: trace_id, claim_id, session_id,     ║
// ║  territory, packet_id, wisdom_ref.                                     ║
// ║                                                                          ║
// ║  Mban vulën 500 (konsistencë me gjithë sistemin). Zero logjikë         ║
// ║  platforme. Zero if/else.                                            ║
// ╚══════════════════════════════════════════════════════════════════════════╝

/// Boshti i identifikuesve që kalon nëpër të tri platformat (Shtresa 3).
#[derive(Debug, Clone, PartialEq)]
pub struct Layer3Context {
    /// Gjurma (lidh gjithë rrjedhën).
    pub trace_id:    u64,
    /// Pretendimi.
    pub claim_id:    u64,
    /// Sesioni.
    pub session_id:  u64,
    /// Territori/domeni.
    pub territory:   String,
    /// Identifikuesi i paketës.
    pub packet_id:   u64,
    /// Referenca te urtësia (wisdom note e lidhur).
    pub wisdom_ref:  u64,
}

impl Layer3Context {
    /// new — krijon kontekstin e Shtresës 3. Zero if.
    pub fn new(
        trace_id: u64,
        claim_id: u64,
        session_id: u64,
        territory: String,
        packet_id: u64,
        wisdom_ref: u64,
    ) -> Self {
        Self { trace_id, claim_id, session_id, territory, packet_id, wisdom_ref }
    }

    /// has_wisdom — a është lidhur me një shënim urtësie? Zero if.
    pub fn has_wisdom(&self) -> bool {
        self.wisdom_ref != 0
    }
}

/// Lloji i ngarkesës që paketa e Shtresës 3 mbart.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layer3PayloadKind {
    HardwareBounds,   // kufijtë nga prod_formula (Quantum)
    DeviceSnapshot,   // gjendja e device-it
    WisdomNote,       // vëzhgim urtësie
    NightWatchReport, // raporti i sentinelit (Shadow)
    NeuralLaw,        // interpretimi neural (Light)
}

impl Layer3PayloadKind {
    pub fn label(self) -> &'static str {
        match self {
            Layer3PayloadKind::HardwareBounds   => "HARDWARE_BOUNDS",
            Layer3PayloadKind::DeviceSnapshot   => "DEVICE_SNAPSHOT",
            Layer3PayloadKind::WisdomNote       => "WISDOM_NOTE",
            Layer3PayloadKind::NightWatchReport => "NIGHT_WATCH_REPORT",
            Layer3PayloadKind::NeuralLaw        => "NEURAL_LAW",
        }
    }
}

/// Zarfi i komunikimit të Shtresës 3.
#[derive(Debug, Clone, PartialEq)]
pub struct Layer3Packet {
    /// Konteksti (boshti i identifikuesve).
    pub context:      Layer3Context,
    /// Çfarë lloj ngarkese.
    pub payload_kind: Layer3PayloadKind,
    /// Sekuenca në rrjedhë.
    pub sequence:     u64,
    /// Vula 500 e maskuar (0xA451).
    pub seal:         u32,
    /// Vula kohore (ns).
    pub timestamp:    u64,
}

impl Layer3Packet {
    /// Vula 500 e maskuar — identike me gjithë ekosistemin.
    pub const SEAL_VALID: u32 = 0xA451;

    /// new — krijon paketë të vulosur. Zero if.
    pub fn new(
        context: Layer3Context,
        payload_kind: Layer3PayloadKind,
        sequence: u64,
        timestamp: u64,
    ) -> Self {
        Self {
            context,
            payload_kind,
            sequence,
            seal: Self::SEAL_VALID,
            timestamp,
        }
    }

    /// seal_ok — a mban vulën 500? Zero if.
    /// (flags & 0xFFFF) ^ 0xA5A5 == 500 — identike me gjithë sistemin.
    pub fn seal_ok(&self) -> bool {
        ((self.seal & 0xFFFF) ^ 0xA5A5) == 500
    }
}
