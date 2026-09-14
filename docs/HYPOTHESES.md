# ESS-MAI hypothesis registry

Status date: 2026-09-14

## Reading rule

Each hypothesis records the external foundation, the ESS-MAI contribution, any bounded innovation proposition, the evidence currently materialized, and the gap that could falsify or limit it. A passing test may confirm a deliberately negative observable; green Cargo output alone does not mean the wider theory passed.

## Normalized hypothesis and claim register

Statuses apply only to the exact statement in the same row. `SUPPORTED` means
supported in the named experiment, not proven universally. No row is marked
`PROVEN_PROPERTY`: the repository currently discloses executable tests and
counterexamples, not a machine-checked universal proof.

| ID | Type | Statement | Rationale | Implementation link | Test | Falsification condition | Current evidence | Status |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| H-003-O | `EXPERIMENTAL_OBSERVATION` | On the disclosed empty-state route, Asht's exact-positive requirement prevents the post-Asht probe from being reached. | Isolates the generation-zero liveness boundary without relaxing authority. | POC 003 `SOURCE_MAP.md` | Empty-state run repeated three times plus exact-pair control | An empty-state run reaches the probe under the same path/inputs, or the exact-pair control also fails before it. | POC 003 `RESULTS.md`, `CLAIM_BOUNDARY.md`; 84/84 capsule tests reported | `SUPPORTED` |
| H-003-G | `HYPOTHESIS` | A single-use, GCL-authorized, Shadow-committed genesis permission can restore cold-start reachability without creating a bypass. | Proposed resolution to the observed safety/liveness cycle. | POC 003 `FAILURE_TO_ADVANCEMENT.md` | Successor experiment not present | Replay, non-empty use, wrong-session use, direct non-Shadow write, pre-commit consumption, or failure to reach the first ordinary cycle | Advancement method only | `NOT_IMPLEMENTED` |
| H-004-L | `IMPLEMENTATION_CLAIM` | One submitted tuple is accepted only when `after <= before`, and a rejected expansion is not appended. | Tests the implemented local non-expansion rule. | POC 004 extracted GCL crate; `SOURCE_MAP.md` | Extracted production tests and POC harness | Acceptance of a tested `after > before`, or mutation after rejection | POC 004 `RESULTS.md`; 19/19 total tests reported | `SUPPORTED` |
| H-004-G | `ARCHITECTURAL_CLAIM` | The current implementation enforces one continuous, phase-ordered LAW-0 history across callers and the outward receipt. | This is the stronger global theory against which the local rule is evaluated. | POC 004 `THEORY.md`, `SHADOW_CONNECTION_FINDING.md` | Reset, phase-regression, terminal-reopen, domain, report-completeness, and receipt-binding counterexamples | Any counterexample is sufficient; several are reproduced | POC 004 `RESULTS.md` records stable counterexamples | `NOT_SUPPORTED` |
| H-005-L | `IMPLEMENTATION_CLAIM` | A handle generation yields at most one successful local authority consumption, including concurrent copies. | Distinguishes copyable FFI representation from hidden Rust-owned authority state. | POC 005 `SOURCE_RUNTIME_MAP.md` | Serial replay, eight-copy concurrency, forged/unknown handle cases | More than one success for one generation, or acceptance of a forged/unknown handle | POC 005 `RESULTS.md`; 15/15 detached tests and 3/3 principal runs reported | `SUPPORTED` |
| H-005-S | `ARCHITECTURAL_CLAIM` | Local generation consumption prevents the same semantic operation from being reauthorized through a fresh generation. | Tests whether identifier-local protection equals semantic non-duplication. | POC 005 `FINAL_VERDICT.txt` | Two fresh generations with the same module/payload | Both generations cross local authorization | The counterexample occurs as predeclared | `FALSIFIED` |
| H-005-D | `HYPOTHESIS` | Durable semantic identity consumed at an idempotent Shadow sink can preserve non-duplication across crash/restart. | Architecture-preserving method for closing H-005-S without granting FFI persistence authority. | POC 005 `FAILURE_TO_ADVANCEMENT.md` | Successor crash/restart/commit experiment absent | Duplicate durable effect, lost authorized effect, bypass without confirmed verdict, or replay after restart | Proposed method only | `NOT_IMPLEMENTED` |
| H-006-R | `IMPLEMENTATION_CLAIM` | Valid required K− history passes the readiness/import gate and hard-blocks a later matching candidate; missing/corrupt/downgraded required history blocks PRO. | Tests whether verified failure evidence becomes operational in a later reasoning step. | POC 006 `SOURCE_RUNTIME_MAP.md` | 30-test detached suite and three principal launches | Valid history has no stated effect, or invalid required history reaches PRO | POC 006 `RESULTS.md`; 30/30 tests and 3/3 runs reported | `SUPPORTED` |
| H-006-X | `ARCHITECTURAL_CLAIM` | The existing cross-subsystem path supplies a full commit-bound receipt and proves re-entry after Shadow termination and authoritative Vault reopen. | Stronger persistence/restart reading of Living Negative Knowledge. | POC 006 `CLAIM_BOUNDARY.md`, `FAILURE_TO_ADVANCEMENT.md` | No detached terminate/reopen test exists | Boolean-only response or absence of detached restart evidence defeats the present-tense claim | `negative_persisted: bool` and restart gap documented | `NOT_SUPPORTED` |
| H-006-PD | `IMPLEMENTATION_CLAIM` | K+ and K− are distinct sibling evidence domains under GCL in the mapped verdict boundary. | Separates constitutional parallelism from type erasure or Boolean opposition. | POC 006 `POSITIVE_NEGATIVE_PARALLELISM.md` | Native focal constructive, rigorous-negative, mixed-pair, and out-of-domain tests | Mixed-domain acceptance, collapse into one Boolean domain, or a child branch inheriting parent authority | Closed domain separation is source-mapped and native focal tests are reported | `SUPPORTED` |
| H-006-PR | `HYPOTHESIS` | A successor can give K+ and K− separate but structurally parallel detached verified-re-entry receipts and effects. | Extends domain separation across the process/restart boundary without making the two channels identical. | POC 006 `POSITIVE_NEGATIVE_PARALLELISM.md`, `FAILURE_TO_ADVANCEMENT.md` | Detached K− path exists; detached K+ counterpart and dual-receipt experiment are absent | Mixed admission laws, shared/type-erased receipt, authority leakage, or failure of either valid path defeats the proposed symmetry | K− only is executed in the detached capsule | `NOT_IMPLEMENTED` |
| H-RW | `DOCUMENTED_ONLY_CLAIM` | The public root workspace is fully closed end to end across every caller, wire, persistence path, and recovery mode. | Deliberately records the broad inference that must not be drawn from four POCs. | Root `README.md`, `Cargo.toml`, source directories | No comprehensive public E2E proof supplied | Any unexecuted or unlinked required edge prevents promotion | POCs explicitly exclude the complete private v1.8.9 workspace and identify concrete gaps | `NOT_SUPPORTED` |

