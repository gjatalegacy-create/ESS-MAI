# Shadow Connection Finding

## Status and evidence basis

This finding is a claim-conservative static audit of the real v189 source tree at:

```text
v1.8.9_ess_mai_phase_015-ws/
  v1.8.9_ess_mai_phase_015/
    ess_mai/
```

The source files identified below are the evidence. The `C01_READ_ONLY_EVIDENCE` and
`C02_READ_ONLY_EVIDENCE` directories may be used as navigation references, but no
technical claim in this finding is derived from them. This file does not claim that the
full v189 runtime was executed by this audit.

## Executive finding

Shadow already materializes four useful mechanisms in one production call path:

1. actual inbound Light bytes are checked against a recomputed SHA-256 digest before
   the Shadow vault is opened;
2. accepted final-evidence material is carried indirectly into the XY verification
   identity and its SHA-256 receipt identifier;
3. a local Shadow LAW-0 close marker and the verification receipt are jointly bound
   into a SHA-256 transaction identifier; and
4. the transaction is appended, flushed and `sync_all`-ed before runtime mutation and
   before the response is returned.

These are real, materialized links. They do **not**, however, constitute a continuous
Light-to-Quantum-to-Shadow LAW-0 transcript. Shadow creates a new local ledger and
records only the constant terminal marker `Verification / kolaps3_D / 1.0 -> 0.0`.
The resulting local digest is present in the durable transaction, but it is absent from
the public verification receipt and from the cycle-response wire. Light already places
its verified `receipt_sha256` in the existing Light-to-Quantum `lgc_seal` as
`legacy:<receipt_sha256>`, but Quantum currently consumes only the primitive-flags
portion of that seal. The existing Quantum-to-Shadow `FinalEvidenceWire` is the narrowest
available carrier for a future continuity transcript, and the existing Shadow
`VerificationContext.law0_digest`/`VaultTransaction` path is the narrowest available
durable sink. Using those carriers still requires explicit schema, codec and protocol
version changes; the current v189 source has no end-to-end LAW-0 transcript.

## Materialized links in current v189

### 1. Inbound digest verification is before authoritative work

The production `run_cycle` path decodes the request and performs identity, transport,
negative-shape and final-evidence checks before opening the Shadow vault:

- `shadow/src/process_bridge.rs:493-515` — production ordering;
- `shadow/src/process_bridge.rs:695-727` — finite-domain, action-evidence and internal
  final-evidence checks;
- `shadow/src/process_bridge.rs:728-732` — SHA-256 is recomputed over the actual
  `light_input_bytes` and compared with the declared `light_input_sha256`;
- `shadow/src/process_bridge.rs:737-947` — NPIM, PD/continuum, action-stage and
  PIM/NPIM/MPRO bindings are checked before the negative asset is accepted.

`FinalEvidenceWire::verifies_internal` also recomputes its package digest and replays
the ordered action evidence:

- `shadow-contracts/src/lib.rs:890-941` — final-evidence fields;
- `shadow-contracts/src/lib.rs:943-1045` — digest recomputation and internal gate;
- `shadow-contracts/src/lib.rs:2869-2877` — corrupt frame rejection test;
- `shadow-contracts/src/lib.rs:2881-2935` — action tamper, duplicate and wrong-schema
  fault tests;
- `shadow/src/process_bridge.rs:1285-1303` — malformed wire returns an error rather
  than panicking.

The precise cryptographic boundary matters: the actual Light payload has an
independent SHA-256 comparison at the Shadow process gate, while the encompassing
`package_digest` remains an FNV-derived `u64` compatibility digest.

### 2. Accepted evidence reaches the verification receipt

`VerificationContext::from_xy` includes `final_evidence_digest`, PD process material
and spine completion material in the XY identity:

- `shadow/src/shadow_gateway.rs:83-95` — verification-context fields;
- `shadow/src/shadow_gateway.rs:117-155` — context construction;
- `shadow/src/shadow_gateway.rs:127-144` — final-evidence material enters the
  FNV-derived XY digest.

Shadow then consumes a single-use internal capability and creates a public
`VerificationReceipt`:

- `shadow/src/shadow_gateway.rs:590-601` — receipt creation in the commit path;
- `shadow/src/sovereign_ffi_gate.rs:74-88` — receipt type;
- `shadow/src/sovereign_ffi_gate.rs:164-230` — capability consumption and receipt
  construction;
