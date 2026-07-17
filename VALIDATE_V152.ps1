$ErrorActionPreference = "Continue"
$Root = Split-Path -Parent $MyInvocation.MyCommand.Path
$Out = Join-Path $Root "validation_v152"
New-Item -ItemType Directory -Force -Path $Out | Out-Null
Set-Location $Root

function Run-Step {
    param([string]$Name, [string]$Command)
    $Log = Join-Path $Out ($Name + ".txt")
    cmd.exe /d /s /c "$Command > `"$Log`" 2>&1"
    return $LASTEXITCODE
}

# Porta kryesore që zbuloi 6 gabimet e v1.5.1.
$AllTargets = Run-Step "01_build_workspace_all_targets" "cargo --color never build --workspace --all-targets"

# Testet e platformave me feature-matrix të vlefshme.
$LightDefault = Run-Step "02_light_tests_default" "cargo --color never test -p light-platform"
$LightDev = Run-Step "03_light_tests_dev_simulation" "cargo --color never test -p light-platform --no-default-features --features dev_simulation"
$QuantumLib = Run-Step "04_quantum_lib_tests" "cargo --color never test -p quantum-platform --lib"
$ShadowPure = Run-Step "05_shadow_tests_pure_rust" "cargo --color never test -p shadow_platform --no-default-features --features pure_rust"
$Workspace = Run-Step "06_workspace_tests" "cargo --color never test --workspace"
$Release = Run-Step "07_release_build" "cargo --color never build --workspace --release"

# Mos aktivizo të gjitha feature-t njëherësh: Shadow e ndalon kombinimin runtime_mode + pure_rust.
$ClippyDefault = Run-Step "08_clippy_workspace" "cargo --color never clippy --workspace --all-targets"
$ClippyShadowPure = Run-Step "09_clippy_shadow_pure_rust" "cargo --color never clippy -p shadow_platform --all-targets --no-default-features --features pure_rust"

$Pairs = @(
    [pscustomobject]@{ Name = "BUILD_ALL_TARGETS"; Exit = $AllTargets }
    [pscustomobject]@{ Name = "LIGHT_TEST_DEFAULT"; Exit = $LightDefault }
    [pscustomobject]@{ Name = "LIGHT_TEST_DEV_SIMULATION"; Exit = $LightDev }
    [pscustomobject]@{ Name = "QUANTUM_LIB_TEST"; Exit = $QuantumLib }
    [pscustomobject]@{ Name = "SHADOW_PURE_RUST_TEST"; Exit = $ShadowPure }
    [pscustomobject]@{ Name = "WORKSPACE_TEST"; Exit = $Workspace }
    [pscustomobject]@{ Name = "RELEASE_BUILD"; Exit = $Release }
    [pscustomobject]@{ Name = "CLIPPY_WORKSPACE"; Exit = $ClippyDefault }
    [pscustomobject]@{ Name = "CLIPPY_SHADOW_PURE_RUST"; Exit = $ClippyShadowPure }
)

$Pairs | ForEach-Object { "$($_.Name)_EXIT=$($_.Exit)" } |
    Set-Content -Encoding utf8 (Join-Path $Out "00_summary.txt")

$Failed = @($Pairs | Where-Object { $_.Exit -ne 0 })
"FAILED_STEPS=$($Failed.Count)" | Add-Content -Encoding utf8 (Join-Path $Out "00_summary.txt")

notepad (Join-Path $Out "00_summary.txt")
