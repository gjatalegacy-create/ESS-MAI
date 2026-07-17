// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  MEMORY/RAW_COGNITIVE_TRACE.RS — Gjurma e Pandryshueshme (Teoria 25)  ║
// ║  GJATA LEGACY™ — Quantum Platform                                     ║
// ║                                                                          ║
// ║  Çdo gjë që ndodh në Quantum regjistrohet si trace.                   ║
// ║  KURRË NUK FSHIHET — vetëm shadow_verdict ndryshon.                    ║
// ║  Quantum REGJISTRON (record → RAW_PENDING), por VETËM Shadow vendos    ║
// ║  verdiktin (set_verdict autorizohet vetëm për shadow_gj_legacy.lgc).   ║
// ║                                                                          ║
// ║  KUJTESË: kjo s'është memory persistente e Quantum — janë gjurmë të    ║
// ║  një cikli; verdiktin e shkruan Shadow. Quantum NUK vendos.           ║
// ║                                                                          ║
// ║  Pseudo përdor IF/WHILE; KONVERTUAR në match/filter (zero if/else).    ║
// ╚══════════════════════════════════════════════════════════════════════════╝

// ─────────────────────────────────────────────────────────────────────────────
// SHADOW VERDICT — gjendja e verifikimit
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShadowVerdict {
    RawPending,       // gjithmonë fillon këtu (Quantum s'vendos)
    VerifiedOptimal,  // Shadow vendosi: primitiv
    VerifiedNegative, // Shadow vendosi: negativ
    ShadowHold,       // Shadow: mban për testim
}

