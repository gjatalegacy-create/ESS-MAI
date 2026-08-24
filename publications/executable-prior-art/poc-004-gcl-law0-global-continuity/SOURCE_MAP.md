# Source Map

`<V189_SOURCE>` denotes the private baseline at `v1.8.9_ess_mai_phase_015-ws/v1.8.9_ess_mai_phase_015/ess_mai` beneath the ESS-MAI project root.

The six public files named in `EXTRACTION_MANIFEST.sha256` are byte-identical executable evidence in this capsule. Locations elsewhere under `<V189_SOURCE>` are static source mapping of a private baseline; readers cannot reproduce those wider mappings from this capsule alone. Any `c01`/`c02` material is reference only and is not evidence for a production claim.

## Shared constitutional source

- `<V189_SOURCE>/gcl-constitution/src/lib.rs:8-14` — private modules and public re-exports.
- `<V189_SOURCE>/gcl-constitution/src/constitution.rs:22-26` — global LAW-0 declaration.
- `constitution.rs:71-90` — `LawViolation`.
- `constitution.rs:92-99` — `LawStep` with public caller-provided fields.
- `constitution.rs:101-106` — `UncertaintyLedger`; internal vector is private.
- `constitution.rs:108-139` — `record`; checks only `after <= before`.
- `constitution.rs:141-159` — current state, collapsed predicate, and step view.
- `constitution.rs:161-172` — report generation; later `before` values are omitted.
- `constitution.rs:175-179` — `pair_space` returns `f32`.
- `constitution.rs:257-260` — LAW-0 constitutional registry entry.
- `constitution.rs:406-426` — upstream reference-chain test.
- `constitution.rs:428-439` — upstream direct-expansion rejection test.
- `<V189_SOURCE>/gcl-constitution/src/phase.rs:3-20` — phase enum and labels.

The exact extracted counterparts are under `extracted/gcl-constitution` at the same relative paths.

## Single physical crate consumption

- `<V189_SOURCE>/Cargo.toml:22` — workspace member.
- `<V189_SOURCE>/light/Cargo.toml:34` — path dependency.
- `<V189_SOURCE>/quantum/Cargo.toml:23` — path dependency.
- `<V189_SOURCE>/shadow/Cargo.toml:39` — path dependency.
- `<V189_SOURCE>/light/src/lab_contracts/gjata_collapse_law.rs:1-5` — re-export.
- `<V189_SOURCE>/quantum/src/lab_contracts/gjata_collapse_law.rs:1-5` — re-export.
- `<V189_SOURCE>/shadow/src/lab_contracts/gjata_collapse_law.rs:1-5` — re-export.

## Light: separate coordinate mechanism

Light re-exports the shared type but does not instantiate `UncertaintyLedger` in the mapped production path.

- `<V189_SOURCE>/light/src/alnur_karina_athar.rs:62-68` — coordinate evidence with `u64` before/after.
- `alnur_karina_athar.rs:308-334` — binding, Besa, and order validation.
- `<V189_SOURCE>/light/src/lgc_algorithm.rs:400-424` — receipt fields.
- `lgc_algorithm.rs:426-533` — receipt recomputation and validation.
- `lgc_algorithm.rs:478-491` — recomputes `(xi+yi)^2` and `xi*yi`.
- `lgc_algorithm.rs:746-766` — production coordination entry and GCL gate.
- `lgc_algorithm.rs:800-824` — real complete/disjoint split checks.
- `lgc_algorithm.rs:837-850` — checked `u64` contraction and fail-closed branch.
- `lgc_algorithm.rs:872-915` — envelope binding.
- `lgc_algorithm.rs:974-1020` — evidence consumption and receipt verification.
- `<V189_SOURCE>/light/src/software_contract.rs:341-350,382-390` — verifies the legacy receipt and embeds `legacy:<receipt_sha256>` in the existing `lgc_seal` string.
- `software_contract.rs:468-518` — checks the canonical contract and recomputes that seal.
- `<V189_SOURCE>/light/src/quantum_bridge.rs:691-724,1128-1167` — the existing Light-to-Quantum wire carries `lgc_seal` and `input_sha256`.
- `<V189_SOURCE>/light/src/main.rs:657-673` — prints valid LAW-0 receipt or stops handoff.

This is materialized coordinate contraction, and the receipt SHA-256 already crosses the Light-to-Quantum boundary inside an existing field. It is still a distinct local mechanism because Quantum does not parse or verify the `legacy:<receipt_sha256>` segment as the first step of one shared ledger.

## Quantum: local shared ledger

- `<V189_SOURCE>/quantum/src/main.rs:11` — shared API import.
- `<V189_SOURCE>/quantum/src/bridge_light/mod.rs:137-153,214-419` — `QuantumInput` retains `lgc_seal` as a string.
- `quantum/src/bridge_light/mod.rs:427-450` — current seal handling reads flags but does not consume the existing legacy receipt SHA-256 segment.
- `<V189_SOURCE>/quantum/src/main.rs:179-200,263-299` — Quantum verifies actual input bytes and consumes the PA/Besa split evidence.
- `<V189_SOURCE>/quantum/src/asht_quantum.rs:59-118` — Besa parsing recomputes and verifies the split SHA-256.
- `quantum/src/main.rs:1313-1318` — creates a new ledger and initial spaces.
- `quantum/src/main.rs:1319-1328` — derives local post-elimination and outcome spaces.
- `quantum/src/main.rs:1329-1352` — records four steps.
- `quantum/src/main.rs:1353-1358` — prints success or returns fail-closed on a local violation.

