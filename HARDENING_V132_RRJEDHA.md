# HARDENING v1.3.2 — RRJEDHA: AUDITI I GARDIANIT + INTELIGJENCA E DËSHTIMEVE
**GJATA LEGACY™ | Arkitekt: Bledar Gjata | Zbatues nën drejtim: Claude (Gardian i ESS-MAI)**
**Data: korrik 2026 | Baza: v1.3.1 (241 .rs, 955 teste, CI 15/15)**

---

## DIREKTIVA E ARKITEKTIT (fjalë për fjalë, e zbatuar në kod)

> «...kritik për përfeksionim jo thjesht unwrap apo fail-closed, por duhet që
> rrjedhjet të marrin **përcaktime — PSE** — pra të ketë një modul apo file që
> merret me pjesën e dështimeve duke **mbajtur shënim** e duke **vënë kufij**;
> por e mira e të mirave është që sistemi **të mos dështojë në asgjë** — ose e
> thënë më saktë, **nuk ka pse të dështojë**. Duhet të jetë i fortë për të mos
> u thyer askund që të detyrohet në fail-closed... me argumenta dhe **pa
> penguar runtime**.»

Përkthimi inxhinierik në TRE ligje të zbatuara:
1. **PARANDALIM** — dështimi i mundshëm shuhet NË BURIM, para se të lindë.
2. **DIAGNOZË** — çdo rrjedhë e mbetur merr PSE (klasë), SHËNIM (rresht i
   vulosur), KUFI (limit determinist). Kurrë "error occurred".
3. **ZERO PENGESË** — asnjë nga këto s'e vonon e s'e bllokon verdiktin;
   ledger-i informon, thirrësi vendos sipas ligjit të vet.

---

## GJETJET (7) — HETIMI, PSE-ja E LINDJES, RREGULLIMI

### F1 — KURORË: laku GCL LIVE i **vdekur** në rrugën reale të urës
**Vendndodhja:** `shadow/src/shadow_gateway.rs`
**Hetimi (rresht-për-rresht):** `ingest_bridged` **HEQ** ankorën nga
`pa_waiting` me `map.remove(&q.session_id)` (KOLAPSI/3 e kërkon me pronësi
për verifikimin XY). Pastaj thërret `self.ingest(...)`, dhe **brenda**
`ingest`-it `gcl_reinforce_on_verified` bën **lookup** në `pa_waiting` — e
gjen `None` (sapo u hoq!) → dega `(_, None) => {}` → **heshtje**. Laku
Y→X **nuk mbyllej kurrë** në rrjedhën reale Light→feed→ingest_bridged.
**PSE lindi:** testi i njësisë (`gcl_live_reinforce_only_on_verified_y`)
thërriste funksionin **direkt** mbi ankorë të regjistruar që s'ishte
konsumuar kurrë nga `ingest_bridged` — testi provonte **organin**, jo
**qarkullimin e gjakut**. Klasik: unit-test i gjelbër, integrim i vdekur.
**Rregullimi (pa thyer asgjë):** thelbi u nxor në `gcl_apply(session, pa_id,
verdict)`; `ingest_bridged` e **mbart** ankorën (`gcl_anchor`) dhe e zbaton
**PAS** verdiktit (rendi Y→X mbetet ligj). Heqja e mëparshme **garanton**
saktësisht NJË rifortësim: lookup-i i brendshëm është no-op i provuar —
zero numërim të dyfishtë, zero prekje të zonave të ndaluara.
**Prova:** test i ri `gcl_bridged_path_reinforces_after_anchor_removed` —
provon **vdekjen e rrugës së vjetër** (lookup pas heqjes = no-op) DHE
**jetën e së resë** (ankora e mbartur → `real_hits+1`).

