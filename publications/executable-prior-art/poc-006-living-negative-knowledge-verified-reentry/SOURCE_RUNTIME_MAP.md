# POC-006 source-to-runtime map

## Authority hierarchy before contradiction analysis

```text
GCL / ESS-MAI constitutional root
  |-- Shadow authority
  |     validate asset -> adjudicate -> transact -> persist -> export
  |-- Quantum authority
        import -> verify readiness -> filter candidate -> permit/refuse PRO

Constructive       -> KnowledgeWrite::Primitive -> K+
RigorousNegative   -> KnowledgeWrite::Negative  -> K-
insufficient proof -> HOLD/refusal               -> no fabricated knowledge
```

Shadow and Quantum are sibling authorities with different jurisdiction. A
failed import does not make verification “fail as an authority”; the verifier
returns the exact negative state and the consumer refuses progression.

## Production graph

| Stage | Source file / coordinates | Symbol | Producer | Actual production caller | Input | Transformation | Output | Consumer / sink | Authority owner | Mutation / persistence | Failure branch | Existing test | Edge status |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Type negative asset | `shadow-contracts/src/negative_asset.rs` | `NegativeAssetEnvelope` | Quantum negative path | Shadow bridge decoder/validator | typed failure evidence | canonical encode/decode + token fields | typed envelope | `negative_asset::validate` | GCL contract | none / wire only | decode/shape error | asset contract tests | `PROVEN_SOURCE_EDGE` |
| Validate parent asset | `shadow/src/process_bridge.rs:500-502`; `shadow/src/negative_asset.rs` | `validate_final_evidence` / `validate` | mediated cycle request | `process_bridge::run_cycle` | envelope + final evidence | independent token, binding and evidence checks | `ValidatedNegativeAsset` | `into_quantum(...Some(...))` | Shadow validation | ownership move / none | cycle returns error | native negative tests | `PROVEN_SOURCE_EDGE` |
| Stage distinct K+/K- writes | `shadow/src/types.rs:582-584,650-675`; `shadow/src/shadow_gj_legacy.rs:348-367`; `shadow/src/shadow_gateway.rs:633-672` | `ConstitutionalVerdict`, `stage_parent_negative` | confirmed verdict | Shadow ingest path | `(Y,X)`, proof, validated asset | closed verdict domain; constructive/negative remain distinct | `KnowledgeWrite::Primitive` or `KnowledgeWrite::Negative` | transaction builder | Shadow under GCL | staged write set / none | mixed/out-of-domain/refused; missing required parent error | four focused verdict tests | `PROVEN_SOURCE_EDGE` |
| Commit transaction | `shadow/src/shadow_gateway.rs:551-629`; `shadow/src/knowledge_vault.rs:591-633` | `VaultTransaction::build`, `commit_transaction` | Shadow gateway | `process_bridge::run_cycle -> ingest_bridged` at 534-539 | identity, writes, lineage, receipt | digest, dedupe, validate, apply | committed transaction + receipt | Vault/WAL | Shadow persistence | authoritative mutation / disk-backed Vault | seal or commit error | native transaction/durability tests | `PROVEN_SOURCE_EDGE` |
| Export committed NK | `shadow/src/process_bridge.rs:541-544,1158-1175` | `publish_negative_export` | same mediated cycle after ingest returns | `run_cycle` | `Shadow::export_negative_knowledge()` | `NKL1`, length, FNV checksum; atomic file replacement | `shadow_nk_export.bin` + `.sealed` marker | next Quantum cycle | Shadow export | writes handoff files / disk | I/O error aborts response | source + bridge tests | `PROVEN_SOURCE_EDGE` |
| Signal commit result | `shadow-contracts/src/lib.rs:1196,2122-2131`; `shadow/src/process_bridge.rs:546-600` | `ShadowCycleResponse.negative_persisted` | committed Shadow response | process bridge encoder | parent-negative staged Boolean | serialize Boolean with main verification receipt | Boolean + generic receipt | Quantum main | wire contract | none / response file | decode error | contract round-trip tests | `PROVEN_SOURCE_EDGE`, structurally incomplete for full negative receipt |
| Require verified history | `quantum/src/main.rs:723-770` | `nk_history_required`, readiness guard | handoff files | Quantum cycle main | marker + blob | downgrade check, sealed import, readiness decision | Vault or fail-closed `false` | `ProEngine::activate` | Quantum import | runtime readiness state / none | corrupt/missing/downgraded history prevents PRO | native gate tests | `PROVEN_SOURCE_EDGE` |
| Import content | `quantum/src/pro_nk_gate.rs:86-284` | `KnowledgeVault::from_negative_export` | verified handoff blob | Quantum main at 756 | sealed bytes | validate format/checksum; parse entries; derive semantic text/stems | Quantum negative entries | `NkGate::post_filter` | Quantum NK gate | in-process Vault / no durable Quantum sink | `NOT_READY` + empty Vault | native 12-test suite | `PROVEN_SOURCE_EDGE` |
| Affect future candidate | `quantum/src/main.rs:772`; `quantum/src/pro.rs:204-224`; `quantum/src/pro_nk_gate.rs` | `ProEngine::activate`, `NkGate::post_filter` | PRO operator outputs | Quantum main / PRO loop | candidate stems + imported Vault | similarity, hard block/penalty, score transformation | candidate score + NK verdict | candidate ranking and cycle flow | Quantum reasoning | access count in in-process imported Vault / none | no candidates or blocked score | PRO/NK native tests | `PROVEN_SOURCE_EDGE` |
| Recover Shadow Vault | `shadow/src/vault_disk.rs`; `shadow/src/knowledge_vault.rs` | WAL round-trip/reopen | committed Vault | Shadow constructor/tests | disk state | encode, replay, reopen | restored entries | later Shadow operations/export | Shadow persistence | disk state | corrupt record/error paths | `durability_roundtrip` passes | `PROVEN_SOURCE_EDGE` plus component runtime test |
| Detached causal experiment | `experiment/src/main.rs` | `guarded_activation` | POC harness | POC binary/test | synthetic minimal encoded entry using exact copied import/filter code | mirror the main readiness guard; baseline/target/ablation/tamper | machine-readable receipt | test assertion/process exit | no added production authority | in-process only / none | nonzero test/process result | 30 tests total | `TEST_ONLY_EDGE` |

## Closure and missing edge

The v189 source contains a production caller chain from Shadow admission to a
sealed handoff and from the next Quantum main cycle to `ProEngine` and
`NkGate`. The detached capsule executes the exact copied import/filter modules
with a minimal authored guard rather than packaging the private Shadow and
Quantum executables.

The current response transports `negative_persisted: bool` together with a
generic verification receipt. It does not transport an independently
verifiable `NegativeCommitReceiptWire` binding negative asset, transaction,
cycle, GCL identity and export digest. This gap is source-observed; it is not a
hard-coded runtime measurement.

## Evidence classification

- Byte-identical focal modules plus native tests: primary source/runtime
  evidence within their stated scopes.
- Non-copied production files: source evidence recorded by path, coordinates,
  byte count and SHA-256.
- Detached adapters and encoded test entry: disclosed POC instrumentation.
- Architecture and academic prose: reference only unless promoted by the
  source/runtime checks above.
- Exact files named “academic 1”, “academic 2”, or “academic 3” were not found
  under the searched v189 tree; `ess-mai.md:5746` and `:5760` are historical
  academic notes and are not runtime proof for this POC.
