# ESS-MAI research domains and evidence boundary

Status date: 2026-09-14

Scope: public repository and the disclosed POC 003–006 capsules

Research owner identified by repository metadata: Bledar Gjata / Gjata Legacy

## Purpose

This document is a research-domain registry, not a statement that the complete ESS-MAI architecture is finished. It separates five kinds of statement that must not be collapsed:

1. **Established external theory** — an idea with relevant prior art cited by a capsule.
2. **Bledar Gjata / ESS-MAI contribution** — the project-specific formulation, composition, experiment, or engineering boundary.
3. **Claimed innovation** — a bounded proposition requiring comparison against prior art; it is not a legal novelty determination.
4. **Materialized in a POC** — behavior executed, or a source edge explicitly mapped, within the named public boundary.
5. **Research gap** — a counterexample, missing connection, unexecuted condition, or unresolved claim.

The capsule `PRIOR_ART.md`, `CLAIM_BOUNDARY.md`, `RESULTS.md`, and `FAILURE_TO_ADVANCEMENT.md` files are authoritative for their individual scopes. This registry does not enlarge them.

## Governance model for evidence

ESS-MAI describes GCL as the superior constitutional boundary and Light, Quantum, and Shadow as bounded roles for coordination, reasoning, and verdict/persistence. The public root workspace lists `light`, `quantum`, `shadow`, and `shadow-contracts` in `Cargo.toml`; that layout is architectural evidence, not proof that every runtime route is complete.

Constructive knowledge (`K+`) and rigorous negative knowledge (`K−`) are sibling evidence channels under governance:

```text
                         GCL / governed verdict boundary
                                      |
                        +-------------+-------------+
                        |                           |
                 constructive K+            rigorous-negative K-
                 supported route             verified failed route
                        |                           |
                        +------ next-cycle --------+
```

They are not `true` and `false`, and neither is a catch-all label for an unknown state. A refusal, missing proof, structural mismatch, or untested condition remains `HOLD`, `NOT EXECUTED`, or an open gap until its own admission rule is satisfied. The public source topology for this distinction is summarized in `publications/executable-prior-art/poc-006-living-negative-knowledge-verified-reentry/POSITIVE_NEGATIVE_PARALLELISM.md`; the capsule cites the closed verdict domain and distinct write branches in its `SOURCE_RUNTIME_MAP.md` and `RESULTS.md`.

## Domain registry

| Domain | Established external theory | Bledar Gjata / ESS-MAI contribution | Bounded claimed innovation | Public materialization | Research gap |
| --- | --- | --- | --- | --- | --- |
| Governed cold-start reachability | Initial-state reasoning, safety/liveness separation, fail-safe defaults, typestate, proof-carrying boundaries, and atomic transactions are established | POC 003 composes extracted GCL, Light/Besa, Quantum/Asht, and a separate Shadow selector to isolate generation-zero reachability | A GCL-authorized, single-use genesis route that carries typed empty-state evidence to the existing Shadow judgment/transaction authority | Empty-state path reproducibly fails closed before the post-Asht probe; an exact-pair harness control reaches the probe | No production Shadow commit is executed; the proposed genesis transaction is not implemented; later blockers are not excluded |
| GCL LAW-0 continuity | Candidate/version-space reduction, constraint pruning, ordered abstract domains, global data-flow consistency, typestate, termination measures, and append-only audit proofs are established | POC 004 distinguishes locally checked non-expansion from globally continuous governed collapse and maps existing platform seams | A canonical, phase-ordered, hash-linked continuity transcript carried across Light, Quantum, and Shadow and bound to the durable receipt | Local `after <= before` rejection and atomicity execute; a local Shadow durable binding and receipt path are source-mapped | Caller-controlled resets, phase regression, terminal reopening, invalid numeric domains, incomplete reporting, and an unlinked outward receipt remain |
| FFI authority non-duplication | Capability-as-authority, reference monitors, one-shot/linear authorization, compare-and-swap, nonce checking, and FFI safety are established | POC 005 composes a copyable C representation with Rust-owned one-shot state and preserves the superior verdict boundary | The project-specific constitutional composition in which FFI representation can cross a boundary without multiplying authority or inheriting persistence jurisdiction | Same-generation serial and concurrent replay refusal executes; valid use reaches the explicit `CompatibilityHold (-8)` | Fresh generations can reauthorize the same semantic action; consumption is not durable across restart; no FFI-to-Vault commit is executed |
| Living negative knowledge / verified re-entry | Failure-driven reminding, learning from planning failures, negative-memory systems, and positive/negative knowledge sharing have prior art | POC 006 connects sealed required negative history to a fail-closed readiness gate and future-candidate filtering, while retaining K+/K− separation | Typed, receipt-bound, governance-preserving re-entry of verified failure evidence across cycles, paired eventually with a distinct positive receipt | Valid required K− history becomes `READY` and hard-blocks a matching candidate; missing/corrupt/downgraded required history blocks PRO | The response exposes a Boolean persistence signal rather than a full commit receipt; detached terminate/reopen is absent; symmetric K+ re-entry is not executed |
| Evidence-bound executable research | Reproducible builds, falsification, controlled experiments, integrity manifests, and transparent negative results are established research practices | The POC collection packages claim boundaries, source maps, procedures, results, failures, and advancement methods as inspectable capsules | Treating a reproduced architectural failure as a first-class, governed research asset alongside supported behavior | POC 003–006 have disclosed Cargo procedures, hashes or extraction checks, results, and CI jobs | Maintainer-produced evidence is not independent replication; behavioral reproduction is distinct from bit-for-bit binary reproducibility; CI does not validate the full root architecture |
| Repo-wide bounded authority architecture | Hierarchical governance, separation of duty, reference monitors, transactions, and runtime verification are established fields | ESS-MAI names a GCL-rooted Light–Quantum–Shadow composition and publishes public Rust components plus surgical POCs | The exact project-specific composition and cross-role evidence contracts are research propositions, not presumed novelty | Root sources and workspace metadata expose the named roles; capsule source maps identify selected paths and boundaries | The public POCs exclude the complete private v1.8.9 workspace and do not close, certify, or fully materialize the repo-wide architecture |

## Public evidence locations

- Repository scope and current public claims: `README.md`.
- Workspace membership and root build comments: `Cargo.toml`.
- Canonical POC collection boundary: `publications/executable-prior-art/README.md`.
- POC 003: `publications/executable-prior-art/poc-003-system-cold-start-reachability/`.
- POC 004: `publications/executable-prior-art/poc-004-gcl-law0-global-continuity/`.
- POC 005: `publications/executable-prior-art/poc-005-ffi-law0-authority-nonduplication/`.
- POC 006: `publications/executable-prior-art/poc-006-living-negative-knowledge-verified-reentry/`.
- Public CI definition: `.github/workflows/poc-validation.yml`.
- Attribution and software identity: `CITATION.cff` and `codemeta.json`.

## Interpretation limits

The POC results are maintainer-recorded and CI-addressable evidence for disclosed scopes. They are not independent scientific replication, formal verification, security certification, patent analysis, or proof of production readiness. “Source-mapped” is not interchangeable with “executed end to end.” “POC ready” is not “global theory materialized.” Nothing in this registry declares v1.8.9 or the full ESS-MAI architecture closed.
