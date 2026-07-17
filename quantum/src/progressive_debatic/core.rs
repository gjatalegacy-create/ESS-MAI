// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  PROGRESSIVE_DEBATIC / core.rs                                           ║
// ║  GJATA LEGACY™ — ESS-MAI Quantum                                         ║
// ║                                                                          ║
// ║  ROLI: Motori i gërmimit kognitiv.                                       ║
// ║    open_session → ingest(çdo sinjal) → ndërto CognitiveSignal →          ║
// ║    përditëso EpistemicTrace → GeniusDetector.evaluate →                  ║
// ║    decide_response → [GeniusSignal?] → PdCognitivePackage për PIM.        ║
// ║                                                                          ║
// ║  KUFIRI KRITIK: PD NUK imponon cognition — e ZBULON. Ndalon kur          ║
// ║  pressure_budget shterron ose kur user ka rezistencë.                    ║
// ║                                                                          ║
// ║  Llogaritjet janë REALE dhe DETERMINISTE (jo pseudo, jo random):         ║
// ║  concept-map, returns, energji, dendësi, koherencë, primitive implicite, ║
// ║  kontradikta produktive. Dominanti që "mbijeton" = kandidati i eliminimit║
// ║  dykahësh. ZERO if/else: match/branchless kudo.                          ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use std::collections::{HashMap, HashSet};
use super::types::*;
use super::genius_detector::{GeniusDetector, most_frequent_concept, mean, tokenize};
use crate::pd_spine_contract::{FixedMass, MASS_SCALE};

/// PdSession — gjendje e plotë e një sesioni gërmimi.
#[derive(Debug, Clone)]
pub struct PdSession {
    pub session_id:         String,
    pub active:             bool,
    pub current_mode:       DebateMode,
    pub pressure_budget:    PressureBudget,
    pub turns:              Vec<PdTurn>,
    pub trace:              EpistemicTrace,
    pub genius_signal:      Option<GeniusSignal>,
    pub genius_sent_to_pim: bool,
    pub territory:          String,
    pub started_at:         u64,
    pub last_activity_at:   u64,
    pub total_turns:        u32,
}

/// ProgressiveDebatic — gjendje globale e modulit (sesionet + detektori).
#[derive(Debug, Clone)]
pub struct ProgressiveDebatic {
    pub sessions: HashMap<String, PdSession>,
    pub detector: GeniusDetector,
    pub active:   bool,
}

impl Default for ProgressiveDebatic {
    fn default() -> Self { Self::new() }
}

impl ProgressiveDebatic {
    pub fn new() -> Self {
        ProgressiveDebatic {
            sessions: HashMap::new(),
            detector: GeniusDetector::default(),
            active: true,
        }
    }

    /// open_session — bërthama e autorizuar; nuk ekspozohet jashtë crate-it.
    pub(crate) fn open_session(&mut self, session_id: &str, territory: &str, mode: DebateMode, ts: u64) {
        let trace = EpistemicTrace::new(session_id, territory, ts);
        let session = PdSession {
            session_id: session_id.to_string(),
            active: true,
            current_mode: mode,
            pressure_budget: PressureBudget::new(mode),
            turns: Vec::new(),
            trace,
            genius_signal: None,
            genius_sent_to_pim: false,
            territory: territory.to_string(),
            started_at: ts,
            last_activity_at: ts,
            total_turns: 0,
        };
        self.sessions.insert(session_id.to_string(), session);
    }

    pub fn session(&self, session_id: &str) -> Option<&PdSession> {
        self.sessions.get(session_id)
    }

