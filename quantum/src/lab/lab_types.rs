// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAB/LAB_TYPES.RS — Strukturat e Digital Lab (Teoria 44)             ║
// ║  GJATA LEGACY™ — Quantum Platform                                     ║
// ║                                                                          ║
// ║  TRL Pipeline 3-fazor (bazë: NASA TRL 1974, adaptuar epistemikisht):  ║
// ║   TRL1 Concept → TRL2 Hypothesis → TRL3 Validation                    ║
// ║                                                                          ║
// ║  Çdo fazë ka hash deterministik për integritet.                       ║
// ║  TrlEvidence → ushqen PIM për matjen e saktë.                         ║
// ║                                                                          ║
// ║  SHËNIM HASH: pseudo thotë SHA256; këtu përdorim FNV-1a të zgjeruar    ║
// ║  (deterministik, no_std, pa varësi të jashtme — si gjithë projekti).  ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::sovereign::sha256::sha256_hex;

// ─────────────────────────────────────────────────────────────────────────────
// LAB HASH — deterministik, për integritet TRL
// ─────────────────────────────────────────────────────────────────────────────

/// lab_hash — SHA-256 i vërtetë (FIPS 180-4), 64-karakter hex.
/// Dy thirrje me të njëjtin input → i njëjti hash (riprodhueshmëri kriptografike).
pub fn lab_hash(input: &str) -> String {
    sha256_hex(input.as_bytes())
}

// ─────────────────────────────────────────────────────────────────────────────
// TRL STATUS — gjendja e një faze
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TrlStatus {
    Created,     // sapo krijuar
    Formulated,  // formuluar (gati për fazën tjetër)
    Validated,   // validuar (kaloi TRL3)
    Rejected,    // refuzuar nga FilterGate ose threshold
}

