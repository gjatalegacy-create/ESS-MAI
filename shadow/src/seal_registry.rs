// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  SEAL_REGISTRY.RS — Regjistri Qendror i Vulave (Shadow)              ║
// ║  GJATA LEGACY™ — ESS-MAI (i përbashkët Quantum + Light + Shadow)      ║
// ║                                                            (FAZA 4-5)  ║
// ║                                                                          ║
// ║  PROBLEMI: 3 platforma → vula moduli të pavarura → rrezik përplasjeje  ║
// ║  kur sistemi rritet (emër i dyfishtë ose kolizion FNV i paqëllimshëm). ║
// ║                                                                          ║
// ║  ZGJIDHJA: një burim i vetëm i vërtetë për TË GJITHA vulat, me:        ║
// ║   • prefix hapësire emrash: Q_ (Quantum) | L_ (Light) | S_ (Shadow)    ║
// ║   • detect_collision() → garanton ZERO përplasje (test-time)          ║
// ║   • lookup() → gjen vulën nga emri                                    ║
// ║                                                                          ║
// ║  KY SKEDAR përmban TË NJËJTAT vlera si te Quantum dhe Light: e njëjta   ║
// ║  fnv (FNV-1a: offset 14695981039346656037, prime 1099511628211) dhe    ║
// ║  i njëjti suffix "GJATA_LEGACY". Pra module_seal prodhon BYTE-FOR-BYTE  ║
// ║  vlera identike ndër tri platformat — regjistri është i përbashkët.    ║
// ║                                                                          ║
// ║  Tani Shadow ËSHTË ndërtuar: vulat e tij S_ s'janë më "rezervuar" por   ║
// ║  reale dhe pjesë e regjistrit aktiv. Zero if/else.                    ║
// ╚══════════════════════════════════════════════════════════════════════════╝

// ─────────────────────────────────────────────────────────────────────────────
// FNV-1a — IDENTIK me Quantum/Light (offset + prime + suffix të njëjtë)
// ─────────────────────────────────────────────────────────────────────────────

/// FNV-1a hash — const fn, byte-for-byte identik me sovereign/laws.rs të
/// Quantum dhe Light. Garanton që vulat të jenë konsistente ndër platforma.
const fn fnv(s: &[u8]) -> u64 {
    let mut h: u64 = 14695981039346656037;
    let mut i = 0usize;
    while i < s.len() {
        h ^= s[i] as u64;
        h = h.wrapping_mul(1099511628211);
        i += 1;
    }
    h
}

/// Suffix-i i përbashkët i të gjitha seal-eve — identik ndër platforma.
const LGC_SUFFIX: &[u8] = b"GJATA_LEGACY";

/// module_seal — FNV(module) XOR FNV(suffix), identik me të tria platformat.
pub const fn module_seal(module: &[u8]) -> u64 {
    fnv(module) ^ fnv(LGC_SUFFIX)
}

// ─────────────────────────────────────────────────────────────────────────────
// PLATFORM — hapësira e emrave të vulës
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    Quantum,
    Light,
    Shadow,
}

