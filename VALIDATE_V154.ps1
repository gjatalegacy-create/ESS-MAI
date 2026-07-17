$ErrorActionPreference = "Continue"
$Root = Split-Path -Parent $MyInvocation.MyCommand.Path
$Desktop = [Environment]::GetFolderPath('Desktop')
$Out = Join-Path $Desktop "ESS_MAI_V154_LOGS"
New-Item -ItemType Directory -Force -Path $Out | Out-Null
Set-Location $Root

function Run-Step {
    param([string]$Name, [string]$Command)
    $Log = Join-Path $Out ($Name + ".txt")
    cmd.exe /d /s /c "$Command > `"$Log`" 2>&1"
    return $LASTEXITCODE
}

# Porta që provoi gabimet e target-eve test në versionet e mëparshme.
$AllTargets = Run-Step "01_build_workspace_all_targets_v154" "cargo --color never build --workspace --all-targets"

# Prova e gjithë workspace-it.
$Workspace = Run-Step "02_test_workspace_v154" "cargo --color never test --workspace"

# Provat e fokusuara të rrjedhës së re dy-fazëshe PD.
$QuantumPd = Run-Step "03_test_quantum_pd_v154" "cargo --color never test -p quantum-platform progressive_debatic"
$LightPd = Run-Step "04_test_light_pd_v154" "cargo --color never test -p light-platform pd_light"
$ShadowReceipt = Run-Step "05_test_shadow_receipt_v154" "cargo --color never test -p shadow_platform verification_output_is_sealed_by_single_use_token"

# Feature-matrix e vlefshme: pure_rust nuk kombinohet me runtime_mode.
$ShadowPure = Run-Step "06_test_shadow_pure_rust_v154" "cargo --color never test -p shadow_platform --no-default-features --features pure_rust"
$Release = Run-Step "07_build_release_v154" "cargo --color never build --workspace --release"
$Clippy = Run-Step "08_clippy_workspace_v154" "cargo --color never clippy --workspace --all-targets"

rustc -Vv 2>&1 | Set-Content -Encoding utf8 (Join-Path $Out "rustc_version_v154.txt")
cargo -V 2>&1 | Set-Content -Encoding utf8 (Join-Path $Out "cargo_version_v154.txt")

$Pairs = @(
    [pscustomobject]@{ Name = "BUILD_ALL_TARGETS"; Exit = $AllTargets }
    [pscustomobject]@{ Name = "WORKSPACE_TEST"; Exit = $Workspace }
    [pscustomobject]@{ Name = "QUANTUM_PD_TEST"; Exit = $QuantumPd }
    [pscustomobject]@{ Name = "LIGHT_PD_TEST"; Exit = $LightPd }
    [pscustomobject]@{ Name = "SHADOW_RECEIPT_TEST"; Exit = $ShadowReceipt }
    [pscustomobject]@{ Name = "SHADOW_PURE_RUST_TEST"; Exit = $ShadowPure }
    [pscustomobject]@{ Name = "RELEASE_BUILD"; Exit = $Release }
    [pscustomobject]@{ Name = "CLIPPY_WORKSPACE"; Exit = $Clippy }
)

$Summary = Join-Path $Out "00_summary_v154.txt"
$Pairs | ForEach-Object { "$($_.Name)_EXIT=$($_.Exit)" } |
    Set-Content -Encoding utf8 $Summary
$Failed = @($Pairs | Where-Object { $_.Exit -ne 0 })
"FAILED_STEPS=$($Failed.Count)" | Add-Content -Encoding utf8 $Summary
"NOTE=No cargo fix; no global panic=abort." | Add-Content -Encoding utf8 $Summary

explorer $Out
notepad $Summary
