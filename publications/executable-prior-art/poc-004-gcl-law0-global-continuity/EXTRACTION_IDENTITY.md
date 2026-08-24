# Extraction Identity

## Extraction boundary

Origin crate:

```text
<V189_SOURCE>/gcl-constitution
```

Extracted crate:

```text
extracted/gcl-constitution
```

The complete minimal GCL crate compilation closure consists of five files. `extension.rs` is included because `lib.rs` declares it even though the LAW-0 experiment does not call it.

| Relative file | Bytes | SHA-256 |
|---|---:|---|
| `Cargo.toml` | 277 | `ecf9912f951d481db32022fb0a9d62725d086e3f3748269a97144034799d0b7c` |
| `src/lib.rs` | 457 | `f8441fa4dd3d4981d9c85420dd4192b6da3db09517b0270c0dede94a977e5d85` |
| `src/constitution.rs` | 23613 | `2ea2645c63d8ca617b3659d2f8480f11f2375845b16e81eb43808a181088807b` |
| `src/phase.rs` | 679 | `8047c0191c0a4c72b648696d06c8f5584a2ce1ae897602c807dcb9da46648881` |
| `src/extension.rs` | 757 | `79c204190c6ab257fb827060cf7f8ffd620da333c6ec9903b89f20818f4d2a9c` |

A sixth production file is extracted independently from Shadow and compiled by the experiment through a path module:

| Origin-relative file | Extracted file | Bytes | SHA-256 |
|---|---|---:|---|
| `shadow/src/lab_contracts/verification_receipt.rs` | `extracted/shadow_verification_receipt.rs` | 4283 | `93f010fbd3171c8ec8f452e3f7332df5c529eaefa7ce6afd910200b0381b8d9c` |

For every row:

```text
source_bytes == extracted_bytes
source_sha256 == extracted_sha256
```

Result: `6/6 BYTE_IDENTICAL`.

## Method

The source files were copied with their bytes preserved. Identity was then checked using SHA-256 independently on each origin and extracted file. `EXTRACTION_MANIFEST.sha256` is the machine-readable expected-hash list, and `verify_extraction.ps1` fails if any extracted file drifts.

## What was not copied

- full Light, Quantum, or Shadow source trees beyond the single Shadow receipt algorithm listed above;
- production binaries;
- Python tools under the origin crate;
- Cargo target artifacts;
- archives or unrelated documentation.

Production-use evidence outside these six exact files is referenced by path and line in `SOURCE_MAP.md`; it was not repackaged as executable source.

## Identity boundary

Byte identity is a provenance fact, not a claim that the code fully implements the theory. The new experiment source and its harness-only connection hash are intentionally separate and have no byte-identity claim.
