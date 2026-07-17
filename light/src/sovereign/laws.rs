// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  SOVEREIGN/LAWS.RS — Ligjet Matematike Invariante (0-COPY)            ║
// ║  GJATA LEGACY™ — Light Platform                                   ║
// ║                                                                          ║
// ║  Adaptim i ZERO-COPY SOVEREIGN HANDSHAKE (ligji që dërgoi arkitekti).  ║
// ║  IDENTIK me Quantum sovereign — të njëjtat konstante, vula 500.        ║
// ║                                                                          ║
// ║  LIGJET (invariante ndaj kompleksitetit — formula s'ndryshon):       ║
// ║   • gate  = clamp((x−lo)/(hi−lo))   → O(1), zero degëzim             ║
// ║   • dot8  = Σ(axisᵢ × wᵢ)           → produkt skalar linear          ║
// ║   • admit = c₁×c₂×c₃×c₄             → produkt binar, s'thyhet        ║
// ║   • fnv   = FNV-1a hash              → seal compile-time              ║
// ║                                                                          ║
// ║  VULA 500: (flags & 0xFFFF) ^ 0xA5A5 == 500 — identike ndër platforma. ║
// ╚══════════════════════════════════════════════════════════════════════════╝

// ─────────────────────────────────────────────────────────────────────────────
// §1  GATE — clamp((x−lo)/(hi−lo)) — zero if/else
// ─────────────────────────────────────────────────────────────────────────────

/// Gate matematike. O(1) gjithmonë, pa degëzim.
#[inline(always)]
pub fn gate(x: f32, lo: f32, hi: f32) -> f32 {
    let raw = (x - lo) / (hi - lo + 1e-9);
    raw.clamp(0.0, 1.0)
}

// ─────────────────────────────────────────────────────────────────────────────
// §2  DOT8 — produkt skalar i peshuar mbi 8 akse
// ─────────────────────────────────────────────────────────────────────────────

/// Produkt skalar i peshuar mbi 8 akse. SIMD-friendly, zero branches.
#[inline(always)]
pub fn dot8(axes: &[f32; 8], weights: &[f32; 8]) -> f32 {
    let mut acc = 0f32;
    let mut i = 0usize;
    while i < 8 {
        acc += axes[i] * weights[i];
        i += 1;
    }
    acc
}

// ─────────────────────────────────────────────────────────────────────────────
// §3  ADMIT4 — produkt binar: admit = c₁ × c₂ × c₃ × c₄
// ─────────────────────────────────────────────────────────────────────────────

/// Produkt binar i 4 kushteve. bit ∈ {0,1}. Asnjë `if`.
#[inline(always)]
pub fn admit4(c1: bool, c2: bool, c3: bool, c4: bool) -> u8 {
    (c1 as u8) * (c2 as u8) * (c3 as u8) * (c4 as u8)
}

// ─────────────────────────────────────────────────────────────────────────────
// §4  FNV-1a — seal compile-time
// ─────────────────────────────────────────────────────────────────────────────

/// FNV-1a hash — const fn, llogaritet në kohë kompilimi.
#[inline(always)]
pub const fn fnv(s: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    let mut i = 0usize;
    while i < s.len() {
        hash ^= s[i] as u64;
        hash = hash.wrapping_mul(0x100000001b3);
        i += 1;
    }
    hash
}

/// Suffix-i i përbashkët për vula moduli (identik me Quantum/Shadow).
pub const LGC_SUFFIX: &[u8] = b"GJATA_LEGACY";

/// Vula e modulit = fnv(module) ^ fnv(GJATA_LEGACY).
pub const fn module_seal(module: &[u8]) -> u64 {
    fnv(module) ^ fnv(LGC_SUFFIX)
}

// ─────────────────────────────────────────────────────────────────────────────
// §5  VULAT E MODULEVE TË LIGHT — compile-time
// ─────────────────────────────────────────────────────────────────────────────

