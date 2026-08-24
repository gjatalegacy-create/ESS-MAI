//! Two production hash functions extracted without the private LGC secret or registry.

use sha2::{Digest, Sha256};

pub fn is_canonical_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

/// SHA-256 me domain separation dhe length-prefix për çdo pjesë.
pub fn compute_domain_sha(domain: &str, parts: &[&[u8]]) -> String {
    let mut h = Sha256::new();
    h.update(b"ESSMAI-DOMAIN-SHA256-V1\0");
    h.update((domain.len() as u64).to_le_bytes());
    h.update(domain.as_bytes());
    for part in parts {
        h.update((part.len() as u64).to_le_bytes());
        h.update(part);
    }
    h.finalize().iter().map(|b| format!("{:02x}", b)).collect()
}

