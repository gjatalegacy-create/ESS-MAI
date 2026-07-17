# ESS-MAI v1.6.5 — Implementation Map

| Kufiri | Implementimi | Përgjegjësia |
|---|---|---|
| Old UI | `upload_project` | Vetëm material + emotion; thërret Light default route |
| Light default project route | `--project-route-once` | APUPK/Shadow register → Quantum Workspace |
| Light legacy route | `--project-route-legacy-once` | APUPK/Shadow register → Quantum full science |
| Shared Light preparation | `prepare_project_handoff_under_gcl` | Një witness, SHA-256, kontratë V164 dhe Vulë 500 për të dyja rrugët |
| Quantum Workspace entry | `--project-workspace-once` | Validim dhe orientim project-only |
| Quantum Workspace module | `project_workspace_router.rs` | Storage/chat/both + SHA-256 record identities |
| Quantum legacy entry | `--project-process-once` | Rrjedha e plotë GCL/PD/TRL/Shadow e v1.6.4 |
| Persistent project store | Shadow APUPK | Mbetet magazina e vetme persistente |
| Token boundary | Existing LGC/Forge/PD modules | Byte-identikë; nuk importohen nga Workspace |
| GCL/Living Trust | V164 contracts | Të pandryshuara; Workspace nuk është autoritet |
| Historical record | `ess-mai.md` + `ess_mai.md` | Byte-identical progress v1.6.5 |
