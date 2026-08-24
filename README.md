# ESS-MAI by Bledar Gjata · Gjata Legacy

## Gjata Collapse Law (GCL) · Sovereign Deterministic Reasoning Research Architecture

**ESS-MAI** is an experimental deep-tech research and systems-engineering project by **Bledar Gjata**. It investigates a Rust-based architecture for bounded hierarchical authority, traceable sovereign AI, deterministic reasoning, and the preservation and operational use of negative knowledge.

- **Author / Architect:** Bledar Gjata
- **Project:** ESS-MAI
- **Organization:** Gjata Legacy
- **Canonical repository:** https://github.com/gjatalegacy-create/ESS-MAI
- **Author profile:** https://github.com/gjatalegacy-create
- **Core research concept:** Gjata Collapse Law (GCL)
- **Implementation environment:** Rust and Cargo
- **Status:** active research prototype with partial materialization

This repository is evaluated through source code, reproducible `cargo build` and test results, documented successes and failures, and executable proof-of-concept packages. A successful build proves compilation only for the tested scope; it does not by itself prove every architectural or scientific claim.

Executable POCs, prior-art records, build evidence, and materialization reports are maintained as additions to this canonical repository. They are **not** published as separate ESS-MAI repositories.

## Executable prior art / POC collection

The public Apache-2.0 collection is maintained at [`publications/executable-prior-art/`](publications/executable-prior-art/). It separates established foundations, pre-existing ESS-MAI materialization, the bounded POC contribution, experimental failures, and the architecture-preserving advancement method. The full private v1.8.9 core is not part of that disclosure.

## Research scope

- Gjata Collapse Law (GCL) and bounded hierarchical authority
- negative knowledge as operational system state
- Light–Quantum–Shadow separation of roles
- traceability and deterministic decision paths
- Rust-based system contracts and fail-closed behavior
- executable POCs and reproducible experimental evidence
- formal-methods-inspired invariants and runtime verification

## Technical documentation / Dokumentacioni teknik

Dokumentacioni teknik më poshtë ruan përshkrimin ekzistues të arkitekturës dhe materializimit të projektit.

## Arkitektura: Triniteti Light–Quantum–Shadow

```
INPUT → LIGHT (koordinim+trace, s'vendos)
      → QUANTUM (arsyetim+presion+eliminim, s'jep verdikt)
      → SHADOW (verdikt + persistencë brenda juridiksionit të deleguar nga GCL)
      → LIGHT → OUTPUT
```

## — Fortësim Sigurie (13 rregullime për sistem real)

| # | Dobësia | Rregullimi |
|---|---------|-----------|
| 1 | Wire format i brishtë (delimiterë në free-text) | **escape/unescape** për content/text (round-trip i provuar) |
| 2 | deserialize validon vetëm trace_id | **fail-closed**: të 7 fushat kritike të detyrueshme |
| 3 | from_payload pranon payload të paplotë | **fail-closed**: trace_id+verdict+seal+state+content |
| 4 | carries_seal me contains (substring) | **seal strukturor**: formula kanonike (flags & 0xFFFF)^0xA5A5==500 |
| 5 | seal = QNT:trace_id (jo provë) | **hash mbi content**: QNT:trace:sha256(envelope) |
| 6 | Contract pa integritet (fusha modifikohen) | **fingerprint** kanonik; enforce() rikomputon e krahason |
| 8 | Fallback no-op duket sukses | **fail-closed**: `--no-default-features` → BusUnavailable |
| 9 | SHA256 C placeholder (zero) | **Rust real i vetmi autoritet**; C hequr nga sha256_of |
| 10 | "zero-copy" pretendohet ku klonon | **dokumentim i ndershëm**: ligji vlen për rrugën kryesore (move) |
| 11 | sovereign_commit payload bosh → sukses | **error -5 EmptyPayload** (jo 0) |
| 12 | ChainRejected humb shkakun | **reason_code + failure_stage** (bit-i mbetet i thjeshtë) |
| 13 | Verdiktet kompresohen herët | diagnoza e plotë ruhet; 0/1 vetëm në dalje |
| 14 | FFI kthen sukses kur vault-i mungon | **error -6 VaultUnavailable**; zero nuk maskon shkrim të munguar |
| 15 | Backend refuzon shkrimin por FFI kthen 0 | **error -7 VaultWriteFailed**; lineage regjistrohet vetëm pas persistimit |

