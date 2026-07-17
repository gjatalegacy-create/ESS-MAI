# ESS-MAI v1.6.5 — Simulation Map

## S1 — Upload default

```text
UI upload → Light --project-route-once
→ Shadow APUPK witness
→ Quantum --project-workspace-once
→ PROJECT_STORAGE_AND_CONVERSATION
```

Pritshmëri: asnjë PD/TRL/verdict; `authority=NONE`, `token_policy=UNCHANGED`.

## S2 — Project chat

```text
domain=project-chat + ScientificProjectInput
→ PROJECT_CONVERSATION
```

Pritshmëri: `conversation_turn_sha256` lidhet me workspace dhe tekstin e turnit.

## S3 — Project storage

```text
domain=project-storage + material/files
→ PROJECT_STORAGE
```

Pritshmëri: `material_sha256` lidhet me titull, hipotezë, assumptions, description dhe files.

## S4 — Legacy science

```text
Light --project-route-legacy-once
→ Quantum --project-process-once
→ run(...)
→ GCL/PD/TRL/Shadow
```

Pritshmëri: sjellja e v1.6.4 ruhet.

## S5 — Non-project request

Request pa `ScientificProjectInput` drejt `--project-workspace-once`.

Pritshmëri: fail-closed, exit 70.

## S6 — Corrupt identity

`project_id`, `trace_id`, `payload_sha256` ose `context_sha256` nuk përputhen.

Pritshmëri: fail-closed para orientimit.

## S7 — Token isolation

Kërkim statik në modulin Workspace për LgcToken/LgcGate/token_forge/mint/SEAL.

Pritshmëri: zero referenca ekzekutive.
