# HARDENING v1.3.1 — PACP NË URË + GCL LIVE + BOOT SOVRAN
**GJATA LEGACY™ | Arkitekt: Bledar Gjata | Zbatues nën drejtim: Claude (Gardian)**
**Data: korrik 2026 | Baza: v1.3.0 (238 .rs, 935 teste)**

---

## AKSIOMA E RE E FORMALIZUAR NË KOD

> **Primitive Anchor Continuity Principle (PACP)** — *«Primitive Anchor (i₀) duhet të
> mbetet i pandryshuar gjatë gjithë ciklit të ekzekutimit dhe të shërbejë si referenca
> përfundimtare e verifikimit. Asnjë konkluzion nuk mund të konsiderohet i verifikuar
> pa u krahasuar me Primitive Anchor.»* — formulimi i Arkitektit, tani invariant i
> zbatuar dhe i testuar në runtime, jo metaforë.

Kollapsi verifikues në Shadow **nuk shkaktohet nga vëzhgimi** — ndodh sepse i₀ **është
aty** si referencë deterministike: Y krahasohet me ankorën, `Verified(Y)` vendoset **së
pari**, dhe **vetëm pas** kësaj propagohet `Trust(X)`. Rendi Y→X është ligj.

---

## ÇFARË U MBYLL NË KËTË VERSION

### 1. PACP në urë — kontrata e re `pa_wire.rs` (×3, byte-identike, md5 i njëjtë)
- **CRC FNV-1a 64 (`u64`, zero deps të jashtme)** mbi **TË PESTA fushat e plota**
  (`sid|pa_hex16|ts|xi:…|yi:…`) — vendimi i Arkitektit: *byte-për-byte*, ts brenda.
- **Light** shkruan me `pa_wire::encode_line` → rreshti 6-fushësh `…|c:%016x`.
- **Shadow** verifikon me `pa_wire::verify_line` **PARA çdo interpretimi**:
  `SealedOk` (CRC i saktë) / `LegacyOk` (3/5 fusha, i deklaruar) /
  `Corrupt` → `ShadowError::TransportCorrupt` — sesioni refuzohet para gjykimit.
- Çdo bit i ndryshuar në urë kapet (test: `single_bit_flip_is_corrupt`).
- Kontratat tani **12/12 + 5/5 identike ×3** (pa_wire u shtua në të tre `mod.rs`).
- *Vendim filozofie:* propozimi i auditit për CBOR/MsgPack u **rrëzua** — thyen ligjin
  "zero external deps"; CRC-tekst e arrin të njëjtin integritet brenda ligjit.

### 2. GCL LIVE LOOP — laku Y→X në runtime real (zero simulim)
- Pikë e re në `shadow_gateway::ingest_bridged` (**jashtë zonës së ndaluar**), **pas**
  verdiktit: `gcl_reinforce_on_verified(session, verdict)`.
- `Verified(Y)=1` **dhe** ankorë PA e regjistruar për sesionin →
  `vault.confirm_primitive(pa_id, 1)`: `real_hits+1` + kaskadë promovimi
  hipotezë→fakt. Sinjal **real**: verdikt i vërtetë mbi ankorë të vërtetë.
- `Verified(Y)=0` → `Trust(X)` **nuk** propagohet (dega negative mbetet te
  `judge_supreme`/`write_negative`, e paprekur).
- API e re vetëm-lexim në vault: `primitive_real_hits(input_id)` — auditon lakun.
- Test: `gcl_live_reinforce_only_on_verified_y` (Y=0 → hits konstant; Y=1 → +1),
  i ndërtuar mbi firmën **reale** 9-argumentëshe të `write_primitive`.

### 3. BOOT SOVRAN — zero rrugë relative, zero improvizim (×3 ekzekutues)
- `handoff_path` i ri **identik në logjikë** në Light/Quantum/Shadow:
  - `ESSMAI_HANDOFF_DIR` **mungon** → `FATAL (fail-closed)` me udhëzim të saktë
    (Linux/macOS + Windows) dhe `exit(1)` — **asnjë fallback cwd**.
  - Prezent → `create_dir_all` + `canonicalize` → rrugë **ABSOLUTE** e deklaruar.
- Gjetja e auditit (fallback relativ i trefishtë) konfirmohej rresht-për-rresht
  para rregullimit — tani `grep 'Err(_) => file.to_string()'` = **zero**.

### 4. TRACE RE-VERIFY — `bridge_light::build_output`
- `trace_id` nuk supozohet më i saktë: pas ndërtimit të seal-it (`QNT:<trace>:<hash16>`),
  hash-i **riprodhohet** nga i njëjti trup kanonik dhe seal-i **rikrahasohet**;
  divergjencë e brendshme = **ALARM KRITIK** me zë — gjurma s'del e heshtur.