Allowed status vocabulary: `OPEN`, `PARTIALLY_TESTED`, `SUPPORTED`,
`NOT_SUPPORTED`, `FALSIFIED`, `NOT_IMPLEMENTED`, and `UNVERIFIED`.

## H-003 — Generation-zero reachability

**Established external theory.** Safe initialization, liveness from initial states, fail-safe defaults, typestate, evidence-carrying boundaries, and transactional commits are established. See `publications/executable-prior-art/poc-003-system-cold-start-reachability/PRIOR_ART.md`.

**Bledar Gjata / ESS-MAI contribution.** The POC builds a surgical causal experiment across the project-specific GCL → Light/Besa → Shadow selection → Quantum/Asht → Shadow boundary.

**Bounded claimed innovation.** A one-time GCL-delegated genesis transaction could carry authenticated empty-state evidence through existing jurisdictions without granting Light, Quantum, a harness, or an operator direct Vault authority. This is a proposed advancement, not an executed result.

**Materialized evidence.** The public capsule reports 84/84 disclosed tests, three repeated empty-state runs that stop at the exact-positive relevance gate, and an exact-pair harness control that reaches the post-Asht probe. See `RESULTS.md` and `CLAIM_BOUNDARY.md` inside the POC 003 directory.

**Falsifier / gap.** The proposed genesis route fails if it writes outside Shadow, can be replayed, works on non-empty state, loses request/session/GCL binding, consumes permission before durable commit, or still cannot reach an ordinary post-genesis cycle. POC 003 executes no production Shadow commit and does not establish full-system success.

**Next experiment.** Implement the smallest Shadow-owned genesis seam in a successor capsule; test non-empty rejection, replay, wrong-session use, failed commit recovery, capability consumption, and the first ordinary post-genesis cycle.

## H-004 — Global LAW-0 continuity

**Established external theory.** Candidate-space reduction, constraint consistency, global data-flow consistency, typestate, well-founded termination measures, proof objects, and append-only audit structures are established. The POC explicitly distinguishes its caller-reported candidate count from Shannon entropy.

**Bledar Gjata / ESS-MAI contribution.** The project formulates GCL LAW-0 as a governed cross-phase invariant and tests whether the existing `UncertaintyLedger` is global rather than merely local.

**Bounded claimed innovation.** A canonical continuity sidecar could be verified and continued by Light, Quantum, and Shadow, then co-bound to the existing transaction and post-commit receipt without transferring sovereignty to the receipt.

**Materialized evidence.** The capsule reports 19/19 tests and five stable runs. Local tuple non-expansion and rejection atomicity are materialized. The tests also reproduce discontinuity, phase regression, terminal reopening, negative/infinite values, and incomplete reporting. Static source mapping identifies an existing local Shadow durable binding and separate receipt path.

