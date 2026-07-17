param(
    [string]$LogRoot = (Join-Path ([Environment]::GetFolderPath('Desktop')) 'ESS_MAI_V157_LOGS')
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

Push-Location $Root
try {
    Run-Logged '00_toolchain' { cargo --version; rustc --version }
    Run-Logged '01_build_workspace_all_targets' { cargo build --workspace --all-targets }
    Run-Logged '02_test_workspace' { cargo test --workspace }

    Push-Location (Join-Path $Root 'ui')
    try { Run-Logged '03_check_new_ui' { cargo check --all-targets } }
    finally { Pop-Location }

    Push-Location (Join-Path $Root 'light\ui\src-tauri')
    try { Run-Logged '04_check_old_ui_emotional_engine' { cargo check --all-targets } }
    finally { Pop-Location }

    $Manifest = Join-Path $Root 'ESS_MAI_V1_5_7_FILELIST.sha256'
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
    $Failures | Set-Content -LiteralPath (Join-Path $LogRoot '05_hash_failures.log') -Encoding UTF8
    if ($Failures.Count -ne 0) { throw "File-list verification failed: $($Failures.Count)" }
    'HASH_OK' | Set-Content -LiteralPath (Join-Path $LogRoot '05_hash_ok.log') -Encoding UTF8

    Write-Host "ESS-MAI v1.5.7 validation completed. Logs: $LogRoot"
}
finally {
    Pop-Location
}
