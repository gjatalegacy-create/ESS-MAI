// ╔══════════════════════════════════════════════════════════════════════════╗
// ║ TOKEN_FORGE — DËSHMITARI I DYTË SOVRAN SHA-256 (Quantum, v1.6.2)      ║
// ║ GJATA LEGACY™ | Arkitekt: Bledar Gjata                                  ║
// ║                                                                          ║
// ║ Burimi 1: SovereignGate (MINT+BURN, anti-replay, gjeneratë).            ║
// ║ Burimi 2: kjo farkë e pavarur. Token-i është SHA-256 real mbi domenin,  ║
// ║ rendin monoton dhe kohën reale. Koha e mban token-in VETËM dëshmitar    ║
// ║ runtime; ai nuk hyn në scoring, collapse ose verdikt determinist.       ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::lab_contracts::gcl_presume::GclReadiness;
use sha2::{Digest, Sha256};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};

pub type ForgeToken = [u8; 32];
pub const TOKEN_FORGE_VERSION: u32 = 0x0001_0602;

/// Numëruesi monoton i farkës — rendi i derdhjeve brenda procesit.
static DERDHJET: AtomicU64 = AtomicU64::new(0);
static I_FUNDIT: OnceLock<Mutex<ForgeToken>> = OnceLock::new();

fn last_slot() -> &'static Mutex<ForgeToken> {
    I_FUNDIT.get_or_init(|| Mutex::new([0u8; 32]))
}

fn tani_ns() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0)
}

/// Derdh një dëshmitar SHA-256 nga domain × rend × kohë.
/// Token-i është jo-determinist me qëllim dhe nuk lejohet në reasoning.
pub fn mint(domain: u64) -> ForgeToken {
    let rendi = DERDHJET.fetch_add(1, Ordering::SeqCst) + 1;
    let koha = tani_ns();
    let mut material = Vec::with_capacity(64);
    material.extend_from_slice(b"ESS_MAI_TOKEN_FORGE_SHA256_V162");
    material.extend_from_slice(&TOKEN_FORGE_VERSION.to_le_bytes());
    material.extend_from_slice(&domain.to_le_bytes());
    material.extend_from_slice(&rendi.to_le_bytes());
    material.extend_from_slice(&koha.to_le_bytes());
    let digest = Sha256::digest(&material);
    let mut token = [0u8; 32];
    token.copy_from_slice(&digest);
    match token.iter().all(|byte| *byte == 0) {
        true => token[31] = 1,
        false => {}
    }
    match last_slot().lock() {
        Ok(mut slot) => *slot = token,
        Err(poisoned) => *poisoned.into_inner() = token,
    }
    token
}

pub fn minted_count() -> u64 { DERDHJET.load(Ordering::SeqCst) }

pub fn last_token() -> ForgeToken {
    match last_slot().lock() {
        Ok(slot) => *slot,
        Err(poisoned) => *poisoned.into_inner(),
    }
}

pub fn token_hex(token: &ForgeToken) -> String {
    let mut out = String::with_capacity(64);
    for byte in token {
        use std::fmt::Write as _;
        let _ = write!(&mut out, "{byte:02x}");
    }
    out
}

pub fn token_words(token: &ForgeToken) -> [u64; 4] {
    let mut words = [0u64; 4];
    for (index, slot) in words.iter_mut().enumerate() {
        let start = index * 8;
        *slot = u64::from_le_bytes([
            token[start], token[start + 1], token[start + 2], token[start + 3],
            token[start + 4], token[start + 5], token[start + 6], token[start + 7],
        ]);
    }
    words
}

pub fn status() -> GclReadiness {
    match minted_count() {
        0 => GclReadiness::Degraded("farka SHA-256 s'ka derdhur ende token në këtë proces"),
        _ => GclReadiness::Ready,
    }
}

#[cfg(test)]
mod token_forge_tests {
    use super::*;
    use std::sync::{Mutex as TestMutex, MutexGuard};

    static TEST_LOCK: TestMutex<()> = TestMutex::new(());

    fn test_guard() -> MutexGuard<'static, ()> {
        match TEST_LOCK.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }

    #[test]
    fn two_mints_always_differ_and_count_grows() {
        let _guard = test_guard();
        let c0 = minted_count();
        let t1 = mint(0xAA);
        let t2 = mint(0xAA);
        assert_ne!(t1, t2, "rendi monoton garanton ndryshim");
        assert_ne!(t1, [0u8; 32]);
        assert_ne!(t2, [0u8; 32]);
        assert_eq!(minted_count(), c0 + 2);
        assert_eq!(last_token(), t2);
        assert_eq!(token_hex(&t2).len(), 64);
    }

    #[test]
    fn domain_separation_produces_distinct_tokens() {
        let _guard = test_guard();
        let a = mint(0x4C49_4748_5400_0001);
        let b = mint(0x5348_4144_4F57_0001);
        assert_ne!(a, b, "domene të ndryshme → token të ndryshëm");
    }

    #[test]
    fn status_flips_ready_after_first_mint() {
        let _guard = test_guard();
        let _ = mint(0x54_45_53_54);
        assert!(status().is_ready(), "pas derdhjes farka është READY");
    }
}