pub const SEAL_COORDINATOR:  u64 = module_seal(b"LIGHT_COORDINATOR");
pub const SEAL_APUPK:        u64 = module_seal(b"APUPK_COORDINATOR");
pub const SEAL_SNB:          u64 = module_seal(b"SNB_ALGORITHM");
pub const SEAL_MINI_EVOLVER: u64 = module_seal(b"LIGHT_MINI_EVOLVER");
pub const SEAL_QUANTUM_BUSS: u64 = module_seal(b"QUANTUM_BUSS");
pub const SEAL_SHADOW_BUSS:  u64 = module_seal(b"SHADOW_BUSS");

// ─────────────────────────────────────────────────────────────────────────────
// §6  VULA 500 — invariant ndër platforma (Light ↔ Quantum ↔ Shadow)
// ─────────────────────────────────────────────────────────────────────────────

pub const SEAL_XOR:       u32 = 0xA5A5;
pub const SEAL_MASK:      u32 = 0xFFFF;
pub const SEAL_PRIMITIVE: u32 = 500;
pub const SEAL_MASKED:    u32 = 0xA451;  // 500 ^ 0xA5A5

/// verify_500 — a është flags-i një vulë primitive 500? Zero if.
/// (flags & 0xFFFF) ^ 0xA5A5 == 500 AND flags != 0.
#[inline(always)]
pub fn verify_500(flags: u32) -> bool {
    let cleared = (flags != 0) as u32;
    let value = (flags & SEAL_MASK) ^ SEAL_XOR;
    let matches = (value == SEAL_PRIMITIVE) as u32;
    (cleared * matches) == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gate_clamps() {
        assert_eq!(gate(0.5, 0.0, 1.0), 0.5);
        assert_eq!(gate(2.0, 0.0, 1.0), 1.0);
        assert_eq!(gate(-1.0, 0.0, 1.0), 0.0);
    }

    #[test]
    fn admit4_is_product() {
        assert_eq!(admit4(true, true, true, true), 1);
        assert_eq!(admit4(true, false, true, true), 0);
    }

    #[test]
    fn module_seals_distinct() {
        // Çdo modul ka vulë të ndryshme.
        assert_ne!(SEAL_COORDINATOR, SEAL_APUPK);
        assert_ne!(SEAL_SNB, SEAL_MINI_EVOLVER);
    }

    #[test]
    fn vula_500_invariant() {
        // 500 ^ 0xA5A5 = 0xA451 (masked). Verifiko round-trip.
        assert_eq!(SEAL_PRIMITIVE ^ SEAL_XOR, SEAL_MASKED);
        // flags ku (flags & 0xFFFF) ^ 0xA5A5 == 500.
        let valid_flags = SEAL_MASKED; // 0xA451 ^ 0xA5A5 = 500
        assert!(verify_500(valid_flags));
        // flags i pavlefshëm.
        assert!(!verify_500(0));
        assert!(!verify_500(0x1234));
    }

    #[test]
    fn fnv_deterministic() {
        assert_eq!(fnv(b"test"), fnv(b"test"));
        assert_ne!(fnv(b"a"), fnv(b"b"));
    }

    #[test]
    fn registry_matches_light_laws() {
        // Konsistencë: vulat e Light në regjistër == ato të deklaruara këtu.
        use crate::sovereign::seal_registry::{SealRegistry, L_COORDINATOR, L_APUPK, L_SNB, L_MINI_EVOLVER};
        assert_eq!(SealRegistry::lookup(L_COORDINATOR), Some(SEAL_COORDINATOR));
        assert_eq!(SealRegistry::lookup(L_APUPK), Some(SEAL_APUPK));
        assert_eq!(SealRegistry::lookup(L_SNB), Some(SEAL_SNB));
        assert_eq!(SealRegistry::lookup(L_MINI_EVOLVER), Some(SEAL_MINI_EVOLVER));
    }
}
