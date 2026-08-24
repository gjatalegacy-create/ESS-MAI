use ess_mai_system_poc_003::{
    authority_wrong_phase_is_rejected, run_experiment, ExperimentMode,
};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

fn shadow_binary() -> &'static Path {
    Path::new(env!("CARGO_BIN_EXE_shadow_platform"))
}

fn handoff(label: &str) -> PathBuf {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    std::env::temp_dir().join(format!(
        "ess-mai-poc003-test-{}-{}-{}",
        std::process::id(),
        COUNTER.fetch_add(1, Ordering::Relaxed),
        label
    ))
}

fn run(mode: ExperimentMode, label: &str) -> ess_mai_system_poc_003::ExperimentReport {
    let directory = handoff(label);
    let report = run_experiment(mode, shadow_binary(), &directory).expect("POC experiment");
    let _ = std::fs::remove_dir_all(directory);
    report
}

#[test]
fn gcl_rejects_wrong_phase_authority() {
    assert!(authority_wrong_phase_is_rejected());
}

#[test]
fn fresh_shadow_process_yields_two_complete_empty_selections() {
    let report = run(ExperimentMode::EmptyColdStart, "two-empty-selections");
    assert_eq!(report.shadow_selection_processes, 2);
    assert_eq!(report.besa_positive_count, 0);
    assert_eq!(report.besa_negative_count, 0);
}

#[test]
fn besa_accepts_the_complete_empty_selection_under_gcl() {
    let report = run(ExperimentMode::EmptyColdStart, "besa-empty");
    assert!(report.gcl_parent_authority);
    assert!(report.besa_empty_selection_accepted);
}

#[test]
fn asht_fails_closed_on_missing_exact_positive_candidate() {
    let report = run(ExperimentMode::EmptyColdStart, "asht-fail-closed");
    assert!(!report.asht_succeeded);
    assert_eq!(
        report.asht_error.as_deref(),
        Some("request-bound relevance found no exact positive candidate")
    );
}

#[test]
fn empty_cold_start_never_reaches_the_post_asht_shadow_probe() {
    let report = run(ExperimentMode::EmptyColdStart, "probe-unreached");
    assert!(!report.post_asht_probe_reached);
    assert!(!report.production_shadow_commit_executed);
}

#[test]
fn exact_pair_control_reaches_post_asht_then_empty_mode_stays_empty() {
    let control = run(
        ExperimentMode::ExactPairPositiveControl,
        "positive-control",
    );
    assert!(control.asht_succeeded);
    assert!(control.post_asht_probe_reached);
    assert!(!control.production_shadow_commit_executed);

    let cold = run(ExperimentMode::EmptyColdStart, "empty-after-control");
    assert!(cold.besa_empty_selection_accepted);
    assert!(!cold.post_asht_probe_reached);
}

