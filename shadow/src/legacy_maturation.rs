// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LEGACY_MATURATION.RS — Përforcimi i Maturimit Primitive→Legacy        ║
// ║  Gjata Legacy™  |  Arkitekt: Bledar Gjata                  (FAZA 3)    ║
// ║                                                                          ║
// ║  PARIMI: një Legacy nuk lind menjëherë — lind me KOHË.                 ║
// ║  Përforcon promovimin pa prishur asgjë ekzistuese.                    ║
// ║                                                                          ║
// ║  Ligji bazë (knowledge_vault): legacy_ready ⇔ ≥5 domain-e unike.       ║
// ║                                                                          ║
// ║  PËRFORCIMI këtu shton MATURIMIN REAL si dimension i dytë i nevojshëm: ║
// ║  jo mjafton përhapja ndër-domain — duhet edhe DËSHMI nga bota reale:   ║
// ║    M1  REAL HITS: sa herë u konfirmua nga përdorimi real (≥ pragu).   ║
// ║    M2  TEMPORAL STABILITY: qëndrueshmëria kohore mbi prag.            ║
// ║    M3  CONSULTATION: u konsultua sa duhet si bazë vendimi.            ║
// ║                                                                          ║
// ║  Asnjë hamendësim: M1-M3 lexojnë vetëm fusha që EKZISTOJNË te          ║
// ║  PrimitiveEntry (real_hits, temporal_score, access_count).             ║
// ║  Zero if/else — krahasim & prodhim boolean / formula.                 ║
// ╚══════════════════════════════════════════════════════════════════════════╝

// ─────────────────────────────────────────────────────────────────────────────
// KONSTANTET — pragjet e maturimit (eksplicite, jo magjike)
// ─────────────────────────────────────────────────────────────────────────────

/// Konfirmime minimale nga bota reale që një primitiv konsiderohet i maturuar.
/// (Një legacy është dije që ka "mbijetuar" përdorimin e përsëritur.)
pub const MIN_REAL_HITS: u32 = 3;

/// Qëndrueshmëria minimale kohore (temporal_score) për maturim.
pub const MIN_TEMPORAL_STABILITY: f32 = 0.50;

/// Konsultime minimale si bazë vendimi (access_count) për maturim.
pub const MIN_CONSULTATIONS: u32 = 2;

// ─────────────────────────────────────────────────────────────────────────────
// MATURATION SIGNALS — hyrjet e papërpunuara (Copy, pa varësi nga vault)
// ─────────────────────────────────────────────────────────────────────────────

/// Sinjalet e maturimit të një primitivi — kopjohen nga PrimitiveEntry.
/// E ndarë nga vault që maturimi të testohet/arsyetohet i pavarur.
#[derive(Debug, Clone, Copy)]
pub struct MaturationSignals {
    pub real_hits:      u32,
    pub temporal_score: f32,
    pub access_count:   u32,
    pub domain_count:   usize,  // domain-e unike (nga primitive_chain)
    pub chain_strength: f32,    // nga primitive_chain::ChainStrength
}

// ─────────────────────────────────────────────────────────────────────────────
// M1, M2, M3 — invariantët e maturimit
// ─────────────────────────────────────────────────────────────────────────────

/// M1 — REAL HITS: a u konfirmua mjaftueshëm nga bota reale? Zero if.
#[inline]
pub fn real_hits_met(s: &MaturationSignals) -> bool {
    s.real_hits >= MIN_REAL_HITS
}

/// M2 — TEMPORAL STABILITY: a është qëndrueshmëria kohore mbi prag? Zero if.
#[inline]
pub fn temporal_stable(s: &MaturationSignals) -> bool {
    s.temporal_score >= MIN_TEMPORAL_STABILITY
}

/// M3 — CONSULTATION: a u konsultua sa duhet si bazë vendimi? Zero if.
#[inline]
pub fn consultation_met(s: &MaturationSignals) -> bool {
    s.access_count >= MIN_CONSULTATIONS
}

// ─────────────────────────────────────────────────────────────────────────────
// MATURATION STATE — gjendja e plotë e maturimit
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MaturationState {
    pub real_hits_ok:   bool,   // M1
    pub temporal_ok:    bool,   // M2
    pub consultation_ok: bool,  // M3
    pub maturity_score: f32,    // [0,1] sa i maturuar
    pub matured:        bool,   // M1 ∧ M2 ∧ M3
}

