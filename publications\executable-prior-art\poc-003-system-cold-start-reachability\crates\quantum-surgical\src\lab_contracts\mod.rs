//! New compatibility shell exposing only contracts required by the surgical POC.

pub mod collapse;
pub mod gjata_collapse_law;
pub mod pa_wire;

pub use collapse::{CollapseOutcome, CollapsePhase, NegativePath, PrimitiveSplit};
pub use gjata_collapse_law::{
    system_laws_seal, GjataCollapseLaw, SupremeDirective,
};