    /// ingest — bërthama e autorizuar; zero if — match mbi gjendje.
    pub(crate) fn ingest(&mut self, session_id: &str, content: &str, ts: u64)
        -> Result<PdEngineOutput, PdError>
    {
        // Sesioni duhet të ekzistojë (fail-closed). Zero if — match.
        let detector = self.detector.clone();
        let session = match self.sessions.get_mut(session_id) {
            Some(s) => s,
            None => return Err(PdError::SessionNotFound(session_id.to_string())),
        };

        // 1. Ndërto CognitiveSignal nga përmbajtja (concept_map PARA shtimit → returns).
        let turn_index = session.trace.total_turns;
        let signal = extract_signal(content, turn_index, ts, &session.trace.concept_map);

        // 2. Përditëso EpistemicTrace me sinjalin e ri.
        update_trace(&mut session.trace, &signal);
        session.total_turns = session.trace.total_turns;
        session.last_activity_at = ts;

        // 3. Vlerëso GeniusSignal mbi trace-in e përditësuar.
        let genius = detector.evaluate(&session.trace);

        // 4. Vendos përgjigjen e PD (mode-aware, minimale për këtë fazë).
        let (resp_type, hint) = decide_response(session.current_mode, &signal, &session.pressure_budget, &session.trace);

        // 5. Përditëso buxhetin nëse u lëshua kundërshtim. Branchless.
        let is_challenge = resp_type.is_challenge();
        session.pressure_budget.challenges_issued += is_challenge as u32;
        let max_ch = PressureBudget::max_for(session.current_mode);
        session.pressure_budget.budget_exhausted =
            session.pressure_budget.challenges_issued >= max_ch;

        // 6. Ndërto PdTurn.
        let turn = PdTurn {
            turn_id: pd_id(&[session_id.as_bytes(), &turn_index.to_le_bytes(), b"turn"]),
            turn_index,
            user_signal: signal,
            pd_response_type: resp_type,
            pd_response_hint: hint,
            challenge_issued: is_challenge,
            mode_at_turn: session.current_mode,
            genius_check: genius.as_ref().map(|g| g.genius_score),
        };
        session.turns.push(turn.clone());

        // 7. Output: GeniusDetected (→ paketë për PIM) ose Continue / BudgetExhausted.
        match genius {
            Some(g) => {
                session.genius_signal = Some(g.clone());
                let package = build_package(session, &g);
                Ok(PdEngineOutput::GeniusDetected { genius_signal: g, package })
            }
            None => match session.pressure_budget.budget_exhausted {
                true  => Ok(PdEngineOutput::BudgetExhausted { trace_id: session.trace.trace_id.clone() }),
                false => Ok(PdEngineOutput::Continue { turn }),
            },
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// extract_signal — nxjerr akset semantike REALE nga përmbajtja. Zero if.
// ─────────────────────────────────────────────────────────────────────────────
pub fn extract_signal(
    content: &str,
    turn_index: u32,
    ts: u64,
    prior_concepts: &HashMap<String, u32>,
) -> CognitiveSignal {
    let words = tokenize(content);
    let content_length = content.chars().count() as u32;

    // Konceptet: fjalë domethënëse (≥4 shkronja), unike, të renditura.
    let mut concept_refs: Vec<String> = words.iter()
        .filter(|w| w.chars().count() >= 4)
        .cloned().collect();
    concept_refs.sort();
    concept_refs.dedup();

    // idea_density: unik/total i fjalëve.
    let total = words.len().max(1);
    let unique: HashSet<&String> = words.iter().collect();
    let idea_density = FixedMass::from_ratio(unique.len() as u32, total as u32).raw();

    // energy_level: 80% gjatësi + 20% theks, fixed u32 [0..10_000].
    let emphasis = content.chars().filter(|&c| c == '!' || c == '?').count() as u32;
    let length_mass = FixedMass::from_ratio(content_length.min(160), 160).raw();
    let emphasis_mass = FixedMass::from_ratio(emphasis.min(3), 3).raw();
    let energy_level = (
        u64::from(length_mass).saturating_mul(8_000)
            .saturating_add(u64::from(emphasis_mass).saturating_mul(2_000))
        / u64::from(MASS_SCALE)
    ).min(u64::from(MASS_SCALE)) as u32;

    // returns_to: koncepte që KANË dalë më parë (kthim te e njëjta ide).
    let returns_to: Vec<String> = concept_refs.iter()
        .filter(|c| prior_concepts.contains_key(*c))
        .cloned().collect();

    // contradicts: nëse sinjali ka fjalë mohuese, konceptet që mohon.
    let has_negation = words.iter().any(|w| matches!(w.as_str(),
        "not" | "never" | "no" | "nuk" | "jo" | "kurre" | "pa"));
    let contradicts: Vec<String> = match has_negation {
        true  => concept_refs.clone(),
        false => Vec::new(),
    };

    CognitiveSignal {
        signal_id: pd_id(&[content.as_bytes(), &ts.to_le_bytes()]),
        turn_index,
        raw_content: content.to_string(),
        content_length,
        idea_density,
        energy_level,
        concept_refs,
        contradicts,
        returns_to,
        timestamp: ts,
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// update_trace — akumulon sinjalin te trace-i (dominanti = kandidati mbijetues).
// ─────────────────────────────────────────────────────────────────────────────
pub fn update_trace(trace: &mut EpistemicTrace, signal: &CognitiveSignal) {
    trace.signals.push(signal.clone());
    trace.total_turns += 1;

    // concept_map: numëro shfaqjet e çdo koncepti.
    for c in &signal.concept_refs {
        *trace.concept_map.entry(c.clone()).or_insert(0) += 1;
    }
    trace.energy_trajectory.push(signal.energy_level);
    trace.idea_evolution.push(signal.idea_density);

    // dominant_concept: më i shpeshti (kandidati që i mbijetoi eliminimit).
    trace.dominant_concept = most_frequent_concept(&trace.concept_map);

    // accumulated_mass: mesatarja e dendësisë së ideve deri tani.
    trace.accumulated_mass = mean(&trace.idea_evolution).min(MASS_SCALE);

    // structural_coherence: fokusi = sa herë del dominanti / total kthesa.
    let dom_count = trace.dominant_concept.as_ref()
        .and_then(|d| trace.concept_map.get(d)).copied().unwrap_or(0);
    trace.structural_coherence =
        FixedMass::from_ratio(dom_count, trace.total_turns.max(1)).raw();

    // implicit_primitives: koncepte që rishfaqen (count ≥ 2) — strukturë në lindje.
    let mut prims: Vec<String> = trace.concept_map.iter()
        .filter(|(_, &c)| c >= 2)
        .map(|(k, _)| k.clone())
        .collect();
    prims.sort();
    trace.implicit_primitives = prims;

    // contradiction_pairs: çift (koncept i mohuar, dominant) — tension produktiv.
    let dom = trace.dominant_concept.clone().unwrap_or_default();
    for c in &signal.contradicts {
        let pair = (c.clone(), dom.clone());
        // Shto vetëm nëse i ri DHE jo vetë-çift. Zero if — match mbi (a,b).
        let novel = !trace.contradiction_pairs.contains(&pair);
        let distinct = c != &dom;
        match novel && distinct {
            true  => trace.contradiction_pairs.push(pair),
            false => {}
        }
    }

    trace.last_updated_at = signal.timestamp;
}

// ─────────────────────────────────────────────────────────────────────────────
// decide_response — vendim minimal mode-aware (faza 1). Zero if — match.
// (Logjika e plotë e 4 mode-ve me scaffold-dialog = pd_modes, faza 2.)
// ─────────────────────────────────────────────────────────────────────────────
/// Pragjet e PROGRESSIVE DEBATIC (i trashë, i Arkitektit):
/// PD e "lodh deri diku" përdoruesin me pyetje të SYNUARA për të nxjerrë
/// indicie; kur akumulon MJAFTUESHËM (masë ≥ prag OSE tavan turn-esh),
/// NDALON së pyeturi dhe kalon në zgjidhje — s'e lodh më.
pub const PROBE_SATURATION_MASS: u32 = 6_000;
pub const PROBE_MAX_TURNS:       u32 = 6;

pub fn decide_response(
    mode: DebateMode,
    signal: &CognitiveSignal,
    budget: &PressureBudget,
    trace:  &EpistemicTrace,
) -> (PdResponseType, String) {
    // ══ SATURIMI (thelbi i PD-së së trashë): mjaftueshëm indicie → NDAL
    // pyetjet, kalo në zgjidhje. Masa reale e akumuluar + tavani i turn-eve.
    let saturated = trace.accumulated_mass >= PROBE_SATURATION_MASS
                 || trace.total_turns      >= PROBE_MAX_TURNS;
    match saturated {
        true => return (PdResponseType::Listen,
            "Kam mjaftueshëm indicie — po e zgjidh tani, s'të lodh më me pyetje.".to_string()),
        false => {}
    }
    // Buxheti shteroi → dëgjo, lër PRO të punojë.
    match budget.budget_exhausted {
        true  => return (PdResponseType::Listen,
                         "dëgjo — buxheti shteroi, lë PRO të punojë".to_string()),
        false => {}
    }
    // Rezistencë → ndihmo pa shtytje.
    match budget.user_resistance_detected {
        true  => return (PdResponseType::Help,
                         "ndihmo pa shtytje — user ka rezistencë".to_string()),
        false => {}
    }

    // ══ PYETJE TË SYNUARA (strukturuesi që NXJERR indicie): boshllëku më i
    // madh i sinjalit REAL përcakton pyetjen — jo pyetje gjenerike.
    // 1) Kundërthënie e kapur → sqarimi i saj është indicia më e vlefshme.
    match signal.contradicts.first() {
        Some(c) => return (PdResponseType::ChallengeGentle, format!(
            "Vërej një kundërthënie rreth «{}» — cila anë qëndron dhe pse?", c)),
        None => {}
    }
    // 2) Rikthim te i njëjti koncept → thellim i synuar aty.
    match signal.returns_to.first() {
        Some(r) => return (PdResponseType::ScaffoldStructure, format!(
            "Po kthehesh te «{}» — më jep një veti a shembull KONKRET për të.", r)),
        None => {}
    }
    // 3) Energji e ulët → mbaje të gjallë me hap të vogël.
    match signal.energy_level < 3_000 {
        true => return (PdResponseType::Help,
            "Vetëm edhe një detaj i vogël: ku e ke parë/ndeshur këtë saktësisht?".to_string()),
        false => {}
    }

    let high = signal.idea_density >= 5_000;
    let low  = signal.idea_density < 3_000;
    match (high, low) {
        (true, _) => (PdResponseType::Help,
                      "ndihmo drejtpërdrejt — ideja është e qartë".to_string()),
        (false, true) => (PdResponseType::Clarify,
                      "Më jep 2–3 detaje konkrete (emra, veti, kontekst) — çfarë saktësisht kërkon të arrish?".to_string()),
        (false, false) => match mode {
            DebateMode::Intellect  => (PdResponseType::Orient,
                "orienton: 'a mos është kështu? apo kështu?'".to_string()),
            DebateMode::Philosophy => (PdResponseType::ScaffoldBullet,
                "kërko pikat kyçe të konceptit".to_string()),
            DebateMode::Scientific => (PdResponseType::ScaffoldMeasure,
                "kërko matje/kauzalitet: 'si e mat? çfarë e kundërshton?'".to_string()),
            DebateMode::Novel      => (PdResponseType::ScaffoldStructure,
                "kërko strukturë/repetabilitet (kundërshtim shumë i butë)".to_string()),
        },
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// build_package — material final për PIM nga GeniusSignal + trace. Zero if.
// ─────────────────────────────────────────────────────────────────────────────
pub fn build_package(session: &PdSession, genius: &GeniusSignal) -> PdCognitivePackage {
    let accumulated_text = session.trace.signals.iter()
        .map(|s| s.raw_content.clone())
        .collect::<Vec<_>>()
        .join(" | ");

    // PD nuk kap fact_candidate → kategori: Hypothesis / PotentialHypothesis.
    let suggested_category = match genius.genius_score >= 7_000 {
        true  => "Hypothesis",
        false => "PotentialHypothesis",
    }.to_string();
    let confidence_hint = genius.genius_score.clamp(3_000, 7_000);

    // estimated_trl nga masa e akumuluar (0/1/2 — PD s'shkon mbi 2).
    let estimated_trl: u8 = match session.trace.accumulated_mass {
        m if m >= 6_500 => 2,
        m if m >= 3_600 => 1,
        _               => 0,
    };

    let pre = PdPreClassification {
        suggested_category,
        confidence_hint,
        reasoning: format!(
            "PD: {} sinjale aktive, koncepti '{}' fitoi strukturë gjatë {} kthesave",
            [genius.returns_to_idea, genius.energy_increasing, genius.concept_gaining_shape]
                .iter().filter(|&&a| a).count(),
            genius.detected_concept, session.trace.total_turns),
    };

    PdCognitivePackage {
        package_id: pd_id(&[session.session_id.as_bytes(), &genius.timestamp.to_le_bytes(), b"pkg"]),
        session_id: session.session_id.clone(),
        genius_signal: genius.clone(),
        pre_classification: pre,
        accumulated_text,
        dominant_concept: genius.detected_concept.clone(),
        concept_territory: session.territory.clone(),
        accumulated_mass: session.trace.accumulated_mass,
        structural_coherence: session.trace.structural_coherence,
        accumulated_turns: session.trace.total_turns,
        estimated_trl,
        origin: "PROGRESSIVE_DEBATIC".to_string(),
        created_at: genius.timestamp,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pd_runtime_rejects_wrong_seal_before_session_mutation() {
        let mut pd = ProgressiveDebatic::new();
        let wrong = PD_SEAL ^ 0x01;
        let result = pd.open_session_sealed(
            "sealed", "science", DebateMode::Intellect, 1, wrong,
        );
        assert!(matches!(result, Err(PdError::SealInvalid(_))));
        assert!(pd.session("sealed").is_none());
    }

    #[test]
    fn pd_runtime_accepts_canonical_seal_and_ingests() {
        let mut pd = ProgressiveDebatic::new();
        assert!(pd.open_session_sealed(
            "sealed", "science", DebateMode::Intellect, 1, PD_SEAL,
        ).is_ok());
        assert!(pd.ingest_sealed(
            "sealed", "deterministic elimination preserves meaning", 2, PD_SEAL,
        ).is_ok());
    }

    #[test]
    fn pd_runtime_rejects_wrong_ingest_seal_before_trace_mutation() {
        let mut pd = ProgressiveDebatic::new();
        assert!(pd.open_session_sealed(
            "sealed", "science", DebateMode::Intellect, 1, PD_SEAL,
        ).is_ok());
        let wrong = PD_SEAL ^ 0x01;
        let result = pd.ingest_sealed(
            "sealed", "this must never enter the epistemic trace", 2, wrong,
        );
        assert!(matches!(result, Err(PdError::SealInvalid(_))));
        assert_eq!(pd.session("sealed").map(|s| s.trace.total_turns), Some(0));
    }

    #[test]
    fn session_not_found_is_fail_closed() {
        let mut pd = ProgressiveDebatic::new();
        let r = pd.ingest("missing", "hello", 1);
        assert!(matches!(
            r,
            Err(PdError::SessionNotFound(ref session_id)) if session_id == "missing"
        ));
    }

    #[test]
    fn returns_detected_on_concept_reuse() {
        let mut pd = ProgressiveDebatic::new();
        pd.open_session("s", "science", DebateMode::Philosophy, 0);
        pd.ingest("s", "elimination shapes the space", 1).ok();
        pd.ingest("s", "elimination again proves the pair", 2).ok();
        let t = &pd.session("s").unwrap().trace;
        // "elimination" u rishfaq → është te returns_to i sinjalit të dytë.
        assert!(t.signals[1].returns_to.iter().any(|c| c == "elimination"));
        // dhe u bë primitive implicite (count ≥ 2).
        assert!(t.implicit_primitives.iter().any(|c| c == "elimination"));
    }

    #[test]
    fn dominant_concept_survives() {
        let mut pd = ProgressiveDebatic::new();
        pd.open_session("s", "science", DebateMode::Scientific, 0);
        pd.ingest("s", "elimination elimination matters", 1).ok();
        pd.ingest("s", "elimination removes noise", 2).ok();
        pd.ingest("s", "other random words appear", 3).ok();
        let t = &pd.session("s").unwrap().trace;
        // Koncepti që del më shpesh "mbijeton" si dominant.
        assert_eq!(t.dominant_concept.as_deref(), Some("elimination"));
        assert!(t.structural_coherence > 0);
    }

    #[test]
    fn contradiction_pair_recorded_on_negation() {
        let mut pd = ProgressiveDebatic::new();
        pd.open_session("s", "industrial", DebateMode::Scientific, 0);
        pd.ingest("s", "reactor stable under load", 1).ok();
        pd.ingest("s", "reactor not stable under load", 2).ok();
        let t = &pd.session("s").unwrap().trace;
        // Mohimi prodhoi të paktën një çift kontradikte produktive.
        assert!(!t.contradiction_pairs.is_empty());
    }

    #[test]
    fn package_estimated_trl_capped_at_two() {
        let mut pd = ProgressiveDebatic::new();
        pd.open_session("s", "science", DebateMode::Philosophy, 0);
        for i in 0..5 {
            pd.ingest("s", "elimination is the operator that proves the decision binary truly here now",
                i as u64 + 1).ok();
        }
        let sess = pd.session("s").unwrap().clone();
        // Sintetizo një genius me score të lartë për të provuar paketimin.
        let g = GeniusSignal {
            signal_id: "x".into(), session_id: "s".into(),
            returns_to_idea: true, energy_increasing: true, concept_gaining_shape: true,
            returns_score: 8_000, energy_score: 7_000, shape_score: 7_000, genius_score: 7_500,
            detected_concept: "elimination".into(), detected_at_turn: 5,
            trace_ref: sess.trace.trace_id.clone(), timestamp: 6,
        };
        let pkg = build_package(&sess, &g);
        assert!(pkg.estimated_trl <= 2);
        assert_eq!(pkg.origin, "PROGRESSIVE_DEBATIC");
        assert_eq!(pkg.pre_classification.suggested_category, "Hypothesis");
        assert!(pkg.pre_classification.confidence_hint <= 7_000);
    }
}
