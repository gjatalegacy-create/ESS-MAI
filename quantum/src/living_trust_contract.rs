// ╔══════════════════════════════════════════════════════════════════════════╗
// ║ LIVING_TRUST_CONTRACT.RS — GCL VULA E GJALLË E BESIMIT v1.6.4          ║
// ║                                                                          ║
// ║ Faza 1 prodhon vetëm `action_state` të lehtë. Ky modul nuk hash-on       ║
// ║ modulet veçmas. Faza 2 kryen NJË SHA-256 mbi gjendjen e konverguar,      ║
// ║ verdiktin suprem, ligjet aktive dhe provën L-500.                        ║
// ║                                                                          ║
// ║ Skedari mbahet byte-identik në Light, Quantum dhe Shadow.                ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use sha2::{Digest, Sha256};

pub const LIVING_TRUST_CONTRACT_VERSION: u32 = 0x0001_0603;
pub const TRUST_INTENSITY_SCALE: u32 = 10_000;
pub const TRUST_KIND_NONE: u8 = 0;
pub const TRUST_KIND_CONSTRUCTIVE: u8 = 1;
pub const TRUST_KIND_RIGOROUS_NEGATIVE: u8 = 2;

pub const SOVEREIGN_SEAL_XOR: u32 = 0xA5A5;
pub const SOVEREIGN_SEAL_MASK: u32 = 0x0000_FFFF;
pub const SOVEREIGN_SEAL_PRIMITIVE: u32 = 500;
pub const LEGACY_BITS_MASK: u32 = 0x54; // LEGACY_PATH | NEGATIVE | ECO
pub const REQUIRED_ACTION_MASK: u64 = 0x0000_0000_0000_03FE;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LivingTrustProof {
    pub action_state: u64,
    pub action_mask: u64,
    pub required_action_mask: u64,
    pub verified: u8,
    pub primitive: u8,
    pub knowledge_band: u8,
    pub lgc_law: u32,
    pub system_laws_seal: u64,
    /// Zero për ciklet normale; SHA-256 i plotë i evidencës së projektit kur
    /// rrjedha GCL procedon një projekt shkencor të përdoruesit.
    pub scientific_project_sha256: [u8; 32],
    /// SHA-256 kanonik i statusit/TRL/rejection të projektit.
    pub scientific_project_verdict_sha256: [u8; 32],
    pub sovereign_flags: u32,
}

impl LivingTrustProof {
    pub fn legacy_bits(self) -> u32 {
        self.lgc_law & LEGACY_BITS_MASK
    }

    pub fn sovereign_value(self) -> u32 {
        (self.sovereign_flags & SOVEREIGN_SEAL_MASK) ^ SOVEREIGN_SEAL_XOR
    }

    pub fn constitutional_kind(self) -> u8 {
        match (self.verified, self.primitive) {
            (1, 1) => TRUST_KIND_CONSTRUCTIVE,
            (0, 0) => TRUST_KIND_RIGOROUS_NEGATIVE,
            _ => TRUST_KIND_NONE,
        }
    }

