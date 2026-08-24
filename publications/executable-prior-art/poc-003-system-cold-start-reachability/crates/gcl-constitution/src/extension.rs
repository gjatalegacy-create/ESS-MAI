//! Typed extension boundary reserved for later ratified GCL phases.
//!
//! Phase 2 intentionally defines no HOLD, Living Negative, Self-Critique,
//! Edge-Integrity, or Constitutional-Closure semantics. Implementations added
//! by later phases must remain subordinate to GCL and must be bound through a
//! new freeze capsule.

/// Generic typed port for a future ratified constitutional clause.
///
/// The trait creates no authority and performs no side effect. It only fixes
/// the type relationship that later clauses must implement without modifying
/// the frozen Phase 2 registry or platform adapters.
pub trait ConstitutionalExtension {
    type Evidence;
    type Outcome;

    fn evaluate(&self, evidence: &Self::Evidence) -> Self::Outcome;
}
