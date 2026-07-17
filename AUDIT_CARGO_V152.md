# ESS-MAI v1.5.2 — Harta e Hetimit dhe Planit të Ndërhyrjes

## Harta sovrane

```text
Input
  ↓
Light / Coordination
  i₀ → PA → Xi/Yi → transport i vulosur
  ↓
Quantum / Reasoning
  LIM → PRO/NPRO → MPRO/APRO → PIM/NPIM
  ↓
Shadow / Verification
  anchor gate → Y verdict → X trust → vault/negative knowledge
  ↓
Light / Output + trace
```

Ndërhyrja nuk ndryshon asnjë nyje të kësaj harte. Ajo prek vetëm portat mbrojtëse dhe testet që provojnë gatishmërinë e nyjeve.

## Matrica e provës Cargo

`cargo build --workspace --all-targets` mbi v1.5.1 raportoi gjashtë error-e:

- 1 × `E0432`: `SEAL_EBPF` kërkohej gabimisht nga namespace-i lokal i Light;
- 2 × `E0425`: `Coordination` ishte variant i pakualifikuar në testin GCL;
- 3 × `E0277`: `unwrap_err()` kërkonte `Debug` mbi `LgcToken` opak.

Build-i ndaloi te target-i `light-platform` test. Kopjet simetrike në Quantum/Shadow u kontrolluan statikisht përpara ndërhyrjes, që error-i të mos zhvendosej thjesht te target-i pasues.

## Plan i zbatuar i ndërhyrjes

1. Izolim i error-it nga warning-et dhe përcaktim i target-it që dështoi.
2. Gjurmim i pronësisë së seal-it në registry, jo shtim mekanik konstanteje.
3. Krahasim i kopjeve Light/Quantum/Shadow dhe korrigjim simetrik vetëm ku kontrata ishte identike.
4. Ruajtje e opacitetit të capability token-it; korrigjim i testit dhe forcim i binding-ut të handle-it.
5. Audit i `unsafe` të ring-ut dhe përputhje me disiplinën SPSC.
6. Kontroll byte-për-byte i motorëve epistemikë, vault-it, ligjeve dhe registrave të vulave.
7. Paketim vetëm pasi auditimi statik dha zero dështime.

## Rrjedha 1 — `SEAL_EBPF`

```text
all-targets
  → light::sovereign::ring::tests
  → import nga light::sovereign::laws
  → emri nuk ekziston
```

Hetimi tregoi se `EBPF_HYDRATOR` është regjistruar si modul Quantum. Shtimi i `SEAL_EBPF` te ligjet Light do të përziente pronësinë e platformave. Zgjidhja është lookup nga registry ndër-platformë dhe provë e `Platform::Quantum`.

## Rrjedha 2 — `Coordination`

```text
all-targets
  → gcl_presume test-only
  → variant i pakualifikuar jashtë scope-it lokal
```

Skedari ekziston byte-identik në të tria platformat. Korrigjimi bëhet ×3 dhe ruan identitetin. Gjatë hetimit u gjet se pohimi mbi `stats.total` ishte i brishtë ndaj testimit paralel; ai u zëvendësua me numërimin e hyrjes me emër unik.

## Rrjedha 3 — `LgcToken: Debug`

```text
unwrap_err()
  → kërkon Debug mbi tipin Ok
  → tipi Ok është capability token opak
```

Derivimi `Debug` mbi token do të ishte korrigjim i gabuar filozofik. Testi u korrigjua pa ekspozuar token-in. Ndjekja e rrjedhës zbuloi se `nonce` dilte në `CapHandle`, por nuk verifikohej nga Light/Quantum; kjo binte ndesh me vetë komentet e capability contract dhe me implementimin më të fortë të Shadow. Slot-i tani mban dhe verifikon nonce-in para konsumit.

## Rrjedha 4 — Ring concurrency

`unsafe impl Sync for RingBuffer` e bënte API-në safe të thirrshme nga shumë producer/consumer, ndërsa algoritmi `load/store` ishte SPSC. Kjo ishte mospërputhje mes tipit dhe P8. Zgjidhja nuk e kthen në MPSC dhe nuk shton mutex: ring-u ndahet me konsum të pronarit në endpoint-e unike SPSC. `LgcBridge::receive_from` e lidh consumer-in unik me të njëjtën portë CRC + seal, prandaj rruga paralele nuk anashkalon autorizimin.

## Stop conditions të kontrolluara

- Nuk u shtua vulë e platformës tjetër në ligjet Light.
- `LgcToken` nuk u bë `Debug`, `Clone`, `Copy` ose `Send`.
- Shadow nuk humbi autoritet.
- Asnjë skedar NPRO/NPIM nuk u ndryshua.
- Asnjë wire payload ose vlerë seal ekzistuese nuk u ndryshua.
- Profilet Cargo nuk u centralizuan, sepse kjo do të kërkonte vendim për `panic=abort` global kundrejt sjelljes së Quantum.

## Validimi i kërkuar në makinën autoritative

1. `cargo build --workspace --all-targets`
2. `cargo test --workspace`
3. testet me feature-matrix nga `VALIDATE_V152.ps1`
4. Clippy pa kombinimin e ndaluar `runtime_mode + pure_rust`

Çdo error i ri trajtohet si provë e re; nuk përdoret `cargo fix`.
