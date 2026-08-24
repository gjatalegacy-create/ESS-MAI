# Verification Results

Verification date: 2026-08-23

Artifact class: `THEORY_POC`

Final classification: `PARTIAL_MATERIALIZATION`

## Build and test outcome

- Cargo: `cargo 1.98.0 (797e8a9bc 2026-08-05)`.
- rustc: `rustc 1.98.0 (88d9e12ae 2026-08-18)`.
- Clean offline locked build: pass.
- Extracted GCL production tests: 11 passed, 0 failed.
- Extracted Shadow receipt tests: 2 passed, 0 failed.
- POC harness tests: 6 passed, 0 failed.
- Total executable unit tests: 19 passed, 0 failed.
- Five final-source runs: 5/5 exit code 0.
- Program output across the five runs: identical.
- Standalone temporary-stage build/test with no v189 and no preexisting target: pass.
- Format check: pass.
- Network downloads: none.

Cargo emitted one non-fatal environment warning: it could not canonicalize `<USER_PROFILE>`. The warning did not affect compilation, tests, program output, or source identity.

## Experiment matrix

| Case | Input | Required by full theory | Observed | Verdict |
|---|---|---|---|---|
| Reference chain | `16 -> 3 -> 2 -> 1 -> 1 -> 0` | accept and collapse | accepted; 5 steps; collapsed | PASS |
| Direct expansion | `2 -> 5` | reject without mutation | rejected; empty ledger preserved | PASS |
| Nonempty rejection atomicity | seed `10 -> 5`, then `5 -> 6` | reject; preserve seed state | rejected; length, current state, and report unchanged | PASS |
| Cross-step continuity | `16 -> 1`, then `100 -> 50` | reject second step | second step accepted; current state grows `1 -> 50` | FAIL |
| Report completeness | same discontinuous chain | expose every boundary | injected `before=100` absent from report | FAIL |
| Phase/terminal order | `Verification 1 -> 0`, then `Coordination 100 -> 50` | reject reopening | accepted; ledger changes from collapsed to uncollapsed | FAIL |
| Non-negative domain | `1 -> -1` | reject | accepted and classified collapsed | FAIL |
| Finite domain | `+infinity -> +infinity` | reject | accepted | FAIL |
| Shadow local durable close | local `Verification 1 -> 0` marker | bind close to committed cycle | source mapping shows the report digest entering cycle, transaction ID, WAL and replay-validation paths; full runtime not executed here | STATIC SOURCE MAPPED / NOT E2E RUN |
| Shadow public receipt binding | change only LAW-0 report/digest | outward receipt must change | byte-identical receipt algorithm returns the same ID because LAW-0 is not an input | FAIL / UNLINKED |
| Harness-only connection | existing receipt ID + LAW-0 digest | tamper changes connection identity | domain-separated connection digest changes and rejects the altered digest | ADVANCEMENT PROBE PASS |
| Cross-platform continuity | Light -> Quantum -> Shadow | one verifiable transcript | components and transport paths exist, but the continuity head is not connected across the boundaries | COMPONENTS PRESENT / UNLINKED |

## Why all harness tests pass while the theory is partial

The counterexample tests assert the behavior that exists now. For example, `cross_step_discontinuity_is_reproducible` passes only when the current API accepts the discontinuous reset. A green Cargo test therefore reproduces the gap; it does not relabel that gap as constitutional success.

## Stable runtime observations

Each of the five runs emitted:

```text
ARTIFACT_TYPE=THEORY_POC
SOURCE_CORE_IDENTITY=VERIFIED_6_OF_6_SHA256
REFERENCE_CHAIN_COLLAPSED=true
LOCAL_EXPANSION_REJECTED=true
NONEMPTY_REJECTION_ATOMIC=true
DISCONTINUITY_ACCEPTED=true
DISCONTINUITY_EXPANDED_GLOBAL_STATE=true
DISCONTINUITY_BEFORE_HIDDEN_FROM_REPORT=true
PHASE_REGRESSION_ACCEPTED=true
COLLAPSED_AFTER_PHASE_REGRESSION=false
NEGATIVE_SPACE_ACCEPTED=true
INFINITE_SPACE_ACCEPTED=true
LOCAL_TUPLE_NON_EXPANSION_CHECK=PASS
GLOBAL_CONTINUITY_ENFORCEMENT=FAIL
PHASE_ORDER_ENFORCEMENT=FAIL
UNCERTAINTY_DOMAIN_ENFORCEMENT=FAIL
PRODUCTION_END_TO_END_EXECUTION=NOT_RUN
SHADOW_LOCAL_LAW0_DURABLE_PATH=SOURCE_MATERIALIZED
SHADOW_VERIFICATION_RECEIPT_PATH=SOURCE_MATERIALIZED
CURRENT_SHADOW_RECEIPT_CHANGES_WITH_LAW0=false
PROPOSED_CONNECTION_CHANGES_WITH_LAW0=true
PROPOSED_CONNECTION_REJECTS_LAW0_TAMPER=true
LAW0_DIGEST_TO_VERIFICATION_RECEIPT=UNLINKED
CROSS_PLATFORM_LEDGER_CONTINUITY=COMPONENTS_PRESENT_CONNECTION_UNLINKED
MATERIALIZATION_STATUS=PARTIAL
EXPERIMENTAL_STATUS=SUCCESS_AND_FAILURE_REPRODUCED
POC_CLASS=THEORY_POC
```

## Result equation

```text
materialized local safety and Shadow durable co-binding
+ reproducible global under-enforcement
+ executable reproduction showing that the tested outward receipt omits LAW-0
+ source-backed, architecture-preserving connection seam
= bounded advancement method
```

`SOURCE_CORE_IDENTITY=VERIFIED_6_OF_6_SHA256` is a literal program-output label for local hash comparison, not external certification. Likewise, `SHADOW_LOCAL_LAW0_DURABLE_PATH=SOURCE_MATERIALIZED` and `SHADOW_VERIFICATION_RECEIPT_PATH=SOURCE_MATERIALIZED` summarize static source mapping outside the six-file executable extraction; the full Shadow durability path was not executed by this capsule.

The evidence supports continued engineering. It does not support a claim that the global law is already complete.
