// PHASE 2 — compatibility surface only.
// The sole physical GCL constitutional source is the gcl-constitution crate.
// This module preserves every existing platform-local import path without
// creating a platform-local registry, state machine, or authority.
pub use gcl_constitution::*;
