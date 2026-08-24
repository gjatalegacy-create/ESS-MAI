# Focused Prior-Art and Advancement Note

Search/review date: 2026-08-23

Scope: monotone candidate-space reduction, phase/state ordering, proof-object transport, and append-only integrity mechanisms relevant to the bounded LAW-0 claim.

This is a focused technical comparison, not an exhaustive patent search, novelty opinion, freedom-to-operate analysis, patentability conclusion, or legal advice.

## Terminology boundary

“Uncertainty” in this POC means an explicitly supplied candidate-space count. It is not Shannon entropy, probability, confidence, semantic truth, or epistemic correctness. The present function checks an ordering relation between two caller-provided `f32` values; it does not independently measure a candidate set.

## Established foundations

- [Mitchell, “Version Spaces: A Candidate Elimination Approach to Rule Learning,” IJCAI 1977](https://www.ijcai.org/Proceedings/77-1/Papers/048.pdf) updates a version space by eliminating hypotheses inconsistent with observations. Candidate-set reduction is therefore established prior work.
- [Mackworth, “Consistency in Networks of Relations,” 1977](https://doi.org/10.1016/0004-3702(77)90007-8) develops consistency methods that prune locally inconsistent values and relations in constraint networks. Local pruning of a search space is established.
- [Cousot and Cousot, “Abstract Interpretation: A Unified Lattice Model for Static Analysis of Programs by Construction or Approximation of Fixpoints,” 1977](https://doi.org/10.1145/512950.512973) formalizes ordered abstract domains and fixpoint approximation. Monotone ordered reasoning and convergence frameworks are established.
- [Kildall, “A Unified Approach to Global Program Optimization,” 1973](https://doi.org/10.1145/512927.512945) gives a formal global data-flow analysis algorithm over program-flow structure. The distinction between local transfer behavior and global analysis consistency is established.
- [Strom and Yemini, “Typestate: A Programming Language Concept for Enhancing Software Reliability,” 1986](https://doi.org/10.1109/TSE.1986.6312929) uses state-refined types to restrict which operations are permitted in a context. Compile-time enforcement of valid transition order is established.
- [Lee, Jones, and Ben-Amram, “The Size-Change Principle for Program Termination,” 2001](https://doi.org/10.1145/360204.360210) connects termination to infinite descent in well-founded values. Mere non-expansion, especially with equality, is not sufficient proof of progress or termination.
- [Necula, “Proof-Carrying Code,” 1997](https://doi.org/10.1145/263699.263712) establishes the separate pattern in which a producer supplies evidence that a consumer validates before accepting code behavior. Producer-to-consumer proof objects are established.
- [RFC 9162, Certificate Transparency Version 2.0](https://www.rfc-editor.org/rfc/rfc9162.html) standardizes append-only Merkle audit structures and consistency proofs. Cryptographically linked auditability is established.
- [Shannon, “A Mathematical Theory of Communication,” 1948](https://doi.org/10.1002/j.1538-7305.1948.tb01338.x) is cited specifically to prevent conflating candidate counts with information-theoretic entropy.

## Existing materialization in ESS-MAI v1.8.9

The private v189 baseline combines:

1. `EXECUTED_IN_PUBLIC_CAPSULE` — a constitutional rule named LAW-0;
2. `EXECUTED_IN_PUBLIC_CAPSULE` — a local gate accepting a reported transition only when `after <= before`;
3. `EXECUTED_IN_PUBLIC_CAPSULE` — explicit Coordination, Reasoning, and Verification labels under GCL;
4. `STATIC_SOURCE_MAPPED` — separate phase-local materializations in Light, Quantum, and Shadow;
5. `EXECUTED_IN_PUBLIC_CAPSULE` — a terminal `is_collapsed` predicate and a human-readable chain report;
6. `STATIC_SOURCE_MAPPED` — a Shadow local close digest co-bound with a verification-receipt digest in one SHA-256 transaction identity and WAL path; and
7. `STATIC_SOURCE_MAPPED` — an existing receipt transport that is recomputed downstream but currently omits the LAW-0 digest and transaction identity. The isolated receipt algorithm itself is executed in the public capsule.

This limited review did not identify an exact description of that named repository-specific composition in the sources above. That observation is not evidence of novelty and must not be represented as a novelty conclusion.

## Contribution of this POC and executed materialization

- The exact shared crate builds independently with zero external dependencies.
- An individual reported tuple with `after <= before` is accepted.
- A direct reported tuple with `after > before` is rejected before append.
- The upstream reference chain reaches zero.
- The executable POC reproduces the missing seam: changing only LAW-0 leaves the current receipt ID unchanged, while the harness-only domain-separated connection changes and detects substitution.

This POC contribution is the explicit, reproducible split between supported local behavior, executable counterexamples to the stronger global theory, and a bounded architecture-preserving advancement seam. It is not represented as completed production integration.

## Existing v189 behavior identified by static source mapping

- `STATIC_SOURCE_MAPPED` — Light, Quantum, and Shadow each contain heterogeneous/local LAW-0-related production behavior.
- `STATIC_SOURCE_MAPPED` — Shadow's local constant close marker enters cycle material, transaction identity, serialization, replay-validation, flush, and `sync_all` paths.
- `STATIC_SOURCE_MAPPED` — Shadow's separate SHA-256 verification receipt is transported and recomputed by Quantum, with corresponding closure checks in Light.
- `STATIC_SOURCE_MAPPED` — Light embeds its legacy receipt SHA-256 in the existing `lgc_seal` transported to Quantum; the current Quantum parser retains the string but does not consume that receipt segment as LAW-0 evidence.

These four statements map a private source baseline. The full Light–Quantum–Shadow runtime and WAL durability path are not executed by this public capsule.

## Experimental failures and remaining gaps

- The values are not bound by this API to an actual candidate set or authoritative measurement.
- Accepted steps are not required to be continuous with prior state.
- Phase order, coverage, and terminal immutability are not enforced by the ledger.
- Raw `f32` values permit negative, infinite, fractional, and eventually inexact count semantics.
- Equality is accepted, so the current rule demonstrates non-expansion for the tested tuple, not strict progress.
- The readable report omits later `before` values.
- No one end-to-end possibility-space transcript is carried from Light through Quantum to Shadow.
- Shadow's local close digest is constant for the identical local `1 -> 0` marker and is not a per-cycle upstream transcript.
- The public receipt does not bind `law0_digest`, `cycle_digest`, or `transaction_id`; durable co-binding inside the transaction is therefore not externally verifiable from the receipt alone.
- `VaultTransaction::validate` integrity-binds the supplied LAW-0 value but does not require it to be nonzero or semantically recompute a transcript.
- The focused review does not establish that the full ESS-MAI combination is novel, patentable, secure, or free to operate.

## Bledar Gjata / ESS-MAI proposed advancement relative to established work

The proposed advancement is not the broad idea of reducing a candidate set or hashing a log. It is the precise constitutional integration and executable boundary below. It is an authored advancement method for future work and is not claimed as production-integrated:

- validated integer `CandidateSpace` rather than raw floating-point counts;
- state-derived continuity rather than caller discipline;
- phase typestate and terminal consumption;
- evidence-bound transition entries;
- one domain-separated hash-linked continuity head passed through existing Light, Quantum, and Shadow jurisdictions;
- reuse of the existing Light `lgc_seal`, Quantum-to-Shadow `FinalEvidenceWire`, and Shadow transaction sink rather than a parallel authority or transport channel;
- terminal binding of that verified head into the existing Shadow transaction and existing verification receipt, with publication only after durable commit;
- fail-before-work verification at each platform boundary;
- a separate well-founded strict measure if convergence or termination is claimed.

This preserves the architecture: GCL remains the constitutional parent, and proof transport does not create a peer or substitute authority.

## Search limitations

The review was focused, English-language, and source-led. It did not search every patent family, non-English disclosure, thesis, repository, standard, commercial system, or unpublished work. Dates and links identify known earlier work; they do not establish a complete legal priority record.
