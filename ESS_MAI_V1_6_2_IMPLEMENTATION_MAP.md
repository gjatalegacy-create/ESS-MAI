# ESS-MAI v1.6.2 — Implementation Map

| Contract/flow | Implementation |
|---|---|
| Default Untrust | `quantum/src/runtime_pulse.rs::begin_cycle` |
| Real module completion | `quantum/src/main.rs` nine `mark_action` sites |
| SRK proof hinge | `quantum/src/srk.rs` + SRK action site in `main.rs` |
| Canonical action ledger | `quantum/src/runtime_pulse.rs::ActionEvidence` |
| Wire evidence words | `shadow-contracts/src/lib.rs::ActionEvidenceWire` |
| Exact stage order/schema | `runtime_pulse.rs`, `shadow-contracts/src/lib.rs` |
| Shadow fold replay | `shadow-contracts::replay_action_evidence` |
| Shadow field cross-binding | `shadow/src/process_bridge.rs::validate_final_evidence` |
| Living Trust ×3 | `living_trust_contract.rs` in Light/Quantum/Shadow |
| Supreme pulse and L-500 | `shadow/src/shadow_gj_legacy.rs` |
| SHA-256 VerificationReceipt ×3 | `lab_contracts/verification_receipt.rs` |
| Sovereign receipt production | `shadow/src/sovereign_ffi_gate.rs` |
| Receipt transport | `shadow-contracts/src/lib.rs`, `shadow/src/process_bridge.rs` |
| Quantum receipt/Trust verification | `quantum/src/main.rs`, `progressive_debatic/runtime.rs` |
| SHA-256 TokenForge witness | `quantum/src/token_forge.rs` |
| GCL action token → pending iZ | `quantum/src/progressive_debatic/seal.rs` |
| Trust/receipt → iZ/next_i0 | `quantum/src/progressive_debatic/runtime.rs` |
| PD Light courier verification | `light/src/pd_light.rs` |
| Nura + emotional UI parallel handoff | `light/src/main.rs`, `legacy_emotional_ui.rs` |
| Shadow complete mediation | `shadow/src/main.rs`, `quantum/src/shadow_process_bridge.rs` |
| Stateful test isolation | test-only mutex in `runtime_pulse.rs` and `token_forge.rs` |
| Cargo release proof | `VALIDATE_V162.ps1` |
| Authoritative history | `ess-mai.md` |

## No invented implementation

- No HMAC/key lifecycle without a sovereign key contract.
- No runtime Cargo attestation without signed build provenance.
- No direct Quantum→Shadow core link.
- No Layer authority outside GCL.
- No PD processing moved into Light.