- `shadow/src/lab_contracts/verification_receipt.rs:37-69` — domain-separated
  SHA-256 receipt identifier.

The receipt identifier binds session, parent i0, primitive anchor, XY digest, PD
digests, Living Trust SHA-256, the constitutional Y/X pair, generation and seal.
Quantum independently recomputes this identifier at
`quantum/src/main.rs:3380-3410`; Light performs its corresponding closure checks at
`light/src/pd_light.rs:175-245`.

### 3. The local LAW-0 marker and receipt reach one transaction

Near terminal verification, Shadow creates a fresh `UncertaintyLedger`, records one
local step and hashes the rendered local report:

- `shadow/src/shadow_gateway.rs:1218-1225` — new local ledger and constant
  `Verification 1.0 -> 0.0` record;
- `shadow/src/shadow_gateway.rs:1228-1230` — domain-separated SHA-256 of the local
  report becomes `law0_digest`.

The commit path binds that local digest into both the cycle material and the vault
transaction:

- `shadow/src/shadow_gateway.rs:563-576` — cycle digest includes `law0_digest`;
- `shadow/src/shadow_gateway.rs:602-616` — transaction construction and vault commit;
- `shadow/src/vault_transaction.rs:13-32` — transaction fields include
  `law0_digest`, `receipt_digest` and the receipt;
- `shadow/src/vault_transaction.rs:69-108` — transaction construction;
- `shadow/src/vault_transaction.rs:110-172` — validation and SHA-256 transaction-ID
  computation.

This is a real joint binding: changing the stored local LAW-0 digest or receipt after
transaction construction changes the transaction hash and causes the existing
validation path to fail. It is not evidence that the local LAW-0 marker was derived
from an upstream continuity transcript.

### 4. Commit is durable before response publication

The transaction is validated before the backend call, the backend completes before
in-memory application, and the response is constructed only after commit succeeds:

- `shadow/src/knowledge_vault.rs:499-540` — validate, replay gate, backend commit,
  then in-memory application;
- `shadow/src/shadow_gateway.rs:616-629` — commit, runtime update, then response;
- `shadow/src/vault_disk.rs:232-415` — transaction encoding, commit marker, decoding
  and validation on replay;
- `shadow/src/vault_disk.rs:609-639` — append, flush and `sync_all`.

Existing fault tests cover important portions of this behavior:

- `shadow/src/vault_transaction.rs:303-361` — transaction validation and tamper
  rejection;
- `shadow/src/knowledge_vault.rs:1618-1626` — backend failure leaves RAM and the
  committed-receipt index unchanged;
- `shadow/src/knowledge_vault.rs:1629-1641` — exactly-once retry;
- `shadow/src/knowledge_vault.rs:1644-1660` — conflicting replay rejection;
- `shadow/src/vault_disk.rs:842-871` — committed transaction survives restart;
- `shadow/src/vault_disk.rs:875-896` — partial-tail recovery and full-record CRC
  corruption rejection;
- `shadow/src/vault_disk.rs:898-922` — an I/O failure is not reported as success on
  the Linux fault-injection path.

## The exact unlinked boundary

The production path stops short of a publicly verifiable global LAW-0 chain at five
specific boundaries.

### Light already exports a receipt identity that Quantum does not consume

The Light receipt itself is stronger than a free-form log. It records exact `u64`
cardinalities and hashes for the primitive split, the before/after LAW-0 values and the
receipt identity:

- `light/src/lgc_algorithm.rs:400-424` — `LegacyLgcReceipt` fields;
- `light/src/lgc_algorithm.rs:427-534` — canonical-field, split, LAW-0, envelope and
  receipt-integrity verification;
- `light/src/lgc_algorithm.rs:478-490` — checked recomputation of
  `(|Xi| + |Yi|)^2 -> |Xi| * |Yi|` with `u64` arithmetic;
- `light/src/lgc_algorithm.rs:826-850` and `986-1019` — construction and runtime
  verification of the actual Coordination receipt.

The existing Light contract verifies that receipt and embeds its SHA-256 identifier in
the already transported `lgc_seal`:

- `light/src/software_contract.rs:341-350` — receipt verification before contract
  construction;