### 5. HW_NOMINAL — deklarimi sovran i sensorëve (boot i Quantum)
- Pa `--features hw_kernel`: `KUJDES: HW_NOMINAL — sensorët JO realë (vlera të
  deklaruara nominale)` + udhëzimi i aktivizimit. Me feature: `SENSORË REALË`.
- Parimi "zero improvizim": vlera nominale **kurrë** e maskuar si matje.

### 6. CI GATE i zgjeruar — 10 → **15 kontrolle**
- [11] pa_wire ×3 identike + i lidhur në të dy anët (writer/reader)
- [12] boot sovran: zero fallback relativ + FATAL i deklaruar ×3
- [13] GCL live i lidhur në `ingest_bridged`
- [14] trace re-verify prezent në `bridge_light`
- [15] deklarimi HW në boot të Quantum

---

## VERDIKTET MBI AUDITET E REJA (dok. në chat + `content`)

| Gjetja | Verdikti | Arsyeja |
|---|---|---|
| Rrugë relative në `handoff_path` (×3) | **PRANUAR** → boot sovran FATAL | Konfirmuar rresht-për-rresht; determinizmi kërkon rrugë absolute të deklaruara |
| `hw_kernel` opsional pa njoftim | **PRANUAR** → HW_NOMINAL me zë | "Zero improvizim" — operatori e di gjithmonë burimin e vlerave |
| Format binar CBOR/MsgPack për urat | **RRËZUAR**; **PRANUAR** thelbi si CRC-tekst | CBOR/MsgPack thyen "zero external deps"; fnv1a64 (u64) e jep integritetin brenda ligjit |
| `trace_id` s'riverifikohet në `build_output` | **PRANUAR** → re-verify fail-loud | Integriteti i gjurmës Light↔Quantum↔Shadow |
| Vetë-kontroll konfigurimi në boot | **PRANUAR** (mbulohet nga boot-i sovran) | Dështimet bëhen të parashikueshme, në nisje, jo vonë |
| "Handshake versioni mes platformave" | **SHTYRË** (dokumentuar) | pa_wire mbulon urën PA me vetë-versionim fushësh (3/5/6); handshake i plotë PD/urat e tjera = fazë e ardhshme, kërkon prekje më të gjerë kontratash |
| "Shadow ~85–90% gati; s'ka më unwrap/panic në pjesën sovrane; time_degraded raporton" | **KONFIRMUAR** (gjendja v1.3.0) | Pa veprim — konstatime të sakta |

---

## BILANCI I VERIFIKUAR (pas v1.3.1)

- **241 .rs** (238 + pa_wire ×3) · **955 teste** (935 + 6 pa_wire ×3 + 1 PACP-feed + 1 GCL-live)
- **Zero if/else** jashtë build.rs · **brace 0** kudo · **zero tokena versioni** · **zero fallback relativ**
- **5/5 zona të ndaluara**: md5 **identike** (të kyçura në CI)
- **Kontratat: 12/12 + 5/5 identike ×3** (pa_wire.rs md5 i njëjtë në të tre)
- **C kernel**: `SOVEREIGN_KERNEL_RUNTIME = OK`
- **CI GATE: 15/15 ✓ EXIT=0**

## RUNTIME REAL — ÇFARË DO TË SHOHËSH NË MAKINË

1. `export ESSMAI_HANDOFF_DIR=/opt/essmai/handoff` (ose Windows-ekuivalenti) — pa të,
   asnjë binar s'niset (FATAL i qëllimshëm).
2. Light shkruan ankorën **të vulosur CRC**; Shadow printon
   `[PACP] ankora e vulosur CRC-OK — i₀ i pandryshuar në urë`.
3. Pas çdo verdikti `D=1`: `[GCL_LIVE] Verified(Y)=1 → Trust(X): … real_hits+1` —
   **laku i besimit rrjedh live**, i matshëm në vault.
4. Boot i Quantum deklaron `HW_NOMINAL` ose `SENSORË REALË` — kurrë heshtje.

## HAPI TJETËR (siç e ke caktuar)

Mbyllja e plotë e **Light** dhe **Quantum** me të njëjtin standard + **`setup_essmai.ps1`**
(instalues Windows: env-vars, dosjet, build me `hw_kernel`, ekzekutim i ci_gate) —
që ura të jetë **gati në makinë**, jo në letër.

---
*"i₀ nuk vëzhgon — i₀ ËSHTË referenca. Kollapsi ndodh sepse ankora është aty." — PACP, tani ligj i ekzekutueshëm.*
