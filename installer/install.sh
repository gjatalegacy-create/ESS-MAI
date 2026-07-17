#!/bin/sh
# ╔════════════════════════════════════════════════════════════╗
# ║  install.sh — Wrapper Linux/Mac për ESS-MAI (Nura Legacy)  ║
# ║  GJATA LEGACY™                                             ║
# ╚════════════════════════════════════════════════════════════╝
set -e
echo "Nura Legacy (ESS-MAI) — Instalim"
echo "Duke kontrolluar Python3..."
if ! command -v python3 >/dev/null 2>&1; then
  echo "Python3 mungon. Instaloje së pari."
  exit 1
fi
cd "$(dirname "$0")"
python3 install.py "$@"
