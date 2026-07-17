@echo off
REM ╔════════════════════════════════════════════════════════════════════════╗
REM ║  setup_essmai.bat — PIKA E HYRJES ME DY-KLIK                            ║
REM ║  GJATA LEGACY™ — Gardiani i ESS-MAI                                      ║
REM ║                                                                        ║
REM ║  Klikoni DY HERE kete skedar. Ai thërret motorin PowerShell vetë-      ║
REM ║  shërues (setup_essmai.ps1), i cili ngre lejet (UAC) dhe instalon       ║
REM ║  gjithçka automatikisht: Rust, cargo, gcc/g++, MSVC c++, WebView2,      ║
REM ║  Tauri, pastaj kompilon C kernel-in dhe ben cargo build.                ║
REM ╚════════════════════════════════════════════════════════════════════════╝
title ESS-MAI - Instalimi Sovran (setup_essmai)
color 0B

echo.
echo  ============================================================
echo    ESS-MAI - INSTALIMI SOVRAN
echo    Gardiani i ESS-MAI / Gjata Legacy
echo  ============================================================
echo.
echo    Ky instalim do te:
echo      - instaloje Rust + cargo (host MSVC)
echo      - instaloje Visual Studio C++ Build Tools (cl.exe)
echo      - instaloje MSYS2 + gcc/g++ (winpthreads)
echo      - instaloje WebView2 + Tauri CLI
echo      - kompiloje C kernel-in (prove ekzekutimi)
echo      - beje cargo build --release
echo.
echo    Do te kerkohet leje administratori (UAC). Kjo eshte normale.
echo.
echo  ------------------------------------------------------------
echo.

REM Gjej skedarin .ps1 ne te njejten dosje me kete .bat.
set "PS1=%~dp0setup_essmai.ps1"

if not exist "%PS1%" (
  echo  [GABIM] Nuk u gjet setup_essmai.ps1 ne kete dosje:
  echo          %~dp0
  echo  Vendosni te dy skedaret ne rrenjen e projektit ess_mai.
  echo.
  pause
  exit /b 1
)

REM Lesho motorin PowerShell me politike te anashkaluar (process-scope).
REM .ps1 vete e ngre veten me UAC nese s'jemi admin.
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%PS1%"

echo.
echo  ------------------------------------------------------------
echo    Instalimi u mbyll. Nese u hap nje dritare e re me UAC,
echo    instalimi vazhdoi atje (shih logun e shfaqur).
echo  ------------------------------------------------------------
echo.
pause
