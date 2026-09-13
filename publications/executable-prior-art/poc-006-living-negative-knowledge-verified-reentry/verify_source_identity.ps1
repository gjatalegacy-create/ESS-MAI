param(
    [Parameter(Mandatory = $true)]
    [string]$V189Root
)

$ErrorActionPreference = 'Stop'
$pocRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$checks = @(
    @('source_core\src\pro_nk_gate.rs', 'quantum\src\pro_nk_gate.rs', '66f14e82141b7c3e18d9e7f487093899fb3a033ae86ebb804d47749a5ba196c6'),
    @('source_core\src\tokenizer.rs', 'quantum\src\tokenizer.rs', 'b7214b7f3c336244cdb2fd373c60dc267521a5e7485396e9f2598f33620fa6df'),
    @('source_core\src\runtime_pulse.rs', 'quantum\src\runtime_pulse.rs', '903547c3900de4f3892c6b8baa382743986558efebbff5f58faedc9dde95969c'),
    @('shadow-contracts\src\negative_asset.rs', 'shadow-contracts\src\negative_asset.rs', 'ecbab84bfc65f2561be133f12b047f5e863423c823e294f5bec8f1e06fb673eb')
)

$passed = 0
foreach ($check in $checks) {
    $copy = Join-Path $pocRoot $check[0]
    $source = Join-Path $V189Root $check[1]
    $copyHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $copy).Hash.ToLowerInvariant()
    $sourceHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $source).Hash.ToLowerInvariant()
    $ok = ($copyHash -eq $sourceHash) -and ($copyHash -eq $check[2])
    if ($ok) { $passed++ }
    "SOURCE_IDENTITY_FILE=$($check[0]) MATCH=$($ok.ToString().ToLowerInvariant()) SHA256=$copyHash"
}

"SOURCE_IDENTITY_MATCHED=$passed/$($checks.Count)"
"SOURCE_IDENTITY_STATUS=$(if ($passed -eq $checks.Count) { 'PASS' } else { 'FAIL' })"
if ($passed -ne $checks.Count) { exit 1 }

