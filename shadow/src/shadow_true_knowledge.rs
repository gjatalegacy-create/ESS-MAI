// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  SHADOW_TRUE_KNOWLEDGE.RS — Pranuesi i Dijes së Ngarkuar (Shadow)    ║
// ║  GJATA LEGACY™ — ESS-MAI Shadow                          (KNOWLEDGE)   ║
// ║                                                                          ║
// ║  Moduli që PRET knowledge të ngarkuar me pip (pipeline pip i zbritur).   ║
// ║  E ndan sipas DOMENEVE dhe e përgatit për sistemim në vault.            ║
// ║                                                                          ║
// ║  LIGJ RRËNJËSOR (sovraniteti i inputit):                              ║
// ║   Çdo knowledge DUHET të ketë kaluar GJURMIM me TRACE FILLESTAR te       ║
// ║   Light PARA se të hyjë këtu. Knowledge PA trace → REFUZOHET.           ║
// ║   Sistemi e di gjithmonë se nga erdhi dija dhe si u gjurmua.            ║
// ║                                                                          ║
// ║  RRJEDHA: pip → Light (trace fillestar) → këtu (verifiko trace + ndaj    ║
// ║  sipas domeneve) → vault (sistemim përfundimtar).                       ║
// ║                                                                          ║
// ║  KUFIRI: ky modul VERIFIKON origjinën (trace) dhe NDAN sipas domenit;   ║
// ║  ruajtjen përfundimtare e bën vault-i. Shadow vendos çfarë pranohet.    ║
// ║  Zero if/else.                                                        ║
// ╚══════════════════════════════════════════════════════════════════════════╝

// ─────────────────────────────────────────────────────────────────────────────
// KNOWLEDGE DOMAIN — domeni ku klasifikohet dija
// ─────────────────────────────────────────────────────────────────────────────

/// Domenet sipas të cilave ndahet knowledge-u i ngarkuar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KnowledgeDomain {
    Physics,
    Chemistry,
    Biology,
    Mathematics,
    ComputerScience,
    Medicine,
    Legal,
    Economics,
    /// Domen i papërcaktuar (kërkon klasifikim manual).
    Unclassified,
}