- `light/src/software_contract.rs:382-390` — the canonical
  `flags:0xA451|legacy:<receipt_sha256>` seal shape;
- `light/src/software_contract.rs:468-518` — canonical receipt-SHA and recomputed-seal
  enforcement;
- `light/src/quantum_bridge.rs:691-724` and `1128-1167` — the existing payload and
  process call carry `lgc_seal` without needing another Light-to-Quantum channel;
- `light/src/light_coordinator.rs:727-756` — production dispatch call site.

Quantum deserializes and stores the full string but currently interprets only the flags:

- `quantum/src/bridge_light/mod.rs:140-153` and `214-419` — `lgc_seal` storage and
  deserialization;
- `quantum/src/bridge_light/mod.rs:427-450` — `carries_seal` extracts primitive flags
  but does not parse or require the `legacy:` receipt SHA;
- `quantum/src/main.rs:1313-1359` — Quantum starts a new ledger and reconstructs a
  separate Coordination step instead of continuing the identified Light receipt.

Therefore the existing wire contains a usable Light receipt identity, but the supported
claim is only that Light transports it. Current v189 does not show Quantum validating,
continuing or forwarding that receipt as LAW-0 continuity evidence.

### No upstream LAW-0 transcript enters Shadow

The Quantum-to-Shadow wire carries final evidence and action-state material, but no
ordered LAW-0 steps, previous head, upstream head, candidate-space boundary or
producer-phase chain:

- `shadow-contracts/src/lib.rs:1063-1095` — `QuantumInboundWire`;
- `shadow-contracts/src/lib.rs:1905-1969` — corresponding wire codec;
- `shadow/src/bridge/quantum_in.rs:16-86` — internal Shadow inbound type.

The final-evidence structures are valuable evidence bindings, but they are not a
possibility-space continuity transcript.

### Shadow starts a new local ledger

`shadow/src/shadow_gateway.rs:1218-1230` does not verify or continue an upstream
ledger. It creates a new ledger and records a fixed local close marker. Accordingly,
the supported claim is:

> Shadow binds a local constant LAW-0 close report into its transaction path.

The unsupported claim is:

> Shadow proves the complete LAW-0 history from Light through Quantum.

### The public receipt does not carry the local LAW-0 or durable-commit identity

`VerificationReceipt` contains neither `law0_digest`, `cycle_digest` nor
`transaction_id`:

- `shadow/src/sovereign_ffi_gate.rs:74-88` — internal public receipt;
- `shadow-contracts/src/lib.rs:1127-1141` — wire receipt;
- `shadow-contracts/src/lib.rs:1191-1197` — cycle response;
- `shadow/src/process_bridge.rs:581-601` — response mapping and write.

The receipt is created before the vault commit, although the production call path
correctly withholds the response until the commit succeeds. Therefore the call order
supports “commit before publication,” but the receipt alone is not an independently
verifiable proof of durable commit or of the local LAW-0 digest.

### Transaction validation binds the value but does not require a LAW-0 proof

`VaultTransaction::validate` checks the Y/X pair, receipt consistency, receipt digest,
transaction identifier and the presence of identity/cycle digests. It does not require
`law0_digest` to be nonzero and cannot determine whether that digest represents a
valid upstream transcript:

- `shadow/src/vault_transaction.rs:110-134` — current validation predicate;
- `shadow/src/vault_transaction.rs:137-172` — the supplied value is hashed into the
  transaction identifier.

Thus a transaction consistently constructed with an all-zero LAW-0 digest is not
rejected by this type-level validation. Production currently supplies a local report
digest, but that caller discipline is not the same as a presence or continuity gate.

## Minimal wiring using existing carriers

The smallest architecture-preserving advancement uses the current Light payload, the
current `FinalEvidenceWire` envelope and the current Shadow transaction sink. It does
not create a fourth authority, a parallel verdict path or a second process channel.
It is still an advancement probe: the required Quantum-to-Shadow fields and wire version
do not exist in current v189.

### 1. Reuse the Light-to-Quantum seal rather than adding another payload field

Quantum can add a fail-closed extractor for the existing canonical
`legacy:<64 lowercase hexadecimal characters>` segment in
`quantum/src/bridge_light/mod.rs:427-450`. Missing, duplicate, malformed or noncanonical
receipt identifiers must reject continuity rather than fall back to an anonymous local
Coordination step.

