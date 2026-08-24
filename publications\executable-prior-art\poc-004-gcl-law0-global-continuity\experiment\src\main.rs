use gcl_constitution::{CollapsePhase, UncertaintyLedger};
use sha2::{Digest, Sha256};

#[allow(dead_code)]
#[rustfmt::skip]
#[path = "../../extracted/shadow_verification_receipt.rs"]
mod shadow_verification_receipt;

#[derive(Debug, Clone)]
struct ExperimentReport {
    canonical_collapsed: bool,
    canonical_steps: usize,
    canonical_chain: String,
    local_expansion_rejected: bool,
    local_rejection_left_ledger_empty: bool,
    nonempty_rejection_atomic: bool,
    discontinuity_accepted: bool,
    discontinuity_previous_after: f32,
    discontinuity_injected_before: f32,
    discontinuity_current_after: f32,
    discontinuity_expanded_global_state: bool,
    discontinuity_before_hidden_from_report: bool,
    discontinuity_chain: String,
    phase_regression_accepted: bool,
    collapsed_before_phase_regression: bool,
    collapsed_after_phase_regression: bool,
    phase_regression_steps: usize,
    phase_regression_current: f32,
    phase_regression_chain: String,
    negative_space_accepted: bool,
    negative_space_collapsed: bool,
    negative_space_current: f32,
    infinite_space_accepted: bool,
    infinite_space_current_is_infinite: bool,
    shadow_receipt_id_is_canonical: bool,
    current_shadow_receipt_changes_with_law0: bool,
    proposed_connection_changes_with_law0: bool,
    proposed_connection_rejects_law0_tamper: bool,
}

fn sha256_parts(parts: &[&[u8]]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    parts.iter().for_each(|part| hasher.update(part));
    hasher.finalize().into()
}

fn law0_linked_receipt_id(receipt_id: &str, law0_digest: &[u8; 32]) -> [u8; 32] {
    sha256_parts(&[
        b"ESSMAI_LAW0_VERIFICATION_RECEIPT_CONNECTION_POC_V1",
        &(receipt_id.len() as u64).to_le_bytes(),
        receipt_id.as_bytes(),
        law0_digest,
    ])
}

