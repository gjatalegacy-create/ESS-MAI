# Audit v1.6.5 — Quantum Project Workspace dhe ruajtja e tokenëve

## Kufiri i ri

```text
Old UI upload
  → Light --project-route-once
  → APUPK registration in Shadow main
  → ProjectContextWitness
  → Quantum --project-workspace-once
  → route + SHA-256 record identities
  → storage/conversation owner
```

Quantum nuk bëhet memory store. Ai vetëm e orienton pjesën `scientific_project`; inputet normale vazhdojnë në rrugën ekzistuese stdin/runtime.

## Ndarja nga procedimi shkencor

```text
WORKSPACE:
--project-workspace-once
  ├─ validate request/APUPK identity
  ├─ classify storage/chat/both
  ├─ hash workspace/material/turn
  └─ return authority=NONE

LEGACY SCIENCE:
--project-process-once
  ├─ run(...)
  ├─ PD/Spine 9/Layers
  ├─ TRL 0–3
  ├─ Shadow multi-stage verification
  └─ receipt/Living Trust/iZ/next i₀
```

## Token audit

Porta Workspace nuk referon:

```text
LgcToken
LgcGate
CapHandle
SovereignGate
token_forge
ForgeToken
mint(...)
SEAL_PD / SEAL_* capability flow
```

Ajo prodhon vetëm SHA-256 domain-separated. SHA-të quhen `workspace_sha256`, `material_sha256` dhe `conversation_turn_sha256`; nuk quhen dhe nuk përdoren si token.

## GCL audit

- APUPK dhe Vula 500 vazhdojnë të krijohen në Light para ndarjes së rrugës.
- `GCL:SCIENTIFIC_PROJECT:V164` mbetet i pandryshuar.
- GCL Project, Living Trust dhe VerificationReceipt nuk u versionuan, sepse porta Workspace nuk ndryshon provën kushtetuese.
- Rruga legacy mbetet e aksesueshme me flag eksplicit.

## Shadow audit

Shadow nuk u modifikua. Projekti regjistrohet në APUPK para se Quantum të orientojë workspace-in. Kështu magazina persistente mbetet në Shadow dhe Quantum nuk krijon memory paralele.

## Rreziqet e mbyllura

1. **Project route accidentally entering full reasoning** — porta Workspace nuk thërret `run`.
2. **Normal user input diverted to project storage** — porta kërkon flag të veçantë dhe `ScientificProjectInput`.
3. **Token mutation** — skedarët token-kritikë janë byte-identikë me v1.6.4.
4. **Legacy regression** — flag-et e vjetra ruhen dhe përdorin të njëjtin APUPK helper.
5. **Wire drift** — Light riverifikon çdo field të përgjigjes Workspace dhe të tre SHA-256.
