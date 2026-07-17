# ESS-MAI v1.6.3 — Auditimi i thellë i GCL Scientific Project Continuum

## Mandati

Ky audit ndjek projektin shkencor të përdoruesit nga lindja e identitetit në Light deri te verdikti suprem në Shadow dhe kthimi përmes PD/iZ. Qëllimi nuk ishte të krijohej një autoritet i katërt ose një rrjedhë paralele, por të lidhej projekti me të njëjtën kushtetutë GCL, të njëjtin Untrust Start to End, të njëjtin Shadow main mediation dhe të njëjtën Living Trust.

Baseline: `v1.6.2_ess_mai`.
Target: `v1.6.3_ess_mai`.

## Gjetjet që nisën versionin

### E0425 — Digital Lab midis dev_harness dhe runtime-it

`run_lab_demo()` ishte i kufizuar me `dev_harness`, por `run_integrated_lab_demo()` kompilohej edhe në target-in default dhe thërriste `persist_negative()`, i cili ekzistonte vetëm me `dev_harness`. Cargo e kapi si E0425.

Kjo nuk ishte vetëm një mospërputhje `cfg`: Digital Lab mban Governance, Raw Cognitive Trace, TRL evidence dhe integrimin e projektit shkencor. v1.6.3 ndan qartë:

- demonstrimin hard-coded: vetëm `dev_harness`;
- procedimin real shkencor: `LabSystemBridge::run_integrated` brenda rrjedhës së projektit real Quantum;
- persistimin negativ production: vetëm brenda full Shadow cycle pas verdict-it suprem.

### E0063 — kontrata Untrust nuk ishte përhapur te fixture-i Shadow

`PassPackage` kishte marrë `quantum_action_mask` dhe `quantum_required_action_mask`, por fixture-i historik i testit Shadow inicializonte vetëm `quantum_action_state`. v1.6.3 e mbyll me maskën kushtetuese `REQUIRED_ACTION_MASK`, jo me zero/default. Fixture-i deklarohet qartë si inner-Shadow test dhe `scientific_project: None`.

### Projekti pozitiv ndalej në Quantum; negative demo shkonte direkt te magazina

Rruga e vjetër Digital Lab printonte evidencën pozitive si material për PIM, ndërsa negative demo mund të thërriste `--negative-once`. Kjo ishte asimetri. v1.6.3 e vendos të gjithë projektin në `FinalEvidenceWire`; Shadow verifikon dhe vetëm statusi rigoroz negativ lejon shkrimin në Negative Knowledge.

### Novel kishte organe, por jo një kontratë të vetme

Light kishte APUPK; Quantum kishte Digital Lab/TRL/SRK/PIM/NPIM; Shadow kishte APUPK memory, GeniusNovel, ShadowEco dhe judge_supreme. Mungonte lidhja tipizuese që provonte se:

1. projekti që lindi në Light;
2. projekti i ruajtur në Shadow;
3. hipoteza që Quantum procedoi;
4. evidenca që Shadow gjykoi;
5. rezultati që hyri në Living Trust;

ishin i njëjti objekt sovran.

## Harta e mbyllur

```text
USER PROJECT
  ↓
LIGHT / APUPK
  project_id + user_id + title + content
  trace_id = fold(project_id + user_id, title)
  input SHA-256 + Vula 500
  ↓ process: --project-register-once
SHADOW MAIN / APUPK
  validate identity/owner/title/trace/V500/progress
  WAL write + flush + fsync
  ↓
ProjectContextWitness
  project/user/trace/revision/contentSHA/V500/contextSHA
  ↓ LIGHT revalidation
LIGHT → QUANTUM MAIN
  process-bound payload + request SHA-256
  ↓
QUANTUM
  Digital Lab + Governance + Raw Cognitive Trace
  TRL evidence + SRK + PIM/NPIM/MPRO
  nine-organ Untrust ledger/mask/state
  project evidence SHA-256
  ↓ FinalEvidenceWire
SHADOW MAIN
  wire/identity/APUPK/revision/content/title/V500 checks
  replay Untrust + module cross-binding
  file magic verification
  same judge_supreme
  GeniusNovel + ShadowEco factualization
  ↓
Novel Factual | Hold | Rigorous Negative
  ↓
project verdict SHA-256
  + Living Trust SHA-256
  + VerificationReceipt
  ↓
QUANTUM PD → output + iZ → next i0
  ↓
PD LIGHT → Nura ∥ emotional UI → new UI
```

## Kontratat e reja

### `gcl_project_contract.rs` ×3

Byte-identik në Light, Quantum dhe Shadow. Kanonizon:

- formulën e `project_trace_id` të APUPK;
- Vula 500;
- SHA-256 e kontekstit APUPK;
- SHA-256 e evidencës shkencore;
- SHA-256 e statusit/verdiktit të projektit;
- kodimin e skedarëve për kufirin Light→Quantum;
- indekset `u64` vetëm për kompatibilitet/diagnostikë.

### `shadow-contracts` protocol v8

Shton:

- project registration request/response;
- Light project intake request/response;
- Quantum project execution request/response;
- `ScientificProjectWire` brenda `FinalEvidenceWire`;
- full project SHA-256 në `ShadowVerdictWire`;
- statuset NONE, UNDER_GCL, HOLD, RIGOROUS_NEGATIVE, NOVEL_FACTUAL;
- kodet e refuzimit Novel.

Quantum vazhdon të varet vetëm nga `shadow_contracts`; nuk linkon Shadow core.

## Lidhja kriptografike

