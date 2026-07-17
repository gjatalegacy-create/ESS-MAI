# v1.6.4 — Static Simulation Map

| Stimulus | Expected boundary result |
|---|---|
| UI sends empty project name | Rejected in UI before Light |
| UI attempts to supply GCL contract/seal | Impossible: fields removed from intake wire |
| UI attempts direct Shadow route | No command/path exists in old UI |
| Oversized content >16 MiB | Rejected in UI |
| Individual file >16 MiB | Rejected in UI |
| Full upload >60 MiB | Rejected in UI |
| Light content SHA differs from Shadow witness | Light route fails closed |
| Quantum sends TRL4 | Wire shape invalid; Shadow GCL gate also rejects |
| Project GCL digest differs from PD digest | Shadow rejects before TRL/file adjudication |
| Spine completion is zero | Shadow rejects before factualization |
| Light sovereign flags do not decode to 500 | Shadow rejects at GCL identity gate |
| Declared file type differs from magic bytes | Shadow rejects evidence |
| TRL3 without visual proof | HOLD / no TRL4 |
| TRL3 + proof but sovereign pair mixed | HOLD / no TRL4 |
| TRL3 + real proof + Y=1,X=1 | Shadow may factualize TRL4 |
| No project in legacy fixture | `scientific_project: None`; normal non-project flow |
| SHA string in PD handoff | Serialized with `{}`; parse as canonical SHA-256 |
| project_id in PD handoff | Serialized and parsed as 16-digit hex |