The caller manually passes each prior variable as the next `before`; the ledger type does not enforce that linkage. The local report is not placed into the Quantum-to-Shadow package.

## Wire boundary

- `<V189_SOURCE>/quantum/src/bridge_light/mod.rs:137-153` — Light-to-Quantum input lacks a continuous LAW-0 transcript.
- `<V189_SOURCE>/shadow-contracts/src/lib.rs:890-941` — `FinalEvidenceWire` contains action/organ evidence and a package digest, but no LAW-0 step transcript.
- `<V189_SOURCE>/shadow-contracts/src/lib.rs:943-1045` — final-evidence digest recomputation and internal verification cover that separate evidence package, not a possibility-space ledger.
- `<V189_SOURCE>/shadow-contracts/src/lib.rs:1063-1095` — `QuantumInboundWire` has no `LawStep`, prior space, ledger head, or upstream LAW-0 digest.
- `<V189_SOURCE>/shadow-contracts/src/lib.rs:1905-1969` — corresponding Quantum encoding and decoding also carry no LAW-0 field.
- `<V189_SOURCE>/shadow/src/process_bridge.rs:695-744` — Shadow validates the final-evidence package and input hash at ingress.
- `<V189_SOURCE>/shadow/src/process_bridge.rs:1055-1101` — conversion into the internal input carries final-evidence, GCL-process, and spine digests, but introduces no LAW-0 transcript or digest.
- `<V189_SOURCE>/shadow/src/bridge/quantum_in.rs:16-86` — internal Shadow inbound type likewise lacks those fields.

These structures provide an existing upstream lineage route into Shadow, but none is a possibility-space LAW-0 transcript and none attests the Quantum ledger report.

The strongest architecture-preserving route is therefore already visible in source: parse and fail-closed verify the existing Light receipt SHA-256 in Quantum; add a canonical `u64` continuity sidecar to the existing `FinalEvidenceWire`; verify it at the existing Shadow process gate; then feed its terminal head into the already-present `VerificationContext.law0_digest` and transaction path. This reuses carriers and authority boundaries, but the Q-to-S sidecar still changes the serialized schema and therefore requires a coordinated protocol version bump.

## Shadow: separate terminal ledger

### Local marker and context

- `<V189_SOURCE>/shadow/src/shadow_gateway.rs:83-95` — private `VerificationContext` has `vds_close_digest` and `law0_digest` slots.
- `<V189_SOURCE>/shadow/src/shadow_gateway.rs:117-155` — `from_xy` binds upstream final-evidence/GCL/spine values into the separate `xy_digest`, but initializes `law0_digest` to zero.
- `<V189_SOURCE>/shadow/src/shadow_gateway.rs:892-980` — production `ingest_bridged` extracts the upstream digests and constructs that context; no upstream LAW-0 transcript is extracted.
- `<V189_SOURCE>/shadow/src/shadow_gateway.rs:1187-1217` — the VDS/structural path must close before the local LAW-0 marker is created.
- `<V189_SOURCE>/shadow/src/shadow_gateway.rs:1218-1225` — a new ledger is created locally and always records the single hard-coded Verification step `kolaps3_D: 1.0 -> 0.0`.
- `<V189_SOURCE>/shadow/src/shadow_gateway.rs:1226-1230` — VDS is hashed separately; `law0_digest` hashes only the local `chain_report` under `ESSMAI_LAW0_CLOSE_V1`.

Because the LAW-0 hash material contains neither session identity nor upstream evidence and the sole step is fixed, every successful mediated cycle currently produces the same local LAW-0 close marker. This is a terminal integrity marker, not a per-cycle Light-to-Quantum-to-Shadow uncertainty transcript. Paths that fail before lines 1218-1230 produce neither this marker nor a LAW-0 receipt.

### Digest-to-WAL binding path

