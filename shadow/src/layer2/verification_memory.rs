// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAYER2/VERIFICATION_MEMORY.RS — Persistent Lab Memory (Shadow)      ║
// ║  GJATA LEGACY™ — ESS-MAI Shadow                          (LAYER 2)    ║
// ║                                                                          ║
// ║  SHKRIRJA "PËR MË LART" — cikli vetë-përmirësues, PA ndryshuar role.   ║
// ║                                                                          ║
// ║  Problemi që zgjidh: ledger-i (VerificationLedgerEntry) dhe negative    ║
// ║  knowledge regjistroheshin, por ASKUSH s'i lexonte për të mësuar.       ║
// ║  Kjo është "Persistent Lab Memory" e dokumentit: sistemi mëson jo       ║
// ║  vetëm nga suksesi, por nga ÇDO dështim — me kalimin e kohës.          ║
// ║                                                                          ║
// ║  CIKLI I MBYLLUR (role TË PANDRYSHUARA):                              ║
// ║    • SHADOW shkruan ledger-in        → roli i tij (vendos + regjistron) ║
// ║    • Memory Index grumbullon precedentë → kujtesë, jo vendim           ║
// ║    • LIGHT lexon precedentin          → roli i tij (ndërton besim)      ║
// ║                                                                          ║
// ║  Index-i NUK vendos, NUK gjykon — vetëm grumbullon e ekspozon precedentë║
// ║  që besueshmëria të bëhet më e zgjuar herën tjetër. Zero if/else.     ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::lab_contracts::memory::{VerificationLedgerEntry, NegativeKnowledgeRecord};
use crate::lab_contracts::verdict::VerificationState;

// ─────────────────────────────────────────────────────────────────────────────
// PRECEDENT — përmbledhja e historisë së verifikimit për një pretendim/rrugë
// ─────────────────────────────────────────────────────────────────────────────

/// Precedenti i grumbulluar për një claim_id — çfarë mësoi sistemi për të.
/// Ky është sinjali që Light lexon për besim më të zgjuar.
#[derive(Debug, Clone, Copy)]
pub struct Precedent {
    pub claim_id:          u64,
    /// Sa herë kaloi verifikimin.
    pub pass_count:        u32,
    /// Sa herë dështoi.
    pub fail_count:        u32,
    /// Forca mesatare e regjistrimeve [0,1].
    pub avg_strength:      f32,
    /// Gjendja më e lartë e arritur ndonjëherë.
    pub best_state:        VerificationState,
}

impl Precedent {
    /// empty — pa precedent (pretendim i panjohur). Zero if.
    pub fn empty(claim_id: u64) -> Self {
        Self {
            claim_id,
            pass_count: 0,
            fail_count: 0,
            avg_strength: 0.0,
            best_state: VerificationState::Unverified,
        }
    }

    /// total_attempts — sa herë u provua gjithsej. Zero if.
    pub fn total_attempts(&self) -> u32 {
        self.pass_count + self.fail_count
    }

    /// has_precedent — a ka ndonjë histori? Zero if.
    pub fn has_precedent(&self) -> bool {
        self.total_attempts() > 0
    }

    /// success_rate — raporti i suksesit [0,1]. Zero if — formula me mbrojtje.
    pub fn success_rate(&self) -> f32 {
        let total = self.total_attempts().max(1) as f32;
        (self.pass_count as f32 / total).clamp(0.0, 1.0)
    }

