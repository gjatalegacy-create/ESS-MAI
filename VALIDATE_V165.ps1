param(
    [string]$LogRoot = (Join-Path ([Environment]::GetFolderPath('Desktop')) 'ESS_MAI_V165_LOGS')
)

$ErrorActionPreference = 'Stop'
[Console]::OutputEncoding = [System.Text.UTF8Encoding]::new()
$Root = Split-Path -Parent $MyInvocation.MyCommand.Path
New-Item -ItemType Directory -Force -Path $LogRoot | Out-Null

function Run-Logged {
    param([string]$Name, [scriptblock]$Command)
    $Path = Join-Path $LogRoot ($Name + '.log')
    & $Command 2>&1 | Tee-Object -FilePath $Path
    $Code = $LASTEXITCODE
    "EXIT_CODE=$Code" | Add-Content -LiteralPath $Path -Encoding UTF8
    if ($Code -ne 0) { throw "$Name failed with exit code $Code. See $Path" }
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
function Require-Hash {
    param([string]$Path, [string]$Expected, [string]$Label)
    $Actual = (Get-FileHash -LiteralPath $Path -Algorithm SHA256).Hash.ToLowerInvariant()
    if ($Actual -ne $Expected) { throw "STATIC GUARD FAILED: $Label hash changed ($Actual)" }
}
function Require-SameHash {
    param([string[]]$Paths, [string]$Label)
    $Hashes = $Paths | ForEach-Object {
        (Get-FileHash -LiteralPath $_ -Algorithm SHA256).Hash.ToLowerInvariant()
    } | Select-Object -Unique
    if ($Hashes.Count -ne 1) { throw "STATIC GUARD FAILED: $Label" }
    "$Label=$($Hashes[0])"
}

Push-Location $Root
try {
    Run-Logged '00_toolchain' { cargo -Vv; rustc -Vv; rustup show; gcc --version }

    foreach ($Manifest in @(
        'light\Cargo.toml','quantum\Cargo.toml','shadow\Cargo.toml',
        'shadow-contracts\Cargo.toml','ui\Cargo.toml','light\ui\src-tauri\Cargo.toml'
    )) {
        Require-Text (Join-Path $Root $Manifest) 'version\s*=\s*"1\.6\.5"' "$Manifest v1.6.5"
    }
    Require-Text (Join-Path $Root 'VERSION_ESSMAI.txt') 'ESS-MAI v1\.6\.5' 'version marker'

    $QuantumMain = Join-Path $Root 'quantum\src\main.rs'
    $Workspace = Join-Path $Root 'quantum\src\project_workspace_router.rs'
    $LightProject = Join-Path $Root 'light\src\project_process_bridge.rs'
    $LightSovereign = Join-Path $Root 'light\src\sovereign_bridges.rs'

    # Project-only split: new default workspace route + explicit legacy route.
    Require-Text $QuantumMain '--project-workspace-once' 'Quantum workspace entrypoint'
    Require-Text $QuantumMain '--project-process-once' 'Quantum legacy scientific entrypoint'
    Require-Text $QuantumMain 'dispatch_project_workspace_once' 'workspace dispatch before runtime stdin'
    Require-Text $LightProject '--project-route-once' 'Light default workspace entrypoint'
    Require-Text $LightProject '--project-route-legacy-once' 'Light legacy entrypoint'
    Require-Text $LightProject '--project-workspace-once' 'Light invokes Quantum workspace'
    Require-Text $LightProject '--project-process-once' 'Light preserves Quantum legacy'
    Require-Text $LightSovereign 'route_project_workspace_under_gcl' 'Light workspace route'
    Require-Text $LightSovereign 'route_scientific_project_under_gcl' 'Light legacy scientific route'
    Require-Text $LightSovereign 'prepare_project_handoff_under_gcl' 'single APUPK/GCL preparation'

    # Workspace is orientation only: no pipeline, no authority, no token API.
    Require-Text $Workspace 'ESS_MAI_QUANTUM_PROJECT_WORKSPACE_V165' 'workspace SHA domain'
    Require-Text $Workspace 'PROJECT_STORAGE' 'storage route'
    Require-Text $Workspace 'PROJECT_CONVERSATION' 'conversation route'
    Require-Text $Workspace 'authority=NONE' 'no authority declaration'
    Require-Text $Workspace 'token_policy=UNCHANGED' 'token boundary declaration'
    Reject-Text $Workspace 'LgcToken|LgcGate|CapHandle|SovereignGate|token_forge|ForgeToken|SEAL_[A-Z_]+|::mint\s*\(|\.mint\s*\(' 'workspace must not use token APIs'

    $WorkspaceRun = [regex]::Match(
        (Get-Content -LiteralPath $QuantumMain -Raw -Encoding UTF8),
        'fn run_project_workspace_once\s*\((?<body>.*?)\n\}\n\nfn dispatch_project_process_once',
        [Text.RegularExpressions.RegexOptions]::Singleline
    ).Groups['body'].Value
    if ([string]::IsNullOrWhiteSpace($WorkspaceRun)) { throw 'workspace function block not found' }
    if ($WorkspaceRun -match '(?m)(^|[^a-zA-Z_])run\s*\(') {
        throw 'STATIC GUARD FAILED: workspace route enters full reasoning run()'
    }
    if ($WorkspaceRun -match 'LgcToken|token_forge|ForgeToken|LgcGate|CapHandle') {
        throw 'STATIC GUARD FAILED: workspace route touches token flow'
    }

    # Existing constitutional contracts deliberately remain V164.
    Require-Text $LightSovereign 'GCL:SCIENTIFIC_PROJECT:V164' 'GCL project identity unchanged'
    foreach ($Contract in @(
        'light\src\gcl_project_contract.rs',
        'quantum\src\gcl_project_contract.rs',
        'shadow\src\gcl_project_contract.rs'
    )) {
        Require-Text (Join-Path $Root $Contract) 'ESS_MAI_GCL_PROJECT_CONTEXT_V164' "$Contract V164 context"
        Require-Text (Join-Path $Root $Contract) 'ESS_MAI_GCL_SCIENTIFIC_PROJECT_EVIDENCE_V164' "$Contract V164 evidence"
    }
    foreach ($Contract in @(
        'light\src\living_trust_contract.rs',
        'quantum\src\living_trust_contract.rs',
        'shadow\src\living_trust_contract.rs'
    )) {
        Require-Text (Join-Path $Root $Contract) 'GCL_LIVING_TRUST_V164' "$Contract V164 trust"
        Require-Text (Join-Path $Root $Contract) 'GCL_LIVING_TRUST_TO_IZ_V164' "$Contract V164 iZ"
    }
    Require-Text (Join-Path $Root 'shadow-contracts\src\lib.rs') 'PROTOCOL_VERSION:\s*u16\s*=\s*9' 'wire protocol unchanged'

    # Token-critical Quantum sources must remain byte-identical to v1.6.4.
    $TokenHashes = @{
        'quantum\src\bridge_shadow\mod.rs' = '9bf2294484036322910dad4870090ceaf209cfa1cae85211899804e89f6a4025'
        'quantum\src\hcp_pro.rs' = '60295e1edcd7082fb8a567204aa4a18046c3a3518028249901d47e2a643ff936'
        'quantum\src\lab_contracts\verification_receipt.rs' = '93f010fbd3171c8ec8f452e3f7332df5c529eaefa7ce6afd910200b0381b8d9c'
        'quantum\src\layer2\hcp_pro_l2.rs' = '51980f6baee02440cbce22f196142698b8459d5c55e2733441130fde7f17a9be'
        'quantum\src\layer3\hcp_pro_l3.rs' = '75fbb66d258932079c06fea70160065a21562b7edae22725a1e8d00da02f0a74'
        'quantum\src\progressive_debatic\runtime.rs' = '0459642c01a01b9896cd0d5bab458c0a4c78a5ad2cdc98b38d5abd136e469218'
        'quantum\src\progressive_debatic\seal.rs' = '37dfe66a4b4efaae0ed85ffa202fff197706c5d3a9da8a92485bd479eeff954b'
        'quantum\src\progressive_debatic\types.rs' = 'd7a77acefe3815e7ba2adb5ff7571eeaeaf1ccc524a816f5a82b1043ecc6a3dc'
        'quantum\src\sovereign\lgc_gate.rs' = '375d4ed5071d7316f39c07e0ebd7bcffbaf7cfec0bf3259b10b92163c9e62ebe'
        'quantum\src\sovereign\ring.rs' = 'a9db1effbe945b8df6c52bf126ccbecdd86987248d0a8f2612cb1dfce0fcce9c'
        'quantum\src\sovereign\seal_registry.rs' = '6ce845f98657234306861addcc2557b02b2e14fb66e1b2b4cef2463879f9b78c'
        'quantum\src\token_forge.rs' = '9b3182ae9164c4a00114405a8790cb2f79bd5fa25d4c34bfb39edc4de379b39e'
    }
    foreach ($Entry in $TokenHashes.GetEnumerator()) {
        Require-Hash (Join-Path $Root $Entry.Key) $Entry.Value "token source $($Entry.Key)"
    }

    # Byte-identical governing contracts across platforms.
    $ProjectHash = Require-SameHash @(
        (Join-Path $Root 'light\src\gcl_project_contract.rs'),
        (Join-Path $Root 'quantum\src\gcl_project_contract.rs'),
        (Join-Path $Root 'shadow\src\gcl_project_contract.rs')
    ) 'GCL_PROJECT_CONTRACT_BYTE_IDENTICAL'
    $TrustHash = Require-SameHash @(
        (Join-Path $Root 'light\src\living_trust_contract.rs'),
        (Join-Path $Root 'quantum\src\living_trust_contract.rs'),
        (Join-Path $Root 'shadow\src\living_trust_contract.rs')
    ) 'LIVING_TRUST_CONTRACT_BYTE_IDENTICAL'
    $ReceiptHash = Require-SameHash @(
        (Join-Path $Root 'light\src\lab_contracts\verification_receipt.rs'),
        (Join-Path $Root 'quantum\src\lab_contracts\verification_receipt.rs'),
        (Join-Path $Root 'shadow\src\lab_contracts\verification_receipt.rs')
    ) 'VERIFICATION_RECEIPT_BYTE_IDENTICAL'

    # UI still owns only upload and emotional reflection.
    $OldUiRust = Join-Path $Root 'light\ui\src-tauri\src\main.rs'
    Require-Text $OldUiRust '#\[tauri::command\]\s*fn upload_project' 'old UI upload'
    Require-Text $OldUiRust '#\[tauri::command\]\s*fn reflect_system_emotion' 'old UI emotion'
    Require-Text $OldUiRust '"--project-route-once"' 'old UI uses default Light route'
    Reject-Text $OldUiRust 'shadow_platform|quantum-platform|--project-workspace-once|--project-process-once' 'old UI cannot bypass Light'

    # Shadow remains unmodified in behavior and owns APUPK persistence.
    Require-Text (Join-Path $Root 'shadow\src\process_bridge.rs') '--project-register-once' 'Shadow APUPK registration'
    Require-Text (Join-Path $Root 'shadow\src\process_bridge.rs') 'validate_scientific_project_against_apupk' 'Shadow APUPK verification'
    Require-Text (Join-Path $Root 'shadow\src\shadow_gj_legacy.rs') 'verify_project_gcl_stage' 'Shadow multi-stage GCL verification'

    # Documentation identity.
    Require-Text (Join-Path $Root 'ess-mai.md') 'Evolucioni v1\.6\.4 → v1\.6\.5' 'v1.6.5 evolution entry'
    $ParallelDoc = Join-Path (Split-Path -Parent $Root) 'ess_mai.md'
    if (-not (Test-Path -LiteralPath $ParallelDoc)) { throw 'parallel ess_mai.md missing' }
    $DocHashes = @(
        (Get-FileHash -LiteralPath (Join-Path $Root 'ess-mai.md') -Algorithm SHA256).Hash,
        (Get-FileHash -LiteralPath $ParallelDoc -Algorithm SHA256).Hash
    ) | Select-Object -Unique
    if ($DocHashes.Count -ne 1) { throw 'parallel ess_mai.md is not byte-identical' }

    @('STATIC_GUARDS_OK',$ProjectHash,$TrustHash,$ReceiptHash) |
        Set-Content -LiteralPath (Join-Path $LogRoot '01_static_guards.log') -Encoding UTF8

    # Ordered Cargo proof; Clippy is strict to prevent new warning debt.
    Run-Logged '02_check_workspace_all_targets' { cargo check --workspace --all-targets -vv }
    Run-Logged '03_build_workspace_all_targets' { cargo build --workspace --all-targets -vv }
    Run-Logged '04_test_compile_only' { cargo test --workspace --all-targets --no-run -vv }
    Run-Logged '05_test_workspace' { cargo test --workspace --all-targets -- --test-threads=1 }
    Run-Logged '06_quantum_all_features' { cargo check -p quantum-platform --all-targets --all-features -vv }
    Run-Logged '07_light_all_features' { cargo check -p light-platform --all-targets --all-features -vv }
    Run-Logged '08_shadow_pure_rust' { cargo test -p shadow_platform --no-default-features --features pure_rust --no-run -vv }
    Run-Logged '09_clippy_no_warning_debt' { cargo clippy --workspace --all-targets --all-features -- -D warnings }
    Run-Logged '10_fmt_check' { cargo fmt --all -- --check }

    Push-Location (Join-Path $Root 'ui')
    try { Run-Logged '11_check_new_ui' { cargo check --all-targets -vv } }
    finally { Pop-Location }
    Push-Location (Join-Path $Root 'light\ui\src-tauri')
    try { Run-Logged '12_check_old_ui' { cargo check --all-targets -vv } }
    finally { Pop-Location }

    $Manifest = Join-Path $Root 'ESS_MAI_V1_6_5_FILELIST.sha256'
    $Failures = @()
    foreach ($Line in Get-Content -LiteralPath $Manifest -Encoding UTF8) {
        if ($Line -match '^([0-9a-f]{64})  (.+)$') {
            $Expected = $Matches[1]
            $Relative = $Matches[2].Replace('/', [IO.Path]::DirectorySeparatorChar)
            $Target = Join-Path $Root $Relative
            if (-not (Test-Path -LiteralPath $Target)) { $Failures += "MISSING $Relative"; continue }
            $Actual = (Get-FileHash -LiteralPath $Target -Algorithm SHA256).Hash.ToLowerInvariant()
            if ($Actual -ne $Expected) { $Failures += "HASH $Relative" }
        }
    }
    $Failures | Set-Content -LiteralPath (Join-Path $LogRoot '13_hash_failures.log') -Encoding UTF8
    if ($Failures.Count -ne 0) { throw "File-list verification failed: $($Failures.Count)" }

    'CARGO_GREEN=TRUE' | Set-Content -LiteralPath (Join-Path $LogRoot '14_release_gate.log') -Encoding UTF8
    Write-Host "ESS-MAI v1.6.5 validation completed. CARGO_GREEN=TRUE. Logs: $LogRoot"
}
finally { Pop-Location }
