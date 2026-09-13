# Experimental results

## Positive controls

| Experiment | Expected | Observed | Verdict |
|---|---:|---:|---|
| First cycle, no prior marker | PRO may activate | `true` | PASS |
| Valid required sealed history | NK status `READY` | `READY` | PASS |
| Valid history entry count | 1 | 1 | PASS |
| Repeated known failure | hard-blocked | `true` | PASS |
| Repeated known failure score | 0.000 | 0.000 | PASS |

## Negative controls

| Falsification attempt | Required result | Observed | Verdict |
|---|---:|---:|---|
| Corrupt required history | PRO refused | `false` | PASS |
| Missing required history | PRO refused | `false` | PASS |
| Unsealed body after sealed marker | downgrade refused | `false` | PASS |
| Full negative receipt on wire | present | absent; Boolean only | GAP CONFIRMED |

## Runtime receipt

```text
ARTIFACT_TYPE=THEORY_POC
THEORY=LIVING_NEGATIVE_KNOWLEDGE_VERIFIED_REENTRY
FIRST_CYCLE_PRO_ACTIVATED=true
VALID_HISTORY_STATUS=READY
VALID_HISTORY_ENTRIES=1
KNOWN_FAILURE_HARD_BLOCKED=true
KNOWN_FAILURE_SCORE=0.000
CORRUPT_HISTORY_STATUS=NOT_READY
CORRUPT_HISTORY_PRO_ACTIVATED=false
MISSING_HISTORY_STATUS=DEGRADED
MISSING_HISTORY_PRO_ACTIVATED=false
DOWNGRADE_STATUS=NOT_READY
DOWNGRADE_PRO_ACTIVATED=false
FULL_NEGATIVE_RECEIPT_WIRE_MATERIALIZED=false
NEGATIVE_PERSISTED_BOOLEAN_ONLY_GAP=true
EXPERIMENT_STATUS=PASS
```

## Interpretation

`EXPERIMENT_STATUS=PASS` means the pre-registered bounded invariants held. It
does not mean every link of Living Negative Knowledge is complete. The
negative wire result is part of the result and determines the advancement
method.

```text
verified materialization
+ reproduced omission
+ authority-preserving closure method
= experimental advancement
```

## Parallel knowledge-domain evidence

Four focused native runs independently passed the closed verdict-domain
controls: constructive `(1,1)`, rigorous negative `(0,0)` with proof,
refusal of mixed pairs, and refusal of out-of-domain bits. Source coordinates
then map the two admitted verdicts to distinct `Primitive` and `Negative`
knowledge writes in one Vault transaction.

These runs establish the two-branch domain and its separation. They are not
presented as a detached positive-history re-entry experiment, and overlapping
filtered test runs are not added into an inflated unique-test total.