    /// trust_signal — sinjali i besueshmërisë që Light lexon [0,1].
    /// Kombinon sukses + forcë + gjendje. Zero if — formula.
    pub fn trust_signal(&self) -> f32 {
        let rate = self.success_rate();
        let strength = self.avg_strength.clamp(0.0, 1.0);
        let state_factor = (self.best_state.rank() as f32 / 5.0).clamp(0.0, 1.0);
        (rate * 0.45 + strength * 0.30 + state_factor * 0.25).clamp(0.0, 1.0)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// VERIFICATION MEMORY INDEX — grumbullon precedentë nga ledger-i
// ─────────────────────────────────────────────────────────────────────────────

/// Kujtesa e përhershme laboratorike: grumbullon ledger-in në precedentë.
///
/// APPEND-ONLY në frymë: çdo regjistrim ledger-i e pasuron kujtesën; asgjë
/// nuk fshihet. Sistemi mëson nga sukses DHE dështim me kalimin e kohës.
#[derive(Debug, Clone)]
pub struct VerificationMemoryIndex {
    /// Regjistrimet e ledger-it (sukses + dështim).
    ledger:    Vec<VerificationLedgerEntry>,
    /// Dështimet si aset (negative knowledge).
    negatives: Vec<NegativeKnowledgeRecord>,
}

impl VerificationMemoryIndex {
    /// new — kujtesë bosh. Zero if.
    pub fn new() -> Self {
        Self { ledger: Vec::new(), negatives: Vec::new() }
    }

    /// absorb_ledger — thith një regjistrim ledger-i (nga Shadow). Zero if.
    /// Shadow shkruan; kjo vetëm grumbullon (kujtesë, jo vendim).
    pub fn absorb_ledger(&mut self, entry: VerificationLedgerEntry) {
        self.ledger.push(entry);
    }

    /// absorb_negative — thith një dështim si aset. Zero if.
    pub fn absorb_negative(&mut self, record: NegativeKnowledgeRecord) {
        self.negatives.push(record);
    }

    /// precedent_for — ndërton precedentin për një claim_id nga e gjithë kujtesa.
    /// Ky është sinjali që Light lexon. Zero if — agregim me iterator.
    pub fn precedent_for(&self, claim_id: u64) -> Precedent {
        // Filtroj regjistrimet për këtë claim.
        let entries: Vec<&VerificationLedgerEntry> = self.ledger.iter()
            .filter(|e| e.claim_id == claim_id)
            .collect();

        // Numëro kalime/dështime.
        let pass_count = entries.iter().filter(|e| e.final_verdict == 1).count() as u32;
        let fail_count = entries.iter().filter(|e| e.final_verdict == 0).count() as u32;

        // Forca mesatare (me mbrojtje nga pjesëtimi me zero).
        let total = entries.len().max(1) as f32;
        let strength_sum: f32 = entries.iter().map(|e| e.strength()).sum();
        let avg_strength = (strength_sum / total).clamp(0.0, 1.0);

        // Gjendja më e lartë e arritur (rank max via fold).
        let best_rank = entries.iter()
            .map(|e| e.state.rank())
            .fold(0u8, |acc, r| acc.max(r));
        let best_state = Self::state_from_rank(best_rank);

        Precedent {
            claim_id,
            pass_count,
            fail_count,
            avg_strength,
            best_state,
        }
    }

    /// is_known_failure — a është një claim i njohur si dështim? Zero if.
    /// Light e përdor për filtrim të butë (më shumë kujdes me dështime të njohura).
    pub fn is_known_failure(&self, claim_id: u64) -> bool {
        self.negatives.iter().any(|n| {
            let matches = n.claim_id == claim_id;
            let verified = n.verified_by_shadow;
            matches & verified
        })
    }

    /// ledger_size — sa regjistrime ka kujtesa. Zero if.
    pub fn ledger_size(&self) -> usize {
        self.ledger.len()
    }

    /// negative_size — sa dështime-aset ka. Zero if.
    pub fn negative_size(&self) -> usize {
        self.negatives.len()
    }

    /// state_from_rank — kthen VerificationState nga rank-u. Zero if — match.
    fn state_from_rank(rank: u8) -> VerificationState {
        match rank {
            0 => VerificationState::Unverified,
            1 => VerificationState::Reasoned,
            2 => VerificationState::PressureTested,
            3 => VerificationState::CrossDomainVerified,
            4 => VerificationState::EvidenceVerified,
            _ => VerificationState::OperationallyVerified,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ledger(claim_id: u64, verdict: u8, state: VerificationState, p: f32, c: f32) -> VerificationLedgerEntry {
        VerificationLedgerEntry {
            claim_id, state,
            evidence_hash: "h".to_string(),
            pressure_score: p, cross_domain_score: c,
            final_verdict: verdict, recorded_at: 100,
        }
    }

    #[test]
    fn empty_index_no_precedent() {
        let idx = VerificationMemoryIndex::new();
        let p = idx.precedent_for(1);
        assert!(!p.has_precedent());
        assert_eq!(p.trust_signal(), 0.0);
    }

    #[test]
    fn absorbs_and_builds_precedent() {
        let mut idx = VerificationMemoryIndex::new();
        idx.absorb_ledger(ledger(1, 1, VerificationState::EvidenceVerified, 0.8, 0.7));
        idx.absorb_ledger(ledger(1, 1, VerificationState::PressureTested, 0.6, 0.5));
        idx.absorb_ledger(ledger(1, 0, VerificationState::Reasoned, 0.3, 0.2));

        let p = idx.precedent_for(1);
        assert!(p.has_precedent());
        assert_eq!(p.pass_count, 2);
        assert_eq!(p.fail_count, 1);
        assert_eq!(p.total_attempts(), 3);
        // Gjendja më e lartë e arritur = EvidenceVerified.
        assert_eq!(p.best_state, VerificationState::EvidenceVerified);
    }

    #[test]
    fn success_rate_correct() {
        let mut idx = VerificationMemoryIndex::new();
        idx.absorb_ledger(ledger(1, 1, VerificationState::Reasoned, 0.5, 0.5));
        idx.absorb_ledger(ledger(1, 1, VerificationState::Reasoned, 0.5, 0.5));
        idx.absorb_ledger(ledger(1, 1, VerificationState::Reasoned, 0.5, 0.5));
        idx.absorb_ledger(ledger(1, 0, VerificationState::Reasoned, 0.5, 0.5));
        let p = idx.precedent_for(1);
        // 3 sukses / 4 = 0.75
        assert!((p.success_rate() - 0.75).abs() < 1e-6);
    }

    #[test]
    fn trust_signal_bounded() {
        let mut idx = VerificationMemoryIndex::new();
        idx.absorb_ledger(ledger(1, 1, VerificationState::OperationallyVerified, 1.0, 1.0));
        let p = idx.precedent_for(1);
        let signal = p.trust_signal();
        assert!(signal > 0.0 && signal <= 1.0);
    }

    #[test]
    fn separate_claims_separate_precedents() {
        let mut idx = VerificationMemoryIndex::new();
        idx.absorb_ledger(ledger(1, 1, VerificationState::EvidenceVerified, 0.8, 0.7));
        idx.absorb_ledger(ledger(2, 0, VerificationState::Reasoned, 0.2, 0.1));
        let p1 = idx.precedent_for(1);
        let p2 = idx.precedent_for(2);
        assert_eq!(p1.pass_count, 1);
        assert_eq!(p2.fail_count, 1);
        assert_eq!(p2.pass_count, 0);
    }

    #[test]
    fn known_failure_detection() {
        let mut idx = VerificationMemoryIndex::new();
        let nk = NegativeKnowledgeRecord::new(
            5, "destabilizes".to_string(), "npro->fail".to_string(), true, 100,
        );
        idx.absorb_negative(nk);
        assert!(idx.is_known_failure(5));
        assert!(!idx.is_known_failure(99));
    }

    #[test]
    fn memory_grows_append_only() {
        let mut idx = VerificationMemoryIndex::new();
        assert_eq!(idx.ledger_size(), 0);
        idx.absorb_ledger(ledger(1, 1, VerificationState::Reasoned, 0.5, 0.5));
        idx.absorb_ledger(ledger(2, 1, VerificationState::Reasoned, 0.5, 0.5));
        assert_eq!(idx.ledger_size(), 2);
    }

    #[test]
    fn precedent_feeds_light_trust() {
        // Cikli i mbyllur: precedenti i fortë → trust_signal i lartë për Light.
        let mut idx = VerificationMemoryIndex::new();
        idx.absorb_ledger(ledger(1, 1, VerificationState::EvidenceVerified, 0.9, 0.8));
        idx.absorb_ledger(ledger(1, 1, VerificationState::EvidenceVerified, 0.9, 0.8));
        let strong = idx.precedent_for(1).trust_signal();

        let mut idx2 = VerificationMemoryIndex::new();
        idx2.absorb_ledger(ledger(2, 0, VerificationState::Reasoned, 0.2, 0.1));
        let weak = idx2.precedent_for(2).trust_signal();

        // Precedenti i mirë jep sinjal më të fortë → Light ndërton besim më të zgjuar.
        assert!(strong > weak);
    }
}
