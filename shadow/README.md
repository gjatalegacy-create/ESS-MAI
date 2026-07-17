# ESS-MAI · SHADOW PLATFORM

**Gjata Legacy™ | Arkitekt: Bledar Gjata**

Autoriteti suprem epistemik i ESS-MAI — **memoria sovrane**. Shadow VETËM
verifikon, kategorizon dhe sistematizon dijen. Nuk propozon, nuk arsyeton, nuk
eliminon (ato i bën Quantum). Është **i vetmi** modul që e kupton vulën **500**
dhe vendos **0/1**, dhe i vetmi që shkruan **Dije Persistente**.

```
LIGHT  ──(transport + vulë 500)──►  ┌─────────┐
                                     │ SHADOW  │ ──► 0 / 1  +  KnowledgeVault
QUANTUM ──(propozim + arsyetim)──►  └─────────┘
```

---

## 0. Kufiri i procesit — v1.5.9

Shadow është target **vetëm binar** (`autolib = false`). `main.rs` përfshin
trupin e `lib.rs`, prandaj `Shadow::ingest`, vault-i, token-i dhe prodhimi i
`VerificationReceipt` ekzistojnë vetëm brenda procesit `shadow_platform`.
Quantum varet vetëm nga `shadow_contracts` dhe dërgon frame të versionuar e të
kontrolluar me checksum te `shadow_platform --bridge-once`. Pa ekzekutimin e
main-it, rrjedha ndalon fail-closed. API-të e përshkruara më poshtë janë API të
brendshme të target-it binar, jo library publike për crate të tjera.

---

## 1. Arkitektura — dy origjina, TË NDARA

Dy hyrjet mbërrijnë nga dy ura të ndryshme dhe mbeten **structe të ndara**:

| Struct          | Origjina | Përmban                                              |
|-----------------|----------|------------------------------------------------------|
| `PassPackage`   | Quantum  | masa epistemike, 5 dimensionet, kandidatët, raw_bytes |
| `LightEnvelope` | Light    | `session_id`, `territory`, **`primitive_flags` (500)**, `proof_chain`, `payload` |

Asnjëra **nuk** futet si nën-fushë e tjetrës. Ato **bashkohen vetëm brenda
`shadow_pipeline.rs`**, në një bartës të brendshëm `ShadowPassage`:

```
PassPackage  ─┐
              ├─►  ShadowPassage  ─►  [9 noda]  ─►  shadow_gj_legacy  ─►  SupremeVerdict
LightEnvelope ─┘     (i brendshëm)
```

**Hyrje e vetme, pipeline i vetëm:** `Shadow::ingest(pkg, light)`.

### Lidhjet me ekosistemin (kontratat e urave)

- **Light → `LightEnvelope`** — `shadow_seal_bridge`. Light llogarit vulën dhe e
  **transporton verbërisht** (`(500 & 0xFFFF) ^ 0xA5A5 = 0xA451`). Vetëm
  `shadow_gj_legacy` e interpreton si 500.
- **Quantum → `PassPackage`** — `quantum_shadow_bridge`. Struktura është 1:1 me
  `ShadowPassPackage` të Quantum.

---

## 2. LIGJI 0 — ZERO-COPY SOVEREIGN

> Shadow është memoria sovrane. Klonimi i inputit shkakton mbingarkesë.

Të gjithë verifikuesit e ndjekin këtë ligj:

- `pkg` dhe `light` **ZHVENDOSEN** (move) brenda `ShadowPassage` — kurrë nuk klonohen.
- Nodet lexojnë vetëm me referencë: `&passage.package`, `&passage.light`.
- `ShadowVerdict` është `Copy` — akumulimi i verdikteve nuk alokon kopje.
- Payload-i (raw_bytes) **materializohet një herë të vetme** — ZHVENDOSET në
  `KnowledgeVault` te `shadow_gj_legacy::judge_supreme` (kufiri i vetëm i
  persistencës). `session_id` zhvendoset në përgjigje me `std::mem::take`.
- `ShadowPassage` qëllimisht **nuk** implementon `Clone`.

