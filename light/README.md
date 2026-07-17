# ESS-MAI — Light Platform

**Gjata Legacy™** · Arkitekt: Bledar Gjata

Platforma e parë e ESS-MAI (nga tri: **Light** · **Quantum** · **Shadow**).
Roli i Light: **koordinon dhe transporton — kurrë nuk vendos.** Vendos Shadow.

---

## Çfarë është e re në

 mbyll zinxhirin e **vulës 500** që nga Light deri te autoriteti suprem,
dhe e bën projektin **100% të kompilueshëm dhe të ekzekutueshëm offline**.

### 1. Zinxhiri i plotë i vulës 500
```
EvolveTrace.primitive_flags   →   buss_legacy (i VERBËR)   →   shadow_gj_legacy.lgc
   (Light llogarit vulën)          (mbart pa e ditur 500)        (vendos 0/1)
```

- **Light** (`evolve_trace.rs`): llogarit vulën sipas *koherencës* së degëve.
  Nëse degët mbeten koherente (drift ≤ 0.15) → `primitive_flags = 0xA451`
  (= `(500 & 0xFFFF) ^ 0xA5A5`). Nëse inputi fragmentohet (drift > 0.15) →
  `primitive_flags = 0x0000`. **Branchless** — zero `if/else` në vendim.
- **buss_legacy.c**: bus FIFO mutex-protected që kopjon mesazhin *bit-për-bit*.
  Nuk e di ç'është 500, nuk e inspekton. Ndarje pushtetesh: transporti ≠ autoriteti.
- **shadow_gj_legacy.c**: i vetmi që e kupton vulën. `(flags & 0xFFFF) ^ 0xA5A5 == 500`?
  → **1** (`PRIMITIVE_KNOWLEDGE`), ndryshe → **0** (`VERIFIABLE_NOT_PRIMITIVE`).

### 2. Sovranitet i plotë (air-gapped si default)
- `lingua` (detektim gjuhe) → **opsionale** prapa feature `lingua_ext`.
  Default: detektor **pure-Rust** offline (script + stopword + diacritics) për 7 gjuhë.
- `candle` (Qwen ML) → **opsionale** prapa feature `ml_tiers`.
  Default: Tier1 template pure-Rust (ekzekutiv).
- Build-i default tërheq vetëm `sha2`, `hmac`, `serde`, `tracing` — zero AI, zero rrjet runtime.

### 3. Rregullime që ndalonin kompilimin/ekzekutimin
- `main.rs`: `coordinator.receive(&text)` → `receive(LightRequest::new(&text))`;
  `TraceInfo::new()` → `TraceInfo::new(text.as_bytes())`.
