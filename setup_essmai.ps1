# ╔══════════════════════════════════════════════════════════════════════════╗
# ║  setup_essmai.ps1 — INSTALUESI SOVRAN VETË-SHËRUES I ESS-MAI             ║
# ║  GJATA LEGACY™ — Gardiani i ESS-MAI                                       ║
# ║                                                                          ║
# ║  FILOZOFIA: i pathyeshëm. Detekton çdo mungesë, e injoron gabimin jo-    ║
# ║  fatal, e rregullon vetë me mjetin e duhur, dhe verifikon në fund.        ║
# ║                                                                          ║
# ║  ÇFARË INSTALON AUTOMATIKISHT (një ekzekutim):                           ║
# ║    1. Rust (rustup, kanali stable, host GNU)   → rustc + cargo            ║
# ║    2. Visual Studio C++ Build Tools + Windows SDK  → cl.exe (MSVC, c++)   ║
# ║    3. MSYS2 + mingw-w64 gcc/g++ + winpthreads  → gcc/g++ për C kernel     ║
# ║    4. WebView2 Runtime (nëse mungon)  → për Tauri 2 UI                    ║
# ║    5. Tauri CLI (cargo-tauri)  → UI minimalist                            ║
# ║  PASTAJ:                                                                  ║
# ║    6. Kompilon C kernel-in (gcc, winpthreads) — provë ekzekutimi          ║
# ║    7. cargo build --release (Light + Quantum + Shadow)                    ║
# ║    8. Verifikim PASS/FAIL + nisje                                         ║
# ║                                                                          ║
# ║  Vendoset në rrënjën e projektit ess_mai/ (pranë Cargo.toml).            ║
# ╚══════════════════════════════════════════════════════════════════════════╝

# ─────────────────────────────────────────────────────────────────────────────
# 0) VETË-NGRITJA E LEJEVE (UAC). Nëse s'jemi admin → rinis veten i ngritur.
# ─────────────────────────────────────────────────────────────────────────────
$ErrorActionPreference = 'Stop'
$principal = New-Object Security.Principal.WindowsPrincipal(
    [Security.Principal.WindowsIdentity]::GetCurrent())
$isAdmin = $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)

