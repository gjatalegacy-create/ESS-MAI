// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  MEMORY/MODULE_REPUTATION.RS — Reputacioni i Moduleve (Teoria 26)     ║
// ║  GJATA LEGACY™ — Quantum Platform                                     ║
// ║                                                                          ║
// ║  Quantum di historikisht cilët module japin reasoning të fortë.       ║
// ║  trust_score nga: approval + success + verified_negative + fusion.    ║
// ║  RISIA: verified_negative kontribuon POZITIVISHT — moduli që gjen      ║
// ║  dështime është më i besueshëm.                                       ║
// ║                                                                          ║
// ║  KUJTESË: reputacion EPHEMERAL brenda sesionit. Quantum s'mban memory  ║
// ║  persistente — clear() e fshin me ciklin.                            ║
// ║                                                                          ║
// ║  Pseudo përdor IF; KONVERTUAR në match/aritmetikë boolean (zero if).   ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use std::collections::HashMap;
use crate::sovereign::laws::gate;

// ─────────────────────────────────────────────────────────────────────────────
// OUTCOME + SHADOW RESULT
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RepOutcome {
    Success,
    Failure,
    Contradiction,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShadowResult {
    Approved,
    Rejected,
    Pending,
}

// ─────────────────────────────────────────────────────────────────────────────
// REPUTATION RECORD
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ReputationRecord {
    pub module_name:         String,
    pub territory:           String,
    pub total_submissions:   u32,
    pub successful_conv:     u32,
    pub shadow_approved:     u32,
    pub shadow_rejected:     u32,
    pub verified_negative:   u32,
    pub contradiction_count: u32,
    pub avg_fusion_score:    f32,
    pub trust_score:         f32,
}

impl ReputationRecord {
    fn new(module_name: &str, territory: &str) -> Self {
        Self {
            module_name: module_name.to_string(),
            territory: territory.to_string(),
            total_submissions: 0,
            successful_conv: 0,
            shadow_approved: 0,
            shadow_rejected: 0,
            verified_negative: 0,
            contradiction_count: 0,
            avg_fusion_score: 0.0,
            trust_score: 0.5,  // neutral fillestar
        }
    }

    /// _compute_trust — formula e besimit.
    /// trust = approval*0.40 + success*0.30 + neg_value*0.15 + fusion*0.15 - contra_penalty.
    /// Zero if — total==0 trajtohet me max(1).
    fn compute_trust(&self) -> f32 {
        // total==0 → kthe 0.5 (branchless: is_zero gate).
        let is_zero = (self.total_submissions == 0) as u32 as f32;
        let total = self.total_submissions.max(1) as f32;

        let approval_rate = self.shadow_approved as f32 / total;
        let success_rate = self.successful_conv as f32 / total;
        let neg_value = self.verified_negative as f32 / total;  // kontribut pozitiv
        let contra_penalty = (self.contradiction_count as f32 / total * 0.3).min(0.3);

        let trust = approval_rate * 0.40
                  + success_rate * 0.30
                  + neg_value * 0.15
                  + self.avg_fusion_score * 0.15
                  - contra_penalty;

        // is_zero=1 → 0.5; ndryshe → clamp(trust).
        is_zero * 0.5 + (1.0 - is_zero) * gate(trust, 0.0, 1.0)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// MODULE REPUTATION — regjistri (ephemeral)
// ─────────────────────────────────────────────────────────────────────────────

pub struct ModuleReputation {
    store: HashMap<String, ReputationRecord>,
}

impl ModuleReputation {
    pub fn new() -> Self {
        Self { store: HashMap::new() }
    }

    fn key(module_name: &str, territory: &str) -> String {
        format!("{}:{}", module_name, territory)
    }

    /// update — pas çdo cikli. Zero if — aritmetikë boolean për counters.
    pub fn update(
        &mut self,
        module_name: &str,
        territory: &str,
        outcome: RepOutcome,
        shadow_result: ShadowResult,
        fusion_score: f32,
    ) {
        let key = Self::key(module_name, territory);
        let record = self.store.entry(key)
            .or_insert_with(|| ReputationRecord::new(module_name, territory));

        record.total_submissions += 1;

        // Counters via aritmetikë boolean (zero if).
        record.successful_conv += (outcome == RepOutcome::Success) as u32;
        record.contradiction_count += (outcome == RepOutcome::Contradiction) as u32;
        record.shadow_approved += (shadow_result == ShadowResult::Approved) as u32;
        record.shadow_rejected += (shadow_result == ShadowResult::Rejected) as u32;

        // verified_negative = FAILURE AND APPROVED (failure i dokumentuar = vlerë).
        let is_verified_neg = (outcome == RepOutcome::Failure)
            && (shadow_result == ShadowResult::Approved);
        record.verified_negative += is_verified_neg as u32;

        // Running average fusion.
        let n = record.total_submissions as f32;
        record.avg_fusion_score =
            (record.avg_fusion_score * (n - 1.0) + fusion_score) / n;

        record.trust_score = record.compute_trust();
    }

    /// get — lexo reputacionin.
    pub fn get(&self, module_name: &str, territory: &str) -> Option<&ReputationRecord> {
        self.store.get(&Self::key(module_name, territory))
    }

    /// trust_score — 0.5 neutral nëse pa histori. Zero if — map + unwrap_or.
    pub fn trust_score(&self, module_name: &str, territory: &str) -> f32 {
        self.get(module_name, territory)
            .map(|r| r.trust_score)
            .unwrap_or(0.5)
    }

    /// apply_to_score — trust modifier mbi score (max ±0.10).
    pub fn apply_to_score(&self, provider: &str, current_score: f32, territory: &str) -> f32 {
        let trust = self.trust_score(provider, territory);
        let modifier = (trust - 0.5) * 0.20;  // max ±0.10
        gate(current_score + modifier, 0.0, 1.0)
    }

    /// rank_modules — renditur sipas trust për një territory.
    pub fn rank_modules(&self, territory: &str) -> Vec<&ReputationRecord> {
        let mut relevant: Vec<&ReputationRecord> = self.store.values()
            .filter(|r| r.territory == territory)
            .collect();
        relevant.sort_by(|a, b| {
            b.trust_score.partial_cmp(&a.trust_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        relevant
    }

    /// clear — fund cikli. Quantum s'mban memory persistente.
    pub fn clear(&mut self) {
        self.store.clear();
    }
}

impl Default for ModuleReputation {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_module_neutral_trust() {
        let rep = ModuleReputation::new();
        // Pa histori → 0.5.
        assert_eq!(rep.trust_score("PRO", "science"), 0.5);
    }

    #[test]
    fn success_and_approval_raise_trust() {
        let mut rep = ModuleReputation::new();
        rep.update("PRO", "science", RepOutcome::Success, ShadowResult::Approved, 0.9);
        let t = rep.trust_score("PRO", "science");
        // approval(1.0)*0.40 + success(1.0)*0.30 + neg(0) + fusion(0.9)*0.15 = 0.835
        assert!(t > 0.5);
    }

    #[test]
    fn verified_negative_is_positive() {
        let mut rep = ModuleReputation::new();
        // FAILURE + APPROVED → verified_negative (kontribut pozitiv).
        rep.update("SRK", "science", RepOutcome::Failure, ShadowResult::Approved, 0.6);
        let r = rep.get("SRK", "science").unwrap();
        assert_eq!(r.verified_negative, 1);
        // Trust s'duhet të jetë zero — failure i dokumentuar ka vlerë.
        assert!(r.trust_score > 0.0);
    }

    #[test]
    fn contradictions_penalize() {
        let mut rep = ModuleReputation::new();
        rep.update("BAD", "science", RepOutcome::Contradiction, ShadowResult::Rejected, 0.2);
        let r = rep.get("BAD", "science").unwrap();
        assert_eq!(r.contradiction_count, 1);
        // Trust ulët nga penalty.
        assert!(r.trust_score < 0.5);
    }

    #[test]
    fn running_average_fusion() {
        let mut rep = ModuleReputation::new();
        rep.update("M", "d", RepOutcome::Success, ShadowResult::Approved, 0.8);
        rep.update("M", "d", RepOutcome::Success, ShadowResult::Approved, 0.6);
        let r = rep.get("M", "d").unwrap();
        // (0.8 + 0.6)/2 = 0.7
        assert!((r.avg_fusion_score - 0.7).abs() < 1e-5);
    }

    #[test]
    fn apply_to_score_modifies() {
        let mut rep = ModuleReputation::new();
        // Ndërto trust të lartë.
        rep.update("GOOD", "sci", RepOutcome::Success, ShadowResult::Approved, 0.9);
        let boosted = rep.apply_to_score("GOOD", 0.7, "sci");
        // Trust > 0.5 → boost i vogël.
        assert!(boosted > 0.7);
        // Provider i panjohur → trust 0.5 → pa modifier.
        let neutral = rep.apply_to_score("UNKNOWN", 0.7, "sci");
        assert!((neutral - 0.7).abs() < 1e-5);
    }

    #[test]
    fn rank_orders_by_trust() {
        let mut rep = ModuleReputation::new();
        rep.update("A", "sci", RepOutcome::Success, ShadowResult::Approved, 0.9);
        rep.update("B", "sci", RepOutcome::Contradiction, ShadowResult::Rejected, 0.2);
        let ranked = rep.rank_modules("sci");
        assert_eq!(ranked[0].module_name, "A"); // më i besuari i pari
    }

    #[test]
    fn clear_wipes_ephemeral() {
        let mut rep = ModuleReputation::new();
        rep.update("A", "sci", RepOutcome::Success, ShadowResult::Approved, 0.9);
        rep.clear();  // Quantum s'mban memory
        assert_eq!(rep.trust_score("A", "sci"), 0.5); // kthehet neutral
    }
}
