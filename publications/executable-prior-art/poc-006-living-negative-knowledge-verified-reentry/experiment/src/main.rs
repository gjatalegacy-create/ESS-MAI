use ess_mai_living_negative_source_core::pro_nk_gate::{
    is_sealed_nk_blob, nk_downgrade, KnowledgeVault, NkGate,
};
use ess_mai_living_negative_source_core::{runtime_pulse, tokenizer};

#[derive(Debug, Clone)]
struct ActivationReceipt {
    history_required: bool,
    nk_status: &'static str,
    vault_entries: usize,
    pro_activated: bool,
    candidate_score: Option<f32>,
    hard_blocked: Option<bool>,
}

fn encode_negative_body(process: &str) -> Vec<u8> {
    let mut entry = Vec::new();
    entry.extend_from_slice(&42u64.to_le_bytes());
    entry.extend_from_slice(&0xA451u32.to_le_bytes());
    entry.extend_from_slice(&0.75f32.to_bits().to_le_bytes());
    entry.extend_from_slice(&1u32.to_le_bytes());
    entry.extend_from_slice(&0u32.to_le_bytes());
    entry.extend_from_slice(&1_725_000_000u64.to_le_bytes());
    entry.extend_from_slice(&(process.len() as u32).to_le_bytes());
    entry.extend_from_slice(process.as_bytes());

    let mut body = Vec::new();
    body.extend_from_slice(&1u32.to_le_bytes());
    body.extend_from_slice(&(entry.len() as u32).to_le_bytes());
    body.extend_from_slice(&entry);
    body
}

fn guarded_activation(
    sealed_marker_exists: bool,
    blob: Option<&[u8]>,
    candidate: &str,
) -> ActivationReceipt {
    runtime_pulse::begin_cycle();
    let blob_declares_seal = blob.map(is_sealed_nk_blob).unwrap_or(false);
    let history_required = sealed_marker_exists | blob_declares_seal;

    let mut vault = match blob {
        Some(bytes) => match nk_downgrade(sealed_marker_exists, is_sealed_nk_blob(bytes)) {
            true => {
                runtime_pulse::mark_notready(runtime_pulse::Stage::NkImport);
                KnowledgeVault::new()
            }
            false => KnowledgeVault::from_negative_export(bytes),
        },
        None => KnowledgeVault::new(),
    };

    let status = runtime_pulse::nk_status();
    if history_required && !status.is_ready() {
        return ActivationReceipt {
            history_required,
            nk_status: status.code(),
            vault_entries: vault.len(),
            pro_activated: false,
            candidate_score: None,
            hard_blocked: None,
        };
    }

    let stems = tokenizer::semantic_stems(candidate);
    let (score, verdict) = NkGate::post_filter(&mut vault, &stems, 0.90);
    ActivationReceipt {
        history_required,
        nk_status: status.code(),
        vault_entries: vault.len(),
        pro_activated: true,
        candidate_score: Some(score),
        hard_blocked: Some(verdict.hard_blocked),
    }
}

fn main() {
    let failure = "perpetual motion energy free infinite machine";
    let body = encode_negative_body(failure);
    let sealed = KnowledgeVault::seal_negative_export(&body);

    let first_cycle = guarded_activation(false, None, "new bounded thermodynamic model");
    let verified_reentry = guarded_activation(true, Some(&sealed), failure);

    let mut corrupted = sealed.clone();
    let last = corrupted.len() - 1;
    corrupted[last] ^= 0x01;
    let corrupt_history = guarded_activation(true, Some(&corrupted), failure);
    let missing_history = guarded_activation(true, None, failure);
    let downgrade = guarded_activation(true, Some(&body), failure);

    println!("ARTIFACT_TYPE=THEORY_POC");
    println!("THEORY=LIVING_NEGATIVE_KNOWLEDGE_VERIFIED_REENTRY");
    println!("FIRST_CYCLE_PRO_ACTIVATED={}", first_cycle.pro_activated);
    println!("VALID_HISTORY_STATUS={}", verified_reentry.nk_status);
    println!("VALID_HISTORY_ENTRIES={}", verified_reentry.vault_entries);
    println!(
        "KNOWN_FAILURE_HARD_BLOCKED={}",
        verified_reentry.hard_blocked.unwrap_or(false)
    );
    println!(
        "KNOWN_FAILURE_SCORE={:.3}",
        verified_reentry.candidate_score.unwrap_or(-1.0)
    );
    println!("CORRUPT_HISTORY_STATUS={}", corrupt_history.nk_status);
    println!("CORRUPT_HISTORY_PRO_ACTIVATED={}", corrupt_history.pro_activated);
    println!("MISSING_HISTORY_STATUS={}", missing_history.nk_status);
    println!("MISSING_HISTORY_PRO_ACTIVATED={}", missing_history.pro_activated);
    println!("DOWNGRADE_STATUS={}", downgrade.nk_status);
    println!("DOWNGRADE_PRO_ACTIVATED={}", downgrade.pro_activated);
    println!("FULL_NEGATIVE_RECEIPT_WIRE_MATERIALIZED=false");
    println!("NEGATIVE_PERSISTED_BOOLEAN_ONLY_GAP=true");

    let passed = first_cycle.pro_activated
        && verified_reentry.history_required
        && verified_reentry.nk_status == "READY"
        && verified_reentry.vault_entries == 1
        && verified_reentry.hard_blocked == Some(true)
        && verified_reentry.candidate_score == Some(0.0)
        && !corrupt_history.pro_activated
        && !missing_history.pro_activated
        && !downgrade.pro_activated;
    println!("EXPERIMENT_STATUS={}", if passed { "PASS" } else { "FAIL" });
}

#[cfg(test)]
mod tests {
    use super::*;
    use ess_mai_living_negative_source_core::lab_contracts::gcl_presume::GclReadiness;

    #[test]
    fn first_cycle_without_history_may_activate() {
        let result = guarded_activation(false, None, "new path");
        assert!(!result.history_required);
        assert!(result.pro_activated);
        assert_eq!(result.candidate_score, Some(0.90));
    }

    #[test]
    fn verified_negative_history_changes_future_reasoning() {
        let failure = "perpetual motion energy free infinite machine";
        let body = encode_negative_body(failure);
        let sealed = KnowledgeVault::seal_negative_export(&body);
        let result = guarded_activation(true, Some(&sealed), failure);
        assert_eq!(result.nk_status, "READY");
        assert_eq!(result.vault_entries, 1);
        assert!(result.pro_activated);
        assert_eq!(result.hard_blocked, Some(true));
        assert_eq!(result.candidate_score, Some(0.0));
    }

    #[test]
    fn corrupt_required_history_blocks_before_reasoning() {
        let body = encode_negative_body("known failure");
        let mut sealed = KnowledgeVault::seal_negative_export(&body);
        let last = sealed.len() - 1;
        sealed[last] ^= 0x80;
        let result = guarded_activation(true, Some(&sealed), "known failure");
        assert_eq!(result.nk_status, "NOT_READY");
        assert!(!result.pro_activated);
        assert_eq!(result.candidate_score, None);
    }

    #[test]
    fn missing_or_downgraded_required_history_blocks_before_reasoning() {
        let missing = guarded_activation(true, None, "known failure");
        assert!(matches!(
            runtime_pulse::nk_status(),
            GclReadiness::Degraded(_)
        ));
        assert!(!missing.pro_activated);

        let body = encode_negative_body("known failure");
        let downgraded = guarded_activation(true, Some(&body), "known failure");
        assert_eq!(downgraded.nk_status, "NOT_READY");
        assert!(!downgraded.pro_activated);
    }
}
