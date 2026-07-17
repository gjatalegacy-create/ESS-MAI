// ╔══════════════════════════════════════════════════════════════════════════╗
// ║ PD_CONTINUUM_CONTRACT.RS — i + U → i₀ → i₀ + 1Q → output + iZ → next i₀ ║
// ║ ESS-MAI v1.5.7 — byte-for-byte identike në Light/Quantum/Shadow          ║
// ╚══════════════════════════════════════════════════════════════════════════╝

pub const PD_CONTINUUM_CONTRACT_VERSION: u32 = 0x0001_0601;
pub const QUESTION_INCREMENT_UNITS: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PdContinuumState {
    I0Stimulated,
    QuestionIncremented,
    SpineActivated,
    VerifiedOutput,
    IzCompleted,
    NextI0Derived,
}

impl PdContinuumState {
    pub fn label(self) -> &'static str {
        match self {
            Self::I0Stimulated       => "I0_STIMULATED",
            Self::QuestionIncremented => "I0_PLUS_1Q",
            Self::SpineActivated     => "PD_SPINE9_ACTIVATED",
            Self::VerifiedOutput     => "PD_VERIFIED_OUTPUT",
            Self::IzCompleted        => "IZ_COMPLETED",
            Self::NextI0Derived      => "OUTPUT_PLUS_IZ_TO_NEXT_I0",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PdContinuumBasis {
    VerifiedPositive,
    VerifiedNegativeRebuild,
}

impl PdContinuumBasis {
    pub fn label(self) -> &'static str {
        match self {
            Self::VerifiedPositive        => "VERIFIED_POSITIVE",
            Self::VerifiedNegativeRebuild => "VERIFIED_NEGATIVE_REBUILD",
        }
    }

    pub fn from_label(label: &str) -> Option<Self> {
        match label {
            "VERIFIED_POSITIVE"        => Some(Self::VerifiedPositive),
            "VERIFIED_NEGATIVE_REBUILD" => Some(Self::VerifiedNegativeRebuild),
            _ => None,
        }
    }
}

/// Lidhja kushtetuese `i + U(user input) → i₀`.
/// `i₀` ruan identitetin ekzistues të primitivit; kontrata e lidh atë me
/// gjendjen fillestare të sesionit dhe me input-in real të përdoruesit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PdI0Stimulus {
    pub contract_version: u32,
    pub session_digest: u64,
    pub initial_i_digest: u64,
    pub user_input_digest: u64,
    pub i0_digest: u64,
    pub stimulus_digest: u64,
    pub created_at: u64,
    pub state: PdContinuumState,
}

impl PdI0Stimulus {
    pub fn new(session_id: &str, parent_i0: &str, user_input: &str, created_at: u64) -> Self {
        let session_digest = digest_parts(&[session_id.as_bytes()]);
        let initial_i_digest = digest_parts(&[
            session_id.as_bytes(),
            b"GCL_INITIAL_I",
        ]);
        let user_input_digest = digest_parts(&[user_input.as_bytes()]);
        let i0_digest = digest_parts(&[parent_i0.as_bytes()]);
        let stimulus_digest = digest_parts(&[
            &PD_CONTINUUM_CONTRACT_VERSION.to_le_bytes(),
            &session_digest.to_le_bytes(),
            &initial_i_digest.to_le_bytes(),
            &user_input_digest.to_le_bytes(),
            &i0_digest.to_le_bytes(),
            &created_at.to_le_bytes(),
            b"I_PLUS_U_TO_I0",
        ]);
        Self {
            contract_version: PD_CONTINUUM_CONTRACT_VERSION,
            session_digest,
            initial_i_digest,
            user_input_digest,
            i0_digest,
            stimulus_digest,
            created_at,
            state: PdContinuumState::I0Stimulated,
        }
    }

    pub fn verifies(&self) -> bool {
        self.contract_version == PD_CONTINUUM_CONTRACT_VERSION
            && self.session_digest != 0
            && self.initial_i_digest != 0
            && self.user_input_digest != 0
            && self.i0_digest != 0
            && self.stimulus_digest == digest_parts(&[
                &self.contract_version.to_le_bytes(),
                &self.session_digest.to_le_bytes(),
                &self.initial_i_digest.to_le_bytes(),
                &self.user_input_digest.to_le_bytes(),
                &self.i0_digest.to_le_bytes(),
                &self.created_at.to_le_bytes(),
                b"I_PLUS_U_TO_I0",
            ])
            && self.state == PdContinuumState::I0Stimulated
    }
}