> **Sqarim (#10):** Ligji ZERO-COPY vlen për **rrugën kryesore** të bashkimit
> (split → ShadowPassage → judge_supreme), e cila përdor vetëm MOVE. Funksioni
> `bridge::quantum_in::derive_light()` është një **kanal i dytë i qëllimshëm**
> (modelon transportin e pavarur Light) dhe klonon fushat transportuese me
> vetëdije. Ky nuk është thyerje e ligjit në rrugën kryesore; është kanal
> ndihmës me kosto klonimi të dokumentuar. Migrim i ardhshëm: `Arc<[u8]>`.

E vetmja "kopje" e mbetur është një `String` i vogël metadata (territory/process)
dhe `Arc::clone` (numërues references, jo të dhëna) — asnjë kopje e payload-it.

---

## 3. Rrugët dhe nodat (renditje fikse = renditja e `ShadowNode`)

`S.Router` zgjedh rrugën sipas masës epistemike:

| Rruga      | Kushti (masë)   | Nodat                                                            |
|------------|-----------------|-----------------------------------------------------------------|
| `Fast`     | `< 0.36`        | Router → Matrix → Judiciary                                     |
| `Standard` | `0.36 – 0.99`   | + Gen5 → Type → Sovereign                                       |
| `Deep`     | `≥ 0.99`        | + Temporal → Emergence → Consensus                             |

Asnjë nod nuk shkruan persistent — **vetëm `shadow_gj_legacy`** (në fund).

---

## 4. Ligjet sovrane (Pika 5 — e thjeshtuar)

```rust
enforce_sovereign_laws(&passage) -> Result<(), ShadowError>
```

Pesë invariantë **strukturorë**; shkelja → `Err(SovereignViolation)` (refuzim i
fortë, pa shkrim, pa verdikt):

| Ligj | Kontrolli                                              |
|------|--------------------------------------------------------|
| L1   | rekursion: `proof_chain ≤ 64`, `candidates ≤ 64`       |
| L2   | entitet i mbrojtur: `suggested_verdict ∈ {0,1,2}`      |
| L3   | pastërti: verdikti `S.Judiciary` i pranishëm           |
| L4   | konvergjencë: pipeline-i u plotësua (`≥ min_nodes`)    |
| L5   | autoritet: asnjë nod jashtë rrugës së zgjedhur         |

**Dallim i rëndësishëm:** *shkelja sovrane* ≠ *Dije Negative*. Një input me masë
të ulët ose që dështon në gjykatë regjistrohet si **Dije Negative** (kufi i
vlefshëm shkencor); shkelja sovrane është refuzim strukturor i sistemit.

---

## 5. Vendimi suprem (ZERO if/else)

`shadow_gj_legacy.lgc` (kernel C, i mbështjellë në Rust):

```
verified  = judiciary_ok                 (ligjet sovrane kaluan ⇒ allow ≡ 1)
primitive = verified  AND  seal_500_pass (vula mbijetoi & jo i ngrirë)
```

Vendimi i kernelit C është thjesht `live & sealed` (vula + ngrirja); `ram_usage`
prek vetëm gjendjen informative, jo vendimin.

### Bitmask: gjendje dije vs. qeverisje

- **`LGC_LAW_*` (kerneli C)** = klasifikues i **gjendjes së dijes**
  (PRIMITIVE / VERIFIED / LEGACY / NEGATIVE).
- **Shtresa Rust** (`enforce_sovereign_laws`) = **qeverisje** (refuzim strukturor).

`KnowledgeVault` ruan 7 dyqane: scientific, negative, hypothesis, fact,
primitive, sovereign, legacy. Primitivët maturohen → **Legacy** kur
`legacy_score > 0.75` (konfirmime reale nga bota).

---

## 6. Persistenca sovrane (FAZA 2 — WAL në disk)

Memoria sovrane duhet të **mbijetojë restart-in**. `KnowledgeVault` mban një
backend opsional persistence; `Shadow::with_disk(path)` e aktivizon (`Shadow::new`
mbetet efemere në RAM).

**Write-Ahead Log logjik + ripërsëritje besnike.** Çdo shkrim regjistrohet si
NJË ngjarje logjike PËRPARA se të aplikohet në RAM (*log-pastaj-apliko*):

```
on_primitive · on_verified · on_negative · on_confirm
```

Në startup, ngjarjet ripërsëriten përmes TË NJËJTËS logjikë aplikimi. Kjo
riprodhon EKZAKT:
- **dedup-in negativ** (frekuencë++ kur përsëritet i njëjti shkak+proces+masë);
- **promovimin Primitive→Legacy** — një primitiv fresh s'e arrin kurrë pragun
  `0.75` (maksimumi 0.65), pra çdo promovim vjen pas një `Confirm` të regjistruar.

**Formati i skedarit** (zero varësi — vetëm `std`):

```
[header:  u64 MAGIC][u32 VERSION]
[rekord:  u32 LEN][u32 CRC32][payload = tag + fusha]   × N
```

- **Durabilitet:** `fsync` (`sync_all`) pas çdo rekordi.
- **Siguri ndaj rrëzimit:** bishti i cunguar/i dëmtuar (CRC që s'përputhet ose
  etiketë e panjohur) shpërfillet dhe log-u shkurtohet te rekordi i fundit i mirë.
- **Ligji 0:** payload-i serializohet me referencë (`&[u8]`) — asnjë klon shtesë;
  disku është kufiri i vetëm i materializimit durable.

**Çfarë NUK persiston (qëllimisht):** `access_count` (popullariteti runtime) është
metadata e butë — rindërtohet afërsisht nga ripërsëritja e `Confirm`. Dyqanet
`scientific`/`sovereign` s'kanë ende shkrues në rrjedhën, prandaj nuk
regjistrohen (gati për t'u shtuar kur të kenë shkrues).

```rust
let shadow = Shadow::new()?;                       // RAM (efemere)
let shadow = Shadow::with_disk("shadow_vault.wal")?; // DURABLE (mbijeton restart)
```

Asnjë kontratë nuk ndryshon për Light/Quantum: durabiliteti është tërësisht i
brendshëm i Shadow-it (Light vetëm transporton vulën 500; Quantum vetëm propozon
`ShadowPassPackage`). Tipet që serializohen (`String`/`Vec<u8>`/`f32`/`u32`/`u64`)
përputhen 1:1 me fushat që mbërrijnë nga të dy modulet.

---

## 7. Ligji i gjurmueshmërisë + Porta sovrane FFI

**Parimi:** në Shadow ruhet VETËM dija me **gjurmë algoritmike të plotë**. Çdo info
pa trace nuk është dije → **fshihet** (nuk persiston). "Sistemi nuk i beson vetes":
asnjë fakt s'pranohet sepse ekziston — pranohet vetëm pasi kalon zinxhirin e
verifikuesve dhe mban një prejardhje të riprodhueshme.

**Lineage (gjurma).** Para çdo shkrimi, `judge_supreme` ndërton një `Lineage` nga:
zinxhiri i verdikteve (`ShadowVerdict.verdict ∈ {0,1}` = Xᵢ(I)) + `proof_chain`
(prejardhja algoritmike nga Light) + një nënshkrim derivimi FNV. Ligji:

```
is_traceable() ⇔ derivation≠0 ∧ proof_len>0 ∧ (cap_sealed ∨ chain_count ≥ MIN_CHAIN=3)
```

Nëse FALSE → inputi fshihet (`lgc_law = 0x20 PURGED`, asnjë shkrim). Nëse TRUE →
dija shkruhet dhe gjurma regjistrohet në `LineageLedger` (verifikim i mëvonshëm i
prejardhjes). Ligji Zinxhir mbahet: **Primitive ⇔ ∏ᵢ Xᵢ(I) = 1** (`is_primitive_chain`);
përndryshe sistemohet sipas gjendjes (jo refuzim i parazgjedhur).

**Porta sovrane FFI (`sovereign_ffi_gate`).** Zbaton LAW_3 (vetëm Shadow shkruan
persistent) edhe përtej kufirit C, me **eliminim matematik** të klonimit:

- C sheh vetëm `CapHandle{gen, nonce}` (dy u64, `repr(C)`); `CapSlot{AtomicBool}`
  jeton i fshehur në heap Rust — C s'ka as tipin, as adresën.
- `LgcToken` është jo-klonueshëm (`PhantomData<*const ()>` → `!Send !Clone`) dhe
  KURRË nuk kalon FFI-në.
- Konsumi bëhet me `compare_exchange(true→false)` atomik: një kapacitet vlen
  **saktësisht një herë**. C mund të kopjojë numrat, jo `AtomicBool`-in → replay-i
  jep `CAS(false→false)` = refuzim hardware (`-1 AlreadyConsumed`).

Integrimi: vula atomike = **autorizim shkrimi** → ndërtohet gjurmë kapaciteti
(`proof = [gen, nonce]`), kontrollohet me ligjin e gjurmueshmërisë, regjistrohet, dhe
dija shkruhet. Sulm klonimi i detektuar → shënohet **VERIFIED_NEGATIVE** (kufi aktiv).

---

## 8. Porta Luvik + Primitive→Legacy ndër-domain + destfake (FAZA 4)

**Parimi:** *sistemi nuk i beson vetes.* Asnjë input nuk bëhet dije pa GJURMË
algoritmike; Quantum punon VETËM me dije të verifikuar e të gjurmueshme.

### 8.1 Luvik — porta e vetme e zbatimit (`luvik.rs`)

Luvik ka dy fytyra:

- **Porta e SHKRIMIT** — `Luvik::admit(&Lineage) -> Result<(), LuvikReject>`.
  judge_supreme e thërret PARA çdo shkrimi. Pa gjurmë → `Err` → destfake (purge).
  Vendimi vjen nga `is_traceable` (zero if/else mbi verdiktin).
- **Porta e LEXIMIT për Quantum** — `Luvik::verified_for_quantum(input_id) ->
  Option<VerifiedKnowledge>`. Kthen dije VETËM nëse mban një gjurmë të
  regjistruar; çdo "info" pa gjurmë → `None`. Quantum (PRO/NPRO/LIM) e thërret
  KËTË, kurrë arkivin drejtpërdrejt.

Integrimi me Quantum:

| Komponenti | Përdor | Garancia e Luvik |
|------------|--------|------------------|
| **PRO** (propozues) | `verified_for_quantum` si bazë propozimi | s'ndërton kurrë mbi info pa gjurmë |
| **NPRO** (anti-propozim) | `is_admissible_for_quantum` për të kundërshtuar | kundërshton vetëm dije reale |
| **LIM** (kufizues) | `VerifiedKnowledge.lineage` (masa/prejardhja) | kufiri llogaritet vetëm mbi të verifikuarën |

### 8.2 Gjurmueshmëria mbart historikun — `PrimitiveTrace`

Çdo herë që një input konfirmohet, regjistrohet një gjurmë: **ku** (domain),
**si** (path Fast/Standard/Deep), **kur** (timestamp), **me çfarë mase**, **me
çfarë vule** (0xA451). Përsëritja e të NJËJTËS përmbajtje (dedupe me
`content_hash = FNV(raw_bytes)`) në një domain TJETËR e shton gjurmën:

```
input → PRIMITIV → primitive_chain:
  trace_1 (science)   →
  trace_2 (industrial)→
  trace_3 (medical)   →
  trace_4 (security)  →
  trace_5 (research)  →  ⇒ ≥5 domain UNIKE + vulë konsistente ⇒ LEGACY
```

`PrimitiveEntry` u zgjerua me `content_hash`, `primitive_chain: Vec<PrimitiveTrace>`
dhe `legacy_ready`. Rregulla (te `knowledge_lineage.rs`, **zero if/else**):

```rust
pub fn cross_domain_legacy_ready(chain: &[PrimitiveTrace]) -> bool {
    let domains = unique_domains(chain);                       // HashSet i domain-eve
    let seal_consistent = chain.iter().all(|t| (t.lgc_seal & 0xFFFF) ^ 0xA5A5 == 500);
    (domains >= MIN_LEGACY_DOMAINS) & seal_consistent          // ≥5 & vulë e njëjtë
}
```

`MIN_LEGACY_DOMAINS = 5`. Promovimi vendoset nga përforcimi NDËR-DOMAIN — jo nga
një skor skalar. WAL-i (FILE_VERSION 2) mbart domain/path/masë/vulë te çdo ngjarje
Primitive, kështu kristalizimi riprodhohet BESNIKËRISHT në replay.

### 8.3 `shadow_destfake` — eliminimi i infos pa gjurmë

Politikë DETERMINISTE (jo probabilitet): arsye→veprim përmes tabele të prerë.

```
Purge   — hidhet menjëherë (default/); numërohet te ledger
Isolate — karantinë (numërohet; s'persiston)
Mark    — shënohet i pavërtetuar (numërohet)
```

`destfake().on_reject(LuvikReject)` zgjedh veprimin dhe e ekzekuton. Eliminim
matematik, jo gjykim heuristik.

### 8.4 Rrjedha e plotë (input → dije)

```
ingest(PassPackage, LightEnvelope)
  → ShadowPassage (dy origjinat bashkohen)
  → pipeline (9 noda; ZERO if/else në verdikt/vulë)
  → judge_supreme:
       lineage = derive_chain(verdicts, light.proof_chain)
       Luvik::admit(&lineage):
         Err → destfake().on_reject → PURGE (lgc_law 0x20)   [s'persiston]
         Ok  → write_{primitive|verified|negative} (zero-copy MOVE)
               write_primitive → PrimitiveTrace → dedupe content_hash
               try_promote_to_legacy → (≥5 domain ⇒ Legacy)
               ledger().record(lineage)
  → Quantum lexon VETËM përmes Luvik::verified_for_quantum (gjurmë ose asgjë)
```

---

## 9. Urat reale Quantum/Light → Shadow (FAZA 3)

Shadow s'varet nga crate-et e Quantum/Light. Kontratat e tyre pasqyrohen si tipa
kufitarë brenda Shadow-it dhe adaptohen → tipat e brendshëm (me ZHVENDOSJE, Ligji 0):

- **`QuantumInbound`** (pasqyrë e `ShadowPassPackage`) → `into_pass_package()` jep
  `PassPackage` (fushat propozuese; skorët ngujohen në [0,1], `suggested_verdict`
  në {0,1,2}). session/territory/vula NUK hyjnë këtu — ato vijnë via Light.
- **`LightInbound`** (pasqyrë e `LightShadowEnvelope`) → `into_envelope()` jep
  `LightEnvelope` (session/territory/`primitive_flags`/`proof_chain`/payload).
  `seal_ok()` kontrollon `(flags & 0xFFFF) ^ 0xA5A5 == 500` (diagnostikë, s'ndryshon).

Hyrjet:

```rust
// Dy kanale (rrjedha reale): Quantum + Light vijnë ndarazi.
shadow.ingest_bridged(q_inbound, l_inbound)?;

// Një burim Quantum → ndahet në (propozim, transport) si Quantum→Light→Shadow.
shadow.ingest_quantum(q_inbound)?;
```

`derive_light()` riprodhon transportin Light nga Quantum (`proof_chain[i] =
(score.to_bits() << 8) | i`), identik me `to_light_shadow_envelope`. Kështu
`proof_chain`-i mbërrin jo-bosh → envelope-i kalon **ligjin e gjurmueshmërisë** (§7).

**RingBuffer `repr(C)` SPSC (zero-copy në kufirin FFI).** `ShadowRing` lejon një
PRODHUES (C, p.sh. Light Hydrator) + një KONSUMATOR (Shadow) të shkëmbejnë mesazhe
pa alokim për mesazh dhe pa `Vec` të ndërmjetëm. Layout repr(C) i ndashëm me C
(`head:u32, tail:u32, slots[RING_CAP]`); prodhuesi boton `tail` me **Release**,
konsumatori e lexon me **Acquire** → happens-before mbi slot-in, i cili lexohet
NË VEND (`consume_with(|flags, seq, &[u8]|)`). Pa mutex (disiplinë SPSC).

### 9.1 Rruga e KTHIMIT Shadow → Quantum/Light (`bridge/shadow_out.rs`)

Shadow vendos; përgjigja shndërrohet në formën që Quantum/Light presin:

- **`SupremeOutcome`** (3-gjendjesh) = MIRROR i `ShadowVerdictMirror` (Quantum):
  `PrimitiveKnowledge` (1) / `VerifiableNotPrimitive` (0) / `ChainRejected` (0).
  `from_verdict` klasifikon BRANCHLESS nga `SupremeVerdict` (bit 0x20 = purge).
- **`ShadowLightResponse`** = MIRROR i `ShadowResponseMirror` (Quantum) /
  `ShadowLightResponse` (kontrata): `{session_id, verdict, shadow_note, legacy_score}`.
- **`LightShadowBridge::receive_from_light(pkg, light)`** — hyrja e kthimit (emri që
  Quantum referon në OPSIONIN A). Ruan ligjin suprem të DY origjinave (pa humbje).
  `DefaultLightShadowBridge::new(&shadow)` jep adaptorin e emërtuar.
- Gateway: `respond_bridged` / `respond_to_quantum` (variante fallible).
- DISTRUST: çdo dështim i brendshëm → `ChainRejected` (s'pranohet me gabim).

### 9.2 Verifikim ndër-platformë (kontratat e mbyllura)

Çdo kontratë u VERIFIKUA kundrejt burimit real (Light + Quantum); puna vetëm në Shadow:

| Komunikimi | Kontrata e burimit | Tipi në Shadow | Gjendja |
|------------|--------------------|----------------|---------|
| Quantum → Shadow | `ShadowPassPackage` (17 fusha) | `QuantumInbound` | përputhje 1:1 ✓ |
| Light → Shadow | `LightShadowEnvelope` (6 fusha) | `LightInbound` | përputhje 1:1 ✓ |
| proof_chain | `to_light_shadow_envelope` `(bits<<8)\|i` | `derive_light` | identik ✓ |
| Vula 500 | `XOR 0xA5A5 & 0xFFFF == 500` (Q+L) | kernel `SB/SGL_SEAL_*` | identik ✓ |
| Vula e maskuar | `SEAL_PRIMITIVE_MASKED = 0xA451` | `0xA451` | identik ✓ |
| ABI `LgcRequest/Result` | 200 / 264 bytes | kernel repr(C) | identik ✓ |
| `BussLegacyMsg.lgc_sealed` | `[u8; 512]` | `BL_LGC_SEALED_LEN 512` | identik ✓ |
| Shadow → Quantum | `ShadowResponseMirror` (4 fusha) | `ShadowLightResponse` | përputhje 1:1 ✓ |
| Verdikti i kthimit | `ShadowVerdictMirror` (3) | `SupremeOutcome` | përputhje 1:1 ✓ |

**FFI (call/callback):** Shadow THËRRET 6 simbole te kerneli (`lgc_init`, `lgc_check`,
`lgc_freeze`, `lgc_unfreeze`, `lgc_get_stats`, `shadow_lgc_seal_check`) — kerneli i
eksporton EKZAKT. Shadow EKSPOZON 5 `no_mangle` (`vault_write_{primitive,verified,
negative}`, `sovereign_{issue_capability,validate_and_write}`). Callback-et e arkivit:
`VaultBackend` (`on_primitive/verified/negative/confirm`), implementuar nga `DiskBackend`.

---

## 10. Kujtesat sovrane nga Light (APUPK + SNB)

Light PËRGATIT dy lloje të reja njohurish dhe ia dorëzon Shadow-it.
Parimi mbahet: **Light përgatit dhe orienton — Shadow VENDOS dhe RUAN.** Kontratat
u verifikuan kundrejt burimit (`apupk_coordinator.rs`, `snb_algorithm.rs`).

### 10.1 APUPK — `shadow_APUPK_memory` (`shadow_apupk.rs`)

*Awaken Project User Personal Knowledge.* Light dërgon `ShadowApupkPackage`
(pasqyruar si `ApupkInbound{trace, initial_progress, project_content}`). Shadow:

- **Vendos (ZERO if/else):** pranon vetëm me gjurmë + përmbajtje —
  `(trace_id≠0 & initial_trace jo-bosh) & (content jo-bosh)`. Ndryshe →
  `ApupkReject::{NoTrace, EmptyContent}` (s'bëhet njohuri).
- **Ruan (Ligji 0):** përmbajtja/shënimet ZHVENDOSEN në `ApupkEntry`, pa klon.
- **Dedupe me `project_id`:** ringarkim → rifreskon (`revisions++`).
- Përditësim progresi: `update_progress(project_id, pct, notes, ts)`.

Hyrja: `Shadow::receive_apupk(pkg) -> Result<u64, ApupkReject>` (trace_id).
Kujtesa globale: `apupk_memory()`.

### 10.2 SNB — `shadow_snb` (`shadow_snb.rs`)

*Shadow Negative-Bug memory.* Light raporton vetëm kur ka bug
(`prepare_for_shadow_snb`); pasqyruar si `BugInbound{timestamp_ns, module_name,
description, flow_trace, severity}`. Shadow:

- **Vendos (ZERO if/else):** pranon vetëm me gjurmë rrjedhe + përshkrim —
  `(flow_trace jo-bosh) & (description jo-bosh)`. Ndryshe →
  `SnbReject::{NoFlowTrace, EmptyDescription}`.
- **Klasifikon ashpërsinë (branchless):** `SnbSeverity::classify` numëron kufijtë
  sovranë (1/5/9) → `Low/Medium/High/Critical`. Kufijtë i vendos SHADOW.
- **Ruan (Ligji 0):** përshkrimi/gjurma ZHVENDOSEN në `BugEntry`.
- **Dedupe me (module + description):** i njëjti bug → `frequency++`.

Hyrja: `Shadow::receive_bug_report(report) -> Result<SnbReceipt, SnbReject>`
(`SnbReceipt{level, frequency}`). Kujtesa globale: `snb_store()`.

### 10.3 Durabiliteti WAL sovran (`sovereign_log.rs`)

Të dyja kujtesat janë **durabël** përmes një WAL-i gjenerik sovran — përgjithësim
besnik i `vault_disk`: header `[u64 MAGIC][u32 VERSION]`, rekord `[u32 LEN][u32
CRC][PAYLOAD]`, CRC32 IEEE, fsync per-rekord, bisht i dëmtuar i shkurtuar në hapje.
Çdo kujtesë ka MAGIC të vetin (skedar i huaj → refuzohet).

- **Log-first:** `store`/`report` e shkruajnë ngjarjen në log PARA aplikimit në RAM.
- **Ripërsëritje besnike:** `apply(…, stored_at/recorded_at)` riprodhon dedupe +
  `revisions`/`frequency` + nivel ekzakt nga log-u (timestamp-et logohen, jo rigjenerohen).
- Nisja: `init_apupk_disk(path)` / `init_snb_disk(path)` (PARA aksesit të parë global);
  ose `ShadowApupkMemory::with_disk(path)` / `ShadowSnb::with_disk(path)` lokalisht.

> Tani të dyja janë sovrane **dhe persistente** (Shadow i vetmi shkrues), me
> gjurmueshmëri PARA ruajtjes. RAM mbetet burimi i leximit; disku durabiliteti.

---

## 11. Ndërtimi & testimi

```bash
# Ndërtim i plotë (Rust + kerneli C përmes build.rs)
cargo build --release

# Testet (invariantë sovranë + maturim + dedup negativ + round-trip në disk)
cargo test

# Bërthama PA shtresën C (pasqyrë Rust e kernelit; build.rs anashkalon C-në)
cargo test --features pure_rust

# Demonstrimi i plotë: Quantum + Light → Shadow → vault → Legacy
cargo run --example full_flow
cargo run --example full_flow --features pure_rust

# VERIFIKIM RUNTIME i kernelit C (standalone, jo përmes cargo) — 27 prova
gcc -std=c11 -Wall -Wextra -O3 -Ikernel verify_kernel.c \
    kernel/shadow_buss.c kernel/buss_legacy.c kernel/shadow_gj_legacy.c \
    -lpthread -o verify_kernel && ./verify_kernel
```

**Kërkesa:** Rust (edition 2021) + një kompilator C (gcc/clang). Zero varësi të
jashtme Rust; një varësi build-time (`cc`).

### Shënim mbi kernelin

`build.rs` kompilon **tre** skedarë C: `shadow_buss.c`, `buss_legacy.c`,
`shadow_gj_legacy.c` (me `-std=c11 -Wall -Wextra -O3`).

Vendimi suprem i kernelit (vula 500, ngrirja, statistikat, siguria NULL) është i
verifikuar **në runtime** nga `verify_kernel.c` (27 prova; kerneli ekzekutohet real
dhe konfirmohet se `0xA451` → PASS, `0x0000`/i ngrirë → BLOCK). Kjo dëshmon edhe
përputhjen ndër-modul: `0xA451` është pikërisht vula që prodhon Light dhe që
Quantum e mbart në `primitive_flags`.

> `kernel/shadow_gj_legacy_kernel.c` është një **gjyqtar i dytë** (eksporton
> `shadow_gj_legacy_judge` dhe bën vetë shkrimet në vault përmes callback-eve
> `vault_write_*`). Ai **QËNDRON JASHTË** build-it për arsye arkitekturore: do
> krijonte një **rrugë të dytë, paralele shkrimi** në arkiv (dy autoritete), gjë
> që cenon parimin e një autoriteti të vetëm suprem. (S'ka përplasje simbolesh me
> të 3 skedarët e build-it — thjesht përjashtohet me qëllim.)

---

## 12. Struktura

```
shadow_v05/
├── Cargo.toml            # · features: default / pure_rust
├── build.rs              # kompilon 3 skedarët C (anashkalon në pure_rust)
├── src/
│   ├── lib.rs            # API publike
│   ├── types.rs          # PassPackage, LightEnvelope (TË NDARA), ShadowPassage
│   ├── knowledge_vault.rs# 7 dyqane + maturim Primitive→Legacy
│   ├── vault_disk.rs     # PERSISTENCA: WAL append-only + ripërsëritje (FAZA 2)
│   ├── knowledge_lineage.rs # LIGJI I GJURMUESHMËRISË (trace-ose-fshi) + ledger
│   ├── sovereign_ffi_gate.rs # PORTA SOVRANE FFI (kapacitet 1×, anti-klonim)
│   ├── luvik.rs          # PORTA SOVRANE E RREPTË (admit + Quantum read-gate)
│   ├── shadow_destfake.rs # ELIMINIMI i infos pa gjurmë (purge/izolim/shënim)
│   ├── shadow_apupk.rs   # shadow_APUPK_memory (njohuri projekti — Light)
│   ├── shadow_snb.rs     # shadow_snb (raporte bug-u — Light)
│   ├── sovereign_log.rs  # WAL gjenerik sovran (durabiliteti i APUPK/SNB)
│   ├── bridge/           # URAT REALE (FAZA 3)
│   │   ├── quantum_in.rs #   ShadowPassPackage → PassPackage
│   │   ├── light_in.rs   #   LightShadowEnvelope → LightEnvelope
│   │   └── shadow_out.rs #   Shadow → Quantum/Light (përgjigja MIRROR)
│   ├── ffi_ring.rs       # RING BUFFER repr(C) SPSC (zero-copy FFI)
│   ├── shadow_router.rs  # zgjedh rrugën
│   ├── shadow_matrix.rs  # sistematizon (konsulton vault)
│   ├── shadow_gen5.rs    # strukturë vektoriale
│   ├── shadow_type.rs    # gjendje paketimi
│   ├── shadow_temporal.rs# validitet kohor (deep)
│   ├── shadow_sovereign.rs# 5 ligjet (score)
│   ├── shadow_emergence.rs# tranzicion (deep)
│   ├── shadow_consensus.rs# konsensus (deep)
│   ├── shadow_judiciary.rs# gjykata epistemike (0/1 final)
│   ├── sovereign_guard.rs # enforce_sovereign_laws → Result
│   ├── shadow_gj_legacy.rs# AUTORITETI SUPREM (FFI C + pasqyrë pure_rust)
│   ├── shadow_pipeline.rs # ORKESTRATORI (bashkon dy origjinat)
│   └── shadow_gateway.rs  # HYRJA: Shadow::ingest
├── kernel/               # C: shadow_buss, buss_legacy, shadow_gj_legacy (+ headers)
├── tests/integration.rs  # invariantë deterministë (API publik)
├── examples/full_flow.rs # demonstrim end-to-end
└── verify_kernel.c       # VERIFIKIM RUNTIME i kernelit C (standalone, 27 prova)
```

---

*Zero if/else në rrugët e verdiktit · Zero ML · Zero-copy sovereign.*
*Gjata Legacy™ — Bledar Gjata.*
