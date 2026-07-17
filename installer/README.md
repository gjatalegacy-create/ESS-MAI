# Nura Legacy (ESS-MAI) — Instalimi

## Çfarë bën installer-i

Instalon ESS-MAI si aplikacion në device-in tënd. **Herën e parë**, tërheq nga web-i
toolchain-in Rust (x86_64) me pip, ndërton të 3 platformat, dhe **integron Rust-in
brenda Shadow-it përgjithmonë** — që të mos kesh nevojë të instalosh Rust veçmas.

## Si instalohet

### Linux / Mac
```sh
./install.sh
```

### Windows
```powershell
.\install.ps1
```

## Çfarë ndodh

1. Kontrollon nëse Rust (x86_64) është integruar tashmë te Shadow
2. **(Hera 1)** Tërheq Rust x86_64 nga web me pip (`setuptools-rust`, `maturin`)
3. Ndërton Light + Quantum + Shadow (cargo release)
4. Integron binarët brenda `shadow_integration/rust_runtime/` (vendi i përhershëm)
5. Krijon pikën e hyrjes `nura_legacy`

Pas instalimit, gjithçka është lokale. Herët e tjera **s'shkojnë në web**.

## Kërkesat
- Python 3.7+
- Lidhje interneti (vetëm herën e parë, për të tërhequr Rust)

---
GJATA LEGACY™ — ESS-MAI
