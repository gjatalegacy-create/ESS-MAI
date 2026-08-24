//! ESS-MAI v1.7.1 — canonical Negative Knowledge asset envelope.
//!
//! This module defines transport-only evidence. It has no sovereign authority,
//! does not produce a verdict, and does not persist anything. Quantum supplies
//! the measured material and a SHA-256 token; Shadow recomputes the same token,
//! binds it to GCL/TRL, and only then applies its existing verdict-gated policy.

pub const NEGATIVE_ASSET_MAGIC: [u8; 8] = *b"ESSNK171";
pub const NEGATIVE_ASSET_VERSION: u32 = 0x0001_0701;
pub const NEGATIVE_ASSET_TOKEN_DOMAIN: &[u8] = b"ESS_MAI_GCL_NEGATIVE_ASSET_TOKEN_V171";
pub const NEGATIVE_ASSET_MAX_ITEMS: usize = 16_384;
pub const NEGATIVE_ASSET_MAX_TEXT: usize = 1_048_576;

pub const NEGATIVE_MODULE_LIM: u8 = 1 << 0;
pub const NEGATIVE_MODULE_NPRO: u8 = 1 << 1;
pub const NEGATIVE_MODULE_NPIM: u8 = 1 << 2;
pub const NEGATIVE_MODULE_APRO: u8 = 1 << 3;
pub const NEGATIVE_MODULE_MPRO: u8 = 1 << 4;
pub const NEGATIVE_MODULE_HCP: u8 = 1 << 5;
pub const NEGATIVE_MODULE_REQUIRED: u8 = NEGATIVE_MODULE_LIM
    | NEGATIVE_MODULE_NPRO
    | NEGATIVE_MODULE_NPIM
    | NEGATIVE_MODULE_APRO
    | NEGATIVE_MODULE_MPRO
    | NEGATIVE_MODULE_HCP;

pub const NEGATIVE_SPACE_XI: u8 = 1;
pub const NEGATIVE_SPACE_YI: u8 = 2;

pub const NEGATIVE_OPERATOR_REVERSE: u8 = 1;
pub const NEGATIVE_OPERATOR_MIRROR: u8 = 2;
pub const NEGATIVE_OPERATOR_CONTRAST: u8 = 3;
pub const NEGATIVE_OPERATOR_TRACE: u8 = 4;

pub const NEGATIVE_FLOW_LIM: u8 = 1;
pub const NEGATIVE_FLOW_NPRO: u8 = 2;
pub const NEGATIVE_FLOW_NPIM: u8 = 3;
pub const NEGATIVE_FLOW_APRO: u8 = 4;
pub const NEGATIVE_FLOW_MPRO: u8 = 5;
pub const NEGATIVE_FLOW_HCP_PRO: u8 = 6;
pub const NEGATIVE_FLOW_REQUIRED: [u8; 6] = [
    NEGATIVE_FLOW_LIM,
    NEGATIVE_FLOW_NPRO,
    NEGATIVE_FLOW_NPIM,
    NEGATIVE_FLOW_APRO,
    NEGATIVE_FLOW_MPRO,
    NEGATIVE_FLOW_HCP_PRO,
];

pub const NEGATIVE_ASSET_CLASS_KNOWLEDGE: u8 = 1;
pub const NEGATIVE_ASSET_CLASS_EXPERIMENTAL_FAILURE: u8 = 2;

