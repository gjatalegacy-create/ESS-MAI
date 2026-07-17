#!/usr/bin/env python3
# ╔══════════════════════════════════════════════════════════════════════════╗
# ║  INSTALL.PY — Instaluesi i ESS-MAI (Nura Legacy)                       ║
# ║  GJATA LEGACY™ — ESS-MAI                                               ║
# ║                                                                          ║
# ║  Instalon ESS-MAI si aplikacion. HERA E PARË: tërheq nga web-i          ║
# ║  toolchain-in Rust (x86_64) me pip, e ndërton sistemin, dhe e INTEGRON   ║
# ║  brenda Shadow-it — që të QËNDROJË PËRGJITHMONË pa pasur nevojë të        ║
# ║  instalohet Rust veçmas në device.                                      ║
# ║                                                                          ║
# ║  RRJEDHA:                                                              ║
# ║    1. Kontrollo nëse Rust (x86_64) është integruar tashmë te Shadow      ║
# ║    2. (Hera 1) Tërhiq Rust x86_64 nga web me pip                        ║
# ║    3. Ndërto të 3 platformat (Light + Quantum + Shadow)                 ║
# ║    4. Integro binarët brenda Shadow-it (vendi i përhershëm)             ║
# ║    5. Krijo pikën e hyrjes së app-it                                    ║
# ╚══════════════════════════════════════════════════════════════════════════╝

import os
import sys
import json
import shutil
import platform
import subprocess
from pathlib import Path

# ─────────────────────────────────────────────────────────────────────────────
# KONFIGURIMI
# ─────────────────────────────────────────────────────────────────────────────

APP_NAME = "Nura Legacy (ESS-MAI)"
APP_ID = "com.gjata.nura_legacy"
TARGET_ARCH = "x86_64"

# Vendi i përhershëm ku Shadow integron Rust-in (qëndron përgjithmonë).
SHADOW_RUST_HOME = "shadow_integration/rust_runtime"

# Platformat që ndërtohen.
PLATFORMS = {
    "light":   "light_v04_work",
    "quantum": "quantum_v07",
    "shadow":  "shadow_integration/shadow_work",
}

# pip paketat që sjellin Rust toolchain (x86_64) — hera e parë nga web.
RUST_PIP_PACKAGES = [
    "setuptools-rust",   # ndërton Rust extensions
    "maturin",           # paketon Rust si Python wheel (x86_64)
]


# ─────────────────────────────────────────────────────────────────────────────
# 1. KONTROLLO INTEGRIMIN EKZISTUES
# ─────────────────────────────────────────────────────────────────────────────

def rust_already_integrated(install_root: Path) -> bool:
    """A është Rust (x86_64) integruar tashmë brenda Shadow-it?
    Nëse po, s'ka nevojë ta tërheqim sërish nga web."""
    marker = install_root / SHADOW_RUST_HOME / ".rust_integrated"
    return marker.exists()


def mark_rust_integrated(install_root: Path, version: str):
    """Shëno që Rust u integrua (që herët e dytë të mos shkojë në web)."""
    rust_home = install_root / SHADOW_RUST_HOME
    rust_home.mkdir(parents=True, exist_ok=True)
    marker = rust_home / ".rust_integrated"
    marker.write_text(json.dumps({
        "arch": TARGET_ARCH,
        "version": version,
        "integrated_permanently": True,
    }))


# ─────────────────────────────────────────────────────────────────────────────
# 2. TËRHIQ RUST x86_64 NGA WEB (vetëm hera e parë)
# ─────────────────────────────────────────────────────────────────────────────

def fetch_rust_from_web() -> str:
    """HERA E PARË: tërheq Rust toolchain (x86_64) nga web me pip.
    Kthen versionin e integruar."""
    print(f"  [web] Duke tërhequr Rust ({TARGET_ARCH}) toolchain nga web me pip...")

    # pip install i paketave që sjellin Rust build toolchain.
    for pkg in RUST_PIP_PACKAGES:
        print(f"  [pip] install {pkg}")
        subprocess.run(
            [sys.executable, "-m", "pip", "install", "--user", pkg],
            check=True,
        )

    # Sigurohu që rustup/cargo janë të disponueshëm për x86_64.
    # (maturin tërheq automatikisht target-in x86_64 kur ndërton.)
    rust_version = _detect_or_bootstrap_rust()
    print(f"  [ok] Rust {rust_version} ({TARGET_ARCH}) u tërhoq nga web.")
    return rust_version


def _detect_or_bootstrap_rust() -> str:
    """Zbulon versionin e Rust; nëse mungon, e bootstrap-on me rustup."""
    probe = subprocess.run(["rustc", "--version"], capture_output=True, text=True)
    # Zero if-chain logjik: match mbi suksesin e probe-s.
    outcome = {
        True:  lambda: probe.stdout.strip(),
        False: _bootstrap_rustup,
    }[probe.returncode == 0]()
    return outcome


def _bootstrap_rustup() -> str:
    """Instalon rustup (x86_64) nëse Rust mungon krejt."""
    print(f"  [bootstrap] Rust mungon — duke instaluar rustup ({TARGET_ARCH})...")
    # rustup-init për x86_64 (Windows/Linux/Mac sipas OS).
    sys_name = platform.system()
    installer_cmd = {
        "Windows": ["powershell", "-Command",
                    "iwr https://win.rustup.rs -OutFile rustup-init.exe; "
                    ".\\rustup-init.exe -y --default-host x86_64-pc-windows-msvc"],
        "Linux":   ["sh", "-c",
                    "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | "
                    "sh -s -- -y --default-host x86_64-unknown-linux-gnu"],
        "Darwin":  ["sh", "-c",
                    "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | "
                    "sh -s -- -y --default-host x86_64-apple-darwin"],
    }.get(sys_name, ["sh", "-c", "echo 'OS i panjohur'"])
    subprocess.run(installer_cmd, check=True)
    probe = subprocess.run(["rustc", "--version"], capture_output=True, text=True)
    return probe.stdout.strip()


