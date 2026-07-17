// ╔══════════════════════════════════════════════════════════════════════════╗
// ║ VERIFICATION_RECEIPT — KONTRATA KANONIKE E MBYLLJES XY                ║
// ║ v1.6.2: receipt-i është SHA-256 real dhe lidh kandidatin PD/Spine 9,  ║
// ║ Vulën e Gjallë dhe token-in sovran të konsumuar nga Shadow.            ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use sha2::{Digest, Sha256};

pub const VERIFICATION_SEAL_NAME: &[u8] = b"SHADOW_VERIFICATION_OUTPUT";
pub const VERIFICATION_RECEIPT_VERSION: u32 = 0x0001_0602;

/// FNV mbetet vetëm si digest jo-sigurie për gjurmë të vjetra/XY.
/// Identiteti i receipt-it NUK përdor më këtë funksion.
pub fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in bytes.iter() {
        h ^= *byte as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

pub fn constitutional_pair(y: u8, x: u8) -> bool {
    matches!((y, x), (1, 1) | (0, 0))
}

fn hex_32(value: &[u8; 32]) -> String {
    let mut out = String::with_capacity(64);
    for byte in value {
        use std::fmt::Write as _;
        let _ = write!(&mut out, "{byte:02x}");
    }
    out
}

#[allow(clippy::too_many_arguments)]
pub fn receipt_id(
    session_id:        &str,
    parent_i0:         &str,
    primitive_anchor:  u64,
    xy_digest:         u64,
    pd_binding_digest: u64,
    pd_continuum_activation_digest: u64,
    living_trust_sha256: &[u8; 32],
    y_verdict:         u8,
    x_verdict:         u8,
    generation:        u64,
    seal:              u64,
) -> String {
    let mut proof = Vec::with_capacity(session_id.len() + parent_i0.len() + 112);
    proof.extend_from_slice(b"ESS_MAI_VERIFICATION_RECEIPT_SHA256_V162");
    proof.extend_from_slice(&VERIFICATION_RECEIPT_VERSION.to_le_bytes());
    proof.extend_from_slice(&(session_id.len() as u64).to_le_bytes());
    proof.extend_from_slice(session_id.as_bytes());
    proof.extend_from_slice(&(parent_i0.len() as u64).to_le_bytes());
    proof.extend_from_slice(parent_i0.as_bytes());
    proof.extend_from_slice(&primitive_anchor.to_le_bytes());
    proof.extend_from_slice(&xy_digest.to_le_bytes());
    proof.extend_from_slice(&pd_binding_digest.to_le_bytes());
    proof.extend_from_slice(&pd_continuum_activation_digest.to_le_bytes());
    proof.extend_from_slice(living_trust_sha256);
    proof.push(y_verdict);
    proof.push(x_verdict);
    proof.extend_from_slice(&generation.to_le_bytes());
    proof.extend_from_slice(&seal.to_le_bytes());
    let digest = Sha256::digest(&proof);
    let mut identity = [0u8; 32];
    identity.copy_from_slice(&digest);
    hex_32(&identity)
}

pub fn is_canonical_receipt_id(value: &str) -> bool {
    value.len() == 64 && value.as_bytes().iter().all(|byte| byte.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn receipt_is_deterministic_and_bound_to_parent_and_pd_candidate() {
        let a = receipt_id("s", "i0-a", 7, 9, 13, 17, &[19; 32], 1, 1, 3, 11);
        let b = receipt_id("s", "i0-a", 7, 9, 13, 17, &[19; 32], 1, 1, 3, 11);
        let other_parent = receipt_id("s", "i0-b", 7, 9, 13, 17, &[19; 32], 1, 1, 3, 11);
        let other_pd = receipt_id("s", "i0-a", 7, 9, 14, 17, &[19; 32], 1, 1, 3, 11);
        let other_continuum = receipt_id("s", "i0-a", 7, 9, 13, 18, &[19; 32], 1, 1, 3, 11);
        let other_trust = receipt_id("s", "i0-a", 7, 9, 13, 17, &[20; 32], 1, 1, 3, 11);
        assert_eq!(a, b);
        assert_eq!(a.len(), 64);
        assert!(is_canonical_receipt_id(&a));
        assert_ne!(a, other_parent);
        assert_ne!(a, other_pd);
        assert_ne!(a, other_continuum);
        assert_ne!(a, other_trust);
    }

    #[test]
    fn only_gcl_pairs_are_constitutional() {
        assert!(constitutional_pair(1, 1));
        assert!(constitutional_pair(0, 0));
        assert!(!constitutional_pair(1, 0));
        assert!(!constitutional_pair(0, 1));
    }
}