/// Canonical research context carried only by a failed scientific experiment.
/// It is transport evidence, not a verdict and not a semantic decision.
pub fn canonical_scientific_failure_context(
    project_title: &str,
    domain: &str,
    hypothesis: &str,
    assumptions: &[String],
    lab_test_id: &str,
    findings: &[String],
    description: &str,
) -> String {
    std::iter::once(project_title)
        .chain(std::iter::once(domain))
        .chain(std::iter::once(hypothesis))
        .chain(assumptions.iter().map(String::as_str))
        .chain(std::iter::once(lab_test_id))
        .chain(findings.iter().map(String::as_str))
        .chain(std::iter::once(description))
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NegativeTargetEvidenceWire {
    pub term: String,
    pub space: u8,
    pub operator: u8,
    pub weakness: u32,
    pub fragment_refs: Vec<u32>,
    pub cause: String,
}

impl NegativeTargetEvidenceWire {
    pub fn shape_is_valid(&self, mass_scale: u32) -> bool {
        let valid_space = matches!(self.space, NEGATIVE_SPACE_XI | NEGATIVE_SPACE_YI);
        let valid_operator = matches!(
            self.operator,
            NEGATIVE_OPERATOR_REVERSE
                | NEGATIVE_OPERATOR_MIRROR
                | NEGATIVE_OPERATOR_CONTRAST
                | NEGATIVE_OPERATOR_TRACE
        );
        !self.term.trim().is_empty()
            && self.term.len() <= NEGATIVE_ASSET_MAX_TEXT
            && valid_space
            && valid_operator
            && self.weakness <= mass_scale
            && self.fragment_refs.len() <= NEGATIVE_ASSET_MAX_ITEMS
            && !self.cause.trim().is_empty()
            && self.cause.len() <= NEGATIVE_ASSET_MAX_TEXT
    }

    pub fn path_wire(&self) -> String {
        let space = match self.space {
            NEGATIVE_SPACE_XI => "Xi",
            NEGATIVE_SPACE_YI => "Yi",
            _ => "Invalid",
        };
        let operator = match self.operator {
            NEGATIVE_OPERATOR_REVERSE => "Reverse",
            NEGATIVE_OPERATOR_MIRROR => "Mirror",
            NEGATIVE_OPERATOR_CONTRAST => "Contrast",
            NEGATIVE_OPERATOR_TRACE => "Trace",
            _ => "Invalid",
        };
        format!(
            "{}~{}~{}^-1:gcl_target:weakness={}",
            self.term, space, operator, self.weakness,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NegativeOperatorArgumentWire {
    pub operator: u8,
    pub weakness: u32,
    /// Exact IEEE-754 material used by the existing NPIM argument string.
    /// Shadow can therefore prove byte-equivalence without re-running NPRO.
    pub weakness_bits: u32,
    pub cause: String,
    pub consequence: String,
}

impl NegativeOperatorArgumentWire {
    pub fn shape_is_valid(&self, mass_scale: u32) -> bool {
        let valid_operator = matches!(
            self.operator,
            NEGATIVE_OPERATOR_REVERSE
                | NEGATIVE_OPERATOR_MIRROR
                | NEGATIVE_OPERATOR_CONTRAST
                | NEGATIVE_OPERATOR_TRACE
        );
        let weakness_value = f32::from_bits(self.weakness_bits);
        valid_operator
            && self.weakness <= mass_scale
            && weakness_value.is_finite()
            && (0.0..=1.0).contains(&weakness_value)
            && crate::fixed_mass(weakness_value) == Some(self.weakness)
            && !self.cause.trim().is_empty()
            && self.cause.len() <= NEGATIVE_ASSET_MAX_TEXT
            && !self.consequence.trim().is_empty()
            && self.consequence.len() <= NEGATIVE_ASSET_MAX_TEXT
    }

    pub fn operator_name(&self) -> &'static str {
        match self.operator {
            NEGATIVE_OPERATOR_REVERSE => "Reverse",
            NEGATIVE_OPERATOR_MIRROR => "Mirror",
            NEGATIVE_OPERATOR_CONTRAST => "Contrast",
            NEGATIVE_OPERATOR_TRACE => "Trace",
            _ => "Invalid",
        }
    }

    pub fn canonical_npim_argument(&self) -> String {
        format!(
            "{}|{}|{}|{:08x}",
            self.operator_name(),
            self.cause,
            self.consequence,
            self.weakness_bits,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NegativeFlowArgumentWire {
    pub module: u8,
    pub measure: u32,
    pub claim: String,
}

impl NegativeFlowArgumentWire {
    pub fn shape_is_valid(&self, mass_scale: u32) -> bool {
        NEGATIVE_FLOW_REQUIRED.contains(&self.module)
            && self.measure <= mass_scale
            && !self.claim.trim().is_empty()
            && self.claim.len() <= NEGATIVE_ASSET_MAX_TEXT
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NegativeDepthMetricsWire {
    pub requested_depth: u32,
    pub applied_depth: u32,
    /// HCP_PRO's measured average for the NPRO family before admission.
    pub npro_family_average: u32,
    /// The actual HCP_PRO Layer-3 admission applied to Negative Knowledge.
    pub npro_family_admitted: bool,
    pub module_coverage: u32,
    pub target_density: u32,
    pub active_families: u32,
    pub hcp_token_generation: u64,
}

impl NegativeDepthMetricsWire {
    pub fn shape_is_valid(&self, mass_scale: u32) -> bool {
        self.requested_depth <= mass_scale
            && self.applied_depth <= mass_scale
            && self.npro_family_average <= mass_scale
            && self.module_coverage <= mass_scale
            && self.target_density <= mass_scale
            && self.active_families <= 4
            && (!self.npro_family_admitted || self.active_families > 0)
            && self.hcp_token_generation != 0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NegativeAssetEnvelope {
    pub version: u32,
    pub module_mask: u8,
    pub asset_class: u8,
    pub gcl_process_digest: u64,
    pub gcl_law_seal: u64,
    pub gcl_system_laws_seal: u64,
    pub parent_i0_digest: u64,
    pub input_sha256: [u8; 32],

    pub lim_epistemic_mass: u32,
    pub lim_reasoning_depth: u32,
    pub npro_avg_weakness: u32,
    pub npro_consensus: u32,
    pub npro_forbidden_count: u32,
    pub npim_failure_density: u32,
    pub npim_contradiction_strength: u32,
    pub npim_elimination_certainty: u32,
    pub npim_negativity_score: u32,
    pub npim_suggestion: u8,
    pub apro_standing: u32,
    pub mpro_npro_mask: u8,
    pub mpro_factic_mass: u32,
    pub depth: NegativeDepthMetricsWire,

    pub project_id: u64,
    pub project_evidence_sha256: [u8; 32],
    pub trl_level: u8,
    pub trl_passed: bool,
    pub experimental_failure: bool,
    pub scientific_failure_context: String,

    pub targets: Vec<NegativeTargetEvidenceWire>,
    pub operator_arguments: Vec<NegativeOperatorArgumentWire>,
    pub flow_arguments: Vec<NegativeFlowArgumentWire>,
    pub token_sha256: [u8; 32],
}

impl NegativeAssetEnvelope {
    pub fn shape_is_valid(&self, mass_scale: u32) -> bool {
        let token_nonzero = self.token_sha256.iter().any(|byte| *byte != 0);
        let input_nonzero = self.input_sha256.iter().any(|byte| *byte != 0);
        let project_hash_zero = self.project_evidence_sha256.iter().all(|byte| *byte == 0);
        let failure_context_empty = self.scientific_failure_context.trim().is_empty();
        let failure_context_bounded =
            self.scientific_failure_context.len() <= NEGATIVE_ASSET_MAX_TEXT;
        let flow_modules = self
            .flow_arguments
            .iter()
            .map(|argument| argument.module)
            .collect::<Vec<_>>();
        let flow_shape = flow_modules.as_slice() == &NEGATIVE_FLOW_REQUIRED[..]
            && self
                .flow_arguments
                .iter()
                .all(|argument| argument.shape_is_valid(mass_scale));
        let project_shape = match self.project_id {
            0 => {
                !self.experimental_failure
                    && self.trl_level == 0
                    && !self.trl_passed
                    && project_hash_zero
                    && failure_context_empty
                    && self.asset_class == NEGATIVE_ASSET_CLASS_KNOWLEDGE
            }
            _ => {
                self.trl_level <= 3
                    && !project_hash_zero
                    && failure_context_bounded
                    && match self.experimental_failure {
                        true => {
                            !self.trl_passed
                                && !failure_context_empty
                                && self.asset_class == NEGATIVE_ASSET_CLASS_EXPERIMENTAL_FAILURE
                        }
                        false => {
                            failure_context_empty
                                && self.asset_class == NEGATIVE_ASSET_CLASS_KNOWLEDGE
                        }
                    }
            }
        };
        self.version == NEGATIVE_ASSET_VERSION
            && self.module_mask == NEGATIVE_MODULE_REQUIRED
            && self.gcl_process_digest != 0
            && self.gcl_law_seal != 0
            && self.gcl_system_laws_seal != 0
            && self.parent_i0_digest != 0
            && input_nonzero
            && self.lim_epistemic_mass <= mass_scale
            && self.lim_reasoning_depth <= mass_scale
            && self.npro_avg_weakness <= mass_scale
            && self.npro_consensus <= mass_scale
            && self.npro_forbidden_count <= 4
            && self.npim_failure_density <= mass_scale
            && self.npim_contradiction_strength <= mass_scale
            && self.npim_elimination_certainty <= mass_scale
            && self.npim_negativity_score <= mass_scale
            && self.npim_suggestion <= 2
            && self.apro_standing <= 4
            && self.mpro_npro_mask <= 0x0F
            && self.mpro_factic_mass <= mass_scale
            && self.depth.shape_is_valid(mass_scale)
            && project_shape
            && self.targets.len() <= NEGATIVE_ASSET_MAX_ITEMS
            && self.operator_arguments.len() <= NEGATIVE_ASSET_MAX_ITEMS
            && self
                .targets
                .iter()
                .all(|target| target.shape_is_valid(mass_scale))
            && self
                .operator_arguments
                .iter()
                .all(|argument| argument.shape_is_valid(mass_scale))
            && flow_shape
            && token_nonzero
    }

    pub fn canonical_body(&self) -> Vec<u8> {
        let mut w = Writer::new();
        w.u32(self.version);
        w.u8(self.module_mask);
        w.u8(self.asset_class);
        w.u64(self.gcl_process_digest);
        w.u64(self.gcl_law_seal);
        w.u64(self.gcl_system_laws_seal);
        w.u64(self.parent_i0_digest);
        w.fixed_32(&self.input_sha256);

        w.u32(self.lim_epistemic_mass);
        w.u32(self.lim_reasoning_depth);
        w.u32(self.npro_avg_weakness);
        w.u32(self.npro_consensus);
        w.u32(self.npro_forbidden_count);
        w.u32(self.npim_failure_density);
        w.u32(self.npim_contradiction_strength);
        w.u32(self.npim_elimination_certainty);
        w.u32(self.npim_negativity_score);
        w.u8(self.npim_suggestion);
        w.u32(self.apro_standing);
        w.u8(self.mpro_npro_mask);
        w.u32(self.mpro_factic_mass);

        w.u32(self.depth.requested_depth);
        w.u32(self.depth.applied_depth);
        w.u32(self.depth.npro_family_average);
        w.bool(self.depth.npro_family_admitted);
        w.u32(self.depth.module_coverage);
        w.u32(self.depth.target_density);
        w.u32(self.depth.active_families);
        w.u64(self.depth.hcp_token_generation);

        w.u64(self.project_id);
        w.fixed_32(&self.project_evidence_sha256);
        w.u8(self.trl_level);
        w.bool(self.trl_passed);
        w.bool(self.experimental_failure);
        w.string(&self.scientific_failure_context);

        w.u32(self.targets.len() as u32);
        for target in &self.targets {
            w.string(&target.term);
            w.u8(target.space);
            w.u8(target.operator);
            w.u32(target.weakness);
            w.vec_u32(&target.fragment_refs);
            w.string(&target.cause);
        }

        w.u32(self.operator_arguments.len() as u32);
        for argument in &self.operator_arguments {
            w.u8(argument.operator);
            w.u32(argument.weakness);
            w.u32(argument.weakness_bits);
            w.string(&argument.cause);
            w.string(&argument.consequence);
        }

        w.u32(self.flow_arguments.len() as u32);
        for argument in &self.flow_arguments {
            w.u8(argument.module);
            w.u32(argument.measure);
            w.string(&argument.claim);
        }
        w.bytes
    }

    /// Material consumed by the next-cycle PRO Negative Knowledge gate.
    /// The persistent process remains visible, while typed targets and failed
    /// scientific context become semantic constraints rather than inert bytes.
    pub fn semantic_feedback_text(&self, process: &str) -> String {
        let target_material = self
            .targets
            .iter()
            .flat_map(|target| [target.term.as_str(), target.cause.as_str()]);
        let operator_material = self
            .operator_arguments
            .iter()
            .flat_map(|argument| [argument.cause.as_str(), argument.consequence.as_str()]);
        let flow_material = self
            .flow_arguments
            .iter()
            .map(|argument| argument.claim.as_str());
        std::iter::once(process)
            .chain(std::iter::once(self.scientific_failure_context.as_str()))
            .chain(target_material)
            .chain(operator_material)
            .chain(flow_material)
            .map(str::trim)
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn encode(&self) -> Vec<u8> {
        let body = self.canonical_body();
        let mut w = Writer::new();
        w.raw(&NEGATIVE_ASSET_MAGIC);
        w.u32(body.len() as u32);
        w.raw(&body);
        w.fixed_32(&self.token_sha256);
        w.bytes
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, NegativeAssetDecodeError> {
        let mut r = Reader::new(bytes);
        let magic = r.take(NEGATIVE_ASSET_MAGIC.len())?;
        match magic == NEGATIVE_ASSET_MAGIC.as_slice() {
            true => {}
            false => return Err(NegativeAssetDecodeError::InvalidMagic),
        }
        let body_len = r.u32()? as usize;
        match body_len <= bytes.len() {
            true => {}
            false => return Err(NegativeAssetDecodeError::InvalidLength),
        }
        let body = r.take(body_len)?;
        let token_sha256 = r.fixed_32()?;
        r.finish()?;

        let mut b = Reader::new(body);
        let version = b.u32()?;
        let module_mask = b.u8()?;
        let asset_class = b.u8()?;
        let gcl_process_digest = b.u64()?;
        let gcl_law_seal = b.u64()?;
        let gcl_system_laws_seal = b.u64()?;
        let parent_i0_digest = b.u64()?;
        let input_sha256 = b.fixed_32()?;

        let lim_epistemic_mass = b.u32()?;
        let lim_reasoning_depth = b.u32()?;
        let npro_avg_weakness = b.u32()?;
        let npro_consensus = b.u32()?;
        let npro_forbidden_count = b.u32()?;
        let npim_failure_density = b.u32()?;
        let npim_contradiction_strength = b.u32()?;
        let npim_elimination_certainty = b.u32()?;
        let npim_negativity_score = b.u32()?;
        let npim_suggestion = b.u8()?;
        let apro_standing = b.u32()?;
        let mpro_npro_mask = b.u8()?;
        let mpro_factic_mass = b.u32()?;

        let depth = NegativeDepthMetricsWire {
            requested_depth: b.u32()?,
            applied_depth: b.u32()?,
            npro_family_average: b.u32()?,
            npro_family_admitted: b.bool()?,
            module_coverage: b.u32()?,
            target_density: b.u32()?,
            active_families: b.u32()?,
            hcp_token_generation: b.u64()?,
        };

        let project_id = b.u64()?;
        let project_evidence_sha256 = b.fixed_32()?;
        let trl_level = b.u8()?;
        let trl_passed = b.bool()?;
        let experimental_failure = b.bool()?;
        let scientific_failure_context = b.string()?;

        let target_count = b.count()?;
        let mut targets = Vec::with_capacity(target_count);
        for _ in 0..target_count {
            targets.push(NegativeTargetEvidenceWire {
                term: b.string()?,
                space: b.u8()?,
                operator: b.u8()?,
                weakness: b.u32()?,
                fragment_refs: b.vec_u32()?,
                cause: b.string()?,
            });
        }

        let argument_count = b.count()?;
        let mut operator_arguments = Vec::with_capacity(argument_count);
        for _ in 0..argument_count {
            operator_arguments.push(NegativeOperatorArgumentWire {
                operator: b.u8()?,
                weakness: b.u32()?,
                weakness_bits: b.u32()?,
                cause: b.string()?,
                consequence: b.string()?,
            });
        }

        let flow_count = b.count()?;
        let mut flow_arguments = Vec::with_capacity(flow_count);
        for _ in 0..flow_count {
            flow_arguments.push(NegativeFlowArgumentWire {
                module: b.u8()?,
                measure: b.u32()?,
                claim: b.string()?,
            });
        }
        b.finish()?;

        Ok(Self {
            version,
            module_mask,
            asset_class,
            gcl_process_digest,
            gcl_law_seal,
            gcl_system_laws_seal,
            parent_i0_digest,
            input_sha256,
            lim_epistemic_mass,
            lim_reasoning_depth,
            npro_avg_weakness,
            npro_consensus,
            npro_forbidden_count,
            npim_failure_density,
            npim_contradiction_strength,
            npim_elimination_certainty,
            npim_negativity_score,
            npim_suggestion,
            apro_standing,
            mpro_npro_mask,
            mpro_factic_mass,
            depth,
            project_id,
            project_evidence_sha256,
            trl_level,
            trl_passed,
            experimental_failure,
            scientific_failure_context,
            targets,
            operator_arguments,
            flow_arguments,
            token_sha256,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NegativeAssetDecodeError {
    Truncated,
    InvalidMagic,
    InvalidLength,
    InvalidBool,
    InvalidUtf8,
    VectorTooLarge,
    TextTooLarge,
    TrailingBytes,
}

struct Writer {
    bytes: Vec<u8>,
}

impl Writer {
    fn new() -> Self {
        Self { bytes: Vec::new() }
    }
    fn raw(&mut self, value: &[u8]) {
        self.bytes.extend_from_slice(value);
    }
    fn u8(&mut self, value: u8) {
        self.bytes.push(value);
    }
    fn bool(&mut self, value: bool) {
        self.u8(value as u8);
    }
    fn u32(&mut self, value: u32) {
        self.raw(&value.to_le_bytes());
    }
    fn u64(&mut self, value: u64) {
        self.raw(&value.to_le_bytes());
    }
    fn fixed_32(&mut self, value: &[u8; 32]) {
        self.raw(value);
    }
    fn string(&mut self, value: &str) {
        self.u32(value.len() as u32);
        self.raw(value.as_bytes());
    }
    fn vec_u32(&mut self, values: &[u32]) {
        self.u32(values.len() as u32);
        for value in values {
            self.u32(*value);
        }
    }
}

struct Reader<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, pos: 0 }
    }
    fn take(&mut self, len: usize) -> Result<&'a [u8], NegativeAssetDecodeError> {
        let end = self
            .pos
            .checked_add(len)
            .ok_or(NegativeAssetDecodeError::Truncated)?;
        let value = self
            .bytes
            .get(self.pos..end)
            .ok_or(NegativeAssetDecodeError::Truncated)?;
        self.pos = end;
        Ok(value)
    }
    fn u8(&mut self) -> Result<u8, NegativeAssetDecodeError> {
        Ok(*self
            .take(1)?
            .first()
            .ok_or(NegativeAssetDecodeError::Truncated)?)
    }
    fn bool(&mut self) -> Result<bool, NegativeAssetDecodeError> {
        match self.u8()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(NegativeAssetDecodeError::InvalidBool),
        }
    }
    fn u32(&mut self) -> Result<u32, NegativeAssetDecodeError> {
        let mut raw = [0u8; 4];
        raw.copy_from_slice(self.take(4)?);
        Ok(u32::from_le_bytes(raw))
    }
    fn u64(&mut self) -> Result<u64, NegativeAssetDecodeError> {
        let mut raw = [0u8; 8];
        raw.copy_from_slice(self.take(8)?);
        Ok(u64::from_le_bytes(raw))
    }
    fn fixed_32(&mut self) -> Result<[u8; 32], NegativeAssetDecodeError> {
        let mut raw = [0u8; 32];
        raw.copy_from_slice(self.take(32)?);
        Ok(raw)
    }
    fn count(&mut self) -> Result<usize, NegativeAssetDecodeError> {
        let count = self.u32()? as usize;
        match count <= NEGATIVE_ASSET_MAX_ITEMS {
            true => Ok(count),
            false => Err(NegativeAssetDecodeError::VectorTooLarge),
        }
    }
    fn string(&mut self) -> Result<String, NegativeAssetDecodeError> {
        let len = self.u32()? as usize;
        match len <= NEGATIVE_ASSET_MAX_TEXT {
            true => {}
            false => return Err(NegativeAssetDecodeError::TextTooLarge),
        }
        String::from_utf8(self.take(len)?.to_vec())
            .map_err(|_| NegativeAssetDecodeError::InvalidUtf8)
    }
    fn vec_u32(&mut self) -> Result<Vec<u32>, NegativeAssetDecodeError> {
        let count = self.count()?;
        let mut values = Vec::with_capacity(count);
        for _ in 0..count {
            values.push(self.u32()?);
        }
        Ok(values)
    }
    fn finish(&self) -> Result<(), NegativeAssetDecodeError> {
        match self.pos == self.bytes.len() {
            true => Ok(()),
            false => Err(NegativeAssetDecodeError::TrailingBytes),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> NegativeAssetEnvelope {
        NegativeAssetEnvelope {
            version: NEGATIVE_ASSET_VERSION,
            module_mask: NEGATIVE_MODULE_REQUIRED,
            asset_class: NEGATIVE_ASSET_CLASS_EXPERIMENTAL_FAILURE,
            gcl_process_digest: 7,
            gcl_law_seal: 8,
            gcl_system_laws_seal: 9,
            parent_i0_digest: 10,
            input_sha256: [1u8; 32],
            lim_epistemic_mass: 5_000,
            lim_reasoning_depth: 6_000,
            npro_avg_weakness: 7_000,
            npro_consensus: 8_000,
            npro_forbidden_count: 2,
            npim_failure_density: 7_000,
            npim_contradiction_strength: 7_500,
            npim_elimination_certainty: 8_000,
            npim_negativity_score: 7_800,
            npim_suggestion: 2,
            apro_standing: 4,
            mpro_npro_mask: 0b1010,
            mpro_factic_mass: 5_500,
            depth: NegativeDepthMetricsWire {
                requested_depth: 8_000,
                applied_depth: 4_480,
                npro_family_average: 8_000,
                npro_family_admitted: true,
                module_coverage: 10_000,
                target_density: 5_000,
                active_families: 4,
                hcp_token_generation: 2,
            },
            project_id: 55,
            project_evidence_sha256: [2u8; 32],
            trl_level: 2,
            trl_passed: false,
            experimental_failure: true,
            scientific_failure_context: canonical_scientific_failure_context(
                "Failure study",
                "materials",
                "unsupported pressure claim",
                &["reproducibility evidence absent".to_string()],
                "LAB-FAIL-17",
                &["pressure response was not reproduced".to_string()],
                "Controlled trial rejected the hypothesis.",
            ),
            targets: vec![NegativeTargetEvidenceWire {
                term: "unsupported".to_string(),
                space: NEGATIVE_SPACE_XI,
                operator: NEGATIVE_OPERATOR_TRACE,
                weakness: 8_000,
                fragment_refs: vec![0],
                cause: "trace ended".to_string(),
            }],
            operator_arguments: vec![NegativeOperatorArgumentWire {
                operator: NEGATIVE_OPERATOR_TRACE,
                weakness: 8_000,
                weakness_bits: 0.8f32.to_bits(),
                cause: "trace ended".to_string(),
                consequence: "dead end".to_string(),
            }],
            flow_arguments: vec![
                NegativeFlowArgumentWire {
                    module: NEGATIVE_FLOW_LIM,
                    measure: 5_000,
                    claim: "LIM bounded the negative search space".to_string(),
                },
                NegativeFlowArgumentWire {
                    module: NEGATIVE_FLOW_NPRO,
                    measure: 8_000,
                    claim: "NPRO measured operator-level weakness".to_string(),
                },
                NegativeFlowArgumentWire {
                    module: NEGATIVE_FLOW_NPIM,
                    measure: 7_800,
                    claim: "NPIM packaged negative evidence".to_string(),
                },
                NegativeFlowArgumentWire {
                    module: NEGATIVE_FLOW_APRO,
                    measure: 10_000,
                    claim: "APRO preserved standing counter-arguments".to_string(),
                },
                NegativeFlowArgumentWire {
                    module: NEGATIVE_FLOW_MPRO,
                    measure: 5_000,
                    claim: "MPRO measured admitted NPRO contribution".to_string(),
                },
                NegativeFlowArgumentWire {
                    module: NEGATIVE_FLOW_HCP_PRO,
                    measure: 4_480,
                    claim: "HCP_PRO applied reasoning depth".to_string(),
                },
            ],
            token_sha256: [3u8; 32],
        }
    }

    #[test]
    fn envelope_roundtrip_is_canonical() {
        let source = sample();
        let bytes = source.encode();
        let decoded = NegativeAssetEnvelope::decode(&bytes).expect("decode");
        assert_eq!(decoded, source);
        assert_eq!(decoded.canonical_body(), source.canonical_body());
        assert!(decoded.shape_is_valid(10_000));
    }

    #[test]
    fn target_path_wire_is_existing_shadow_shape() {
        let mut source = sample();
        let target = source.targets.remove(0);
        assert!(target.path_wire().starts_with("unsupported~Xi~Trace^-1"));
    }

    #[test]
    fn operator_argument_reconstructs_existing_npim_wire() {
        let mut source = sample();
        let argument = source.operator_arguments.remove(0);
        assert_eq!(
            argument.canonical_npim_argument(),
            format!("Trace|trace ended|dead end|{:08x}", 0.8f32.to_bits()),
        );
    }

    #[test]
    fn semantic_feedback_keeps_failed_hypothesis_and_negative_term_alive() {
        let source = sample();
        let text = source.semantic_feedback_text("LAB:NPIM");
        assert!(text.contains("unsupported pressure claim"));
        assert!(text.contains("unsupported"));
        assert!(text.contains("trace ended"));
    }
}