impl MaturationState {
    /// assess — vlerëson maturimin nga sinjalet.
    ///
    /// matured = real_hits_ok ∧ temporal_ok ∧ consultation_ok.
    /// maturity_score = mesatare e peshuar e tre sinjaleve të normalizuara.
    /// Zero if — produkt boolean + formula.
    pub fn assess(s: &MaturationSignals) -> Self {
        let real_hits_ok    = real_hits_met(s);
        let temporal_ok     = temporal_stable(s);
        let consultation_ok = consultation_met(s);

        // matured = produkt i të treve.
        let matured = real_hits_ok & temporal_ok & consultation_ok;

        // maturity_score: normalizo secilin sinjal dhe peshoji.
        // real_hits → /10 (ngopet te 10), temporal → direkt, access → /5.
        let hits_norm   = (s.real_hits as f32 / 10.0).clamp(0.0, 1.0);
        let temporal_norm = s.temporal_score.clamp(0.0, 1.0);
        let access_norm = (s.access_count as f32 / 5.0).clamp(0.0, 1.0);

        let maturity_score = (hits_norm * 0.40
                            + temporal_norm * 0.35
                            + access_norm * 0.25)
                            .clamp(0.0, 1.0);

        Self {
            real_hits_ok,
            temporal_ok,
            consultation_ok,
            maturity_score,
            matured,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// LEGACY REASON — PSE u bë legacy (prejardhja e plotë, plani #7)
// ─────────────────────────────────────────────────────────────────────────────

/// Arsyeja e plotë se SI/PSE një primitiv u bë legacy.
/// Ky është "çdo arsye se si erdhi një legacy" — gjurma e maturimit.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LegacyReason {
    pub domain_count:    usize,  // sa domaine kontribuan
    pub real_hits:       u32,    // sa konfirmime reale
    pub maturity_score:  f32,    // sa i maturuar ishte
    pub chain_strength:  f32,    // sa i fortë ishte zinxhiri
    pub cross_domain_ok: bool,   // a u plotësua rregulla ndër-domain
    pub maturation_ok:   bool,   // a u plotësua maturimi
}

impl LegacyReason {
    /// derive — ndërton arsyen e plotë nga sinjalet + gjendja e maturimit.
    pub fn derive(s: &MaturationSignals, cross_domain_ok: bool) -> Self {
        let state = MaturationState::assess(s);
        Self {
            domain_count:    s.domain_count,
            real_hits:       s.real_hits,
            maturity_score:  state.maturity_score,
            chain_strength:  s.chain_strength,
            cross_domain_ok,
            maturation_ok:   state.matured,
        }
    }

    /// summary — përshkrim njerëzor i prejardhjes (për audit/observim).
    pub fn summary(&self) -> String {
        format!(
            "LEGACY[domains={} hits={} maturity={:.2} strength={:.2} cross_domain={} matured={}]",
            self.domain_count, self.real_hits, self.maturity_score,
            self.chain_strength, self.cross_domain_ok, self.maturation_ok
        )
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// MATURATION GATE — vendimi i përforcuar i promovimit
// ─────────────────────────────────────────────────────────────────────────────

/// Porta e maturimit: legacy kërkon BOTH cross-domain AND maturim real.
/// Ky është përforcimi kryesor i Fazës 3 mbi vendimin ekzistues.
pub struct MaturationGate;

impl MaturationGate {
    /// legacy_qualified — vendimi PËRFORCUAR: a kualifikohet për legacy?
    ///
    /// E VËRTETË vetëm kur BOTH:
    ///   • cross_domain_ok (rregulla ekzistuese: ≥5 domaine + integritet zinxhiri)
    ///   • matured (M1 ∧ M2 ∧ M3: konfirmim real + qëndrueshmëri + konsultim)
    ///
    /// Kjo e bën legacy "më të rëndë": jo vetëm përhapje, por edhe kohë/dëshmi.
    /// Zero if — produkt boolean.
    pub fn legacy_qualified(s: &MaturationSignals, cross_domain_ok: bool) -> bool {
        let state = MaturationState::assess(s);
        cross_domain_ok & state.matured
    }

    /// qualified_with_reason — vendimi + arsyeja e plotë (prejardhja).
    /// Kthen (qualified, reason) — reason regjistrohet për observimin e legacy.
    pub fn qualified_with_reason(
        s: &MaturationSignals,
        cross_domain_ok: bool,
    ) -> (bool, LegacyReason) {
        let reason = LegacyReason::derive(s, cross_domain_ok);
        // qualified = cross_domain ∧ maturation (zero if).
        let qualified = cross_domain_ok & reason.maturation_ok;
        (qualified, reason)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn signals(hits: u32, temporal: f32, access: u32, domains: usize, strength: f32) -> MaturationSignals {
        MaturationSignals {
            real_hits: hits,
            temporal_score: temporal,
            access_count: access,
            domain_count: domains,
            chain_strength: strength,
        }
    }

    #[test]
    fn m1_real_hits_threshold() {
        assert!(real_hits_met(&signals(3, 0.0, 0, 0, 0.0)));  // ==3 OK
        assert!(real_hits_met(&signals(5, 0.0, 0, 0, 0.0)));  // >3 OK
        assert!(!real_hits_met(&signals(2, 0.0, 0, 0, 0.0))); // <3 JO
    }

    #[test]
    fn m2_temporal_threshold() {
        assert!(temporal_stable(&signals(0, 0.50, 0, 0, 0.0)));  // ==0.50 OK
        assert!(temporal_stable(&signals(0, 0.80, 0, 0, 0.0)));  // >0.50 OK
        assert!(!temporal_stable(&signals(0, 0.30, 0, 0, 0.0))); // <0.50 JO
    }

    #[test]
    fn m3_consultation_threshold() {
        assert!(consultation_met(&signals(0, 0.0, 2, 0, 0.0)));  // ==2 OK
        assert!(!consultation_met(&signals(0, 0.0, 1, 0, 0.0))); // <2 JO
    }

    #[test]
    fn maturation_full_pass() {
        // Të tre sinjalet mbi prag → matured.
        let s = signals(5, 0.7, 3, 5, 0.8);
        let state = MaturationState::assess(&s);
        assert!(state.real_hits_ok);
        assert!(state.temporal_ok);
        assert!(state.consultation_ok);
        assert!(state.matured);
        assert!(state.maturity_score > 0.5);
    }

    #[test]
    fn maturation_fails_on_low_hits() {
        // Hits nën prag → jo matured edhe pse të tjerat OK.
        let s = signals(1, 0.9, 5, 5, 0.9);
        let state = MaturationState::assess(&s);
        assert!(!state.real_hits_ok);
        assert!(!state.matured);
    }

    #[test]
    fn maturation_fails_on_low_temporal() {
        let s = signals(5, 0.2, 5, 5, 0.9); // temporal nën prag
        let state = MaturationState::assess(&s);
        assert!(!state.temporal_ok);
        assert!(!state.matured);
    }

    #[test]
    fn gate_requires_both_dimensions() {
        let mature = signals(5, 0.7, 3, 5, 0.8);
        // Cross-domain OK + matured → kualifikohet.
        assert!(MaturationGate::legacy_qualified(&mature, true));
        // Cross-domain JO → NUK kualifikohet (edhe pse matured).
        assert!(!MaturationGate::legacy_qualified(&mature, false));
        // Matured JO + cross-domain OK → NUK kualifikohet.
        let immature = signals(1, 0.1, 0, 5, 0.2);
        assert!(!MaturationGate::legacy_qualified(&immature, true));
    }

    #[test]
    fn legacy_reason_records_provenance() {
        // Arsyeja e plotë regjistron SI erdhi legacy (plani #7).
        let s = signals(5, 0.7, 3, 6, 0.85);
        let (qualified, reason) = MaturationGate::qualified_with_reason(&s, true);
        assert!(qualified);
        assert_eq!(reason.domain_count, 6);
        assert_eq!(reason.real_hits, 5);
        assert!(reason.cross_domain_ok);
        assert!(reason.maturation_ok);
        // Summary përmban prejardhjen.
        let summ = reason.summary();
        assert!(summ.contains("domains=6"));
        assert!(summ.contains("matured=true"));
    }

    #[test]
    fn legacy_reason_explains_rejection() {
        // Kur s'kualifikohet, arsyeja tregon PSE.
        let s = signals(1, 0.2, 0, 3, 0.1);
        let (qualified, reason) = MaturationGate::qualified_with_reason(&s, false);
        assert!(!qualified);
        // Të dyja dimensionet dështuan — arsyeja e dokumenton.
        assert!(!reason.cross_domain_ok);
        assert!(!reason.maturation_ok);
    }

    #[test]
    fn maturity_score_monotonic() {
        // Më shumë hits/temporal/access → maturity më i lartë.
        let low = MaturationState::assess(&signals(3, 0.5, 2, 5, 0.5));
        let high = MaturationState::assess(&signals(10, 0.9, 5, 5, 0.9));
        assert!(high.maturity_score > low.maturity_score);
    }
}