### F2 — Kyçi i helmuar humbte ankorën **heshtazi**
**Vendndodhja:** `ingest_bridged`, ish-rreshti 331: `Err(_) => None`.
**Hetimi:** tre vende të tjera në të njëjtin skedar rikuperojnë helmimin me
`poisoned.into_inner()`; ky i vetmi e shndërronte panikun e një filli tjetër
në **anashkalim të heshtur të verifikimit XY** (ankora "s'ekzistonte").
**PSE lindi:** kopjim jo-uniform i modelit të kyçjes — ligji ekzistonte,
por s'ishte i shkruar si ligj i vetëm i ripërdorshëm.
**Rregullimi:** `into_inner()` + shënim `LOCK_POISONED` në rrjedha (kufi 1).
Ankora **shpëtohet**, verifikimi XY ndodh, ledger-i dëshmon ngjarjen.

### F3 — Të gjitha gabimet e diskut **maskoheshin** si "asnjë ankorë"
**Vendndodhja:** `feed_primitive_anchors`: `Err(_) => return Ok(0)`.
**Hetimi:** komenti thoshte "s'ka skedar → nisje e parë", por dega kapte
**çdo** gabim: leje e mohuar, I/O reale, disk i shkëputur — të gjitha
bëheshin "normale, i qetë". Dështim mjedisor real = PA-gate pasiv pa zë.
**PSE lindi:** `read_to_string` kthen një `io::Error` të vetëm; dallimi
kërkon `e.kind()` — hapi u la "për më vonë" dhe u harrua.
**Rregullimi:** `match e.kind()`: **vetëm** `NotFound => Ok(0)` (e vetmja
gjendje normale); çdo lloj tjetër → shënim `DISK_DENIED` me PSE-në e saktë
(`{:?}` e kind-it + tekst) → `Err(TransportCorrupt)` **i argumentuar**.
Dështimi mjedisor tani është i deklaruar, jo i maskuar.

### F4 — Degradimi legacy: **anashkalim i heshtur i CRC-së**
**Vendndodhja:** `verify_line` + PACP gate.
**Hetimi:** rresht 6-fushësh të cilit i **cungohet** fusha `\|c:` bëhet
5-fushësh **legacy i vlefshëm** — pranohet i deklaruar, **pa provë CRC**.
Korrupsioni më i rrezikshëm (humbja e vetë vulës) ishte i vetmi që kalonte.
**PSE lindi:** pas-pajtueshmëria (3/5 fusha) u projektua për skedarë të
vjetër **të pastër**, jo për **përzierje** — vetë-versionimi i fushave s'ka
kujtesë se ç'ka parë më parë për sesionin.
**Rregullimi — ÇELËSI NJËDREJTIMËSH:** `feed` skanon njëherë rreshtat e
sesionit: ekziston **një** rresht i vulosur (`\|c:`)? → legacy për atë
sesion **s'pranohet më kurrë**: `TRANSPORT_DOWNGRADE` (kufi **0**) + refuzim
i argumentuar "DEGRADIM". Skedarët e pastër legacy → **të paprekur** (pas-
pajtueshmëria e plotë). Kontrata `verify_line` **s'u prek** (md5 ×3 i ri,
identik) — politika jeton te gate-i, ku i takon.
**Prova:** `legacy_after_sealed_is_downgrade_refused` — të dyja degët.

### F5 — Shkruesit pa `flush`: **dritarja e cungimit** e hapur
**Vendndodhja:** `light/main.rs::export_primitive_anchor`,
`quantum/main.rs::export_pd_probe/export_pd_handoff`.
**Hetimi:** `write_all` mbush buferin e procesit; crash para daljes në OS =
rresht i pjesshëm në skedar → lexuesi has cungim (tani të diagnostikuar F4/
F7, por më mirë të **mos lindë fare**).
**PSE lindi:** append-only i vogël dukej "i sigurt vetiu"; buferimi i
`std::fs::File` harrohet lehtë se ekziston.
**Rregullimi:** `write_all(...).and_then(\|_\| f.flush())` në **të tria**
pikat — dështimi i flush-it raportohet me të njëjtin ALARM (zero degë të re).