After the existing PA and Besa gates, Quantum already has the material required to
recompute the Coordination boundary:

- `quantum/src/main.rs:263-299` — PA/split acquisition and Besa companion gate;
- `quantum/src/asht_quantum.rs:59-118` — Besa lineage validation and recomputation of
  the same `split_sha256` from Xi/Yi;
- `light/src/lgc_algorithm.rs:478-490` — the exact checked arithmetic that defines the
  Light receipt boundary.

The connection probe should reproduce that boundary with checked `u64` arithmetic and
bind it to the extracted Light receipt SHA plus the verified split SHA. It should not
convert the source counts to `f32` before hashing or comparing them. The current shared
`UncertaintyLedger` stores `before` and `after` as `f32`
(`gcl-constitution/src/constitution.rs:92-105,115-139`), so it cannot serve as an exact
wire representation for arbitrarily large integer cardinalities. In particular, exact
integer representation is not guaranteed above `2^24`. A small canonical `u64` wire
record may coexist with the current local ledger without changing phase authority.

If implemented and validated, this minimal route would bind continuity to the
Light-issued receipt identifier under the existing Light validation path. Independent
revalidation of the complete Legacy V2
envelope in Quantum would require sharing or reproducing additional receipt-validation
logic and must be claimed separately.

### 2. Extend `FinalEvidenceWire`; do not create a new Quantum-to-Shadow channel

`FinalEvidenceWire` is already built after Quantum evidence processing, recomputed before
send, serialized inside `ShadowCycleRequest`, and independently checked by Shadow:

- `shadow-contracts/src/lib.rs:890-1045` — structure, digest and internal gate;
- `shadow-contracts/src/lib.rs:1803-1902` — final-evidence codec;
- `shadow-contracts/src/lib.rs:2100-2116` — cycle-request codec;
- `quantum/src/main.rs:1896-1973` — construction and pre-send verification;
- `quantum/src/main.rs:1980-2045` — insertion into the existing Shadow request;
- `quantum/src/shadow_process_bridge.rs:15-28` — existing process boundary and
  response-session check;
- `shadow/src/process_bridge.rs:695-810` — Shadow-side final-evidence, input-SHA and
  PD-continuum validation.

The minimal schema extension is a dedicated, canonically encoded LAW-0 sidecar inside
that structure, for example:

```text
light_receipt_sha256
split_sha256
steps[] = { phase_code, step_code, before_u64, after_u64 }
transcript_sha256
```

The sidecar should not overload `quantum_action_evidence` or `pim_proof_chain`, because
those fields already have different validated meanings. It should be included in both
`FinalEvidenceWire::recompute_digest` and the binary put/get functions. Because the
encompassing package and frame checks are FNV-derived `u64` compatibility checks
(`shadow-contracts/src/lib.rs:943-985,1236-1243,1473-1526`), the transcript should also
carry a domain-separated SHA-256 recomputed with the existing helper at
`shadow-contracts/src/lib.rs:308-321`.

This is a real wire change. `shadow-contracts/src/lib.rs:13` currently declares
`PROTOCOL_VERSION = 10`, and `shadow-contracts/src/lib.rs:1473-1526` rejects a frame whose
version is not exactly current. Adding sidecar fields to `FinalEvidenceWire` therefore
requires a coordinated protocol-version bump, codec updates on both sides and updated
round-trip/corruption tests. Silently changing field order under version 10 would make
old and new binaries incompatible and must not be presented as architecture-preserving
compatibility.

### 3. Enforce junctions, not only per-step monotonicity

The current ledger accepts a step when only `after <= before` holds
(`gcl-constitution/src/constitution.rs:115-139`). A disconnected sequence such as
`16 -> 4` followed by `99 -> 1` can therefore pass the current per-step predicate.
The continuity sidecar must additionally enforce, fail closed:

1. exactly one Coordination entry derived from the verified Light split and receipt;
2. Coordination before all Reasoning entries and Verification only at Shadow;
3. `step[i + 1].before == step[i].after` for every junction;
4. `after <= before` for every entry, with checked integer conversions and arithmetic;
5. a canonical step-code/order policy so reordered or duplicate stages fail; and
6. equality between the decoded canonical bytes and `transcript_sha256`.