impl KnowledgeDomain {
    pub fn label(self) -> &'static str {
        match self {
            KnowledgeDomain::Physics         => "physics",
            KnowledgeDomain::Chemistry       => "chemistry",
            KnowledgeDomain::Biology         => "biology",
            KnowledgeDomain::Mathematics     => "mathematics",
            KnowledgeDomain::ComputerScience => "computer_science",
            KnowledgeDomain::Medicine        => "medicine",
            KnowledgeDomain::Legal           => "legal",
            KnowledgeDomain::Economics       => "economics",
            KnowledgeDomain::Unclassified    => "unclassified",
        }
    }

    /// from_hint — klasifikon nga një fjalëkyç domeni. Zero if — match.
    pub fn from_hint(hint: &str) -> Self {
        match hint {
            "physics"          => KnowledgeDomain::Physics,
            "chemistry"        => KnowledgeDomain::Chemistry,
            "biology"          => KnowledgeDomain::Biology,
            "mathematics"      => KnowledgeDomain::Mathematics,
            "computer_science" => KnowledgeDomain::ComputerScience,
            "medicine"         => KnowledgeDomain::Medicine,
            "legal"            => KnowledgeDomain::Legal,
            "economics"        => KnowledgeDomain::Economics,
            _                  => KnowledgeDomain::Unclassified,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// TRACED KNOWLEDGE — knowledge me provë gjurmimi fillestar
// ─────────────────────────────────────────────────────────────────────────────

/// Knowledge i ngarkuar me pip, që MBART provën e gjurmimit fillestar nga Light.
/// trace_id != 0 është prova që ka kaluar gjurmim. trace_id == 0 → i pagjurmuar.
#[derive(Debug, Clone, PartialEq)]
pub struct TracedKnowledge {
    /// Gjurma fillestare nga Light (0 = i pagjurmuar → do refuzohet).
    pub trace_id:     u64,
    /// Përmbajtja e dijes.
    pub content:      Vec<u8>,
    /// Burimi (psh "pip:numpy-docs").
    pub source:       String,
    /// Fjalëkyçi i domenit (për klasifikim).
    pub domain_hint:  String,
    /// Vula kohore e ngarkimit.
    pub uploaded_at:  u64,
}

impl TracedKnowledge {
    /// new — krijon knowledge me trace. Zero if.
    pub fn new(trace_id: u64, content: Vec<u8>, source: String, domain_hint: String, uploaded_at: u64) -> Self {
        Self { trace_id, content, source, domain_hint, uploaded_at }
    }

    /// has_initial_trace — a ka kaluar gjurmim fillestar? Zero if.
    /// (trace_id != 0 → po; 0 → jo, sepse Light cakton trace jo-zero.)
    pub fn has_initial_trace(&self) -> bool {
        self.trace_id != 0
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// INTAKE RESULT — rezultati i pranimit të dijes
// ─────────────────────────────────────────────────────────────────────────────

/// Arsyeja e refuzimit të knowledge-ut.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntakeRejection {
    None,            // u pranua
    NoInitialTrace,  // s'kaloi gjurmim fillestar (shkelje sovraniteti)
    EmptyContent,    // përmbajtje bosh
}

impl IntakeRejection {
    pub fn label(self) -> &'static str {
        match self {
            IntakeRejection::None           => "ACCEPTED",
            IntakeRejection::NoInitialTrace => "NO_INITIAL_TRACE",
            IntakeRejection::EmptyContent   => "EMPTY_CONTENT",
        }
    }
}

/// Dija e klasifikuar gati për vault.
#[derive(Debug, Clone, PartialEq)]
pub struct ClassifiedKnowledge {
    pub trace_id:    u64,
    pub domain:      KnowledgeDomain,
    pub content:     Vec<u8>,
    pub source:      String,
    pub classified_at: u64,
}

/// Rezultati i pranimit.
#[derive(Debug, Clone, PartialEq)]
pub struct IntakeResult {
    /// A u pranua.
    pub accepted:    bool,
    /// Arsyeja nëse s'u pranua.
    pub rejection:   IntakeRejection,
    /// Dija e klasifikuar (Some nëse u pranua).
    pub classified:  Option<ClassifiedKnowledge>,
}

// ─────────────────────────────────────────────────────────────────────────────
// SHADOW TRUE KNOWLEDGE — pranuesi që detyron trace + ndan sipas domeneve
// ─────────────────────────────────────────────────────────────────────────────

/// Magazina e dijes së pranuar, e ndarë sipas domeneve.
#[derive(Debug, Clone)]
pub struct ShadowTrueKnowledge {
    /// Dija e pranuar dhe klasifikuar (append-only).
    entries: Vec<ClassifiedKnowledge>,
}

impl ShadowTrueKnowledge {
    /// new — magazinë bosh. Zero if.
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    /// intake — pranon knowledge NËSE ka trace fillestar, e ndan sipas domenit.
    ///
    /// LIGJ: pa trace fillestar → REFUZOHET (sovraniteti i inputit).
    /// Zero if — match shterues mbi (ka_trace, ka_përmbajtje).
    pub fn intake(&mut self, knowledge: &TracedKnowledge, at: u64) -> IntakeResult {
        let has_trace = knowledge.has_initial_trace();
        let has_content = !knowledge.content.is_empty();

        // Vendimi i pranimit (prioritet: trace > përmbajtje). Zero if — match.
        let (accepted, rejection) = match (has_trace, has_content) {
            (false, _)    => (false, IntakeRejection::NoInitialTrace),
            (true, false) => (false, IntakeRejection::EmptyContent),
            (true, true)  => (true, IntakeRejection::None),
        };

        // Klasifikimi (vetëm nëse pranuar). Zero if — match.
        let classified = match accepted {
            true => {
                let domain = KnowledgeDomain::from_hint(&knowledge.domain_hint);
                let entry = ClassifiedKnowledge {
                    trace_id:      knowledge.trace_id,
                    domain,
                    content:       knowledge.content.clone(),
                    source:        knowledge.source.clone(),
                    classified_at: at,
                };
                self.entries.push(entry.clone());
                Some(entry)
            }
            false => None,
        };

        IntakeResult {
            accepted,
            rejection,
            classified,
        }
    }

    /// count — sa hyrje ka magazina. Zero if.
    pub fn count(&self) -> usize {
        self.entries.len()
    }

    /// entries_for_domain — dija e një domeni specifik. Zero if — filter.
    pub fn entries_for_domain(&self, domain: KnowledgeDomain) -> Vec<&ClassifiedKnowledge> {
        self.entries.iter()
            .filter(|e| (e.domain as u8) == (domain as u8))
            .collect()
    }

    /// domain_count — sa domene unike janë mbushur. Zero if.
    pub fn domain_count(&self) -> usize {
        let mut domains: Vec<u8> = self.entries.iter().map(|e| e.domain as u8).collect();
        domains.sort();
        domains.dedup();
        domains.len()
    }

    /// traced_count — sa hyrje kanë trace (duhet të jenë TË GJITHA). Zero if.
    pub fn traced_count(&self) -> usize {
        self.entries.iter().filter(|e| e.trace_id != 0).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn traced_kb(trace_id: u64, domain: &str) -> TracedKnowledge {
        TracedKnowledge::new(
            trace_id,
            b"scientific content".to_vec(),
            "pip:source".to_string(),
            domain.to_string(),
            100,
        )
    }

    #[test]
    fn accepts_traced_knowledge() {
        let mut tk = ShadowTrueKnowledge::new();
        let result = tk.intake(&traced_kb(42, "physics"), 1000);
        assert!(result.accepted);
        assert_eq!(result.rejection, IntakeRejection::None);
        assert_eq!(tk.count(), 1);
        // Klasifikuar si physics.
        let c = result.classified.unwrap();
        assert_eq!(c.domain, KnowledgeDomain::Physics);
    }

    #[test]
    fn rejects_untraced_knowledge() {
        // LIGJ: knowledge pa trace fillestar → REFUZOHET.
        let mut tk = ShadowTrueKnowledge::new();
        let result = tk.intake(&traced_kb(0, "physics"), 1000); // trace_id=0
        assert!(!result.accepted);
        assert_eq!(result.rejection, IntakeRejection::NoInitialTrace);
        assert_eq!(tk.count(), 0); // s'u ruajt
        assert!(result.classified.is_none());
    }

    #[test]
    fn rejects_empty_content() {
        let mut tk = ShadowTrueKnowledge::new();
        let mut kb = traced_kb(42, "physics");
        kb.content = Vec::new();
        let result = tk.intake(&kb, 1000);
        assert!(!result.accepted);
        assert_eq!(result.rejection, IntakeRejection::EmptyContent);
    }

    #[test]
    fn splits_by_domain() {
        let mut tk = ShadowTrueKnowledge::new();
        tk.intake(&traced_kb(1, "physics"), 100);
        tk.intake(&traced_kb(2, "chemistry"), 200);
        tk.intake(&traced_kb(3, "physics"), 300);
        assert_eq!(tk.entries_for_domain(KnowledgeDomain::Physics).len(), 2);
        assert_eq!(tk.entries_for_domain(KnowledgeDomain::Chemistry).len(), 1);
        assert_eq!(tk.domain_count(), 2);
    }

    #[test]
    fn unknown_domain_unclassified() {
        let mut tk = ShadowTrueKnowledge::new();
        let result = tk.intake(&traced_kb(1, "astrology"), 100);
        assert!(result.accepted); // pranohet por...
        assert_eq!(result.classified.unwrap().domain, KnowledgeDomain::Unclassified);
    }

    #[test]
    fn all_accepted_have_trace() {
        // INVARIANT: çdo hyrje e pranuar DUHET të ketë trace.
        let mut tk = ShadowTrueKnowledge::new();
        tk.intake(&traced_kb(1, "physics"), 100);
        tk.intake(&traced_kb(2, "biology"), 200);
        tk.intake(&traced_kb(0, "chemistry"), 300); // ky refuzohet
        // Të gjitha të pranuarat kanë trace.
        assert_eq!(tk.count(), 2);
        assert_eq!(tk.traced_count(), 2); // = count → invariant mbahet
    }

    #[test]
    fn domain_from_hint_works() {
        assert_eq!(KnowledgeDomain::from_hint("physics"), KnowledgeDomain::Physics);
        assert_eq!(KnowledgeDomain::from_hint("legal"), KnowledgeDomain::Legal);
        assert_eq!(KnowledgeDomain::from_hint("xyz"), KnowledgeDomain::Unclassified);
    }
}