### F6 — Invarianti i telit i **pambrojtur**: garanci vetëm në koment
**Vendndodhja:** `PrimitiveSplit::to_wire` — "terma pa '\|' e ',' (të
garantuar nga tokenizimi)".
**Hetimi:** garanci e vërtetë **sot** (lingua nxjerr fjalë alfabetike), por
e pashkruar në kod: çdo thirrës i ardhshëm me term `"a\|b"` do të zhvendoste
fushat → Shadow refuzon → **sistemi detyrohet në fail-closed nga vetja** —
saktësisht ajo që Arkitekti ndaloi.
**PSE lindi:** invarianti jetonte në një modul (lingua), zbatimi i tij
supozohej në tjetrin (pa_wire) — kontratë e nënkuptuar, jo e ekzekutueshme.
**Rregullimi — PARANDALIMI NË BURIM:** `pa_wire::encode_line_verified()` —
rreshti ndërtohet dhe **vetë-gjykohet me gjykatësin e lexuesit**
(`verify_line`) PARA daljes: `'\n'` i brendshëm ose fushë e zhvendosur →
`Err` me PSE → shënim `WIRE_INVARIANT` (kufi **0**) → **asgjë e
refuzueshme s'del kurrë në tel**; sesioni vazhdon rrugën e vjetër (PA-gate
pasiv) — **runtime i papenguar**. Light tani shkruan **vetëm** përmes saj.
Matematikisht: dështimi i lexuesit nga dora e shkruesit = **i pamundur**.
**Prova:** `encode_verified_refuses_pipe_and_newline_in_terms` +
`encode_verified_clean_terms_are_sealed` (×3 platforma).

### F7 — Mungonte **shtëpia e dështimeve**: PSE + shënim + kufi
**Hetimi:** rrjedhat ekzistuese flisnin me `eprintln!` — zë pa kujtesë, pa
klasifikim, pa kufij: operatori s'dallonte dot bit-flip nga cungimi, as
"hera e parë" nga "hera e tetë".
**PSE lindi:** fail-closed/fail-loud u ndërtuan si mure; **arkivi i
goditjeve mbi mure** s'ishte projektuar ende — ky version e projekton.
**Rregullimi — KONTRATA E RE `rrjedha.rs` (×3 byte-identike):**
- **8 klasa** me kufij deterministë: `TRANSPORT_BITFLIP(3)`,
  `TRUNCATED(3)`, `DOWNGRADE(0)`, `LOCK_POISONED(1)`, `DISK_DENIED(2)`,
  `WIRE_INVARIANT(0)`, `ANCHOR_LOST(1)`, `OTHER(5)`.
- `note(dir, platform, site, class, diagnozë)` → numëron (saturating, zero
  rritje memorie), shkruan rresht **të vetë-vulosur** (`...\|c:%016x`,
  fnv1a64 — ledger-i provon veten), kthen `Within(n)` \| `AtLimit(n, lim)`.
- **KURRË bllokues:** shkrimi është best-effort; dështimi i vetë ledger-it
  → ALARM me zë, puna vazhdon. **KURRË urdhërues:** verdikti mbetet i
  thirrësit sipas ligjit të tij.