These checks are the material advancement over three locally monotone reports. They do
not give Quantum Coordination authority or Shadow Reasoning authority: each receiver
verifies and continues evidence issued by the superior phase without recreating that
phase's bounded jurisdictional decision or evidence.

### 4. Continue the verified Quantum head in the existing Shadow sink

`shadow/src/process_bridge.rs:695-810` is the existing fail-closed process gate, and
`shadow/src/process_bridge.rs:1055-1101` plus
`shadow/src/bridge/quantum_in.rs:16-86,158-224` are the existing ownership path into the
Shadow gateway. The new sidecar must remain available through that path rather than
being reduced immediately to the current FNV package digest.

At `shadow/src/shadow_gateway.rs:950-980`, Shadow already holds the leased PA and its
actual Xi/Yi split. It can therefore recompute the same checked `u64` Coordination
boundary and require equality with the first transcript entry before judgement. After
successful verification it should append exactly:

```text
Verification: upstream_last_after -> 0
```

For an accepted single survivor this is `1 -> 0`. For an honest Quantum refusal whose
reasoning space is already empty, it is `0 -> 0`. Equality is already allowed by the
LAW-0 rule; manufacturing a disconnected constant `1 -> 0` for the refusal path would
hide, rather than prove, the junction.

The complete canonical transcript SHA-256 can then replace the local-only report input
at `shadow/src/shadow_gateway.rs:1218-1230`. No new durable subsystem is needed:

- `shadow/src/shadow_gateway.rs:83-155` already provides
  `VerificationContext.law0_digest`;
- `shadow/src/shadow_gateway.rs:563-576` already includes it in the cycle SHA-256;
- `shadow/src/shadow_gateway.rs:602-616` already sends it to `VaultTransaction`;
- `shadow/src/vault_transaction.rs:137-172` already includes it in the SHA-256
  transaction identifier;
- `shadow/src/vault_disk.rs:236-243,317-329,388-410` already persists and validates
  the transaction field.

The public receipt remains a separate caveat. Exposing the terminal LAW-0 identity
would require versioned changes to `VerificationReceipt`, its `receipt_id` material,
`VerificationReceiptWire` and the response codec. Without those changes, the supported
result is “complete transcript committed inside the Shadow transaction,” not “complete
transcript independently visible in the current terminal receipt.”

### 5. Required falsification tests

The advancement should be rejected unless all of these tests are explicit:

| Fault or path | Required result |
|---|---|
| Valid Light receipt ID, verified split and continuous Coordination -> Reasoning -> Verification entries | accepted; one nonzero canonical terminal transcript SHA-256 |
| One nibble changed in the `legacy:` receipt SHA | rejected in Quantum before construction of Shadow evidence |
| Missing, duplicated or noncanonical `legacy:` segment | rejected; no anonymous local Coordination fallback |
| Each entry is locally monotone but a junction is disconnected | rejected even though the current `UncertaintyLedger::record` predicate would accept the individual entries |
| Phase reordered, duplicated, or Verification supplied by Quantum | rejected before Shadow core |
| Quantum refusal ends with zero candidate space | Shadow continues `0 -> 0`; a substituted disconnected `1 -> 0` is rejected |
| One transcript byte or declared transcript SHA changed in the Quantum-to-Shadow request | rejected by SHA-256/semantic verification before vault mutation |
| Sidecar fields encoded under the old protocol version | rejected; no silent version-10 reinterpretation |
| Verified terminal transcript SHA changed after transaction construction | existing transaction validation rejects the changed transaction identity |
| Current public receipt inspected without a schema change | correctly reported as not containing the LAW-0 transcript identity |

Passing these tests would demonstrate feasibility of the missing connection. It would
not show that unmodified v189 currently implements the connection, nor would it prove
external signer authentication or the truth of the underlying evidence.

## Integrity and authentication boundary

The source supports deterministic integrity and tamper-evidence claims within the
described call path. It does not support stronger authentication claims:

- final-evidence `package_digest` and XY compatibility identity use FNV-derived
  `u64` values, not collision-resistant hashes;
- receipt and transaction identities use unkeyed SHA-256;
- a party already authorized to construct all inputs can construct a new consistent
  digest; and
- no external signature, MAC, hardware attestation or remote trust root is established
  by these source paths.

