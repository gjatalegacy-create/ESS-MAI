# Experiment Protocol

## Objective

Execute the byte-identical v189 `UncertaintyLedger` in isolation and determine which parts of continuous LAW-0 uncertainty reduction are materially enforced.

## Controlled boundary

- Production core: unchanged extracted `gcl-constitution` crate and unchanged Shadow `verification_receipt.rs` algorithm.
- Experimental glue: `experiment/src/main.rs`.
- Dependencies: Rust standard library, local path crate, and locked cached `sha2` used by the extracted Shadow algorithm and harness connection probe.
- Network: offline.
- Full v189 runtime: not executed.
- Production source tree: read-only.

## Cases

### E1 — reference contraction

Input: `16 -> 3 -> 2 -> 1 -> 1 -> 0` across labelled phases.

Expected current implementation: accept five steps and return collapsed.

Purpose: establish positive materialization and preserve equality behavior.

### E2 — direct local expansion

Input: `2 -> 5`.

Expected: return `LawViolation`; ledger remains empty.

Purpose: demonstrate the existing local fail-closed gate in the bounded case.

### E3 — rejection atomicity on populated state

Input: accept `10 -> 5`, then attempt `5 -> 6`.

Expected: reject the second step; length, current state, and report remain unchanged.

Purpose: demonstrate that the local gate rejects before mutation in the bounded case.

### E4 — discontinuous reset

Input: accept `16 -> 1`, then attempt caller-supplied `100 -> 50`.

Required by global theory: reject the second step because 100 is not the current state 1.

Observed-current expectation: accept, set current state to 50, and omit 100 from `chain_report`.

### E5 — terminal phase regression

Input: accept `Verification 1 -> 0`, then attempt `Coordination 100 -> 50`.

Required by global theory: reject.

Observed-current expectation: accept and change `is_collapsed` from true to false.

### E6 — negative domain

Input: `Reasoning 1 -> -1`.

Required by count semantics: reject.

Observed-current expectation: accept and classify as collapsed.

### E7 — non-finite domain

Input: `Coordination +infinity -> +infinity`.

Required by measured-count semantics: reject.

Observed-current expectation: accept.

### E8 — Shadow receipt connection boundary

Input: one canonical Shadow receipt input; two distinct LAW-0 report digests produced with the same production domain tag; then a harness-only domain-separated connection of the existing receipt ID and each LAW-0 digest.

Required by the proposed outward continuity proof: changing the LAW-0 digest must change the published connection identity.

Observed-current expectation: the byte-identical Shadow receipt ID remains unchanged because `law0_digest` is not an input. The harness-only connection digest must change and must reject substitution of the altered LAW-0 digest.

Purpose: reproduce the exact unlinked boundary and test the smallest architecture-preserving connection without modifying v189. This probe does not claim production integration.

## Interpretation rule

- `PASS` means the observed behavior meets the bounded test requirement.
- `FAIL` means a counterexample to the stronger theory was reproduced.
- `COMPONENTS PRESENT / UNLINKED` means the source contains the required mechanisms but no current production binding joins them at the stated boundary.
- A Cargo test that asserts an observed failure passes when the failure is reproduced; it does not convert the constitutional result to PASS.

## Repetition

After a clean locked offline build and full test suite, run the final executable five times. Exit code, program fields, and ordering must be identical for all five runs.