impl ShadowVerdict {
    pub fn label(self) -> &'static str {
        match self {
            ShadowVerdict::RawPending       => "RAW_PENDING",
            ShadowVerdict::VerifiedOptimal  => "VERIFIED_OPTIMAL",
            ShadowVerdict::VerifiedNegative => "VERIFIED_NEGATIVE",
            ShadowVerdict::ShadowHold       => "SHADOW_HOLD",
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// EVENT TYPE + OUTCOME
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EventType {
    Reasoning,
    Elimination,
    Failure,
    Contradiction,
    Convergence,
    Exploration,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Outcome {
    Success,
    Failure,
    Contradiction,
    Unknown,
}

// ─────────────────────────────────────────────────────────────────────────────
// COGNITIVE TRACE — një regjistrim
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct CognitiveTrace {
    pub trace_id:        String,
    pub cycle_id:        String,
    pub phase:           String,
    pub territory:       String,
    pub pressure_state:  String,
    pub event_type:      EventType,
    pub payload:         String,   // i plotë, pa filtrim
    pub module_source:   String,
    pub outcome:         Outcome,
    pub cause:           String,
    pub consequence:     String,
    pub lineage_ref:     String,
    pub shadow_verdict:  ShadowVerdict,  // i vetmi që ndryshon
}

// ─────────────────────────────────────────────────────────────────────────────
// TRACE STORE — append-only, asnjë delete
// ─────────────────────────────────────────────────────────────────────────────

/// I vetmi autoritet që ndryshon verdiktin (Shadow).
const VERDICT_AUTHORITY: &str = "shadow_gj_legacy.lgc";

pub struct RawCognitiveTrace {
    store:   Vec<CognitiveTrace>,
    next_id: u64,
}

impl RawCognitiveTrace {
    pub fn new() -> Self {
        Self { store: Vec::new(), next_id: 0 }
    }

    fn generate_id(&mut self) -> String {
        let id = format!("CT_{}", self.next_id);
        self.next_id += 1;
        id
    }

    /// record — regjistron një trace të ri. GJITHMONË RAW_PENDING.
    /// Quantum regjistron, por nuk vendos verdiktin.
    #[allow(clippy::too_many_arguments)]
    pub fn record(
        &mut self,
        cycle_id: &str,
        phase: &str,
        territory: &str,
        pressure_state: &str,
        event_type: EventType,
        payload: &str,
        module_source: &str,
        outcome: Outcome,
        cause: &str,
        consequence: &str,
        lineage_ref: &str,
    ) -> String {
        let trace_id = self.generate_id();
        self.store.push(CognitiveTrace {
            trace_id: trace_id.clone(),
            cycle_id: cycle_id.to_string(),
            phase: phase.to_string(),
            territory: territory.to_string(),
            pressure_state: pressure_state.to_string(),
            event_type,
            payload: payload.to_string(),
            module_source: module_source.to_string(),
            outcome,
            cause: cause.to_string(),
            consequence: consequence.to_string(),
            lineage_ref: lineage_ref.to_string(),
            shadow_verdict: ShadowVerdict::RawPending,  // gjithmonë
        });
        trace_id
    }

    /// set_verdict — VETËM Shadow. Quantum s'mund ta thërrasë me sukses.
    /// Zero if — match mbi (autoritet, gjetje).
    pub fn set_verdict(&mut self, trace_id: &str, verdict: ShadowVerdict, caller: &str) -> bool {
        // Autoriteti: vetëm shadow_gj_legacy.lgc.
        match caller == VERDICT_AUTHORITY {
            false => false,  // s'ka autoritet — Quantum përfshirë
            true => {
                // Gjej trace-n dhe ndrysho VETËM verdiktin.
                let found = self.store.iter_mut()
                    .find(|t| t.trace_id == trace_id);
                match found {
                    Some(t) => { t.shadow_verdict = verdict; true }
                    None => false,
                }
            }
        }
    }

    // ── QUERY ────────────────────────────────────────────────────────────────

    pub fn get_by_cycle(&self, cycle_id: &str) -> Vec<&CognitiveTrace> {
        self.store.iter().filter(|t| t.cycle_id == cycle_id).collect()
    }

    pub fn get_pending(&self) -> Vec<&CognitiveTrace> {
        self.store.iter()
            .filter(|t| t.shadow_verdict == ShadowVerdict::RawPending)
            .collect()
    }

    /// get_failures — FAILURE + VERIFIED_NEGATIVE, opsionalisht per territory.
    pub fn get_failures(&self, territory: &str) -> Vec<&CognitiveTrace> {
        // territory bosh → pa filtër; ndryshe → filtro (branchless me predikat).
        let no_filter = territory.is_empty();
        self.store.iter()
            .filter(|t| {
                t.outcome == Outcome::Failure
                    && t.shadow_verdict == ShadowVerdict::VerifiedNegative
                    && (no_filter || t.territory == territory)
            })
            .collect()
    }

    /// get_lineage — ndjek zinxhirin lineage_ref prapa.
    /// Zero if — while let + match.
    pub fn get_lineage(&self, trace_id: &str) -> Vec<&CognitiveTrace> {
        let mut chain = Vec::new();
        let mut current = trace_id.to_string();

        // Loop pa if — match mbi find.
        loop {
            match self.find_by_id(&current) {
                Some(t) => {
                    chain.push(t);
                    // lineage_ref bosh → fund (zero if: match mbi is_empty).
                    match t.lineage_ref.is_empty() {
                        true => break,
                        false => current = t.lineage_ref.clone(),
                    }
                }
                None => break,
            }
        }
        chain
    }

    pub fn find_by_id(&self, trace_id: &str) -> Option<&CognitiveTrace> {
        self.store.iter().find(|t| t.trace_id == trace_id)
    }

    /// stats — numërime sipas verdiktit.
    pub fn stats(&self) -> TraceStats {
        let count = |v: ShadowVerdict| {
            self.store.iter().filter(|t| t.shadow_verdict == v).count()
        };
        TraceStats {
            total: self.store.len(),
            raw_pending: count(ShadowVerdict::RawPending),
            verified_optimal: count(ShadowVerdict::VerifiedOptimal),
            verified_negative: count(ShadowVerdict::VerifiedNegative),
            shadow_hold: count(ShadowVerdict::ShadowHold),
        }
    }

    pub fn len(&self) -> usize { self.store.len() }
    pub fn is_empty(&self) -> bool { self.store.is_empty() }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TraceStats {
    pub total:             usize,
    pub raw_pending:       usize,
    pub verified_optimal:  usize,
    pub verified_negative: usize,
    pub shadow_hold:       usize,
}

impl Default for RawCognitiveTrace {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rec(t: &mut RawCognitiveTrace, cycle: &str, outcome: Outcome, lineage: &str) -> String {
        t.record(
            cycle, "DETECT", "science", "LOW",
            EventType::Reasoning, "payload", "LIM",
            outcome, "", "", lineage,
        )
    }

    #[test]
    fn record_starts_raw_pending() {
        let mut t = RawCognitiveTrace::new();
        let id = rec(&mut t, "c1", Outcome::Success, "");
        let trace = t.find_by_id(&id).unwrap();
        assert_eq!(trace.shadow_verdict, ShadowVerdict::RawPending);
    }

    #[test]
    fn only_shadow_sets_verdict() {
        let mut t = RawCognitiveTrace::new();
        let id = rec(&mut t, "c1", Outcome::Success, "");
        // Quantum tenton → dështon.
        assert!(!t.set_verdict(&id, ShadowVerdict::VerifiedOptimal, "QuantumReasoner"));
        // verdiktin s'e ndryshoi
        assert_eq!(t.find_by_id(&id).unwrap().shadow_verdict, ShadowVerdict::RawPending);
        // Shadow → sukses.
        assert!(t.set_verdict(&id, ShadowVerdict::VerifiedOptimal, "shadow_gj_legacy.lgc"));
        assert_eq!(t.find_by_id(&id).unwrap().shadow_verdict, ShadowVerdict::VerifiedOptimal);
    }

    #[test]
    fn store_never_deletes() {
        let mut t = RawCognitiveTrace::new();
        rec(&mut t, "c1", Outcome::Success, "");
        rec(&mut t, "c1", Outcome::Failure, "");
        // Asnjë metodë delete — store vetëm rritet.
        assert_eq!(t.len(), 2);
    }

    #[test]
    fn get_by_cycle_filters() {
        let mut t = RawCognitiveTrace::new();
        rec(&mut t, "c1", Outcome::Success, "");
        rec(&mut t, "c2", Outcome::Success, "");
        rec(&mut t, "c1", Outcome::Success, "");
        assert_eq!(t.get_by_cycle("c1").len(), 2);
        assert_eq!(t.get_by_cycle("c2").len(), 1);
    }

    #[test]
    fn pending_query() {
        let mut t = RawCognitiveTrace::new();
        let id1 = rec(&mut t, "c1", Outcome::Success, "");
        rec(&mut t, "c1", Outcome::Success, "");
        // Një merr verdict nga Shadow.
        t.set_verdict(&id1, ShadowVerdict::VerifiedOptimal, "shadow_gj_legacy.lgc");
        assert_eq!(t.get_pending().len(), 1); // mbetet 1 pending
    }

    #[test]
    fn lineage_chain_follows_refs() {
        let mut t = RawCognitiveTrace::new();
        let id1 = rec(&mut t, "c1", Outcome::Success, "");        // rrënja
        let id2 = rec(&mut t, "c1", Outcome::Success, &id1);      // → id1
        let _id3 = rec(&mut t, "c1", Outcome::Success, &id2);     // → id2
        let lineage = t.get_lineage(&_id3);
        // id3 → id2 → id1 = 3 trace
        assert_eq!(lineage.len(), 3);
    }

    #[test]
    fn stats_count_verdicts() {
        let mut t = RawCognitiveTrace::new();
        let id1 = rec(&mut t, "c1", Outcome::Success, "");
        rec(&mut t, "c1", Outcome::Failure, "");
        t.set_verdict(&id1, ShadowVerdict::VerifiedOptimal, "shadow_gj_legacy.lgc");
        let s = t.stats();
        assert_eq!(s.total, 2);
        assert_eq!(s.verified_optimal, 1);
        assert_eq!(s.raw_pending, 1);
    }

    #[test]
    fn failures_filtered_by_territory() {
        let mut t = RawCognitiveTrace::new();
        let id1 = t.record("c1", "P", "science", "LOW", EventType::Failure,
            "p", "M", Outcome::Failure, "", "", "");
        t.set_verdict(&id1, ShadowVerdict::VerifiedNegative, "shadow_gj_legacy.lgc");
        let id2 = t.record("c1", "P", "industrial", "LOW", EventType::Failure,
            "p", "M", Outcome::Failure, "", "", "");
        t.set_verdict(&id2, ShadowVerdict::VerifiedNegative, "shadow_gj_legacy.lgc");
        // Pa filtër → 2; me territory → 1.
        assert_eq!(t.get_failures("").len(), 2);
        assert_eq!(t.get_failures("science").len(), 1);
    }
}
