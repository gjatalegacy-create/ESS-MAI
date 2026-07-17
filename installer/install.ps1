# ╔════════════════════════════════════════════════════════════╗
# ║  install.ps1 — Wrapper Windows për ESS-MAI (Nura Legacy)   ║
# ║  GJATA LEGACY™                                             ║
# ╚════════════════════════════════════════════════════════════╝
Write-Host "Nura Legacy (ESS-MAI) — Instalim"
Write-Host "Duke kontrolluar Python..."
if (-not (Get-Command python -ErrorAction SilentlyContinue)) {
    Write-Host "Python mungon. Instaloje së pari nga python.org"
    exit 1
}
Set-Location $PSScriptRoot
python install.py $args
