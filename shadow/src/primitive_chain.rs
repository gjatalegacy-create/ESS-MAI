// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  PRIMITIVE_CHAIN.RS — Përforcimi i Zinxhirit Primitiv (FAZA 2)        ║
// ║  Gjata Legacy™  |  Arkitekt: Bledar Gjata                             ║
// ║                                                                          ║
// ║  Përforcon LIGJIN PRIMITIVE pa prishur asgjë ekzistuese.              ║
// ║                                                                          ║
// ║  Ligji bazë (knowledge_lineage): UNANIM_PRIMITIVE ⇔ ∏ᵢ Xᵢ = 1        ║
// ║  (unanimitet i verifikuesve — VERIFIKIM te Shadow, jo PA e Light-it). ║
// ║                                                                          ║
// ║  PËRFORCIMI këtu shton TRE invariantë mbi gjurmën (PrimitiveTrace),    ║
// ║  sepse një primitiv i vërtetë s'është vetëm "të gjithë thanë 1" —     ║
// ║  duhet edhe integritet i provës:                                      ║
// ║    I1  SEAL CONSISTENCY: çdo gjurmë mban vulën 500 (0xA451).          ║
// ║    I2  TEMPORAL MONOTONICITY: gjurmët rriten në kohë (jo prapa).      ║
// ║    I3  MASS FLOOR: masa epistemike e çdo gjurme ≥ pragu primitiv.     ║
// ║                                                                          ║
// ║  Asnjë hamendësim: të tre invariantët lexojnë vetëm fusha që          ║
// ║  EKZISTOJNË te PrimitiveTrace (domain, timestamp, mass, lgc_seal).     ║
// ║  Zero if/else — krahasim & prodhim boolean.                           ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::types::PrimitiveTrace;

// ─────────────────────────────────────────────────────────────────────────────
// KONSTANTET — pragje të përforcimit (eksplicite, jo magjike)
// ─────────────────────────────────────────────────────────────────────────────

/// Vula 500 e maskuar — identike me gjithë ekosistemin (knowledge_lineage).
pub const SEAL_PRIMITIVE_500: u32 = 500;
pub const SEAL_XOR_MASK:      u32 = 0xA5A5;
pub const SEAL_LOW_MASK:      u32 = 0xFFFF;

/// Masa minimale epistemike që një gjurmë të llogaritet primitive-grade.
/// (Konservative: nën këtë, gjurma s'kontribuon në fortësi.)
pub const PRIMITIVE_MASS_FLOOR: f32 = 0.50;

// ─────────────────────────────────────────────────────────────────────────────
// I1 — SEAL CONSISTENCY
// ─────────────────────────────────────────────────────────────────────────────

/// seal_consistent — a mban ÇDO gjurmë vulën 500? Zero if — all() boolean.
/// (flags & 0xFFFF) ^ 0xA5A5 == 500 për secilën gjurmë.
#[inline]
pub fn seal_consistent(chain: &[PrimitiveTrace]) -> bool {
    chain.iter().all(|t| ((t.lgc_seal & SEAL_LOW_MASK) ^ SEAL_XOR_MASK) == SEAL_PRIMITIVE_500)
}

// ─────────────────────────────────────────────────────────────────────────────
// I2 — TEMPORAL MONOTONICITY
// ─────────────────────────────────────────────────────────────────────────────

/// temporally_monotonic — a rriten timestamp-et (asnjë gjurmë "prapa kohës")?
///
/// Një zinxhir i vërtetë përforcimi ndodh përpara në kohë; gjurmë me kohë
/// që kthehet prapa sinjalizon manipulim ose mbledhje jo-koherente.
/// windows(2): çdo çift fqinjë duhet t[i+1] >= t[i]. Zero if — all() boolean.
/// Zinxhir me 0 ose 1 gjurmë → trivialisht monoton (all mbi bosh = true).
#[inline]
pub fn temporally_monotonic(chain: &[PrimitiveTrace]) -> bool {
    chain.windows(2).all(|w| w[1].timestamp >= w[0].timestamp)
}

// ─────────────────────────────────────────────────────────────────────────────
// I3 — MASS FLOOR
// ─────────────────────────────────────────────────────────────────────────────

