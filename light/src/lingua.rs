// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LINGUA.RS — ESS-MAI LIGHT PLATFORM                                      ║
// ║  Gjata Legacy™  |  Arkitekt: Bledar Gjata                                ║
// ║                                                                          ║
// ║  "Oratori" gjuhësor — DY GJUHË FULL (Shqip + English), NJË version       ║
// ║  ekzekutiv (pure Rust, zero varësi të jashtme, zero modele, offline).    ║
// ║                                                                          ║
// ║  INPUT SIDE (para trace):                                                ║
// ║    parse_input(raw) → SystemQuery                                        ║
// ║      detect_lang()    → SQ / EN                                          ║
// ║      normalize()      → pastro noise (tokenizimi i paprekur)             ║
// ║      extract_intent() → Query/Command/Hypothesis/Statement               ║
// ║      extract_terms()  → fjalët kyçe për pipeline                         ║
// ║      signal_type()    → Effect/Pattern/Contradiction/PartialCause        ║
// ║                                                                          ║
// ║  OUTPUT SIDE (pas Shadow verdict + SHA256 + KODUNIK):                    ║
// ║    format_output(content, lang) → tekst akademik SQ ose EN               ║
// ║                                                                          ║
// ║  LIGJE: zero if/else (vetëm match), zero rrjet, zero AI të jashtme.      ║
// ║  Kontrata publike (§ API) e pandryshuar për light_coordinator.           ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::trace_info::DomainHint;

// ════════════════════════════════════════════════════════════════════════════
// §1  DETECTED LANG — 2 gjuhë FULL + Unknown
// ════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, PartialEq)]
pub enum DetectedLang {
    En,       // English — full
    Sq,       // Shqip — full
    Unknown,  // E padetektuar
}

impl DetectedLang {
    pub fn code(&self) -> &str {
        match self {
            Self::En      => "EN",
            Self::Sq      => "SQ",
            Self::Unknown => "UNK",
        }
    }

