# Source Map and Extraction Boundary

Private source baseline:

```text
<V189_SOURCE>/ = v1.8.9 ESS-MAI source root
```

The private path itself is not published. C01/C02 are not evidence.

## Whole-file production extracts

`EXTRACTION_MANIFEST.sha256` records origin path, capsule path, byte size, and SHA-256 for 20 whole-file matches:

### Shared GCL

- `gcl-constitution/Cargo.toml`
- `gcl-constitution/src/constitution.rs`
- `gcl-constitution/src/extension.rs`
- `gcl-constitution/src/lib.rs`
- `gcl-constitution/src/phase.rs`

### Public Shadow contracts

- `shadow-contracts/Cargo.toml`
- `shadow-contracts/src/lib.rs`
- `shadow-contracts/src/negative_asset.rs`

### Quantum

- `quantum/src/candidate_weight.rs`
- `quantum/src/lab_contracts/collapse.rs`
- `quantum/src/lab_contracts/gjata_collapse_law.rs`
- `quantum/src/lab_contracts/pa_wire.rs`
- `quantum/src/request_bound_relevance.rs`
- `quantum/src/ultimatum_collapse_law.rs`

### Light / Quantum / Shadow process seams

- `light/src/alnur_karina_athar.rs`
- `quantum/src/asht_quantum.rs`
- `light/src/besa_nlight.rs`
- `shadow/src/selection_hold.rs`
- `quantum/src/shadow_process_bridge.rs`
- `light/src/shadow_selection_bridge.rs`

## Exact excerpt

```text
origin:  <V189_SOURCE>/shadow/src/knowledge_vault.rs:777-900
capsule: crates/system-poc/src/shadow_projection.rs:70-193
lines:   124
normalized-LF SHA-256:
ac838a0c0736bfed88f5a00318cb51aa001e8042e66f4695cec7ac65c90c2204
```

The excerpt is the read-only bounded candidate projection. The surrounding `KnowledgeVault` empty-store shell and entry types in `shadow_projection.rs` are new harness glue.

## New POC glue

- root workspace `Cargo.toml` and `Cargo.lock`;
- `crates/quantum-surgical/Cargo.toml` and its module `lib.rs` / `lab_contracts/mod.rs`;
- `crates/system-poc/Cargo.toml`;
- compatibility shells `knowledge_lineage.rs`, `lab_contracts.rs`, and `lgc_algorithm.rs`;
- `shadow_adapter.rs` and the non-excerpt portions of `shadow_projection.rs`;
- `experiment.rs`;
- both POC binaries and the integration test.

The 773-byte `lgc_algorithm.rs` is a new public hash shim. The full private Light file is deliberately excluded because it contains material outside the surgical publication boundary.

## Process boundary

The extracted Light and Quantum bridge clients launch the new surgical `shadow_platform` binary using the existing `--selection-once <request> <response>` contract. This executes a real separate process and real wire encoding/verification, but not the full production Shadow executable.

## Trust boundary

`ESSMAI_SHADOW_BIN` selects the executable and `ESSMAI_HANDOFF_DIR` selects writable handoff storage. The POC harness binds both to per-run temporary paths and deletes them after each run. A production deployment would require trusted binary identity, directory ownership/ACL controls, and stronger process-launch policy.

## Explicit exclusions

- full private Light `lgc_algorithm.rs`;
- full private Shadow `knowledge_vault.rs`;
- unsafe/FFI storage closure;
- production Shadow writer/transaction/WAL core;
- all full-workspace manifests and structural hashes;
- C01/C02;
- generated targets, binaries, hold/WAL files, and private paths.
