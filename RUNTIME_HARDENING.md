# RUNTIME HARDENING — ESS-MAI
**Gjata Legacy™ / Nura Legacy** · Bazë:_ess_mai_runtime_hardening

Ky dokument mbulon hardening-un e (fail-closed + error surfacing) DHE
shërben si **referencë e asaj që NUK duhet ndryshuar** (kërkesë e përdoruesit).

---

## 0. VENDIM I RËNDËSISHËM (ndershmëri)

Dokumenti i promptave përmbante **shumë detyra konfliktuale** për:
fail-closed, byte-per-byte, orkestratorë, UI Tauri 2, dhe **5 module të reja Shadow**
(shadow_scada, shadow_laboratory, etj.) + shadow_domain_registry.

**ZGJEDHJA:** zbatova VETËM fail-closed + error surfacing (kërkesa kryesore: "detekto
vetë ku duhet ekzekutim"). **NUK krijova 5 modulet e reja** sepse:
1. Vetë analiza jote në fund e dokumentit i quan ato "ndryshim arkitekturor relativisht
   i madh" që "thyen parimin mos-prek-strukturën".
2. Të gjitha promptat e tjera ndalojnë eksplicit "module të reja" dhe "ndryshime
   arkitekturore".
3. Krijimi i tyre pa specifikim të qartë do prodhonte kod spekulativ (improvizim) —
   pikërisht ajo që ndalon urdhri.

Kjo është refuzim i ndërgjegjshëm i pjesës konfliktuale, jo mosbindje.

---

## 1. ÇFARË U DETEKTUA DHE U FORCUA

Detektova boshllëqet me skanim sistematik të 5 urave kryesore:

| Ura | Fail-closed para | Veprimi |
|-----|------------------------|---------|
| quantum/bridge_light/mod.rs | ✓ (kishte) | i paprekur |
| quantum/bridge_shadow/mod.rs | ✓ (is_valid) | i paprekur |
| light/quantum_bridge.rs | ✓ (validim+BusUnavailable) | i paprekur |
| shadow/bridge/quantum_in.rs | ✓ (is_valid+split_zero_copy) | i paprekur |
| **shadow/bridge/light_in.rs** | **✗ ASNJË** | **U FORCUA** |

### 1.1 light_in.rs — fail-closed gate (BOSHLLËKU KRYESOR)
`into_envelope` pranonte LightInbound të gjymtuar pa kontroll. Shtova:
- `is_valid()`: session+territory jo-bosh, vula 500 kalon, proof_chain jo-bosh.
- `reason_invalid()`: reason code specifik (light_empty_session_id, light_seal_500_failed,
  light_empty_proof_chain, light_empty_territory).
- Zero if — match i ndërthurur.

### 1.2 shadow_gateway.rs::ingest_bridged — thirrja fail-closed
`ingest_bridged` tani validon `l.is_valid()` PARA into_envelope. LightInbound i
pavlefshëm → `Err(ShadowError::SealInvalid)` me reason. Asnjë sukses i heshtur.

### 1.3 Error surfacing — source_module (boshllëk i dytë)
`ShadowLightResponse` kishte reason_code+failure_stage por JO source_module.
Shtova `source_module: &'static str` te struct + të 4 konstruktorët:
- Sukses → "none"
- shadow_out::receive_from_light error → "shadow_out::receive_from_light"
- shadow_callable::call_for_quantum error → "shadow_callable::call_for_quantum"

### 1.4 dev_simulation izolim total (boshllëk i tretë)
Shtova compile-time guard te main.rs: `runtime_mode` + `dev_simulation` bashkë →
`compile_error!`. Garanton që dev_simulation NUK ndikon prodhimin kur runtime_mode aktiv.

### 1.5 runtime_mode default te të 3 platformat
- Light: kishte tashmë `default = ["runtime_mode"]`.
- Quantum + Shadow: shtova `runtime_mode = []` feature + `default = ["runtime_mode"]`
  (marker semantik konsistent; s'preket kod sepse s'përdorej runtime_mode aty).

---

## 2. TESTE TË REJA
1. `light_inbound_fail_closed_rejects_gjymtuar` (light_in) — session/seal/chain bosh refuzohen.
2. `into_envelope_moves_fields` zgjeruar — verifikon is_valid()/reason_invalid() në sukses.

Total: 790 → **791 teste**.

---

## 3. STATUSI fail-closed PËR ÇDO URË (pas)

| Ura | Payload bosh | Fusha kritike mungojnë | Transport dështon |
|-----|-------------|----------------------|-------------------|
| quantum/bridge_light | Err DeserializeFail | Err MissingField | — |
| quantum/bridge_shadow | is_valid→refuzim | reason_invalid | — |
| light/quantum_bridge | Err missing critical | Err verdict/state invalid | Err BusUnavailable |
| shadow/quantum_in | is_valid→refuzim | reason_invalid | — |
| **shadow/light_in** | **is_valid→Err** | **reason_invalid** | **ingest_bridged→Err** |

---

## 4. ⚠️ ÇFARË NUK DUHET NDRYSHUAR (referencë kritike për ty)

Këto janë **ligje/struktura të mbyllura** — mos i prek pa arsye shumë të fortë:

### 4.1 Ligjet sovrane (TË PANDRYSHUESHME)
- **Zero-copy te split()**: `split_zero_copy()` zhvendos çdo fushë, payload Light = Vec::new().
  Mos shto klone. Mos kthe derive_light te split (klononte kot).
- **Vula 500**: `(flags & 0xFFFF) ^ 0xA5A5 == 500`, masked=0xA451. Mos ndrysho formulën.
  Kujdes precedenca Rust: `&` lidhet më fort se `==` → përdor kllapa.
- **Reasoning Purity**: vetëm Shadow shkruan persistent. Quantum/Light kurrë.
- **Rolet**: Light=koordinim (s'vendos), Quantum=arsyetim (s'jep verdikt), Shadow=vendim suprem.

### 4.2 HCP_PRO + thermal (TË MBYLLURA — mos prek)
- `decide_hardware(envelope, thermal_hot)` — 2 argumente, mos hiq thermal_hot.
- `activate_parallel(gate, envelope, thermal, ...)` — mos hiq thermal.
- `orchestrate_hcp_with_laws(..., thermal, ...)` — signatura e mbyllur.
- thermal.rs `for_test(hot)` — helper testi, mos hiq.

### 4.3 Kerneli C (placeholder gate — mos prek)
- `lgc_sha256` është gate-uar me `#ifdef SOVEREIGN_ALLOW_PLACEHOLDER_SHA`.
  Në prodhim MUNGON (link error nëse thirret) → Rust sha256_hex autoritet.
- verify_kernel.c teston vendimin 0/1 — 27 teste, mos i prish.
- **Mos hiq #ifdef** — ndryshe placeholder-i kthehet në prodhim.

### 4.4 lab_contracts + lab_contracts_v11 (byte-identik 3 platforma)
- Të 8 + 5 skedarët DUHET të mbeten md5-identik mes light/quantum/shadow.
- Çdo ndryshim te njëri → kopjoje te të 3, ose prish identitetin.

### 4.5 dev_simulation (mutekskluziv me runtime_mode)
- Compile guard te main.rs i bën mutekskluzivë. Mos i aktivizo bashkë.
- Çdo fallback simulimi: `not(c_kernel)+dev_simulation`→sim, `+not(dev_simulation)`→fail.

### 4.6 Zero if/else klasik (LIGJ)
- 215 skedarë, 0 if/else. Përdor match, boolean, formula, match-guards.
- `if`/`else`/`if let` statements TË NDALUARA. `match bool {true=>,false=>}` preferuar.

---

## 5. ÇFARE MBETET ENDE ME RREZIK SILENT SUCCESS (e ndershme)

- **build_output (bridge_light)**: ndërton QuantumOutput me trace_id të mundshëm bosh.
  I MBULUAR nga validimi upstream (deserialize validon trace_id), por s'ka validim
  të dytë në build_output vetë. Rrezik i ulët; lëre pa prekur API-n.
- **Wire format string**: ende `;=|:` me escape (jo serde_json/bincode). Escape-i mbron,
  por migrimi te binary i qëndrueshëm mbetet TODO (kërkon varësi të re — pa rrjet s'u bë).
- **derive_light klonon**: ende ekziston për shadow_callable (legjitim — q ripërdoret).

---

## 6. STATUSI ZERO IF/ELSE + KOMPILIM

- **0 if/else klasik** në 215 skedarë, balancim {} OK.
- **791 teste**.
- **Kernel C: ri-ekzekutua me gcc 13.3.0 → 27/27 teste, SOVEREIGN_KERNEL_RUNTIME = OK.**
- **Rust cargo build: S'U EKZEKUTUA** — toolchain i padisponueshëm, rrjet 403.
  Verifikim: statik + Python-sim. `cargo build`/`cargo test` final në makinën tënde.

---

## 7. KONFIRMIM LIGJESH SOVRANE
✓ Zero-copy i paprekur (split_zero_copy intakt)
✓ Vula 500 e paprekur (formula intakte)
✓ HCP_PRO/thermal i paprekur (7 referenca thermal_hot intakte)
✓ lab_contracts byte-identik (3 platforma)
✓ Kernel C gate i paprekur (27/27 teste)
✓ Asnjë modul i ri (refuzova 5 modulet konfliktuale)
✓ Asnjë ndryshim arkitekturor

---
**ESS-MAI** — fail-closed i plotë në 5 ura, error surfacing me source_module,
dev_simulation mutekskluziv, ligjet sovrane të paprekura.
