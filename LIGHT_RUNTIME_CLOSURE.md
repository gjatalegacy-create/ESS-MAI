# LIGHT PLATFORM — MBYLLJE NË RUNTIME ()
**Gjata Legacy™ / Nura Legacy** · Bazë: · Platforma: **LIGHT (e plotë)**

Plani: mbyll platformat një nga një në runtime. Kjo është **platforma e parë: LIGHT**,
e trajtuar në tërësi (52 skedarë, 11459 rreshta). Pas Light: Quantum, pastaj Shadow.

---

## 1. DETEKTIMI (sistematik, pa hamendësim)

Skanova të 52 skedarët e Light. Boshllëku kryesor i runtime-it:

### 1.1 ⚠️ 42 .unwrap() në rrugë jo-test = PANIC potencial në runtime
Çdo `.unwrap()` jashtë testeve është një pikë ku procesi mund të rrëzohet në runtime.
Shpërndarja: nura_core(13), main(13), ess_manifest(9), apupk(2), snb(1), mini_evolver(1),
lingua(1), legalgocrypt(1), + 1 .expect() te lgc_algorithm.

**TË GJITHA u konvertuan në fail-safe** sipas natyrës:

| Lloji | Para | Tani (fail-safe) | Pse |
|-------|------|------------------|-----|
| Lock/RwLock (24) | `.lock().unwrap()` | `.lock().unwrap_or_else(\|p\| p.into_inner())` | Rikuperon lock poisoned — s'rrëzon procesin |
| SystemTime (4) | `.duration_since(EPOCH).unwrap()` | `.unwrap_or_default()` | Ora para 1970 → ZERO, jo panic |
| I/O stdout (13) | `writeln!(...).unwrap()` | `.ok()` | Broken pipe → injoro, s'ka ku shkruhet |
| chars.last (1) | `.last().unwrap()` | `match { Some=>.., None=>return }` | Fail-safe absolut |

### 1.2 Lock poisoning — sqarim teknik
Para: nëse një thread paniko duke mbajtur një Mutex/RwLock, lock-u "poisoned".
Çdo `.lock().unwrap()` tjetër → panic → **rrëzon GJITHË procesin sovran**.
Tani: `into_inner()` rikuperon guard-in nga PoisonError → sistemi VAZHDON. Fail-safe real.

### 1.3 .expect() te generate_hmac — MBAJTUR qëllimisht
`HmacSha256::new_from_slice(LGC_SECRET).expect()` — LGC_SECRET është konstante valide,
HMAC pranon çdo gjatësi çelësi → s'dështon KURRË. Alternativa (HMAC bosh) do shkatërronte
vulosjen kriptografike. Ky panic është MBROJTJE sigurie, jo dobësi. Dokumentuar qartë.

---

## 2. LIGJI "INPUT S'HYN PA TRACE" — forcuar te UI

`ui_channel::receive_input` ngul trace menjëherë (mirë), por s'validonte trace_id/text.
Shtova te `TracedInput`:
- `is_traced()`: trace_id jo-zero DHE tekst jo-bosh. Zero if — match.
- `reason_untraced()`: reason code (ui_zero_trace_id / ui_empty_text / traced).

Kjo zbaton ligjin: hyrje pa gjurmë të vlefshme = e identifikueshme si e pavlefshme.

---

## 3. RRJEDHA KRYESORE — verifikuar (tashmë fail-closed)

`light_coordinator::receive()` u inspektua plotësisht:
- HAPI 1: TraceInfo::new mbi raw bytes — **trace menjëherë** ✓
- HAPI 4: SoftwareContract::create + enforce — kontratë dështon → kthim early me ContractFailed ✓
- HAPI 5-6: EvolveTrace.branch — dështim → Err path ✓
Asnjë silent success. Rrjedha respekton rolet: Light koordinon, s'vendos.

---

## 4. TESTE TË REJA
1. `traced_input_fail_closed_requires_trace_and_text` (ui_channel) — input pa trace refuzohet.
Plus testet ekzistuese të Light: **137 teste**.

---

## 5. KERNELI C I LIGHT — kompilim REAL me gcc
Të 3 kernelët (shadow_gj_legacy.c, buss_legacy.c, light_buss.c) kompilojnë pastër
me `gcc -std=c11 -Wall -Wextra`. Placeholder `lgc_sha256` MUNGON në prodhim (gate
`#ifdef SOVEREIGN_ALLOW_PLACEHOLDER_SHA` aktiv) — verifikuar me `nm`.

---

## 6. STATUSI PËRFUNDIMTAR — LIGHT
- **0 if/else klasik** (52 skedarë)
- **0 skedarë të pabalancuar**
- **0 .unwrap() real** (42 → fail-safe)
- **137 teste**
- **Kerneli C kompilon pastër** (gate aktiv)

---

## 7. ⚠️ ÇFARË NUK DUHET NDRYSHUAR (referencë)
- **Lock recovery pattern**: `unwrap_or_else(\|p\| p.into_inner())` — mos e kthe në unwrap.
- **.expect() te generate_hmac**: MBAJ — është mbrojtje sigurie (dokumentuar §1.3).
- **Vula 500** te shadow_seal_bridge/light_hardening: formula intakte, mos prek.
- **lab_contracts** (8 skedarë): byte-identik me Quantum+Shadow — mos ndrysho vetëm te Light.
- **Cargo.toml runtime_mode default + compile_error guard**: mos hiq.
- **Zero if/else**: ligj — match/boolean/formula.

---

## 8. KUFIZIM I NDERSHËM
- **Kerneli C: kompiluar REAL me gcc** — s'është simulim.
- **Rust cargo build: S'U EKZEKUTUA** — toolchain i padisponueshëm, rrjet 403.
  Verifikim: statik (0 if/else + balancim) + Python-sim i logjikës.
  `cargo build`/`cargo test` final në makinën tënde. Pikat me rrezik kompilimi:
  pattern-i `unwrap_or_else(\|p\| p.into_inner())` (kërkon që guard-i të mbështesë
  into_inner — standard për Mutex/RwLock), is_traced te TracedInput.

---
**LIGHT — i mbyllur për runtime. Gati për Quantum (platforma e dytë).**
