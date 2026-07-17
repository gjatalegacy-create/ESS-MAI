# ESS-MAI v1.6.7 — Nura Legacy Tauri 2 Implementation Plan

## 1. Governing boundary

The UI is a non-sovereign shell. Its only executable backend boundary is:

```text
UI → Tauri 2 → Light
```

The UI and Tauri backend must never spawn Quantum or Shadow directly, mint or transport LGC/Forge/capability tokens, calculate a GCL verdict, declare TRL4, or construct a Living Trust receipt.

## 2. Implemented functions

### Home / normal Nura conversation

`ask_nura` starts `light-platform`, writes the bounded user input to Light stdin, reads stdout/stderr, accepts Nura text only when Light releases a `Nura:` line, and maps the same runtime output into the existing emotional-command contract.

### Deep Research

Deep Research is request metadata delivered to Light. It is not authority and it does not choose Quantum or Shadow directly.

### Research Domains

Domain selection prepares metadata for project intake. It does not execute a sovereign module.

### My Light Knowledge

The old UI accepts bounded user files and reflects their local selection. Files are not declared verified by the UI.

### My Living Project

Two explicit, non-overlapping routes are available:

- `workspace` → Light `--project-route-once`: APUPK registration followed by Quantum project-storage/conversation orientation;
- `scientific` → Light `--project-route-legacy-once`: the existing full scientific project route under GCL.

Both routes begin in Light. The UI uses `shadow_contracts` only as a zero-authority wire codec.

### My Evolution

Displays trace, emotional phase, last project route, and token policy returned or observed at the UI boundary. It produces no system truth.

### System Modules

Uses the exact runtime responsibilities:

- Light Coordination;
- Quantum Reasoning Pipeline and Project Workspace orientation;
- Shadow Multi-Step Verification and APUPK persistence;
- TRL4 only inside Shadow under the same GCL process.

The label “Quantomic” is not used.

## 3. Hardening decisions

- strict Tauri CSP; no remote scripts, styles, or web APIs;
- no Tauri filesystem or shell plugin;
- no direct Quantum/Shadow process call;
- absolute `ESSMAI_LIGHT` override only;
- canonical binary validation;
- bounded chat, text, file count, file size, and aggregate size;
- NUL rejection for text fields;
- versioned `shadow_contracts` encoder/decoder for project handoff;
- unique temporary request/response paths;
- cleanup after each process handoff;
- explicit workspace versus scientific project selection;
- no implicit scientific escalation;
- no token fields in UI request or response structures.

## 4. Build order

From the ESS-MAI root:

```powershell
cargo build --workspace --release
cd ui
cargo check --all-targets
cargo build --release
```

Run:

```powershell
$env:ESSMAI_LIGHT = (Resolve-Path "..\target\release\light-platform.exe").Path
.\target\release\essmai_ui.exe
```

Light resolves Quantum and Shadow through its existing process boundary. Configure existing ESS-MAI runtime variables, including the handoff directory, according to the main project setup.

## 5. Acceptance gates

1. `cargo check --workspace --all-targets` passes.
2. `cargo test --workspace --all-targets --no-run` passes.
3. UI `cargo check --all-targets` passes independently.
4. UI Clippy passes with `-D warnings`.
5. Normal chat spawns Light only.
6. Project Workspace calls Light `--project-route-once` only.
7. Scientific Project calls Light `--project-route-legacy-once` only after explicit selection.
8. No UI source contains a direct `Command::new` call for Quantum or Shadow.
9. No UI source imports LGC, ForgeToken, CapHandle, VerificationReceipt, Living Trust, or Shadow authority types.
10. Project receipt is decoded through `shadow_contracts` and displays the Light/Shadow witness only.
