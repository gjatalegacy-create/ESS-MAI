# Results

## Executive matrix

| Experiment layer | Observation | Result |
|---|---|---|
| Surgical extraction | 20 whole files + one exact 124-line excerpt verified | PASS |
| Cargo resolution | all local manifests stay inside the capsule | PASS |
| Locked offline build | workspace and all targets | PASS |
| Disclosed tests | 84 passed, 0 failed | PASS |
| GCL phase authority | wrong-phase directives rejected | PASS |
| Besa process boundary | separate Shadow selector invoked | PASS |
| Empty Besa selection | 0 positive, 0 negative, complete emptiness accepted | PASS |
| Asht process boundary | second Shadow selector invoked | PASS |
| Empty-state relevance | exact positive candidate absent | FAIL-CLOSED |
| Post-Asht probe | not reached from empty state | FAIL |
| Production Shadow commit | not executed | NOT RUN |
| Exact-pair control | 1 positive + 1 negative; Asht passes | CONTROL PASS |
| Complete production E2E | outside disclosed boundary | NOT CLAIMED |

## Test totals

```text
gcl-constitution                    11 passed
shadow-contracts                   13 passed
ess-mai-quantum-surgical           44 passed
ess-mai-system-poc-003 unit        10 passed
cold_start_reachability             6 passed
TOTAL                              84 passed; 0 failed
```

The build completes with three dead-code warnings from preserved paths not called by this bounded experiment. The warnings are retained as scope evidence; they are not compilation failures.

## Empty-state observations

All three fresh runs emitted:

```text
artifact=ESS-MAI-SYSTEM-POC-003
mode=EMPTY_COLD_START
gcl_parent_authority=true
shadow_selection_processes=2
besa_selection=positive:0 negative:0 accepted_empty:true
asht_succeeded=false
asht_error=request-bound relevance found no exact positive candidate
post_asht_probe_reached=false
production_shadow_commit_executed=false
classification=COLD_START_REACHABILITY_GAP_REPRODUCED
```

This is an experimental failure of cold-start completion and a successful reproduction of the boundary.

## Positive causal control

The harness-only exact-pair mode emitted:

```text
mode=EXACT_PAIR_POSITIVE_CONTROL
gcl_parent_authority=true
shadow_selection_processes=2
besa_selection=positive:1 negative:1 accepted_empty:false
asht_succeeded=true
asht_error=NONE
post_asht_probe_reached=true
production_shadow_commit_executed=false
classification=POST_ASHT_REACHABILITY_CONTROL_PASS
```

The control shows that the disclosed Asht gate can pass when its exact evidence precondition exists. It does not show that the harness candidates are valid production knowledge and is not proposed as the fix.

## Causal interpretation

```text
same GCL/Light/Besa/Asht path
+ empty candidate state   → fail-closed before final-Shadow reachability
+ exact-pair test control → post-Asht reachability
= missing generation-zero evidence is the bounded causal variable
```

This interpretation is limited to the disclosed POC. It does not exclude later blockers in the private runtime.

## Formatting boundary

`cargo fmt --all -- --check` is not a release gate for this capsule because the current rustfmt would rewrite byte-identical production extracts. Those diffs are preserved rather than “fixed.” Formatting is therefore recorded as `NOT_APPLIED_TO_PRODUCTION_EXTRACTS`, while Cargo build, tests, and runs are the executable gates.
