/// Faza e kolapsit — ku ndodhet formula në rrjedhën 3-platformëshe.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollapsePhase {
    /// Light: i0 → PA → split (Xi, Yi). Koordinim, jo arsyetim.
    Coordination,
    /// Quantum: eliminim + kërkim i~ + nxjerrje XY. Arsyetim, jo vendim.
    Reasoning,
    /// Shadow: XY ↔ (Xi,Yi) → D ∈ {0,1}. Vendim, jo arsyetim i ri.
    Verification,
}

impl CollapsePhase {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Coordination => "KOLAPS_KOORDINIMI",
            Self::Reasoning => "KOLAPS_ARSYETIMI",
            Self::Verification => "KOLAPS_VERIFIKIMI",
        }
    }
}
