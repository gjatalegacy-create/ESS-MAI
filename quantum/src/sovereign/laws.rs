// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  SOVEREIGN/LAWS.RS — Ligjet Matematike Invariante (0-COPY)            ║
// ║  GJATA LEGACY™ — Quantum Platform                                     ║
// ║                                                                          ║
// ║  Adaptim i ZERO-COPY SOVEREIGN HANDSHAKE (ligji që dërgoi arkitekti).  ║
// ║                                                                          ║
// ║  LIGJET (invariante ndaj kompleksitetit — formula s'ndryshon):       ║
// ║   • gate  = clamp((x−lo)/(hi−lo))   → O(1), zero degëzim             ║
// ║   • dot8  = Σ(axisᵢ × wᵢ)           → produkt skalar linear          ║
// ║   • elim  = w × cg × (1−nk)          → eliminim, zero branches        ║
// ║   • admit = c₁×c₂×c₃×c₄             → produkt binar, s'thyhet        ║
// ║   • fnv   = FNV-1a hash              → seal compile-time              ║
// ║                                                                          ║
// ║  Këto NUK janë komente. Janë funksione ekzekutive — ligje.            ║
// ╚══════════════════════════════════════════════════════════════════════════╝

// ─────────────────────────────────────────────────────────────────────────────
// §1  GATE — clamp((x−lo)/(hi−lo)) — zero if/else
// ─────────────────────────────────────────────────────────────────────────────

/// Gate matematike. O(1) gjithmonë, pa degëzim.
/// raw = (x−lo)/(hi−lo+ε); caller-i e clamp-on në [0,1].
#[inline(always)]
pub fn gate(x: f32, lo: f32, hi: f32) -> f32 {
    let raw = (x - lo) / (hi - lo + 1e-9);
    raw.clamp(0.0, 1.0)
}

// ─────────────────────────────────────────────────────────────────────────────
// §2  DOT8 — masa epistemike = Σ(axisᵢ × wᵢ)
//     Invariant: ndaj numrit të akseve, formula është e njëjtë.
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
// §3  ELIM — eliminim: final = w × cg × (1−nk)
//     Invariant: ndaj numrit të path-eve, formula është e njëjtë.
// ─────────────────────────────────────────────────────────────────────────────

/// Formula e eliminimit (nga adaptive_elimination): operator × gate × (1−nk).
/// Tre operacione, zero branches.
#[inline(always)]
pub fn elim(w: f32, cg: f32, nk: f32) -> f32 {
    w * cg * (1.0 - nk)
}

// ─────────────────────────────────────────────────────────────────────────────
// §4  ADMIT4 — produkt binar: admit = c₁ × c₂ × c₃ × c₄
//     Invariant: shto kushte → shto faktorë, formula e njëjtë.
// ─────────────────────────────────────────────────────────────────────────────

/// Produkt binar i 4 kushteve (nga shadow_judiciary). bit ∈ {0,1}.
/// Asnjë `if` — nëse ndonjë kusht është false (0), produkti = 0.
#[inline(always)]
pub fn admit4(c1: bool, c2: bool, c3: bool, c4: bool) -> u8 {
    (c1 as u8) * (c2 as u8) * (c3 as u8) * (c4 as u8)
}

// ─────────────────────────────────────────────────────────────────────────────
// §5  FNV-1a — seal compile-time
// ─────────────────────────────────────────────────────────────────────────────

/// FNV-1a hash — const fn, llogaritet në kohë kompilimi.
#[inline(always)]
pub const fn fnv(s: &[u8]) -> u64 {
    let mut h: u64 = 14695981039346656037;
    let mut i = 0usize;
    while i < s.len() {
        h ^= s[i] as u64;
        h = h.wrapping_mul(1099511628211);
        i += 1;
    }
    h
}

/// Suffix-i i përbashkët i të gjitha seal-eve.
pub const LGC_SUFFIX: &[u8] = b"GJATA_LEGACY";

/// Seal i një moduli — FNV(module) XOR FNV(suffix), compile-time.
pub const fn module_seal(module: &[u8]) -> u64 {
    fnv(module) ^ fnv(LGC_SUFFIX)
}

// Seal-et e moduleve të Quantum — llogariten një herë, compile-time.
pub const SEAL_LIM:  u64 = module_seal(b"LINEAR_INFO_METRIC");
pub const SEAL_PRO:  u64 = module_seal(b"PRO_CORE");
pub const SEAL_SRK:  u64 = module_seal(b"SCIENTIFIC_REASONING_KERNEL");
pub const SEAL_PIM:  u64 = module_seal(b"PASS_INFO_METRIC");
pub const SEAL_PD:   u64 = module_seal(b"PROGRESSIVE_DEBATIC");
pub const SEAL_EBPF: u64 = module_seal(b"EBPF_HYDRATOR");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gate_clamps() {
        assert_eq!(gate(0.5, 0.0, 1.0), 0.5);
        assert_eq!(gate(-1.0, 0.0, 1.0), 0.0);  // nën lo → 0
        assert_eq!(gate(2.0, 0.0, 1.0), 1.0);   // mbi hi → 1
    }

    #[test]
    fn dot8_linear() {
        let axes = [1.0; 8];
        let w = [0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.3];
        let m = dot8(&axes, &w);
        assert!((m - 1.0).abs() < 1e-5); // shuma e peshave
    }

    #[test]
    fn elim_zeroes_on_full_nk() {
        // nk=1.0 → eliminim i plotë → 0
        assert_eq!(elim(0.9, 0.8, 1.0), 0.0);
        // nk=0 → pa eliminim → w*cg
        assert!((elim(0.9, 0.8, 0.0) - 0.72).abs() < 1e-5);
    }

    #[test]
    fn admit4_product() {
        assert_eq!(admit4(true, true, true, true), 1);
        assert_eq!(admit4(true, false, true, true), 0); // një false → 0
    }

    #[test]
    fn seals_are_distinct() {
        // Çdo modul seal i ndryshëm (FNV mbi emra të ndryshëm).
        assert_ne!(SEAL_LIM, SEAL_PRO);
        assert_ne!(SEAL_PRO, SEAL_PIM);
        assert_ne!(SEAL_PIM, SEAL_PD);
        assert_ne!(SEAL_PD, SEAL_EBPF);
        assert_ne!(SEAL_LIM, SEAL_EBPF);
    }

    #[test]
    fn registry_matches_quantum_laws() {
        // Konsistencë: vulat e Quantum në regjistër == ato të deklaruara këtu.
        use crate::sovereign::seal_registry::{SealRegistry, Q_LIM, Q_PRO, Q_SRK, Q_PIM, Q_PD, Q_EBPF};
        assert_eq!(SealRegistry::lookup(Q_LIM), Some(SEAL_LIM));
        assert_eq!(SealRegistry::lookup(Q_PRO), Some(SEAL_PRO));
        assert_eq!(SealRegistry::lookup(Q_SRK), Some(SEAL_SRK));
        assert_eq!(SealRegistry::lookup(Q_PIM), Some(SEAL_PIM));
        assert_eq!(SealRegistry::lookup(Q_PD), Some(SEAL_PD));
        assert_eq!(SealRegistry::lookup(Q_EBPF), Some(SEAL_EBPF));
    }
}
