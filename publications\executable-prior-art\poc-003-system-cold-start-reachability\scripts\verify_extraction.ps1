$ErrorActionPreference = 'Stop'

$pocRoot = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)
$manifestPath = Join-Path $pocRoot 'EXTRACTION_MANIFEST.sha256'
$wholeFilesVerified = 0
$excerptVerified = $false

function Get-NormalizedLfExcerptSha256 {
    param(
        [Parameter(Mandatory = $true)] [string] $Path,
        [Parameter(Mandatory = $true)] [int] $FirstLine,
        [Parameter(Mandatory = $true)] [int] $LastLine
    )

    $allLines = [IO.File]::ReadAllLines($Path)
    if ($FirstLine -lt 1 -or $LastLine -lt $FirstLine -or $LastLine -gt $allLines.Length) {
        throw "Invalid excerpt range $FirstLine-$LastLine for $Path"
    }

    $selected = $allLines[($FirstLine - 1)..($LastLine - 1)]
    $canonical = ($selected -join "`n") + "`n"
    $utf8 = [Text.UTF8Encoding]::new($false)
    $sha256 = [Security.Cryptography.SHA256]::Create()
    try {
        $hash = $sha256.ComputeHash($utf8.GetBytes($canonical))
        return [BitConverter]::ToString($hash).Replace('-', '').ToLowerInvariant()
    }
    finally {
        $sha256.Dispose()
    }
}

foreach ($row in Get-Content -LiteralPath $manifestPath) {
    if ([string]::IsNullOrWhiteSpace($row)) {
        continue
    }

    if ($row -match '^# EXCERPT-SHA256 ([0-9a-f]{64})  (\S+)  ([0-9]+)  ([0-9]+)$') {
        if ($excerptVerified) {
            throw 'Duplicate excerpt identity record'
        }

        $expected = $Matches[1]
        $relative = $Matches[2]
        $firstLine = [int]$Matches[3]
        $lastLine = [int]$Matches[4]
        $candidate = Join-Path $pocRoot ($relative.Replace('/', [IO.Path]::DirectorySeparatorChar))
        if (-not (Test-Path -LiteralPath $candidate -PathType Leaf)) {
            throw "Missing excerpt carrier: $relative"
        }

        $actual = Get-NormalizedLfExcerptSha256 -Path $candidate -FirstLine $firstLine -LastLine $lastLine
        if ($actual -ne $expected) {
            throw "Excerpt identity mismatch: $relative lines=$firstLine-$lastLine expected=$expected actual=$actual"
        }

        Write-Output "PASS EXCERPT $expected  $relative lines=$firstLine-$lastLine"
        $excerptVerified = $true
        continue
    }

    if ($row.StartsWith('#')) {
        continue
    }

    if ($row -notmatch '^([0-9a-f]{64})  (.+)$') {
        throw "Invalid extraction-manifest row: $row"
    }

    $expected = $Matches[1]
    $relative = $Matches[2]
    $candidate = Join-Path $pocRoot ($relative.Replace('/', [IO.Path]::DirectorySeparatorChar))
    if (-not (Test-Path -LiteralPath $candidate -PathType Leaf)) {
        throw "Missing extracted file: $relative"
    }

    $actual = (Get-FileHash -Algorithm SHA256 -LiteralPath $candidate).Hash.ToLowerInvariant()
    if ($actual -ne $expected) {
        throw "Whole-file identity mismatch: $relative expected=$expected actual=$actual"
    }

    Write-Output "PASS WHOLE-FILE $expected  $relative"
    $wholeFilesVerified++
}

if ($wholeFilesVerified -ne 20) {
    throw "Expected 20 whole-file extracts; verified $wholeFilesVerified"
}
if (-not $excerptVerified) {
    throw 'Excerpt identity record was not verified'
}

Write-Output 'EXTRACTION_IDENTITY=PASS'
Write-Output "WHOLE_FILES_VERIFIED=$wholeFilesVerified"
Write-Output 'EXCERPTS_VERIFIED=1'
