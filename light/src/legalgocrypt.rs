// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LEGALGOCRYPT.RS — ALGORITMI I PLOTË (SOFTWARE + HARDWARE)             ║
// ║  GJATA LEGACY™                                                          ║
// ║                                                                          ║
// ║  FORMAT FINAL (9 segmente):                                             ║
// ║  LGC::ROOT|LIGHT|SHADOW|TRACE|LOCATION|DSL|TIME|CHAIN|CONTRACT        ║
// ║                                                                          ║
// ║  ── ALGORITMI HARDWARE ─────────────────────────────────────────────── ║
// ║  Merr VETËM numrin e kontratës → permutim                              ║
// ║  "Gjal873921J" → last_char + reverse(digits) → "J129378"              ║
// ║                                                                          ║
// ║  ── ALGORITMI SOFTWARE ─────────────────────────────────────────────── ║
// ║  Shadow module (3): reverse i plotë  kng → gnk                        ║
// ║  Shadow file   (4): swap outer       phsc → chsp                       ║
// ║  Location: country + reverse(city)   tir → rit                        ║
// ║  Immutable code: 4142991                                                ║
// ╚══════════════════════════════════════════════════════════════════════════╝

const SEP:            char = '|';
const TIME_SEP:       char = '~';
const PREFIX:         &str = "LGC::";
const IMMUTABLE_CODE: &str = "4142991";
const GJL_PREFIX:     &str = "GjL_";
const VOWELS:         &str = "aeiouAEIOU";

use crate::lgc_algorithm::{
    compute_domain_sha, is_canonical_sha256, verify_kodunik_attestation,
};

const V2_PREFIX: &str = "LGC2::";

// ═════════════════════════════════════════════════════════════════════════════
// ██  HARDWARE ALGORITHM                                                   ██
// ═════════════════════════════════════════════════════════════════════════════
//  Input:  "Gjal873921J"
//  Hapi 1: last char        → 'J'
//  Hapi 2: digits from body → "129378"
//  Hapi 3: reverse digits   → "873921"
//  Output: "J129378"
// ─────────────────────────────────────────────────────────────────────────────