/// `1Q(question IQ)` është një increment i vetëm, i identifikuar nga pyetja,
/// lloji i përgjigjes së PD dhe stimuli i₀. Nuk shpik score IQ numerik.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PdQuestionIncrement {
    pub contract_version: u32,
    pub stimulus_digest: u64,
    pub unit_count: u32,
    pub question_digest: u64,
    pub response_kind_digest: u64,
    pub increment_digest: u64,
    pub created_at: u64,
    pub state: PdContinuumState,
}

impl PdQuestionIncrement {
    pub fn new(
        stimulus: &PdI0Stimulus,
        question: &str,
        response_kind: &str,
        created_at: u64,
    ) -> Self {
        let unit_count = match question.trim().is_empty() {
            true => 0,
            false => QUESTION_INCREMENT_UNITS,
        };
        let question_digest = digest_parts(&[question.as_bytes()]);
        let response_kind_digest = digest_parts(&[response_kind.as_bytes()]);
        let increment_digest = digest_parts(&[
            &PD_CONTINUUM_CONTRACT_VERSION.to_le_bytes(),
            &stimulus.stimulus_digest.to_le_bytes(),
            &unit_count.to_le_bytes(),
            &question_digest.to_le_bytes(),
            &response_kind_digest.to_le_bytes(),
            &created_at.to_le_bytes(),
            b"I0_PLUS_ONE_QUESTION_IQ",
        ]);
        Self {
            contract_version: PD_CONTINUUM_CONTRACT_VERSION,
            stimulus_digest: stimulus.stimulus_digest,
            unit_count,
            question_digest,
            response_kind_digest,
            increment_digest,
            created_at,
            state: PdContinuumState::QuestionIncremented,
        }
    }

    pub fn verifies(&self, stimulus: &PdI0Stimulus) -> bool {
        self.contract_version == PD_CONTINUUM_CONTRACT_VERSION
            && stimulus.verifies()
            && self.stimulus_digest == stimulus.stimulus_digest
            && self.unit_count == QUESTION_INCREMENT_UNITS
            && self.question_digest != 0
            && self.response_kind_digest != 0
            && self.increment_digest == digest_parts(&[
                &self.contract_version.to_le_bytes(),
                &self.stimulus_digest.to_le_bytes(),
                &self.unit_count.to_le_bytes(),
                &self.question_digest.to_le_bytes(),
                &self.response_kind_digest.to_le_bytes(),
                &self.created_at.to_le_bytes(),
                b"I0_PLUS_ONE_QUESTION_IQ",
            ])
            && self.state == PdContinuumState::QuestionIncremented
    }
}

/// Aktivizimi që PD ia dorëzon Spine 9. Ai mban së bashku i₀-në e nxitur dhe
/// incrementin e vetëm 1Q, ndaj shtresat nuk mund të punojnë mbi një pyetje
/// tjetër ose mbi një i₀ tjetër.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PdContinuumActivation {
    pub contract_version: u32,
    pub stimulus: PdI0Stimulus,
    pub question_increment: PdQuestionIncrement,
    pub activation_digest: u64,
    pub state: PdContinuumState,
}

