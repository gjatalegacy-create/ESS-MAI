# System POC Protocol

## Classification

```text
ARTIFACT_CLASS=SYSTEM_POC
CAPSULE_VERSION=0.2.0
SOURCE_BASELINE=v1.8.9
EXPECTED_OUTCOME=SUCCESS_AND_FAILURE_REPRODUCED
```

This is a POC, never a product demo. A fail-closed cold-start result remains valid POC evidence when the disclosed path, preconditions, control, and counterexample are reproducible.

## Primary experiment

1. start with a fresh per-run handoff directory;
2. use an empty surgical Shadow selection state;
3. issue the correct GCL Coordination directive;
4. execute extracted Light Alnur/Besa behavior;
5. cross a separate Shadow process boundary;
6. execute extracted Quantum UCL/Asht behavior;
7. cross the Shadow selector boundary again;
8. observe whether the post-Asht probe is reached.

Success criterion for the stronger cold-start claim: post-Asht probe reached without fabricated prior knowledge.

Observed: not reached; Asht fails closed on missing exact positive candidate.

## Causal control

Repeat the same disclosed gate with one exact positive and one exact negative candidate supplied by the harness selector.

Control success criterion: Asht passes and the post-Asht probe is reached.

Observed: pass.

The control is an experimental instrument, not a production fix or authoritative knowledge source.

## Evidence rules

- production-exact files must remain hash-identical;
- excerpt identity must remain line- and hash-identical;
- new glue must be named as glue;
- build/test/run logs must exclude private absolute paths;
- no generated binary is part of the capsule;
- a passing counterexample test means reproducibility of the gap, not system completion;
- `production_shadow_commit_executed=false` must remain explicit.

## Separation from other POCs

POC 003 is a distinct capsule inside the same canonical ESS-MAI repository. POC 004 tests LAW-0 global continuity separately. Neither capsule retroactively changes the other's evidence.
