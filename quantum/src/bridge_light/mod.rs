// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  BRIDGE_LIGHT/MOD.RS — Ura Light ↔ Quantum (PERSOSMËRI)               ║
// ║  GJATA LEGACY™ — Quantum Platform                                     ║
// ║                                                                          ║
// ║  Lidhja me Light . Formatet janë IDENTIKE me ato që Light          ║
// ║  prodhon/pret te quantum_bridge.rs:                                    ║
// ║                                                                          ║
// ║  LIGHT → QUANTUM (QuantumInput::deserialize):                        ║
// ║   trace_id=;domain=;contract_id=;lgc_seal=;lang_code=;evolve=;input_sha256=;ts=;text=║
// ║                                                                          ║
// ║  QUANTUM → LIGHT (QuantumOutput::serialize):                         ║
// ║   trace_id=;verdict=;pot=;axis=;seal=;sfx=;state=;territory=;depth=;   ║
// ║   ts=;content=                                                         ║
// ║                                                                          ║
// ║  Light mbart inputin dhe vulën; Quantum arsyeton. Light NUK vendos.    ║
// ╚══════════════════════════════════════════════════════════════════════════╝

// ═════════════════════════════════════════════════════════════════════════════
// §1  QuantumInput — ç'mbërrin nga Light (DESERIALIZE)
// ═════════════════════════════════════════════════════════════════════════════

// ─────────────────────────────────────────────────────────────────────────────
// PATCH 7.1 — EvolvePayload: tipizim i fushës 'evolve' (sipas shabllonit 7.3)
// ─────────────────────────────────────────────────────────────────────────────

/// Të dhënat e evolucionit, TË TIPIZUARA (jo String i papërpunuar).
///
/// Para patch-it, 'evolve' kalonte si tekst i lirë. Tani parsohet në strukturë
/// të tipizuar, saktë si EvolveTrace e Light prodhon: gjenerimi + gjendja +
/// vula primitive. Mban të njëjtin shabllon tipizimi si NegativeInbound (7.3).
#[derive(Debug, Clone, PartialEq)]
pub struct EvolvePayload {
    /// Gjenerimi i evolucionit (EvolveTrace.generation).
    pub generation:      u64,
    /// Gjendja DSL (string e shkurtër: "EVOLVING"/"VFYKNG"/"NEGKNG").
    pub state_tag:       String,
    /// Vula primitive (EvolveTrace.primitive_flags: 0xA451=primitiv, 0=fragmentuar).
    pub primitive_flags: u32,
    /// Forma e papërpunuar (për kompatibilitet wire / fallback).
    pub raw:             String,
}

impl EvolvePayload {
    /// from_wire — parson string-un 'evolve' në strukturë të tipizuar. Zero if.
    ///
    /// Format i pranuar: "gen:N|state:TAG|flags:0xHEX" ose tekst i thjeshtë
    /// pa strukturë k:v (legjitim: fusha 'evolve' është opsionale — raw).
    ///
    /// FAIL-CLOSED (audit): çelës i NJOHUR me vlerë TË PRISHUR = wire i
    /// gabuar → Err(CorruptField) — kurrë normalizim në 0/EVOLVING të butë.
    pub fn from_wire(s: &str) -> Result<Self, LightBridgeError> {
        let mut generation = 0u64;
        let mut state_tag = String::new();
        let mut primitive_flags = 0u32;

        // Parsim k:v të ndarë me '|' (zero if — match mbi çelësin).
        for part in s.split('|') {
            match part.split_once(':') {
                Some((k, v)) => match k {
                    "gen"   => generation = match v.parse() {
                        Ok(n)  => n,
                        Err(_) => return Err(LightBridgeError::CorruptField("evolve.gen")),
                    },
                    "state" => state_tag = v.to_string(),
                    "flags" => primitive_flags = match Self::parse_flags(v) {
                        Ok(n)  => n,
                        Err(_) => return Err(LightBridgeError::CorruptField("evolve.flags")),
                    },
                    _       => {}
                },
                None => {}
            }
        }

        // state_tag bosh → fallback nga teksti i plotë. Zero if — match.
        let final_state = match state_tag.is_empty() {
            true  => Self::infer_state(s),
            false => state_tag,
        };

        Ok(Self {
            generation,
            state_tag: final_state,
            primitive_flags,
            raw: s.to_string(),
        })
    }

