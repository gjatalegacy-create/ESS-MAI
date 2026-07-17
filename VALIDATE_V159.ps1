param(
    [string]$LogRoot = (Join-Path ([Environment]::GetFolderPath('Desktop')) 'ESS_MAI_V159_LOGS')
)

$ErrorActionPreference = 'Stop'
[Console]::OutputEncoding = [System.Text.UTF8Encoding]::new()
$Root = Split-Path -Parent $MyInvocation.MyCommand.Path
New-Item -ItemType Directory -Force -Path $LogRoot | Out-Null

function Run-Logged {
    param([string]$Name, [scriptblock]$Command)
    $Path = Join-Path $LogRoot ($Name + '.log')
    & $Command 2>&1 | Tee-Object -FilePath $Path
    if ($LASTEXITCODE -ne 0) {
        throw "$Name failed with exit code $LASTEXITCODE. See $Path"
    }
}

function Require-Text {
    param([string]$Path, [string]$Pattern, [string]$Label)
    $Text = Get-Content -LiteralPath $Path -Raw -Encoding UTF8
    if ($Text -notmatch $Pattern) { throw "STATIC GUARD FAILED: $Label" }
}

function Reject-Text {
    param([string]$Path, [string]$Pattern, [string]$Label)
    $Text = Get-Content -LiteralPath $Path -Raw -Encoding UTF8
    if ($Text -match $Pattern) { throw "STATIC GUARD FAILED: $Label" }
}

Push-Location $Root
try {
    Run-Logged '00_toolchain' { cargo --version; rustc --version }

    # Architecture guards: these fail before compilation if complete mediation regresses.
    Require-Text (Join-Path $Root 'shadow\Cargo.toml') 'autolib\s*=\s*false' 'Shadow must remain binary-only'
    Reject-Text  (Join-Path $Root 'shadow\Cargo.toml') '(?m)^\[lib\]' 'Shadow [lib] target must not return'
    Reject-Text  (Join-Path $Root 'shadow\Cargo.toml') 'rlib|staticlib' 'Shadow linkable crate-types must not return'
    Require-Text (Join-Path $Root 'shadow\src\main.rs') 'include!\("lib\.rs"\)' 'main.rs must include the Shadow constitution'
    Require-Text (Join-Path $Root 'shadow\src\main.rs') 'process_bridge::dispatch_from_args' 'main.rs must own process mediation'
    Require-Text (Join-Path $Root 'shadow\src\main.rs') '\#\[path\s*=\s*"\.\./tests/integration\.rs"\]' 'Shadow integration invariants must remain inside the binary test target'
    Reject-Text  (Join-Path $Root 'shadow\examples\full_flow.rs') 'use\s+shadow_platform|shadow_platform::' 'Examples must not restore direct-core access'
    Require-Text (Join-Path $Root 'quantum\Cargo.toml') 'shadow_contracts\s*=\s*\{\s*path\s*=\s*"\.\./shadow-contracts"' 'Quantum must depend only on public Shadow contracts'
    Reject-Text  (Join-Path $Root 'quantum\Cargo.toml') 'path\s*=\s*"\.\./shadow"|package\s*=\s*"shadow_platform"' 'Quantum must not link Shadow core'
    Reject-Text  (Join-Path $Root 'quantum\src\main.rs') 'shadow_lib::|shadow_platform::' 'Quantum main must not call Shadow core'
    Reject-Text  (Join-Path $Root 'quantum\src\shadow_process_bridge.rs') 'shadow_lib::|shadow_platform::' 'Quantum process bridge must not call Shadow core'
    Require-Text (Join-Path $Root 'quantum\src\main.rs') "source:\s*&'static\s+str" 'E0521 fix must remain'
    Require-Text (Join-Path $Root 'quantum\src\shadow_process_bridge.rs') 'Command::new\(&shadow_bin\)' 'Quantum must execute Shadow main.rs'
    Require-Text (Join-Path $Root 'shadow\src\process_bridge.rs') 'ingest_bridged' 'Shadow main mediation must call internal verification core'
    Require-Text (Join-Path $Root 'shadow-contracts\src\lib.rs') 'InvalidChecksum' 'Wire checksum guard must exist'

    'STATIC_GUARDS_OK' | Set-Content -LiteralPath (Join-Path $LogRoot '01_static_guards.log') -Encoding UTF8

    Run-Logged '02_build_workspace_all_targets' { cargo build --workspace --all-targets }
    Run-Logged '03_check_workspace_all_targets' { cargo check --workspace --all-targets }
    Run-Logged '04_test_compile_only' { cargo test --workspace --all-targets --no-run }
    Run-Logged '05_test_workspace' { cargo test --workspace }
    Run-Logged '06_clippy_workspace' { cargo clippy --workspace --all-targets -- -W clippy::all }

    Push-Location (Join-Path $Root 'ui')
    try { Run-Logged '07_check_new_ui' { cargo check --all-targets } }
    finally { Pop-Location }

    Push-Location (Join-Path $Root 'light\ui\src-tauri')
    try { Run-Logged '08_check_old_ui_emotional_engine' { cargo check --all-targets } }
    finally { Pop-Location }

    $ShadowExe = Join-Path $Root 'target\debug\shadow_platform.exe'
    if (-not (Test-Path -LiteralPath $ShadowExe)) {
        throw "Shadow main executable missing after workspace build: $ShadowExe"
    }
    "SHADOW_MAIN_PRESENT $ShadowExe" | Set-Content -LiteralPath (Join-Path $LogRoot '09_shadow_main_present.log') -Encoding UTF8

    $Manifest = Join-Path $Root 'ESS_MAI_V1_5_9_FILELIST.sha256'
    $Failures = @()
    foreach ($Line in Get-Content -LiteralPath $Manifest -Encoding UTF8) {
        if ($Line -match '^([0-9a-f]{64})  (.+)$') {
            $Expected = $Matches[1]
            $Relative = $Matches[2].Replace('/', [IO.Path]::DirectorySeparatorChar)
            $Target = Join-Path $Root $Relative
            if (-not (Test-Path -LiteralPath $Target)) {
                $Failures += "MISSING $Relative"
                continue
            }
            $Actual = (Get-FileHash -LiteralPath $Target -Algorithm SHA256).Hash.ToLowerInvariant()
            if ($Actual -ne $Expected) { $Failures += "HASH $Relative" }
        }
    }
    $Failures | Set-Content -LiteralPath (Join-Path $LogRoot '10_hash_failures.log') -Encoding UTF8
    if ($Failures.Count -ne 0) { throw "File-list verification failed: $($Failures.Count)" }
    'HASH_OK' | Set-Content -LiteralPath (Join-Path $LogRoot '10_hash_ok.log') -Encoding UTF8

    Write-Host "ESS-MAI v1.5.9 validation completed. Logs: $LogRoot"
}
finally {
    Pop-Location
}
