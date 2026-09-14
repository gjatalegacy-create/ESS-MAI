# Experimental results

## Cargo and repeatability gate

| Gate | Observed | Verdict |
|---|---:|---|
| strict release build, workspace/all targets/locked | exit 0 | PASS |
| source-core tests | 22/22 | PASS |
| experiment tests | 4/4 | PASS |
| shadow-contract tests | 4/4 | PASS |
| total detached tests | 30/30 | PASS |
| independent principal experiment launches | 3/3 identical | PASS |

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
RUN0_INSTRUMENT_VALIDATION=PASS
BASELINE_NO_HISTORY_PRO_ACTIVATED=true
TARGET_VERIFIED_HISTORY_HARD_BLOCKED=true
ABLATION_MISSING_REQUIRED_HISTORY_PRO_ACTIVATED=false
ADVERSARIAL_CORRUPT_HISTORY_PRO_ACTIVATED=false
ADVERSARIAL_DOWNGRADE_PRO_ACTIVATED=false
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
RECEIPT_GAP_EVIDENCE_CLASS=SOURCE_INSPECTION_NOT_RUNTIME
DETACHED_RESTART_RECOVERY=NOT_EXECUTED
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

## EXPERIMENTAL SUCCESS

Valid required sealed history became `READY`, imported one failure, and
changed the matching later candidate from the baseline path to hard-blocked
with score zero. Missing, corrupt and downgraded required history prevented
PRO activation. The strict release build, all 30 tests and all three
independent launches passed with no unexpected variance.

## EXPERIMENTAL FAILURE

The detached POC does not execute Shadow commit, process termination, Vault
reopen and subsequent Quantum import as one chain. Native WAL recovery passes
only at component scope. The current cross-subsystem response also exposes
`negative_persisted` as a Boolean rather than a typed, independently
verifiable negative-commit receipt. K+ is source-evidenced and natively tested
but is not executed as symmetric detached positive-history re-entry here.

## ADVANCEMENT METHOD

After the existing Shadow transaction commits, emit one typed
`NegativeCommitReceiptWire` binding the asset token, transaction/cycle,
GCL identity, export digest and commit result. Quantum must verify that receipt
before importing the sealed history. A successor POC must terminate Shadow,
reopen the real Vault, launch the next Quantum cycle and reproduce the effect.
This connection preserves Shadow as the only persistence authority and
Quantum as a verification/reasoning consumer.

## Parallel knowledge-domain evidence

Four focused native runs independently passed the closed verdict-domain
controls: constructive `(1,1)`, rigorous negative `(0,0)` with proof,
refusal of mixed pairs, and refusal of out-of-domain bits. Source coordinates
then map the two admitted verdicts to distinct `Primitive` and `Negative`
knowledge writes in one Vault transaction.

These runs establish the two-branch domain and its separation. They are not
presented as a detached positive-history re-entry experiment, and overlapping
filtered test runs are not added into an inflated unique-test total.

## Runtime-witness binary identity

```text
BINARY_NAME=poc006-living-negative-experiment.exe
STANDARD_BUILD_BINARY_SHA256=e9801ea55ed4e8c42eb77553091463e92c60ffd32acfabadaefe0cba9096a722
DETACHED_BUILD_BINARY_SHA256=7aeabdd12020f0be4e91934cfd08d569a32979b74f85e36d42da1017f79dba03
FINAL_PRE_RELEASE_BINARY_SHA256=c9a6a24b4b3af742f96fd92562260458f833f90d6c1967b06ee614d9eabbacfb
STANDARD_AND_DETACHED_BINARY_BYTES=1318364
FINAL_PRE_RELEASE_BINARY_BYTES=1318356
BUILD_PROFILE=release
EXIT_STATUS=0
CODE_INPUT_TREE_SHA256=c60c8cb43da16c05c29e962838b21a6570ed67bf559ca3de5627e625c5e537bb
CARGO_LOCK_SHA256=9375b5e540ec1b283bd242a560d215133a66be27dd83cf9af16a3c90da51ca7d
CODE_INPUT_DIFFS_BETWEEN_PUBLIC_AND_DETACHED=0
BEHAVIORAL_RECEIPT_REPRODUCIBILITY=PASS
BIT_FOR_BIT_BINARY_REPRODUCIBILITY=NOT_ESTABLISHED
```

All three binaries produced the same required experiment receipt, but clean
builds in different roots did not produce the same executable SHA-256. The
binary is therefore recorded as a runtime witness, not claimed as a
bit-for-bit reproducible artifact, and is not stored in the capsule. A later
packaging POC can isolate path remapping and reproducible-linker controls; this
does not change the POC-006 theory verdict.
