$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location $Root

function Run-Step([string]$Name, [scriptblock]$Command) {
    Write-Host "`n=== $Name ===" -ForegroundColor Cyan
    & $Command
    if ($LASTEXITCODE -ne 0) { throw "$Name failed with exit code $LASTEXITCODE" }
}

Run-Step "Workspace fmt" { cargo fmt --all -- --check }
Run-Step "Workspace check" { cargo check --workspace --all-targets }
Run-Step "Workspace tests compile" { cargo test --workspace --all-targets --no-run }
Run-Step "Workspace clippy" { cargo clippy --workspace --all-targets -- -D warnings }
Run-Step "Workspace release" { cargo build --workspace --release }

Set-Location (Join-Path $Root "ui")
Run-Step "UI fmt" { cargo fmt -- --check }
Run-Step "UI check" { cargo check --all-targets }
Run-Step "UI clippy" { cargo clippy --all-targets -- -D warnings }
Run-Step "UI release" { cargo build --release }

$UiSource = Get-Content (Join-Path $Root "ui\src\main.rs") -Raw
if ($UiSource -match 'Command::new\([^\)]*(quantum|shadow)') {
    throw "UI source contains a forbidden direct Quantum/Shadow process call"
}
if ($UiSource -match 'LgcToken|ForgeToken|CapHandle|VerificationReceipt|LivingTrust') {
    throw "UI source imports or names a forbidden sovereign token/receipt type"
}
if ($UiSource -notmatch 'Command::new\(&light\)') {
    throw "UI source does not contain the required Light-only process call"
}
if ($UiSource -notmatch '--project-route-once' -or $UiSource -notmatch '--project-route-legacy-once') {
    throw "Project routes are incomplete"
}

Write-Host "`nESS-MAI v1.6.7 Tauri 2 validation passed." -ForegroundColor Green
