# Negative results as governed research evidence

Status date: 2026-09-14

## Principle

Negative experimental outcomes advance the method when they are preserved with provenance, the tested boundary, a falsifiable observable, and an architecture-preserving next experiment. They are not defects to hide behind an overall green test count.

Negative knowledge (`K−`) is also not a Boolean `false`. Within the documented ESS-MAI formulation, K− is a typed sibling of constructive knowledge (`K+`) under the same superior governance boundary. K+ records a supported route; K− records a rigorously established failed route with proof and context. Unknown, missing, corrupt, structurally refused, or unexecuted states remain holds or gaps rather than being converted into K−.

## Sibling evidence channels

| Channel | Admission | Research effect | Invalid substitution |
| --- | --- | --- | --- |
| Constructive `K+` | Evidence survives the constructive verdict path | Reinforce or extend a supported route | “Test process exited zero” is not automatically constructive domain knowledge |
| Rigorous negative `K−` | A failed route is verified with its evidence, scope, and limitations | Exclude or penalize the verified route in a later cycle | A Boolean, exception, missing file, or untested path is not automatically negative knowledge |
| Hold / gap | Evidence is missing, invalid, out of domain, or execution was not performed | Block authority and define the next experiment | A hold must not be relabeled K+ or K− to simplify control flow |

The public topology and its current asymmetry are documented at `publications/executable-prior-art/poc-006-living-negative-knowledge-verified-reentry/POSITIVE_NEGATIVE_PARALLELISM.md`.

## Negative-result ledger

### POC 003 — cold-start completion does not occur

- **Observed:** from an empty candidate state, Besa accepts complete emptiness, Asht fails closed for lack of an exact positive candidate, the post-Asht probe is not reached, and no production Shadow commit executes.
- **Why it is knowledge:** the exact-pair control traverses the same disclosed gate and reaches the post-Asht probe, localizing the tested causal variable to generation-zero evidence availability.
- **What it does not mean:** fail-closed behavior is not itself a defect in authority policy, and the harness control is not a production seed or fix.
- **Advancement:** test a GCL-authorized, one-use genesis transaction that remains adjudicated and persisted by Shadow.
- **Evidence:** `publications/executable-prior-art/poc-003-system-cold-start-reachability/RESULTS.md` and `FAILURE_TO_ADVANCEMENT.md`.

### POC 004 — local non-expansion is not global continuity

- **Observed:** cross-step reset is accepted; terminal Verification can be followed by Coordination; negative and infinite values are accepted; a supplied boundary is omitted from `chain_report`; the outward receipt does not change when only the LAW-0 report changes.
- **Why it is knowledge:** tests intentionally pass when they reproduce these counterexamples. The green harness result preserves current behavior instead of asserting that the global law passed.
- **What it does not mean:** local rejection and source-mapped durable co-binding are not absent; they remain positive evidence at narrower scopes.
- **Advancement:** make the ledger head private, enforce typed phase order and terminal immutability, use a finite non-negative integer domain, serialize complete transitions, and bind the canonical head to the post-commit receipt.
- **Evidence:** `publications/executable-prior-art/poc-004-gcl-law0-global-continuity/RESULTS.md` and `FAILURE_TO_ADVANCEMENT.md`.

### POC 005 — generation-local success does not imply semantic or durable non-duplication

- **Observed:** two fresh generations for the same module and payload both cross local authorization; a locally authorized non-empty call ends at `CompatibilityHold (-8)` and performs no Vault write; restart durability is unexecuted.
- **Method/tooling negative:** the first strict all-target build failed because authored adapter shapes were incomplete. The focal extracted source was kept unchanged; the adapter-only closure was then tested.
- **Reproducibility negative:** behavioral receipts matched, while binaries built in different roots did not have identical SHA-256 values. Bit-for-bit binary reproducibility is not established.
- **Why it is knowledge:** these results bound the valid statement to one issued generation and demonstrate that preserving the superior verdict boundary is part of success, not a missing green return code.
- **Advancement:** atomically consume a durable semantic authorization identity at an idempotent, verdict-authorized Shadow transaction sink and test crash/restart/replay.
- **Evidence:** `publications/executable-prior-art/poc-005-ffi-law0-authority-nonduplication/RESULTS.md`, `FINAL_VERDICT.txt`, and `FAILURE_TO_ADVANCEMENT.md`.

### POC 006 — verified re-entry is incomplete across the persistence boundary

- **Observed:** the cross-subsystem response carries `negative_persisted: bool`; it does not independently bind the negative asset, transaction, cycle, GCL identity, export digest, and commit result. Detached terminate/reopen across authoritative Shadow storage and the next Quantum process was not executed.
- **Scope negative:** K+ is source-evidenced and natively tested as a distinct branch, but POC 006 executes only K− re-entry. A symmetric positive-history re-entry claim would overstate the capsule.
- **Integrity negative:** the reproduced FNV seal detects the tested corruption and downgrade cases but is not claimed to provide cryptographic collision resistance.
- **Reproducibility negative:** behavioral receipts matched across clean builds, but bit-for-bit executable identity was not established.
- **Why it is knowledge:** the missing/corrupt/downgraded controls show fail-closed behavior; the Boolean receipt and restart omissions identify the smallest missing cross-cycle proof.
- **Advancement:** issue a typed receipt only after Shadow commit, bind the export to it, verify it before Quantum activation, and execute a real terminate/reopen/re-import successor POC.
- **Evidence:** `publications/executable-prior-art/poc-006-living-negative-knowledge-verified-reentry/RESULTS.md`, `FINAL_VERDICT.txt`, and `FAILURE_TO_ADVANCEMENT.md`.

## Repo-wide negative knowledge

The collection itself records a general limit: POC 003–006 are surgical public evidence, not the complete private v1.8.9 workspace. Source presence, test totals, CI configuration, and a release DOI do not establish full-system runtime closure, production readiness, security, formal correctness, patent novelty, or independent replication.

This limit is a useful research result because it prevents scope drift. It requires future claims to name the exact edge executed and stops a local pass from silently absorbing unresolved global behavior.

## Required record for a new negative result

Every new entry should include:

1. source version, public path, and integrity identity;
2. precondition, controlled input, expected observable, and actual observable;
3. whether the evidence is runtime, test, static source mapping, or external reference;
4. authority at which the route stopped and whether any persistent mutation occurred;
5. positive control, ablation, adversarial case, or explanation of why one is unavailable;
6. exact reproduction command and environment;
7. narrow supported conclusion and forbidden inference;
8. smallest architecture-preserving next experiment.

Historical negative results must remain citable after a successor succeeds. Advancement appends evidence; it does not rewrite the failed observation into a success.