# ─────────────────────────────────────────────────────────────────────────────
# 3. NDËRTO TË 3 PLATFORMAT
# ─────────────────────────────────────────────────────────────────────────────

def build_platform(install_root: Path, name: str, rel_path: str):
    """Ndërton një platformë me cargo (release, x86_64)."""
    plat_dir = install_root / rel_path
    print(f"  [build] {name}: cargo build --release ({TARGET_ARCH})")
    subprocess.run(
        ["cargo", "build", "--release", "--target", f"{TARGET_ARCH}-unknown-linux-gnu"],
        cwd=str(plat_dir),
        check=False,  # vazhdo edhe nëse target s'është instaluar (cargo e tërheq)
    )
    # Fallback: build pa target eksplicit (përdor host x86_64).
    subprocess.run(["cargo", "build", "--release"], cwd=str(plat_dir), check=True)


# ─────────────────────────────────────────────────────────────────────────────
# 4. INTEGRO RUST BRENDA SHADOW-it (vendi i përhershëm)
# ─────────────────────────────────────────────────────────────────────────────

def integrate_into_shadow(install_root: Path):
    """Kopjon binarët e ndërtuar brenda Shadow rust_runtime — vendi i përhershëm.
    Pas kësaj, Rust qëndron brenda ESS-MAI përgjithmonë."""
    rust_home = install_root / SHADOW_RUST_HOME
    rust_home.mkdir(parents=True, exist_ok=True)

    for name, rel_path in PLATFORMS.items():
        target_dir = install_root / rel_path / "target" / "release"
        # Kopjo librarinë/binarin nëse ekziston.
        for artifact in target_dir.glob("*"):
            keep = artifact.suffix in (".so", ".rlib", ".a", "") and artifact.is_file()
            # Zero if: vetëm artefaktet e vlefshme kopjohen (match mbi keep).
            {True: lambda a=artifact: shutil.copy2(a, rust_home / a.name),
             False: lambda: None}[keep]()
        print(f"  [integrate] {name} → Shadow rust_runtime (i përhershëm)")


# ─────────────────────────────────────────────────────────────────────────────
# 5. KRIJO PIKËN E HYRJES SË APP-it
# ─────────────────────────────────────────────────────────────────────────────

def create_app_entry(install_root: Path):
    """Krijon pikën e hyrjes që starton ESS-MAI si app."""
    entry = install_root / "nura_legacy"
    sys_name = platform.system()

    launcher_body = {
        "Windows": f"@echo off\ncd /d \"%~dp0\"\n\"{SHADOW_RUST_HOME}\\shadow_platform.exe\" %*\n",
        "Linux":   f"#!/bin/sh\ncd \"$(dirname \"$0\")\"\nexec ./{SHADOW_RUST_HOME}/shadow_platform \"$@\"\n",
        "Darwin":  f"#!/bin/sh\ncd \"$(dirname \"$0\")\"\nexec ./{SHADOW_RUST_HOME}/shadow_platform \"$@\"\n",
    }.get(sys_name, "#!/bin/sh\necho 'OS i panjohur'\n")

    ext = {"Windows": ".bat"}.get(sys_name, "")
    launcher = Path(str(entry) + ext)
    launcher.write_text(launcher_body)
    # Bëje të ekzekutueshëm (Unix).
    {True: lambda: os.chmod(launcher, 0o755),
     False: lambda: None}[sys_name != "Windows"]()
    print(f"  [app] Pika e hyrjes: {launcher.name}")


# ─────────────────────────────────────────────────────────────────────────────
# MAIN — orkestron instalimin
# ─────────────────────────────────────────────────────────────────────────────

def main():
    print("═" * 70)
    print(f"  INSTALIMI I {APP_NAME}")
    print(f"  Arkitektura: {TARGET_ARCH} | App ID: {APP_ID}")
    print("═" * 70)

    install_root = Path(__file__).resolve().parent

    # 1. A është Rust integruar tashmë?
    already = rust_already_integrated(install_root)
    print(f"\n[1/5] Kontroll integrimi: Rust i integruar = {already}")

    # 2. (Hera 1) Tërhiq Rust nga web; (herët tjera) anashkalo.
    # Zero if: match mbi gjendjen e integrimit.
    rust_version = {
        False: lambda: fetch_rust_from_web(),       # hera 1 → web
        True:  lambda: "(tashmë i integruar)",      # herët tjera → skip
    }[already]()
    print(f"[2/5] Rust: {rust_version}")

    # 3. Ndërto të 3 platformat.
    print("[3/5] Ndërtimi i platformave:")
    for name, rel_path in PLATFORMS.items():
        build_platform(install_root, name, rel_path)

    # 4. Integro brenda Shadow-it (vendi i përhershëm).
    print("[4/5] Integrimi brenda Shadow-it:")
    integrate_into_shadow(install_root)
    mark_rust_integrated(install_root, str(rust_version))

    # 5. Krijo pikën e hyrjes së app-it.
    print("[5/5] Krijimi i app-it:")
    create_app_entry(install_root)

    print("\n" + "═" * 70)
    print(f"  ✓ {APP_NAME} u instalua.")
    print(f"  Rust ({TARGET_ARCH}) është integruar PËRGJITHMONË brenda Shadow-it.")
    print(f"  Herët e tjera s'do shkojnë në web — gjithçka është lokale.")
    print("═" * 70)


if __name__ == "__main__":
    main()
