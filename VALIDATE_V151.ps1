$ErrorActionPreference = "Continue"
$Root = Split-Path -Parent $MyInvocation.MyCommand.Path
$Out = Join-Path $Root "validation_v151"
New-Item -ItemType Directory -Force -Path $Out | Out-Null
Set-Location $Root

function Run-Step {
    param([string]$Name, [string]$Command)
    $Log = Join-Path $Out ($Name + ".txt")
    cmd.exe /d /s /c "$Command > `"$Log`" 2>&1"
    return $LASTEXITCODE
}

$QSerial = Run-Step "01_quantum_serial" "cargo --color never test -p quantum-platform --lib -- --test-threads=1"
$QParallel = Run-Step "02_quantum_parallel" "cargo --color never test -p quantum-platform --lib"
$Workspace = Run-Step "03_workspace_tests" "cargo --color never test --workspace"
$Release = Run-Step "04_release_build" "cargo --color never build --release"
$Clippy = Run-Step "05_clippy" "cargo --color never clippy --workspace --all-targets --all-features"

@(
    "QUANTUM_SERIAL_EXIT=$QSerial"
    "QUANTUM_PARALLEL_EXIT=$QParallel"
    "WORKSPACE_TEST_EXIT=$Workspace"
    "RELEASE_BUILD_EXIT=$Release"
    "CLIPPY_EXIT=$Clippy"
) | Set-Content -Encoding utf8 (Join-Path $Out "00_summary.txt")

notepad (Join-Path $Out "00_summary.txt")
