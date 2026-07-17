# ESS-MAI v1.5.9 — Shadow Main Necessary Precondition

## Objective

Make `shadow/src/main.rs` a real, non-bypassable process boundary for communication with the Shadow authority.

## Architectural correction

Before v1.5.9, Quantum linked `shadow_platform` as an `rlib` and directly called `Shadow::new`, `ingest_bridged`, and `on_negative`. Shadow main.rs was therefore not a necessary precondition.

v1.5.9 changes the build and runtime model:

- `shadow/Cargo.toml`: `autolib=false`; no `[lib]`; only `shadow_platform` binary target.
- `shadow/src/main.rs`: includes `lib.rs`, so the core compiles only inside the executable.
- `quantum/Cargo.toml`: removes the Shadow core dependency.
- New `shadow_contracts` crate: public wire shapes and deterministic checksummed codec only.
- Quantum spawns the Shadow executable for each mediated cycle.
- Shadow main opens the persistent vault, feeds the PA gate, calls the internal core, seals the receipt, persists NPIM negative knowledge, and returns a checksummed response.
- Missing Shadow binary, non-zero process exit, corrupted response, or session mismatch is fail-closed.

## Preserved contracts

- Light and Quantum origins remain separate until Shadow.
- GCL order Y → X is unchanged.
- `VerificationReceipt` binding fields remain unchanged.
- PD finalization still re-computes and verifies the receipt before releasing `output + iZ → next i₀`.
- The v1.5.8 E0521 correction remains in place.

## Verification status

Static architecture checks passed in the packaging environment. Cargo/rustc were unavailable there, so Windows GNU Cargo verification is mandatory through `VALIDATE_V159.ps1`.

## Test and documentation continuity

- The former external `shadow/tests/integration.rs` is retained and included by `shadow/src/main.rs` under `#[cfg(test)]`; it now verifies the same sovereign invariants inside the binary target rather than through a linkable library.
- `shadow/examples/full_flow.rs` no longer imports Shadow core; it documents the mediated runtime command path.
- `shadow/README.md`, `README_INSTALIMI.md`, and the generated launcher comment now describe main.rs as the necessary runtime condition.
