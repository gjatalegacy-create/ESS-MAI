# ESS-MAI v1.6.5 — Changelog

## Qëllimi

v1.6.5 ndan rrugën e projektit në dy porta të qarta pa ndryshuar GCL, Living Trust, receipt-et ose capability tokenët:

1. **Project Workspace** — rruga default për magazinim dhe bisedë rreth projektit;
2. **Legacy Scientific Process** — rruga ekzistuese v1.6.4 për procedim të plotë GCL/PD/TRL/Shadow.

## Quantum Project Workspace

U shtua `quantum/src/project_workspace_router.rs` dhe porta:

```text
quantum-platform --project-workspace-once REQUEST RESPONSE
```

Kjo portë:

- pranon vetëm request me `ScientificProjectInput` dhe APUPK witness të vlefshëm;
- verifikon `project_id`, `trace_id`, `context_sha256`, `payload_sha256` dhe request SHA-256;
- orienton vetëm në `PROJECT_STORAGE`, `PROJECT_CONVERSATION` ose `PROJECT_STORAGE_AND_CONVERSATION`;
- prodhon identitete SHA-256 për workspace, material dhe turn bisede;
- nuk ruan vetë materialin, sepse Quantum nuk është magazinë persistente;
- nuk thërret `run`, PD, Spine 9, TRL ose Shadow supreme;
- nuk hap, nuk mint-on dhe nuk transporton LGC/Forge/capability token.

## Rruga legacy

Portat ekzistuese ruhen:

```text
light-platform   --project-route-legacy-once REQUEST RESPONSE
quantum-platform --project-process-once REQUEST RESPONSE
```

`--project-process-once` mbetet rruga e vjetër e procedimit shkencor. Kodi i saj nuk u ndryshua.

## Light

- `--project-route-once` është tani porta default e Project Workspace.
- Light regjistron projektin APUPK te Shadow main dhe riverifikon witness-in përpara Quantum.
- Rruga e re dhe ajo legacy ndajnë të njëjtin helper kushtetues për APUPK, SHA-256, `GCL:SCIENTIFIC_PROJECT:V164` dhe Vulën 500.
- Light verifikon përgjigjen Workspace: versionin, projektin, trace-in, SHA-256, `authority=NONE`, `token_policy=UNCHANGED` dhe deklarimin e rrugës legacy.

## Shadow

Kodi Shadow nuk u ndryshua. Shadow vazhdon të jetë:

- magazina persistente APUPK;
- pronari i verifikimit shumëhapësh;
- i vetmi vend ku mund të lindë TRL4 dhe verdict-i suprem në rrugën legacy.

## Tokenët dhe domain-et

Nuk u ndryshuan:

- `quantum/src/sovereign/lgc_gate.rs`;
- `quantum/src/token_forge.rs`;
- modulet PD seal/token;
- verification receipt;
- GCL Project domains V164;
- Living Trust domains V164;
- `shadow-contracts::PROTOCOL_VERSION = 9`.

v1.6.5 shton vetëm një domain të ri jo-autoritar:

```text
ESS_MAI_QUANTUM_PROJECT_WORKSPACE_V165
```

Ky domain prodhon identitete rekordi, jo token ose verdict.

## Gjendja e verifikimit

- Parsim sintaksor Tree-sitter: të gjithë skedarët Rust, 0 nyje `ERROR`.
- Guard-et statike: rruga e re nuk importon token/GCL gate dhe nuk thërret pipeline-in `run`.
- Hash-et e skedarëve token-kritikë ruhen identike me v1.6.4.
- Cargo-green mbetet për t’u provuar me `VALIDATE_V165.ps1` në Windows GNU / Rust 1.96.0.
