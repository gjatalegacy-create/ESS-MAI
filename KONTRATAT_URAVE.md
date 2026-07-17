# ESS-MAI — KONTRATAT E URAVE (platformë ⇄ platformë)

**GJATA LEGACY™ — dokument kontrate. Formatet më poshtë janë TË MBYLLURA:
çdo ndryshim kërkon vendim të Arkitektit dhe përditësim të njëkohshëm të
shkruesit, lexuesit dhe këtij dokumenti.**

Tri platformat janë **binarë të ndarë**. Komunikojnë me: (a) struktura
`repr(C)`/bridge me pronësi (move), (b) skedarë këmbimi append-only me
rreshta `|`-të-ndarë. Transporti i skedarëve është **fail-open** (mungesa →
pasiv); vendimet janë **fail-closed** (te gate-et e ingest-it).

---

## URA 1 — Quantum → Light: `quantum_pd_export.txt`

| | |
|---|---|
| Shkruesi | `quantum/src/main.rs :: export_pd_handoff()` |
| Lexuesi | `light/src/main.rs :: read_pd_surface()` → `pd_light::parse_handoff()` |
| Formati | `session_id|dominant_concept|accumulated_mass|structural_coherence|genius_score|estimated_trl[|probe_hint][|c:%016x]\n` |
| Fushat | **6/7 legacy të deklaruara** ose **8 = 7 + vulë CRC** (`|c:`, fnv1a64 mbi 7-shen) — v1.4.0 |
| Shkrimi | `pa_wire::seal_body_verified(body,&[6,7])` — **vetë-gjykim para daljes** (WIRE_INVARIANT kufi 0) |
| Leximi | `pa_wire::verify_line_generic(line,&[6,7],8)` **PARA interpretimit** + **çelës njëdrejtimësh** (legacy pas vule = DEGRADIM→pasiv) + heqje `|c:` para `parse_handoff` |
| Semantika | append-only; lexuesi merr rreshtin e **fundit** të session-it |
| Dështimi | shkruesi fail-open (UI pasiv); lexuesi `Option` (asgjë → pa `[PD_LIGHT]`); **çdo rrjedhë → `rrjedha::note` me PSE** |

## URA 2 — Light → Shadow: `light_pa_export.txt` (PA-gate)

| | |
|---|---|
| Shkruesi | `light/src/main.rs :: export_primitive_anchor()` |
| Ushqyesi | `shadow/src/shadow_gateway.rs :: feed_primitive_anchors(path, session)` |
| Thirrja | `quantum/src/main.rs` — pas `Shadow::new()`, **para** `ingest_bridged` |
| Formati | `session_id|pa_id_hex|ts_ns\n` — `pa_id_hex = {:016x}` (16 hex, lowercase) |
| Semantika | append-only; ushqyesi merr ankorën e **fundit** të session-it dhe thërret `register_primitive_anchor` (idempotent) |
| Gate | **log-only** aktualisht; shndërrimi në bllokues = vendim i Arkitektit (i shtyrë) |
| Testet | `shadow_gateway.rs :: pa_feed_tests` (4 teste transporti) |

## URA 3 — Quantum → Shadow: `ingest_bridged(QuantumInbound, LightInbound)`

| | |
|---|---|
| Hyrja | `shadow/src/shadow_gateway.rs :: ingest_bridged` — **hyrja e vetme sovrane** |
| Origjina Q | `QuantumInbound` (package_id, session_id, territory, masat epistemike, HCP) |
| Origjina L | `LightInbound` nga `QuantumShadowBridge::split(&inbound)` |
| Ligji | **fail-closed**: `l.is_valid()==false` → `Err(SealInvalid)` para çdo pune; LIGJI 0 — move, zero klon |

## URA 4 — Light → Quantum: transporti bus + `split`

| | |
|---|---|
| Rrjedha | Light dispatch → bus (light_buss, CRC32) → `QuantumShadowBridge::split(&inbound)` → `(pkg, light_in)` |
| Konsumi | `light_in.payload` → `raw_bytes` të LIM; `light_in` kalon **i plotë** te URA 3 |

## URA 5 — Shadow → Light: `shadow_seal_bridge` (ZONË E NDALUAR)

| | |
|---|---|
| Skedari | `light/src/shadow_seal_bridge.rs` — **byte-for-byte i paprekshëm** |
| Statusi | verifikuar identik me origjinalin (md5); 9 API publike |
| Semantika | verdikti + vula 500/0xA451 kthehen te Light për `format_output` |

---

## URA 6 — Të TRIA → dëshmi: `rrjedha_ledger.txt` (v1.3.2, VETËM-SHTIM)

| | |
|---|---|
| Shkruesit | `lab_contracts::rrjedha::note()` — Light/Quantum/Shadow (kontratë ×3) |
| Lexuesi | ASNJË në runtime — skedar **dëshmie** për auditorin/operatorin |
| Formati | `ts_ns\|platform\|site\|class\|count\|diagnoza\|c:%016x\n` — **7 fusha, vetë-CRC** |
| Klasat | `TRANSPORT_BITFLIP/TRUNCATED/DOWNGRADE, LOCK_POISONED, DISK_DENIED, WIRE_INVARIANT, ANCHOR_LOST, OTHER` — secila me **kufi determinist** |
| Semantika | append-only; **KURRË autoritet, KURRË bllokues** — dështimi i vetë ledger-it raportohet me zë dhe puna vazhdon |
| Ligji | çdo rrjedhë merr **PSE** (diagnozë e klasifikuar) + **SHËNIM** (rresht i vulosur) + **KUFI** (`RrjedhaVerdict::AtLimit` → thirrësi vendos sipas ligjit të tij) |
| Ndarja | 7-fushëshi refuzohet qëllimisht nga `pa_wire::verify_line` — ledger-i s'ngatërrohet dot kurrë me ankorat (i provuar me test) |

