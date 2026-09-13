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
cargo build --workspace --release --locked --offline
cargo test --workspace --locked --offline -- --test-threads=1
cargo run -p poc005-ffi-law0-experiment --release --locked --offline
```

Expected:

```text
build exit = 0
tests = 1 passed, 0 failed
runtime exit = 0
EXPERIMENT_STATUS=PASS
FIRST_VALID_NONEMPTY_CODE=-8
COPIED_HANDLE_REPLAY_CODE=-1
CONCURRENT_COPIES_POST_AUTH=1
CONCURRENT_COPIES_REJECTED=7
```

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

