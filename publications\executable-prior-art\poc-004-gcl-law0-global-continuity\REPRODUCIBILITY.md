# Reproducibility

## Prerequisites used

- Windows PowerShell
- Cargo 1.98.0
- rustc 1.98.0
- no C compiler required
- locked `sha2` dependency available in the local Cargo cache
- network disabled by `--offline`; no package download occurs during verification

## Location

Run commands from:

```text
<ESS_MAI_REPOSITORY>/publications/executable-prior-art/poc-004-gcl-law0-global-continuity
```

The workspace contains the extracted production crate and the new experiment as local members. It has no path dependency back into v189.

## Verify extraction

```powershell
.\verify_extraction.ps1
```

Expected end state:

```text
EXTRACTION_IDENTITY=PASS
FILES_VERIFIED=6
```

## Format, clean build, and tests

```powershell
$pocCargo = Join-Path $env:USERPROFILE '.cargo\bin\cargo.exe'
& $pocCargo fmt --all -- --check
& $pocCargo clean
& $pocCargo build --workspace --locked --offline
& $pocCargo test --workspace --locked --offline
```

Expected unit-test result:

```text
gcl-constitution: 11 passed; 0 failed
experiment:       8 passed; 0 failed
total:            19 passed; 0 failed
```

The eight experiment-binary tests comprise two tests carried unchanged in the extracted Shadow receipt module plus six POC harness tests.

`cargo clean` must be invoked only from this POC root; its target is `<POC_ROOT>/target`.

## Five executions

```powershell
1..5 | ForEach-Object {
    & $pocCargo run --quiet --package gcl-law0-global-continuity-experiment --locked --offline
    if ($LASTEXITCODE -ne 0) { throw "POC execution $_ failed" }
}
```

Each execution must end with:

```text
GLOBAL_CONTINUITY_ENFORCEMENT=FAIL
PHASE_ORDER_ENFORCEMENT=FAIL
UNCERTAINTY_DOMAIN_ENFORCEMENT=FAIL
PRODUCTION_END_TO_END_EXECUTION=NOT_RUN
SHADOW_LOCAL_LAW0_DURABLE_PATH=SOURCE_MATERIALIZED
SHADOW_VERIFICATION_RECEIPT_PATH=SOURCE_MATERIALIZED
LAW0_DIGEST_TO_VERIFICATION_RECEIPT=UNLINKED
CROSS_PLATFORM_LEDGER_CONTINUITY=COMPONENTS_PRESENT_CONNECTION_UNLINKED
MATERIALIZATION_STATUS=PARTIAL
EXPERIMENTAL_STATUS=SUCCESS_AND_FAILURE_REPRODUCED
POC_CLASS=THEORY_POC
```

The program output should be byte-identical across the five runs, apart from a possible Cargo environment warning written before program output.

## Hash verification

`evidence/artifact_hashes.sha256` contains SHA-256 for the publication files while excluding:

- `target/`;
- `.git/`;
- the hash manifest itself;
- generated binaries and compiler metadata.

To verify a row manually:

```powershell
Get-FileHash -Algorithm SHA256 -LiteralPath <FILE>
```

## Interpretation

The build demonstrates standalone compilation of the exact extracted crate in this local reproduction. Static source locations outside the extraction are provenance mapping, not executed end-to-end evidence. See `CLAIM_BOUNDARY.md`.
