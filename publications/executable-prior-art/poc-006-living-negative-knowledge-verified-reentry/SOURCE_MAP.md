# Source map and caller closure

## Byte-identical focal files

| POC path | v189 source path | SHA-256 | Role |
|---|---|---|---|
| `source_core/src/pro_nk_gate.rs` | `quantum/src/pro_nk_gate.rs` | `66f14e82141b7c3e18d9e7f487093899fb3a033ae86ebb804d47749a5ba196c6` | sealed import, downgrade detection, post-filter |
| `source_core/src/tokenizer.rs` | `quantum/src/tokenizer.rs` | `b7214b7f3c336244cdb2fd373c60dc267521a5e7485396e9f2598f33620fa6df` | semantic stems used by the gate |
| `source_core/src/runtime_pulse.rs` | `quantum/src/runtime_pulse.rs` | `903547c3900de4f3892c6b8baa382743986558efebbff5f58faedc9dde95969c` | NK readiness state |
| `shadow-contracts/src/negative_asset.rs` | `shadow-contracts/src/negative_asset.rs` | `ecbab84bfc65f2561be133f12b047f5e863423c823e294f5bec8f1e06fb673eb` | typed negative asset contract |

## Authored detached adapters

- `source_core/src/lib.rs` supplies only the narrow GCL readiness,
  `FragmentVector`, and FNV module paths required by the exact focal files.
- `shadow-contracts/src/lib.rs` exposes the exact asset module and its fixed
  mass helper; it grants no Shadow persistence authority.
- `experiment/src/main.rs` constructs the falsification cases and guards the
  activation call exactly at the re-entry boundary.

Adapters are compilation scaffolding and are not represented as v189 source.

## Additional read-only source evidence — not copied

| v189 path | SHA-256 | Coordinates | Finding |
|---|---|---|---|
| `shadow/src/types.rs` | `72b163bea72d6f7dfc98e8ef25c980e18a87dfa58cb3b92ffda5a3f066fa8650` | 582-584, 650-675 | closed constructive/rigorous-negative domain and refusal law |
| `shadow/src/shadow_gj_legacy.rs` | `b4b5f11904625c3c9ee39c67c6331a0a2f12246e847581cae46de7d6b0dbf287` | 348-367 | sealed verdict maps to Primitive or Negative write |
| `shadow/src/shadow_commit.rs` | `8884de24b2cd999959489fee9bf99671d6cb46df3d82edbbbd6e05f2f935a557` | 7-25 | both variants share one closed knowledge write-set |
| `shadow/src/knowledge_vault.rs` | `5dd362f225682d86d782d0b3b3e122a7d4eb58916ce344ae380add9d3602c36d` | 591-633 | both variants are applied by one Vault transaction |

These full files are deliberately not duplicated into the capsule. Their
paths, coordinates, sizes and hashes are recorded in
`evidence/07_parallel_knowledge_branch_source.txt`.

## Native caller closure used as evidence

```text
Shadow negative asset validation
  shadow/src/negative_asset.rs
    -> shadow/src/shadow_gateway.rs transaction/staging
    -> shadow/src/knowledge_vault.rs validation/apply/dedupe/WAL
    -> shadow/src/process_bridge.rs NK export
    -> shadow-contracts wire negative_persisted Boolean
    -> quantum/src/main.rs history_required/readiness guard
    -> quantum/src/pro_nk_gate.rs import/filter
    -> quantum/src/main.rs ProEngine::activate
```

Relevant coordinates are preserved in `evidence/05_source_identity.txt` and
the native command records. Coordinates are navigation aids; tests and hashes
are the promoted evidence.