impl PdContinuumActivation {
    pub fn new(
        session_id: &str,
        parent_i0: &str,
        user_input: &str,
        question: &str,
        response_kind: &str,
        created_at: u64,
    ) -> Self {
        let stimulus = PdI0Stimulus::new(session_id, parent_i0, user_input, created_at);
        let question_increment = PdQuestionIncrement::new(
            &stimulus,
            question,
            response_kind,
            created_at,
        );
        let activation_digest = digest_parts(&[
            &PD_CONTINUUM_CONTRACT_VERSION.to_le_bytes(),
            &stimulus.stimulus_digest.to_le_bytes(),
            &question_increment.increment_digest.to_le_bytes(),
            b"PD_CONTINUUM_ACTIVATION",
        ]);
        Self {
            contract_version: PD_CONTINUUM_CONTRACT_VERSION,
            stimulus,
            question_increment,
            activation_digest,
            state: PdContinuumState::SpineActivated,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.contract_version == PD_CONTINUUM_CONTRACT_VERSION
            && self.stimulus.verifies()
            && self.question_increment.verifies(&self.stimulus)
            && self.activation_digest == digest_parts(&[
                &self.contract_version.to_le_bytes(),
                &self.stimulus.stimulus_digest.to_le_bytes(),
                &self.question_increment.increment_digest.to_le_bytes(),
                b"PD_CONTINUUM_ACTIVATION",
            ])
            && self.state == PdContinuumState::SpineActivated
    }
}

pub fn derive_pd_output_digest(
    activation_digest: u64,
    pd_binding_digest: u64,
    question_increment_digest: u64,
    verification_receipt_id: &str,
    output_text: &str,
    y_verdict: u8,
    x_verdict: u8,
    basis: PdContinuumBasis,
) -> u64 {
    digest_parts(&[
        &PD_CONTINUUM_CONTRACT_VERSION.to_le_bytes(),
        &activation_digest.to_le_bytes(),
        &pd_binding_digest.to_le_bytes(),
        &question_increment_digest.to_le_bytes(),
        verification_receipt_id.as_bytes(),
        output_text.as_bytes(),
        &[y_verdict],
        &[x_verdict],
        basis.label().as_bytes(),
        b"PD_VERIFIED_OUTPUT",
    ])
}

pub fn pd_output_id(output_digest: u64) -> String {
    format!("pdo-{output_digest:016x}")
}

pub fn derive_iz_digest(
    activation_digest: u64,
    question_increment_digest: u64,
    pd_output_digest: u64,
    living_trust_digest: u64,
    verification_receipt_id: &str,
    basis: PdContinuumBasis,
) -> u64 {
    digest_parts(&[
        &PD_CONTINUUM_CONTRACT_VERSION.to_le_bytes(),
        &activation_digest.to_le_bytes(),
        &question_increment_digest.to_le_bytes(),
        &pd_output_digest.to_le_bytes(),
        &living_trust_digest.to_le_bytes(),
        verification_receipt_id.as_bytes(),
        basis.label().as_bytes(),
        b"PD_COMPLETES_IZ_WITH_LIVING_TRUST_V161",
    ])
}

pub fn iz_id(iz_digest: u64) -> String {
    format!("iZ-{iz_digest:016x}")
}

/// Formula finale e deklaruar nga GCL: `PD output + iZ → next i₀`.
pub fn derive_next_i0_id(pd_output_digest: u64, iz_digest: u64) -> String {
    let next_digest = digest_parts(&[
        &PD_CONTINUUM_CONTRACT_VERSION.to_le_bytes(),
        &pd_output_digest.to_le_bytes(),
        &iz_digest.to_le_bytes(),
        b"PD_OUTPUT_PLUS_IZ_TO_NEXT_I0",
    ]);
    format!("i0-{next_digest:016x}")
}

pub fn derive_completion_digest(
    activation_digest: u64,
    question_increment_digest: u64,
    pd_output_digest: u64,
    iz_digest: u64,
    living_trust_digest: u64,
    next_i0_id: &str,
    verification_receipt_id: &str,
    basis: PdContinuumBasis,
) -> u64 {
    digest_parts(&[
        &PD_CONTINUUM_CONTRACT_VERSION.to_le_bytes(),
        &activation_digest.to_le_bytes(),
        &question_increment_digest.to_le_bytes(),
        &pd_output_digest.to_le_bytes(),
        &iz_digest.to_le_bytes(),
        &living_trust_digest.to_le_bytes(),
        next_i0_id.as_bytes(),
        verification_receipt_id.as_bytes(),
        basis.label().as_bytes(),
        b"PD_CONTINUUM_COMPLETE",
    ])
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PdContinuumCompletion {
    pub contract_version: u32,
    pub activation_digest: u64,
    pub question_increment_digest: u64,
    pub pd_output_digest: u64,
    pub iz_digest: u64,
    pub living_trust_digest: u64,
    pub next_i0_id: String,
    pub verification_receipt_digest: u64,
    pub basis: PdContinuumBasis,
    pub output_state: PdContinuumState,
    pub iz_state: PdContinuumState,
    pub completion_digest: u64,
    pub state: PdContinuumState,
}

impl PdContinuumCompletion {
    pub fn new(
        activation_digest: u64,
        question_increment_digest: u64,
        pd_output_digest: u64,
        iz_digest: u64,
        living_trust_digest: u64,
        next_i0_id: String,
        verification_receipt_id: &str,
        basis: PdContinuumBasis,
    ) -> Self {
        let verification_receipt_digest = digest_parts(&[
            verification_receipt_id.as_bytes(),
        ]);
        let completion_digest = derive_completion_digest(
            activation_digest,
            question_increment_digest,
            pd_output_digest,
            iz_digest,
            living_trust_digest,
            &next_i0_id,
            verification_receipt_id,
            basis,
        );
        Self {
            contract_version: PD_CONTINUUM_CONTRACT_VERSION,
            activation_digest,
            question_increment_digest,
            pd_output_digest,
            iz_digest,
            living_trust_digest,
            next_i0_id,
            verification_receipt_digest,
            basis,
            output_state: PdContinuumState::VerifiedOutput,
            iz_state: PdContinuumState::IzCompleted,
            completion_digest,
            state: PdContinuumState::NextI0Derived,
        }
    }

    pub fn closes_formula(&self, verification_receipt_id: &str) -> bool {
        self.contract_version == PD_CONTINUUM_CONTRACT_VERSION
            && self.activation_digest != 0
            && self.question_increment_digest != 0
            && self.pd_output_digest != 0
            && self.iz_digest != 0
            && self.living_trust_digest != 0
            && self.next_i0_id == derive_next_i0_id(self.pd_output_digest, self.iz_digest)
            && self.verification_receipt_digest == digest_parts(&[
                verification_receipt_id.as_bytes(),
            ])
            && self.output_state == PdContinuumState::VerifiedOutput
            && self.iz_state == PdContinuumState::IzCompleted
            && self.completion_digest == derive_completion_digest(
                self.activation_digest,
                self.question_increment_digest,
                self.pd_output_digest,
                self.iz_digest,
                self.living_trust_digest,
                &self.next_i0_id,
                verification_receipt_id,
                self.basis,
            )
            && self.state == PdContinuumState::NextI0Derived
    }
}

pub fn digest_parts(parts: &[&[u8]]) -> u64 {
    parts.iter().fold(0xcbf29ce484222325u64, |acc, part| {
        part.iter().fold(acc, |hash, byte| {
            (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3)
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn i_plus_user_input_binds_existing_i0() {
        let s = PdI0Stimulus::new("session", "i0-A", "input real", 10);
        assert!(s.verifies());
    }

    #[test]
    fn exactly_one_question_activates_pd_continuum() {
        let a = PdContinuumActivation::new(
            "session", "i0-A", "input real", "Cila është prova?", "CLARIFY", 10,
        );
        assert!(a.is_ready());
        assert_eq!(a.question_increment.unit_count, 1);
    }

    #[test]
    fn output_plus_iz_derives_next_i0() {
        let activation = 11;
        let question = 12;
        let output = derive_pd_output_digest(
            activation, 13, question, "receipt", "output text", 1, 1,
            PdContinuumBasis::VerifiedPositive,
        );
        let trust = 99;
        let iz = derive_iz_digest(
            activation, question, output, trust, "receipt",
            PdContinuumBasis::VerifiedPositive,
        );
        let next = derive_next_i0_id(output, iz);
        let completion = PdContinuumCompletion::new(
            activation, question, output, iz, trust, next, "receipt",
            PdContinuumBasis::VerifiedPositive,
        );
        assert!(completion.closes_formula("receipt"));
    }

    #[test]
    fn changing_verified_output_text_changes_iz_and_next_i0() {
        let basis = PdContinuumBasis::VerifiedPositive;
        let a_output = derive_pd_output_digest(
            11, 13, 12, "receipt", "output A", 1, 1, basis,
        );
        let b_output = derive_pd_output_digest(
            11, 13, 12, "receipt", "output B", 1, 1, basis,
        );
        let a_iz = derive_iz_digest(11, 12, a_output, 99, "receipt", basis);
        let b_iz = derive_iz_digest(11, 12, b_output, 99, "receipt", basis);
        assert_ne!(a_output, b_output);
        assert_ne!(a_iz, b_iz);
        assert_ne!(
            derive_next_i0_id(a_output, a_iz),
            derive_next_i0_id(b_output, b_iz),
        );
    }
}