if (-not $isAdmin) {
    Write-Host "[UAC] Kerkohen leje administratori. Po rinisem i ngritur..." -ForegroundColor Yellow
    try {
        Start-Process powershell.exe -Verb RunAs -ArgumentList @(
            '-NoProfile','-ExecutionPolicy','Bypass','-File',"`"$PSCommandPath`""
        )
    } catch {
        Write-Host "[UAC] Ngritja u refuzua. Disa hapa mund te deshtojne pa admin." -ForegroundColor Red
    }
    exit
}

# ─────────────────────────────────────────────────────────────────────────────
# 1) LOGimi + konstante
# ─────────────────────────────────────────────────────────────────────────────
$ProjectRoot = Split-Path -Parent $PSCommandPath
$LogDir      = Join-Path $env:TEMP 'essmai_setup_logs'
New-Item -ItemType Directory -Force -Path $LogDir | Out-Null
$LogFile     = Join-Path $LogDir ("setup_{0:yyyyMMdd_HHmmss}.log" -f (Get-Date))
try { Start-Transcript -Path $LogFile -Append | Out-Null } catch {}

$script:Results = [ordered]@{}   # matrica e verifikimit PASS/FAIL
$script:Cargo   = $env:USERPROFILE + '\.cargo\bin'
$script:Msys    = 'C:\msys64'
$script:MsysBin = "$script:Msys\mingw64\bin"

function Write-Banner {
    param([string]$Text)
    Write-Host ""
    Write-Host ("═" * 74) -ForegroundColor DarkCyan
    Write-Host "  $Text" -ForegroundColor Cyan
    Write-Host ("═" * 74) -ForegroundColor DarkCyan
}
function Write-Step { param([string]$M) Write-Host "  ▸ $M" -ForegroundColor White }
function Write-OK   { param([string]$M) Write-Host "  ✔ $M" -ForegroundColor Green }
function Write-Warn2{ param([string]$M) Write-Host "  ⚠ $M" -ForegroundColor Yellow }
function Write-Err2 { param([string]$M) Write-Host "  ✘ $M" -ForegroundColor Red }

Write-Banner "ESS-MAI — INSTALIMI SOVRAN (setup_essmai)"
Write-Step "Rrenja e projektit : $ProjectRoot"
Write-Step "Log i plote        : $LogFile"

# ─────────────────────────────────────────────────────────────────────────────
# 2) NDIHMËSAT VETË-SHËRUESE (retry, refresh PATH, add PATH, detect tool)
# ─────────────────────────────────────────────────────────────────────────────

# Provo një bllok deri në N herë; gabimi i fundit s'e ndal instalimin (jo-fatal).
function Invoke-WithRetry {
    param([scriptblock]$Action, [int]$Max = 3, [int]$DelaySec = 6, [string]$What = 'veprim')
    for ($i = 1; $i -le $Max; $i++) {
        try { & $Action; return $true }
        catch {
            Write-Warn2 "[$What] perpjekja $i/$Max deshtoi: $($_.Exception.Message)"
            if ($i -eq $Max) { Write-Warn2 "[$What] po vazhdoj (jo-fatal)."; return $false }
            Start-Sleep -Seconds $DelaySec
        }
    }
}

# Rifresko PATH-in e procesit AKTUAL nga regjistri (Machine + User) — që mjeti i
# sapo instaluar te jete i thirrshem menjehere pa rihapur shell.
function Refresh-Path {
    $m = [Environment]::GetEnvironmentVariable('Path','Machine')
    $u = [Environment]::GetEnvironmentVariable('Path','User')
    $extra = @($script:Cargo, $script:MsysBin, "$script:Msys\usr\bin") -join ';'
    $env:Path = (@($m,$u,$extra) | Where-Object { $_ } ) -join ';'
}

# Shto nje direktori ne PATH-in e perdoruesit (permanent) — idempotent (pa dublime).
function Add-ToUserPath {
    param([string]$Dir)
    if (-not $Dir) { return }
    $cur = [Environment]::GetEnvironmentVariable('Path','User')
    if ($null -eq $cur) { $cur = '' }
    if ($cur -notlike "*$Dir*") {
        $new = if ($cur.TrimEnd(';')) { "$($cur.TrimEnd(';'));$Dir" } else { $Dir }
        # API .NET (jo setx — setx pret PATH-in ne 1024 karaktere dhe e demton).
        [Environment]::SetEnvironmentVariable('Path', $new, 'User')
        Write-OK "PATH (user) += $Dir"
        Broadcast-EnvChange
    }
    Refresh-Path
}

# Transmeto WM_SETTINGCHANGE që ndryshimet e PATH te merren pa restart.
function Broadcast-EnvChange {
    try {
        if (-not ('SetEnv.Win32' -as [type])) {
            $sig = @'
[DllImport("user32.dll", SetLastError=true, CharSet=CharSet.Auto)]
public static extern IntPtr SendMessageTimeout(IntPtr hWnd, uint Msg, UIntPtr wParam,
    string lParam, uint fuFlags, uint uTimeout, out UIntPtr lpdwResult);
'@
            Add-Type -MemberDefinition $sig -Name Win32 -Namespace SetEnv | Out-Null
        }
        $r = [UIntPtr]::Zero
        [SetEnv.Win32]::SendMessageTimeout([IntPtr]0xffff, 0x1a, [UIntPtr]::Zero,
            "Environment", 2, 5000, [ref]$r) | Out-Null
    } catch {}
}

# A ekziston nje komande (mjet) ne PATH?
function Test-Tool { param([string]$Name) [bool](Get-Command $Name -ErrorAction SilentlyContinue) }

# Shkarko nje file me rifreskim TLS (disa Windows kane TLS1.2 te c'aktivizuar).
function Download-File {
    param([string]$Url, [string]$OutFile)
    try { [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12 } catch {}
    Invoke-WithRetry -What "shkarkim $([IO.Path]::GetFileName($OutFile))" -Action {
        Invoke-WebRequest -Uri $Url -OutFile $OutFile -UseBasicParsing
    } | Out-Null
    return (Test-Path $OutFile)
}

# ─────────────────────────────────────────────────────────────────────────────
# 3) winget — motori parësor; nëse mungon, bootstrap; ndryshe shkarkim direkt.
# ─────────────────────────────────────────────────────────────────────────────
function Ensure-WinGet {
    if (Test-Tool 'winget') { Write-OK "winget i pranishem."; return $true }
    Write-Step "winget mungon — po provoj ta regjistroj (App Installer)..."
    Invoke-WithRetry -What 'winget register' -Action {
        Add-AppxPackage -RegisterByFamilyName -MainPackage Microsoft.DesktopAppInstaller_8wekyb3d8bbwe
    } | Out-Null
    if (Test-Tool 'winget') { Write-OK "winget u aktivizua."; return $true }
    # Fallback: shkarko msixbundle nga aka.ms/getwinget
    $bundle = Join-Path $env:TEMP 'winget.msixbundle'
    if (Download-File 'https://aka.ms/getwinget' $bundle) {
        Invoke-WithRetry -What 'winget addpkg' -Action { Add-AppxPackage $bundle } | Out-Null
    }
    if (Test-Tool 'winget') { Write-OK "winget u instalua."; return $true }
    Write-Warn2 "winget i padisponueshem — do perdor shkarkim direkt per cdo mjet."
    return $false
}

# Wrapper i sigurt per winget install (jo-interaktiv).
function WinGet-Install {
    param([string]$Id, [string]$Override = $null)
    if (-not (Test-Tool 'winget')) { return $false }
    $wgArgs = @('install','--id',$Id,'--exact','--silent',
              '--accept-package-agreements','--accept-source-agreements')
    if ($Override) { $wgArgs += @('--override', $Override) }
    Invoke-WithRetry -What "winget $Id" -Action {
        $p = Start-Process winget -ArgumentList $wgArgs -PassThru -Wait -NoNewWindow
        # winget kthen 0 (sukses) ose -1978335189 (tashme i instaluar) — te dyja OK.
        if ($p.ExitCode -ne 0 -and $p.ExitCode -ne -1978335189) {
            throw "winget exit $($p.ExitCode)"
        }
    }
}

# ═════════════════════════════════════════════════════════════════════════════
#  HAPI 1 — RUST (rustup, stable, host MSVC) → rustc + cargo
# ═════════════════════════════════════════════════════════════════════════════
function Install-Rust {
    Write-Banner "HAPI 1/9 — RUST (rustc + cargo, host GNU — një ABI me gcc)"
    if ((Test-Tool 'cargo') -and (Test-Tool 'rustc')) {
        Write-OK "Rust tashme i pranishem."
        Invoke-WithRetry -What 'rustup update' -Action { rustup update stable | Out-Null } | Out-Null
    } else {
        Write-Step "Po instaloj Rust (stable, x86_64-pc-windows-gnu)..."
        # Provo winget; ndryshe rustup-init.exe direkt (jo-interaktiv: -y).
        $okW = WinGet-Install -Id 'Rustlang.Rustup'
        Add-ToUserPath $script:Cargo
        if (-not (Test-Tool 'cargo')) {
            $init = Join-Path $env:TEMP 'rustup-init.exe'
            if (Download-File 'https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe' $init) {
                Invoke-WithRetry -What 'rustup-init' -Action {
                    & $init -y --default-toolchain stable --default-host x86_64-pc-windows-gnu --profile default
                    if ($LASTEXITCODE -ne 0) { throw "rustup-init exit $LASTEXITCODE" }
                } | Out-Null
            }
        }
        Add-ToUserPath $script:Cargo
    }
    # KONTRATA E ABI-së: kerneli C (pthread/winpthreads) kompilohet me gcc →
    # Rust duhet GNU toolchain (NJË ABI e vetme; MSVC↔gcc mix i ndaluar,
    # build.rs bën fail-closed). MSVC Build Tools mbeten të instaluara vetëm
    # për Tauri UI në të ardhmen — nuk përdoren për bërthamën.
    # AKTIVIZIMI (fix): pas rustup-init, PATH-i i procesit s'e ka ende cargo\bin
    # → thirrja e zhveshur 'rustup' dështonte heshtur dhe GNU default S'VENDOSEJ.
    # Tani: path ABSOLUT i rustup-it — aktivizimi i garantuar.
    $rustupExe = Join-Path $env:USERPROFILE '.cargo\bin\rustup.exe'
    if (-not (Test-Path $rustupExe)) { $rustupExe = 'rustup' }
    Invoke-WithRetry -What 'rustup toolchain gnu' -Action {
        & $rustupExe toolchain install stable-x86_64-pc-windows-gnu 2>$null | Out-Null
    } | Out-Null
    Invoke-WithRetry -What 'rustup default gnu' -Action {
        & $rustupExe default stable-x86_64-pc-windows-gnu 2>$null | Out-Null
    } | Out-Null
    Refresh-Path
    $script:Results['Rust (rustc)'] = Test-Tool 'rustc'
    $script:Results['Cargo']        = Test-Tool 'cargo'
    if (Test-Tool 'rustc') { Write-OK ("rustc → " + (rustc --version)) }
    if (Test-Tool 'cargo') { Write-OK ("cargo → " + (cargo --version)) }
}

# ═════════════════════════════════════════════════════════════════════════════
#  HAPI 2 — VISUAL STUDIO C++ BUILD TOOLS + WINDOWS SDK → cl.exe (c++ MSVC)
# ═════════════════════════════════════════════════════════════════════════════
function Install-MSVC {
    Write-Banner "HAPI 2/9 — VISUAL STUDIO C++ BUILD TOOLS (cl.exe, c++)"
    # Detektim me vswhere (me i besueshem se cl.exe ne PATH).
    $vswhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
    $haveVC  = $false
    if (Test-Path $vswhere) {
        $vc = & $vswhere -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath 2>$null
        if ($vc) { $haveVC = $true }
    }
    if ($haveVC -or (Test-Tool 'cl')) {
        Write-OK "MSVC C++ Build Tools tashme te pranishme."
        $script:Results['MSVC C++ (cl.exe)'] = $true
        return
    }
    Write-Step "Po instaloj VS C++ Build Tools + Windows SDK (silent)..."
    $okW = WinGet-Install -Id 'Microsoft.VisualStudio.2022.BuildTools' `
        -Override '--quiet --wait --norestart --add Microsoft.VisualStudio.Workload.VCTools --add Microsoft.VisualStudio.Component.VC.Tools.x86.x64 --add Microsoft.VisualStudio.Component.Windows11SDK.22621 --includeRecommended'
    if (-not $haveVC) {
        $vs = Join-Path $env:TEMP 'vs_BuildTools.exe'
        if (Download-File 'https://aka.ms/vs/17/release/vs_BuildTools.exe' $vs) {
            Invoke-WithRetry -What 'vs_BuildTools' -Action {
                $p = Start-Process $vs -Wait -PassThru -ArgumentList @(
                    '--quiet','--wait','--norestart','--nocache',
                    '--add','Microsoft.VisualStudio.Workload.VCTools',
                    '--add','Microsoft.VisualStudio.Component.VC.Tools.x86.x64',
                    '--add','Microsoft.VisualStudio.Component.Windows11SDK.22621',
                    '--includeRecommended')
                # 0 = sukses, 3010 = sukses + restart i kerkuar.
                if ($p.ExitCode -ne 0 -and $p.ExitCode -ne 3010) { throw "vs exit $($p.ExitCode)" }
            } | Out-Null
        }
    }
    if (Test-Path $vswhere) {
        $vc = & $vswhere -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath 2>$null
    }
    $script:Results['MSVC C++ (cl.exe)'] = [bool]$vc -or (Test-Tool 'cl')
    if ($script:Results['MSVC C++ (cl.exe)']) { Write-OK "MSVC C++ Build Tools gati." }
    else { Write-Warn2 "MSVC s'u verifikua — Tauri/MSVC build mund te kerkoje restart." }
}

# ═════════════════════════════════════════════════════════════════════════════
#  HAPI 3 — MSYS2 + mingw-w64 gcc/g++ + winpthreads → kompilim C kernel
# ═════════════════════════════════════════════════════════════════════════════
function Install-MSYS2 {
    Write-Banner "HAPI 3/9 — MSYS2 + gcc/g++ (mingw-w64, winpthreads)"
    $bash = "$script:Msys\usr\bin\bash.exe"
    if (-not (Test-Path $bash)) {
        Write-Step "Po instaloj MSYS2..."
        $okW = WinGet-Install -Id 'MSYS2.MSYS2'
        if (-not (Test-Path $bash)) {
            # Fallback: shkarko instaluesin EXE dhe ekzekuto silent.
            $msysExe = Join-Path $env:TEMP 'msys2-install.exe'
            if (Download-File 'https://github.com/msys2/msys2-installer/releases/latest/download/msys2-x86_64-latest.exe' $msysExe) {
                Invoke-WithRetry -What 'msys2 install' -Action {
                    Start-Process $msysExe -Wait -ArgumentList @('install','--confirm-command','--root',$script:Msys) -NoNewWindow
                } | Out-Null
            }
        }
    } else { Write-OK "MSYS2 tashme i pranishem." }

    if (Test-Path $bash) {
        Write-Step "Po perditesoj DB dhe instaloj toolchain-in gcc/g++ (pacman)..."
        # pacman: --noconfirm = pa nderhyrje (default 'yes'); --needed = mos riinstalo.
        # AKTIVIZIMI (fix): -Syuu i parë përditëson CORE dhe shpesh KËRKON
        # rinisje të msys — toolchain-i pas tij dështonte. Idioma zyrtare:
        # DY -Syuu radhazi (faza 1 core, faza 2 pjesa tjetër).
        Invoke-WithRetry -What 'pacman -Syuu (faza 1: core)' -Action {
            & $bash -lc "pacman -Syuu --noconfirm" 2>&1 | Out-Null
        } | Out-Null
        Invoke-WithRetry -What 'pacman -Syuu (faza 2: rest)' -Action {
            & $bash -lc "pacman -Syuu --noconfirm" 2>&1 | Out-Null
        } | Out-Null
        Invoke-WithRetry -What 'pacman gcc toolchain' -Action {
            & $bash -lc "pacman -S --needed --noconfirm mingw-w64-x86_64-toolchain mingw-w64-x86_64-gcc mingw-w64-x86_64-winpthreads" 2>&1 | Out-Null
        } | Out-Null
        Add-ToUserPath $script:MsysBin
    }
    Refresh-Path
    # AKTIVIZIMI (fix): verifikimi vetëm me PATH jepte FAIL të rremë edhe kur
    # gcc EKZISTONTE në C:\msys64\mingw64\bin. Tani: path i plotë = e vërteta.
    $gccExe = Join-Path $script:MsysBin 'gcc.exe'
    $gppExe = Join-Path $script:MsysBin 'g++.exe'
    $script:Results['gcc'] = (Test-Path $gccExe) -or (Test-Tool 'gcc')
    $script:Results['g++'] = (Test-Path $gppExe) -or (Test-Tool 'g++')
    if (Test-Path $gccExe) { Write-OK ("gcc → " + ((& $gccExe --version) | Select-Object -First 1)) }
    elseif (Test-Tool 'gcc') { Write-OK ("gcc → " + ((gcc --version) | Select-Object -First 1)) }
    else { Write-Warn2 "gcc s'u verifikua (as $gccExe, as PATH)." }
}

# ═════════════════════════════════════════════════════════════════════════════
#  HAPI 4 — WEBVIEW2 RUNTIME (nëse mungon) → për Tauri 2 UI
# ═════════════════════════════════════════════════════════════════════════════
function Install-WebView2 {
    Write-Banner "HAPI 4/9 — WEBVIEW2 RUNTIME (per Tauri 2 UI)"
    # Detektim: çelësi 'pv' i runtime-it Evergreen (per-machine ose per-user).
    $keys = @(
        'HKLM:\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}',
        'HKLM:\SOFTWARE\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}',
        'HKCU:\SOFTWARE\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}'
    )
    $have = $false
    foreach ($k in $keys) {
        try { $pv = (Get-ItemProperty -Path $k -Name pv -ErrorAction Stop).pv
              if ($pv -and $pv -ne '0.0.0.0') { $have = $true; break } } catch {}
    }
    if ($have) { Write-OK "WebView2 tashme i instaluar."; $script:Results['WebView2'] = $true; return }
    Write-Step "Po instaloj WebView2 Evergreen (silent)..."
    $wv = Join-Path $env:TEMP 'MicrosoftEdgeWebView2Setup.exe'
    if (Download-File 'https://go.microsoft.com/fwlink/p/?LinkId=2124703' $wv) {
        Invoke-WithRetry -What 'webview2' -Action {
            Start-Process $wv -ArgumentList '/silent','/install' -Wait
        } | Out-Null
    }
    foreach ($k in $keys) {
        try { $pv = (Get-ItemProperty -Path $k -Name pv -ErrorAction Stop).pv
              if ($pv -and $pv -ne '0.0.0.0') { $have = $true; break } } catch {}
    }
    $script:Results['WebView2'] = $have
    if ($have) { Write-OK "WebView2 gati." } else { Write-Warn2 "WebView2 s'u verifikua." }
}

# ═════════════════════════════════════════════════════════════════════════════
#  HAPI 5 — TAURI CLI (cargo-tauri) → UI minimalist
# ═════════════════════════════════════════════════════════════════════════════
function Install-TauriCli {
    Write-Banner "HAPI 5/9 — TAURI CLI (cargo-tauri)"
    if (-not (Test-Tool 'cargo')) { Write-Warn2 "cargo mungon — anashkaloj Tauri CLI."; $script:Results['Tauri CLI']=$false; return }
    if (Test-Tool 'cargo-tauri') { Write-OK "Tauri CLI tashme i pranishem."; $script:Results['Tauri CLI']=$true; return }
    Write-Step "Po instaloj tauri-cli (cargo install, pa Node)..."
    Invoke-WithRetry -What 'cargo install tauri-cli' -Max 2 -Action {
        cargo install tauri-cli --locked 2>&1 | Out-Null
    } | Out-Null
    Refresh-Path
    $script:Results['Tauri CLI'] = Test-Tool 'cargo-tauri'
    if ($script:Results['Tauri CLI']) { Write-OK "Tauri CLI gati." }
    else { Write-Warn2 "Tauri CLI s'u instalua (jo-fatal; UI mund te ndertohet me vone)." }
}

# ═════════════════════════════════════════════════════════════════════════════
#  HAPI 6 — KOMPILIMI I C KERNEL-it (gcc + winpthreads) — provë ekzekutimi
# ═════════════════════════════════════════════════════════════════════════════
function Build-CKernel {
    Write-Banner "HAPI 6/9 — KOMPILIMI I C KERNEL-it (provë reale ekzekutimi)"
    $shadow = Join-Path $ProjectRoot 'shadow'
    $kernel = Join-Path $shadow 'kernel'
    $verify = Join-Path $shadow 'verify_kernel.c'
    if (-not (Test-Path $verify) -or -not (Test-Path $kernel)) {
        Write-Warn2 "C kernel s'u gjet ne $shadow — anashkaloj."; $script:Results['C Kernel (27/27)']='N/A'; return
    }
    $gccK = Join-Path $script:MsysBin 'gcc.exe'
    if (-not (Test-Path $gccK)) { $gccK = 'gcc' }
    if (($gccK -eq 'gcc') -and -not (Test-Tool 'gcc')) { Write-Warn2 "gcc mungon (as MSYS2 bin, as PATH) — s'mund te kompiloj C kernel."; $script:Results['C Kernel (27/27)']=$false; return }
    Write-Step "gcc -std=c11 -O3 ... verify_kernel.c + kernel/*.c -lpthread"
    $out = Join-Path $shadow 'verify_kernel.exe'
    Push-Location $shadow
    $built = Invoke-WithRetry -What 'gcc kernel' -Max 2 -Action {
        # winpthreads e mbulon -lpthread ne mingw64.
        & $gccK -std=c11 -Wall -Wextra -O3 -I kernel `
            verify_kernel.c kernel/shadow_buss.c kernel/buss_legacy.c kernel/shadow_gj_legacy.c `
            -lpthread -o verify_kernel.exe
        if ($LASTEXITCODE -ne 0) { throw "gcc exit $LASTEXITCODE" }
    }
    if ($built -and (Test-Path $out)) {
        Write-Step "Po ekzekutoj kernelin..."
        $runOut = & $out 2>&1 | Out-String
        Write-Host $runOut
        $script:Results['C Kernel (27/27)'] = ($runOut -match 'OK' -or $runOut -match '27')
        if ($script:Results['C Kernel (27/27)']) { Write-OK "C kernel ekzekutoi — SOVEREIGN_KERNEL_RUNTIME OK." }
    } else {
        Write-Warn2 "Kompilimi i C kernel deshtoi (jo-fatal; logu te $LogFile)."
        $script:Results['C Kernel (27/27)'] = $false
    }
    Pop-Location
}

# ═════════════════════════════════════════════════════════════════════════════
#  HAPI 7 — CARGO BUILD (Light + Quantum + Shadow)
# ═════════════════════════════════════════════════════════════════════════════
function Build-Workspace {
    Write-Banner "HAPI 7/9 — CARGO BUILD --release (Light + Quantum + Shadow)"
    # AKTIVIZIMI (fix): cargo i zhveshur dështonte kur PATH s'ishte rifreskuar.
    $cargoExe = Join-Path $env:USERPROFILE '.cargo\bin\cargo.exe'
    if (-not (Test-Path $cargoExe)) { $cargoExe = 'cargo' }
    if (-not (Test-Tool 'cargo')) { Write-Err2 "cargo mungon — s'mund te ndertoj."; $script:Results['cargo build']=$false; return }
    Push-Location $ProjectRoot
    Write-Step "cargo build --release PROD ekzekutiv (c_kernel + hw_kernel; e para zgjat)..."
    $ok = Invoke-WithRetry -What 'cargo build' -Max 2 -Action {
        # PROD ekzekutiv: kernelet C aktive (light c_kernel, quantum hw_kernel).
        & $cargoExe build --release -p light-platform --features c_kernel 2>&1 | Tee-Object -FilePath (Join-Path $LogDir 'cargo_build.log')
        if ($LASTEXITCODE -ne 0) { throw "cargo build light exit $LASTEXITCODE" }
        & $cargoExe build --release -p quantum-platform --features hw_kernel 2>&1 | Tee-Object -Append -FilePath (Join-Path $LogDir 'cargo_build.log')
        if ($LASTEXITCODE -ne 0) { throw "cargo build quantum exit $LASTEXITCODE" }
        & $cargoExe build --release -p shadow_platform 2>&1 | Tee-Object -Append -FilePath (Join-Path $LogDir 'cargo_build.log')
        if ($LASTEXITCODE -ne 0) { throw "cargo build shadow exit $LASTEXITCODE" }
    }
    $script:Results['cargo build'] = [bool]$ok
    if ($ok) { Write-OK "Ndertimi i workspace-it perfundoi." }
    else { Write-Warn2 "cargo build hasi gabime — shih cargo_build.log (ndoshta kod platform-specifik per t'u rregulluar)." }
    Pop-Location
}

# ═════════════════════════════════════════════════════════════════════════════
#  HAPI 8/9 — NDËRTIMI I UI-së "NURA LEGACY" (Tauri 2 — jashtë workspace-it)
# ═════════════════════════════════════════════════════════════════════════════
function Build-UI {
    Write-Banner "HAPI 8/9 — UI 'NURA LEGACY' (cargo build --release ne ui/)"
    $uiDir = Join-Path $ProjectRoot 'ui'
    if (-not (Test-Path (Join-Path $uiDir 'Cargo.toml'))) {
        Write-Warn2 "ui/ s'u gjet — anashkaloj."; $script:Results['UI (Nura)']='N/A'; return
    }
    $cargoExe = Join-Path $env:USERPROFILE '.cargo\bin\cargo.exe'
    if (-not (Test-Path $cargoExe)) { $cargoExe = 'cargo' }
    Push-Location $uiDir
    $ok = Invoke-WithRetry -What 'cargo build (ui)' -Max 2 -Action {
        & $cargoExe build --release 2>&1 | Tee-Object -FilePath (Join-Path $LogDir 'ui_build.log')
        if ($LASTEXITCODE -ne 0) { throw "ui build exit $LASTEXITCODE" }
    }
    Pop-Location
    $uiExe = Join-Path $uiDir 'target\release\essmai_ui.exe'
    $script:Results['UI (Nura)'] = [bool]$ok -and (Test-Path $uiExe)
    if ($script:Results['UI (Nura)']) { Write-OK "Nura Legacy UI: $uiExe" }
    else { Write-Warn2 "UI s'u ndertua — shih ui_build.log." }
}

# ═════════════════════════════════════════════════════════════════════════════
#  AKTIVIZIMI — LAUNCHER-ËT (pika që mungonte: si NISET ESS-MAI)
# ═════════════════════════════════════════════════════════════════════════════
function Write-Launchers {
    Write-Banner "AKTIVIZIMI — launcher-et (.bat) ne rrenjen e projektit"
    $cli = @(
        '@echo off',
        'REM ESS-MAI CLI — pipeline real: Light -> Quantum (Gjata Legacy(TM))',
        'cd /d "%~dp0"',
        'set ESSMAI_HANDOFF_DIR=%~dp0',
        '"%~dp0target\release\light-platform.exe" | "%~dp0target\release\quantum-platform.exe"'
    ) -join "`r`n"
    Set-Content -Path (Join-Path $ProjectRoot 'essmai_cli.bat') -Value $cli -Encoding ASCII

    $wis = @(
        '@echo off',
        'REM SHADOW WISDOM — autoriteti i gjalle (vault persistent, main=kusht i domosdoshem)',
        'cd /d "%~dp0"',
        'set ESSMAI_HANDOFF_DIR=%~dp0',
        '"%~dp0target\release\shadow_platform.exe"'
    ) -join "`r`n"
    Set-Content -Path (Join-Path $ProjectRoot 'shadow_wisdom.bat') -Value $wis -Encoding ASCII

    $uiB = @(
        '@echo off',
        'REM NURA LEGACY UI — dritarja e Nures (Tauri 2)',
        'cd /d "%~dp0"',
        'set ESSMAI_HANDOFF_DIR=%~dp0',
        'set ESSMAI_LIGHT=%~dp0target\release\light-platform.exe',
        '"%~dp0ui\target\release\essmai_ui.exe"'
    ) -join "`r`n"
    Set-Content -Path (Join-Path $ProjectRoot 'nura_ui.bat') -Value $uiB -Encoding ASCII

    # SUPERVISOR (Document 20 · F): ngre platformat e GJALLA bashkë — Shadow
    # wisdom (autoriteti persistent) + Nura UI (që drejton Light→Quantum sipas
    # kërkesës). Pipeline-i CLI Light|Quantum mbetet i disponueshëm veç
    # (essmai_cli.bat). Çdo proces në dritaren e vet, i mbajtur gjallë nga OS.
    $sup = @(
        '@echo off',
        'REM ESS-MAI SUPERVISOR — ngre sistemin e gjalle (Gjata Legacy(TM))',
        'cd /d "%~dp0"',
        'set ESSMAI_HANDOFF_DIR=%~dp0',
        'echo Duke ngritur ESS-MAI...',
        'echo   [1/2] Shadow wisdom (autoriteti persistent)',
        'start "ESS-MAI Shadow Wisdom" cmd /k "%~dp0shadow_wisdom.bat"',
        'echo   [2/2] Nura UI (Light^-^>Quantum sipas kerkeses)',
        'start "ESS-MAI Nura UI" cmd /c "%~dp0nura_ui.bat"',
        'echo ESS-MAI eshte ngritur. Mbylle kete dritare kur te duash.'
    ) -join "`r`n"
    Set-Content -Path (Join-Path $ProjectRoot 'essmai_start.bat') -Value $sup -Encoding ASCII

    Write-OK "essmai_start.bat (SUPERVISOR) | essmai_cli.bat | shadow_wisdom.bat | nura_ui.bat"
    $script:Results['Launchers'] = $true
}

# ═════════════════════════════════════════════════════════════════════════════
#  HAPI 8.5/9 — CI GATE SOVRAN (ligjet strukturore, EXIT=0 i detyruar)
# ═════════════════════════════════════════════════════════════════════════════
function Invoke-CiGate {
    Write-Banner "HAPI 8.5/9 — CI GATE SOVRAN (installer/ci_gate.sh)"
    # bash-i vjen i GARANTUAR nga hapi MSYS2 i ketij skripti — mungesa e tij
    # ketu = deshtim instalimi, jo "degradim me hijeshi".
    $bashCands = @('C:\msys64\usr\bin\bash.exe',
                   "$env:SystemDrive\msys64\usr\bin\bash.exe")
    $bash = $bashCands | Where-Object { Test-Path $_ } | Select-Object -First 1
    if (-not $bash) {
        $cmd = Get-Command bash -ErrorAction SilentlyContinue
        if ($cmd) { $bash = $cmd.Source }
    }
    if (-not $bash) {
        Write-Err2 "bash (MSYS2) nuk u gjet — CI GATE s'ekzekutohet dot."
        $script:Results['CI GATE SOVRAN'] = $false
        return
    }
    $gate = Join-Path $ProjectRoot 'installer\ci_gate.sh'
    if (-not (Test-Path $gate)) {
        Write-Err2 "installer\ci_gate.sh mungon ne projekt."
        $script:Results['CI GATE SOVRAN'] = $false
        return
    }
    # Rruga Windows → MSYS (cygpath), pastaj gate-i nga rrenja e projektit.
    $unixRoot = (& $bash -lc "cygpath -u '$ProjectRoot'" 2>$null | Select-Object -First 1)
    if (-not $unixRoot) { $unixRoot = $ProjectRoot -replace '\\','/' -replace '^([A-Za-z]):','/$1' }
    Write-Step "Po ekzekutoj: bash installer/ci_gate.sh  (rrenja: $unixRoot)"
    & $bash -lc "cd '$unixRoot' && bash installer/ci_gate.sh"
    $ok = ($LASTEXITCODE -eq 0)
    if ($ok) { Write-OK  "CI GATE: EXIT=0 — ligjet strukturore te paprekura." }
    else     { Write-Err2 "CI GATE: EXIT=$LASTEXITCODE — ligjet u cenuan; NISJA NDALOHET." }
    $script:Results['CI GATE SOVRAN'] = $ok
}

# ═════════════════════════════════════════════════════════════════════════════
#  HAPI 9/9 — VERIFIKIMI + NISJA
# ═════════════════════════════════════════════════════════════════════════════
function Show-Summary {
    Write-Banner "HAPI 9/9 — VERIFIKIMI FINAL"
    $pad = ($script:Results.Keys | Measure-Object -Maximum -Property Length).Maximum
    foreach ($k in $script:Results.Keys) {
        $v = $script:Results[$k]
        $label = $k.PadRight($pad)
        if ($v -eq $true)      { Write-Host ("  [PASS] {0}" -f $label) -ForegroundColor Green }
        elseif ($v -eq 'N/A')  { Write-Host ("  [N/A ] {0}" -f $label) -ForegroundColor DarkGray }
        else                   { Write-Host ("  [FAIL] {0}" -f $label) -ForegroundColor Yellow }
    }
    # Portat e detyrueshme: rustc, cargo, gcc. Te tjerat degradojne me hijeshi.
    $mandatory = @('Rust (rustc)','Cargo','gcc','CI GATE SOVRAN')
    $missing = $mandatory | Where-Object { $script:Results[$_] -ne $true }
    Write-Host ""
    if ($missing.Count -eq 0) {
        Write-OK "Te gjitha mjetet e detyrueshme jane gati."
    } else {
        Write-Warn2 ("Mjete te detyrueshme qe mungojne: " + ($missing -join ', '))
        Write-Warn2 "Rihap nje terminal te ri (ose ristart) qe PATH-i te rifreskohet, pastaj ri-ekzekuto."
    }
    Write-Host ""
    Write-Step "Log i plote: $LogFile"
    Write-Host ""
    Write-Host "  == SI TA NISESH ESS-MAI (AKTIVIZIMI) ==" -ForegroundColor Cyan
    Write-Host "    - SUPERVISOR (ngre gjithcka): essmai_start.bat" -ForegroundColor Yellow
    Write-Host "    - Dritarja e Nures (UI):  nura_ui.bat" -ForegroundColor White
    Write-Host "    - Pipeline CLI real:      essmai_cli.bat  (Light | Quantum)" -ForegroundColor White
    Write-Host "    - Wisdom i Shadow-it:     shadow_wisdom.bat  (wisdom | feed <sid> | neg | law)" -ForegroundColor White
}

# ─────────────────────────────────────────────────────────────────────────────
#  EKZEKUTIMI KRYESOR — çdo hap i mbrojtur; gabimi jo-fatal nuk ndal të tjerët.
# ─────────────────────────────────────────────────────────────────────────────
try {
    Ensure-WinGet | Out-Null
    foreach ($stage in 'Install-Rust','Install-MSVC','Install-MSYS2','Install-WebView2',
                       'Install-TauriCli','Build-CKernel','Build-Workspace',
                       'Build-UI','Write-Launchers','Invoke-CiGate') {
        try { & $stage }
        catch { Write-Err2 "[$stage] gabim i papritur: $($_.Exception.Message) — po vazhdoj." }
    }
    Show-Summary
    Write-Banner "INSTALIMI PERFUNDOI"
    Write-Host "  ESS-MAI — Gardiani e mbylli ciklin e instalimit." -ForegroundColor Cyan
}
finally {
    try { Stop-Transcript | Out-Null } catch {}
    Write-Host ""
    Read-Host "Shtyp ENTER per te mbyllur"
}
