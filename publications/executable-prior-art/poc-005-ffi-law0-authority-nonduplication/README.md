# POC-005 — FFI LAW-0 Authority Non-Duplication

- **Artifact class:** `THEORY_POC`
- **Experimental classification:** `PARTIAL_POC` — local property ready, global theory not materialised
- **Author:** Bledar Gjata
- **Project / organization:** ESS-MAI / Gjata Legacy
- **Verification date:** 2026-09-14
- **License:** Apache-2.0

## Falsifiable claim

A C-compatible handle may be copied bit-for-bit, but copying the handle must
not copy or replenish the authority represented by the Rust-owned slot:

```text
Copy(CapHandle) != Copy(Authority)
fresh(slot) --successful CAS(true,false)--> consumed(slot)
consumed(slot) --any replay--> AlreadyConsumed
```

## What the experiment established

The focal v189 file is compiled byte-for-byte in a detached Cargo workspace.
The runtime experiment observes:

- a forged nonce is refused with `-3`;
- the first valid, non-empty use crosses the one-shot authorization gate once;
- that call then stops at the existing `CompatibilityHold (-8)`;
- copied and repeated handles return `AlreadyConsumed (-1)`;
- eight concurrent bit-identical copies produce exactly one post-authorization
  result and seven replay refusals;
- an empty payload returns `-5` only after consuming authority, and its replay
  returns `-1`;
- two fresh generations for the same module and payload both reach `-8`, a
  published counterexample to stronger semantic idempotency.

The positive result is real: identifier-local handle replay is prevented. The
negative result is equally real: this legacy FFI surface does **not** complete a
knowledge write. Its source intentionally requires a committed constitutional
verdict.

## Why a green Cargo result does not hide the gap

The experiment is expected to pass only when `-8` remains visible. A test that
expected `0` would falsely claim end-to-end persistence. The scientific result
is therefore:

```text
materialized one-shot authority
+ reproducible direct-write hold
+ explicit semantic-replay boundary
= bounded advancement method
```

## Verified execution

- native v189 focal tests: **8 passed, 0 failed**;
- detached release build: **pass**;
- strict detached release tests: **15 passed, 0 failed**;
- release runtime repeatability: **3/3**, `EXPERIMENT_STATUS=PASS`;
- source identity: **3/3 focal files byte-identical**;
- v189 source files modified: **0**;
- Cargo target: external to this capsule.

Start with [CLAIM.md](CLAIM.md), [SOURCE_RUNTIME_MAP.md](SOURCE_RUNTIME_MAP.md)
and [EXPERIMENT_PROTOCOL.md](EXPERIMENT_PROTOCOL.md). Results and boundaries
are in [RESULTS.md](RESULTS.md), [FINAL_VERDICT.txt](FINAL_VERDICT.txt),
[SCIENTIFIC_REPORT.md](SCIENTIFIC_REPORT.md), [PRIOR_ART.md](PRIOR_ART.md), and
[REPRODUCIBILITY.md](REPRODUCIBILITY.md).
