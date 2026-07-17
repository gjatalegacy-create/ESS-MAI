# ESS-MAI v1.6.4 — Changelog

## Qëllimi

v1.6.4 mbyll kufijtë e paqartë të v1.6.3 rreth magazinës së projektit të përdoruesit, UI-së së vjetër, TRL-së dhe verifikimit shumëhapësh në Shadow. Ndryshimi nuk krijon rrjedhë paralele: projekti vazhdon vetëm nën të njëjtin GCL process, Spine 9, receipt dhe Living Trust.

## UI e vjetër — rol minimal

- U hoqën komandat placeholder `explore_input`, `get_output` dhe `upload_knowledge_dialog`.
- UI-ja e vjetër ekspozon vetëm:
  - `upload_project` — pranon materialin e përdoruesit dhe ia dorëzon Light-it;
  - `reflect_system_emotion` — pasqyron gjendjen reale si komandë emocionale.
- UI-ja nuk prodhon më `ready_for_shadow`, verdict, TRL, kontratë GCL ose Vulë 500.
- UI-ja nuk nis Shadow dhe nuk nis Quantum.
- Wire-i i intake-it nuk përmban më `project_id`, `user_id`, timestamp, `contract_id` ose `lgc_seal`; identiteti dhe autoriteti lindin vetëm në Light.

## Light — pronar i koordinimit

- Light mbetet porta e vetme e projektit përmes `--project-route-once`.
- Light krijon `user_id`, `project_id` dhe timestamp-in e intake-it; UI dërgon vetëm material.
- Light krijon identitetin kushtetues `GCL:SCIENTIFIC_PROJECT:V164` dhe Vulën e transportuar nga witness-i APUPK.
- Light regjistron projektin te Shadow main, riverifikon `content_sha256`, pastaj thërret Quantum me witness-in e njëjtë.
- U korrigjua schema 45-fushëshe e PD handoff-it: SHA-256 transportohet si tekst kanonik, ndërsa `project_id` si `{:016x}`.

## TRL brenda GCL

- `QUANTUM_MAX_TRL = 3`: Quantum/Digital Lab prodhon vetëm evidencë TRL 0–3.
- `SHADOW_FACTUAL_TRL = 4`: TRL4 lind vetëm në Shadow.
- Wire-i refuzon një projekt hyrës që pretendon TRL4.
- TRL nuk është autoritet më vete dhe nuk prodhon verdict sovran.
- TRL është evidencë e matur brenda të njëjtit GCL process dhe përdoret nga Shadow në faza të ndara.

## Shadow — verifikim shumëhapësh

Shadow tani e shpreh qartë rendin:

1. lidhja e identitetit të projektit me GCL/Spine dhe Vulën Light;
2. verifikimi i SHA-256 dhe kufijve TRL/mass;
3. verifikimi i llojit real të skedarëve me magic bytes;
4. ShadowLab verifikon mbështetjen TRL;
5. vetëm pas çiftit sovran `(Y=1, X=1)`, GeniusNovel mund të faktualizojë TRL4;
6. rezultati lidhet me SupremeVerdict, Living Trust, receipt, iZ dhe next i₀.

- U shtua porta `verify_project_gcl_stage` në `shadow_gj_legacy.rs`.
- U hoq importi i papërdorur `TrlVerdict` nga `shadow_eco.rs`.
- Dy fixture-at e vjetër të Shadow deklarojnë tani qartë `scientific_project: None`.

## Kontratat dhe versionimi

- `shadow-contracts` kaloi në `PROTOCOL_VERSION = 9`.
- Domain-et SHA-256 të Project Continuum dhe Living Trust kaluan nga V163 në V164 në Light, Quantum dhe Shadow, byte-identikisht.
- Manifestet dhe konfigurimet UI kaluan në `1.6.4`.

## Gjendja e verifikimit

- Auditimi statik dhe kontrolli i identitetit të kontratave kryhen brenda paketës.
- Ky ambient nuk kishte `cargo`/`rustc`; prandaj Cargo-green nuk pretendohet.
- `VALIDATE_V164.ps1` është release gate për Windows GNU / Rust 1.96.0.
