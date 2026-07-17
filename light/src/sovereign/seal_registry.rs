// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  SOVEREIGN/SEAL_REGISTRY.RS — Regjistri Qendror i Vulave              ║
// ║  GJATA LEGACY™ — ESS-MAI (i përbashkët Quantum + Light + Shadow)      ║
// ║                                                                          ║
// ║  PROBLEMI: 3 platforma → vula moduli të pavarura → rrezik përplasjeje  ║
// ║  kur sistemi rritet (emër i dyfishtë ose kolizion FNV i paqëllimshëm). ║
// ║                                                                          ║
// ║  ZGJIDHJA: një burim i vetëm i vërtetë për TË GJITHA vulat, me:        ║
// ║   • prefix hapësire emrash: Q_ (Quantum) | L_ (Light) | S_ (Shadow)    ║
// ║   • detect_collision() → garanton ZERO përplasje (test-time)          ║
// ║   • lookup() → gjen vulën nga emri                                    ║
// ║                                                                          ║
// ║  KY SKEDAR ËSHTË IDENTIK në Quantum dhe Light (byte-for-byte).         ║
// ║  Të dyja platformat ndajnë të njëjtin regjistër pa crate të përbashkët.║
// ║  Zero if/else — match/iterator/formula.                               ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::sovereign::laws::module_seal;

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
// REGJISTRI — burimi i vetëm i vërtetë për TË GJITHA vulat
// ─────────────────────────────────────────────────────────────────────────────

// Emrat e moduleve — burimi kanonik. Çdo platformë ka prefiksin e vet logjik.
// (Emri fizik që hashohet mbetet i njëjtë me atë te laws.rs e secilës platformë,
//  që vulat të jenë konsistente me kodin ekzistues.)

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

// ── SHADOW (rezervuar — Shadow s'është ndërtuar ende, por vulat e tij
//    deklarohen këtu që asnjë platformë tjetër të mos i ripërdorë emrat) ──────
pub const S_JUDICIARY:  &str = "SHADOW_JUDICIARY";
pub const S_VNK:        &str = "VERIFIED_NEGATIVE_KNOWLEDGE";
pub const S_VOK:        &str = "VERIFIED_OPTIMAL_KNOWLEDGE";
pub const S_LGC:        &str = "SHADOW_GJ_LEGACY";
pub const S_VERIFY_OUT: &str = "SHADOW_VERIFICATION_OUTPUT";

/// Numri total i vulave të regjistruara.
pub const REGISTRY_SIZE: usize = 17;

/// build_registry — ndërton tabelën e plotë të vulave (compile-time emrat,
/// runtime vlerat nga module_seal). Burimi i vetëm i vërtetë.
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
        // Shadow (rezervuar)
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
    pub total:       usize,
    pub unique:      usize,
    pub has_collision: bool,
    pub min_hamming: u32,   // distanca minimale (sa bit ndryshojnë çift-i më i afërt)
}

pub struct SealRegistry;

impl SealRegistry {
    /// detect_collision — kontrollon nëse ndonjë dy vula janë identike.
    ///
    /// Krahason çdo çift O(n²) — n=15, pra 105 krahasime, tepër e shpejtë.
    /// Zero if — numërim via formula boolean.
    pub fn detect_collision() -> CollisionReport {
        let registry = build_registry();
        let total = registry.len();

        // Numëro çiftet identike + gjej Hamming minimal.
        let mut collisions = 0usize;
        let mut min_hamming = 64u32;
        let mut i = 0usize;
        while i < total {
            let mut j = i + 1;
            while j < total {
                let a = registry[i].seal;
                let b = registry[j].seal;
                // identik → collision (branchless: (a==b) as usize).
                collisions += (a == b) as usize;
                // Hamming distance (sa bit ndryshojnë).
                let diff = (a ^ b).count_ones();
                // min via formula (zero if): merr më të voglin.
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

    /// lookup — gjen vulën nga emri i modulit. None nëse s'ekziston.
    /// Zero if — iterator find.
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
        // Distanca minimale duhet të jetë e madhe (vula të largëta).
        let report = SealRegistry::detect_collision();
        // Të paktën 8 bit ndryshim mes çdo çifti (siguri e lartë).
        assert!(report.min_hamming >= 8);
    }

    #[test]
    fn lookup_finds_quantum_seal() {
        let seal = SealRegistry::lookup(Q_LIM);
        assert!(seal.is_some());
        // Përputhet me module_seal direkt.
        assert_eq!(seal.unwrap(), module_seal(Q_LIM.as_bytes()));
    }

    #[test]
    fn lookup_finds_light_seal() {
        let seal = SealRegistry::lookup(L_APUPK);
        assert!(seal.is_some());
        assert_eq!(seal.unwrap(), module_seal(L_APUPK.as_bytes()));
    }

    #[test]
    fn lookup_unknown_returns_none() {
        assert!(SealRegistry::lookup("NONEXISTENT_MODULE").is_none());
    }

    #[test]
    fn platform_of_seal() {
        let lim_seal = module_seal(Q_LIM.as_bytes());
        assert_eq!(SealRegistry::platform_of(lim_seal), Some(Platform::Quantum));
        let apupk_seal = module_seal(L_APUPK.as_bytes());
        assert_eq!(SealRegistry::platform_of(apupk_seal), Some(Platform::Light));
    }

    #[test]
    fn count_by_platform_correct() {
        // Quantum: 6, Light: 6, Shadow: 4.
        assert_eq!(SealRegistry::count_by_platform(Platform::Quantum), 6);
        assert_eq!(SealRegistry::count_by_platform(Platform::Light), 6);
        assert_eq!(SealRegistry::count_by_platform(Platform::Shadow), 4);
    }

    #[test]
    fn is_registered_works() {
        assert!(SealRegistry::is_registered(module_seal(Q_PRO.as_bytes())));
        assert!(!SealRegistry::is_registered(0xDEADBEEF));
    }

    #[test]
    fn platform_prefixes() {
        assert_eq!(Platform::Quantum.prefix(), "Q");
        assert_eq!(Platform::Light.prefix(), "L");
        assert_eq!(Platform::Shadow.prefix(), "S");
    }

}
