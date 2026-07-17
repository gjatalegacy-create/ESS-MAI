# ESS-MAI v1.6.3 — Harta e implementimit

## Light

| Skedari | Ndryshimi |
|---|---|
| `light/src/gcl_project_contract.rs` | kontrata Project SHA-256 byte-identike ×3 |
| `light/src/project_process_bridge.rs` | Shadow registration + Quantum process execution + request binding |
| `light/src/sovereign_bridges.rs` | rrjedha APUPK→Shadow witness→Quantum |
| `light/src/quantum_bridge.rs` | payload kanonik shkencor; bus legacy i etiketuar |
| `light/src/apupk/apupk_coordinator.rs` | titulli hyn në paketën APUPK |
| `light/src/main.rs` | `--project-route-once`; PD sealed field count 46 |
| `light/src/pd_light.rs` | project/Novel fields, Trust recomputation, Nura surface |
| `light/src/legacy_emotional_ui.rs` | project continuity signal |
| `light/src/living_trust_contract.rs` | full evidence/verdict SHA në Trust |

## Quantum

| Skedari | Ndryshimi |
|---|---|
| `quantum/src/gcl_project_contract.rs` | kontrata Project SHA-256 ×3 |
| `quantum/src/bridge_light/mod.rs` | ScientificProjectInput deserialize/validate |
| `quantum/src/main.rs` | real project entrypoint, Digital Lab integration, FinalEvidence, Trust/PD handoff |
| `quantum/src/shadow_process_bridge.rs` | dev-only standalone negative path |
| `quantum/src/living_trust_contract.rs` | project SHA identities në Trust |
| `quantum/src/lib.rs` | eksport i kontratës Project |

## Shadow contracts

| Skedari | Ndryshimi |
|---|---|
| `shadow-contracts/src/lib.rs` | protocol v8, project wire types/codecs, FinalEvidence/Verdit expansion |

## Shadow

| Skedari | Ndryshimi |
|---|---|
| `shadow/src/gcl_project_contract.rs` | kontrata Project SHA-256 ×3 |
| `shadow/src/process_bridge.rs` | registration, APUPK lock, material/APUPK checks, negative gate |
| `shadow/src/shadow_apupk.rs` | WAL v2, title/V500, durable store, owner/trace/progress checks |
| `shadow/src/sovereign_log.rs` | checked append + flush + fsync |
| `shadow/src/types.rs` | ScientificProjectContext dhe verdict fields |
| `shadow/src/shadow_gj_legacy.rs` | same judge Novel/Hold/negative, project SHA→Trust |
| `shadow/src/shadow_gateway.rs` | project transfer into PassPackage |
| `shadow/src/bridge/quantum_in.rs` | project context transport |
| `shadow/src/bridge/shadow_out.rs` | project result transport |
| `shadow/src/sovereign_ffi_gate.rs` | receipt/Trust linkage me project verdict |
| `shadow/tests/integration.rs` | E0063 semantic fixture closure |
| `shadow/Cargo.toml` | v1.6.3 + explicit dev_harness |

## Release/documentation

- të gjitha manifests/Tauri configs → 1.6.3;
- `ess-mai.md` → hyrje autoritative e detajuar;
- `CHANGELOG_v1.6.3.md`;
- audit/simulation/implementation maps;
- `VALIDATE_V163.ps1`;
- diff, changed-files, manifest SHA-256.
