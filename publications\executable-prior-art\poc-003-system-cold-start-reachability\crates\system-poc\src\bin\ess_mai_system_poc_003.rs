//! New command-line harness for ESS-MAI SYSTEM POC 003.

use ess_mai_system_poc_003::{run_experiment, ExperimentMode};

fn shadow_binary_name() -> &'static str {
    #[cfg(windows)]
    {
        "shadow_platform.exe"
    }
    #[cfg(not(windows))]
    {
        "shadow_platform"
    }
}

fn main() {
    let mode = match std::env::args().nth(1).as_deref() {
        None | Some("--empty-cold-start") => ExperimentMode::EmptyColdStart,
        Some("--exact-pair-positive-control") => ExperimentMode::ExactPairPositiveControl,
        Some(other) => {
            eprintln!("unknown mode: {other}");
            std::process::exit(2);
        }
    };
    let current = match std::env::current_exe() {
        Ok(value) => value,
        Err(error) => {
            eprintln!("current executable lookup failed: {error}");
            std::process::exit(2);
        }
    };
    let shadow = match current.parent() {
        Some(parent) => parent.join(shadow_binary_name()),
        None => {
            eprintln!("POC executable has no parent directory");
            std::process::exit(2);
        }
    };
    let handoff = std::env::temp_dir().join(format!(
        "ess-mai-poc003-{}-{}",
        std::process::id(),
        mode.label().to_ascii_lowercase()
    ));
    let result = run_experiment(mode, &shadow, &handoff);
    match result {
        Ok(report) => println!("{report}"),
        Err(error) => {
            eprintln!("POC-003 failed: {error}");
            let _ = std::fs::remove_dir_all(&handoff);
            std::process::exit(1);
        }
    }
    let _ = std::fs::remove_dir_all(&handoff);
}