Accordingly, this finding does not describe an end-to-end cryptographic attestation or
an unforgeable signer identity.

## Current production versus POC advancement probe

| Boundary | Current v189 production | POC advancement probe |
|---|---|---|
| Inbound material | Recomputes SHA-256 over actual Light bytes and validates the final-evidence package | Recompute candidate, evidence and rule digests before accepting a continuity entry |
| LAW-0 state | Separate caller-maintained ledgers; Shadow records one new local `1 -> 0` marker | One canonical, hash-linked transcript with explicit previous head, sequence, phase, role, before and after |
| Phase authority | Phase is caller-supplied data in the current ledger | State-derived Coordination -> Reasoning -> Verification transitions; evidence transport does not transfer sovereignty |
| Receipt | Verification receipt binds XY/PD/trust/YX but not LAW-0 or commit identity | Harness-only continuity receipt binds session, cycle, evidence, rule version, previous head and current head |
| Transaction | Local `law0_digest` and receipt digest are siblings inside the transaction hash | Feed the verified terminal head into the existing transaction-binding behavior and require a nonzero verified head |
| Publication | Production response is returned after commit, but omits transaction and LAW-0 identities | Publish a connection proof only after the commit adapter reports success; backend fault yields no proof or state mutation |
| Claim | Local close binding and durable ordering | Feasibility of the missing connection, explicitly not a claim that v189 already implements it |

The advancement probe belongs only in this POC. It must not modify v189, and it must be
labelled `ADVANCEMENT_PROBE` or `HARNESS_ONLY_CONNECTION`, not production code, while
retaining the `POC` artifact classification. If source-exact v189 files are copied into the probe, their byte identity and
their harness-only dependency adapters must be reported separately.

## Experimental acceptance matrix for the connection probe

The smallest useful executable probe should distinguish current success from the
remaining failure:

| Experiment | Required result |
|---|---|
| Valid Coordination -> Reasoning -> Verification chain with matching inbound digests | accepted; one terminal nonzero head |
| Payload byte changed while declared digest remains stale | rejected before transaction construction |
| Cross-step reset, reordered phase or transition after terminal close | rejected without state mutation |
| Terminal LAW-0 digest changed after transaction construction | existing transaction validation rejects it |
| Receipt field changed after transaction construction | existing transaction validation rejects it |
| Transaction consistently built with zero LAW-0 digest | record current acceptance as a failure/gap, not a success |
| Injected backend failure | no published connection proof and no committed in-memory state |
| Retry of the same transaction | same transaction identity and receipt, no duplicate state |
| Same identity with a conflicting cycle | fail closed |

An executable green test may intentionally assert a reproduced failure, such as the
current acceptance of a zero LAW-0 digest. The result label must remain `FAIL` or
`GAP_REPRODUCED`; test-process success must not relabel the constitutional result.

## What was connected to achieve materialization

In current v189, materialization was achieved by connecting:

1. actual Light input bytes to a recomputed inbound SHA-256 gate;
2. accepted final-evidence identity to the XY-derived verification receipt;
3. the local constant LAW-0 close digest and the receipt digest as separate inputs to
   one transaction identifier;
4. that transaction to an append/flush/`sync_all` WAL path; and
5. successful durable commit to subsequent runtime mutation and response publication.

The POC advancement must connect the one boundary that remains open: a verified
upstream continuity head to Shadow's terminal close, then to the transaction and to a
post-commit connection proof. It must do so without allowing Light, Quantum or Shadow
to assume GCL sovereignty or a sibling authority's jurisdiction.

## Claim boundary

This finding supports the following statements:

- the required inbound validation, receipt hashing, transaction hashing, durable WAL
  ordering and fail-closed fault patterns exist in v189 source;
- Shadow's locally generated LAW-0 report digest is bound into its transaction; and
- a small external POC can test the missing connection without changing v189.

It does not establish:

- a current global Light-to-Quantum-to-Shadow LAW-0 transcript;
- provenance of the caller-reported candidate spaces;
- a collision-resistant end-to-end chain across the FNV compatibility boundaries;
- signer authentication, remote attestation or resistance to an already-authorized
  constructor;
- that the public receipt independently proves durable commit;
- full-system production readiness, security certification, semantic correctness,
  truth, patent novelty, patent invalidity, legal priority or grant eligibility.