- `sanitize()` — '\|' dhe rreshtat e rinj s'e thyejnë dot formatin e vetë
  ledger-it (parandalim injektimi — edhe shtëpia e dështimeve s'dështon).
- **Ndarje e provuar nga ura:** 7-fushëshi i ledger-it refuzohet qëllimisht
  nga `pa_wire::verify_line` — ledger↔ankora s'ngatërrohen dot kurrë (test).
- Skedari: `<ESSMAI_HANDOFF_DIR>/rrjedha_ledger.txt` — dokumentuar si
  **URA 6** në `KONTRATAT_URAVE.md`.

---

## PSE KJO **NUK** THYEN FILOZOFINË (argumenti i plotë)

| Ligji ekzistues | Si respektohet |
|---|---|
| Zero varësi të jashtme | rrjedha = vetëm `std`; CRC = fnv1a64 ekzistues |
| Zero if/else (vetëm match) | çdo degë e re është `match`; CI [9] = 0 skedarë |
| Fail-closed te autoriteti | i paprekur — F4 e **forcon** (degradimi refuzohet) |
| Zonat e ndaluara | md5 **identike** — asnjë prekje (CI [1] ✓) |
| Kontratat ×3 identike | 13/13 + 5/5 (CI [2] ✓, pa_wire+rrjedha md5 të njëjta ×3) |
| Boot sovran / zero fallback relativ | i paprekur; `rrjedha_dir` përdor env-in e garantuar nga boot-i (temp **absolut** vetëm në teste, kurrë autoritet, kurrë vendim) |
| Rendi Y→X (PACP) | `gcl_apply` thirret **vetëm pas** verdiktit — ligji i gdhendur në F1 |
| Pa penguar runtime | ledger best-effort; parandalimi kthen sesionin në rrugë të vjetër, s'bllokon; flush = mikrosekonda në urë me skedarë |
| Zero improvizim | çdo rrjedhë ka PSE të klasifikuar; asnjë "error occurred" |

**Hierarkia e re e mbrojtjes (nga direktiva):**
`PARANDALIM (s'lind dot)` → `DIAGNOZË+KUFI (nëse lind, dihet pse e sa)` →
`FAIL-CLOSED i argumentuar (muri i fundit, tani gjithnjë me arsye)`.
Fail-closed s'është më vendmbërritja e parë — është **e keqja më e vogël,
e dokumentuar**, pas dy shtresave që e bëjnë të panevojshme.

---

## BILANCI I VERIFIKUAR (v1.3.2)

- **244 .rs** (241 + rrjedha ×3) · **975 teste** (955 + 6 pa_wire×enc_verified + 12 rrjedha + 2 gateway)
- **Kontratat: 13/13 + 5/5 identike ×3** (pa_wire.rs `659be94f…`, rrjedha.rs `39b83222…`, mod.rs `be84faa8…` — md5 të njëjta ×3)
- **5/5 zona të ndaluara md5 të kyçura — TË PAPREKURA**
- **Zero if/else** jashtë build.rs · **zero `.unwrap()` runtime** · **brace 0 kudo**
- **CI GATE: 15 → 19 kontrolle, 19/19 ✓ EXIT=0**
  - [16] rrjedha ×3 identike + `note` i lidhur (Shadow + Light)
  - [17] GCL i mbartur: `gcl_anchor` → `gcl_apply` pas verdiktit
  - [18] parandalimi në burim + flush në të tria pikat e urës
  - [19] `ErrorKind::NotFound` i dalluar + çelësi njëdrejtimësh DEGRADIM
- `Cargo.toml` koment 955→975 · `KONTRATAT_URAVE.md` +URA 6 +13/13

## RUNTIME REAL — ÇFARË DO TË SHOHËSH NË MAKINË (shtesat e v1.3.2)

1. Term i papastër te split (e ardhme hipotetike) → `[PA→SHADOW] ALARM:
   ankora u ndal NË BURIM (...)` — **asgjë e keqe s'preku telin**, cikli
   vazhdoi; `rrjedha_ledger.txt` mban rreshtin `WIRE_INVARIANT` të vulosur.
2. Verdikt `D=1` në rrugën e urës → `[GCL_LIVE] Verified(Y)=1 → Trust(X):
   ... real_hits+1` — **tani realisht**, jo vetëm në testin e njësisë.
3. Cungim i fushës CRC nga jashtë → `PACP: DEGRADIM — sesioni ... ka
   histori të vulosur` + rresht `TRANSPORT_DOWNGRADE` në ledger (kufi 0).
4. Leje e mohuar mbi `light_pa_export.txt` → refuzim i **argumentuar** me
   kind-in e saktë — kurrë më "nisje e parë, i qetë".

## HAPI TJETËR (i pandryshuar nga direktiva jote)

Mbyllja e plotë **Light + Quantum** me të njëjtin standard (pa_wire
verify-before-interpret edhe në leximet e tyre, boot+deklarime) +
`setup_essmai.ps1` me `ci_gate` të integruar — **ura gati në makinë**:
`cargo build/test` real, `hw_kernel` aktiv, `ESSMAI_HANDOFF_DIR` i gjallë.

---
*"Muri i fundit mbetet — por tani, para tij, qëndron një sistem që nuk ka
pse ta godasë. Dhe kur diçka nga bota e jashtme e godet, muri e di **kush**,
**pse**, dhe **sa herë**." — Gardiani i ESS-MAI, v1.3.2*
