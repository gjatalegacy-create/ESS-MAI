// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LEGACY_OBSERVER.RS — Vëzhguesi i Përhershëm i Primitivëve            ║
// ║  Gjata Legacy™ | Arkitekt: Bledar Gjata                    (FAZA 7)    ║
// ║                                                                          ║
// ║  PARIMI: një primitiv mund të kërkojë SHUMË KOHË për t'u bërë legacy.   ║
// ║  Prandaj duhet të dimë ÇDO ARSYE se SI erdhi një legacy — prejardhja    ║
// ║  e plotë, e regjistruar përgjithmonë.                                  ║
// ║                                                                          ║
// ║  Ky vëzhgues:                                                          ║
// ║   • Regjistron çdo LINDJE legacy me arsyen e plotë (LegacyReason).      ║
// ║   • Është APPEND-ONLY: asnjë lindje s'fshihet kurrë (parimi i Shadow).  ║
// ║   • Mban statistika agregate mbi maturimin (sa kohë, sa hits, etj.).   ║
// ║                                                                          ║
// ║  Ndërtohet mbi hook-un e Fazës 3: PrimitiveEntry::legacy_reason()       ║
// ║  → LegacyReason. Asnjë hamendësim — vetëm fusha që ekzistojnë.         ║
// ║  Zero if/else — match/formula/iterator.                               ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use std::sync::RwLock;
use crate::legacy_maturation::LegacyReason;

// ─────────────────────────────────────────────────────────────────────────────
// LEGACY BIRTH — regjistrimi i përhershëm i një lindjeje legacy
// ─────────────────────────────────────────────────────────────────────────────

/// Një lindje legacy e regjistruar përgjithmonë: KUSH u bë legacy, KUR,
/// dhe — më e rëndësishmja — PSE/SI (arsyeja e plotë e maturimit).
#[derive(Debug, Clone, Copy)]
pub struct LegacyBirth {
    pub origin_primitive_id: u64,    // cili primitiv u bë legacy
    pub crystallized_at:     u64,    // momenti i kristalizimit (ts_ns)
    pub legacy_score:        f32,    // skori final
    pub reason:              LegacyReason,  // ARSYEJA e plotë (prejardhja)
}

impl LegacyBirth {
    /// summary — përshkrim i plotë njerëzor i lindjes (për audit).
    pub fn summary(&self) -> String {
        format!(
            "BIRTH[id={} at={} score={:.2}] {}",
            self.origin_primitive_id,
            self.crystallized_at,
            self.legacy_score,
            self.reason.summary()
        )
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// OBSERVATION STATS — statistika agregate mbi maturimin
// ─────────────────────────────────────────────────────────────────────────────

/// Statistika agregate mbi të gjitha lindjet legacy të vëzhguara.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ObservationStats {
    pub total_births:        usize,  // sa legacy janë lindur gjithsej
    pub avg_maturity:        f32,    // maturimi mesatar në lindje
    pub avg_real_hits:       f32,    // hits mesatare në lindje
    pub avg_domain_count:    f32,    // domaine mesatare në lindje
    pub avg_chain_strength:  f32,    // fortësi zinxhiri mesatare
}

impl ObservationStats {
    /// empty — gjendja fillestare (asnjë lindje).
    pub fn empty() -> Self {
        Self {
            total_births: 0,
            avg_maturity: 0.0,
            avg_real_hits: 0.0,
            avg_domain_count: 0.0,
            avg_chain_strength: 0.0,
        }
    }