impl TrlStatus {
    pub fn label(self) -> &'static str {
        match self {
            TrlStatus::Created    => "CREATED",
            TrlStatus::Formulated => "FORMULATED",
            TrlStatus::Validated  => "VALIDATED",
            TrlStatus::Rejected   => "REJECTED",
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// TRL1 — CONCEPT
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Concept {
    pub concept_id:  String,
    pub title:       String,
    pub domain:      String,
    pub raw_idea:    String,
    pub lab_hash:    String,
    pub trl_level:   u8,
    pub status:      TrlStatus,
}

impl Concept {
    /// create — TRL1: llogarit hash-in deterministik.
    pub fn create(title: &str, domain: &str, raw_idea: &str) -> Self {
        let combined = format!("{}|{}|{}", title, domain, raw_idea);
        let hash = lab_hash(&combined);
        let concept_id = format!("CON_{}", &hash[..8]);
        Self {
            concept_id,
            title: title.to_string(),
            domain: domain.to_string(),
            raw_idea: raw_idea.to_string(),
            lab_hash: hash,
            trl_level: 1,
            status: TrlStatus::Created,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// TRL2 — HYPOTHESIS
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Hypothesis {
    pub hypothesis_id:   String,
    pub concept_id:      String,
    pub domain:          String,
    pub statement:       String,
    pub assumptions:     Vec<String>,
    pub hypothesis_hash: String,
    pub trl_level:       u8,
    pub status:          TrlStatus,
    pub measurable:      bool,   // invariant: duhet matëshëm
    pub reproducible:    bool,   // invariant: duhet i riprodhueshëm
}

impl Hypothesis {
    /// formulate — TRL2: nga Concept, me invariantët measurable + reproducible.
    pub fn formulate(concept: &Concept, statement: &str, assumptions: Vec<String>) -> Self {
        let assum_joined = assumptions.join(";");
        let combined = format!("{}|{}|{}", concept.concept_id, statement, assum_joined);
        let hash = lab_hash(&combined);
        let hypothesis_id = format!("HYP_{}", &hash[..8]);

        // Invariantët: statement jo bosh → measurable; assumptions të dokumentuara → reproducible.
        let measurable = !statement.is_empty();
        let reproducible = !assumptions.is_empty();

        Self {
            hypothesis_id,
            concept_id: concept.concept_id.clone(),
            domain: concept.domain.clone(),
            statement: statement.to_string(),
            assumptions,
            hypothesis_hash: hash,
            trl_level: 2,
            status: TrlStatus::Formulated,
            measurable,
            reproducible,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// SIMULATION RESULT — output i SimulationEngine
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct SimulationResult {
    pub outcome:       SimOutcome,
    pub metrics:       SimMetrics,
    pub repetitions:   u32,     // sa herë u përsërit (riprodhueshmëri)
    pub consistency:   f32,     // [0,1] sa konsistent ndër përsëritjet
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SimOutcome {
    Converged,    // simulimi konvergoi
    Diverged,     // u largua (jo i qëndrueshëm)
    Inconclusive, // i paqartë
}

impl SimOutcome {
    pub fn label(self) -> &'static str {
        match self {
            SimOutcome::Converged    => "CONVERGED",
            SimOutcome::Diverged     => "DIVERGED",
            SimOutcome::Inconclusive => "INCONCLUSIVE",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SimMetrics {
    pub stability:    f32,   // qëndrueshmëria e rezultatit
    pub signal_noise: f32,   // raporti sinjal/zhurmë
    pub convergence:  f32,   // sa shpejt konvergoi
}

// ─────────────────────────────────────────────────────────────────────────────
// TRL EVIDENCE — output për PIM (matja e saktë)
// ─────────────────────────────────────────────────────────────────────────────

/// TrlEvidence — pikërisht ç'pret PIM nga Digital Lab.
/// Kjo është lidhja: Lab mat → PIM përdor për potentiality.
#[derive(Debug, Clone)]
pub struct TrlEvidence {
    pub trl_level:        u8,
    pub trl_passed:       bool,
    pub lab_test_id:      Option<String>,
    pub test_description: String,
    pub findings:         String,
    pub confidence:       f32,
    pub pending_lab:      bool,  // true nëse Lab s'u konsultua
    pub reproducibility:  f32,   // [0,1] për PIM dimension
}

impl TrlEvidence {
    /// pending — kur Lab s'u konsultua (PIM e di që mungon evidenca).
    pub fn pending() -> Self {
        Self {
            trl_level: 0,
            trl_passed: false,
            lab_test_id: None,
            test_description: "Lab nuk u konsultua".to_string(),
            findings: "".to_string(),
            confidence: 0.0,
            pending_lab: true,
            reproducibility: 0.0,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// TRL3 RESULT — rezultati final i validimit
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Trl3Result {
    pub promoted:        bool,        // a u promovua (kërkon FilterGate + threshold)
    pub score:           f32,         // confidence [0,1]
    pub rejection_reason: RejectionReason,
    pub evidence:        TrlEvidence, // për PIM
    pub gate_passed:     bool,        // a kaloi FilterGate (invariant)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RejectionReason {
    None,          // nuk u refuzua
    FilterGate,    // FilterGate refuzoi (invariant violation)
    ScoreLow,      // score < threshold 0.70
    PrerequisiteFail, // TRL e mëparshme s'u plotësua
}

impl RejectionReason {
    pub fn label(self) -> &'static str {
        match self {
            RejectionReason::None             => "NONE",
            RejectionReason::FilterGate       => "FILTER_GATE",
            RejectionReason::ScoreLow         => "SCORE_LOW",
            RejectionReason::PrerequisiteFail => "PREREQUISITE_FAIL",
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// LAB ERROR — gabimet e Lab-it (jo panic)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum LabError {
    PrerequisiteNotMet(String),  // TRL e mëparshme mungon
    EngineBlocked(String),       // engine jo aktiv
    InvariantViolation(String),  // shkelje invarianti
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lab_hash_deterministic() {
        // I njëjti input → i njëjti hash (riprodhueshmëri).
        let h1 = lab_hash("test input");
        let h2 = lab_hash("test input");
        assert_eq!(h1, h2);
        // Input i ndryshëm → hash i ndryshëm.
        let h3 = lab_hash("different input");
        assert_ne!(h1, h3);
        // Gjatësia 64 hex (SHA-256).
        assert_eq!(h1.len(), 64);
    }

    #[test]
    fn concept_creates_with_hash() {
        let c = Concept::create("Quantum Reasoning", "science", "deterministic elimination");
        assert_eq!(c.trl_level, 1);
        assert_eq!(c.status, TrlStatus::Created);
        assert!(c.concept_id.starts_with("CON_"));
        assert_eq!(c.lab_hash.len(), 64);
    }

    #[test]
    fn concept_hash_is_deterministic() {
        let c1 = Concept::create("T", "d", "idea");
        let c2 = Concept::create("T", "d", "idea");
        assert_eq!(c1.lab_hash, c2.lab_hash);
        assert_eq!(c1.concept_id, c2.concept_id);
    }

    #[test]
    fn hypothesis_requires_concept() {
        let c = Concept::create("T", "science", "idea");
        let h = Hypothesis::formulate(&c, "X causes Y", vec!["assumption1".to_string()]);
        assert_eq!(h.trl_level, 2);
        assert_eq!(h.concept_id, c.concept_id);
        assert!(h.measurable);     // statement jo bosh
        assert!(h.reproducible);   // assumptions të dokumentuara
    }

    #[test]
    fn hypothesis_invariants_enforced() {
        let c = Concept::create("T", "science", "idea");
        // Statement bosh → jo measurable.
        let h_no_stmt = Hypothesis::formulate(&c, "", vec!["a".to_string()]);
        assert!(!h_no_stmt.measurable);
        // Pa assumptions → jo reproducible.
        let h_no_assum = Hypothesis::formulate(&c, "claim", vec![]);
        assert!(!h_no_assum.reproducible);
    }

    #[test]
    fn trl_evidence_pending() {
        let e = TrlEvidence::pending();
        assert!(e.pending_lab);
        assert!(!e.trl_passed);
        assert_eq!(e.trl_level, 0);
        assert!(e.lab_test_id.is_none());
    }

    #[test]
    fn rejection_reasons_labeled() {
        assert_eq!(RejectionReason::FilterGate.label(), "FILTER_GATE");
        assert_eq!(RejectionReason::ScoreLow.label(), "SCORE_LOW");
        assert_eq!(RejectionReason::None.label(), "NONE");
    }
}