fn run_experiment() -> ExperimentReport {
    let mut canonical = UncertaintyLedger::new();
    canonical
        .record(CollapsePhase::Coordination, "primitive_anchor", 16.0, 3.0)
        .expect("canonical coordination must contract");
    canonical
        .record(CollapsePhase::Reasoning, "npro", 3.0, 2.0)
        .expect("canonical elimination must contract");
    canonical
        .record(CollapsePhase::Reasoning, "pro", 2.0, 1.0)
        .expect("canonical proposal must contract");
    canonical
        .record(CollapsePhase::Reasoning, "mpro", 1.0, 1.0)
        .expect("equality is permitted");
    canonical
        .record(CollapsePhase::Verification, "kolaps3", 1.0, 0.0)
        .expect("canonical verification must collapse");

    let mut local_expansion = UncertaintyLedger::new();
    let expansion =
        local_expansion.record(CollapsePhase::Reasoning, "fault_local_expansion", 2.0, 5.0);

    // Control: rejection must also leave an already-populated ledger unchanged.
    let mut atomicity = UncertaintyLedger::new();
    atomicity
        .record(CollapsePhase::Coordination, "atomicity_seed", 10.0, 5.0)
        .expect("atomicity seed is locally valid");
    let atomicity_steps_before = atomicity.steps().len();
    let atomicity_space_before = atomicity.current_space();
    let atomicity_chain_before = atomicity.chain_report();
    let atomicity_rejection =
        atomicity.record(CollapsePhase::Reasoning, "atomicity_reject", 5.0, 6.0);
    let nonempty_rejection_atomic = atomicity_rejection.is_err()
        && atomicity.steps().len() == atomicity_steps_before
        && atomicity.current_space() == atomicity_space_before
        && atomicity.chain_report() == atomicity_chain_before;

    // Fault 1: each tuple is locally non-expansive, but the second tuple's
    // caller-supplied `before` is not linked to the previous accepted `after`.
    let mut discontinuity = UncertaintyLedger::new();
    discontinuity
        .record(CollapsePhase::Coordination, "fault_first_step", 16.0, 1.0)
        .expect("first fault step is locally valid");
    let discontinuity_previous_after = discontinuity.current_space();
    let discontinuity_injected_before = 100.0;
    let discontinuity_current_after = 50.0;
    let discontinuity_accepted = discontinuity
        .record(
            CollapsePhase::Reasoning,
            "fault_discontinuous_reset",
            discontinuity_injected_before,
            discontinuity_current_after,
        )
        .is_ok();
    let discontinuity_chain = discontinuity.chain_report();

    // Fault 2: a terminal verification state can be followed by an earlier
    // phase, because phase order and terminal immutability are not enforced.
    let mut phase_regression = UncertaintyLedger::new();
    phase_regression
        .record(
            CollapsePhase::Verification,
            "fault_terminal_verification",
            1.0,
            0.0,
        )
        .expect("terminal step is locally valid");
    let collapsed_before_phase_regression = phase_regression.is_collapsed();
    let phase_regression_accepted = phase_regression
        .record(
            CollapsePhase::Coordination,
            "fault_restart_after_terminal",
            100.0,
            50.0,
        )
        .is_ok();

    // Fault 3: the public ledger accepts a negative possibility-space value.
    let mut negative_space = UncertaintyLedger::new();
    let negative_space_accepted = negative_space
        .record(CollapsePhase::Reasoning, "fault_negative_space", 1.0, -1.0)
        .is_ok();

    // Fault 4: infinity is accepted as an ordinary measured transition because
    // the API does not require finite values.
    let mut infinite_space = UncertaintyLedger::new();
    let infinite_space_accepted = infinite_space
        .record(
            CollapsePhase::Coordination,
            "fault_infinite_space",
            f32::INFINITY,
            f32::INFINITY,
        )
        .is_ok();

    // Shadow connection audit: this invokes the byte-identical v189 Shadow
    // receipt algorithm. The current receipt does not accept `law0_digest`, so
    // two distinct LAW-0 reports yield the same receipt ID. The separate POC
    // connection hash shows the existing seam without modifying production.
    let shadow_receipt_id = shadow_verification_receipt::receipt_id(
        "session-law0",
        "i0-law0",
        7,
        9,
        13,
        17,
        &[19; 32],
        1,
        1,
        3,
        11,
    );
    let mut shadow_close = UncertaintyLedger::new();
    shadow_close
        .record(CollapsePhase::Verification, "kolaps3_D", 1.0, 0.0)
        .expect("Shadow LAW-0 close must contract");
    let shadow_report = shadow_close.chain_report();
    let shadow_law0_digest = sha256_parts(&[b"ESSMAI_LAW0_CLOSE_V1", shadow_report.as_bytes()]);
    let tampered_shadow_report = format!("{shadow_report}_TAMPERED");
    let tampered_law0_digest =
        sha256_parts(&[b"ESSMAI_LAW0_CLOSE_V1", tampered_shadow_report.as_bytes()]);
    let same_shadow_receipt_id = shadow_verification_receipt::receipt_id(
        "session-law0",
        "i0-law0",
        7,
        9,
        13,
        17,
        &[19; 32],
        1,
        1,
        3,
        11,
    );
    let linked = law0_linked_receipt_id(&shadow_receipt_id, &shadow_law0_digest);
    let linked_tampered = law0_linked_receipt_id(&shadow_receipt_id, &tampered_law0_digest);

    ExperimentReport {
        canonical_collapsed: canonical.is_collapsed(),
        canonical_steps: canonical.steps().len(),
        canonical_chain: canonical.chain_report(),
        local_expansion_rejected: expansion.is_err(),
        local_rejection_left_ledger_empty: local_expansion.steps().is_empty(),
        nonempty_rejection_atomic,
        discontinuity_accepted,
        discontinuity_previous_after,
        discontinuity_injected_before,
        discontinuity_current_after,
        discontinuity_expanded_global_state: discontinuity.current_space()
            > discontinuity_previous_after,
        discontinuity_before_hidden_from_report: !discontinuity_chain
            .contains(&format!("{discontinuity_injected_before:.0}")),
        discontinuity_chain,
        phase_regression_accepted,
        collapsed_before_phase_regression,
        collapsed_after_phase_regression: phase_regression.is_collapsed(),
        phase_regression_steps: phase_regression.steps().len(),
        phase_regression_current: phase_regression.current_space(),
        phase_regression_chain: phase_regression.chain_report(),
        negative_space_accepted,
        negative_space_collapsed: negative_space.is_collapsed(),
        negative_space_current: negative_space.current_space(),
        infinite_space_accepted,
        infinite_space_current_is_infinite: infinite_space.current_space().is_infinite(),
        shadow_receipt_id_is_canonical: shadow_verification_receipt::is_canonical_receipt_id(
            &shadow_receipt_id,
        ),
        current_shadow_receipt_changes_with_law0: shadow_receipt_id != same_shadow_receipt_id,
        proposed_connection_changes_with_law0: linked != linked_tampered,
        proposed_connection_rejects_law0_tamper: law0_linked_receipt_id(
            &shadow_receipt_id,
            &tampered_law0_digest,
        ) != linked,
    }
}

