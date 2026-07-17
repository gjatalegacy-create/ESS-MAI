param(
    [string]$LogRoot = (Join-Path ([Environment]::GetFolderPath('Desktop')) 'ESS_MAI_V162_LOGS')
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

function Require-SameHash {
    param([string[]]$Paths, [string]$Label)
    $Hashes = $Paths | ForEach-Object {
        (Get-FileHash -LiteralPath $_ -Algorithm SHA256).Hash.ToLowerInvariant()
    } | Select-Object -Unique
    if ($Hashes.Count -ne 1) { throw "STATIC GUARD FAILED: $Label" }
    "$Label = $($Hashes[0])"
}

Push-Location $Root
try {
    Run-Logged '00_toolchain' { cargo --version; rustc --version }

    foreach ($Manifest in @(
        'light\Cargo.toml', 'quantum\Cargo.toml', 'shadow\Cargo.toml',
        'shadow-contracts\Cargo.toml', 'ui\Cargo.toml',
        'light\ui\src-tauri\Cargo.toml'
    )) {
        Require-Text (Join-Path $Root $Manifest) 'version\s*=\s*"1\.6\.2"' "$Manifest must be v1.6.2"
    }
    Require-Text (Join-Path $Root 'VERSION_ESSMAI.txt') 'ESS-MAI v1\.6\.2' 'version marker'

    # Complete mediation: Shadow core exists only inside the mandatory executable.
    Require-Text (Join-Path $Root 'shadow\Cargo.toml') 'autolib\s*=\s*false' 'Shadow binary-only'
    Reject-Text  (Join-Path $Root 'shadow\Cargo.toml') '(?m)^\[lib\]' 'Shadow [lib] target must not return'
    Reject-Text  (Join-Path $Root 'shadow\Cargo.toml') 'rlib|staticlib' 'Shadow linkable crate-types must not return'
    Require-Text (Join-Path $Root 'shadow\src\main.rs') 'include!\("lib\.rs"\)' 'Shadow main owns core'
    Require-Text (Join-Path $Root 'shadow\src\main.rs') 'process_bridge::dispatch_from_args' 'Shadow main owns mediation'
    Require-Text (Join-Path $Root 'quantum\Cargo.toml') 'shadow_contracts\s*=\s*\{\s*path\s*=\s*"\.\./shadow-contracts"' 'Quantum knows only wire contracts'
    Reject-Text  (Join-Path $Root 'quantum\Cargo.toml') 'path\s*=\s*"\.\./shadow"|package\s*=\s*"shadow_platform"' 'Quantum must not link Shadow core'
    Reject-Text  (Join-Path $Root 'quantum\src\main.rs') 'shadow_lib::|shadow_platform::' 'Quantum main must not call Shadow core'
    Require-Text (Join-Path $Root 'quantum\src\main.rs') "source:\s*&'static\s+str" 'E0521 correction remains'

    # Byte-identical governing contracts.
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

    # Untrust Phase 1: real module completions, zero SHA, exact ledger and order.
    $Pulse = Join-Path $Root 'quantum\src\runtime_pulse.rs'
    Require-Text $Pulse 'static ACTION_STATE:\s*AtomicU64' 'per-cycle action state'
    Require-Text $Pulse 'static ACTION_MASK:\s*AtomicU64' 'per-cycle action mask'
    Require-Text $Pulse 'ACTION_STATE\.store\(0' 'Untrust resets state every cycle'
    Require-Text $Pulse 'ACTION_MASK\.store\(0' 'Untrust resets mask every cycle'
    Require-Text $Pulse 'evidence_words:\s*Vec<u64>' 'raw canonical action words retained'
    Require-Text $Pulse 'REQUIRED_ACTION_ORDER:\s*\[u8;\s*9\]' 'canonical module order'
    Require-Text $Pulse 'pub fn replay_action_evidence' 'local ledger replay'
    Require-Text $Pulse 'action_mask\(\) == REQUIRED_ACTION_MASK' 'exact required organ set'
    Reject-Text  $Pulse 'Sha256|sha2::' 'Phase 1 must contain zero SHA-256'

    $Main = Join-Path $Root 'quantum\src\main.rs'
    foreach ($Stage in @('Hpro','Pro','Npro','Npim','Srk','Pim','Apro','Mpro','Hcp')) {
        Require-Text $Main "mark_action\([\s\S]{0,160}Stage::$Stage" "real mark_action for $Stage"
    }
    Require-Text $Main 'Stage::TokenForge' 'TokenForge readiness exists'
    Reject-Text  $Main 'mark_action\([\s\S]{0,120}Stage::TokenForge' 'TokenForge must not contaminate reasoning action_state'
    Require-Text $Main 'action_convergence_complete\(\)' 'final package requires complete convergence'

    # Shadow replays raw evidence and cross-binds all nine organs.
    $Wire = Join-Path $Root 'shadow-contracts\src\lib.rs'
    Require-Text $Wire 'PROTOCOL_VERSION:\s*u16\s*=\s*5' 'wire protocol v5'
    Require-Text $Wire 'pub evidence_words:\s*Vec<u64>' 'wire carries canonical action words'
    Require-Text $Wire 'fn replay_action_evidence' 'Shadow-contract replay exists'
    Require-Text $Wire 'converge_action_words' 'Shadow recomputes contribution'
    Require-Text $Wire 'canonical_action_order' 'Shadow requires canonical order'
    Require-Text $Wire 'quantum_action_mask == self\.quantum_required_action_mask' 'exact action mask required'
    $ShadowBridge = Join-Path $Root 'shadow\src\process_bridge.rs'
    foreach ($StageConst in @(
        'ACTION_STAGE_HPRO','ACTION_STAGE_PRO','ACTION_STAGE_NPRO','ACTION_STAGE_SRK',
        'ACTION_STAGE_APRO','ACTION_STAGE_MPRO','ACTION_STAGE_PIM','ACTION_STAGE_NPIM','ACTION_STAGE_HCP'
    )) {
        Require-Text $ShadowBridge $StageConst "Shadow cross-binding $StageConst"
    }
    Require-Text $ShadowBridge 'ledger-i Untrust nuk lidhet fushë-për-fushë' 'fail-closed action cross-binding'

    # Living Trust Phase 2 and L-500.
    foreach ($Contract in @(
        'light\src\living_trust_contract.rs',
        'quantum\src\living_trust_contract.rs',
        'shadow\src\living_trust_contract.rs'
    )) {
        $P = Join-Path $Root $Contract
        Require-Text $P 'Sha256::digest\(&material\)' "$Contract real SHA-256"
        Require-Text $P 'GCL_LIVING_TRUST_V162' "$Contract domain separator"
        Require-Text $P 'SOVEREIGN_SEAL_PRIMITIVE:\s*u32\s*=\s*500' "$Contract L-500"
        Require-Text $P 'self\.action_mask == self\.required_action_mask' "$Contract exact organ mask"
        Require-Text $P 'TRUST_KIND_CONSTRUCTIVE' "$Contract constructive trust"
        Require-Text $P 'TRUST_KIND_RIGOROUS_NEGATIVE' "$Contract rigorous negative trust"
    }
    $Supreme = Join-Path $Root 'shadow\src\shadow_gj_legacy.rs'
    Require-Text $Supreme 'fn seal_living_trust' 'supreme trust pulse'
    Require-Text $Supreme 'SGL_SEAL_XOR' 'L-500 XOR live'
    Require-Text $Supreme 'SGL_SEAL_MASK' 'L-500 mask live'
    Require-Text $Supreme 'SGL_SEAL_PRIMITIVE' 'L-500 primitive live'
    Require-Text $Supreme 'Self::seal_living_trust\(verdict\)' 'supreme exits seal Trust'

    # All trust-governing receipt/token identities are SHA-256.
    foreach ($Receipt in @(
        'light\src\lab_contracts\verification_receipt.rs',
        'quantum\src\lab_contracts\verification_receipt.rs',
        'shadow\src\lab_contracts\verification_receipt.rs'
    )) {
        $P = Join-Path $Root $Receipt
        Require-Text $P 'use sha2::\{Digest, Sha256\}' "$Receipt imports SHA-256"
        Require-Text $P 'VERIFICATION_RECEIPT_VERSION:\s*u32\s*=\s*0x0001_0602' "$Receipt v1.6.2 domain"
        Require-Text $P 'Sha256::digest\(&proof\)' "$Receipt hashes canonical proof"
        Require-Text $P 'living_trust_sha256:\s*&\[u8;\s*32\]' "$Receipt binds full Living Trust"
        Require-Text $P 'value\.len\(\) == 64' "$Receipt ID is 64 hex chars"
        Reject-Text  $P 'format!\("\{:016x\}"\s*,\s*fnv1a64' "$Receipt must not use FNV identity"
    }
    $Forge = Join-Path $Root 'quantum\src\token_forge.rs'
    Require-Text $Forge 'use sha2::\{Digest, Sha256\}' 'TokenForge imports SHA-256'
    Require-Text $Forge 'pub type ForgeToken = \[u8;\s*32\]' 'TokenForge 32-byte identity'
    Require-Text $Forge 'Sha256::digest\(&material\)' 'TokenForge real SHA-256'
    Reject-Text  $Forge 'fnv1a' 'TokenForge must contain no FNV token'

    # Receipt → Living Trust → iZ → next i0 binding across Shadow/Quantum/Light.
    Require-Text $Wire 'pub living_trust_sha256:\s*String' 'full trust identity on wire'
    Require-Text $Wire 'w\.string\(&x\.living_trust_sha256\)' 'receipt wire encoder carries full Trust'
    Require-Text $Wire 'living_trust_sha256:\s*r\.string\(\)\?' 'receipt wire decoder carries full Trust'
    Require-Text (Join-Path $Root 'shadow\src\sovereign_ffi_gate.rs') 'living_trust_sha256:\s*verdict\.living_trust_sha256' 'Shadow receipt stores full Trust'
    Require-Text (Join-Path $Root 'quantum\src\main.rs') 'receipt_trust_identity == trust_identity' 'Quantum compares receipt and verdict Trust'
    Require-Text (Join-Path $Root 'quantum\src\progressive_debatic\runtime.rs') 'completion\.living_trust_sha256' 'PD carries full Trust into iZ'
    Require-Text (Join-Path $Root 'quantum\src\progressive_debatic\runtime.rs') 'pending\.pre_seal\.action_sha256' 'iZ binds GCL action token'
    Require-Text (Join-Path $Root 'light\src\pd_light.rs') 'compute_with_intensity' 'Light independently recomputes Trust'
    Require-Text (Join-Path $Root 'light\src\pd_light.rs') 'round_trip_37_fields' 'Light v1.6.2 handoff schema'
    Require-Text (Join-Path $Root 'quantum\src\main.rs') '37 fusha trupi \+ CRC \(38 total\)' 'Quantum/Light handoff count'

    # Documentation and parallel document.
    Require-Text (Join-Path $Root 'ess-mai.md') 'Evolucioni v1\.6\.1 → v1\.6\.2' 'authoritative evolution entry'
    $ParallelDoc = Join-Path (Split-Path -Parent $Root) 'ess_mai.md'
    if (-not (Test-Path -LiteralPath $ParallelDoc)) { throw 'parallel ess_mai.md missing' }
    $DocHashes = @(
        (Get-FileHash -LiteralPath (Join-Path $Root 'ess-mai.md') -Algorithm SHA256).Hash,
        (Get-FileHash -LiteralPath $ParallelDoc -Algorithm SHA256).Hash
    ) | Select-Object -Unique
    if ($DocHashes.Count -ne 1) { throw 'parallel ess_mai.md is not byte-identical' }

    @(
        'STATIC_GUARDS_OK',
        $TrustHash, $ReceiptHash, $ContinuumHash, $SpineHash, $GclHash
    ) | Set-Content -LiteralPath (Join-Path $LogRoot '01_static_guards.log') -Encoding UTF8

    # Cargo-green is the release proof. Any non-zero exit keeps v1.6.2 pending.
    Run-Logged '02_build_workspace_all_targets' { cargo build --workspace --all-targets }
    Run-Logged '03_check_workspace_all_targets' { cargo check --workspace --all-targets }
    Run-Logged '04_test_compile_only' { cargo test --workspace --all-targets --no-run }
    Run-Logged '05_test_workspace' { cargo test --workspace }
    Run-Logged '06_clippy_workspace' { cargo clippy --workspace --all-targets -- -W clippy::all }

    $FmtLog = Join-Path $LogRoot '07_fmt_check_report.log'
    cargo fmt --all -- --check 2>&1 | Tee-Object -FilePath $FmtLog
    "FMT_EXIT_CODE=$LASTEXITCODE" | Add-Content -LiteralPath $FmtLog -Encoding UTF8

    Push-Location (Join-Path $Root 'ui')
    try { Run-Logged '08_check_new_ui' { cargo check --all-targets } }
    finally { Pop-Location }

    Push-Location (Join-Path $Root 'light\ui\src-tauri')
    try { Run-Logged '09_check_old_ui_emotional_engine' { cargo check --all-targets } }
    finally { Pop-Location }

    $ShadowExe = Join-Path $Root 'target\debug\shadow_platform.exe'
    if (-not (Test-Path -LiteralPath $ShadowExe)) {
        throw "Shadow main executable missing after workspace build: $ShadowExe"
    }

    $Manifest = Join-Path $Root 'ESS_MAI_V1_6_2_FILELIST.sha256'
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
    'CARGO_GREEN=TRUE' | Set-Content -LiteralPath (Join-Path $LogRoot '11_release_gate.log') -Encoding UTF8

    Write-Host "ESS-MAI v1.6.2 validation completed. CARGO_GREEN=TRUE. Logs: $LogRoot"
}
finally {
    Pop-Location
}
