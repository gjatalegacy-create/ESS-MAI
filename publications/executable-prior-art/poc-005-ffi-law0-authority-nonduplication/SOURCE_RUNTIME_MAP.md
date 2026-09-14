# POC-005 source-to-runtime map

## Authority hierarchy

```text
GCL / ESS-MAI constitutional authority
  -> Shadow verdict and persistence jurisdiction
     -> LawConfirmedVerdict / VerificationReceipt
        -> bounded FFI compatibility gate
           -> Rust-owned CapSlot authority
              -> C-visible CapHandle representation
```

The handle is data below the authority owner. Copying the child
representation does not copy the parent-owned `CapSlot`. Conversely, the
local gate is not allowed to assume the superior verdict or Vault authority.

## Runtime edges

| Stage | Source file / coordinates | Symbol | Producer | Actual caller | Input | Transformation | Output | Consumer / sink | Authority owner | Mutation / persistence | Failure branch | Existing test | Edge status |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Define transport | `shadow/src/sovereign_ffi_gate.rs:61-67` | `CapHandle` | Rust ABI | external caller/test | generation + nonce | `repr(C)`, `Copy` | two words | `sovereign_validate_and_write` | no independent authority | none / none | invalid zero handle | copy/replay tests | `PROVEN_SOURCE_EDGE` |
| Issue authority | `shadow/src/sovereign_ffi_gate.rs:118-131,242-249` | `SovereignGate::issue`, `sovereign_issue_capability` | Rust registry | in-repository callers found only under tests; foreign caller possible when explicit feature is enabled | module seal | allocate slot, increment generation | handle + hidden live slot | foreign/test caller | `SovereignGate` | insert `CapSlot` / process memory only | poisoned lock is reported then recovered | focal native suite | `TEST_ONLY_EDGE` for in-repository closure |
| Validate identity | `shadow/src/sovereign_ffi_gate.rs:137-146,273-293` | `validate_and_burn` | caller presenting handle | `sovereign_validate_and_write` | handle | slot lookup + generation + nonce checks | `LgcToken` or error | `sovereign_commit` or error code | hidden `CapSlot` | no mutation until CAS / none | `-2`, `-3` | unknown/forged tests | `PROVEN_SOURCE_EDGE` inside enabled surface |
| Consume authority | `shadow/src/sovereign_ffi_gate.rs:143-146` | `AtomicBool::compare_exchange` | valid slot | `validate_and_burn` | `true` state | atomic `true -> false` | one token or `AlreadyConsumed` | commit or `-1` | hidden `CapSlot` | irreversible in-process burn / not durable | `-1` | replay + concurrent POC | `PROVEN_SOURCE_EDGE` |
| Borrow payload | `shadow/src/sovereign_ffi_gate.rs:300-337` | `sovereign_commit`, `borrow_ffi_payload` | successful validation | `sovereign_validate_and_write` | token, handle, pointer, length | borrowed byte slice; FNV input id; lineage | traceable lineage or error | compatibility boundary | Shadow compatibility function | authority already consumed / none | `-4`, `-5` | empty/pointer tests | `PROVEN_SOURCE_EDGE` |
| Enforce superior boundary | `shadow/src/sovereign_ffi_gate.rs:310-320` | `sovereign_commit` | traceable local result | same function | lineage | refuse direct write without committed verdict | `-8 CompatibilityHold` | terminal return; no Vault sink | superior Shadow/GCL verdict path | zero knowledge mutation / none | always `-8` for non-empty traceable payload | non-empty hold test | `PROVEN_SOURCE_EDGE` |
| Seal verified output | `shadow/src/sovereign_ffi_gate.rs:164-183`; `shadow/src/shadow_gateway.rs:590-616` | `seal_verified_output` | confirmed Shadow verdict | `ShadowGateway` commit flow | `LawConfirmedVerdict` + cycle identity | issue and burn verification token, form receipt, build transaction | `VerificationReceipt`, `VaultTransaction` | `KnowledgeVault::commit_transaction` | Shadow under GCL | committed transaction / Vault | seal or Vault error | native Shadow tests | `PROVEN_SOURCE_EDGE`, not executed by detached FFI experiment |
| Detached falsification | `experiment/src/main.rs` | `run_experiment` | POC harness | POC binary/test | exact copied focal module | baseline, replay, tamper, race, semantic counterexample | machine-readable receipt | process exit/test assertion | no new production authority | only copied module's process state / none | nonzero test/process failure | one Cargo test + runtime | `TEST_ONLY_EDGE` |

## Caller-closure finding

Repository-wide symbol search found no non-test in-repository caller of
`sovereign_issue_capability` or `sovereign_validate_and_write`. Both exports
are compiled only under `test` or `legacy_ffi_compat`. Therefore POC-005 makes
no default-production FFI claim. The separate `seal_verified_output` route is
called by the real Shadow transaction path, but the detached experiment does
not substitute that route for the held direct-write surface.

## Authoritative boundary

```text
copyable handle
  -> hidden slot validation
  -> one in-process burn
  -> lineage construction
  -> CompatibilityHold (-8)
  -X-> no direct KnowledgeVault commit
```

`-X->` is an experimentally visible absence of the production sink, not a
request to weaken the authority hierarchy.