    /// from_births — llogarit statistikat nga një prerje lindjesh.
    /// Mbrojtje pjesëtimi: max(1). Zero if — formula.
    pub fn from_births(births: &[LegacyBirth]) -> Self {
        let n = births.len();
        let denom = n.max(1) as f32;

        let sum_maturity:  f32 = births.iter().map(|b| b.reason.maturity_score).sum();
        let sum_hits:      f32 = births.iter().map(|b| b.reason.real_hits as f32).sum();
        let sum_domains:   f32 = births.iter().map(|b| b.reason.domain_count as f32).sum();
        let sum_strength:  f32 = births.iter().map(|b| b.reason.chain_strength).sum();

        // Faktori i pranisë: 0 lindje → mesataret mbeten 0 (zero if).
        let present = (n > 0) as u32 as f32;

        Self {
            total_births: n,
            avg_maturity:       present * (sum_maturity / denom),
            avg_real_hits:      present * (sum_hits / denom),
            avg_domain_count:   present * (sum_domains / denom),
            avg_chain_strength: present * (sum_strength / denom),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// LEGACY OBSERVER — vëzhguesi i përhershëm (append-only)
// ─────────────────────────────────────────────────────────────────────────────

/// Vëzhguesi i përhershëm mbi primitivët → legacy.
///
/// APPEND-ONLY: çdo lindje regjistrohet dhe s'fshihet kurrë (parimi i Shadow:
/// asnjë gjurmë dije nuk humbet). Mban historinë e plotë të çdo promovimi.
#[derive(Debug)]
pub struct LegacyObserver {
    births: RwLock<Vec<LegacyBirth>>,
}

impl LegacyObserver {
    pub fn new() -> Self {
        Self { births: RwLock::new(Vec::new()) }
    }

    /// observe_birth — regjistron një lindje legacy me arsyen e plotë.
    /// APPEND-ONLY: vetëm shton, kurrë s'fshin. Zero if.
    pub fn observe_birth(
        &self,
        origin_primitive_id: u64,
        crystallized_at: u64,
        legacy_score: f32,
        reason: LegacyReason,
    ) {
        let birth = LegacyBirth {
            origin_primitive_id,
            crystallized_at,
            legacy_score,
            reason,
        };
        // Dështimi i lock-ut s'duhet ta rrëzojë rrjedhën (match, jo unwrap-panik).
        match self.births.write() {
            Ok(mut b) => b.push(birth),
            Err(poisoned) => {
                eprintln!("[LEGACY_OBS] ALARM: births lock i helmuar (observe_birth) — rikuperim + regjistrim");
                poisoned.into_inner().push(birth);
            }
        }
    }

    /// birth_count — sa lindje janë vëzhguar. Zero if.
    pub fn birth_count(&self) -> usize {
        match self.births.read() {
            Ok(b) => b.len(),
            Err(poisoned) => {
                eprintln!("[LEGACY_OBS] ALARM: births lock i helmuar (birth_count) — rikuperim");
                poisoned.into_inner().len()
            }
        }
    }

    /// stats — statistika agregate mbi të gjitha lindjet. Zero if.
    pub fn stats(&self) -> ObservationStats {
        match self.births.read() {
            Ok(b) => ObservationStats::from_births(&b),
            Err(poisoned) => {
                eprintln!("[LEGACY_OBS] ALARM: births lock i helmuar (stats) — rikuperim (statistika të sakta)");
                ObservationStats::from_births(&poisoned.into_inner())
            }
        }
    }

    /// reason_for — gjen arsyen e lindjes së një primitivi specifik.
    /// None nëse s'është vëzhguar ende. Zero if — iterator find.
    pub fn reason_for(&self, origin_primitive_id: u64) -> Option<LegacyReason> {
        match self.births.read() {
            Ok(b) => b.iter()
                .find(|birth| birth.origin_primitive_id == origin_primitive_id)
                .map(|birth| birth.reason),
            Err(poisoned) => {
                eprintln!("[LEGACY_OBS] ALARM: births lock i helmuar (reason_for) — rikuperim");
                poisoned.into_inner().iter()
                    .find(|birth| birth.origin_primitive_id == origin_primitive_id)
                    .map(|birth| birth.reason)
            }
        }
    }

    /// all_summaries — përshkrimet e plota të të gjitha lindjeve (audit).
    pub fn all_summaries(&self) -> Vec<String> {
        match self.births.read() {
            Ok(b) => b.iter().map(|birth| birth.summary()).collect(),
            Err(poisoned) => {
                eprintln!("[LEGACY_OBS] ALARM: births lock i helmuar (all_summaries) — rikuperim");
                poisoned.into_inner().iter().map(|birth| birth.summary()).collect()
            }
        }
    }

    /// was_observed — a është regjistruar lindja e këtij primitivi? Zero if.
    pub fn was_observed(&self, origin_primitive_id: u64) -> bool {
        match self.births.read() {
            Ok(b) => b.iter().any(|birth| birth.origin_primitive_id == origin_primitive_id),
            Err(poisoned) => {
                eprintln!("[LEGACY_OBS] ALARM: births lock i helmuar (was_observed) — rikuperim");
                poisoned.into_inner().iter().any(|birth| birth.origin_primitive_id == origin_primitive_id)
            }
        }
    }
}

impl Default for LegacyObserver {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reason(domains: usize, hits: u32, maturity: f32, strength: f32) -> LegacyReason {
        LegacyReason {
            domain_count:    domains,
            real_hits:       hits,
            maturity_score:  maturity,
            chain_strength:  strength,
            cross_domain_ok: true,
            maturation_ok:   true,
        }
    }

    #[test]
    fn observer_starts_empty() {
        let obs = LegacyObserver::new();
        assert_eq!(obs.birth_count(), 0);
        assert_eq!(obs.stats().total_births, 0);
    }

    #[test]
    fn observe_records_birth() {
        let obs = LegacyObserver::new();
        obs.observe_birth(100, 5000, 0.8, reason(5, 4, 0.7, 0.8));
        assert_eq!(obs.birth_count(), 1);
        assert!(obs.was_observed(100));
        assert!(!obs.was_observed(999));
    }

    #[test]
    fn append_only_never_loses() {
        let obs = LegacyObserver::new();
        // Regjistro 3 lindje — të gjitha mbahen.
        obs.observe_birth(1, 100, 0.6, reason(5, 3, 0.6, 0.7));
        obs.observe_birth(2, 200, 0.7, reason(6, 4, 0.7, 0.8));
        obs.observe_birth(3, 300, 0.8, reason(7, 5, 0.8, 0.9));
        assert_eq!(obs.birth_count(), 3);
    }

    #[test]
    fn reason_for_retrieves_provenance() {
        let obs = LegacyObserver::new();
        obs.observe_birth(42, 1000, 0.75, reason(6, 5, 0.8, 0.85));
        let r = obs.reason_for(42);
        assert!(r.is_some());
        let r = r.unwrap();
        assert_eq!(r.domain_count, 6);
        assert_eq!(r.real_hits, 5);
        // Primitiv i pavëzhguar → None.
        assert!(obs.reason_for(999).is_none());
    }

    #[test]
    fn stats_aggregate_correctly() {
        let obs = LegacyObserver::new();
        // Dy lindje: maturity 0.6 dhe 0.8 → mesatare 0.7.
        obs.observe_birth(1, 100, 0.6, reason(4, 3, 0.6, 0.6));
        obs.observe_birth(2, 200, 0.8, reason(6, 5, 0.8, 0.8));
        let stats = obs.stats();
        assert_eq!(stats.total_births, 2);
        assert!((stats.avg_maturity - 0.7).abs() < 1e-5);
        assert!((stats.avg_real_hits - 4.0).abs() < 1e-5);   // (3+5)/2
        assert!((stats.avg_domain_count - 5.0).abs() < 1e-5); // (4+6)/2
    }

    #[test]
    fn empty_stats_no_division_by_zero() {
        // ObservationStats nga prerje boshe → mesataret 0, pa panik.
        let stats = ObservationStats::from_births(&[]);
        assert_eq!(stats.total_births, 0);
        assert_eq!(stats.avg_maturity, 0.0);
        assert_eq!(stats.avg_real_hits, 0.0);
    }

    #[test]
    fn summaries_contain_provenance() {
        let obs = LegacyObserver::new();
        obs.observe_birth(7, 1234, 0.9, reason(8, 6, 0.85, 0.9));
        let summaries = obs.all_summaries();
        assert_eq!(summaries.len(), 1);
        // Përshkrimi përmban id-në dhe prejardhjen.
        assert!(summaries[0].contains("id=7"));
        assert!(summaries[0].contains("domains=8"));
    }

    #[test]
    fn birth_summary_includes_reason() {
        let birth = LegacyBirth {
            origin_primitive_id: 50,
            crystallized_at: 9999,
            legacy_score: 0.77,
            reason: reason(5, 4, 0.7, 0.75),
        };
        let s = birth.summary();
        assert!(s.contains("id=50"));
        assert!(s.contains("matured=true"));
    }
}
