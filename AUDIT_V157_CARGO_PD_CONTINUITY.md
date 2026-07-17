# ESS-MAI v1.5.7 — Cargo and PD Continuity Audit

## Authority and scope

This audit preserves the governing hierarchy:

`Royal Intelligent Systems (RIS) -> NurAtomic Architecture -> ESS-MAI`

Only findings supported by the supplied Cargo reports and the v1.5.7 source tree were implemented. `ess-mai.md` was not modified because this patch closes a compiler contract and the file-identity validation path; it does not introduce or complete a new theory.

## Evidence set

- `cargo check --workspace --all-targets`: exit 101.
- `cargo clippy --workspace --all-targets -- -W clippy::all`: exit 101.
- `cargo test --workspace --all-targets --no-run`: exit 101.
- `cargo fmt --all -- --check`: exit 1.
- Rust/Cargo evidence environment: Rust 1.96.0, Windows GNU target.

## Finding F-001 — compiler blocker E0521

**Status:** IMPLEMENTED; requires Cargo re-verification in the Windows build environment.

`quantum/src/main.rs::export_pd_verified_line` accepted `source: &str` and passed it to `lab_contracts::rrjedha::note`, whose `site` contract is `&'static str`. Rust therefore rejected the binary target because a function-local borrow could escape into a static ledger-site contract.

### Flow verification

The helper has exactly two callers:

- `export_pd_probe` passes the literal `"main::export_pd_probe"`.
- `export_pd_handoff` passes the literal `"main::export_pd_handoff"`.

No dynamic or user-controlled value reaches `source`. The correct contract is therefore `source: &'static str`.

### Implemented change

```rust
source: &'static str,
```

This narrows the helper contract to the already-existing call graph. It does not alter the handoff body, verification receipt, CRC, PD state transition, or ledger behavior.

## Finding F-002 — SHA-256 manifest path incompatibility

**Status:** IMPLEMENTED and statically verified.

`VALIDATE_V157.ps1` treats each manifest path as relative to the project root. The supplied `ESS_MAI_V1_5_7_FILELIST.sha256` stored absolute packaging paths beginning with `/mnt/data/essmai_v157_work/...`. On Windows these become invalid targets under the project root and validation reports every entry as missing.

The original 356 hashes were verified against the pristine archive after removing the stale packaging prefix: 356 present, 0 missing, 0 mismatched. The manifest was then regenerated with sorted project-relative paths and current SHA-256 values, excluding the manifest itself to avoid recursive identity.

## PD Continuum execution map

```text
Light input
  -> i + U
  -> canonical i0 / Primitive Anchor
  -> light_pa_export.txt
  -> Quantum read_primitive_context
  -> PD open_session_sealed
  -> ingest_for_spine_sealed
  -> PdSpineRequest + PdContinuumActivation
  -> Spine 9
       -> Layer 1 receipt
       -> Layer 2 receipt chained to Layer 1
       -> Layer 3 receipt chained to Layer 2
  -> prepare_after_spine_sealed
  -> PendingNextI0 + pre-seal + GCL authorization token
  -> QuantumInbound to Shadow
  -> Shadow PA waiting-anchor consume
  -> VerificationContext(parent_i0, PA, XY, PD binding, activation)
  -> GCL / sovereign verification seal
  -> VerificationReceipt
  -> Quantum finalize_after_verification
       -> 1/1: VerifiedPositive
       -> 0/0: VerifiedNegativeRebuild
       -> other pair: rejected
  -> PdVerifiedOutput + PdIzCompletion
  -> deterministic next i0
  -> quantum_pd_export.txt (25 body fields + CRC)
  -> Light verify wire + receipt + output + iZ + completion
  -> VerifiedPdSurface
  -> Nura speech surface
```

## Cross-platform contract identity

The following files were byte-identical before this patch and remain unmodified:

- `pd_continuum_contract.rs` across Light/Quantum/Shadow:
  `3b321a035d8ef6358c8d09ade0c00a32d05e5e6ff2068b670935b361991cad56`
- `pd_spine_contract.rs` across Light/Quantum/Shadow:
  `ae4843ac3395a66b3ccbc0b6a578c56408da85c05f33eb5fcda157e652258c3b`
- `lab_contracts/verification_receipt.rs` across Light/Quantum/Shadow:
  `cfb106bf6752b017128ad367592f36718b05738c0ed6114a07f4ea159927b15e`

## Cargo warning classification

The supplied `cargo check` report contains one unique compiler error: E0521. It also contains unused-import, unused-mut, and dead-code warnings. These warnings were not mass-edited because:

1. they do not block compilation;
2. some imports/constants may serve optional feature or legacy compatibility paths;
3. automatic cleanup without feature-by-feature proof could change architectural availability;
4. `cargo fix` was not authorized.

The Clippy report is dominated by style recommendations that replace `match` with `if`/`if let`, plus unwrap and complexity warnings. Those were not applied because the ESS-MAI architecture explicitly uses match-driven state transitions and forbids speculative style rewrites.

The format check reports broad formatting drift. Running `cargo fmt` over the full project would create a very large non-semantic diff and obscure the evidence-backed patch, so formatting was left unchanged.

## Static verification performed

- E0521 source/callee lifetime contract traced.
- All callers of the changed helper inspected.
- No unresolved module/import/visibility error reported.
- No dependency/build/linker error reported.
- Cargo check, Clippy, and test-compile reports all stop on the same E0521 blocker.
- PD wire field count and Light parser agreement inspected.
- Shadow receipt binds session, parent i0, PA, XY digest, PD candidate digest, continuum activation, Y/X, generation, and seal.
- Light recomputes receipt, PD output digest, iZ digest, next-i0 identity, and continuum completion before creating `VerifiedPdSurface`.
- Cross-platform contract files hashed byte-for-byte.
- Regenerated manifest paths are relative and all listed hashes validate against the patched tree.

## Verification boundary

Cargo/rustc are not installed in the packaging environment used for this patch. Therefore no new Cargo-green claim is made here. The code change is compiler-directed and statically closed, but the authoritative executable verification must be rerun in the supplied Windows GNU environment.

## Required Windows verification

From the project root:

```powershell
cargo check --workspace --all-targets
cargo test --workspace --all-targets --no-run
cargo clippy --workspace --all-targets -- -W clippy::all
.\VALIDATE_V157.ps1
```

Expected first closure criterion: E0521 must disappear. Remaining warnings must be audited separately and must not be treated as permission for automatic architectural rewrites.
