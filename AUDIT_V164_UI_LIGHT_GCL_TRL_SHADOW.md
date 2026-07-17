# Audit v1.6.4 — UI → Light → GCL/TRL → Shadow

## Kufiri i UI-së së vjetër

UI-ja e vjetër është sipërfaqe hyrëse dhe emocionale, jo organ procedues.

```text
User project material
        ↓
Old UI: upload_project
        ↓
Light --project-route-once
```

UI-ja nuk ka thirrje direkte drejt `shadow_platform`, `quantum-platform`, `--project-register-once` ose `--project-process-once`. Ajo nuk prodhon as `project_id`, `user_id`, timestamp, kontratë ose vulë.

## Rrjedha e vetme e projektit

```text
Old UI upload
↓
Light validates bounded material
↓
Light APUPK + Vula 500
↓
Shadow main --project-register-once
↓
ProjectContextWitness
↓
Light riverifies content SHA-256
↓
Quantum main --project-process-once
↓
PD / GCL / Spine 9 / Digital Lab (TRL ≤ 3)
↓
Shadow main final cycle
↓
GCL identity gate
↓
magic-byte evidence gate
↓
ShadowLab TRL verification
↓
(Y=1, X=1) + Novel proof
↓
TRL4 factualization, ose HOLD/NEGATIVE
↓
SupremeVerdict → Living Trust → Receipt → PD Light
```

## Ndarja e TRL-së

- Quantum: mat dhe propozon evidencë deri TRL3.
- ShadowLab: riverifikon nëse TRL e mbështet rezultatin.
- GeniusNovel: nuk arsyeton; verifikon dokumentacionin real.
- Shadow supreme: vetëm këtu mund të lindë TRL4.
- Light/UI: vetëm transportojnë/pasqyrojnë rezultatin; nuk ndryshojnë nivelin.

## Bllokuesit e v1.6.3 të mbyllur

### LowerHex

Në Quantum dhe testin PD Light, fusha tekstuale SHA-256 ishte përballë `{:016x}`. Në v1.6.4 rendi është:

```text
project_evidence_sha256: {}
project_id:               {:016x}
project_context_sha256:   {}
```

### Shadow fixtures

Dy konstruktorë testimi të `QuantumInbound` kishin mbetur pa fushën e re. Tani deklarojnë `scientific_project: None`, pa falsifikuar një projekt.

## Gjëra që nuk u bënë

- UI nuk mori autoritet GCL.
- Light nuk mori autoritet verdict-i.
- Quantum nuk prodhon TRL4.
- Shadow core nuk u linkua te Quantum.
- Nuk u krijua rrjedhë Novel paralele me GCL.
- Nuk u shtua kod i fryrë ose orkestrator i ri për TRL.