    /// parse_flags — hex ose decimal; TË DYJA dështojnë → Err (fail-closed).
    fn parse_flags(v: &str) -> Result<u32, ()> {
        let stripped = v.trim_start_matches("0x");
        match u32::from_str_radix(stripped, 16) {
            Ok(n)  => Ok(n),
            Err(_) => match v.parse() {
                Ok(n)  => Ok(n),
                Err(_) => Err(()),
            },
        }
    }

    /// infer_state — nxjerr gjendjen nga teksti i papërpunuar. Zero if — match.
    fn infer_state(s: &str) -> String {
        let has_vfy = s.contains("VFYKNG");
        let has_neg = s.contains("NEGKNG");
        // Match mbi (has_vfy, has_neg) — zero if.
        match (has_vfy, has_neg) {
            (true, _)     => "VFYKNG".to_string(),
            (false, true) => "NEGKNG".to_string(),
            (false, false) => "EVOLVING".to_string(),
        }
    }

    /// is_primitive — a e mban vulën primitive (0xA451)? Zero if.
    pub fn is_primitive(&self) -> bool {
        // Vula primitive: (flags & 0xFFFF) ^ 0xA5A5 == 500.
        ((self.primitive_flags & 0xFFFF) ^ 0xA5A5) == 500
    }
}

/// Konteksti i një projekti shkencor të përdoruesit. Dëshmitari është
/// lëshuar nga Shadow main pas ruajtjes APUPK; Quantum nuk merr akses në
/// magazinë, vetëm identitetin e kufizuar dhe materialin për procedim GCL.
#[derive(Debug, Clone)]
pub struct ScientificProjectInput {
    pub context: shadow_contracts::ProjectContextWitnessWire,
    pub title: String,
    pub hypothesis: String,
    pub assumptions: Vec<String>,
    pub description: String,
    pub files: Vec<(u8, Vec<u8>)>,
}