## Ndarja Prod vs Dev (#8)

```bash
# Testim/ndërtim i plotë — v1.6.0 kërkon binarin Shadow main.rs:
cargo build --workspace
cargo test --workspace --all-targets

# Runtime (PowerShell shembull):
# $env:ESSMAI_HANDOFF_DIR='C:\essmai\handoff'
# cargo run -p quantum-platform
# Quantum gjen shadow_platform.exe si sibling ose nga ESSMAI_SHADOW_BIN.

# PRODHIM (fail-closed — kërkon kernel/bus real):
cargo build --release -p light-platform --no-default-features --features c_kernel
```


## Complete mediation v1.5.9

Shadow është target **vetëm binar** (`autolib=false`). `lib.rs` përfshihet brenda
`shadow/src/main.rs`; Quantum varet vetëm nga `shadow_contracts` dhe komunikon
me procesin `shadow_platform`. Pa Shadow main aktiv nuk ka ingest, vault, token
ose `VerificationReceipt`; rrjedha ndalon fail-closed.


## Architectural closure v1.6.0

v1.6.0 ruan complete mediation të v1.5.9 dhe mbyll katër vazhdimësi të
provuara në kod:

1. GCL ekziston para aktivizimit të Spine 9 dhe i njëjti `process_digest`
   udhëton në Layer 1 → Layer 2 → Layer 3; Layers vetëm e thellojnë procesin.
   Çdo mode i PD aktivizon të gjithë maskën `111`.
2. PIM + NPIM + MPRO prodhojnë një paketë finale me provat PIM, argumentet
   NPIM, blob lineage dhe 16 matjet MPRO. Paketa bart edhe materialin e plotë
   të continuum/activation/Spine; Shadow main rillogarit `i+U→i₀→1Q`, GCL,
   aktivizimin, materialin kanonik të çdo Layer-i, të tre receipts dhe
   completion-in para core-it.
3. SHA-256 i inputit prodhohet në Light, verifikohet në Quantum dhe
   rillogaritet përsëri në Shadow.
4. PD Light është korrier iZ: Nura dhe UI-ja e vjetër emocionale marrin të
   njëjtin sinjal të verifikuar në paralel. UI-ja e vjetër e transmeton si
   `[PD_LIGHT/IZ]` në stdout-in e Light; Tauri e kthen në komandë për UI-në e re.

`light_spine` mbetet spine emocional/interpretues i Light-it dhe nuk është
PD Spine 9. Legacy Shadow C mbetet vëzhguesi i vazhdueshëm i afrimit drejt
Legacy; nuk zëvendësohet nga receipt-i final i Shadow main.

Verifikimi i plotë në Windows GNU:

```powershell
.\VALIDATE_V160.ps1
```

## Invariantet kushtetuese të synuara dhe të testueshme

1. **Zero if/else klasik** — match/boolean/formula (match-guards të lejuar).
2. **Quantum s'vendos kurrë** — prodhon provë për Shadow.
3. **GCL është autoriteti sovran kushtetues** — Shadow ushtron vetëm juridiksionin e deleguar të verdiktit/persistencës dhe nuk merr sovranitetin e parent-it.
4. **Asnjë klon i panevojshëm** — move në rrugën kryesore.
5. **Trace para çdo logjike**.
6. **Reasoning Purity** — vetëm Shadow shkruan persistent.

## Platformat

| Platformë | Rol | Cargo |
|-----------|-----|-------|
| `light/`   | Koordinim, trace, transport, UI, kontrata | `light-platform` |
| `quantum/` | Arsyetim, NPRO, HCP_PRO, MPRO, Digital Lab | `quantum-platform` |
| `shadow/`  | Verdikt dhe persistencë brenda autoritetit të deleguar nga GCL; ECO, NightWatch, VNK vault | `shadow_platform` |

---
**GJATA LEGACY™ — ESS-MAI** • Sovereign Deterministic Reasoning Substrate