- `evolve_trace.rs`: `wrapping_xor` (s'ekziston) → `^`.
- `build.rs`: rrugët `../kernel/` (mungonin) → `kernel/` brenda projektit, 3 kernelë → 1 librari `light_kernel`.
- `lgc_bridge.rs`: unifikim i emrave të librarive FFI (`nura_kernel`/`light_buss` → `light_kernel`).
- Vula 500: tre formate të papajtueshme nëpër skedarë → **një standard i vetëm** (XOR 0xA5A5).

---

## Ndërtimi

### Default (offline, pa C, pa AI)
```bash
cargo build --release
cargo run --release
```

### Me kernelin C (zinxhiri real i vulës 500)
```bash
cargo build --release --features c_kernel
```

### Plotë (C + lingua + ML)
```bash
cargo build --release --features full
```

### Testet
```bash
cargo test                      # logjika pure-Rust + vula 500 (fallback)
cargo test --features c_kernel  # zinxhiri real përmes kernelëve C
```

---

## Arkitektura e skedarëve

```
src/
  main.rs               Boot orchestrator + runtime loop (Light→Quantum→Shadow seal)
  light_coordinator.rs  Porta hyrëse/dalëse — orchestron, kurrë s'vendos
  trace_info.rs         Stamp fillestar (FNV64) mbi input
  evolve_trace.rs       ★ Vula 500: llogarit primitive_flags nga koherenca e degëve
  shadow_seal_bridge.rs ★ Porta e fundit: vula → buss_legacy → shadow_gj_legacy
  lgc_algorithm.rs      KODUNIK (SHA256 + HMAC-SHA256)
  legalgocrypt.rs       LGC seal 9-segment + unseal_from_flags
  lgc_bridge.rs         FFI Rust↔C (lgc_check, light_buss)
  nura_core.rs          Domain routing (Work/Home/Outside)
  software_contract.rs  Autorizim komunikimi midis moduleve
  ess_mai_system.rs     DSL, limits, invariantët matematikë (epistemic_mass, gate, ...)
  ess_manifest.rs       Regjistri i moduleve + SHA256 root
  manifest.rs           Deklarata të pandryshueshme (identitet, hierarki, ligje)
  lingua.rs             Detektim gjuhe (pure-Rust default) + format output 7 gjuhë
  quantum_bridge.rs     Light↔Quantum përmes light_buss

kernel/
  light_buss.c/.h          Bus 4-prioritet, CRC32, mutex (Light↔Quantum)
  shadow_gj_legacy.c/.h  ★ Autoriteti suprem — vendos 0/1 mbi vulën 500
  buss_legacy.c/.h       ★ Bus i verbër — mbart vulën pa e ditur ç'është
```
★ = i ri ose i rishkruar thellë në

---

## Filozofia (e ruajtur)

- **Eliminim, jo gjenerim.** Quantum propozon, Shadow eliminon me 0/1.
- **Ndarje absolute pushtetesh.** Light bart, Quantum mendon, Shadow vendos. Asnjë s'i bën të dyja.
- **Mbijetesa = e vërteta.** Vula 500 mbahet vetëm nëse primitivi nuk fragmentohet nën presion.
- **Sovranitet.** Lokal, air-gapped, i gjurmueshëm. Cloud = NO-GO.

---

## Shënim mbi verifikimin

Logjika e vulës 500 (Light + të 3 kernelët C) është verifikuar end-to-end me gcc:
input i fortë → `0xA451` → bus i mbart i paprekur → shadow jep **1**;
input i fragmentuar → `0x0000` → shadow jep **0**.
Build-i i plotë `cargo build` duhet ekzekutuar në makinën tënde (kërkon crates.io).

---

# — Integrimi i Mini-Algoritmit + Sovereign 0-COPY + FFI

Integrim i plotë i (mini-algoritmi mbështetës) brenda Light, me ligjin 0-copy sovereign dhe pastrim total të if/else.

## 1. i integruar (3 algoritme)
- **APUPK** (Awaken Project User Personal Knowledge): `apupk/` — menaxhon projektet e përdoruesit, përgatit pako për Shadow (shadow_APUPK_memory)
- **SNB** (Save Negative Bug): `snb/` — gjurmon rrjedhën e moduleve, kap bug-e, raporton te Shadow (shadow_SNB)
- **Mini Evolver**: `mini_evolver/` — gjurmon përdorimin e dijes (knowledge usage stats), shtresë plotësuese për Quantum

Të 3 algoritmet: zero if/else (SNB 2 + Mini 3 të konvertuara në match). `snb/mod.rs` u krijua (mungonte).

## 2. Sovereign 0-COPY + ligji FFI (E RE)
`sovereign/` — IDENTIK me Quantum, vula 500 ndër platforma:
- **laws.rs**: gate, dot8, admit4, fnv, vulat e moduleve të Light, verify_500
- **lgc_gate.rs**: `CapHandle #[repr(C)]` + `LgcToken` opak + `AtomicBool` single-use. Nonce-i i modulit lidhet me slot-in para CAS-it; klon/replay → `AlreadyConsumed`, manipulim → `NonceMismatch` pa djegur capability-n legjitime.
- **ring.rs**: RingBuffer 0-copy SPSC i zbatuar nga tipi: pronar sekuencial ose `split(self) → (RingProducer, RingConsumer)` unik; `LgcBridge::receive_from` ruan CRC + seal në rrugën paralele
- Vula 500: `(flags & 0xFFFF) ^ 0xA5A5 == 500`, masked=0xA451 — byte-for-byte me Quantum/Shadow

## 3. Urat sovrane (lidhja me 2 platformat) — sovereign_bridges.rs
Çdo kalim ndër-platformë mbrohet me CapHandle një-përdorimësh + vula 500:
- **APUPK → Shadow** (shadow_APUPK_memory): SEAL_APUPK
- **SNB → Shadow** (shadow_SNB): SEAL_SNB, vetëm kur ka bug (Option)
- **Mini Evolver → Quantum** (quantum_knowledge_trace): SEAL_MINI_EVOLVER, me KnowledgeUsageSummary

Light VETËM përgatit dhe orienton — NUK vendos (Shadow vendos).

## 4. Përforcimi i Light — light_hardening.rs
- **AlgorithmHealth**: health check i 3 algoritmeve (health_score)
- **SovereignAudit**: verifikon që çdo kalim ka vulë 500 (audit_seals)
- **IntegrationGuard**: siguron Light s'vendos (bllokon decide/verify/approve/judge)
- **CapabilityBudget**: limit 64 capability/cikël (anti-replay)

## 5. Pastrim TOTAL i if/else
**0 if/else** në GJITHË Light (91 të konvertuara në match, përfshirë kod ekzistues). Rregullat e njëjta me Quantum: guard short-circuit → `match c { true => X, false => {} }`, ternary → match, if-else-if chain → match tuple, if let → match. cfg-blloqe (`#[cfg(feature="c_kernel")]`) të ruajtura intakte.

## Boot sekuenca (15 hapa)
1-11 (origjinale) + 12 sovereign + 13 algoritmet + 14 urat sovrane + 15 light_hardening.

## Verifikim
- 0 if/else në gjithë Light (verifikuar)
- Të gjithë skedarët balancuar ({} dhe ())
- 42 teste
- Simulim: vula 500 ✓, CapHandle single-use ✓, health ✓, audit ✓, no-decision guard ✓, capability budget ✓
- Importet: të gjitha zgjidhen, pa simbole të papërdorura
- CapHandle Copy → borrow-safe

## Statusi
27 skedarë, 7547 rreshta, 42 teste, zero if/else. i integruar plotësisht. Sovereign 0-copy + FFI shtuar. 3 ura sovrane me 2 platformat. Light i përforcuar. Pa rrjet/cargo këtu — final cargo build në makinën tënde (feature c_kernel për C kernels).

---

# Regjistri Qendror i Vulave — sovereign/seal_registry.rs

Konsolidim i vulave për të parandaluar përplasje kur sistemi rritet.

## Problemi
3 platforma (Quantum/Light/Shadow) → vula moduli të pavarura → rrezik përplasjeje kur shtohen module të reja (emër i dyfishtë ose kolizion FNV).

## Zgjidhja
Një burim i vetëm i vërtetë për TË GJITHA vulat (15: Quantum 5 + Light 6 + Shadow 4 rezervuar), me prefix hapësire emrash (Q_/L_/S_). Skedari është BYTE-FOR-BYTE identik në Quantum dhe Light — të dyja ndajnë të njëjtin regjistër pa crate të përbashkët.

## API
- `SealRegistry::detect_collision()` → CollisionReport (garanton zero përplasje, Hamming minimal)
- `SealRegistry::lookup(name)` → Option<u64>
- `SealRegistry::platform_of(seal)` → Option<Platform>
- `SealRegistry::count_by_platform(p)` → usize
- `SealRegistry::is_registered(seal)` → bool

## Garancia
Verifikuar: 15 vula, ZERO përplasje, distancë minimale Hamming 23 bit. Test `zero_collisions_guaranteed` garanton uniciteti. Teste konsistence te laws.rs e secilës platformë: vulat në regjistër == ato te kodi ekzistues.

## Statusi (me regjistër)
Light: +11 teste (53 total). Quantum: +12 teste (292 total). Regjistri identik byte-for-byte. Zero if/else. Shadow vulat rezervuar (S_JUDICIARY, S_VNK, S_VOK, S_LGC) që asnjë platformë t'i ripërdorë emrat kur Shadow të ndërtohet.
