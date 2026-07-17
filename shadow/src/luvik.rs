// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LUVIK.RS — PORTA SOVRANE E RREPTË (Strict Epistemic Gate)            ║
// ║  Gjata Legacy™ | Bledar Gjata                                        ║
// ║                                                                          ║
// ║  Luvik është PIKA E VETME E ZBATIMIT për dy ligje:                    ║
// ║                                                                          ║
// ║  1) PORTA E SHKRIMIT (admit): asnjë input nuk bëhet dije pa GJURMË     ║
// ║     algoritmike të plotë. Refuzimi → destfake (purge/izolim).         ║
// ║                                                                          ║
// ║  2) PORTA E LEXIMIT PËR QUANTUM (verified_for_quantum): Quantum merr   ║
// ║     VETËM dije të verifikuar që mban një gjurmë të regjistruar. Çdo    ║
// ║     "info" pa gjurmë → None (s'ekziston si dije për Quantum).         ║
// ║                                                                          ║
// ║  "Sistemi nuk i beson vetes": Quantum (PRO/NPRO/LIM) NUK e prek kurrë  ║
// ║  arkivin drejtpërdrejt — vetëm përmes Luvik-ut.                       ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::knowledge_lineage::{enforce_traceability, ledger, Lineage};

/// Arsyeja e refuzimit nga porta e shkrimit.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LuvikReject {
    /// Pa gjurmë algoritmike (proof_chain bosh / zinxhir i pamjaftueshëm / nënshkrim zero).
    NoAlgorithmicTrace,
    /// Vula 500 e hequr (transport i fragmentuar).
    SealStripped,
}

impl std::fmt::Display for LuvikReject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let m = match self {
            LuvikReject::NoAlgorithmicTrace => "pa gjurmë algoritmike",
            LuvikReject::SealStripped       => "vula 500 e hequr",
        };
        write!(f, "LUVIK_REJECT: {m}")
    }
}

/// Pamje e dijes së VERIFIKUAR që Luvik ia ekspozon Quantum-it — gjithmonë me gjurmë.
#[derive(Debug, Clone, Copy)]
pub struct VerifiedKnowledge {
    pub input_id: u64,
    pub lineage:  Lineage, // prejardhja e plotë — provë e verifikimit
}

impl VerifiedKnowledge {
    /// A është prejardhja primitive sipas produktit ∏ Xᵢ = 1?
    pub fn is_primitive_chain(&self) -> bool {
        self.lineage.is_primitive_chain()
    }
}

pub struct Luvik;

impl Luvik {
    // ── PORTA E SHKRIMIT ────────────────────────────────────────────────

    /// Lejon kalimin drejt arkivit VETËM nëse lineage-i kalon ligjin e
    /// gjurmueshmërisë. Përndryshe → `Err(LuvikReject)` → thirrësi e dërgon
    /// te destfake (purge). ZERO if/else: vendimi vjen nga `is_traceable`.
    pub fn admit(lineage: &Lineage) -> Result<(), LuvikReject> {
        enforce_traceability(lineage).map_err(|_| LuvikReject::NoAlgorithmicTrace)
    }

    /// A pranohet ky lineage për shkrim? (formë boolean e `admit`.)
    pub fn admits(lineage: &Lineage) -> bool {
        lineage.is_traceable()
    }

    // ── PORTA E LEXIMIT PËR QUANTUM ─────────────────────────────────────

    /// Kthen dije VETËM nëse mban një gjurmë të regjistruar (e verifikuar).
    /// Quantum (PRO/NPRO/LIM) e thërret KËTË, kurrë arkivin drejtpërdrejt.
    pub fn verified_for_quantum(input_id: u64) -> Option<VerifiedKnowledge> {
        let lineage = ledger().lookup(input_id)?; // pa gjurmë → None (s'është dije)
        Some(VerifiedKnowledge { input_id, lineage })
    }

    /// A lejohet Quantum ta përdorë këtë input si bazë vendimi? (vetëm me gjurmë).
    pub fn is_admissible_for_quantum(input_id: u64) -> bool {
        ledger().lookup(input_id).is_some()
    }

    /// Sa copë dije të gjurmueshme janë të disponueshme për Quantum.
    pub fn admissible_count() -> usize {
        ledger().traced_count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::knowledge_lineage::Lineage;

    #[test]
    fn admit_blocks_untraceable() {
        // Lineage me proof_chain bosh → s'është i gjurmueshëm → refuzohet.
        let bad = Lineage::derive_capability(1, &[]); // proof bosh
        assert!(!Luvik::admits(&bad));
        assert_eq!(Luvik::admit(&bad), Err(LuvikReject::NoAlgorithmicTrace));
    }

    #[test]
    fn admit_allows_traceable() {
        let good = Lineage::derive_capability(2, &[7, 9]); // proof jo-bosh + cap_sealed
        assert!(Luvik::admits(&good));
        assert!(Luvik::admit(&good).is_ok());
    }

    #[test]
    fn quantum_gate_returns_none_without_trace() {
        // Një input_id i paregjistruar → Quantum s'merr asgjë.
        assert!(Luvik::verified_for_quantum(0xDEAD_BEEF_0000_0001).is_none());
        assert!(!Luvik::is_admissible_for_quantum(0xDEAD_BEEF_0000_0001));
    }

    #[test]
    fn quantum_gate_exposes_recorded_trace() {
        let lin = Lineage::derive_capability(0x5151_5151, &[1]);
        ledger().record(lin);
        let vk = Luvik::verified_for_quantum(0x5151_5151).unwrap();
        assert_eq!(vk.input_id, 0x5151_5151);
        assert!(Luvik::is_admissible_for_quantum(0x5151_5151));
    }
}
