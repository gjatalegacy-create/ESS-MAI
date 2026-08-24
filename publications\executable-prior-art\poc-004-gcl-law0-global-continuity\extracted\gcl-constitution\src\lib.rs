//! Single physical source of the existing ESS-MAI GCL constitutional contract.
//!
//! This crate is a contract source, not a fourth runtime platform, orchestrator,
//! verifier, court, vault, or terminal adjudicator. Light, Quantum, and Shadow
//! retain their existing roles and consume this same source through local
//! compatibility re-exports.

mod constitution;
mod phase;

pub mod extension;

pub use constitution::*;
pub use phase::CollapsePhase;
