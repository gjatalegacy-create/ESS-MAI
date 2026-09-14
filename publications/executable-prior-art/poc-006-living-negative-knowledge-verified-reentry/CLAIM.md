# POC-006 falsifiable claim

## CLAIM

> Does the existing ESS-MAI v189 negative-knowledge import and filtering code
> cause valid, required, sealed history to enter `READY` state and hard-block a
> semantically matching later candidate, while preventing reasoning when the
> required history is missing, corrupt, or downgraded?

This is a bounded verified-re-entry claim. It is not a claim that every
upstream persistence and downstream receipt edge is reproduced in one
detached process chain.

## REQUIRED OBSERVABLES

- without a history requirement, the baseline candidate may reach filtering;
- a valid sealed blob imports exactly one negative entry and sets NK readiness
  to `READY`;
- a matching later candidate is hard-blocked and receives score `0.000`;
- a corrupt required blob prevents PRO activation and yields `NOT_READY`;
- missing required history prevents PRO activation and yields `DEGRADED`;
- an unsealed body after a sealed-history marker is detected as downgrade,
  yields `NOT_READY`, and prevents PRO activation;
- all three independent deterministic launches produce the same receipt.

## FORBIDDEN INFERENCES

The detached experiment does not establish a full Shadow-process termination
and reopen cycle, an independently verifiable negative-commit receipt, a
symmetric positive-history re-entry path, cryptographic collision resistance
for the FNV seal, universal avoidance of every failure, hostile-machine
durability, or closure of the complete ESS-MAI v189 architecture.

The Boolean `negative_persisted` wire finding is source evidence, not a value
measured by this detached runtime harness. Native WAL tests are component
evidence, not a substitute for a cross-process end-to-end recovery receipt.

## FALSIFICATION CONDITION

The narrow claim is falsified if valid sealed history cannot become `READY`,
if the imported matching failure does not change the candidate to hard-block
with score zero, or if missing/corrupt/downgraded required history is allowed
to activate PRO.

The larger Living Negative Knowledge lifecycle remains only partially
materialised while the wire carries a Boolean instead of a complete commit
receipt and while the detached POC does not execute terminate/reopen recovery
through the authoritative Shadow Vault sink.