```text
P_L = Light(project_id,user_id,title,content,V500)
C_S = SHA256(project_id || user_id || trace_id || revision || title || contentSHA || V500)
E_Q = SHA256(C_S || title || domain || hypothesis || assumptions || GCL process || TRL || findings || docs || files)
V_S = SHA256(project_id || status || C_S || E_Q || Novel fields)
T   = SHA256(action convergence || supreme verdict || laws || E_Q || V_S || L500)
```

Living Trust përdor SHA-256 të plota 32-byte për evidencën dhe verdict-in e projektit. Vlerat `u64` mbeten vetëm indekse compatibility dhe nuk janë baza e Besimit.

## Light

- `--project-route-once` është entrypoint real i projektit.
- APUPK prodhon identitetin dhe Vula 500.
- Shadow main regjistron projektin durable dhe jep witness.
- Light riverifikon project/user/trace/flags/context SHA dhe content SHA.
- Projekti i madh nuk përdor bus-in legacy 2048-byte; Light nis procesin Quantum me kontratë të versionuar.
- Përgjigjja Quantum lidhet me SHA-256 e frame-it të kërkesës.

Light nuk jep verdict Novel.

## Quantum

- `--project-process-once` konsumon payload-in real të Light-it.
- Rillogarit payload SHA dhe lidh request me APUPK witness.
- Digital Lab nuk është më vetëm skenë hard-coded: procedon titullin, domain-in, inputin, hipotezën dhe supozimet reale.
- Projekti kalon të njëjtën rrjedhë HPRO→PRO→NPRO→NPIM→SRK→PIM→APRO→MPRO→HCP.
- Evidenca shkencore hyn në të njëjtën `FinalEvidenceWire`.
- Quantum nuk hyn në APUPK/Knowledge/Negative stores.
- Quantum rillogarit Living Trust dhe refuzon mospërputhjen me Shadow.

## Shadow main dhe core

`shadow/main.rs` mbetet porta e detyrueshme. `lib.rs` është core i përfshirë vetëm brenda binarit; `process_bridge.rs` është dekoderi/validuesi i kufirit.

Shadow:

1. rillogarit paketën finale;
2. rillogarit input SHA;
3. rillogarit project context/evidence SHA;
4. kontrollon projektin kundrejt APUPK durable;
5. riluaj action ledger dhe maskën;
6. kryqëzon HPRO/PRO/NPRO/NPIM/SRK/PIM/APRO/MPRO/HCP;
7. verifikon magic bytes të skedarëve;
8. thërret `classify_with_factualization` vetëm për çiftin sovran (1,1);
9. prodhon statusin brenda të njëjtit `SupremeVerdict`;
10. lidh statusin me Living Trust dhe Receipt.

## APUPK hardening

- WAL version 2 dhe skedar i ri `shadow_apupk_v163.wal`.
- Titulli dhe Light Vula 500 ruhen në WAL.
- Pronësia e `project_id`, formula e trace-it, titulli, progresi finite dhe Vula 500 kontrollohen para WAL.
- Witness jepet vetëm pas `write + flush + fsync`.
- Dry ndër-proces me `create_new` ndalon dy one-shot Shadow writers.
- Projekti krahasohet me APUPK për project/user/trace/revision/title/flags/content SHA përpara verdict-it.

## Negative Knowledge

Standalone `--negative-once` është vetëm `dev_harness`. Në production:

- status pozitiv/hold/Novel nuk shkruan Negative Knowledge;
- (Y,X)=(0,0) ose `RIGOROUS_NEGATIVE` kërkon persistim;
- Quantum nuk eksporton PD/iZ negativ kur persistimi i detyrueshëm dështon.

## PD Light

Handoff-i v1.6.3 ka 45 fusha trupi + CRC. Ai mban:

- full project evidence SHA;
- project ID;
- full context SHA;
- status;
- factualization/TRL/proof/rejection;
- Living Trust të plotë.

Light pret 46 fusha të vulosura dhe rillogarit project verdict SHA + Living Trust para Nura/UI.

## Kufijtë e ndalur me vetëdije

- Nuk u shpik HMAC/enkriptim për process wire: nuk ka kontratë sovrane çelësash, rotacioni ose revokimi. Frame-i ekzistues përdor checksum; materialet kritike riverifikohen me SHA-256.
- APUPK WAL ka CRC, fsync dhe lock, por jo MAC/enkriptim pa key authority.
- WAL v1.6.2 nuk migrohet automatikisht; v1.6.3 përdor skedar të ri dhe kërkon riregjistrim.
- Dryni stale pas crash-it kërkon operator; nuk u shpik recovery pa attestation.
- Final Novel status udhëton në SupremeVerdict/LivingTrust/PD. Nuk u shpik një APUPK status-event WAL pa kontratë të autorizuar.
- One-shot Light intake konfirmon regjistrim+procedim; output-i final konsumohet nga rrjedha standarde PD Light/Nura.
- Cargo-green mbetet pending deri te `VALIDATE_V163.ps1` në Windows GNU.

## Verifikimi lokal

- 277 skedarë Rust kaluan skanimin strukturor të delimiterëve/strings/comments.
- 7 Cargo manifests dhe 3 JSON u parsuan.
- 18 objekte C u kompiluan: Light, Quantum HW dhe Shadow me GCC+Clang.
- 0 warning/error C me flags e build scripts.
- Kontratat Project, Living Trust, PD Continuum dhe PD Spine janë byte-identike ×3.

Ky audit nuk e quan Rust Cargo-green pa kompajlluesin Rust.
