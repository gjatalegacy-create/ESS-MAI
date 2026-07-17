param(
    [string]$LogRoot = (Join-Path ([Environment]::GetFolderPath('Desktop')) 'ESS_MAI_V161_LOGS')
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
        Require-Text (Join-Path $Root $Manifest) 'version\s*=\s*"1\.6\.1"' "$Manifest must be v1.6.1"
    }
    Require-Text (Join-Path $Root 'VERSION_ESSMAI.txt') 'ESS-MAI v1\.6\.1' 'version marker'

    # Complete mediation remains non-bypassable.
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

    # Living Trust Phase 1: zero-SHA action convergence.
    $Pulse = Join-Path $Root 'quantum\src\runtime_pulse.rs'
    Require-Text $Pulse 'static ACTION_STATE:\s*AtomicU64' 'global per-cycle action state'
    Require-Text $Pulse 'ACTION_STATE\.store\(0' 'distrust resets every cycle'
    Require-Text $Pulse 'pub fn converge_words' 'light convergence exists'
    Require-Text $Pulse 'rotate_left' 'rotation convergence'
    Require-Text $Pulse 'wrapping_add' 'wrapping addition convergence'
    Reject-Text  $Pulse 'Sha256|sha2::' 'Phase 1 must contain zero SHA-256'
    foreach ($Stage in @('Pro','Npro','Npim','Pim','Apro','Mpro')) {
        Require-Text (Join-Path $Root 'quantum\src\main.rs') "Stage::$Stage" "action stage $Stage"
    }
    Require-Text (Join-Path $Root 'quantum\src\main.rs') 'runtime_pulse::action_state\(\)' 'action state enters final evidence'

    # Living Trust Phase 2 and L-500.
    foreach ($Contract in @(
        'light\src\living_trust_contract.rs',
        'quantum\src\living_trust_contract.rs',
        'shadow\src\living_trust_contract.rs'
    )) {
        $P = Join-Path $Root $Contract
        Require-Text $P 'Sha256::digest\(&material\)' "$Contract real SHA-256"
        Require-Text $P 'GCL_LIVING_TRUST_V161' "$Contract domain separator"
        Require-Text $P 'SOVEREIGN_SEAL_PRIMITIVE:\s*u32\s*=\s*500' "$Contract L-500"
        Require-Text $P 'TRUST_KIND_CONSTRUCTIVE' "$Contract constructive trust"
        Require-Text $P 'TRUST_KIND_RIGOROUS_NEGATIVE' "$Contract negative trust"
        Require-Text $P '&self\.intensity\.to_le_bytes\(\)' "$Contract intensity bound outside identity"
    }
    $Supreme = Join-Path $Root 'shadow\src\shadow_gj_legacy.rs'
    Require-Text $Supreme 'fn seal_living_trust' 'supreme trust pulse'
    Require-Text $Supreme 'SGL_SEAL_XOR' 'existing L-500 XOR live'
    Require-Text $Supreme 'SGL_SEAL_MASK' 'existing L-500 mask live'
    Require-Text $Supreme 'SGL_SEAL_PRIMITIVE' 'existing L-500 primitive live'
    Require-Text $Supreme 'Self::seal_living_trust\(verdict\)' 'all supreme exits seal trust'
    Require-Text $Supreme 'legacy_score_compute' 'existing weighted trust intensity source'

    # Wire ×3, receipt and PD/iZ/next-i0 binding.
    $Wire = Join-Path $Root 'shadow-contracts\src\lib.rs'
    Require-Text $Wire 'PROTOCOL_VERSION:\s*u16\s*=\s*3' 'wire protocol v3'
    Require-Text $Wire 'pub living_trust_sha256:\s*String' 'full trust identity on wire'
    Require-Text $Wire 'pub living_trust_digest:\s*u64' 'receipt trust binding on wire'
    Require-Text $Wire 'w\.string\(&v\.living_trust_sha256\)' 'wire encoder carries Trust'
    Require-Text $Wire 'living_trust_sha256:\s*r\.string\(\)\?' 'wire decoder carries Trust'

    foreach ($Receipt in @(
        'light\src\lab_contracts\verification_receipt.rs',
        'quantum\src\lab_contracts\verification_receipt.rs',
        'shadow\src\lab_contracts\verification_receipt.rs'
    )) {
        Require-Text (Join-Path $Root $Receipt) 'living_trust_digest:\s*u64' "$Receipt receipt binds Trust"
        Require-Text (Join-Path $Root $Receipt) '&living_trust_digest\.to_le_bytes\(\)' "$Receipt receipt ID material"
    }

    Require-Text (Join-Path $Root 'shadow\src\sovereign_ffi_gate.rs') 'living_trust_digest' 'Shadow sovereign receipt binds Trust'
    Require-Text (Join-Path $Root 'quantum\src\main.rs') 'verification_receipt\.living_trust_digest == living_trust_digest' 'Quantum verifies receipt Trust binding'
    Require-Text (Join-Path $Root 'quantum\src\progressive_debatic\types.rs') 'pub living_trust_sha256:\s*\[u8;\s*32\]' 'PD carries full Trust identity'
    Require-Text (Join-Path $Root 'quantum\src\progressive_debatic\runtime.rs') 'living_trust_digest' 'PD runtime binds Trust'
    Require-Text (Join-Path $Root 'quantum\src\pd_continuum_contract.rs') 'PD_COMPLETES_IZ_WITH_LIVING_TRUST_V161' 'Trust seeds iZ'
    Require-Text (Join-Path $Root 'light\src\pd_light.rs') 'compute_with_intensity' 'Light independently recomputes Trust'
    Require-Text (Join-Path $Root 'light\src\pd_light.rs') '35 fushat e trupit' 'v1.6.1 handoff schema'
    Require-Text (Join-Path $Root 'quantum\src\main.rs') '35 fusha trupi \+ CRC \(36 total\)' 'Quantum/Light handoff count'

    # Documentation and parallel document.
    Require-Text (Join-Path $Root 'ess-mai.md') 'Evolucioni v1\.6\.0 → v1\.6\.1' 'authoritative evolution entry'
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

    $Manifest = Join-Path $Root 'ESS_MAI_V1_6_1_FILELIST.sha256'
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

    Write-Host "ESS-MAI v1.6.1 validation completed. Logs: $LogRoot"
}
finally {
    Pop-Location
}
