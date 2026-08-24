$ErrorActionPreference = 'Stop'

$pocRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$manifestPath = Join-Path $pocRoot 'EXTRACTION_MANIFEST.sha256'
$verified = 0

foreach ($row in Get-Content -LiteralPath $manifestPath) {
    if ($row -notmatch '^([0-9a-f]{64})  (.+)$') {
        throw "Invalid extraction-manifest row: $row"
    }

    $expected = $Matches[1]
    $relative = $Matches[2].Replace('/', [IO.Path]::DirectorySeparatorChar)
    $candidate = Join-Path $pocRoot $relative

    if (-not (Test-Path -LiteralPath $candidate -PathType Leaf)) {
        throw "Missing extracted file: $relative"
    }

    $actual = (Get-FileHash -Algorithm SHA256 -LiteralPath $candidate).Hash.ToLowerInvariant()
    if ($actual -ne $expected) {
        throw "Extraction identity mismatch: $relative expected=$expected actual=$actual"
    }

    Write-Output "PASS $expected  $($Matches[2])"
    $verified++
}

Write-Output 'EXTRACTION_IDENTITY=PASS'
Write-Output "FILES_VERIFIED=$verified"
