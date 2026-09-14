//! Compilation shell for the byte-identical v189 sovereign FFI gate.
//!
//! `sovereign_ffi_gate.rs`, `living_trust_contract.rs`, and
//! `verification_receipt.rs` are copied byte-for-byte from v189. The small
//! modules below satisfy non-focal type and lineage dependencies so the gate
//! can be compiled and attacked outside the ESS-MAI workspace. They also
//! expose the minimum shapes required when Cargo's `--all-targets` compiles the
//! upstream file's `cfg(test)` module. They carry no persistence authority.

extern crate self as shadow_contracts;

pub const PROJECT_STATUS_NONE: u8 = 0;

pub mod types {
    #[derive(Debug, Clone, Copy, PartialEq)]
    #[repr(u8)]
    pub enum KnowledgeBand {
        Negative = 0,
        Hypothesis = 1,
        Fact = 2,
        Scientific = 3,
    }

    impl KnowledgeBand {
        pub fn as_u8(self) -> u8 {
            self as u8
        }
    }

    #[derive(Debug, Clone)]
    pub struct SupremeVerdict {
        pub verified: u8,
        pub primitive: u8,
        pub knowledge_band: KnowledgeBand,
        pub lgc_law: u32,
        pub legacy_score: f32,
        pub lgc_dsl: String,
        pub living_trust_sha256: [u8; 32],
        pub living_trust_intensity: u32,
        pub living_trust_kind: u8,
        pub living_trust_action_state: u64,
        pub living_trust_action_mask: u64,
        pub living_trust_required_action_mask: u64,
        pub living_trust_system_laws_seal: u64,
        pub living_trust_sovereign_flags: u32,
        pub living_trust_sovereign_value: u32,
        pub project_id: u64,
        pub project_status: u8,
        pub project_context_digest: u64,
        pub project_evidence_digest: u64,
        pub project_context_sha256: [u8; 32],
        pub project_evidence_sha256: [u8; 32],
        pub novel_factualized: bool,
        pub novel_trl_level: u8,
        pub novel_proof_score: u32,
        pub novel_rejection: u8,
    }

    #[derive(Debug, Clone)]
    pub struct LawConfirmedVerdict {
        verdict: SupremeVerdict,
    }

    impl LawConfirmedVerdict {
        pub fn confirm(verdict: SupremeVerdict, negative_proof_present: bool) -> Result<Self, &'static str> {
            match (verdict.verified, verdict.primitive, negative_proof_present) {
                (1, 1, _) | (0, 0, true) => Ok(Self { verdict }),
                (0, 0, false) => Err("missing negative proof"),
                _ => Err("non-constitutional verdict pair"),
            }
        }

        pub fn from_closed_verdict(verdict: SupremeVerdict) -> Self {
            Self { verdict }
        }

        pub fn as_supreme(&self) -> &SupremeVerdict {
            &self.verdict
        }
    }
}

pub mod living_trust_contract;
pub mod verification_receipt;

pub mod lab_contracts {
    pub mod gjata_collapse_law {
        /// Non-zero compilation-shell value used only by the upstream unit-test
        /// fixture. The production GCL registry is not reproduced in this POC.
        pub fn system_laws_seal() -> u64 {
            0x8877_6655_4433_2211
        }
    }

    pub mod verification_receipt {
        pub use crate::verification_receipt::*;
    }
}

pub mod seal_registry {
    const fn fnv(bytes: &[u8]) -> u64 {
        let mut hash = 14695981039346656037u64;
        let mut index = 0usize;
        while index < bytes.len() {
            hash ^= bytes[index] as u64;
            hash = hash.wrapping_mul(1099511628211);
            index += 1;
        }
        hash
    }

    pub const fn module_seal(module: &[u8]) -> u64 {
        fnv(module) ^ fnv(b"GJATA_LEGACY")
    }
}

pub mod knowledge_lineage {
    const FNV_OFFSET: u64 = 14695981039346656037;

    const fn fnv1a(mut seed: u64, bytes: &[u8]) -> u64 {
        let mut index = 0;
        while index < bytes.len() {
            seed ^= bytes[index] as u64;
            seed = seed.wrapping_mul(1099511628211);
            index += 1;
        }
        seed
    }

    fn mix_u64(seed: u64, value: u64) -> u64 {
        fnv1a(seed, &value.to_le_bytes())
    }

    /// Adapter preserving the v189 capability-lineage rule used by the focal gate.
    pub struct Lineage {
        proof_len: u32,
        derivation: u64,
        cap_sealed: bool,
    }

    impl Lineage {
        pub fn derive_capability(input_id: u64, proof_chain: &[u64]) -> Self {
            let mut digest = mix_u64(FNV_OFFSET, input_id);
            digest = mix_u64(digest, 0xCA9A_B111_7E57_5EA1);
            for item in proof_chain {
                digest = mix_u64(digest, *item);
            }
            Self {
                proof_len: proof_chain.len() as u32,
                derivation: digest,
                cap_sealed: true,
            }
        }

        pub fn is_traceable(&self) -> bool {
            self.derivation != 0 && self.proof_len > 0 && self.cap_sealed
        }
    }
}

#[path = "sovereign_ffi_gate.rs"]
pub mod sovereign_ffi_gate;
