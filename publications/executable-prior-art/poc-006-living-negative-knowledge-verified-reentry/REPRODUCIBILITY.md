# Reproducibility

## Environment used

```text
OS: Windows 10 Pro 10.0.19045
host: x86_64-pc-windows-gnu
rustc: 1.98.0 (88d9e12ae 2026-08-18)
cargo: 1.98.0 (797e8a9bc 2026-08-05)
```

Run from this POC root. Keep the build tree outside the capsule:

```powershell
$env:CARGO_TARGET_DIR = 'C:\path\outside\poc006-target'
rustc --version --verbose
cargo --version --verbose
cargo build --workspace --all-targets --release --locked
cargo test --workspace --all-targets --release --locked -- --test-threads=1
1..3 | ForEach-Object {
  cargo run -p poc006-living-negative-experiment --release --locked
  if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
}
Remove-Item Env:CARGO_TARGET_DIR
```

If dependencies are already cached, `--offline` may be added while preserving
`--locked`. Network retrieval changes availability, not the recorded lockfile.
The expected strict result is build PASS, 30/30 tests, and three identical
runtime receipts with `EXPERIMENT_STATUS=PASS`.

Behavioral reproducibility and bit-for-bit binary reproducibility are separate
claims. Three clean build roots used identical Rust/Cargo inputs and produced
the same semantic receipt, but their Windows executable SHA-256 values were
different. Exact witness hashes are recorded in `RESULTS.md`; byte-identical
binary reproduction is therefore **not established** by this release.

## Source-identity check

```powershell
.\verify_source_identity.ps1 -V189Root 'C:\path\to\v189\ess_mai'
```

The expected result is `SOURCE_IDENTITY_STATUS=PASS` and four matching files.

## Native v189 commands

From the v189 workspace, with an external target directory:

```powershell
cargo test -p quantum-platform pro_nk_gate --locked -- --test-threads=1
cargo test -p shadow_platform --no-default-features --features pure_rust negative_ --locked -- --test-threads=1
cargo test -p shadow_platform --no-default-features --features pure_rust durability_roundtrip --locked -- --test-threads=1
cargo test -p shadow_platform --no-default-features --features pure_rust constructive_pair_is_closed --locked -- --test-threads=1
cargo test -p shadow_platform --no-default-features --features pure_rust rigorous_negative_requires_real_proof --locked -- --test-threads=1
cargo test -p shadow_platform --no-default-features --features pure_rust mixed_pairs_are_structural_refusals --locked -- --test-threads=1
cargo test -p shadow_platform --no-default-features --features pure_rust out_of_domain_bits_are_refused --locked -- --test-threads=1
```

The experiment does not require or authorize modification of the v189 source.
The four final filters are reported separately because one may overlap broader
name filters; they must not be summed as four additional unique tests.

## Reproducible artifact identities

```text
POC Cargo.lock SHA-256:
9375b5e540ec1b283bd242a560d215133a66be27dd83cf9af16a3c90da51ca7d

v189 Cargo.lock SHA-256:
8ea0dccfd6024207438f8a4491e41f88c133113e838b7d1d853fadfeb73f9bbf

detached code source-tree SHA-256:
bf5b5321d3f258e0134320f76dcdb305488323ac166797a56a9c56e13297493f
```

The runtime binary is not retained as primary evidence. Its hash and complete
reproduction command are preserved in `evidence/04_binary_and_source_identity.txt`.
