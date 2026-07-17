# ESS-MAI v1.6.4 — Implementation Map

| Kufiri | Implementimi | Përgjegjësia |
|---|---|---|
| Old UI upload | `light/ui/src-tauri/src/main.rs::upload_project` | Pranon material; thërret vetëm Light |
| Old UI emotion | `reflect_system_emotion` + `ui_contracts/emotional_command.rs` | Pasqyrim, jo reasoning |
| Light intake wire | `shadow-contracts::LightProjectIntakeRequestWire` | Vetëm material përdoruesi; pa identitet/kontratë/vulë |
| Light route | `light/src/project_process_bridge.rs` | Validim, APUPK route, response |
| Light authority boundary | `light/src/sovereign_bridges.rs` | Krijon GCL contract id dhe vulën nga witness-i |
| Quantum project process | `quantum/src/main.rs --project-process-once` | PD/GCL/Spine/Digital Lab, TRL ≤ 3 |
| Public TRL bounds | `shadow-contracts/src/lib.rs` | `QUANTUM_MAX_TRL=3`, `SHADOW_FACTUAL_TRL=4` |
| Shadow mediation | `shadow/src/process_bridge.rs` | Wire/APUPK/final evidence validation |
| Shadow GCL gate | `shadow/src/shadow_gj_legacy.rs::verify_project_gcl_stage` | GCL/Spine/SHA/Vula/TRL bounds |
| Shadow file gate | `verify_project_file_kinds` | Magic-byte verification |
| Shadow TRL gate | `shadow_lab.rs` | TRL support verdict |
| Shadow TRL4 | `shadow_genius_novel.rs` | Factualization from real documentation |
| Supreme integration | `adjudicate_project_under_gcl` | Status/result under same sovereign verdict |
| PD handoff | `quantum/src/main.rs`, `light/src/pd_light.rs` | 45 fields aligned and parseable |
| Historical record | `ess-mai.md` + outer `ess_mai.md` | Byte-identical v1.6.4 progress |
