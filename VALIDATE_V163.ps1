param(
    [string]$LogRoot = (Join-Path ([Environment]::GetFolderPath('Desktop')) 'ESS_MAI_V163_LOGS')
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
        Require-Text (Join-Path $Root $Manifest) 'version\s*=\s*"1\.6\.3"' "$Manifest v1.6.3"
    }
    Require-Text (Join-Path $Root 'VERSION_ESSMAI.txt') 'ESS-MAI v1\.6\.3' 'version marker'

    # Complete mediation: Quantum knows only the public wire crate.
    Require-Text (Join-Path $Root 'shadow\Cargo.toml') 'autolib\s*=\s*false' 'Shadow binary-only'
    Reject-Text  (Join-Path $Root 'shadow\Cargo.toml') '(?m)^\[lib\]' 'Shadow [lib] must not return'
    Reject-Text  (Join-Path $Root 'shadow\Cargo.toml') 'rlib|staticlib' 'Shadow linkable core must not return'
    Require-Text (Join-Path $Root 'shadow\src\main.rs') 'include!\("lib\.rs"\)' 'Shadow main owns core'
    Require-Text (Join-Path $Root 'shadow\src\main.rs') 'process_bridge::dispatch_from_args' 'Shadow main owns mediation'
    Require-Text (Join-Path $Root 'quantum\Cargo.toml') 'shadow_contracts\s*=\s*\{\s*path\s*=\s*"\.\./shadow-contracts"' 'Quantum public contracts only'
    Reject-Text  (Join-Path $Root 'quantum\Cargo.toml') 'path\s*=\s*"\.\./shadow"|package\s*=\s*"shadow_platform"' 'Quantum must not link Shadow core'
    Reject-Text  (Join-Path $Root 'quantum\src\main.rs') 'shadow_lib::|shadow_platform::' 'Quantum must not call Shadow core'

    # Original Cargo failures must remain closed semantically.
    Require-Text (Join-Path $Root 'quantum\src\main.rs') '#\[cfg\(feature\s*=\s*"dev_harness"\)\]\s*fn run_integrated_lab_demo' 'E0425 helper feature closure'
    Require-Text (Join-Path $Root 'quantum\src\shadow_process_bridge.rs') '#\[cfg\(feature\s*=\s*"dev_harness"\)\]\s*pub fn persist_negative' 'negative demo feature closure'
    Require-Text (Join-Path $Root 'shadow\tests\integration.rs') 'quantum_action_mask:\s*crate::runtime_pulse::REQUIRED_ACTION_MASK' 'E0063 action mask fixture'
    Require-Text (Join-Path $Root 'shadow\tests\integration.rs') 'quantum_required_action_mask:\s*crate::runtime_pulse::REQUIRED_ACTION_MASK' 'E0063 required mask fixture'
    Require-Text (Join-Path $Root 'shadow\tests\integration.rs') 'scientific_project:\s*None' 'fixture explicitly non-project'

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
    $GclHash = Require-SameHash @(
        (Join-Path $Root 'light\src\lab_contracts\gjata_collapse_law.rs'),
        (Join-Path $Root 'quantum\src\lab_contracts\gjata_collapse_law.rs'),
        (Join-Path $Root 'shadow\src\lab_contracts\gjata_collapse_law.rs')
    ) 'GCL_LAW_BYTE_IDENTICAL'

    # Project continuum and full identities.
    $Contracts = Join-Path $Root 'shadow-contracts\src\lib.rs'
    Require-Text $Contracts 'PROTOCOL_VERSION:\s*u16\s*=\s*8' 'wire protocol v8'
    Require-Text $Contracts 'pub scientific_project:\s*Option<ScientificProjectWire>' 'project inside FinalEvidence'
    Require-Text $Contracts 'pub project_context_sha256:\s*String' 'full context SHA on verdict wire'
    Require-Text $Contracts 'pub project_evidence_sha256:\s*String' 'full evidence SHA on verdict wire'
    foreach ($Fn in @(
        'encode_project_registration_request','decode_project_registration_request',
        'encode_light_project_intake_request','decode_light_project_intake_request',
        'encode_quantum_project_execution_request','decode_quantum_project_execution_request'
    )) { Require-Text $Contracts "pub fn $Fn" "wire function $Fn" }

    $LightProject = Join-Path $Root 'light\src\project_process_bridge.rs'
    Require-Text $LightProject '--project-route-once' 'Light project runtime entrypoint'
    Require-Text $LightProject '--project-register-once' 'Light requests Shadow APUPK context'
    Require-Text $LightProject '--project-process-once' 'Light invokes real Quantum process'
    Require-Text $LightProject 'response\.request_sha256\s*==\s*request_sha256' 'Quantum response bound to request'
    Require-Text (Join-Path $Root 'quantum\src\main.rs') '--project-process-once' 'Quantum project runtime entrypoint'
    Require-Text (Join-Path $Root 'shadow\src\process_bridge.rs') '--project-register-once' 'Shadow project registration entrypoint'
    Require-Text (Join-Path $Root 'shadow\src\process_bridge.rs') 'validate_scientific_project_against_apupk' 'Shadow verifies durable context'
    Require-Text (Join-Path $Root 'shadow\src\process_bridge.rs') 'ApupkProcessLock::acquire' 'interprocess APUPK lock'
    Require-Text (Join-Path $Root 'shadow\src\shadow_apupk.rs') 'pub fn store_durable' 'durable-before-witness'
    Require-Text (Join-Path $Root 'shadow\src\sovereign_log.rs') 'pub fn append_checked' 'checked WAL append'
    Require-Text (Join-Path $Root 'shadow\src\sovereign_log.rs') 'sync_all\(\)' 'WAL fsync'

    foreach ($Contract in @(
        'light\src\living_trust_contract.rs',
        'quantum\src\living_trust_contract.rs',
        'shadow\src\living_trust_contract.rs'
    )) {
        $P = Join-Path $Root $Contract
        Require-Text $P 'GCL_LIVING_TRUST_V163' "$Contract v1.6.3 domain"
        Require-Text $P 'scientific_project_sha256:\s*\[u8;\s*32\]' "$Contract full evidence SHA"
        Require-Text $P 'scientific_project_verdict_sha256:\s*\[u8;\s*32\]' "$Contract full verdict SHA"
        Require-Text $P 'Sha256::digest\(&material\)' "$Contract real SHA-256"
    }

    $Supreme = Join-Path $Root 'shadow\src\shadow_gj_legacy.rs'
    Require-Text $Supreme 'adjudicate_project_under_gcl' 'same supreme judge adjudicates project'
    Require-Text $Supreme 'classify_with_factualization' 'Novel factualization path'
    Require-Text $Supreme 'verify_project_file_kinds' 'magic-byte verification'
    Require-Text $Supreme 'PROJECT_STATUS_NOVEL_FACTUAL' 'Novel status'
    Require-Text $Supreme 'PROJECT_STATUS_RIGOROUS_NEGATIVE' 'rigorous negative status'
    Require-Text $Supreme 'verdict_sha256_or_zero' 'project verdict bound to Living Trust'

    # Negative Knowledge only after a verified negative cycle.
    $ShadowBridge = Join-Path $Root 'shadow\src\process_bridge.rs'
    Require-Text $ShadowBridge 'must_persist_negative' 'negative persist gate'
    Require-Text $ShadowBridge 'PROJECT_STATUS_RIGOROUS_NEGATIVE' 'project negative gate'
    Require-Text $ShadowBridge '#\[cfg\(feature\s*=\s*"dev_harness"\)\]\s*fn run_negative' 'standalone negative path dev-only'

    # PD Light receives the project result and recomputes full Trust.
    Require-Text (Join-Path $Root 'quantum\src\main.rs') '45 fusha trupi \+ CRC \(46 total\)' 'Quantum PD v1.6.3 field count'
    Require-Text (Join-Path $Root 'light\src\main.rs') 'verify_line_generic\(line,\s*&\[6,\s*7\],\s*46\)' 'Light accepts 46 sealed fields'
    Require-Text (Join-Path $Root 'light\src\pd_light.rs') 'round_trip_45_body_fields' 'Light PD 45 body fields'
    Require-Text (Join-Path $Root 'light\src\pd_light.rs') 'living_trust_scientific_project_sha256' 'PD Light full project evidence SHA'
    Require-Text (Join-Path $Root 'light\src\pd_light.rs') 'verdict_sha256_or_zero' 'PD Light recomputes project verdict SHA'

    # Documentation and package identity.
    Require-Text (Join-Path $Root 'ess-mai.md') 'Evolucioni v1\.6\.2 → v1\.6\.3' 'authoritative evolution entry'
    $ParallelDoc = Join-Path (Split-Path -Parent $Root) 'ess_mai.md'
    if (-not (Test-Path -LiteralPath $ParallelDoc)) { throw 'parallel ess_mai.md missing' }
    $DocHashes = @(
        (Get-FileHash -LiteralPath (Join-Path $Root 'ess-mai.md') -Algorithm SHA256).Hash,
        (Get-FileHash -LiteralPath $ParallelDoc -Algorithm SHA256).Hash
    ) | Select-Object -Unique
    if ($DocHashes.Count -ne 1) { throw 'parallel ess_mai.md is not byte-identical' }

    @('STATIC_GUARDS_OK',$ProjectHash,$TrustHash,$ReceiptHash,$ContinuumHash,$SpineHash,$GclHash) |
        Set-Content -LiteralPath (Join-Path $LogRoot '01_static_guards.log') -Encoding UTF8

    # Ordered Cargo proof. We do not use --all-features because Shadow intentionally
    # rejects runtime_mode+pure_rust in build.rs.
    Run-Logged '02_check_workspace_all_targets' { cargo check --workspace --all-targets -vv }
    Run-Logged '03_build_workspace_all_targets' { cargo build --workspace --all-targets -vv }
    Run-Logged '04_test_compile_only' { cargo test --workspace --all-targets --no-run -vv }
    Run-Logged '05_quantum_dev_harness' { cargo check -p quantum-platform --bin quantum-platform --features dev_harness -vv }
    Run-Logged '06_shadow_dev_harness' { cargo check -p shadow_platform --bin shadow_platform --features dev_harness -vv }
    Run-Logged '07_shadow_pure_rust_tests' { cargo test -p shadow_platform --no-default-features --features pure_rust --no-run -vv }
    Run-Logged '08_clippy_workspace' { cargo clippy --workspace --all-targets -- -W clippy::all }
    Run-Logged '09_fmt_check' { cargo fmt --all -- --check }

    Push-Location (Join-Path $Root 'ui')
    try { Run-Logged '10_check_new_ui' { cargo check --all-targets -vv } }
    finally { Pop-Location }
    Push-Location (Join-Path $Root 'light\ui\src-tauri')
    try { Run-Logged '11_check_old_ui' { cargo check --all-targets -vv } }
    finally { Pop-Location }

    $Manifest = Join-Path $Root 'ESS_MAI_V1_6_3_FILELIST.sha256'
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
    $Failures | Set-Content -LiteralPath (Join-Path $LogRoot '12_hash_failures.log') -Encoding UTF8
    if ($Failures.Count -ne 0) { throw "File-list verification failed: $($Failures.Count)" }
    'CARGO_GREEN=TRUE' | Set-Content -LiteralPath (Join-Path $LogRoot '13_release_gate.log') -Encoding UTF8
    Write-Host "ESS-MAI v1.6.3 validation completed. CARGO_GREEN=TRUE. Logs: $LogRoot"
}
finally { Pop-Location }
