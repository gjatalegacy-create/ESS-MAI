# Reproducibility

## Tested environment

- Windows PowerShell;
- Cargo 1.98.0;
- rustc 1.98.0;
- host `x86_64-pc-windows-gnu`;
- locked dependency graph;
- network disabled by `--offline`;
- build target outside the publication capsule.

Run from:

```text
<ESS_MAI_REPOSITORY>/publications/executable-prior-art/poc-003-system-cold-start-reachability
```

## Verify disclosed extraction

```powershell
.\scripts\verify_extraction.ps1
```

Expected:

```text
EXTRACTION_IDENTITY=PASS
WHOLE_FILES_VERIFIED=20
EXCERPTS_VERIFIED=1
```

The script verifies the disclosed files against this capsule's manifest. Origin comparison was performed locally against the private v1.8.9 baseline; a reviewer needs that same baseline to repeat origin comparison independently.

## Build and test without writing generated artifacts into the capsule

```powershell
$pocCargo = Join-Path $env:USERPROFILE '.cargo\bin\cargo.exe'
$targetDir = Join-Path $env:TEMP ('ess-mai-poc003-' + [guid]::NewGuid().ToString('N'))
$env:CARGO_TARGET_DIR = $targetDir

& $pocCargo build --workspace --all-targets --locked --offline
if ($LASTEXITCODE -ne 0) { throw 'POC003 build failed' }

& $pocCargo test --workspace --locked --offline -- --test-threads=1
if ($LASTEXITCODE -ne 0) { throw 'POC003 tests failed' }
```

Expected total:

```text
84 passed; 0 failed
```

## Run the empty-state experiment and positive control

```powershell
& $pocCargo build --workspace --bins --locked --offline
if ($LASTEXITCODE -ne 0) { throw 'POC003 binary build failed' }

$pocExe = Join-Path $targetDir 'debug\ess-mai-system-poc-003.exe'

1..3 | ForEach-Object {
    & $pocExe --empty-cold-start
    if ($LASTEXITCODE -ne 0) { throw "empty run $_ failed" }
}

& $pocExe --exact-pair-positive-control
if ($LASTEXITCODE -ne 0) { throw 'positive control failed' }
```

After verification:

```powershell
Remove-Item Env:CARGO_TARGET_DIR
Remove-Item -LiteralPath $targetDir -Recurse -Force
```

The cleanup target is the explicit fresh temporary directory created above, never the repository or project root.

## Formatting identity boundary

Do not run `cargo fmt` as a mutating command over the capsule. Current rustfmt proposes formatting changes to several byte-identical production extracts. Applying them would destroy extraction identity.

`cargo fmt --all -- --check` therefore reports diffs and is recorded as `NOT_A_RELEASE_GATE_FOR_EXACT_EXTRACTS`. This does not change the Cargo build/test result.

## Runtime trust boundary

The extracted clients honor `ESSMAI_SHADOW_BIN` and `ESSMAI_HANDOFF_DIR`. The harness sets them to its freshly built sibling selector and a unique temporary directory. Do not reuse this experiment with untrusted executable paths or shared writable handoff directories.

## Interpretation

The public POC executes a bounded multi-process selection path. It does not execute the full private production binaries or final production Shadow commit. See `CLAIM_BOUNDARY.md`.