/// mass_floor_met — a e kalon masa e ÇDO gjurme pragun primitiv?
/// Zero if — all() boolean.
#[inline]
pub fn mass_floor_met(chain: &[PrimitiveTrace]) -> bool {
    chain.iter().all(|t| t.mass >= PRIMITIVE_MASS_FLOOR)
}

// ─────────────────────────────────────────────────────────────────────────────
// CHAIN INTEGRITY — agregimi i tre invariantëve
// ─────────────────────────────────────────────────────────────────────────────

/// Raport i integritetit të zinxhirit — secili invariant veç + verdikt total.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChainIntegrity {
    pub seal_ok:      bool,   // I1
    pub temporal_ok:  bool,   // I2
    pub mass_ok:      bool,   // I3
    pub trace_count:  usize,
    pub integral:     bool,   // I1 ∧ I2 ∧ I3 ∧ (jo bosh)
}

impl ChainIntegrity {
    /// verify — llogarit të tre invariantët mbi zinxhirin.
    ///
    /// integral = seal_ok ∧ temporal_ok ∧ mass_ok ∧ (trace_count > 0).
    /// Zero if — produkt boolean.
    pub fn verify(chain: &[PrimitiveTrace]) -> Self {
        let seal_ok     = seal_consistent(chain);
        let temporal_ok = temporally_monotonic(chain);
        let mass_ok     = mass_floor_met(chain);
        let non_empty   = !chain.is_empty();

        // integral = produkt i të katërve (bool AND).
        let integral = seal_ok & temporal_ok & mass_ok & non_empty;

        Self {
            seal_ok,
            temporal_ok,
            mass_ok,
            trace_count: chain.len(),
            integral,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// CHAIN STRENGTH — fortësia e matshme e zinxhirit primitiv
// ─────────────────────────────────────────────────────────────────────────────

/// Fortësia e zinxhirit — sa "i thellë" dhe konsistent është përforcimi.
/// Përdoret nga Faza 3 (legacy) si një nga sinjalet e maturimit.
#[derive(Debug, Clone, Copy)]
pub struct ChainStrength {
    pub depth:        usize,  // sa gjurmë gjithsej
    pub avg_mass:     f32,    // masa mesatare e gjurmëve
    pub integral:     bool,   // a kaloi integritetin (I1∧I2∧I3)
    pub strength:     f32,    // [0,1] skor i përgjithshëm
}

impl ChainStrength {
    /// compute — fortësia nga thellësia + masa mesatare + integriteti.
    ///
    /// strength = integral_factor × (depth_factor × 0.5 + mass_factor × 0.5)
    /// ku integral_factor ∈ {0,1} (pa integritet → fortësi 0).
    /// Zero if — formula me faktorë boolean.
    pub fn compute(chain: &[PrimitiveTrace]) -> Self {
        let depth = chain.len();
        let integrity = ChainIntegrity::verify(chain);
        let integral = integrity.integral;

        // Masa mesatare (mbrojtje pjesëtimi: max(1)).
        let sum_mass: f32 = chain.iter().map(|t| t.mass).sum();
        let avg_mass = sum_mass / depth.max(1) as f32;

        // depth_factor: ngopet te ~10 gjurmë (depth/10, clamp 1).
        let depth_factor = (depth as f32 / 10.0).clamp(0.0, 1.0);
        // mass_factor: masa mesatare e clamp-uar.
        let mass_factor = avg_mass.clamp(0.0, 1.0);
        // integral_factor: pa integritet → 0 (zero if).
        let integral_factor = integral as u32 as f32;

        let strength = (integral_factor * (depth_factor * 0.5 + mass_factor * 0.5))
            .clamp(0.0, 1.0);

        Self {
            depth,
            avg_mass,
            integral,
            strength,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ShadowPath;

    // Ndihmës: gjurmë me vlera të kontrolluara.
    fn trace(domain: &str, ts: u64, mass: f32, seal: u32) -> PrimitiveTrace {
        PrimitiveTrace {
            domain: domain.to_string(),
            timestamp: ts,
            mass,
            path_taken: ShadowPath::Standard,
            lgc_seal: seal,
        }
    }

    // Vula 500 e vlefshme: 0xA451 (sepse (0xA451 & 0xFFFF) ^ 0xA5A5 = 500).
    const VALID_SEAL: u32 = 0xA451;

    #[test]
    fn valid_seal_constant_correct() {
        // Provë matematike e vlerës së vulës.
        assert_eq!((VALID_SEAL & 0xFFFF) ^ 0xA5A5, 500);
    }

    #[test]
    fn seal_consistent_all_valid() {
        let chain = vec![
            trace("physics", 100, 0.8, VALID_SEAL),
            trace("chemistry", 200, 0.7, VALID_SEAL),
        ];
        assert!(seal_consistent(&chain));
    }

    #[test]
    fn seal_consistent_detects_invalid() {
        let chain = vec![
            trace("physics", 100, 0.8, VALID_SEAL),
            trace("chemistry", 200, 0.7, 0x1234), // vulë e gabuar
        ];
        assert!(!seal_consistent(&chain));
    }

    #[test]
    fn temporal_monotonic_forward() {
        let chain = vec![
            trace("a", 100, 0.8, VALID_SEAL),
            trace("b", 200, 0.8, VALID_SEAL),
            trace("c", 300, 0.8, VALID_SEAL),
        ];
        assert!(temporally_monotonic(&chain));
    }

    #[test]
    fn temporal_detects_backward() {
        let chain = vec![
            trace("a", 300, 0.8, VALID_SEAL),
            trace("b", 100, 0.8, VALID_SEAL), // koha kthehet prapa
        ];
        assert!(!temporally_monotonic(&chain));
    }

    #[test]
    fn temporal_trivial_for_short_chains() {
        // 0 ose 1 gjurmë → trivialisht monoton.
        let empty: Vec<PrimitiveTrace> = vec![];
        assert!(temporally_monotonic(&empty));
        let one = vec![trace("a", 100, 0.8, VALID_SEAL)];
        assert!(temporally_monotonic(&one));
    }

    #[test]
    fn mass_floor_enforced() {
        let strong = vec![trace("a", 100, 0.8, VALID_SEAL)];
        assert!(mass_floor_met(&strong));
        let weak = vec![trace("a", 100, 0.3, VALID_SEAL)]; // nën 0.50
        assert!(!mass_floor_met(&weak));
    }

    #[test]
    fn chain_integrity_full_pass() {
        let chain = vec![
            trace("physics", 100, 0.8, VALID_SEAL),
            trace("chemistry", 200, 0.7, VALID_SEAL),
            trace("biology", 300, 0.9, VALID_SEAL),
        ];
        let integrity = ChainIntegrity::verify(&chain);
        assert!(integrity.seal_ok);
        assert!(integrity.temporal_ok);
        assert!(integrity.mass_ok);
        assert!(integrity.integral);
        assert_eq!(integrity.trace_count, 3);
    }

    #[test]
    fn chain_integrity_empty_not_integral() {
        let empty: Vec<PrimitiveTrace> = vec![];
        let integrity = ChainIntegrity::verify(&empty);
        // Bosh → jo integral (edhe pse invariantët trivialisht true).
        assert!(!integrity.integral);
    }

    #[test]
    fn chain_integrity_one_bad_seal_breaks() {
        let chain = vec![
            trace("physics", 100, 0.8, VALID_SEAL),
            trace("chemistry", 200, 0.7, 0xDEAD), // vulë e keqe
        ];
        let integrity = ChainIntegrity::verify(&chain);
        assert!(!integrity.seal_ok);
        assert!(!integrity.integral);
    }

    #[test]
    fn chain_strength_scales_with_depth() {
        // Zinxhir i thellë + i fortë → strength i lartë.
        let deep: Vec<PrimitiveTrace> = (0..10)
            .map(|i| trace("d", 100 + i * 10, 0.9, VALID_SEAL))
            .collect();
        let s_deep = ChainStrength::compute(&deep);
        assert!(s_deep.integral);
        assert!(s_deep.strength > 0.7);

        // Zinxhir i cekët → strength më i ulët.
        let shallow = vec![trace("d", 100, 0.9, VALID_SEAL)];
        let s_shallow = ChainStrength::compute(&shallow);
        assert!(s_shallow.strength < s_deep.strength);
    }

    #[test]
    fn chain_strength_zero_without_integrity() {
        // Pa integritet (vulë e keqe) → strength = 0.
        let chain = vec![
            trace("a", 100, 0.9, 0x0000), // vulë e pavlefshme
        ];
        let s = ChainStrength::compute(&chain);
        assert!(!s.integral);
        assert_eq!(s.strength, 0.0);
    }
}
