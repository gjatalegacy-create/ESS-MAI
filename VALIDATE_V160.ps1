param(
    [string]$LogRoot = (Join-Path ([Environment]::GetFolderPath('Desktop')) 'ESS_MAI_V160_LOGS')
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

    # Version identity.
    foreach ($Manifest in @(
        'light\Cargo.toml', 'quantum\Cargo.toml', 'shadow\Cargo.toml',
        'shadow-contracts\Cargo.toml', 'ui\Cargo.toml',
        'light\ui\src-tauri\Cargo.toml'
    )) {
        Require-Text (Join-Path $Root $Manifest) 'version\s*=\s*"1\.6\.0"' "$Manifest must be v1.6.0"
    }
    Require-Text (Join-Path $Root 'VERSION_ESSMAI.txt') 'ESS-MAI v1\.6\.0' 'version marker'

    # v1.5.9 complete mediation must remain intact.
    Require-Text (Join-Path $Root 'shadow\Cargo.toml') 'autolib\s*=\s*false' 'Shadow must remain binary-only'
    Reject-Text  (Join-Path $Root 'shadow\Cargo.toml') '(?m)^\[lib\]' 'Shadow [lib] target must not return'
    Reject-Text  (Join-Path $Root 'shadow\Cargo.toml') 'rlib|staticlib' 'Shadow linkable crate-types must not return'
    Require-Text (Join-Path $Root 'shadow\src\main.rs') 'include!\("lib\.rs"\)' 'main.rs must include Shadow core'
    Require-Text (Join-Path $Root 'shadow\src\main.rs') 'process_bridge::dispatch_from_args' 'main.rs must own mediation'
    Require-Text (Join-Path $Root 'quantum\Cargo.toml') 'shadow_contracts\s*=\s*\{\s*path\s*=\s*"\.\./shadow-contracts"' 'Quantum must know only public Shadow wire contracts'
    Reject-Text  (Join-Path $Root 'quantum\Cargo.toml') 'path\s*=\s*"\.\./shadow"|package\s*=\s*"shadow_platform"' 'Quantum must not link Shadow core'
    Reject-Text  (Join-Path $Root 'quantum\src\main.rs') 'shadow_lib::|shadow_platform::' 'Quantum main must not call Shadow core'
    Require-Text (Join-Path $Root 'quantum\src\main.rs') "source:\s*&'static\s+str" 'E0521 lifetime correction must remain'
    Require-Text (Join-Path $Root 'quantum\src\shadow_process_bridge.rs') 'Command::new\(&shadow_bin\)' 'Quantum must execute Shadow main'

    # GCL must exist before Spine 9 and remain the same through all Layers.
    foreach ($Contract in @(
        'light\src\pd_spine_contract.rs',
        'quantum\src\pd_spine_contract.rs',
        'shadow\src\pd_spine_contract.rs'
    )) {
        $P = Join-Path $Root $Contract
        Require-Text $P 'pub struct GclProcessAuthority' "$Contract GCL process authority"
        Require-Text $P 'pub gcl_process_digest:\s*u64' "$Contract Layer GCL binding"
        Require-Text $P 'PD_LAYER_RECEIPT_UNDER_GCL_V160' "$Contract Layer receipt version"
        Require-Text $P 'PD_SPINE9_COMPLETE_UNDER_GCL_V160' "$Contract completion version"
        Require-Text $P 'let required_layer_mask = ALL_LAYERS_MASK' "$Contract must activate all Layers"
        Require-Text $P 'pub result_material_digest:\s*u64' "$Contract must bind Layer result material"
    }
    $ContractHashEvidence = Require-SameHash @(
        (Join-Path $Root 'light\src\pd_spine_contract.rs'),
        (Join-Path $Root 'quantum\src\pd_spine_contract.rs'),
        (Join-Path $Root 'shadow\src\pd_spine_contract.rs')
    ) 'PD_SPINE_CONTRACT_BYTE_IDENTICAL'

    # Final PIM/NPIM/MPRO evidence and input lineage.
    Require-Text (Join-Path $Root 'shadow-contracts\src\lib.rs') 'PROTOCOL_VERSION:\s*u16\s*=\s*2' 'wire protocol v2'
    Require-Text (Join-Path $Root 'shadow-contracts\src\lib.rs') 'pub struct PdContinuumEvidenceWire' 'recomputable i+U→i0→1Q evidence contract'
    Require-Text (Join-Path $Root 'shadow-contracts\src\lib.rs') 'pub struct PdActivationEvidenceWire' 'recomputable PD cognitive activation contract'
    Require-Text (Join-Path $Root 'shadow-contracts\src\lib.rs') 'pub fn recompute_stimulus_digest' 'Shadow wire recomputes i+U→i0 stimulus'
    Require-Text (Join-Path $Root 'shadow-contracts\src\lib.rs') 'pub fn recompute_increment_digest' 'Shadow wire recomputes 1Q increment'
    Require-Text (Join-Path $Root 'shadow-contracts\src\lib.rs') 'pub fn recompute_contract_digest' 'Shadow wire recomputes PD activation contract'
    Require-Text (Join-Path $Root 'shadow-contracts\src\lib.rs') 'pub struct PdLayerEvidenceWire' 'recomputable Layer evidence contract'
    Require-Text (Join-Path $Root 'shadow-contracts\src\lib.rs') 'pub struct PdSpineEvidenceWire' 'recomputable Spine evidence contract'
    Require-Text (Join-Path $Root 'shadow-contracts\src\lib.rs') 'pub result_material_digest:\s*u64' 'Layer result material binding on wire'
    Require-Text (Join-Path $Root 'shadow-contracts\src\lib.rs') 'pub result_material:\s*Vec<u8>' 'Layer canonical source material on wire'
    Require-Text (Join-Path $Root 'shadow-contracts\src\lib.rs') 'pub fn recompute_material_digest' 'Shadow wire recomputes Layer source material'
    Require-Text (Join-Path $Root 'shadow-contracts\src\lib.rs') 'pub fn recompute_activation_id' 'Shadow wire recomputes Spine activation'
    Require-Text (Join-Path $Root 'shadow-contracts\src\lib.rs') 'pub fn recompute_completion_digest' 'Shadow wire recomputes Spine completion'
    Require-Text (Join-Path $Root 'shadow-contracts\src\lib.rs') 'pub struct FinalEvidenceWire' 'final evidence contract'
    Require-Text (Join-Path $Root 'shadow-contracts\src\lib.rs') 'pub mpro_measurements:\s*Vec<u8>' 'sixteen MPRO evidence values'
    Require-Text (Join-Path $Root 'shadow-contracts\src\lib.rs') 'pub light_input_sha256:\s*String' 'Light SHA-256 lineage'
    Require-Text (Join-Path $Root 'shadow-contracts\src\lib.rs') 'pub npim_arguments_blob_digest:\s*u64' 'NPIM blob binding'
    Require-Text (Join-Path $Root 'quantum\src\main.rs') 'input_sha256=.*Light→Quantum' 'Quantum must verify Light input SHA-256'
    Require-Text (Join-Path $Root 'quantum\src\main.rs') 'mpro_measurements\.extend_from_slice\(&pro4\)' 'MPRO PRO measurements'
    Require-Text (Join-Path $Root 'quantum\src\main.rs') 'mpro_measurements\.extend_from_slice\(&npro4\)' 'MPRO NPRO measurements'
    Require-Text (Join-Path $Root 'quantum\src\main.rs') 'mpro_measurements\.extend_from_slice\(&hpro4\)' 'MPRO HPRO measurements'
    Require-Text (Join-Path $Root 'quantum\src\main.rs') 'mpro_measurements\.extend_from_slice\(&apro_arg\.measures\)' 'MPRO APRO measurements'
    Require-Text (Join-Path $Root 'shadow\src\process_bridge.rs') 'Sha256::digest\(&evidence\.light_input_bytes\)' 'Shadow must recompute Light SHA-256'
    Require-Text (Join-Path $Root 'shadow\src\process_bridge.rs') 'NaN/Infinity u refuzua' 'finite-number gate'
    Require-Text (Join-Path $Root 'shadow\src\process_bridge.rs') 'evidence\.verifies_internal\(\)' 'Shadow independently verifies final evidence'
    Require-Text (Join-Path $Root 'shadow\src\process_bridge.rs') 'continuum_identity' 'Shadow cross-checks continuum against Light/Quantum identity'
    Require-Text (Join-Path $Root 'shadow\src\process_bridge.rs') 'spine_material_identity' 'Shadow cross-checks canonical Layer source material'
    Require-Text (Join-Path $Root 'shadow\src\process_bridge.rs') 'I_PLUS_U_TO_I0|i \+ U → i₀ → 1Q' 'Shadow retains i+U→i0→1Q proof boundary'
    Require-Text (Join-Path $Root 'shadow\src\process_bridge.rs') 'mpro_factic_mass' 'Shadow MPRO recomputation check'
    Require-Text (Join-Path $Root 'shadow\src\process_bridge.rs') 'negative\.suggestion_code == evidence\.npim_suggestion' 'NPIM suggestion binding'

    # PD Light courier and the real old-emotional-UI → Tauri → new-UI transport.
    Require-Text (Join-Path $Root 'light\src\pd_light.rs') 'pub struct VerifiedPdDelivery' 'typed PD Light delivery'
    Require-Text (Join-Path $Root 'light\src\legacy_emotional_ui.rs') 'pub struct LegacyEmotionalTransmission' 'typed old-UI transmission'
    Require-Text (Join-Path $Root 'light\src\legacy_emotional_ui.rs') '\[PD_LIGHT/IZ\]' 'iZ emotional runtime marker'
    Require-Text (Join-Path $Root 'light\src\legacy_emotional_ui.rs') 'source=OLD_UI_EMOTIONAL_ENGINE target=NEW_UI' 'old UI targets new UI'
    Require-Text (Join-Path $Root 'light\src\main.rs') 'transmission\.as_str\(\)' 'Light stdout carries old-UI transmission'
    Require-Text (Join-Path $Root 'light\src\main.rs') '\[LIGHT_EMOTIONAL_SPINE\]' 'Light emotional spine label'
    Reject-Text  (Join-Path $Root 'light\src\main.rs') '\[SPINE9\]' 'Light runtime must not claim Quantum PD Spine 9'
    Require-Text (Join-Path $Root 'ui_contracts\emotional_command.rs') 'output\.contains\("\[PD_LIGHT/IZ\]"\)' 'Tauri emotional parser consumes iZ marker'
    Require-Text (Join-Path $Root 'ui_contracts\emotional_command.rs') 'OLD_UI_EMOTIONAL_ENGINE' 'new UI preserves old emotional source'

    # Legacy Shadow remains distinct and present.
    Require-Text (Join-Path $Root 'light\src\shadow_seal_bridge.rs') 'ShadowSealBridge' 'Legacy Shadow bridge remains'
    Require-Text (Join-Path $Root 'light\src\main.rs') 'SHADOW_GJ_LEGACY' 'Legacy Shadow runtime observation remains'

    @(
        'STATIC_GUARDS_OK',
        $ContractHashEvidence
    ) | Set-Content -LiteralPath (Join-Path $LogRoot '01_static_guards.log') -Encoding UTF8

    Run-Logged '02_build_workspace_all_targets' { cargo build --workspace --all-targets }
    Run-Logged '03_check_workspace_all_targets' { cargo check --workspace --all-targets }
    Run-Logged '04_test_compile_only' { cargo test --workspace --all-targets --no-run }
    Run-Logged '05_test_workspace' { cargo test --workspace }
    Run-Logged '06_clippy_workspace' { cargo clippy --workspace --all-targets -- -W clippy::all }

    # Existing style debt is recorded but does not overwrite architectural proof.
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
    "SHADOW_MAIN_PRESENT $ShadowExe" | Set-Content -LiteralPath (Join-Path $LogRoot '10_shadow_main_present.log') -Encoding UTF8

    $Manifest = Join-Path $Root 'ESS_MAI_V1_6_0_FILELIST.sha256'
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
    $Failures | Set-Content -LiteralPath (Join-Path $LogRoot '11_hash_failures.log') -Encoding UTF8
    if ($Failures.Count -ne 0) { throw "File-list verification failed: $($Failures.Count)" }
    'HASH_OK' | Set-Content -LiteralPath (Join-Path $LogRoot '11_hash_ok.log') -Encoding UTF8

    Write-Host "ESS-MAI v1.6.0 validation completed. Logs: $LogRoot"
}
finally {
    Pop-Location
}