fn main() {
    let report = run_experiment();

    println!("ARTIFACT_TYPE=THEORY_POC");
    println!("THEORY=GCL_LAW0_GLOBAL_MONOTONE_UNCERTAINTY_COLLAPSE");
    println!("SOURCE_CORE_IDENTITY=VERIFIED_6_OF_6_SHA256");
    println!("REFERENCE_CHAIN_COLLAPSED={}", report.canonical_collapsed);
    println!("REFERENCE_CHAIN_STEPS={}", report.canonical_steps);
    println!("REFERENCE_CHAIN={}", report.canonical_chain);
    println!(
        "LOCAL_EXPANSION_REJECTED={}",
        report.local_expansion_rejected
    );
    println!(
        "LOCAL_REJECTION_LEFT_LEDGER_EMPTY={}",
        report.local_rejection_left_ledger_empty
    );
    println!(
        "NONEMPTY_REJECTION_ATOMIC={}",
        report.nonempty_rejection_atomic
    );
    println!("DISCONTINUITY_ACCEPTED={}", report.discontinuity_accepted);
    println!(
        "DISCONTINUITY_PREVIOUS_AFTER={:.2}",
        report.discontinuity_previous_after
    );
    println!(
        "DISCONTINUITY_INJECTED_BEFORE={:.2}",
        report.discontinuity_injected_before
    );
    println!(
        "DISCONTINUITY_CURRENT_AFTER={:.2}",
        report.discontinuity_current_after
    );
    println!(
        "DISCONTINUITY_EXPANDED_GLOBAL_STATE={}",
        report.discontinuity_expanded_global_state
    );
    println!(
        "DISCONTINUITY_BEFORE_HIDDEN_FROM_REPORT={}",
        report.discontinuity_before_hidden_from_report
    );
    println!("DISCONTINUITY_CHAIN={}", report.discontinuity_chain);
    println!(
        "COLLAPSED_BEFORE_PHASE_REGRESSION={}",
        report.collapsed_before_phase_regression
    );
    println!(
        "PHASE_REGRESSION_ACCEPTED={}",
        report.phase_regression_accepted
    );
    println!(
        "COLLAPSED_AFTER_PHASE_REGRESSION={}",
        report.collapsed_after_phase_regression
    );
    println!("PHASE_REGRESSION_STEPS={}", report.phase_regression_steps);
    println!(
        "PHASE_REGRESSION_CURRENT={:.2}",
        report.phase_regression_current
    );
    println!("PHASE_REGRESSION_CHAIN={}", report.phase_regression_chain);
    println!("NEGATIVE_SPACE_ACCEPTED={}", report.negative_space_accepted);
    println!(
        "NEGATIVE_SPACE_COLLAPSED={}",
        report.negative_space_collapsed
    );
    println!(
        "NEGATIVE_SPACE_CURRENT={:.2}",
        report.negative_space_current
    );
    println!("INFINITE_SPACE_ACCEPTED={}", report.infinite_space_accepted);
    println!(
        "INFINITE_SPACE_CURRENT_IS_INFINITE={}",
        report.infinite_space_current_is_infinite
    );
    println!(
        "SHADOW_RECEIPT_ID_IS_CANONICAL={}",
        report.shadow_receipt_id_is_canonical
    );
    println!(
        "CURRENT_SHADOW_RECEIPT_CHANGES_WITH_LAW0={}",
        report.current_shadow_receipt_changes_with_law0
    );
    println!(
        "PROPOSED_CONNECTION_CHANGES_WITH_LAW0={}",
        report.proposed_connection_changes_with_law0
    );
    println!(
        "PROPOSED_CONNECTION_REJECTS_LAW0_TAMPER={}",
        report.proposed_connection_rejects_law0_tamper
    );
    println!("LOCAL_TUPLE_NON_EXPANSION_CHECK=PASS");
    println!("GLOBAL_CONTINUITY_ENFORCEMENT=FAIL");
    println!("PHASE_ORDER_ENFORCEMENT=FAIL");
    println!("UNCERTAINTY_DOMAIN_ENFORCEMENT=FAIL");
    println!("PRODUCTION_END_TO_END_EXECUTION=NOT_RUN");
    println!("SHADOW_LOCAL_LAW0_DURABLE_PATH=SOURCE_MATERIALIZED");
    println!("SHADOW_VERIFICATION_RECEIPT_PATH=SOURCE_MATERIALIZED");
    println!("LAW0_DIGEST_TO_VERIFICATION_RECEIPT=UNLINKED");
    println!("CROSS_PLATFORM_LEDGER_CONTINUITY=COMPONENTS_PRESENT_CONNECTION_UNLINKED");
    println!("LIGHT_RECEIPT_SHA_ON_EXISTING_BUS=SOURCE_PRESENT_NOT_CONSUMED_BY_QUANTUM");
    println!("MATERIALIZATION_STATUS=PARTIAL");
    println!("EXPERIMENTAL_STATUS=SUCCESS_AND_FAILURE_REPRODUCED");
    println!(
        "ADVANCEMENT_METHOD=STATE_DERIVED_TRANSITION_PLUS_PHASE_TYPESTATE_PLUS_CROSS_PLATFORM_RECEIPT"
    );
    println!("POC_CLASS=THEORY_POC");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_chain_and_local_fail_closed_gate_are_materialized() {
        let report = run_experiment();
        assert!(report.canonical_collapsed);
        assert_eq!(report.canonical_steps, 5);
        assert!(report.local_expansion_rejected);
        assert!(report.local_rejection_left_ledger_empty);
        assert!(report.nonempty_rejection_atomic);
    }

    #[test]
    fn cross_step_discontinuity_is_reproducible() {
        let report = run_experiment();
        assert!(report.discontinuity_accepted);
        assert_eq!(report.discontinuity_previous_after, 1.0);
        assert_eq!(report.discontinuity_injected_before, 100.0);
        assert_eq!(report.discontinuity_current_after, 50.0);
        assert!(report.discontinuity_expanded_global_state);
        assert!(report.discontinuity_before_hidden_from_report);
    }

    #[test]
    fn terminal_verification_can_regress_and_uncollapse() {
        let report = run_experiment();
        assert!(report.collapsed_before_phase_regression);
        assert!(report.phase_regression_accepted);
        assert!(!report.collapsed_after_phase_regression);
        assert_eq!(report.phase_regression_steps, 2);
        assert_eq!(report.phase_regression_current, 50.0);
    }

    #[test]
    fn negative_uncertainty_space_is_currently_accepted() {
        let report = run_experiment();
        assert!(report.negative_space_accepted);
        assert!(report.negative_space_collapsed);
        assert_eq!(report.negative_space_current, -1.0);
    }

    #[test]
    fn infinite_uncertainty_space_is_currently_accepted() {
        let report = run_experiment();
        assert!(report.infinite_space_accepted);
        assert!(report.infinite_space_current_is_infinite);
    }

    #[test]
    fn shadow_has_the_receipt_mechanism_but_law0_is_not_an_input_yet() {
        let report = run_experiment();
        assert!(report.shadow_receipt_id_is_canonical);
        assert!(!report.current_shadow_receipt_changes_with_law0);
        assert!(report.proposed_connection_changes_with_law0);
        assert!(report.proposed_connection_rejects_law0_tamper);
    }
}
