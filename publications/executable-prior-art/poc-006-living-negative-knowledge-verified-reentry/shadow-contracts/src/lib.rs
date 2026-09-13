//! Surgical transport-only closure for POC-006.
//!
//! The focal `negative_asset.rs` is byte-identical to v189. This crate root
//! exposes only that module and grants no Shadow authority or persistence.

pub mod negative_asset;
pub use negative_asset::*;

pub const MASS_SCALE: u32 = 10_000;

pub fn fixed_mass(value: f32) -> Option<u32> {
    match value.is_finite() {
        true => Some((value.clamp(0.0, 1.0) * MASS_SCALE as f32) as u32),
        false => None,
    }
}