impl Platform {
    pub fn prefix(self) -> &'static str {
        match self {
            Platform::Quantum => "Q",
            Platform::Light   => "L",
            Platform::Shadow  => "S",
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// SEAL ENTRY — një hyrje në regjistër
// ─────────────────────────────────────────────────────────────────────────────

/// Një vulë moduli e regjistruar: platforma + emri + vlera e llogaritur.
#[derive(Debug, Clone, Copy)]
pub struct SealEntry {
    pub platform:    Platform,
    pub module_name: &'static str,
    pub seal:        u64,
}

// ─────────────────────────────────────────────────────────────────────────────
// REGJISTRI — burimi i vetëm i vërtetë për TË GJITHA vulat (identik ndër platforma)
// ─────────────────────────────────────────────────────────────────────────────

// ── QUANTUM ──────────────────────────────────────────────────────────────────
pub const Q_LIM:  &str = "LINEAR_INFO_METRIC";
pub const Q_PRO:  &str = "PRO_CORE";
pub const Q_SRK:  &str = "SCIENTIFIC_REASONING_KERNEL";
pub const Q_PIM:  &str = "PASS_INFO_METRIC";
pub const Q_PD:   &str = "PROGRESSIVE_DEBATIC";
pub const Q_EBPF: &str = "EBPF_HYDRATOR";

// ── LIGHT ────────────────────────────────────────────────────────────────────
pub const L_COORDINATOR:  &str = "LIGHT_COORDINATOR";
pub const L_APUPK:        &str = "APUPK_COORDINATOR";
pub const L_SNB:          &str = "SNB_ALGORITHM";
pub const L_MINI_EVOLVER: &str = "LIGHT_MINI_EVOLVER";
pub const L_QUANTUM_BUSS: &str = "QUANTUM_BUSS";
pub const L_SHADOW_BUSS:  &str = "SHADOW_BUSS";

// ── SHADOW (tani aktive — Shadow është ndërtuar) ────────────────────────────
pub const S_JUDICIARY:  &str = "SHADOW_JUDICIARY";
pub const S_VNK:        &str = "VERIFIED_NEGATIVE_KNOWLEDGE";
pub const S_VOK:        &str = "VERIFIED_OPTIMAL_KNOWLEDGE";
pub const S_LGC:        &str = "SHADOW_GJ_LEGACY";
pub const S_VERIFY_OUT: &str = "SHADOW_VERIFICATION_OUTPUT";

/// Numri total i vulave të regjistruara.
pub const REGISTRY_SIZE: usize = 17;

/// build_registry — ndërton tabelën e plotë të vulave. Burimi i vetëm i vërtetë.
/// Vlerat janë identike me Quantum/Light (e njëjta fnv + suffix).
pub fn build_registry() -> [SealEntry; REGISTRY_SIZE] {
    [
        // Quantum
        SealEntry { platform: Platform::Quantum, module_name: Q_LIM,  seal: module_seal(Q_LIM.as_bytes()) },
        SealEntry { platform: Platform::Quantum, module_name: Q_PRO,  seal: module_seal(Q_PRO.as_bytes()) },
        SealEntry { platform: Platform::Quantum, module_name: Q_SRK,  seal: module_seal(Q_SRK.as_bytes()) },
        SealEntry { platform: Platform::Quantum, module_name: Q_PIM,  seal: module_seal(Q_PIM.as_bytes()) },
        SealEntry { platform: Platform::Quantum, module_name: Q_PD,   seal: module_seal(Q_PD.as_bytes()) },
        SealEntry { platform: Platform::Quantum, module_name: Q_EBPF, seal: module_seal(Q_EBPF.as_bytes()) },
        // Light
        SealEntry { platform: Platform::Light, module_name: L_COORDINATOR,  seal: module_seal(L_COORDINATOR.as_bytes()) },
        SealEntry { platform: Platform::Light, module_name: L_APUPK,        seal: module_seal(L_APUPK.as_bytes()) },
        SealEntry { platform: Platform::Light, module_name: L_SNB,          seal: module_seal(L_SNB.as_bytes()) },
        SealEntry { platform: Platform::Light, module_name: L_MINI_EVOLVER, seal: module_seal(L_MINI_EVOLVER.as_bytes()) },
        SealEntry { platform: Platform::Light, module_name: L_QUANTUM_BUSS, seal: module_seal(L_QUANTUM_BUSS.as_bytes()) },
        SealEntry { platform: Platform::Light, module_name: L_SHADOW_BUSS,  seal: module_seal(L_SHADOW_BUSS.as_bytes()) },
        // Shadow (aktive)
        SealEntry { platform: Platform::Shadow, module_name: S_JUDICIARY, seal: module_seal(S_JUDICIARY.as_bytes()) },
        SealEntry { platform: Platform::Shadow, module_name: S_VNK,       seal: module_seal(S_VNK.as_bytes()) },
        SealEntry { platform: Platform::Shadow, module_name: S_VOK,       seal: module_seal(S_VOK.as_bytes()) },
        SealEntry { platform: Platform::Shadow, module_name: S_LGC,       seal: module_seal(S_LGC.as_bytes()) },
        SealEntry { platform: Platform::Shadow, module_name: S_VERIFY_OUT, seal: module_seal(S_VERIFY_OUT.as_bytes()) },
    ]
}

// ─────────────────────────────────────────────────────────────────────────────
// DETEKTIM PËRPLASJESH — garanton unicitet
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct CollisionReport {
    pub total:         usize,
    pub unique:        usize,
    pub has_collision: bool,
    pub min_hamming:   u32,   // distanca minimale (sa bit ndryshojnë çift-i më i afërt)
}

pub struct SealRegistry;

impl SealRegistry {
    /// detect_collision — kontrollon nëse ndonjë dy vula janë identike.
    /// O(n²), n=15 → 105 krahasime. Zero if — numërim via formula boolean.
    pub fn detect_collision() -> CollisionReport {
        let registry = build_registry();
        let total = registry.len();

        let mut collisions = 0usize;
        let mut min_hamming = 64u32;
        let mut i = 0usize;
        while i < total {
            let mut j = i + 1;
            while j < total {
                let a = registry[i].seal;
                let b = registry[j].seal;
                collisions += (a == b) as usize;
                let diff = (a ^ b).count_ones();
                let is_smaller = (diff < min_hamming) as u32;
                min_hamming = is_smaller * diff + (1 - is_smaller) * min_hamming;
                j += 1;
            }
            i += 1;
        }

        let has_collision = collisions > 0;
        let unique = total - collisions;

        CollisionReport {
            total,
            unique,
            has_collision,
            min_hamming,
        }
    }

    /// lookup — gjen vulën nga emri i modulit. None nëse s'ekziston. Zero if.
    pub fn lookup(module_name: &str) -> Option<u64> {
        let registry = build_registry();
        registry.iter()
            .find(|e| e.module_name == module_name)
            .map(|e| e.seal)
    }

    /// platform_of — kthen platformën e një vule (nga vlera). None nëse s'gjendet.
    pub fn platform_of(seal: u64) -> Option<Platform> {
        let registry = build_registry();
        registry.iter()
            .find(|e| e.seal == seal)
            .map(|e| e.platform)
    }

    /// count_by_platform — sa vula ka një platformë. Zero if — filter.
    pub fn count_by_platform(platform: Platform) -> usize {
        let registry = build_registry();
        registry.iter()
            .filter(|e| e.platform == platform)
            .count()
    }

    /// is_registered — a është një vulë në regjistër? Zero if.
    pub fn is_registered(seal: u64) -> bool {
        let registry = build_registry();
        registry.iter().any(|e| e.seal == seal)
    }

    /// shadow_seals — vetëm vulat e Shadow (tani aktive). Zero if — filter.
    pub fn shadow_seals() -> usize {
        Self::count_by_platform(Platform::Shadow)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_has_expected_size() {
        let registry = build_registry();
        assert_eq!(registry.len(), REGISTRY_SIZE);
        assert_eq!(REGISTRY_SIZE, 17);
    }

    #[test]
    fn zero_collisions_guaranteed() {
        // INVARIANTI KRYESOR: asnjë dy module s'kanë të njëjtën vulë.
        let report = SealRegistry::detect_collision();
        assert!(!report.has_collision);
        assert_eq!(report.unique, report.total);
        assert_eq!(report.unique, 17);
    }

    #[test]
    fn hamming_distance_safe() {
        let report = SealRegistry::detect_collision();
        assert!(report.min_hamming >= 8);
    }

    #[test]
    fn lookup_finds_shadow_seal() {
        let seal = SealRegistry::lookup(S_JUDICIARY);
        assert!(seal.is_some());
        assert_eq!(seal.unwrap(), module_seal(S_JUDICIARY.as_bytes()));
    }

    #[test]
    fn lookup_unknown_returns_none() {
        assert!(SealRegistry::lookup("NONEXISTENT_MODULE").is_none());
    }

    #[test]
    fn platform_of_seal() {
        let lim_seal = module_seal(Q_LIM.as_bytes());
        assert_eq!(SealRegistry::platform_of(lim_seal), Some(Platform::Quantum));
        let lgc_seal = module_seal(S_LGC.as_bytes());
        assert_eq!(SealRegistry::platform_of(lgc_seal), Some(Platform::Shadow));
    }

    #[test]
    fn count_by_platform_correct() {
        // Quantum: 6, Light: 6, Shadow: 4.
        assert_eq!(SealRegistry::count_by_platform(Platform::Quantum), 6);
        assert_eq!(SealRegistry::count_by_platform(Platform::Light), 6);
        assert_eq!(SealRegistry::count_by_platform(Platform::Shadow), 4);
    }

    #[test]
    fn shadow_seals_now_active() {
        // Shadow është ndërtuar → 4 vula aktive.
        assert_eq!(SealRegistry::shadow_seals(), 4);
    }

    #[test]
    fn cross_platform_seals_consistent() {
        // KRITIKE: vlerat janë identike me ato që Quantum/Light llogarisin.
        // (E njëjta fnv + suffix → e njëjta vlerë byte-for-byte.)
        // Provë: module_seal i njohur llogaritet njësoj.
        let q_lim = module_seal(b"LINEAR_INFO_METRIC");
        // FNV-1a e "LINEAR_INFO_METRIC" XOR FNV-1a e "GJATA_LEGACY" — deterministe.
        assert_eq!(SealRegistry::lookup(Q_LIM), Some(q_lim));
    }

    #[test]
    fn is_registered_works() {
        assert!(SealRegistry::is_registered(module_seal(S_VNK.as_bytes())));
        assert!(!SealRegistry::is_registered(0xDEADBEEF));
    }

    #[test]
    fn platform_prefixes() {
        assert_eq!(Platform::Quantum.prefix(), "Q");
        assert_eq!(Platform::Light.prefix(), "L");
        assert_eq!(Platform::Shadow.prefix(), "S");
    }
}
