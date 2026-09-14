# Reproducibility

All build output must remain outside this capsule.

## Toolchain used

```text
rustc 1.98.0 (88d9e12ae 2026-08-18)
cargo 1.98.0 (797e8a9bc 2026-08-05)
host: x86_64-pc-windows-gnu
```

## Build and test

PowerShell:

```powershell
$env:CARGO_TARGET_DIR = "<workspace>\_audit_build_cache\poc005_reproduction"
rustc --version --verbose
cargo --version --verbose
cargo build --workspace --all-targets --release --locked
cargo test --workspace --all-targets --release --locked -- --test-threads=1
1..3 | ForEach-Object {
  cargo run -p poc005-ffi-law0-experiment --release --locked
  if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
}
```

Expected:

```text
build exit = 0
tests = 15 passed, 0 failed
runtime runs = 3 passed, 0 failed
EXPERIMENT_STATUS=PASS
FIRST_VALID_NONEMPTY_CODE=-8
COPIED_HANDLE_REPLAY_CODE=-1
CONCURRENT_COPIES_POST_AUTH=1
CONCURRENT_COPIES_REJECTED=7
COUNTEREXAMPLE_STRONG_SEMANTIC_IDEMPOTENCY=FOUND
```

The strict method first exposed an incomplete authored adapter for the copied
file's `cfg(test)` target. That exit-101 result and the adapter-only closure are
preserved in `evidence/08_standard_protocol_runs.txt`. The three focal v189
files remained byte-identical.

Behavioral reproducibility and bit-for-bit binary reproducibility are separate
claims. Three clean build roots used identical Rust/Cargo inputs and produced
the same 3/3 semantic receipt, but their Windows executable SHA-256 values were
different. Exact witness hashes are recorded in `RESULTS.md`; byte-identical
binary reproduction is therefore **not established** by this release.

## Native-source confirmation

From the v189 workspace, with an external target:

```powershell
$env:CARGO_TARGET_DIR = "<workspace>\_audit_build_cache\poc005_native_shadow"
cargo test -p shadow_platform --no-default-features --features pure_rust sovereign_ffi_gate --locked -- --test-threads=1
```

Expected: 8 passed, 0 failed. This command reads the source and writes only to
the external Cargo target and ordinary OS temporary storage.

## Identity checks

```powershell
.\verify_source_identity.ps1 -V189SourceRoot "<path-to-v189-ess_mai>"
Get-FileHash -Algorithm SHA256 .\Cargo.lock
```

The release binary is not primary evidence and is not stored. Rebuild it using
the exact command above and compare only within a matching toolchain/platform
environment.