**Falsifier / gap.** The global hypothesis is not materialized while callers can reset `before`, regress phases, reopen terminal zero, admit invalid domains, omit boundaries from reports, or transport a receipt not bound to the LAW-0 transcript. The capsule does not execute the production binaries end to end.

**Next experiment.** Replace caller-owned boundary state with a private typed ledger head; enforce phase order and terminal immutability; use an integer candidate-space type; serialize complete transitions; carry and verify one canonical head across existing wires; bind it to the durable transaction and outward receipt.

## H-005 — FFI representation does not duplicate authority

**Established external theory.** Capabilities, one-shot authorization, atomic consumption, reference monitors, fail-safe defaults, and Rust FFI ownership are prior art. POC 005 does not claim their invention.

**Bledar Gjata / ESS-MAI contribution.** The POC tests a project-specific composition: a `repr(C)`, copyable handle remains below a hidden Rust-owned slot, and local authorization still cannot bypass the superior Shadow/GCL verdict and persistence path.

**Bounded claimed innovation.** The exact constitutional composition—copyable representation, non-copyable authority, and no inherited write jurisdiction—is the bounded proposition. Any legal novelty conclusion remains outside this repository.

**Materialized evidence.** The capsule reports 15/15 detached tests and three repeatable principal runs. One generation yields at most one post-authorization result across serial and eight-copy concurrent cases; forged and unknown handles are refused. Valid non-empty use ends at `CompatibilityHold (-8)`, not a knowledge write.

**Falsifier / gap.** Two fresh generations for the same module/payload both cross local authorization, so semantic idempotency is not established. Consumption is in-process rather than restart-durable, no default-production in-repository caller was found in the declared search, and no FFI-to-Vault persistence is executed.

**Next experiment.** Bind a durable semantic authorization key to the existing confirmed-verdict transaction and an idempotent Shadow sink. Exercise prepare/commit crash points, concurrency, restart, absent or mismatched verdicts, and duplicate-response recovery.

## H-006 — Verified negative knowledge changes a later cycle

**Established external theory.** Failure-driven reminding, failure-aware memory, learning to avoid planning problems, runtime governance, and positive/negative knowledge sharing have relevant prior art. Negative knowledge as a broad concept is not claimed as an ESS-MAI invention.

**Bledar Gjata / ESS-MAI contribution.** POC 006 tests governed, fail-closed import of required sealed K− history and its effect on a later matching candidate. It also maps K+ and K− as semantically distinct sibling streams beneath one verdict boundary.

**Bounded claimed innovation.** The proposition is a project-specific verified re-entry chain in which a typed negative commit receipt binds asset, transaction, cycle, GCL version, export digest, and consumer verification. A future positive receipt should be parallel but not type-erased into the negative channel.

**Materialized evidence.** The capsule reports a strict release build, 30/30 tests, and three identical launches. Valid required history becomes `READY`, imports one entry, hard-blocks the matching candidate, and sets its score to `0.000`. Missing, corrupt, and downgraded required history blocks PRO. Source/native evidence maps distinct constructive and rigorous-negative write branches.

**Falsifier / gap.** The current response carries `negative_persisted: bool`, not an independently verifiable commit receipt. The detached capsule does not terminate Shadow, reopen the authoritative Vault, and launch the next Quantum process. It does not execute symmetric positive-history re-entry. The reproduced FNV seal supports only the tested corruption/downgrade checks, not cryptographic collision resistance.

**Next experiment.** Emit a versioned receipt only after Shadow commit; bind it to the sealed export; require Quantum verification before PRO; then run a detached terminate/reopen/re-import chain. Follow with a dual-receipt K+/K− experiment that preserves distinct admission laws and effects.

## H-RW — Repo-wide architecture hypothesis

**Established external theory.** Hierarchical authority, separation of duty, evidence mediation, transactions, fail-closed gates, and runtime verification are established fields.

**Bledar Gjata / ESS-MAI contribution.** The repository organizes a GCL-rooted Light–Quantum–Shadow research architecture and publishes source plus bounded POCs intended to connect theory to executable evidence.

**Bounded claimed innovation.** The exact composition, terminology, and proposed cross-role receipts are project claims. Their novelty and completeness must be evaluated claim by claim, not inferred from the architecture diagram or repository size.

**Materialized evidence.** `Cargo.toml` exposes four public workspace members. The root `README.md` describes the role separation. POC 003–006 execute or source-map selected edges.

**Falsifier / gap.** The four POCs do not jointly constitute an end-to-end proof of the full system. The complete private v1.8.9 source is not published in these capsules, and no registry entry may infer that all production callers, wires, persistence paths, recovery modes, or security properties have been exercised.

## Hypothesis promotion rule

A gap may be promoted to materialized knowledge only when a successor artifact supplies: a falsifiable claim; an immutable or versioned source boundary; positive and negative controls; exact commands; outputs or machine-readable receipts; integrity checks; an authority-preserving failure path; and a claim boundary updated without rewriting the earlier negative result.
