# ESS-MAI v1.5.2 — Cargo-Evidence Closure and Sovereign Contract Hardening

## Baseline

Kjo release është projekti i plotë `v1.5.1` plus vetëm ndërhyrje të lidhura me prova konkrete nga:

- `cargo build --workspace --all-targets` i ekzekutuar në Windows;
- gjashtë error-et e target-it `light-platform` test;
- ndjekja statike e kopjeve byte-identike dhe kontratave të prekura.

## Korrigjimet

### 1. Pronësia e vulës eBPF në Light

`light::sovereign::ring` nuk importon më `SEAL_EBPF` nga ligjet lokale të Light. Testi merr vulën `EBPF_HYDRATOR` nga `SealRegistry`, verifikon se pronari është `Platform::Quantum`, dhe pastaj provon handshake-un. Nuk u shtua asnjë vulë Quantum në namespace-in e ligjeve Light.

### 2. GCL Presume ×3

Kopjet Light, Quantum dhe Shadow mbeten byte-identike.

- `CollapsePhase::Coordination` përdoret plotësisht i kualifikuar.
- Testi i idempotencës nuk krahason më totalin global, i cili mund të ndryshojë nga testet paralele; numëron vetëm emrin unik të hyrjes që po provon.

### 3. Capability gate Light + Quantum

- `LgcToken` mbetet opak dhe nuk merr `Debug` vetëm për të kënaqur `unwrap_err()`.
- Testet përdorin `Result::err()`, pa ekspozuar token-in.
- `CapHandle` është `#[repr(C)]`, me layout dy `u64`.
- `CapSlot` ruan `expected_nonce`.
- `validate()` kontrollon nonce-in para CAS-it; manipulimi jep `NonceMismatch` dhe nuk djeg capability-n legjitime.
- Konvertimi `u64 → usize` është fail-closed me `try_from`.
- Replay/clone vazhdon të refuzohet me `AlreadyConsumed`.

### 4. RingBuffer SPSC i zbatuar nga tipi

Deklarata P8 `SPSC ring` tani mbrohet nga forma e API-së:

- `RingBuffer` është pronar sekuencial dhe jo `Sync`;
- `split(self)` prodhon saktësisht një `RingProducer` dhe një `RingConsumer`;
- endpoint-et nuk janë `Clone/Sync` dhe operacionet kërkojnë `&mut self`;
- core-i ndahet vetëm mes këtyre dy endpoint-eve;
- `LgcBridge::receive_from` lidh consumer-in e tipizuar me të njëjtën portë CRC + seal, pa rrugë anashkaluese;
- matematikat `head & MASK`, CRC, `Ev.mass` dhe rrjedha `Ring → LgcBridge` mbeten të pandryshuara;
- zero kopje payload-i dhe zero mutex.

### 5. Validation matrix

`VALIDATE_V152.ps1` nuk përdor më `--all-features`, sepse Shadow ndalon me ligj kombinimin `runtime_mode + pure_rust`. Testet dhe Clippy ndahen në feature-matrix të vlefshme.

## Të paprekura

- Light → Quantum → Shadow dhe autoriteti final i Shadow.
- NPRO/NPIM, negativity_score dhe pragjet e tyre.
- Wire payload-et dhe vulat numerike ekzistuese.
- Shadow persistence dhe vault semantics të v1.5.1.
- Cargo profile policy: mbetet për vendim të veçantë, sepse strategjia `panic` nuk mund të ruhet per-package nga workspace profile overrides.

## Statusi i validimit

Mjedisi i paketimit nuk ka `rustc/cargo`; prandaj nuk deklarohet cargo-green pa ekzekutuar `VALIDATE_V152.ps1` në makinën autoritative. Janë kryer kontrolli statik i scope-it, simetrisë, delimiter-ëve, hash-eve dhe eliminimi i gjashtë modeleve të error-it të raportuar.
