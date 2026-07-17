# ESS-MAI v1.5.8

## Scope

Evidence-only correction over v1.5.7.

## Implemented

- Corrected `E0521` in `quantum/src/main.rs` by narrowing the source-site contract from `&str` to `&'static str`.
- Verified both callers pass static literals.
- Preserved `VerificationReceipt`, PD Continuum, `output + iZ → next i₀`, GCL authority, and the Shadow verification seal.
- Normalized the project SHA-256 manifest to relative paths so the Windows validator can resolve files from the project root.

## Verification status

The original Windows Cargo logs identify E0521 as the single blocking compiler error. A fresh Cargo run remained required after packaging.
