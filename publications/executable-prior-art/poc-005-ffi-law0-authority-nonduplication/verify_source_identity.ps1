param(
    [Parameter(Mandatory = $true)]
    [string]$V189SourceRoot
)

$ErrorActionPreference = "Stop"
$capsuleRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$records = @(
    @{
        Source = "shadow\src\sovereign_ffi_gate.rs"
        Copy = "source_core\src\sovereign_ffi_gate.rs"
        Sha256 = "88ed03f02697ca772fb8d6bfd8575f16ce6404e720265a423ecffaa001081142"
    },
    @{
        Source = "shadow\src\living_trust_contract.rs"
        Copy = "source_core\src\living_trust_contract.rs"
        Sha256 = "7107a2db3b8fd8e05fa2ad12093cd4fa55cf5fb35e76154189a88edecab4bcc2"
    },
    @{
        Source = "shadow\src\lab_contracts\verification_receipt.rs"
        Copy = "source_core\src\verification_receipt.rs"
        Sha256 = "93f010fbd3171c8ec8f452e3f7332df5c529eaefa7ce6afd910200b0381b8d9c"
    }
)

$failed = $false
foreach ($record in $records) {
    $source = Join-Path $V189SourceRoot $record.Source
    $copy = Join-Path $capsuleRoot $record.Copy
    $sourceHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $source).Hash.ToLowerInvariant()
    $copyHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $copy).Hash.ToLowerInvariant()
    $ok = $sourceHash -eq $record.Sha256 -and $copyHash -eq $record.Sha256
    [pscustomobject]@{
        Source = $record.Source
        ExpectedSha256 = $record.Sha256
        SourceSha256 = $sourceHash
        CopySha256 = $copyHash
        Match = $ok
    }
    if (-not $ok) {
        $failed = $true
    }
}

if ($failed) {
    "SOURCE_IDENTITY_STATUS=FAIL"
    throw "At least one focal source identity check failed."
}

"SOURCE_IDENTITY_MATCHED=$($records.Count)/$($records.Count)"
"SOURCE_IDENTITY_STATUS=PASS"