pub mod hardware {
    pub fn encode_contract_serial(contract_number: &str) -> String {
        let chars: Vec<char> = contract_number.chars().collect();
        match chars.is_empty() { true => return contract_number.to_string(), false => {} }
        // FAIL-SAFE (      ): chars.last() me match (jo unwrap) — i mbrojtur nga is_empty.
        let last_char = match chars.last() {
            Some(c) => *c,
            None    => return contract_number.to_string(),
        };
        let digits: String = chars[..chars.len()-1].iter()
            .filter(|c| c.is_ascii_digit()).collect();
        let reversed: String = digits.chars().rev().collect();
        format!("{}{}", last_char, reversed)
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// ██  SOFTWARE ALGORITHM — HELPERS                                         ██
// ═════════════════════════════════════════════════════════════════════════════

fn shadow_module(m: &str) -> String { m.chars().rev().collect() }

fn shadow_file(f: &str) -> String {
    let c: Vec<char> = f.chars().collect();
    match c.len() != 4 { true => return c.iter().rev().collect(), false => {} }
    vec![c[3], c[1], c[2], c[0]].iter().collect()
}

fn reverse_city(city: &str) -> String { city.chars().rev().collect() }

fn consonants(s: &str, n: usize) -> String {
    s.chars().filter(|c| c.is_alphabetic() && !VOWELS.contains(*c))
        .map(|c| c.to_ascii_lowercase()).take(n).collect()
}

// ─────────────────────────────────────────────────────────────────────────────
// STRUCTS
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct LegalGoCrypt {
    pub root:     String,
    pub light:    LightCall,
    pub shadow:   ShadowCall,
    pub trace:    Trace,
    pub location: Location,
    pub dsl:      String,
    pub time:     Time,
    pub chain:    String,
    pub contract: ContractBinding,
}

#[derive(Debug, Clone)]
pub struct LightCall { pub platform: String, pub module: String, pub file: String }
impl LightCall {
    pub fn encode(&self) -> String {
        format!("{}/mdl-{}.{}", self.platform, self.module, self.file)
    }
}

#[derive(Debug, Clone)]
pub struct ShadowCall { pub platform: String, pub module: String, pub file: String }
impl ShadowCall {
    pub fn from_light(l: &LightCall) -> Self {
        Self { platform: "shd".into(), module: shadow_module(&l.module), file: shadow_file(&l.file) }
    }
    pub fn encode(&self) -> String {
        format!("{}/mdl-{}.{}", self.platform, self.module, self.file)
    }
}

#[derive(Debug, Clone)]
pub struct Trace { pub base: u64, pub generated: u64, pub transform: Option<u32> }
impl Trace {
    pub fn encode(&self) -> String {
        let b = format!("{:07}", self.base);
        let g = format!("{:07}", self.generated);
        match self.transform { Some(t) => format!("{}.{},{}", b, g, t), None => format!("{}.{}", b, g) }
    }
}

#[derive(Debug, Clone)]
pub struct Location { pub country: String, pub city: String }
impl Location { pub fn encode(&self) -> String { format!("{}-{}", self.country, self.city) } }

#[derive(Debug, Clone)]
pub struct Time { pub origin: String, pub generated: String }
impl Time { pub fn encode(&self) -> String { format!("{}~{}", self.origin, self.generated) } }

/// Segmenti 9 — CONTRACT BINDING
/// client_code     = kodi GjL_... nga legalgocrypt_hardware
/// contract_serial = numri i kontratës i permutuar nga HARDWARE ALGORITHM
///                   "Gjal873921J" → "J129378"
#[derive(Debug, Clone)]
pub struct ContractBinding {
    pub client_code:     String,
    pub contract_serial: String,
}

impl ContractBinding {
    /// contract_number_raw → hardware::encode_contract_serial() automatikisht
    pub fn new(client_code: &str, contract_number_raw: &str) -> Self {
        Self {
            client_code:     client_code.to_string(),
            contract_serial: hardware::encode_contract_serial(contract_number_raw),
        }
    }
    pub fn encode(&self) -> String {
        format!("contract-{}/hw-{}", self.client_code, self.contract_serial)
    }
}

#[derive(Debug, Clone)]
pub struct LegacyV2Fields {
    pub version: u8,
    pub trace_id: String,
    pub trace_origin: u64,
    pub generation: u64,
    pub input_sha256: String,
    pub primitive_anchor: String,
    pub primitive_flags: u32,
    pub xi_len: usize,
    pub yi_len: usize,
    pub xi_sha256: String,
    pub yi_sha256: String,
    pub split_sha256: String,
    pub law_before: u64,
    pub law_after: u64,
    pub law_seal: u64,
    pub system_laws_seal: u64,
    pub legacy_v1_sha256: String,
    pub binding_sha256: String,
    pub kodunik: String,
    pub hmac_sig: String,
    pub counter: u64,
    pub timestamp: u64,
    pub contract_lineage: String,
    pub bridge_lineage: String,
}

#[derive(Debug, Clone)]
pub struct LegacyEnvelopeV2 {
    pub legacy: LegalGoCrypt,
    pub fields: LegacyV2Fields,
    pub integrity_sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum V2Error {
    MissingPrefix,
    WrongFieldCount(usize),
    InvalidLegacy,
    InvalidNumber(&'static str),
    InvalidIntegrity,
    InvalidBinding,
    InvalidKodunik,
    InvalidInvariant(&'static str),
}

impl std::fmt::Display for V2Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingPrefix => write!(f, "LGC_V2_MISSING_PREFIX"),
            Self::WrongFieldCount(n) => write!(f, "LGC_V2_WRONG_FIELD_COUNT:{n}"),
            Self::InvalidLegacy => write!(f, "LGC_V2_INVALID_LEGACY_V1"),
            Self::InvalidNumber(name) => write!(f, "LGC_V2_INVALID_NUMBER:{name}"),
            Self::InvalidIntegrity => write!(f, "LGC_V2_INVALID_INTEGRITY"),
            Self::InvalidBinding => write!(f, "LGC_V2_INVALID_BINDING"),
            Self::InvalidKodunik => write!(f, "LGC_V2_INVALID_KODUNIK"),
            Self::InvalidInvariant(name) => write!(f, "LGC_V2_INVALID_INVARIANT:{name}"),
        }
    }
}

pub fn legacy_v2_binding_sha(fields: &LegacyV2Fields) -> String {
    let trace_origin = fields.trace_origin.to_le_bytes();
    let generation = fields.generation.to_le_bytes();
    let primitive_flags = fields.primitive_flags.to_le_bytes();
    let xi_len = (fields.xi_len as u64).to_le_bytes();
    let yi_len = (fields.yi_len as u64).to_le_bytes();
    let law_before = fields.law_before.to_le_bytes();
    let law_after = fields.law_after.to_le_bytes();
    let law_seal = fields.law_seal.to_le_bytes();
    let system_laws_seal = fields.system_laws_seal.to_le_bytes();
    compute_domain_sha(
        "ESSMAI/LEGALGOCRYPT/V2/BINDING",
        &[
            fields.trace_id.as_bytes(),
            &trace_origin,
            &generation,
            fields.input_sha256.as_bytes(),
            fields.primitive_anchor.as_bytes(),
            &primitive_flags,
            &xi_len,
            &yi_len,
            fields.xi_sha256.as_bytes(),
            fields.yi_sha256.as_bytes(),
            fields.split_sha256.as_bytes(),
            &law_before,
            &law_after,
            &law_seal,
            &system_laws_seal,
            fields.legacy_v1_sha256.as_bytes(),
            fields.contract_lineage.as_bytes(),
            fields.bridge_lineage.as_bytes(),
        ],
    )
}

// ─────────────────────────────────────────────────────────────────────────────
// GENERATOR
// ─────────────────────────────────────────────────────────────────────────────

pub struct LgcGenerator;
impl LgcGenerator {
    pub fn generate(
        identity: &str, platform_light: &str, module_name: &str, file_name: &str,
        trace_base: u64, trace_gen: u64, trace_transform: Option<u32>,
        country: &str, city_normal: &str, dsl_chain: &[&str],
        time_origin: &str, time_generated: &str, platform_chain: &[&str],
        client_code: &str,
        contract_number_raw: &str,   // → hardware permuton automatikisht
    ) -> LegalGoCrypt {
        let light = LightCall {
            platform: platform_light.to_string(),
            module:   consonants(module_name, 3),
            file:     consonants(file_name, 4),
        };
        let shadow = ShadowCall::from_light(&light);
        LegalGoCrypt {
            root: format!("1-{}_", identity),
            light, shadow,
            trace: Trace { base: trace_base, generated: trace_gen, transform: trace_transform },
            location: Location { country: country.to_string(), city: reverse_city(city_normal) },
            dsl: format!("{{{}}}", dsl_chain.join("<")),
            time: Time { origin: time_origin.to_string(), generated: time_generated.to_string() },
            chain: format!("{}{}", platform_chain.join("/"), IMMUTABLE_CODE),
            contract: ContractBinding::new(client_code, contract_number_raw),
        }
    }

    pub fn encode(lgc: &LegalGoCrypt) -> String {
        format!(
            "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            PREFIX,
            lgc.root,              SEP,
            lgc.light.encode(),    SEP,
            lgc.shadow.encode(),   SEP,
            lgc.trace.encode(),    SEP,
            lgc.location.encode(), SEP,
            lgc.dsl,               SEP,
            lgc.time.encode(),     SEP,
            lgc.chain,             SEP,
            lgc.contract.encode(),
        )
    }
}

impl LgcGenerator {
    pub fn legacy_v1_sha256(legacy: &LegalGoCrypt) -> String {
        let wire = Self::encode(legacy);
        compute_domain_sha("ESSMAI/LEGALGOCRYPT/V2/LEGACY-V1", &[wire.as_bytes()])
    }

    pub fn generate_v2(
        legacy: LegalGoCrypt,
        fields: LegacyV2Fields,
    ) -> Result<LegacyEnvelopeV2, V2Error> {
        if fields.version != 2 {
            return Err(V2Error::InvalidInvariant("version"));
        }
        if fields.legacy_v1_sha256 != Self::legacy_v1_sha256(&legacy) {
            return Err(V2Error::InvalidLegacy);
        }
        if fields.binding_sha256 != legacy_v2_binding_sha(&fields) {
            return Err(V2Error::InvalidBinding);
        }
        let mut envelope = LegacyEnvelopeV2 {
            legacy,
            fields,
            integrity_sha256: String::new(),
        };
        let unsigned = Self::encode_v2_unsigned(&envelope);
        envelope.integrity_sha256 = compute_domain_sha(
            "ESSMAI/LEGALGOCRYPT/V2/INTEGRITY",
            &[unsigned.as_bytes()],
        );
        Ok(envelope)
    }

    fn encode_v2_unsigned(envelope: &LegacyEnvelopeV2) -> String {
        let legacy = Self::encode(&envelope.legacy);
        let legacy_body = legacy.strip_prefix(PREFIX).unwrap_or(legacy.as_str());
        let f = &envelope.fields;
        format!(
            "{V2_PREFIX}{legacy_body}|{}|{}|{:016x}|{}|{}|{}|{:08x}|{}|{}|{}|{}|{}|{}|{}|{:016x}|{:016x}|{}|{}|{}|{}|{}|{}|{}|{}",
            f.version,
            f.trace_id,
            f.trace_origin,
            f.generation,
            f.input_sha256,
            f.primitive_anchor,
            f.primitive_flags,
            f.xi_len,
            f.yi_len,
            f.xi_sha256,
            f.yi_sha256,
            f.split_sha256,
            f.law_before,
            f.law_after,
            f.law_seal,
            f.system_laws_seal,
            f.legacy_v1_sha256,
            f.binding_sha256,
            f.kodunik,
            f.hmac_sig,
            f.counter,
            f.timestamp,
            f.contract_lineage,
            f.bridge_lineage,
        )
    }

    pub fn encode_v2(envelope: &LegacyEnvelopeV2) -> String {
        format!(
            "{}|{}",
            Self::encode_v2_unsigned(envelope),
            envelope.integrity_sha256,
        )
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// PARSER
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum ParseError {
    MissingPrefix, WrongFieldCount(usize),
    InvalidLight(String), InvalidShadow(String), InvalidTrace(String),
    InvalidLocation(String), InvalidTime(String), InvalidContract(String),
}

pub struct LgcParser;
impl LgcParser {
    pub fn parse(s: &str) -> Result<LegalGoCrypt, ParseError> {
        let body = s.strip_prefix(PREFIX).ok_or(ParseError::MissingPrefix)?;
        let parts: Vec<&str> = body.split(SEP).collect();
        match parts.len() != 9 { true => return Err(ParseError::WrongFieldCount(parts.len())), false => {} }
        let light  = Self::parse_light(parts[1])?;
        let shadow = Self::parse_shadow(parts[2])?;
        Ok(LegalGoCrypt {
            root: parts[0].to_string(), light, shadow,
            trace:    Self::parse_trace(parts[3])?,
            location: Self::parse_location(parts[4])?,
            dsl:      parts[5].to_string(),
            time:     Self::parse_time(parts[6])?,
            chain:    parts[7].to_string(),
            contract: Self::parse_contract(parts[8])?,
        })
    }

    fn parse_light(s: &str) -> Result<LightCall, ParseError> {
        let e = || ParseError::InvalidLight(s.to_string());
        let (p, r) = s.split_once('/').ok_or_else(e)?;
        let r = r.strip_prefix("mdl-").ok_or_else(e)?;
        let (m, f) = r.split_once('.').ok_or_else(e)?;
        Ok(LightCall { platform: p.into(), module: m.into(), file: f.into() })
    }

    fn parse_shadow(s: &str) -> Result<ShadowCall, ParseError> {
        let e = || ParseError::InvalidShadow(s.to_string());
        let (p, r) = s.split_once('/').ok_or_else(e)?;
        let r = r.strip_prefix("mdl-").ok_or_else(e)?;
        let (m, f) = r.split_once('.').ok_or_else(e)?;
        Ok(ShadowCall { platform: p.into(), module: m.into(), file: f.into() })
    }

    fn parse_trace(s: &str) -> Result<Trace, ParseError> {
        let e = || ParseError::InvalidTrace(s.to_string());
        let (main, t) = match s.split_once(',') {
            Some((m, t)) => (m, Some(t.parse::<u32>().map_err(|_| e())?)),
            None => (s, None),
        };
        let (b, g) = main.split_once('.').ok_or_else(e)?;
        Ok(Trace { base: b.parse().map_err(|_| e())?, generated: g.parse().map_err(|_| e())?, transform: t })
    }

    fn parse_location(s: &str) -> Result<Location, ParseError> {
        let e = || ParseError::InvalidLocation(s.to_string());
        let (c, city) = s.split_once('-').ok_or_else(e)?;
        Ok(Location { country: c.into(), city: city.into() })
    }

    fn parse_time(s: &str) -> Result<Time, ParseError> {
        let e = || ParseError::InvalidTime(s.to_string());
        let (o, g) = s.split_once(TIME_SEP).ok_or_else(e)?;
        Ok(Time { origin: o.into(), generated: g.into() })
    }

    fn parse_contract(s: &str) -> Result<ContractBinding, ParseError> {
        let e = || ParseError::InvalidContract(s.to_string());
        let s = s.strip_prefix("contract-").ok_or_else(e)?;
        // rfind "/hw-" sepse client_code ka "/" brenda
        let idx = s.rfind("/hw-").ok_or_else(e)?;
        Ok(ContractBinding {
            client_code:     s[..idx].to_string(),
            contract_serial: s[idx+4..].to_string(),
        })
    }
}

impl LgcParser {
    pub fn parse_v2(s: &str) -> Result<LegacyEnvelopeV2, V2Error> {
        let body = s.strip_prefix(V2_PREFIX).ok_or(V2Error::MissingPrefix)?;
        let parts: Vec<&str> = body.split(SEP).collect();
        if parts.len() != 34 {
            return Err(V2Error::WrongFieldCount(parts.len()));
        }
        let legacy_wire = format!("{PREFIX}{}", parts[..9].join("|"));
        let legacy = Self::parse(&legacy_wire).map_err(|_| V2Error::InvalidLegacy)?;
        let dec_u64 = |index: usize, name: &'static str| {
            parts[index]
                .parse::<u64>()
                .map_err(|_| V2Error::InvalidNumber(name))
        };
        let hex_u64 = |index: usize, name: &'static str| {
            u64::from_str_radix(parts[index], 16)
                .map_err(|_| V2Error::InvalidNumber(name))
        };
        let usize_value = |index: usize, name: &'static str| {
            parts[index]
                .parse::<usize>()
                .map_err(|_| V2Error::InvalidNumber(name))
        };
        let version = parts[9]
            .parse::<u8>()
            .map_err(|_| V2Error::InvalidNumber("version"))?;
        let primitive_flags = u32::from_str_radix(parts[15], 16)
            .map_err(|_| V2Error::InvalidNumber("primitive_flags"))?;
        Ok(LegacyEnvelopeV2 {
            legacy,
            fields: LegacyV2Fields {
                version,
                trace_id: parts[10].to_string(),
                trace_origin: hex_u64(11, "trace_origin")?,
                generation: dec_u64(12, "generation")?,
                input_sha256: parts[13].to_string(),
                primitive_anchor: parts[14].to_string(),
                primitive_flags,
                xi_len: usize_value(16, "xi_len")?,
                yi_len: usize_value(17, "yi_len")?,
                xi_sha256: parts[18].to_string(),
                yi_sha256: parts[19].to_string(),
                split_sha256: parts[20].to_string(),
                law_before: dec_u64(21, "law_before")?,
                law_after: dec_u64(22, "law_after")?,
                law_seal: hex_u64(23, "law_seal")?,
                system_laws_seal: hex_u64(24, "system_laws_seal")?,
                legacy_v1_sha256: parts[25].to_string(),
                binding_sha256: parts[26].to_string(),
                kodunik: parts[27].to_string(),
                hmac_sig: parts[28].to_string(),
                counter: dec_u64(29, "counter")?,
                timestamp: dec_u64(30, "timestamp")?,
                contract_lineage: parts[31].to_string(),
                bridge_lineage: parts[32].to_string(),
            },
            integrity_sha256: parts[33].to_string(),
        })
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// VALIDATOR
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct ValidationResult {
    pub valid: bool, pub shadow_ok: bool, pub trace_ok: bool,
    pub immutable_ok: bool, pub dsl_ok: bool, pub contract_ok: bool,
    pub errors: Vec<String>,
}

pub struct LgcValidator;
impl LgcValidator {
    pub fn validate(s: &str) -> ValidationResult {
        let parsed = match LgcParser::parse(s) {
            Ok(p) => p,
            Err(e) => return ValidationResult {
                valid: false, shadow_ok: false, trace_ok: false,
                immutable_ok: false, dsl_ok: false, contract_ok: false,
                errors: vec![format!("PARSE ERROR: {:?}", e)],
            },
        };
        let mut errors = Vec::new();

        let exp = ShadowCall::from_light(&parsed.light);
        let shadow_ok = parsed.shadow.module == exp.module && parsed.shadow.file == exp.file;
        match shadow_ok { false => errors.push(format!("Shadow gabim: exp module='{}' file='{}'", exp.module, exp.file)), true => {} }

        let trace_ok = parsed.trace.generated >= parsed.trace.base;
        match trace_ok { false => errors.push("Trace: generated < base".into()), true => {} }

        let immutable_ok = parsed.chain.ends_with(IMMUTABLE_CODE);
        match immutable_ok { false => errors.push(format!("Immutable '{}' mungon", IMMUTABLE_CODE)), true => {} }

        let dsl_ok = parsed.dsl.starts_with('{') && parsed.dsl.ends_with('}');
        match dsl_ok { false => errors.push("DSL format gabim".into()), true => {} }

        let contract_ok = !parsed.contract.client_code.is_empty()
            && !parsed.contract.contract_serial.is_empty()
            && parsed.contract.client_code.starts_with(GJL_PREFIX);
        match contract_ok { false => errors.push("Contract invalid".into()), true => {} }

        let valid = shadow_ok && trace_ok && immutable_ok && dsl_ok && contract_ok && errors.is_empty();
        ValidationResult { valid, shadow_ok, trace_ok, immutable_ok, dsl_ok, contract_ok, errors }
    }
}

impl LgcValidator {
    pub fn validate_v2(s: &str) -> Result<LegacyEnvelopeV2, V2Error> {
        let envelope = LgcParser::parse_v2(s)?;
        let legacy_wire = LgcGenerator::encode(&envelope.legacy);
        if !Self::validate(&legacy_wire).valid {
            return Err(V2Error::InvalidLegacy);
        }
        let f = &envelope.fields;
        if f.version != 2 || f.generation == 0 {
            return Err(V2Error::InvalidInvariant("version_or_generation"));
        }
        if f.primitive_flags != 0xA451 {
            return Err(V2Error::InvalidInvariant("seal_500"));
        }
        if f.law_seal == 0 || f.system_laws_seal == 0 {
            return Err(V2Error::InvalidInvariant("law_seals"));
        }
        for sha in [
            &f.input_sha256,
            &f.xi_sha256,
            &f.yi_sha256,
            &f.split_sha256,
            &f.legacy_v1_sha256,
            &f.binding_sha256,
            &envelope.integrity_sha256,
        ] {
            if !is_canonical_sha256(sha) {
                return Err(V2Error::InvalidInvariant("canonical_sha256"));
            }
        }
        let pa_ok = f.primitive_anchor.len() == 16
            && f.primitive_anchor
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b));
        if !pa_ok || f.trace_id.is_empty() {
            return Err(V2Error::InvalidInvariant("trace_or_pa"));
        }
        let term_count = f
            .xi_len
            .checked_add(f.yi_len)
            .ok_or(V2Error::InvalidInvariant("term_count_overflow"))?;
        let expected_before = (term_count as u64)
            .checked_mul(term_count as u64)
            .ok_or(V2Error::InvalidInvariant("law_before_overflow"))?;
        let expected_after = (f.xi_len as u64)
            .checked_mul(f.yi_len as u64)
            .ok_or(V2Error::InvalidInvariant("law_after_overflow"))?;
        if f.law_before != expected_before
            || f.law_after != expected_after
            || f.law_after > f.law_before
        {
            return Err(V2Error::InvalidInvariant("law0"));
        }
        if f.contract_lineage != "light_coordinator>quantum_platform:route_input"
            || f.bridge_lineage != "LIGHT>QUANTUM>SHADOW"
        {
            return Err(V2Error::InvalidInvariant("lineage"));
        }
        if f.legacy_v1_sha256 != LgcGenerator::legacy_v1_sha256(&envelope.legacy) {
            return Err(V2Error::InvalidLegacy);
        }
        if f.binding_sha256 != legacy_v2_binding_sha(f) {
            return Err(V2Error::InvalidBinding);
        }
        if !verify_kodunik_attestation(
            &f.kodunik,
            &f.binding_sha256,
            &f.hmac_sig,
            f.counter,
            f.timestamp,
        ) {
            return Err(V2Error::InvalidKodunik);
        }
        let expected_integrity = compute_domain_sha(
            "ESSMAI/LEGALGOCRYPT/V2/INTEGRITY",
            &[LgcGenerator::encode_v2_unsigned(&envelope).as_bytes()],
        );
        if envelope.integrity_sha256 != expected_integrity {
            return Err(V2Error::InvalidIntegrity);
        }
        Ok(envelope)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// TESTS
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    const CLIENT_CODE:     &str = "GjL_26226/trAl-MMSP_J129378/WJ/Q22K_M";
    const CONTRACT_RAW:    &str = "Gjal873921J";
    const CONTRACT_SERIAL: &str = "J129378";

    const CANONICAL: &str = concat!(
        "LGC::1-gjl_",
        "|lhg/mdl-kng.phsc",
        "|shd/mdl-gnk.chsp",
        "|0000000.0000001,1",
        "|alb-rit",
        "|{A<gjb<gjl<shdgj}",
        "|0:0+~000:002",
        "|essmai/lgh/qnt/shd4142991",
        "|contract-GjL_26226/trAl-MMSP_J129378/WJ/Q22K_M/hw-J129378",
    );

    #[test]
    fn test_hardware_serial() {
        assert_eq!(hardware::encode_contract_serial(CONTRACT_RAW), CONTRACT_SERIAL);
    }

    #[test]
    fn test_shadow() {
        assert_eq!(shadow_module("kng"), "gnk");
        assert_eq!(shadow_file("phsc"), "chsp");
    }

    #[test]
    fn test_contract_auto_permutes() {
        let cb = ContractBinding::new(CLIENT_CODE, CONTRACT_RAW);
        assert_eq!(cb.contract_serial, CONTRACT_SERIAL);
        assert_eq!(cb.encode(), format!("contract-{}/hw-{}", CLIENT_CODE, CONTRACT_SERIAL));
    }

    #[test]
    fn test_generate_encode() {
        let lgc = LgcGenerator::generate(
            "gjl", "lhg", "knga", "phsca", 0, 1, Some(1), "alb", "tir",
            &["A", "gjb", "gjl", "shdgj"], "0:0+", "000:002",
            &["essmai", "lgh", "qnt", "shd"], CLIENT_CODE, CONTRACT_RAW,
        );
        assert_eq!(LgcGenerator::encode(&lgc), CANONICAL);
    }

    #[test]
    fn test_parse() {
        let lgc = LgcParser::parse(CANONICAL).unwrap();
        assert_eq!(lgc.contract.client_code,     CLIENT_CODE);
        assert_eq!(lgc.contract.contract_serial, CONTRACT_SERIAL);
    }

    #[test]
    fn test_validate() {
        let r = LgcValidator::validate(CANONICAL);
        assert!(r.valid, "{:?}", r.errors);
    }

    #[test]
    fn test_tamper_fails() {
        assert!(!LgcValidator::validate(&CANONICAL.replace("gnk","xyz")).valid);
        assert!(!LgcValidator::validate(&CANONICAL.replace("4142991","0000000")).valid);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// SEAL BRIDGE — lidhja me lgc_bridge.rs
// Nga shadow_gj_legacy_seal.c: (flags & 0x0000FFFF) ^ 0xA5A5
// lgc_bridge e thërret: crate::legalgocrypt::unseal_from_flags(flags)
// ─────────────────────────────────────────────────────────────────────────────
pub fn unseal_from_flags(flags: u32) -> u32 {
    const XK:  u32 = 0xA5A5;
    const MSK: u32 = 0x0000_FFFF;
    (flags & MSK) ^ XK
}
