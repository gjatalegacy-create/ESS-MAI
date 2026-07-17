# ESS-MAI — Instalimi Sovran (`setup_essmai`)

**Gardiani i ESS-MAI / Gjata Legacy™**

Dy skedarë, një klikim. Vendosi **të dy** në rrënjën e projektit `ess_mai/` (pranë `Cargo.toml`, `light/`, `quantum/`, `shadow/`):

```
ess_mai/
├── setup_essmai.bat      ← KLIKO KETE (dy-klik)
├── setup_essmai.ps1      ← motori vetë-shërues (thirret nga .bat)
├── Cargo.toml
├── light/  quantum/  shadow/
└── ...
```

## Si përdoret

1. Kliko **dy herë** `setup_essmai.bat`.
2. Prano kërkesën e administratorit (**UAC**) — është normale (PATH-i i makinës, MSVC dhe WebView2 kërkojnë leje).
3. Prit. Instalimi bën gjithçka vetë.

## Çfarë instalon automatikisht

| # | Komponenti | Pse |
|---|---|---|
| 1 | **Rust** (rustup, stable, host **GNU/mingw**) → `rustc` + `cargo` | një ABI e vetme me gcc + winpthreads (kerneli C) |
| 2 | **Visual Studio C++ Build Tools** + Windows SDK → `cl.exe` | vetëm për Tauri UI në të ardhmen (bërthama s'e përdor) |
| 3 | **MSYS2** + `mingw-w64 gcc/g++` + **winpthreads** | kompilimi i C kernel-it (që përdor `pthread`) |
| 4 | **WebView2 Runtime** (nëse mungon) | UI-ja e Tauri 2 |
| 5 | **Tauri CLI** (`cargo-tauri`) | UI minimalist |

Pastaj: **kompilon C kernel-in** (provë reale ekzekutimi), bën **`cargo build --release` PROD ekzekutiv** (light `c_kernel`, quantum `hw_kernel`, shadow), dhe shfaq një matricë **PASS/FAIL** + log të plotë.

## Filozofia: i pathyeshëm (vetë-shërues)

- **Idempotent** — nëse një mjet ekziston, e kapërcen; nëse mungon, e instalon.
- **Injoron gabimet jo-fatale** — vazhdon dhe i rregjistron në log.
- **Retry** — riprovon shkarkimet/instalimet që dështojnë përkohësisht.
- **Fallback** — `winget` si motor parësor; nëse mungon ose bllokohet, kalon në **shkarkim direkt** nga burimet zyrtare.
- **PATH automatik** — e vendos PATH-in (user) me API-në `.NET` (jo `setx`), transmeton `WM_SETTINGCHANGE`, dhe e rifreskon procesin aktual që mjeti i ri të jetë i thirrshëm menjëherë.
- **Porta të detyrueshme**: `rustc`, `cargo`, `gcc`. Të tjerat degradojnë me hijeshi.

## Shënime të rëndësishme

- **C kernel + `pthread`:** në Windows `pthread` mbulohet nga **winpthreads** (vjen me toolchain-in mingw-w64 të MSYS2). Prandaj instalohet MSYS2.
- **Leximet HW/thermal në Windows:** firmware-i i shumë desktopëve **nuk** e mbështet zonën termike ACPI, ndaj leximi i drejtpërdrejtë i temperaturës dështon në nivel firmware-i (jo gabim kodi). Kjo trajtohet me **fallback** në kod (kthen vlerë nominale në vend që të dështojë) — pjesë e fazës së kodit cross-platform (shih më poshtë).
- **Nëse `cargo build` ndal me gabime platform-specifike:** disa pjesë të kodit janë shkruar për Linux. Faza tjetër (kod) i bën ato **cross-platform** me `cfg(target_os)` / `#ifdef _WIN32`. Logu te `%TEMP%\essmai_setup_logs\cargo_build.log` tregon saktësisht çfarë mbetet.
- **Restart:** pas instalimit të MSVC ose ndryshimit të PATH-it, ndonjëherë duhet **rihapje terminali** (ose ristart) që gjithçka të rifreskohet; ri-ekzekuto `setup_essmai.bat` dhe do t'i kapërcejë hapat e bërë.

## Pas instalimit

Binarët dalin te `target/release/`. UI-ja e Tauri (kur të ndërtohet) del si instalues `…-setup.exe` (NSIS) te `target/release/bundle/nsis/`.


---

## AKTIVIZIMI — SI NISET ESS-MAI (pas instalimit)

Installer-i krijon automatikisht **3 launcher-ë** në rrënjën e projektit:

| Launcher | Çfarë nis | Shënim |
|---|---|---|
| `essmai_start.bat` | **SUPERVISOR** — ngre Shadow wisdom + Nura UI bashkë | Nisja me një klik e sistemit të gjallë |
| `nura_ui.bat` | **Dritarja e Nurës** (Tauri 2) | UI thërret binarin REAL `light-platform` — zero simulim |
| `essmai_cli.bat` | Pipeline CLI: `light-platform \| quantum-platform` | Shkruaj një rresht → kolapsi 3-fazor ekzekutohet |
| `shadow_wisdom.bat` | **Shadow Wisdom** — autoriteti i gjallë | main.rs = kushti i domosdoshëm; lib.rs kompilohet vetëm brenda tij si autoritet. Vault persistent (`shadow_wisdom.svlt`). Komandat: `wisdom`, `feed <session_id>`, `neg`, `law`, `exit` |

Boot-i i secilit binar **LEXON dhe SHPALL** `gjata_collapse_law` (manifesti + vula FNV) — ligji është i gjallë në çdo nisje.

### Rregullimet e aktivizimit
- `rustup`/`cargo` thirren me **path absolut** (`%USERPROFILE%\.cargo\bin\...`) — GNU default vendoset i garantuar edhe pa rihapje shell-i.
- `pacman -Syuu` **dy-fazor** (core → rest) — idioma zyrtare MSYS2.
- `gcc` verifikohet me **path të plotë** (`C:\msys64\mingw64\bin\gcc.exe`), jo vetëm në PATH.
- **HAPI 8/9 i ri**: ndërtimi i UI-së `ui/` (jashtë workspace-it — s'e rëndon build-in bazë).
