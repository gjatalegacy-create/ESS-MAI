# ESS-MAI v1.5.9 — Shadow Main Necessary Precondition Audit

## Authority and scope

Baseline: `v1.5.8_ess_mai`.

Authorized correction: make `shadow/src/main.rs` a necessary runtime and build
precondition for access to the Shadow authority, while preserving the existing
PD Continuum, Spine 9, GCL Y→X verification order, receipt binding, Light/Quantum
origin separation, and fail-closed behavior.

No license was added or changed.

## Evidence before the correction

In v1.5.8, Quantum depended directly on the Shadow package as an `rlib` and
constructed the authority in-process. Therefore `shadow/src/main.rs` was not a
necessary condition for `Shadow::new`, `ingest_bridged`, `on_negative`, or the
production of `VerificationReceipt`.

The separate `E0521` compiler error was local to the Quantum ledger call. Its
correct fix remains `source: &'static str`; it did not establish process
mediation by itself.

## Implemented architecture

```text
Light + Quantum state
        ↓
ShadowCycleRequest
        ↓ version/kind/length/FNV corruption check
Quantum process bridge
        ↓ starts child executable
shadow_platform main.rs
        ↓ Phase9 no-bypass check
        ↓ persistent vault open
        ↓ Light PA feed
        ↓ public wire → private internal types
        ↓ Shadow::ingest_bridged
        ↓ GCL Verification Collapse Y→X
        ↓ sovereign token consumption
        ↓ VerificationReceipt
        ↓ NPIM negative persistence in same Shadow instance
ShadowCycleResponse
        ↓ version/kind/length/FNV corruption check
Quantum receipt recomputation
        ↓
PD finalize_after_verification
        ↓
output + iZ → next i₀
```

## Build closure

- `shadow/Cargo.toml` is binary-only with `autolib = false` and no `[lib]`.
- `shadow/src/main.rs` includes `lib.rs`; the core is compiled only inside the
  `shadow_platform` executable.
- Quantum has no dependency on `../shadow` and no `shadow_lib::` or
  `shadow_platform::` core call.
- `shadow_contracts` contains transport forms and codecs only; it has no Shadow
  constructor, vault, token, sovereign seal API, or persistent writer.
- Missing Shadow executable, invalid frame, non-zero child status, missing
  response, or mismatched session stops the Quantum flow fail-closed.

## Test continuity

The historical `shadow/tests/integration.rs` was not discarded. It is included
from `shadow/src/main.rs` under `#[cfg(test)]`, so the same sovereign invariants
are tested inside the binary target without recreating a linkable library.

The former direct-core example was rewritten to document the mediated runtime
path and no longer imports Shadow core.

## Static verification completed

- All modified Cargo manifests parse as TOML.
- All platform/UI package versions are `1.5.9`.
- Modified Rust files passed delimiter/bracket structural scanning.
- `QuantumInboundWire` and internal `QuantumInbound` match in all 29 fields.
- `LightInboundWire` and internal `LightInbound` match in all 6 fields.
- PD Continuum contract is byte-identical across Light/Quantum/Shadow:
  `3b321a035d8ef6358c8d09ade0c00a32d05e5e6ff2068b670935b361991cad56`.
- PD Spine 9 contract is byte-identical across Light/Quantum/Shadow:
  `ae4843ac3395a66b3ccbc0b6a578c56408da85c05f33eb5fcda157e652258c3b`.
- VerificationReceipt contract is byte-identical across Light/Quantum/Shadow:
  `cfb106bf6752b017128ad367592f36718b05738c0ed6114a07f4ea159927b15e`.
- The v1.5.8 `E0521` correction remains present.
- `ess-mai.md` records the theory, implementation, verification, evidence, and
  release status of both v1.5.8 and v1.5.9.

## Verification boundary

The packaging environment does not contain `cargo`, `rustc`, `rustfmt`, or
PowerShell. Therefore the release is statically closed but is not claimed as
Cargo-green. `VALIDATE_V159.ps1` is the authoritative executable proof on the
user's Windows GNU Rust 1.96.0 environment.