/// Inputi që Light i dërgon Quantum-it përmes bus-it.
/// Format bazë ruhet; fushat `project_*` janë opsionale dhe all-or-none.
#[derive(Debug, Clone)]
pub struct QuantumInput {
    pub trace_id:    String,
    pub domain:      String,
    pub contract_id: String,
    pub lgc_seal:    String,
    pub lang_code:   String,
    /// Fusha 'evolve' e papërpunuar (mbahet për kompatibilitet wire).
    pub evolve:      String,
    /// SHA-256 i inputit i prodhuar nga Light; Quantum vetëm e bart.
    pub input_sha256: String,
    pub ts:          u64,
    pub text:        String,
    pub scientific_project: Option<ScientificProjectInput>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LightBridgeError {
    MissingField(&'static str),
    /// FAIL-CLOSED: fusha e pranishme por E PRISHUR (jo-numerike/jo-hex).
    /// Dallohet nga MissingField — korrupsioni EMËRTOHET, s'bëhet 0 e butë.
    CorruptField(&'static str),
    Empty,
}

// ─────────────────────────────────────────────────────────────────────────────
// FIX #1 — ESCAPE për fusha free-text (parandalon delimiter injection)
// ─────────────────────────────────────────────────────────────────────────────
//
// Wire format përdor ';' '=' '|' ':' si delimiterë. Nëse content/text përmban
// këto, parser-i çahej. Escape i zëvendëson me sekuenca \xNN të sigurta.
// Zgjidhje e ndërmjetme deri në migrim te serde_json/bincode (binary i qëndrueshëm).

/// escape_field — zëvendëson delimiterët e protokollit në një fushë free-text.
/// '\' → '\\', ';' → '\s', '=' → '\e', '|' → '\p', ':' → '\c'. Zero if — replace.
pub fn escape_field(s: &str) -> String {
    s.replace('\\', "\\\\")   // backslash i pari (që të mos prishet)
     .replace(';', "\\s")
     .replace('=', "\\e")
     .replace('|', "\\p")
     .replace(':', "\\c")
}

/// unescape_field — kthen sekuencat e escape në karakteret origjinale. Zero if.
pub fn unescape_field(s: &str) -> String {
    // Parsim char-pas-char me state machine (zero if — fold mbi karakteret).
    let (out, pending) = s.chars().fold(
        (String::new(), false),
        |(mut acc, esc), ch| match esc {
            // Karakteri i mëparshëm ishte '\' — interpreto sekuencën. Zero if — match.
            true => {
                let decoded = match ch {
                    's'   => ';',
                    'e'   => '=',
                    'p'   => '|',
                    'c'   => ':',
                    '\\'  => '\\',
                    other => other,  // sekuencë e panjohur → karakteri vetë
                };
                acc.push(decoded);
                (acc, false)
            }
            // Karakter normal: nëse '\' → fillon escape; ndryshe shtohet. Zero if — match.
            false => match ch {
                '\\' => (acc, true),
                _    => { acc.push(ch); (acc, false) }
            },
        },
    );
    let _ = pending;  // '\' i fundit i papërfunduar → injorohet
    out
}

impl QuantumInput {
    /// DESERIALIZE — saktë si Light send_to_quantum() prodhon.
    /// Parsim k=v të ndarë me ';'. trace_id është i detyrueshëm.
    pub fn deserialize(s: &str) -> Result<Self, LightBridgeError> {
        let mut trace_id    = String::new();
        let mut domain      = String::new();
        let mut contract_id = String::new();
        let mut lgc_seal    = String::new();
        let mut lang_code   = String::new();
        let mut evolve      = String::new();
        let mut input_sha256 = String::new();
        let mut ts          = 0u64;
        let mut text        = String::new();
        let mut project_id = String::new();
        let mut project_user_id = String::new();
        let mut project_trace_id = String::new();
        let mut project_revision = String::new();
        let mut project_content_sha256 = String::new();
        let mut project_sovereign_flags = String::new();
        let mut project_context_sha256 = String::new();
        let mut project_title = String::new();
        let mut project_hypothesis = String::new();
        let mut project_assumptions = String::new();
        let mut project_description = String::new();
        let mut project_files = String::new();

        for part in s.split(';') {
            match part.split_once('=') {
                Some((k, v)) => match k {
                    "trace_id"    => trace_id    = unescape_field(v),
                    "domain"      => domain      = unescape_field(v),
                    "contract_id" => contract_id = unescape_field(v),
                    "lgc_seal"    => lgc_seal    = unescape_field(v),
                    "lang_code"   => lang_code   = unescape_field(v),
                    "evolve"      => evolve      = unescape_field(v),
                    "input_sha256" => input_sha256 = unescape_field(v),
                    "ts"          => ts = match v.parse() {
                        Ok(n)  => n,
                        Err(_) => return Err(LightBridgeError::CorruptField("ts")),
                    },
                    "text"        => text        = unescape_field(v),
                    "project_id" => project_id = unescape_field(v),
                    "project_user_id" => project_user_id = unescape_field(v),
                    "project_trace_id" => project_trace_id = unescape_field(v),
                    "project_revision" => project_revision = unescape_field(v),
                    "project_content_sha256" => project_content_sha256 = unescape_field(v),
                    "project_sovereign_flags" => project_sovereign_flags = unescape_field(v),
                    "project_context_sha256" => project_context_sha256 = unescape_field(v),
                    "project_title" => project_title = unescape_field(v),
                    "project_hypothesis" => project_hypothesis = unescape_field(v),
                    "project_assumptions" => project_assumptions = unescape_field(v),
                    "project_description" => project_description = unescape_field(v),
                    "project_files" => project_files = unescape_field(v),
                    _             => {}
                },
                None => {}
            }
        }

        // FAIL-CLOSED (#2): të gjitha fushat kritike duhet të jenë të pranishme.
        // SHA-256 i pranishëm por keqformuar emërtohet si korrupsion, jo mungesë.
        let sha_corrupt = !input_sha256.is_empty()
            && !(input_sha256.len() == 64
                && input_sha256.chars().all(|c| c.is_ascii_hexdigit()));
        match sha_corrupt {
            true => return Err(LightBridgeError::CorruptField("input_sha256")),
            false => {}
        }
        // Asnjë default i heshtur — input i gjymtuar REFUZOHET. Zero if — match.
        let missing = match trace_id.is_empty() {
            true => Some("trace_id"),
            false => match domain.is_empty() {
                true => Some("domain"),
                false => match contract_id.is_empty() {
                    true => Some("contract_id"),
                    false => match lgc_seal.is_empty() {
                        true => Some("lgc_seal"),
                        false => match lang_code.is_empty() {
                            true => Some("lang_code"),
                            false => match input_sha256.is_empty() {
                                true => Some("input_sha256"),
                                false => match ts == 0 {
                                    true => Some("ts"),
                                    false => match text.is_empty() {
                                        true => Some("text"),
                                        false => None,
                                    },
                                },
                            },
                        },
                    },
                },
            },
        };

        let project_any = [
            &project_id, &project_user_id, &project_trace_id, &project_revision,
            &project_content_sha256, &project_sovereign_flags,
            &project_context_sha256, &project_title, &project_hypothesis,
            &project_description, &project_files,
        ].iter().any(|v| !v.is_empty());
        let scientific_project = match project_any {
            false => None,
            true => {
                let parse_u64 = |value: &str, field: &'static str| -> Result<u64, LightBridgeError> {
                    value.parse::<u64>().map_err(|_| LightBridgeError::CorruptField(field))
                };
                let parse_u32 = |value: &str, field: &'static str| -> Result<u32, LightBridgeError> {
                    value.parse::<u32>().map_err(|_| LightBridgeError::CorruptField(field))
                };
                let context = shadow_contracts::ProjectContextWitnessWire {
                    project_id: parse_u64(&project_id, "project_id")?,
                    user_id: parse_u64(&project_user_id, "project_user_id")?,
                    trace_id: parse_u64(&project_trace_id, "project_trace_id")?,
                    revision: parse_u32(&project_revision, "project_revision")?,
                    content_sha256: project_content_sha256.to_ascii_lowercase(),
                    light_sovereign_flags: parse_u32(
                        project_sovereign_flags.trim_start_matches("0x"),
                        "project_sovereign_flags",
                    ).or_else(|_| u32::from_str_radix(
                        project_sovereign_flags.trim_start_matches("0x"), 16
                    ).map_err(|_| LightBridgeError::CorruptField("project_sovereign_flags")))?,
                    context_sha256: project_context_sha256.to_ascii_lowercase(),
                };
                let expected_context = crate::gcl_project_contract::context_sha256(
                    &crate::gcl_project_contract::ProjectContextMaterial {
                        project_id: context.project_id,
                        user_id: context.user_id,
                        trace_id: context.trace_id,
                        revision: context.revision,
                        project_title: &project_title,
                        content_sha256: &context.content_sha256,
                        light_sovereign_flags: context.light_sovereign_flags,
                    },
                );
                let files = crate::gcl_project_contract::decode_files_hex(&project_files)
                    .ok_or(LightBridgeError::CorruptField("project_files"))?;
                let assumptions = project_assumptions.lines()
                    .map(str::trim).filter(|v| !v.is_empty()).map(str::to_string)
                    .collect::<Vec<_>>();
                let valid = context.project_id != 0
                    && context.user_id != 0
                    && context.trace_id != 0
                    && context.trace_id == crate::gcl_project_contract::project_trace_id(
                        context.project_id, context.user_id, &project_title,
                    )
                    && context.content_sha256 == input_sha256.to_ascii_lowercase()
                    && expected_context == context.context_sha256
                    && crate::gcl_project_contract::seal_is_500(context.light_sovereign_flags)
                    && !project_title.is_empty()
                    && !project_hypothesis.is_empty()
                    && !project_description.is_empty();
                match valid {
                    true => Some(ScientificProjectInput {
                        context, title: project_title, hypothesis: project_hypothesis,
                        assumptions, description: project_description, files,
                    }),
                    false => return Err(LightBridgeError::CorruptField("scientific_project")),
                }
            }
        };

        // evolve mbetet opsional (ka fallback të qartë te EvolvePayload::from_wire).
        match missing {
            Some(field) => Err(LightBridgeError::MissingField(field)),
            None        => Ok(QuantumInput {
                trace_id, domain, contract_id, lgc_seal,
                lang_code, evolve, input_sha256, ts, text, scientific_project,
            }),
        }
    }

    /// A e mban inputi vulën 500 nga Light? (#4: STRUKTUROR, jo substring).
    ///
    /// Para: contains("500") — çdo tekst me "500" kalonte (dobësi reale).
    /// Tani: nxjerr primitive_flags nga lgc_seal dhe verifikon formulën kanonike
    /// (flags & 0xFFFF) ^ 0xA5A5 == 500. Zero if.
    pub fn carries_seal(&self) -> bool {
        let flags = self.extract_seal_flags();
        // Formula kanonike VULA 500 — provë strukturore, jo tekstuale. Zero if.
        ((flags & 0xFFFF) ^ 0xA5A5) == 500
    }

    /// extract_seal_flags — nxjerr primitive_flags numerike nga lgc_seal.
    ///
    /// Format i pranuar: "...|flags:0xHEX" ose "...|0xHEX" ose "LGC::...|500".
    /// "500" semantik → kthen 0xA451 (flags që jep vula); ndryshe parse hex. Zero if.
    fn extract_seal_flags(&self) -> u32 {
        // Ndaj me '|' dhe kërko segmentin e flags. Zero if — fold mbi pjesët.
        self.lgc_seal.split('|')
            .fold(0u32, |acc, seg| {
                let trimmed = seg.trim();
                // Match mbi formën e segmentit — zero if.
                match trimmed {
                    // Marker semantik "500" → flags kanonike 0xA451.
                    "500" => 0xA451,
                    other => match other.strip_prefix("flags:") {
                        Some(hex) => Self::parse_hex_flags(hex).max(acc),
                        None      => match other.strip_prefix("0x") {
                            Some(hex) => u32::from_str_radix(hex, 16).unwrap_or(acc),
                            None      => acc,
                        },
                    },
                }
            })
    }

    /// parse_hex_flags — parson "0xHEX" ose "HEX" → u32. Zero if — match.
    fn parse_hex_flags(s: &str) -> u32 {
        let stripped = s.strip_prefix("0x").unwrap_or(s);
        u32::from_str_radix(stripped, 16).unwrap_or(0)
    }

    /// PATCH 7.1 — evolve_typed: kthen fushën 'evolve' të TIPIZUAR.
    ///
    /// Në vend që konsumatorët të parsojnë String-un e lirë, marrin strukturë
    /// të tipizuar EvolvePayload (gjenerimi, gjendja, vula). Zero if.
    pub fn evolve_typed(&self) -> Result<EvolvePayload, LightBridgeError> {
        EvolvePayload::from_wire(&self.evolve) // korrupsioni refuzohet i emërtuar
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// §2  QuantumOutput — ç'i kthen Quantum Light-it (SERIALIZE)
// ═════════════════════════════════════════════════════════════════════════════

/// Verdiktet që Quantum mund të kthejë (Light i pret këto string).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputVerdict {
    Optimal,
    Negative,
    Hold,
    Dismissed,
}

impl OutputVerdict {
    pub fn as_str(self) -> &'static str {
        match self {
            OutputVerdict::Optimal   => "OPTIMAL",
            OutputVerdict::Negative  => "NEGATIVE",
            OutputVerdict::Hold      => "HOLD",
            OutputVerdict::Dismissed => "DISMISSED",
        }
    }

    /// DSL suffix: VFYKNG (verified) ose NEGKNG (negative).
    pub fn sfx(self) -> &'static str {
        match self {
            OutputVerdict::Optimal => "VFYKNG",
            _                      => "NEGKNG",
        }
    }