- `<V189_SOURCE>/shadow/src/shadow_gateway.rs:527-576` — `commit_decision` places `law0_digest` beside identity, XY/PD, verdict, lineage, VDS, runtime, and write material in `cycle_digest`.
- `<V189_SOURCE>/shadow/src/shadow_gateway.rs:577-629` — the cycle is de-duplicated, a separate verification receipt is sealed, and the local digest is passed into `VaultTransaction::build` before commit.
- `<V189_SOURCE>/shadow/src/vault_transaction.rs:13-32` — `VaultTransaction` stores `cycle_digest`, `law0_digest`, `receipt_digest`, and the receipt as distinct fields.
- `<V189_SOURCE>/shadow/src/vault_transaction.rs:69-108` — transaction construction accepts the caller-supplied local digest.
- `<V189_SOURCE>/shadow/src/vault_transaction.rs:110-135` — validation checks verdict/receipt consistency and transaction integrity, but does not require a nonzero LAW-0 digest or recompute it from a transcript.
- `<V189_SOURCE>/shadow/src/vault_transaction.rs:137-172` — `compute_transaction_id` hashes `law0_digest` and the other transaction material; therefore changing the stored digest changes the transaction identifier.
- `<V189_SOURCE>/shadow/src/vault_transaction.rs:176-195` — the sibling `receipt_digest` is computed only from receipt fields and does not add LAW-0 to the receipt.
- `<V189_SOURCE>/shadow/src/vault_disk.rs:232-255` — WAL transaction encoding writes both `transaction_id` and `law0_digest`.
- `<V189_SOURCE>/shadow/src/vault_disk.rs:317-410` — WAL decoding restores the digest and validates transaction/commit consistency.
- `<V189_SOURCE>/shadow/src/vault_disk.rs:609-627` — framed WAL data is written, flushed, and `sync_all` is called.
- `<V189_SOURCE>/shadow/src/knowledge_vault.rs:499-540` — WAL/fsync completes before the in-memory mutation is applied.

The source-mapped path is: local fixed `1 -> 0` report → SHA-256 `law0_digest` → `cycle_digest` → `VaultTransaction` → `transaction_id` → WAL bytes. The mapping documents the local byte-integrity and co-binding path; this capsule did not execute the full v189 runtime durability path and does not prove semantic origin or global ledger continuity.

### Receipt and public response boundary

- `<V189_SOURCE>/shadow/src/sovereign_ffi_gate.rs:70-88` — `VerificationReceipt` has no `law0_digest`, `cycle_digest`, or `transaction_id` field.
- `<V189_SOURCE>/shadow/src/sovereign_ffi_gate.rs:164-230` — receipt seal/construction does not receive LAW-0 material.
- `<V189_SOURCE>/shadow/src/lab_contracts/verification_receipt.rs:36-70` — canonical `receipt_id` omits LAW-0, cycle, and transaction identifiers.
- `<V189_SOURCE>/shadow/src/process_bridge.rs:546-597` — the process boundary maps only verdict and the LAW-0-free verification receipt into the response.
- `<V189_SOURCE>/shadow-contracts/src/lib.rs:1126-1141` — `VerificationReceiptWire` has no LAW-0 field.
- `<V189_SOURCE>/shadow-contracts/src/lib.rs:1191-1197` — `ShadowCycleResponse` exposes no cycle, transaction, or LAW-0 digest.
- `<V189_SOURCE>/shadow-contracts/src/lib.rs:2011-2041` — receipt wire encoding/decoding confirms the omission.
- `<V189_SOURCE>/shadow-contracts/src/lib.rs:2117-2134` — cycle-response encoding/decoding has no hidden LAW-0 payload.

Consequently, the durable transaction co-binds the receipt and local LAW-0 marker as sibling evidence, but the receipt itself cannot attest that marker and callers cannot recover it from the public response.

### Durable retrieval boundary

- `<V189_SOURCE>/shadow/src/vault_transaction.rs:40-46` — `DurableCommit` returns transaction/cycle identifiers, status, and receipt, but no `law0_digest`.
- `<V189_SOURCE>/shadow/src/knowledge_vault.rs:475-497` — committed-cycle lookup reconstructs that LAW-0-free `DurableCommit`.
- `<V189_SOURCE>/shadow/src/knowledge_vault.rs:542-659` — apply/replay destructures the persisted field as `_law0_digest` and creates the commit/index without retaining a retrievable LAW-0 value.
- `<V189_SOURCE>/shadow/src/anchor_lease.rs:19-25,73-85` — lease evidence links consumption to `transaction_id`, not directly to LAW-0.

The digest remains integrity-bound inside the WAL transaction, but the normal committed-transaction API drops it after apply/replay. Retrieving or semantically replaying the LAW-0 transcript is therefore not materialized.

### Protocol implication

- `<V189_SOURCE>/shadow-contracts/src/lib.rs:13` — the current wire protocol version is `10`.
- `<V189_SOURCE>/shadow-contracts/src/lib.rs:1490-1500` — frame decoding requires the exact protocol version.

Adding an upstream LAW-0 transcript/digest or exposing it in `VerificationReceiptWire`/`ShadowCycleResponse` changes the serialized schema. It therefore requires a coordinated protocol-version bump and matching Quantum/Shadow encoders, decoders, internal types, receipt semantics, and compatibility tests; it is not a documentation-only change.

## POC harness

- `experiment/src/main.rs:55` — experiment entry logic.
- `experiment/src/main.rs:92` — discontinuity counterexample.
- `experiment/src/main.rs:111` — terminal regression counterexample.
- `experiment/src/main.rs:132` — negative-domain counterexample.
- `experiment/src/main.rs:138` — non-finite counterexample.
- `experiment/src/main.rs:150` — exact Shadow receipt and harness-only LAW-0 connection probe.
- `experiment/src/main.rs:344-398` — six POC harness test functions; the extracted Shadow module also carries two unchanged source tests.

The harness is new publication glue and is not represented as production source.
