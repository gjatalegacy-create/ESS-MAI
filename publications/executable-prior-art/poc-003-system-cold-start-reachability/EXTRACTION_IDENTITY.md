# Extraction Identity

## Whole-file closure

Twenty disclosed files match the private v1.8.9 baseline by relative role, byte count, and SHA-256:

```text
Rust source files: 18
existing Cargo manifests: 2
whole-file mismatches: 0
```

The capsule paths are individually sealed in `EXTRACTION_MANIFEST.sha256`.

## Exact Shadow excerpt

```text
origin role: shadow/src/knowledge_vault.rs
origin lines: 777-900
capsule carrier: crates/system-poc/src/shadow_projection.rs
capsule lines: 70-193
line count: 124
normalized-LF SHA-256:
ac838a0c0736bfed88f5a00318cb51aa001e8042e66f4695cec7ac65c90c2204
```

The excerpt is exact. The surrounding reduced empty-store shell is new POC glue and is not represented as production source.

## Verification command

```powershell
.\scripts\verify_extraction.ps1
```

Expected:

```text
EXTRACTION_IDENTITY=PASS
WHOLE_FILES_VERIFIED=20
EXCERPTS_VERIFIED=1
```

The public verifier freezes the disclosed extraction. It does not disclose the private origin files and cannot independently reconstruct them.

## Source-mutation rule

The 20 whole-file extracts and the 124-line excerpt must not be formatted, cleaned up, or edited. Any future production change must create a new capsule version and new origin comparison, not rewrite this evidence.
