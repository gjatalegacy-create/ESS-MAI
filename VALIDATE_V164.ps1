param(
    [string]$LogRoot = (Join-Path ([Environment]::GetFolderPath('Desktop')) 'ESS_MAI_V164_LOGS')
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
        Require-Text (Join-Path $Root $Manifest) 'version\s*=\s*"1\.6\.4"' "$Manifest v1.6.4"
    }
    Require-Text (Join-Path $Root 'VERSION_ESSMAI.txt') 'ESS-MAI v1\.6\.4' 'version marker'

    # Old UI: exactly upload + emotion; no reasoning and no direct Shadow/Quantum route.
    $OldUiRust = Join-Path $Root 'light\ui\src-tauri\src\main.rs'
    $OldUiJs = Join-Path $Root 'light\ui\src\main.js'
    Require-Text $OldUiRust '#\[tauri::command\]\s*fn upload_project' 'old UI upload command'
    Require-Text $OldUiRust '#\[tauri::command\]\s*fn reflect_system_emotion' 'old UI emotion command'
    Require-Text $OldUiRust '"--project-route-once"' 'old UI calls only Light project route'
    Reject-Text $OldUiRust 'shadow_platform|quantum-platform|--project-register-once|--project-process-once' 'old UI direct sovereign process call'
    Reject-Text $OldUiRust 'ready_for_shadow|explore_input|upload_knowledge_dialog|get_output' 'old UI placeholder authority removed'
    Reject-Text $OldUiJs 'ask_nura|explore_input|upload_knowledge_dialog|get_output' 'old UI frontend minimal role'
    Require-Text (Join-Path $Root 'light\ui\src-tauri\tauri.conf.json') '"withGlobalTauri"\s*:\s*true' 'old UI global Tauri bridge'

    # Intake wire contains user material only; Light creates constitutional fields.
    $Contracts = Join-Path $Root 'shadow-contracts\src\lib.rs'
    Require-Text $Contracts 'PROTOCOL_VERSION:\s*u16\s*=\s*9' 'wire protocol v9'
    Require-Text $Contracts 'QUANTUM_MAX_TRL:\s*u8\s*=\s*3' 'Quantum TRL bound'
    Require-Text $Contracts 'SHADOW_FACTUAL_TRL:\s*u8\s*=\s*4' 'Shadow-only TRL4'
    Require-Text $Contracts 'self\.trl_level\s*<=\s*QUANTUM_MAX_TRL' 'inbound TRL4 rejected'
    $IntakeBlock = [regex]::Match(
        (Get-Content -LiteralPath $Contracts -Raw -Encoding UTF8),
        'pub struct LightProjectIntakeRequestWire\s*\{(?<body>.*?)\n\}',
        [Text.RegularExpressions.RegexOptions]::Singleline
    ).Groups['body'].Value
    if ($IntakeBlock -match 'project_id|user_id|timestamp|contract_id|lgc_seal|trl_|verdict') {
        throw 'STATIC GUARD FAILED: UI intake wire contains authority fields'
    }

    $LightSovereign = Join-Path $Root 'light\src\sovereign_bridges.rs'
    Require-Text (Join-Path $Root 'light\src\project_process_bridge.rs') 'let project_id = shadow_contracts::fnv1a64' 'Light owns project identity'
    Require-Text (Join-Path $Root 'light\src\project_process_bridge.rs') 'let timestamp_ns = current_time_ns' 'Light owns intake timestamp'
    Require-Text $LightSovereign 'GCL:SCIENTIFIC_PROJECT:V164' 'Light owns project contract identity'
    Require-Text $LightSovereign 'witness\.light_sovereign_flags' 'Light derives seal from real witness'
    Require-Text $LightSovereign 'ProjectContextWitness nuk lidhet me SHA-256' 'Light riverifies witness content'

    # Complete mediation: Quantum knows only public wire contracts.
    Require-Text (Join-Path $Root 'shadow\Cargo.toml') 'autolib\s*=\s*false' 'Shadow binary-only'
    Reject-Text  (Join-Path $Root 'shadow\Cargo.toml') '(?m)^\[lib\]' 'Shadow [lib] must not return'
    Reject-Text  (Join-Path $Root 'shadow\Cargo.toml') 'rlib|staticlib' 'Shadow linkable core must not return'
    Require-Text (Join-Path $Root 'shadow\src\main.rs') 'include!\("lib\.rs"\)' 'Shadow main owns core'
    Require-Text (Join-Path $Root 'shadow\src\main.rs') 'process_bridge::dispatch_from_args' 'Shadow main owns mediation'
    Require-Text (Join-Path $Root 'quantum\Cargo.toml') 'shadow_contracts\s*=\s*\{\s*path\s*=\s*"\.\./shadow-contracts"' 'Quantum public contracts only'
    Reject-Text  (Join-Path $Root 'quantum\Cargo.toml') 'path\s*=\s*"\.\./shadow"|package\s*=\s*"shadow_platform"' 'Quantum must not link Shadow core'
    Reject-Text  (Join-Path $Root 'quantum\src\main.rs') 'shadow_lib::|shadow_platform::' 'Quantum must not call Shadow core'

    # v1.6.3 compiler blockers.
    Require-Text (Join-Path $Root 'quantum\src\main.rs') '\|\{\}\|\{:\s*016x\}\|\{\}\|\{\}\|\{\}\|\{\}\|\{\}\|\{\}\|\{\}\|\{:\s*08x\}\|' 'Quantum SHA/project_id/context formatter alignment'
    Require-Text (Join-Path $Root 'light\src\pd_light.rs') 'round_trip_45_body_fields' 'PD Light schema test exists'
    Require-Text (Join-Path $Root 'light\src\pd_light.rs') '\|\{\}\|\{:\s*016x\}\|\{\}\|\{\}\|\{\}\|\{\}\|\{\}\|\{\}\|\{\}\|\{:\s*08x\}\|' 'Light SHA/project_id/context formatter alignment'
    Require-Text (Join-Path $Root 'shadow\src\bridge\mod.rs') 'scientific_project:\s*None' 'Shadow bridge legacy fixture explicit'
    Require-Text (Join-Path $Root 'shadow\src\bridge\shadow_callable.rs') 'scientific_project:\s*None' 'Shadow callable legacy fixture explicit'

    # Shadow multi-stage verification under the same GCL.
    $Supreme = Join-Path $Root 'shadow\src\shadow_gj_legacy.rs'
    Require-Text $Supreme 'verify_project_gcl_stage' 'Shadow GCL identity stage'
    Require-Text $Supreme 'project\.gcl_process_digest\s*==\s*package\.pd_gcl_process_digest' 'same GCL process'
    Require-Text $Supreme 'package\.spine_completion_digest\s*!=\s*0' 'Spine completion gate'
    Require-Text $Supreme 'project\.trl_level\s*<=\s*shadow_contracts::QUANTUM_MAX_TRL' 'Shadow rejects inbound TRL4'
    Require-Text $Supreme 'verify_project_file_kinds' 'magic-byte stage'
    Require-Text $Supreme 'ShadowEco::classify_with_factualization' 'TRL4 factualization stage'
    Require-Text $Supreme 'let sovereign_pair = verified == 1 && primitive == 1' 'sovereign pair before TRL4'
    Require-Text (Join-Path $Root 'shadow\src\shadow_genius_novel.rs') 'shadow_contracts::SHADOW_FACTUAL_TRL' 'single source of TRL4 truth'

    # Byte-identical governing contracts.
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
    $ContinuumHash = Require-SameHash @(
        (Join-Path $Root 'light\src\pd_continuum_contract.rs'),
        (Join-Path $Root 'quantum\src\pd_continuum_contract.rs'),
        (Join-Path $Root 'shadow\src\pd_continuum_contract.rs')
    ) 'PD_CONTINUUM_BYTE_IDENTICAL'
    $SpineHash = Require-SameHash @(
        (Join-Path $Root 'light\src\pd_spine_contract.rs'),
        (Join-Path $Root 'quantum\src\pd_spine_contract.rs'),
        (Join-Path $Root 'shadow\src\pd_spine_contract.rs')
    ) 'PD_SPINE_BYTE_IDENTICAL'
    $LabIndexHash = Require-SameHash @(
        (Join-Path $Root 'light\src\lab_contracts\mod.rs'),
        (Join-Path $Root 'quantum\src\lab_contracts\mod.rs'),
        (Join-Path $Root 'shadow\src\lab_contracts\mod.rs')
    ) 'LAB_CONTRACT_INDEX_BYTE_IDENTICAL'

    foreach ($Contract in @(
        'light\src\gcl_project_contract.rs',
        'quantum\src\gcl_project_contract.rs',
        'shadow\src\gcl_project_contract.rs'
    )) {
        Require-Text (Join-Path $Root $Contract) 'ESS_MAI_GCL_PROJECT_CONTEXT_V164' "$Contract V164 context domain"
        Require-Text (Join-Path $Root $Contract) 'ESS_MAI_GCL_SCIENTIFIC_PROJECT_EVIDENCE_V164' "$Contract V164 evidence domain"
    }
    foreach ($Contract in @(
        'light\src\living_trust_contract.rs',
        'quantum\src\living_trust_contract.rs',
        'shadow\src\living_trust_contract.rs'
    )) {
        Require-Text (Join-Path $Root $Contract) 'GCL_LIVING_TRUST_V164' "$Contract V164 trust domain"
        Require-Text (Join-Path $Root $Contract) 'GCL_LIVING_TRUST_TO_IZ_V164' "$Contract V164 iZ domain"
    }

    # Runtime paths.
    $LightProject = Join-Path $Root 'light\src\project_process_bridge.rs'
    Require-Text $LightProject '--project-route-once' 'Light project runtime entrypoint'
    Require-Text $LightProject '--project-register-once' 'Light requests Shadow APUPK context'
    Require-Text $LightProject '--project-process-once' 'Light invokes real Quantum process'
    Require-Text (Join-Path $Root 'quantum\src\main.rs') '--project-process-once' 'Quantum project runtime entrypoint'
    Require-Text (Join-Path $Root 'shadow\src\process_bridge.rs') '--project-register-once' 'Shadow project registration entrypoint'
    Require-Text (Join-Path $Root 'shadow\src\process_bridge.rs') 'validate_scientific_project_against_apupk' 'Shadow verifies durable APUPK context'

    # Documentation identity and progress.
    Require-Text (Join-Path $Root 'ess-mai.md') 'Evolucioni v1\.6\.3 → v1\.6\.4' 'authoritative evolution entry'
    $ParallelDoc = Join-Path (Split-Path -Parent $Root) 'ess_mai.md'
    if (-not (Test-Path -LiteralPath $ParallelDoc)) { throw 'parallel ess_mai.md missing' }
    $DocHashes = @(
        (Get-FileHash -LiteralPath (Join-Path $Root 'ess-mai.md') -Algorithm SHA256).Hash,
        (Get-FileHash -LiteralPath $ParallelDoc -Algorithm SHA256).Hash
    ) | Select-Object -Unique
    if ($DocHashes.Count -ne 1) { throw 'parallel ess_mai.md is not byte-identical' }

    @('STATIC_GUARDS_OK',$ProjectHash,$TrustHash,$ReceiptHash,$ContinuumHash,$SpineHash,$LabIndexHash) |
        Set-Content -LiteralPath (Join-Path $LogRoot '01_static_guards.log') -Encoding UTF8

    # Ordered Cargo proof. Shadow intentionally separates runtime_mode and pure_rust.
    Run-Logged '02_check_workspace_all_targets' { cargo check --workspace --all-targets -vv }
    Run-Logged '03_build_workspace_all_targets' { cargo build --workspace --all-targets -vv }
    Run-Logged '04_test_compile_only' { cargo test --workspace --all-targets --no-run -vv }
    Run-Logged '05_test_workspace' { cargo test --workspace --all-targets -- --test-threads=1 }
    Run-Logged '06_quantum_dev_harness' { cargo check -p quantum-platform --bin quantum-platform --features dev_harness -vv }
    Run-Logged '07_shadow_dev_harness' { cargo check -p shadow_platform --bin shadow_platform --features dev_harness -vv }
    Run-Logged '08_shadow_pure_rust_tests' { cargo test -p shadow_platform --no-default-features --features pure_rust --no-run -vv }
    Run-Logged '09_clippy_workspace' { cargo clippy --workspace --all-targets -- -W clippy::all }
    Run-Logged '10_fmt_check' { cargo fmt --all -- --check }

    Push-Location (Join-Path $Root 'ui')
    try { Run-Logged '11_check_new_ui' { cargo check --all-targets -vv } }
    finally { Pop-Location }
    Push-Location (Join-Path $Root 'light\ui\src-tauri')
    try { Run-Logged '12_check_old_ui' { cargo check --all-targets -vv } }
    finally { Pop-Location }

    $Manifest = Join-Path $Root 'ESS_MAI_V1_6_4_FILELIST.sha256'
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
    Write-Host "ESS-MAI v1.6.4 validation completed. CARGO_GREEN=TRUE. Logs: $LogRoot"
}
finally { Pop-Location }
