# Source map and extraction identity

## Source identity

The local source snapshot is carried by the directory named
`v1.8.9_ess_mai_phase_015`. Its internal Cargo packages and `VERSION` file
report `1.7.8`. Both identities are recorded; neither is silently rewritten.

| Detached focal file | v189 source path | Bytes | Lines | SHA-256 | Identity |
|---|---|---:|---:|---|---|
| `source_core/src/sovereign_ffi_gate.rs` | `shadow/src/sovereign_ffi_gate.rs` | 24,080 | 498 | `88ed03f02697ca772fb8d6bfd8575f16ce6404e720265a423ecffaa001081142` | byte-identical |
| `source_core/src/living_trust_contract.rs` | `shadow/src/living_trust_contract.rs` | 9,813 | 257 | `7107a2db3b8fd8e05fa2ad12093cd4fa55cf5fb35e76154189a88edecab4bcc2` | byte-identical |
| `source_core/src/verification_receipt.rs` | `shadow/src/lab_contracts/verification_receipt.rs` | 4,283 | 104 | `93f010fbd3171c8ec8f452e3f7332df5c529eaefa7ce6afd910200b0381b8d9c` | byte-identical |

## Focal coordinates in v189

- `shadow/src/sovereign_ffi_gate.rs:63` — C-visible `CapHandle`;
- line 95 — Rust-owned `CapSlot`;
- line 122 — issuance;
- line 137 — nonce checks and atomic burn;
- line 242 — C ABI issuance;
- line 273 — C ABI validate/write boundary;
- line 300 — direct-write commit function;
- lines 357–484 — replay, forgery, unknown-handle, third-use, hold and pointer tests.

Related superior authority:

- `shadow/src/types.rs:644` — private `LawConfirmedVerdict`;
- line 650 — constitutional pair closure.

## Authored extraction shell

`source_core/src/lib.rs` is not claimed byte-identical. It is a narrow
compilation adapter for non-focal types and capability lineage. The experiment
is also new POC code. Neither adapter carries Vault persistence authority.

The exact files and the adapters are distinguished in the manifest and hashes.