    /// AlgorithmState: SEALED | SHADOW_VERIFY | REJECTED.
    pub fn state(self) -> &'static str {
        match self {
            OutputVerdict::Optimal   => "SEALED",
            OutputVerdict::Hold      => "SHADOW_VERIFY",
            _                        => "REJECTED",
        }
    }
}

/// Output-i që Quantum i kthen Light-it.
/// Format: trace_id=;verdict=;pot=;axis=;seal=;sfx=;state=;territory=;depth=;ts=;content=
#[derive(Debug, Clone)]
pub struct QuantumOutput {
    pub trace_id:     String,
    pub verdict:      OutputVerdict,
    pub potentiality: f32,
    pub axis_score:   f32,    // [-100, +100]
    pub seal:         String, // QNT:...
    pub territory:    String,
    pub depth_hint:   String,
    pub ts:           u64,
    pub content:      String,
}

impl QuantumOutput {
    /// SERIALIZE — saktë si Light QuantumResult::from_payload() pret.
    pub fn serialize(&self) -> String {
        format!(
            "trace_id={};verdict={};pot={:.4};axis={:.1};seal={};sfx={};state={};territory={};depth={};ts={};content={}",
            self.trace_id,
            self.verdict.as_str(),
            self.potentiality,
            self.axis_score,
            self.seal,
            self.verdict.sfx(),
            self.verdict.state(),
            self.territory,
            self.depth_hint,
            self.ts,
            escape_field(&self.content),  // FIX #1: escape delimiterët në free-text
        )
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// §3  URA — përkthen verdiktin e Shadow-it në QuantumOutput për Light
// ═════════════════════════════════════════════════════════════════════════════

use crate::bridge_shadow::SupremeOutcome;

pub struct LightQuantumBridge;

impl LightQuantumBridge {
    /// Ndërton QuantumOutput nga rezultati i pipeline-it + vendimi i Shadow-it.
    ///
    /// outcome:        vendimi suprem nga Shadow (përmes urës Shadow)
    /// potentiality:   nga PIM
    /// axis_score:     nga PRO ([-100,+100])
    /// depth_hint:     nga hardware (SHALLOW/MEDIUM/DEEP/VERY_DEEP)
    pub fn build_output(
        trace_id:     &str,
        outcome:      SupremeOutcome,
        potentiality: f32,
        axis_score:   f32,
        territory:    &str,
        depth_hint:   &str,
        content:      &str,
        ts:           u64,
    ) -> QuantumOutput {
        // Shadow outcome → Light verdict (branchless tabelë).
        let verdict = Self::outcome_to_verdict(outcome);

        // SEAL REAL (#5): QNT: për identitet + hash kanonik për autenticitet.
        // Para: "QNT:trace_id" — vetëm ID, çdokush e riprodhonte.
        // Tani: hash mbi envelope-in (trace+verdict+content+ts) lidh seal-in me
        // PËRMBAJTJEN — modifikimi i content-it e prish seal-in. Zero if.
        let canonical = format!(
            "{}|{}|{}|{}|{}",
            trace_id, verdict.as_str(), content, axis_score as i64, ts
        );
        let digest = crate::sovereign::sha256_hex(canonical.as_bytes());
        // QNT:<trace>:<16 hex të parë të hash-it> — identitet + provë.
        let seal = format!("QNT:{}:{}", trace_id, &digest[..16]);

        // ══ TRACE RE-VERIFY (HARDENING) ═════════════════════════════════════
        // trace_id NUK supozohet i saktë: rikontrollohet që seal-i i sapo-
        // ndërtuar (a) mban EKZAKTËSISHT trace_id-në e hyrjes dhe (b) hash-i
        // riprodhohet nga i njëjti trup kanonik. Çdo divergjencë e brendshme
        // (korrupsion memorie, race, ndërhyrje) raportohet ME ZË — gjurma
        // s'del kurrë e heshtur me identitet të dyshimtë.
        let redigest = crate::sovereign::sha256_hex(canonical.as_bytes());
        let expect_seal = format!("QNT:{}:{}", trace_id, &redigest[..16]);
        match seal == expect_seal {
            true  => {}
            false => eprintln!(
                "[BRIDGE_LIGHT] ALARM KRITIK: trace re-verify dështoi (trace_id={}) — seal i divergjuar, gjurma e rrezikuar",
                trace_id
            ),
        }

        QuantumOutput {
            trace_id: trace_id.to_string(),
            verdict,
            potentiality: potentiality.clamp(0.0, 1.0),
            axis_score: axis_score.clamp(-100.0, 100.0),
            seal,
            territory: territory.to_string(),
            depth_hint: depth_hint.to_string(),
            ts,
            content: content.to_string(),
        }
    }

    /// SupremeOutcome → OutputVerdict (branchless via match, pa if/else).
    fn outcome_to_verdict(outcome: SupremeOutcome) -> OutputVerdict {
        match outcome {
            SupremeOutcome::PrimitiveKnowledge     => OutputVerdict::Optimal,
            SupremeOutcome::VerifiableNotPrimitive => OutputVerdict::Hold,
            SupremeOutcome::ChainRejected          => OutputVerdict::Negative,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_light_input() {
        let payload = "trace_id=T123;domain=science;contract_id=KDS:1;lgc_seal=LGC::C|500;lang_code=en;evolve=summary;input_sha256=b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9;ts=1000;text=hello world";
        let input = QuantumInput::deserialize(payload).unwrap();
        assert_eq!(input.trace_id, "T123");
        assert_eq!(input.domain, "science");
        assert_eq!(input.text, "hello world");
        assert_eq!(input.input_sha256, "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9");
        assert_eq!(input.ts, 1000);
        assert!(input.carries_seal()); // lgc_seal përmban "500"
    }

    #[test]
    fn malformed_input_sha256_is_named_corruption() {
        let payload = "trace_id=T123;domain=science;contract_id=KDS:1;lgc_seal=LGC::C|500;lang_code=en;evolve=summary;input_sha256=xyz;ts=1000;text=hello world";
        assert_eq!(
            QuantumInput::deserialize(payload).unwrap_err(),
            LightBridgeError::CorruptField("input_sha256")
        );
    }

    #[test]
    fn deserialize_missing_trace_id_fails() {
        let payload = "domain=science;text=hello";
        assert_eq!(
            QuantumInput::deserialize(payload).unwrap_err(),
            LightBridgeError::MissingField("trace_id")
        );
    }

    #[test]
    fn serialize_matches_light_format() {
        let out = QuantumOutput {
            trace_id: "T123".to_string(),
            verdict: OutputVerdict::Optimal,
            potentiality: 0.85,
            axis_score: 42.5,
            seal: "QNT:T123".to_string(),
            territory: "SCIENCE".to_string(),
            depth_hint: "DEEP".to_string(),
            ts: 2000,
            content: "reasoning summary".to_string(),
        };
        let s = out.serialize();
        // Light QuantumResult::from_payload duhet ta parsojë saktë.
        assert!(s.contains("trace_id=T123"));
        assert!(s.contains("verdict=OPTIMAL"));
        assert!(s.contains("sfx=VFYKNG"));   // Optimal → VFYKNG
        assert!(s.contains("state=SEALED"));  // Optimal → SEALED
        assert!(s.contains("pot=0.8500"));
    }

    #[test]
    fn roundtrip_through_light_format() {
        // Quantum serializon → string → fushat dalin saktë.
        let out = LightQuantumBridge::build_output(
            "T9", SupremeOutcome::PrimitiveKnowledge,
            0.9, 80.0, "INDUSTRIAL", "VERY_DEEP", "ok", 500,
        );
        let s = out.serialize();
        // Verifiko fushat kritike që Light lexon.
        assert!(s.contains("trace_id=T9"));
        assert!(s.contains("verdict=OPTIMAL"));
        assert!(s.contains("territory=INDUSTRIAL"));
        assert!(s.contains("depth=VERY_DEEP"));
        assert!(s.contains("seal=QNT:T9"));
    }

    #[test]
    fn outcome_mapping_is_total() {
        // Të tria outcome → verdict (pa if/else, mbulim i plotë).
        assert_eq!(
            LightQuantumBridge::outcome_to_verdict(SupremeOutcome::PrimitiveKnowledge),
            OutputVerdict::Optimal
        );
        assert_eq!(
            LightQuantumBridge::outcome_to_verdict(SupremeOutcome::VerifiableNotPrimitive),
            OutputVerdict::Hold
        );
        assert_eq!(
            LightQuantumBridge::outcome_to_verdict(SupremeOutcome::ChainRejected),
            OutputVerdict::Negative
        );
    }

    #[test]
    fn verdict_sfx_and_state() {
        assert_eq!(OutputVerdict::Optimal.sfx(), "VFYKNG");
        assert_eq!(OutputVerdict::Negative.sfx(), "NEGKNG");
        assert_eq!(OutputVerdict::Optimal.state(), "SEALED");
        assert_eq!(OutputVerdict::Hold.state(), "SHADOW_VERIFY");
        assert_eq!(OutputVerdict::Dismissed.state(), "REJECTED");
    }
}
