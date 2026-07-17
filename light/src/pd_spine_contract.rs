// ╔══════════════════════════════════════════════════════════════════════════╗
// ║ PD_SPINE_CONTRACT.RS — GCL-governed PD ↔ Spine 9 ↔ Layer 1/2/3       ║
// ║ ESS-MAI v1.6.0 — byte-for-byte identike në Light/Quantum/Shadow        ║
// ╚══════════════════════════════════════════════════════════════════════════╝

pub const PD_SPINE_CONTRACT_VERSION: u32 = 0x0001_0600;
pub const MASS_SCALE: u32 = 10_000;
pub const MPRO_EVALUATIONS: u32 = 16;
pub const LAYER_1_MASK: u8 = 0b001;
pub const LAYER_2_MASK: u8 = 0b010;
pub const LAYER_3_MASK: u8 = 0b100;
pub const ALL_LAYERS_MASK: u8 = LAYER_1_MASK | LAYER_2_MASK | LAYER_3_MASK;

use crate::pd_continuum_contract::PdContinuumActivation;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FixedMass(pub u32);
impl FixedMass {
    pub const ZERO: Self = Self(0);
    pub const FULL: Self = Self(MASS_SCALE);
    pub fn new(raw: u32) -> Self { Self(raw.min(MASS_SCALE)) }
    pub fn from_ratio(positive: u32, total: u32) -> Self {
        match total {
            0 => Self::ZERO,
            _ => Self::new(positive.saturating_mul(MASS_SCALE) / total),
        }
    }
    pub fn from_basis_points(raw: u32) -> Self { Self::new(raw) }
    pub fn raw(self) -> u32 { self.0 }
    pub fn is_at_least_half(self) -> bool { self.0 >= MASS_SCALE / 2 }
    pub fn decimal_4(self) -> String {
        format!("{}.{:04}", self.0 / MASS_SCALE, self.0 % MASS_SCALE)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PdCognitiveMode { Intellect, Philosophy, Scientific, Novel }
impl PdCognitiveMode {
    pub fn label(self) -> &'static str {
        match self {
            Self::Intellect => "INTELLECT",
            Self::Philosophy => "PHILOSOPHY",
            Self::Scientific => "SCIENTIFIC",
            Self::Novel => "NOVEL",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PdActivationOrigin {
    NormalTurn, GeniusAutonomousResearch, BudgetExhausted, SessionClosed,
}
impl PdActivationOrigin {
    pub fn label(self) -> &'static str {
        match self {
            Self::NormalTurn => "NORMAL_TURN",
            Self::GeniusAutonomousResearch => "GENIUS_AUTONOMOUS_RESEARCH",
            Self::BudgetExhausted => "BUDGET_EXHAUSTED",
            Self::SessionClosed => "SESSION_CLOSED",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PdActivationContract {
    pub mode: PdCognitiveMode,
    pub origin: PdActivationOrigin,
    pub response_kind_digest: u64,
    pub genius_signal_digest: u64,
    pub trace_mass: FixedMass,
    pub structural_coherence: FixedMass,
    pub required_layer_mask: u8,
    pub contract_digest: u64,
}
impl PdActivationContract {
    pub fn new(
        mode: PdCognitiveMode,
        origin: PdActivationOrigin,
        response_kind: &str,
        genius_signal_digest: u64,
        trace_mass: u32,
        structural_coherence: u32,
    ) -> Self {
        // PD aktivizon të gjithë Spine 9 në çdo modalitet. Mode përcakton
        // karakterin/thellësinë e procedimit, jo heqjen e një Layer-i.
        let required_layer_mask = ALL_LAYERS_MASK;
        let response_kind_digest = digest_parts(&[response_kind.as_bytes()]);
        let trace_mass = FixedMass::new(trace_mass);
        let structural_coherence = FixedMass::new(structural_coherence);
        let contract_digest = digest_parts(&[
            &PD_SPINE_CONTRACT_VERSION.to_le_bytes(),
            mode.label().as_bytes(),
            origin.label().as_bytes(),
            &response_kind_digest.to_le_bytes(),
            &genius_signal_digest.to_le_bytes(),
            &trace_mass.raw().to_le_bytes(),
            &structural_coherence.raw().to_le_bytes(),
            &[required_layer_mask],
            b"PD_ACTIVATION_CONTRACT_V160",
        ]);
        Self {
            mode, origin, response_kind_digest, genius_signal_digest, trace_mass,
            structural_coherence, required_layer_mask, contract_digest,
        }
    }

    pub fn verifies(&self) -> bool {
        self.required_layer_mask == ALL_LAYERS_MASK
            && self.contract_digest == digest_parts(&[
                &PD_SPINE_CONTRACT_VERSION.to_le_bytes(),
                self.mode.label().as_bytes(),
                self.origin.label().as_bytes(),
                &self.response_kind_digest.to_le_bytes(),
                &self.genius_signal_digest.to_le_bytes(),
                &self.trace_mass.raw().to_le_bytes(),
                &self.structural_coherence.raw().to_le_bytes(),
                &[self.required_layer_mask],
                b"PD_ACTIVATION_CONTRACT_V160",
            ])
            && match self.origin {
                PdActivationOrigin::GeniusAutonomousResearch => {
                    self.mode == PdCognitiveMode::Novel
                        && self.genius_signal_digest != 0
                        && self.required_layer_mask == ALL_LAYERS_MASK
                }
                _ => true,
            }
    }
}

/// Autoriteti i të njëjtit proces GCL që ekziston PARA Spine 9 dhe mbetet
/// i pandryshuar gjatë Layer 1→2→3. Layers e thellojnë procedimin; nuk e
/// kryqëzojnë, anashkalojnë ose zëvendësojnë GCL-në.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GclProcessAuthority {
    pub law_seal: u64,
    pub system_laws_seal: u64,
    pub phase_digest: u64,
    pub process_digest: u64,
}
impl GclProcessAuthority {
    pub fn new(
        law_seal: u64,
        system_laws_seal: u64,
        phase_label: &str,
        session_digest: u64,
        parent_i0_digest: u64,
        continuum_activation_digest: u64,
        activation_contract_digest: u64,
        created_at: u64,
    ) -> Self {
        let phase_digest = digest_parts(&[phase_label.as_bytes()]);
        let process_digest = digest_parts(&[
            &PD_SPINE_CONTRACT_VERSION.to_le_bytes(),
            &law_seal.to_le_bytes(),
            &system_laws_seal.to_le_bytes(),
            &phase_digest.to_le_bytes(),
            &session_digest.to_le_bytes(),
            &parent_i0_digest.to_le_bytes(),
            &continuum_activation_digest.to_le_bytes(),
            &activation_contract_digest.to_le_bytes(),
            &created_at.to_le_bytes(),
            b"GCL_PD_SPINE_PROCESS_V160",
        ]);
        Self { law_seal, system_laws_seal, phase_digest, process_digest }
    }

    pub fn verifies_for(
        &self,
        expected_law_seal: u64,
        expected_system_laws_seal: u64,
        session_digest: u64,
        parent_i0_digest: u64,
        continuum_activation_digest: u64,
        activation_contract_digest: u64,
        created_at: u64,
    ) -> bool {
        self.law_seal == expected_law_seal
            && self.system_laws_seal == expected_system_laws_seal
            && self.phase_digest == digest_parts(&[b"KOLAPS_ARSYETIMI"])
            && self.process_digest == digest_parts(&[
                &PD_SPINE_CONTRACT_VERSION.to_le_bytes(),
                &self.law_seal.to_le_bytes(),
                &self.system_laws_seal.to_le_bytes(),
                &self.phase_digest.to_le_bytes(),
                &session_digest.to_le_bytes(),
                &parent_i0_digest.to_le_bytes(),
                &continuum_activation_digest.to_le_bytes(),
                &activation_contract_digest.to_le_bytes(),
                &created_at.to_le_bytes(),
                b"GCL_PD_SPINE_PROCESS_V160",
            ])
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PdSpineState {
    TurnCaptured, TraceAccumulated, CognitivePackaged, SpineActivated,
    LayersComplete, PreSealed, WaitingShadow, Verified, PdLight, Nura,
}
impl PdSpineState {
    pub fn label(self) -> &'static str {
        match self {
            Self::TurnCaptured => "TURN_CAPTURED",
            Self::TraceAccumulated => "TRACE_ACCUMULATED",
            Self::CognitivePackaged => "COGNITIVE_PACKAGED",
            Self::SpineActivated => "SPINE9_ACTIVATED_UNDER_GCL",
            Self::LayersComplete => "LAYERS_1_2_3_COMPLETE_UNDER_GCL",
            Self::PreSealed => "PD_PRE_SEALED",
            Self::WaitingShadow => "WAITING_SHADOW",
            Self::Verified => "SHADOW_VERIFIED",
            Self::PdLight => "PD_LIGHT",
            Self::Nura => "NURA",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PdSpineRequest {
    pub contract_version: u32,
    pub activation_id: u64,
    pub session_digest: u64,
    pub parent_i0_digest: u64,
    pub trace_digest: u64,
    pub turn_digest: u64,
    pub created_at: u64,
    pub continuum: PdContinuumActivation,
    pub activation: PdActivationContract,
    pub gcl: GclProcessAuthority,
    pub state: PdSpineState,
}
impl PdSpineRequest {
    pub fn new(
        session_id: &str,
        parent_i0: &str,
        user_input: &str,
        question: &str,
        response_kind: &str,
        trace_ref: &str,
        turn_ref: &str,
        created_at: u64,
    ) -> Self {
        Self::new_with_activation_under_gcl(
            session_id,
            parent_i0,
            user_input,
            question,
            response_kind,
            trace_ref,
            turn_ref,
            created_at,
            PdActivationContract::new(
                PdCognitiveMode::Intellect,
                PdActivationOrigin::NormalTurn,
                response_kind,
                0,
                0,
                0,
            ),
            digest_parts(&[b"gjata_collapse_law"]),
            digest_parts(&[b"ESS_MAI_SYSTEM_LAWS_TEST"]),
            "KOLAPS_ARSYETIMI",
        )
    }

    pub fn new_with_activation_under_gcl(
        session_id: &str,
        parent_i0: &str,
        user_input: &str,
        question: &str,
        response_kind: &str,
        trace_ref: &str,
        turn_ref: &str,
        created_at: u64,
        activation: PdActivationContract,
        law_seal: u64,
        system_laws_seal: u64,
        phase_label: &str,
    ) -> Self {
        let continuum = PdContinuumActivation::new(
            session_id, parent_i0, user_input, question, response_kind, created_at,
        );
        let session_digest = digest_parts(&[session_id.as_bytes()]);
        let parent_i0_digest = digest_parts(&[parent_i0.as_bytes()]);
        let trace_digest = digest_parts(&[trace_ref.as_bytes()]);
        let turn_digest = digest_parts(&[turn_ref.as_bytes()]);
        let gcl = GclProcessAuthority::new(
            law_seal,
            system_laws_seal,
            phase_label,
            session_digest,
            parent_i0_digest,
            continuum.activation_digest,
            activation.contract_digest,
            created_at,
        );
        let activation_id = digest_parts(&[
            &PD_SPINE_CONTRACT_VERSION.to_le_bytes(),
            &session_digest.to_le_bytes(),
            &parent_i0_digest.to_le_bytes(),
            &trace_digest.to_le_bytes(),
            &turn_digest.to_le_bytes(),
            &continuum.activation_digest.to_le_bytes(),
            &activation.contract_digest.to_le_bytes(),
            &gcl.process_digest.to_le_bytes(),
            &created_at.to_le_bytes(),
            b"PD_SPINE9_ACTIVATION_V160",
        ]);
        Self {
            contract_version: PD_SPINE_CONTRACT_VERSION,
            activation_id,
            session_digest,
            parent_i0_digest,
            trace_digest,
            turn_digest,
            created_at,
            continuum,
            activation,
            gcl,
            state: PdSpineState::SpineActivated,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PdLayerReceipt {
    pub layer_mask: u8,
    pub activation_id: u64,
    pub activation_contract_digest: u64,
    pub gcl_process_digest: u64,
    pub parent_result_digest: u64,
    pub result_material_digest: u64,
    pub result_digest: u64,
    pub completed: bool,
}
impl PdLayerReceipt {
    pub fn new(
        layer_mask: u8,
        activation_id: u64,
        activation_contract_digest: u64,
        gcl_process_digest: u64,
        parent_result_digest: u64,
        result_material_digest: u64,
        completed: bool,
    ) -> Self {
        let result_digest = digest_parts(&[
            &layer_mask.to_le_bytes(),
            &activation_id.to_le_bytes(),
            &activation_contract_digest.to_le_bytes(),
            &gcl_process_digest.to_le_bytes(),
            &parent_result_digest.to_le_bytes(),
            &result_material_digest.to_le_bytes(),
            &[completed as u8],
            b"PD_LAYER_RECEIPT_UNDER_GCL_V160",
        ]);
        Self {
            layer_mask,
            activation_id,
            activation_contract_digest,
            gcl_process_digest,
            parent_result_digest,
            result_material_digest,
            result_digest,
            completed,
        }
    }

    pub fn verifies(
        &self,
        request: &PdSpineRequest,
        expected_layer: u8,
        expected_parent: u64,
    ) -> bool {
        self.completed
            && self.layer_mask == expected_layer
            && self.activation_id == request.activation_id
            && self.activation_contract_digest == request.activation.contract_digest
            && self.gcl_process_digest == request.gcl.process_digest
            && self.parent_result_digest == expected_parent
            && self.result_digest == digest_parts(&[
                &self.layer_mask.to_le_bytes(),
                &self.activation_id.to_le_bytes(),
                &self.activation_contract_digest.to_le_bytes(),
                &self.gcl_process_digest.to_le_bytes(),
                &self.parent_result_digest.to_le_bytes(),
                &self.result_material_digest.to_le_bytes(),
                &[self.completed as u8],
                b"PD_LAYER_RECEIPT_UNDER_GCL_V160",
            ])
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PdSpineCompletion {
    pub request: PdSpineRequest,
    pub layer_mask: u8,
    pub layer1: PdLayerReceipt,
    pub layer2: PdLayerReceipt,
    pub layer3: PdLayerReceipt,
    pub evidence_digest: u64,
    pub mpro_mass: FixedMass,
    pub ready_for_shadow: bool,
    pub completed_at: u64,
}
impl PdSpineCompletion {
    pub fn closes_required_layers(&self) -> bool {
        let required = self.request.activation.required_layer_mask;
        let l1 = self.layer1.verifies(&self.request, LAYER_1_MASK, 0);
        let l2 = self.layer2.verifies(
            &self.request, LAYER_2_MASK, self.layer1.result_digest,
        );
        let l3 = self.layer3.verifies(
            &self.request, LAYER_3_MASK, self.layer2.result_digest,
        );
        let required_ok = ((required & LAYER_1_MASK) == 0 || l1)
            && ((required & LAYER_2_MASK) == 0 || l2)
            && ((required & LAYER_3_MASK) == 0 || l3);
        self.request.contract_version == PD_SPINE_CONTRACT_VERSION
            && self.request.continuum.is_ready()
            && self.request.activation.verifies()
            && self.request.gcl.process_digest != 0
            && (self.layer_mask & required) == required
            && required_ok
            && self.evidence_digest != 0
            && self.ready_for_shadow
    }

    pub fn closes_all_layers(&self) -> bool {
        self.closes_required_layers()
            && self.layer_mask == ALL_LAYERS_MASK
            && self.layer1.completed
            && self.layer2.completed
            && self.layer3.completed
    }

    pub fn completion_digest(&self) -> u64 {
        digest_parts(&[
            &self.request.activation_id.to_le_bytes(),
            &self.request.activation.contract_digest.to_le_bytes(),
            &self.request.gcl.process_digest.to_le_bytes(),
            &self.layer1.result_digest.to_le_bytes(),
            &self.layer2.result_digest.to_le_bytes(),
            &self.layer3.result_digest.to_le_bytes(),
            &[self.layer_mask],
            &self.evidence_digest.to_le_bytes(),
            &self.mpro_mass.raw().to_le_bytes(),
            &self.completed_at.to_le_bytes(),
            b"PD_SPINE9_COMPLETE_UNDER_GCL_V160",
        ])
    }
}

pub fn digest_parts(parts: &[&[u8]]) -> u64 {
    parts.iter().fold(0xcbf2_9ce4_8422_2325u64, |acc, part| {
        part.iter().fold(acc, |hash, byte| {
            (hash ^ u64::from(*byte)).wrapping_mul(0x0000_0100_0000_01b3)
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn genius_requires_novel_and_three_layers() {
        let a = PdActivationContract::new(
            PdCognitiveMode::Novel,
            PdActivationOrigin::GeniusAutonomousResearch,
            "GENIUS",
            7,
            8000,
            7000,
        );
        assert!(a.verifies());
        assert_eq!(a.required_layer_mask, ALL_LAYERS_MASK);
    }

    #[test]
    fn every_layer_is_bound_to_the_same_gcl_process() {
        let r = PdSpineRequest::new(
            "s", "i0", "input", "question", "CLARIFY", "trace", "turn", 1,
        );
        let l1 = PdLayerReceipt::new(
            LAYER_1_MASK,
            r.activation_id,
            r.activation.contract_digest,
            r.gcl.process_digest,
            0,
            9,
            true,
        );
        assert!(l1.verifies(&r, LAYER_1_MASK, 0));
    }
}