---

## URA 8 — Light+Quantum+Shadow → `trl_vector.txt` (VEKTORI TRL, v1.4.5)

| | |
|---|---|
| Shkruesit | Light `in` (besueshmëria e lingua-s), Quantum `rsn` (estimated_trl real i PD), Shadow `ver` (verdikti real) |
| Formati | `session_id\|dim\|lvl\|c:%016x` — 3 fusha + vulë (`verify_line_generic(&[3],4)`) |
| Montimi | **vetëm Shadow** (i vetmi që sheh fundin) lexon 3 dimensionet dhe ndërton `TrlVector` në `gcl_apply` |
| Ligji | zinxhiri i plotë deeptech Light→Quantum→Shadow; `system_trl=min`; `[TRL_VEKTOR] i PLOTË/i pjesshëm` |
| Dështimi | append+flush best-effort; dimension që mungon → vektor i pjesshëm i deklaruar |

---

## Kontrata e brendshme e përbashkët: `lab_contracts/` (×3 identike)

15 skedarë **byte-for-byte identikë** në `light/`, `quantum/`, `shadow/`:
`mod.rs, trust.rs, trl.rs, pressure.rs, evidence.rs, verdict.rs, memory.rs,
message.rs, domains.rs, collapse.rs, gjata_collapse_law.rs, pa_wire.rs,
rrjedha.rs, gcl_presume.rs, first_guardian.rs`.

`first_guardian.rs` (v1.4.4) — **DËSHMIA E FRYMËS SË PARË** (paradigma e Gardianit):
në çdo ndezje, çdo platformë ushtron organet e veta kritike NË MAKINË
(`RiteCheck` fn-pointer → bool), vulos dëshminë me vetë-CRC në
`first_guardian.txt` (`ts|platform|FRYMA|ok/total|te_dështuarit|c:`) dhe
dështimin e EMËRON (rrjedha Other + ALARM) pa bllokuar kurrë runtime-in.
Përfshin **Known-Vector Live Lock**: fnv mbi vektorin e njohur + rrethi
encode→verify riprovohen live në çdo ndezje.

`trl.rs` (v1.4.4) — +**TrlVector** (shtylla deeptech): input/reasoning/
verification me `system_trl()=min` (zinxhiri i dobët) dhe `is_complete()`;
dimensioni i arsyetimit i lidhur me `evidence.trl_level` real në Quantum.

`gcl_presume.rs` (v1.4.1) — **gatishmëria nën ligj**: gjata_collapse_law
(CollapsePhase: Coordination/Reasoning/Verification) është KOMANDANTI i tre
makinave të platformave; presume varet prej tij dhe mban në gatishmëri edhe
modulet anësore — ndërfaqja `gcl_role()`/`gcl_status()` (rol + READY/DEGRADED/
NOT_READY), regjistrim idempotent me fn-pointer, `sweep(platform, phase, dir)`
me matricë domosdoshmërie (Support kurrë i domosdoshëm); rol i domosdoshëm JO
gati → shënim `rrjedha` me PSE; **kurrë bllokues**.

`pa_wire.rs` (v1.3.2) — përveç `encode_line/verify_line`, tani edhe
`encode_line_verified()`: **parandalimi në burim** — shkruesi vetë-gjykohet
me gjykatësin e lexuesit PARA daljes në tel; asgjë e refuzueshme s'emetohet.

`rrjedha.rs` (v1.3.2) — **inteligjenca e dështimeve**: klasa+kufij+shënim i
vulosur; informon, kurrë s'urdhëron; zero varësi, zero if/else, zero bllokim.

## URA 7 — Shadow → Quantum: `shadow_nk_export.bin` (NKB1, VULA BINARE, v1.4.2)

| | |
|---|---|
| Shkruesi | `KnowledgeVault::seal_negative_export()` (Quantum e mbështjell para shkrimit) |
| Lexuesi | `KnowledgeVault::from_negative_export()` — verifikon kokën para parse |
| Formati | **NKL1(4) \| body_len_le(8) \| crc_le(8) \| trup** (v1.4.3); NKB1(4)\|crc(8)\|trup pranohet (v1.4.2); raw legacy vetëm PA histori vule |
| Çelësi | markeri `shadow_nk_export.sealed` — sapo vuloset NJËHERË, raw = **DEGRADIM** (`nk_downgrade`) |
| Semantika | binar, count/len-prefixed; **një-drejtimësh**: sapo vulohet, s'zbritet |
| Dështimi | CRC s'përputhet → vault BOSH + alarm (fail-open i deklaruar); legacy pa kokë pranohet i pandryshuar |
| Ligji | dija negative KURRË s'kalon e prishur — bitflip në disk kapet në lexim |

---

`domains.rs` — regjistri kanonik i **9 domeneve bërthamë** (slot i 10-të i
rezervuar për Arkitektin): çelësa ASCII kanonikë për numërimin unik të
kristalizimit (≥5 domene, vula 0xA451) + `territory_key()` → territoret
ekzistuese të Quantum + `normalize_domain_key()` (vetëm identifikatorë —
**tokenizimi gjuhësor i lingua-s i paprekur**).