    pub fn is_admissible(self) -> bool {
        self.action_state != 0
            && self.required_action_mask == REQUIRED_ACTION_MASK
            && self.action_mask == self.required_action_mask
            && self.system_laws_seal != 0
            && self.sovereign_flags != 0
            && self.sovereign_value() == SOVEREIGN_SEAL_PRIMITIVE
            && self.verified <= 1
            && self.primitive <= 1
            && self.constitutional_kind() != TRUST_KIND_NONE
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LivingTrustSeal {
    pub identity_sha256: [u8; 32],
    pub intensity: u32,
    pub kind: u8,
    pub sovereign_value: u32,
}

impl LivingTrustSeal {
    pub fn identity_hex(self) -> String {
        hex_32(&self.identity_sha256)
    }

    pub fn identity_digest(self) -> u64 {
        digest_parts(&[
            self.identity_sha256.as_slice(),
            &self.intensity.to_le_bytes(),
            &[self.kind],
            &self.sovereign_value.to_le_bytes(),
            b"GCL_LIVING_TRUST_TO_IZ_V164",
        ])
    }

    pub fn verifies(self, proof: LivingTrustProof, legacy_score: f32) -> bool {
        match compute(proof, legacy_score) {
            Some(expected) => expected == self,
            None => false,
        }
    }
}

pub fn intensity_from_legacy_score(legacy_score: f32) -> Option<u32> {
    match legacy_score.is_finite() {
        true => Some(
            (legacy_score.clamp(0.0, 1.0) * TRUST_INTENSITY_SCALE as f32)
                .round() as u32,
        ),
        false => None,
    }
}

/// Pulsi i vetëm SHA-256. Nuk ruan state dhe nuk kryen I/O.
pub fn compute(proof: LivingTrustProof, legacy_score: f32) -> Option<LivingTrustSeal> {
    let intensity = intensity_from_legacy_score(legacy_score)?;
    compute_with_intensity(proof, intensity)
}

/// Riverifikimi ndër-platformik përdor intensitetin fixed-point të vulosur.
/// Kjo shmang rikonstruksionin me float në kufirin Quantum → Light, ndërsa
/// materiali kanonik i SHA-256 mbetet byte-identik në të tria platformat.
pub fn compute_with_intensity(
    proof: LivingTrustProof,
    intensity: u32,
) -> Option<LivingTrustSeal> {
    match proof.is_admissible() && intensity <= TRUST_INTENSITY_SCALE {
        true => {}
        false => return None,
    }
    let kind = proof.constitutional_kind();
    let sovereign_value = proof.sovereign_value();

    let mut material = Vec::with_capacity(128);
    material.extend_from_slice(b"GCL_LIVING_TRUST_V164");
    material.extend_from_slice(&LIVING_TRUST_CONTRACT_VERSION.to_le_bytes());
    material.extend_from_slice(&proof.action_state.to_le_bytes());
    material.extend_from_slice(&proof.action_mask.to_le_bytes());
    material.extend_from_slice(&proof.required_action_mask.to_le_bytes());
    material.push(proof.verified);
    material.push(proof.primitive);
    material.push(proof.knowledge_band);
    material.extend_from_slice(&proof.lgc_law.to_le_bytes());
    material.extend_from_slice(&proof.legacy_bits().to_le_bytes());
    material.extend_from_slice(&proof.system_laws_seal.to_le_bytes());
    material.extend_from_slice(&proof.scientific_project_sha256);
    material.extend_from_slice(&proof.scientific_project_verdict_sha256);
    material.extend_from_slice(&proof.sovereign_flags.to_le_bytes());
    material.extend_from_slice(&sovereign_value.to_le_bytes());

    // Identiteti 32-byte ndjek formulën kushtetuese të paradigmës:
    // action_state + verdict + ligjet aktive + L-500. Intensiteti dhe lloji
    // janë forca/klasa e Besimit pranë identitetit; lidhen më pas te
    // VerificationReceipt, iZ dhe next_i0 përmes `identity_digest()`.
    let digest = Sha256::digest(&material);
    let mut identity_sha256 = [0u8; 32];
    identity_sha256.copy_from_slice(&digest);
    Some(LivingTrustSeal {
        identity_sha256,
        intensity,
        kind,
        sovereign_value,
    })
}

pub fn parse_hex_32(value: &str) -> Option<[u8; 32]> {
    match value.len() == 64 && value.as_bytes().iter().all(|byte| byte.is_ascii_hexdigit()) {
        true => {}
        false => return None,
    }
    let mut out = [0u8; 32];
    for (index, slot) in out.iter_mut().enumerate() {
        let start = index * 2;
        *slot = u8::from_str_radix(&value[start..start + 2], 16).ok()?;
    }
    Some(out)
}

pub fn hex_32(value: &[u8; 32]) -> String {
    let mut out = String::with_capacity(64);
    for byte in value {
        use std::fmt::Write as _;
        let _ = write!(&mut out, "{byte:02x}");
    }
    out
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

    fn proof(verified: u8, primitive: u8) -> LivingTrustProof {
        LivingTrustProof {
            action_state: 0x1122_3344_5566_7788,
            action_mask: REQUIRED_ACTION_MASK,
            required_action_mask: REQUIRED_ACTION_MASK,
            verified,
            primitive,
            knowledge_band: 2,
            lgc_law: 0x47,
            system_laws_seal: 0x8877_6655_4433_2211,
            scientific_project_sha256: [0u8; 32],
            scientific_project_verdict_sha256: [0u8; 32],
            sovereign_flags: 0xA451,
        }
    }

    #[test]
    fn constructive_and_rigorous_negative_are_constitutional() {
        assert_eq!(compute(proof(1, 1), 0.75).unwrap().kind, TRUST_KIND_CONSTRUCTIVE);
        assert_eq!(compute(proof(0, 0), 0.75).unwrap().kind, TRUST_KIND_RIGOROUS_NEGATIVE);
        assert!(compute(proof(1, 0), 0.75).is_none());
    }

    #[test]
    fn l500_and_complete_action_convergence_are_mandatory() {
        let mut p = proof(1, 1);
        p.sovereign_flags = 0;
        assert!(compute(p, 0.5).is_none());
        p = proof(1, 1);
        p.action_state = 0;
        assert!(compute(p, 0.5).is_none());
        p = proof(1, 1);
        p.action_mask &= !0x10;
        assert!(compute(p, 0.5).is_none());
    }

    #[test]
    fn scientific_project_is_bound_to_living_trust() {
        let base = compute(proof(1, 1), 0.75).unwrap();
        let mut project = proof(1, 1);
        project.scientific_project_sha256 = [0x11u8; 32];
        project.scientific_project_verdict_sha256 = [0x88u8; 32];
        let with_project = compute(project, 0.75).unwrap();
        assert_ne!(base.identity_sha256, with_project.identity_sha256);
    }

    #[test]
    fn same_whole_system_produces_same_sha256() {
        let a = compute(proof(1, 1), 0.75).unwrap();
        let b = compute(proof(1, 1), 0.75).unwrap();
        assert_eq!(a, b);
        assert_eq!(compute_with_intensity(proof(1, 1), a.intensity), Some(a));
        let stronger = compute_with_intensity(proof(1, 1), 9_000).unwrap();
        assert_eq!(stronger.identity_sha256, a.identity_sha256);
        assert_ne!(stronger.identity_digest(), a.identity_digest());
        assert_eq!(parse_hex_32(&a.identity_hex()), Some(a.identity_sha256));
    }
}
