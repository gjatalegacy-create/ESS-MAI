# Theory POC Protocol

## Classification rule

This artifact is a POC for one falsifiable theory: globally continuous LAW-0 candidate-space non-expansion under GCL.

Its classification does not depend on all experiments producing the desired constitutional outcome. A bounded negative result remains POC evidence when the source, protocol, and counterexample are reproducible.

## Required composition

Every theory POC in this publication series must contain:

1. one bounded theory and falsification rule;
2. an isolated minimal production-source closure;
3. explicit separation between extracted source and experimental glue;
4. a clean Cargo build;
5. at least one materialized success;
6. at least one honest failure or a justified statement that none was found;
7. a failure-to-advancement method that preserves the architecture;
8. prior-art and claim boundaries;
9. source and artifact hashes;
10. reproducibility evidence.

## Separation from the future system POC

This package tests the LAW-0 theory through its smallest real implementation surface. It is not the later full-system extraction of the current ESS-MAI runtime.

The system POC must be a distinct capsule/directory inside the existing canonical ESS-MAI repository, never a separate repository. It must state which production paths it executes end to end and must not retroactively alter the evidence in this theory POC.

## Evidence labels

- `MATERIALIZED`: present and positively executed or directly verified.
- `FAIL`: a required invariant is contradicted by a reproducible accepted case.
- `NOT_MATERIALIZED`: no current implementation path was found for the bounded requirement.
- `COMPONENTS_PRESENT_CONNECTION_UNLINKED`: the necessary source mechanisms exist, but the required production binding between them does not.
- `NOT_RUN`: present or mapped behavior was not executed by this capsule.
- `PARTIAL_MATERIALIZATION`: at least one core mechanism is real, while one or more required global boundaries fail or are absent.

## Integrity rule

Extracted source remains byte-identical. Any future corrective implementation must be a new version with a new hash manifest and results, not a silent replacement of this snapshot.