    pub fn label(&self) -> &str {
        match self {
            Self::En      => "English",
            Self::Sq      => "Shqip",
            Self::Unknown => "Unknown",
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// §2  INTENT — çfarë dëshiron user
// ════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, PartialEq)]
pub enum Intent {
    Query,      // kërkon informacion
    Command,    // urdhëron veprim
    Hypothesis, // propozon hipotezë
    Statement,  // deklaron fakt
}

impl Intent {
    pub fn label(&self) -> &str {
        match self {
            Self::Query      => "QUERY",
            Self::Command    => "COMMAND",
            Self::Hypothesis => "HYPOTHESIS",
            Self::Statement  => "STATEMENT",
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// §3  SIGNAL TYPE — për quantum operator selection
// ════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, PartialEq)]
pub enum SignalType {
    Effect,         // PRO-REVERSE
    Pattern,        // PRO-MIRROR
    Contradiction,  // PRO-CONTRAST
    PartialCause,   // PRO-TRACE
}

impl SignalType {
    pub fn label(&self) -> &str {
        match self {
            Self::Effect        => "Effect",
            Self::Pattern       => "Pattern",
            Self::Contradiction => "Contradiction",
            Self::PartialCause  => "PartialCause",
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// §4  SYSTEM QUERY — output i lingua për pipeline
// ════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub struct SystemQuery {
    /// Gjuha e detektuar
    pub lang:        DetectedLang,

    /// Teksti i normalizuar — pastër, gati për pipeline
    pub normalized:  String,

    /// Intent i user
    pub intent:      Intent,

    /// Signal type për quantum operators
    pub signal_type: SignalType,

    /// Fjalët kyçe të domain
    pub domain_terms: Vec<String>,

    /// Confidence i detektimit [0,1]
    pub confidence:  f32,
}

impl SystemQuery {
    /// Konverto DetectedLang → DomainHint për trace_info
    pub fn domain_hint(&self) -> DomainHint {
        // Detektim i thjeshtë nga terms — light coordinator e rafinon me nura_core
        let text = self.normalized.to_lowercase();
        let work_kw: &[&str] = &[
            "scada","industry","business","contract","governance",
            "motor","sensor","industri","biznes","kontratë",
        ];
        let home_kw: &[&str] = &[
            "home","family","house","matter","iot","device",
            "shtëpi","familje","pajisje",
        ];
        let outside_kw: &[&str] = &[
            "mobile","web","outside","browse","internet",
            "jashtë","shfleto","kërko",
        ];

        let w = work_kw.iter().filter(|&&k| text.contains(k)).count();
        let h = home_kw.iter().filter(|&&k| text.contains(k)).count();
        let o = outside_kw.iter().filter(|&&k| text.contains(k)).count();

        match (h > w && h > o, o > w && o > h, w > 0) {
            (true, _, _) => DomainHint::Home,
            (_, true, _) => DomainHint::Outside,
            (_, _, true) => DomainHint::Work,
            _            => DomainHint::Unknown,
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// §5  TIER — NJË version ekzekutiv (pure Rust). Pa modele, pa stub-e.
//     RAM lexohet vetëm për status/telemetri; s'ndryshon sjelljen.
// ════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, PartialEq)]
pub enum Tier {
    One,   // Pure Rust templates — ekzekutiv, i vetmi version aktiv
}

impl Tier {
    pub fn label(&self) -> &str {
        match self {
            Self::One => "TIER1_RUST",
        }
    }
}

/// Kthen tier-in ekzekutiv (një version i vetëm).
pub fn detect_tier() -> Tier {
    Tier::One
}

/// RAM disponibël në MB — Linux: /proc/meminfo; Windows/të tjera: fallback.
/// Përdoret vetëm për status; kurrë s'e ndal rrjedhën (fail-open telemetri).
fn available_ram_mb() -> u64 {
    #[cfg(target_os = "linux")]
    {
        match std::fs::read_to_string("/proc/meminfo") {
            Ok(content) => {
                for line in content.lines() {
                    match line.starts_with("MemAvailable:") {
                        true => {
                            let kb: u64 = line.split_whitespace()
                                .nth(1)
                                .unwrap_or("0")
                                .parse()
                                .unwrap_or(0);
                            return kb / 1024;
                        }
                        false => {}
                    }
                }
            }
            Err(_) => eprintln!("[LINGUA] SHËNIM: /proc/meminfo s'u lexua — RAM-detekt në fallback nominal (s'ndikon tier-in e vetëm)"),
        }
        2048
    }
    #[cfg(not(target_os = "linux"))]
    {
        2048 // fallback nominal — s'ndikon sjelljen (një tier i vetëm)
    }
}

// ════════════════════════════════════════════════════════════════════════════
// §6  NORMALIZER — pure Rust, O(1). TOKENIZIMI I PAPREKUR.
// ════════════════════════════════════════════════════════════════════════════

pub struct Normalizer;

impl Normalizer {
    /// Normalizo input: trim, whitespace, control chars, quote fix
    pub fn normalize(text: &str) -> String {
        let mut result = String::with_capacity(text.len());
        let mut prev_space = false;

        for ch in text.chars() {
            match ch {
                // Control characters (tab → space, newline → space)
                '\t' | '\n' | '\r' => {
                    match !prev_space && !result.is_empty() {
                        true => {
                            result.push(' ');
                            prev_space = true;
                        }
                        false => {}
                    }
                }
                // Multiple spaces → single
                ' ' => {
                    match !prev_space && !result.is_empty() {
                        true => {
                            result.push(' ');
                            prev_space = true;
                        }
                        false => {}
                    }
                }
                // Normalize curly quotes → straight
                '\u{201C}' | '\u{201D}' => { result.push('"'); prev_space = false; }
                '\u{2018}' | '\u{2019}' => { result.push('\''); prev_space = false; }
                // Zero-width direction markers — removed
                '\u{200F}' | '\u{200E}' => {}
                // Emoji and special unicode blocks — remove
                c if (c as u32) > 0x1F000 && (c as u32) < 0x1FFFF => {}
                // Normal chars
                c => {
                    result.push(c);
                    prev_space = c == ' ';
                }
            }
        }

        result.trim().to_string()
    }

    /// Heq stopwords dhe kthen content words (tokenizimi: split → filter → lower)
    pub fn content_words(text: &str, lang: &DetectedLang) -> Vec<String> {
        let stopwords = Self::stopwords(lang);
        text.split_whitespace()
            .filter(|w| {
                let lower = w.to_lowercase();
                let clean: String = lower.chars()
                    .filter(|c| c.is_alphabetic())
                    .collect();
                clean.len() > 2 && !stopwords.contains(&clean.as_str())
            })
            .map(|w| w.to_lowercase())
            .collect()
    }

    /// Stopwords FULL për dy gjuhët. (Fjalët 1–2 shkronjëshe filtrohen vetiu
    /// nga rregulli len>2 i tokenizimit; listat mbajnë format 3+ shkronjëshe.)
    fn stopwords(lang: &DetectedLang) -> &'static [&'static str] {
        match lang {
            DetectedLang::En => &[
                "the","and","for","are","was","were","been","being","have","has",
                "had","this","that","these","those","with","from","into","onto",
                "over","under","about","after","before","between","through","during",
                "above","below","again","then","once","here","there","when","where",
                "why","how","all","any","both","each","few","more","most","other",
                "some","such","nor","not","only","own","same","than","too","very",
                "can","will","just","should","could","would","now","does","did",
                "but","because","until","while","against","you","your","yours",
                "they","them","their","theirs","she","her","hers","him","his","its",
                "what","which","who","whom","out","off","down",
            ],
            DetectedLang::Sq => &[
                "dhe","ose","për","nga","një","por","është","janë","jam","jemi",
                "jeni","kjo","ky","kët","ajo","ata","ato","atë","këto","këta",
                "duke","pas","para","mbi","nën","tek","deri","që","kur","pse",
                "çfarë","cila","cili","cilët","nuk","mos","edhe","apo","ndaj",
                "pra","vetëm","shumë","pak","gjithë","disa","tij","saj","tyre",
                "ynë","jonë","ime","tim","tënd","tuaj","kanë","kishte","ishte",
                "qenë","bëhet","bëri","gjatë","sipas","brenda","jashtë","midis",
                "ndër","tjetër","tjera","gjithashtu","kështu","ashtu","aty","këtu",
            ],
            DetectedLang::Unknown => &[],
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// §7  LANG DETECTOR — pure-Rust, offline, deterministik (SQ / EN).
//     Sovraniteti: ESS-MAI kompilon & ekzekuton krejt air-gapped, zero varësi.
//     Strategjia: stopword scoring EN/SQ + diacritic hints (ë, ç) për SQ.
//     Confidence = score_top / sum → [0,1], dysheme 0.30. Ky është "hint" —
//     Quantum & Shadow rivlerësojnë; Shadow s'i beson verbërisht.
// ════════════════════════════════════════════════════════════════════════════

pub struct LangDetector;

impl LangDetector {
    /// Detekton gjuhën + kthen confidence [0,1]. Pure-Rust, zero rrjet.
    pub fn detect(text: &str) -> (DetectedLang, f32) {
        match text.trim().is_empty() {
            true => return (DetectedLang::Unknown, 0.0),
            false => {}
        }
        detect_pure_rust(text)
    }
}

fn detect_pure_rust(text: &str) -> (DetectedLang, f32) {
    let lower = text.to_lowercase();

    // ── 1. Stopword scoring EN / SQ ────────────────────────────────────────
    // Fjalë funksionale me hapësira rreth tyre → match si " word ".
    const EN: &[&str] = &[
        " the "," and "," is "," of "," to "," in "," that "," it "," for ",
        " what "," how "," why "," are "," this "," with "," from "," not ",
        " you "," have "," will "," can ",
    ];
    const SQ: &[&str] = &[
        " dhe "," është "," të "," një "," me "," për "," çfarë "," si ",
        " pse "," nuk "," janë "," ku "," kjo "," nga "," por "," ose ",
        " kam "," do "," duhet "," mund ",
    ];

    // Mbështjellim me hapësira që " the " të kapë fjalën në fillim/fund.
    let padded = format!(" {} ", lower);

    let count = |stop: &[&str]| -> f32 {
        stop.iter().map(|w| padded.matches(w).count() as f32).sum()
    };

    let mut scores = [
        (DetectedLang::En, count(EN)),
        (DetectedLang::Sq, count(SQ)),
    ];

    // ── 2. Diacritic hints — shenjat unike shqipe (ë, ç) ───────────────────
    let bump = |lang: DetectedLang, scores: &mut [(DetectedLang, f32); 2], by: f32| {
        for s in scores.iter_mut() {
            match s.0 == lang { true => s.1 += by, false => {} }
        }
    };
    match lower.contains('ë') || lower.contains('ç') {
        true  => bump(DetectedLang::Sq, &mut scores, 2.0),
        false => {}
    }

    // ── 3. Zgjedhja + confidence ───────────────────────────────────────────
    let sum: f32 = scores.iter().map(|(_, s)| *s).sum();
    let (top_lang, top_score) = scores.iter()
        .fold((DetectedLang::Unknown, 0.0f32), |acc, (l, s)| {
            match *s > acc.1 { true => (l.clone(), *s), false => acc }
        });

    match top_score <= 0.0 || sum <= 0.0 {
        true => {
            // Asnjë sinjal → English si default i butë me confidence të ulët
            return (DetectedLang::En, 0.30);
        }
        false => {}
    }

    let conf = (top_score / sum).clamp(0.30, 1.0);
    (top_lang, conf)
}

// ════════════════════════════════════════════════════════════════════════════
// §8  INTENT ENGINE — pure Rust, keyword-based (EN + SQ)
// ════════════════════════════════════════════════════════════════════════════

pub struct IntentEngine;

impl IntentEngine {
    /// Ekstrakton intent nga teksti
    pub fn extract_intent(text: &str) -> Intent {
        let t = text.trim();

        // Pyetje?
        let is_query = t.ends_with('?')
            || t.to_lowercase().starts_with("what")
            || t.to_lowercase().starts_with("how")
            || t.to_lowercase().starts_with("why")
            || t.to_lowercase().starts_with("when")
            || t.to_lowercase().starts_with("çfarë")
            || t.to_lowercase().starts_with("cfarë")
            || t.to_lowercase().starts_with("si ")
            || t.to_lowercase().starts_with("pse")
            || t.to_lowercase().starts_with("kur ")
            || t.contains("?");
        match is_query {
            true => return Intent::Query,
            false => {}
        }

        // Hipotezë?
        let hyp_kw: &[&str] = &[
            "hypothesis","hipotezë","hipoteze",
            "suppose","supozoj","supozojmë","assume","perhaps",
            "if we","nëse","sikur",
        ];
        let tl = t.to_lowercase();
        match hyp_kw.iter().any(|k| tl.contains(k)) {
            true => return Intent::Hypothesis,
            false => {}
        }

        // Urdhër?
        let cmd_kw: &[&str] = &[
            "show","list","create","generate","run","execute",
            "bëj","krijo","shfaq","listo","gjenero","ekzekuto","nis",
        ];
        let is_command = cmd_kw.iter().any(|k| tl.starts_with(k) || tl.contains(" !"))
            || t.ends_with('!');
        match is_command {
            true => return Intent::Command,
            false => {}
        }

        Intent::Statement
    }

    /// Detekton signal type për quantum operator selection
    pub fn signal_type(text: &str) -> SignalType {
        let t = text.to_lowercase();

        // Contradiction — PRO-CONTRAST
        let contra: &[&str] = &[
            "but","however","although","despite","yet",
            "nuk","por","megjithatë","megjithëse","ndonëse","kundër",
        ];
        match contra.iter().any(|k| t.contains(k)) {
            true => return SignalType::Contradiction,
            false => {}
        }

        // Effect (causal — PRO-REVERSE)
        let effect: &[&str] = &[
            "because","therefore","thus","hence","caused by","due to",
            "prandaj","sepse","shkaktuar","rrjedhimisht","si pasojë",
        ];
        match effect.iter().any(|k| t.contains(k)) {
            true => return SignalType::Effect,
            false => {}
        }

        // Partial Cause (abductive — PRO-TRACE)
        let partial: &[&str] = &[
            "maybe","perhaps","possibly","might","could","probably",
            "ndoshta","mbase","mundet","ka gjasa","me siguri",
        ];
        match partial.iter().any(|k| t.contains(k)) {
            true => return SignalType::PartialCause,
            false => {}
        }

        SignalType::Pattern // default — PRO-MIRROR
    }

    /// Ekstrakton domain terms (content words filtrat)
    pub fn domain_terms(text: &str, lang: &DetectedLang) -> Vec<String> {
        let mut terms = Normalizer::content_words(text, lang);
        terms.sort();
        terms.dedup();
        terms.truncate(20); // max 20 terms
        terms
    }
}

// ════════════════════════════════════════════════════════════════════════════
// §9  TIER1 ENGINE — Pure Rust academic templates
//     2 gjuhë FULL × 4 intent types × 2 verdict types — ekzekutiv, zero model
// ════════════════════════════════════════════════════════════════════════════

pub struct Tier1Engine;

impl Tier1Engine {
    pub fn format_output(
        content:  &str,
        lang:     &DetectedLang,
        intent:   &Intent,
        verified: bool,
    ) -> String {
        let template = Self::template(lang, intent, verified);
        template.replace("{content}", content)
    }

    fn template(
        lang:     &DetectedLang,
        intent:   &Intent,
        verified: bool,
    ) -> &'static str {
        match (lang, intent, verified) {
            // ── ENGLISH — FULL ────────────────────────────────────────────
            (DetectedLang::En, Intent::Query, true) =>
                "Based on verified epistemic analysis: {content}. \
                 This determination has been formally validated through \
                 the sovereign verification pipeline.",
            (DetectedLang::En, Intent::Query, false) =>
                "The query could not be conclusively verified at this stage. \
                 Current epistemic assessment: {content}. \
                 Further evidence is required for formal validation.",
            (DetectedLang::En, Intent::Hypothesis, true) =>
                "The proposed hypothesis has been evaluated and confirmed: {content}. \
                 Verification status: Optimal. Formal epistemic seal applied.",
            (DetectedLang::En, Intent::Hypothesis, false) =>
                "The proposed hypothesis requires additional evidence: {content}. \
                 Current classification: Potential. Shadow hold pending.",
            (DetectedLang::En, Intent::Command, true) =>
                "The command has been executed and verified: {content}. \
                 Epistemic integrity confirmed. Sovereign seal applied.",
            (DetectedLang::En, Intent::Command, false) =>
                "The command was received; execution is pending verification: {content}.",
            (DetectedLang::En, Intent::Statement, true) =>
                "Verified output: {content}. \
                 Epistemic integrity confirmed. Sovereign seal applied.",
            (DetectedLang::En, Intent::Statement, false) =>
                "Preliminary output pending full verification: {content}.",

            // ── SHQIP — FULL ──────────────────────────────────────────────
            (DetectedLang::Sq, Intent::Query, true) =>
                "Bazuar në analizën epistemike të verifikuar: {content}. \
                 Ky përcaktim është validuar zyrtarisht nëpërmjet \
                 pipeline-it të verifikimit sovran.",
            (DetectedLang::Sq, Intent::Query, false) =>
                "Kërkesa nuk mund të verifikohej përfundimisht në këtë fazë. \
                 Vlerësimi epistemik aktual: {content}. \
                 Kërkohen dëshmi të mëtejshme për validim zyrtar.",
            (DetectedLang::Sq, Intent::Hypothesis, true) =>
                "Hipoteza e propozuar është vlerësuar dhe konfirmuar: {content}. \
                 Statusi i verifikimit: Optimal. Vula epistemike e aplikuar.",
            (DetectedLang::Sq, Intent::Hypothesis, false) =>
                "Hipoteza e propozuar kërkon dëshmi shtesë: {content}. \
                 Klasifikimi aktual: Potencial. Shadow hold në pritje.",
            (DetectedLang::Sq, Intent::Command, true) =>
                "Komanda është ekzekutuar dhe verifikuar: {content}. \
                 Integriteti epistemik i konfirmuar. Vula sovrane e aplikuar.",
            (DetectedLang::Sq, Intent::Command, false) =>
                "Komanda u pranua; ekzekutimi është në pritje të verifikimit: {content}.",
            (DetectedLang::Sq, Intent::Statement, true) =>
                "Output i verifikuar: {content}. \
                 Integriteti epistemik i konfirmuar. Vula sovrane e aplikuar.",
            (DetectedLang::Sq, Intent::Statement, false) =>
                "Output fillestar në pritje të verifikimit të plotë: {content}.",

            // ── UNKNOWN / FALLBACK ────────────────────────────────────────
            (_, _, true)  =>
                "Verified output: {content}. Epistemic seal applied.",
            (_, _, false) =>
                "Preliminary output pending verification: {content}.",
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// §10 LINGUA CORE — API uniforme, kurrë nuk ndryshon
// ════════════════════════════════════════════════════════════════════════════

pub struct LinguaCore {
    pub tier: Tier,
}

impl LinguaCore {
    pub fn new() -> Self {
        Self { tier: detect_tier() }
    }

    /// Me tier manual — për testing ose override
    pub fn with_tier(tier: Tier) -> Self {
        Self { tier }
    }

    // ── INPUT SIDE ────────────────────────────────────────────────────────

    /// INPUT: gjuhë natyrore → SystemQuery për pipeline
    /// Thirrur nga light_coordinator PARA trace_info
    pub fn parse_input(&self, raw: &str) -> SystemQuery {
        // §10.1 — Normalizim
        let normalized = Normalizer::normalize(raw);

        // §10.2 — Detektim gjuhe
        let (lang, confidence) = LangDetector::detect(&normalized);

        // §10.3 — Intent
        let intent = IntentEngine::extract_intent(&normalized);

        // §10.4 — Signal type (për quantum)
        let signal_type = IntentEngine::signal_type(&normalized);

        // §10.5 — Domain terms
        let domain_terms = IntentEngine::domain_terms(&normalized, &lang);

        SystemQuery {
            lang,
            normalized,
            intent,
            signal_type,
            domain_terms,
            confidence,
        }
    }

    // ── OUTPUT SIDE ───────────────────────────────────────────────────────

    /// OUTPUT: sistem → tekst akademik në gjuhën e user (SQ/EN)
    /// Thirrur nga light_coordinator PAS Shadow verdict + SHA256 + KODUNIK
    pub fn format_output(
        &self,
        content:  &str,
        lang:     &DetectedLang,
        intent:   &Intent,
        verified: bool,
    ) -> String {
        // Një version ekzekutiv: Tier1 pure-Rust, direkt, pa zinxhir fallback.
        Tier1Engine::format_output(content, lang, intent, verified)
    }

    /// Status i LinguaCore
    pub fn status(&self) -> String {
        format!("LinguaCore → tier:{} ram:{}MB",
            self.tier.label(),
            available_ram_mb())
    }
}

impl Default for LinguaCore {
    fn default() -> Self { Self::new() }
}

// ════════════════════════════════════════════════════════════════════════════
// §11 TESTE — dy gjuhët full + tokenizimi i paprekur
// ════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_albanian_full() {
        let (lang, conf) = LangDetector::detect("Çfarë është integriteti epistemik dhe pse duhet?");
        assert_eq!(lang, DetectedLang::Sq);
        assert!(conf >= 0.30);
    }

    #[test]
    fn detect_english_full() {
        let (lang, conf) = LangDetector::detect("What is the epistemic integrity of this system?");
        assert_eq!(lang, DetectedLang::En);
        assert!(conf >= 0.30);
    }

    #[test]
    fn detect_empty_is_unknown() {
        let (lang, conf) = LangDetector::detect("   ");
        assert_eq!(lang, DetectedLang::Unknown);
        assert_eq!(conf, 0.0);
    }

    #[test]
    fn tokenizer_preserved_content_words() {
        // Tokenizimi: split → filter(len>2, jo stopword) → lowercase. I PAPREKUR.
        let words = Normalizer::content_words(
            "The sovereign system verifies epistemic integrity", &DetectedLang::En);
        assert!(words.contains(&"sovereign".to_string()));
        assert!(words.contains(&"system".to_string()));
        assert!(words.contains(&"verifies".to_string()));
        assert!(!words.contains(&"the".to_string())); // stopword hiqet
    }

    #[test]
    fn tokenizer_albanian_stopwords() {
        let words = Normalizer::content_words(
            "Sistemi sovran verifikon integritetin epistemik dhe vulën", &DetectedLang::Sq);
        assert!(words.contains(&"sistemi".to_string()));
        assert!(words.contains(&"verifikon".to_string()));
        assert!(!words.contains(&"dhe".to_string())); // stopword shqip hiqet
    }

    #[test]
    fn normalize_unchanged_semantics() {
        assert_eq!(Normalizer::normalize("  a\t\tb\n\nc  "), "a b c");
        assert_eq!(Normalizer::normalize("\u{201C}quote\u{201D}"), "\"quote\"");
    }

    #[test]
    fn intent_query_both_languages() {
        assert_eq!(IntentEngine::extract_intent("Pse funksionon vula sovrane?"), Intent::Query);
        assert_eq!(IntentEngine::extract_intent("How does the seal work?"), Intent::Query);
    }

    #[test]
    fn intent_command_both_languages() {
        assert_eq!(IntentEngine::extract_intent("Krijo raportin final"), Intent::Command);
        assert_eq!(IntentEngine::extract_intent("Generate the final report"), Intent::Command);
    }

    #[test]
    fn templates_full_coverage_sq_en() {
        // 2 gjuhë × 4 intents × 2 verdicts — çdo kombinim kthen template real.
        let langs   = [DetectedLang::Sq, DetectedLang::En];
        let intents = [Intent::Query, Intent::Command, Intent::Hypothesis, Intent::Statement];
        for lang in langs.iter() {
            for intent in intents.iter() {
                for v in [true, false] {
                    let out = Tier1Engine::format_output("X", lang, intent, v);
                    assert!(out.contains('X'));
                    assert!(out.len() > 10);
                }
            }
        }
    }

    #[test]
    fn single_executive_tier() {
        let core = LinguaCore::new();
        assert_eq!(core.tier, Tier::One);
        let out = core.format_output("verdikt", &DetectedLang::Sq, &Intent::Statement, true);
        assert!(out.contains("verdikt"));
        assert!(out.contains("Vula sovrane"));
    }
}
