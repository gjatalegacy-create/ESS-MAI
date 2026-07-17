// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LIM_COLLECTOR.RS — GRUPI 1 (KOLEKSIONISTI)                           ║
// ║  GJATA LEGACY™ — Quantum Platform / Linear Info Metric                ║
// ║                                                                          ║
// ║  ROLI: mbledh fragmente ndër-turn. NUK mat. Vetëm akumulon.           ║
// ║  Pret derisa ka material të mjaftueshëm → pastaj Grupi 2 mat.          ║
// ║                                                                          ║
// ║  KJO është ajo që mungonte nga lim_pro_pim_pipeline.rs:               ║
// ║  ai maste per-mesazh; LIM i vërtetë mat mbi GRUMBULLIMIN multi-turn.   ║
// ╚══════════════════════════════════════════════════════════════════════════╝

// ─────────────────────────────────────────────────────────────────────────────
// PRAGJET E AKUMULIMIT — kur ka "material të mjaftueshëm"
// ─────────────────────────────────────────────────────────────────────────────

/// Minimumi i fragmenteve para se matja të ketë kuptim.
/// Nën këtë, krahasimi midis fragmenteve s'jep sinjal.
const MIN_FRAGMENTS: usize = 2;

/// Minimumi i fjalëve semantike totale të akumuluara.
const MIN_TOTAL_WORDS: usize = 8;

// ─────────────────────────────────────────────────────────────────────────────
// GRUPI 1 — COLLECTOR
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct LimCollector {
    /// Fragmentet e akumuluara ndër-turn (rendi ruhet).
    fragments: Vec<String>,
    /// Numri i turn-eve të mbledhura.
    turn_count: u32,
    /// Sesioni.
    session_id: String,
    /// Territori (domeni).
    territory: String,
}

impl LimCollector {
    pub fn new(session_id: &str, territory: &str) -> Self {
        Self {
            fragments: Vec::new(),
            turn_count: 0,
            session_id: session_id.to_string(),
            territory: territory.to_string(),
        }
    }

    /// Akumulon një fragment të ri (një turn). NUK mat.
    /// Fragmentet bosh injorohen.
    pub fn collect(&mut self, fragment: &str) {
        let trimmed = fragment.trim();
        match trimmed.is_empty() {
            true => return,
            false => {}
        }
        self.fragments.push(trimmed.to_string());
        self.turn_count += 1;
    }

    /// A ka material të mjaftueshëm që Grupi 2 të masë?
    ///
    /// Kërkon: ≥ MIN_FRAGMENTS fragmente DHE ≥ MIN_TOTAL_WORDS fjalë.
    /// Branchless logic për vendimin (jo error handling).
    pub fn has_sufficient_material(&self) -> bool {
        let enough_fragments = self.fragments.len() >= MIN_FRAGMENTS;
        let total_words: usize = self.fragments
            .iter()
            .map(|f| f.split_whitespace().count())
            .sum();
        let enough_words = total_words >= MIN_TOTAL_WORDS;
        enough_fragments && enough_words
    }

    /// Fragmentet e akumuluara (për Grupin 2).
    pub fn fragments(&self) -> &[String] {
        &self.fragments
    }

    /// Numri i fragmenteve.
    pub fn len(&self) -> usize {
        self.fragments.len()
    }

    pub fn is_empty(&self) -> bool {
        self.fragments.is_empty()
    }

    pub fn turn_count(&self) -> u32 {
        self.turn_count
    }

    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    pub fn territory(&self) -> &str {
        &self.territory
    }

    /// Grumbullimi i plotë si një tekst i vetëm (fragmentet bashkë).
    /// Përdoret nga analizues që duan tërë konversacionin.
    pub fn combined(&self) -> String {
        self.fragments.join("\n")
    }

    /// Pastron koleksionin (fund sesioni). Ephemeral — zero persistent.
    pub fn reset(&mut self) {
        self.fragments.clear();
        self.turn_count = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collector_accumulates() {
        let mut c = LimCollector::new("s1", "science");
        c.collect("first fragment here");
        c.collect("second fragment added");
        assert_eq!(c.len(), 2);
        assert_eq!(c.turn_count(), 2);
    }

    #[test]
    fn ignores_empty() {
        let mut c = LimCollector::new("s1", "science");
        c.collect("real");
        c.collect("   ");
        c.collect("");
        assert_eq!(c.len(), 1);
    }

    #[test]
    fn insufficient_then_sufficient() {
        let mut c = LimCollector::new("s1", "science");
        c.collect("one two three");
        // 1 fragment → insufficient
        assert!(!c.has_sufficient_material());
        c.collect("four five six seven eight");
        // 2 fragments, 8 words → sufficient
        assert!(c.has_sufficient_material());
    }

    #[test]
    fn combined_joins() {
        let mut c = LimCollector::new("s1", "x");
        c.collect("alpha");
        c.collect("beta");
        assert_eq!(c.combined(), "alpha\nbeta");
    }
}
