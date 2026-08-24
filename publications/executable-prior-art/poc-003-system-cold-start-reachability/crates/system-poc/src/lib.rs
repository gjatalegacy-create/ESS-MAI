//! ESS-MAI SYSTEM POC 003 surgical runtime shell.
//!
//! Files named in `EXTRACTION_MANIFEST.sha256` are byte-identical production
//! extracts. This file and the modules explicitly marked as harness are new POC
//! glue and are not represented as v1.8.9 production source.

pub mod alnur_karina_athar;
pub mod asht_quantum;
pub mod besa_nlight;
pub mod experiment;
pub mod lab_contracts;
pub mod lgc_algorithm;

mod knowledge_lineage;
mod selection_hold;
mod shadow_adapter;
mod shadow_process_bridge;
mod shadow_projection;
mod shadow_selection_bridge;

pub use experiment::{
    authority_wrong_phase_is_rejected, run_experiment, ExperimentMode, ExperimentReport,
};
pub use shadow_adapter::run_shadow_selector_once;

