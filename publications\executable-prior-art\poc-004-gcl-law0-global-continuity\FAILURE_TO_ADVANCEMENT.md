# Failure-to-Advancement Method

## Principle

The current code is preserved as evidence. Advancement must occur in a future implementation branch, not by rewriting the extracted artifact after observing the result.

The governing rule is:

```text
materialized success + reproducible failure = bounded engineering work
```

## Failure 1 — caller-controlled cross-step reset

Observed:

```text
16 -> 1
100 -> 50   accepted
```

Cause: each call supplies both `before` and `after`; the ledger does not derive the new `before` from its private state.

Advancement:

- initialize the ledger once with a validated starting `CandidateSpace`;
- expose a transition API that accepts only the proposed `after` plus evidence;
- derive `before` from the ledger head;
- reject without mutation unless the transition is continuous.

Acceptance criterion: no public safe API can create `next.before != previous.after`.

## Failure 2 — phase regression and terminal reopening

Observed:

```text
Verification 1 -> 0
Coordination 100 -> 50   accepted
```

Cause: `CollapsePhase` is stored as caller data; it is not a state-machine permission.

Advancement:

- encode phase progression with sealed typestates or a private transition state;
- require Coordination before Reasoning and Reasoning before Verification;
- allow explicitly defined repetition only within the current phase;
- make terminal closure consume the mutable ledger and yield an immutable receipt.

Acceptance criterion: skipped, reordered, post-terminal, and sibling-authority transitions fail before state mutation.

## Failure 3 — invalid numeric domain

Observed:

```text
1 -> -1                   accepted and called collapsed
+infinity -> +infinity   accepted
```

Cause: raw `f32` values represent a candidate count, and the API has no explicit finite/non-negative validation. Large integer counts also lose exactness in `f32`.

Advancement:

- introduce a private-constructor `CandidateSpace(u64)` or `CandidateSpace(u128)`;
- use checked multiplication and checked conversion;
- represent an unknown/uninitialized state as a distinct enum, not infinity;
- keep probability or score types separate from candidate counts.

Acceptance criterion: negative, non-finite, fractional, overflowed, and inexact count states are unrepresentable or fail closed.

## Failure 4 — incomplete report boundary

Observed: the report for the discontinuous chain prints `16 -> 1 -> 50` and omits the injected second `before=100`.

Advancement:

- serialize every complete transition tuple;
- include sequence number, phase, producer role, previous head, before, after, candidate-set digest, evidence digest, rule/version digest, session, and cycle;
- domain-separate and hash-chain the encoded entries;
- make human-readable output a rendering of the verified canonical transcript.

Acceptance criterion: any changed or omitted boundary changes the ledger head and fails verification.

## Failure 5 — existing platform mechanisms are not connected as one LAW-0 proof

Observed:

- Light materializes a separate checked integer coordinate receipt;
- Quantum creates a new local shared ledger;
- Shadow creates another new local terminal ledger whose report is the same constant close marker for every successful production cycle;
- Shadow already binds that local digest into `cycle_digest`, `VaultTransaction`, `transaction_id`, and the WAL;
- Shadow separately publishes a verification receipt that Quantum recomputes and Light closes;
- the receipt and response wire omit `law0_digest`, `cycle_digest`, and `transaction_id`, while the Quantum-to-Shadow input omits an upstream LAW-0 head.

Advancement:

```text
Light produces a validated Coordination head
  -> Quantum verifies and continues the head through Reasoning
  -> Shadow verifies the upstream head and appends its Verification close
  -> existing VaultTransaction binds the terminal head
  -> existing response path publishes a post-commit connection proof
```

Each consumer must recompute continuity and evidence bindings before authoritative work. Reuse the existing `FinalEvidenceWire`, `VerificationContext.law0_digest`, `cycle_digest`, `VaultTransaction`, `VerificationReceipt`, and `VerificationReceiptWire` seams; do not introduce a parallel Shadow authority or a second commit subsystem. The receipt is a proof object subordinate to GCL, not a new authority.

The source already supplies the first carrier: Light embeds `legacy:<receipt_sha256>` in `lgc_seal`, and the existing Light-to-Quantum wire transports that string. Quantum currently retains but does not consume the legacy segment. The source also supplies the second carrier and sink: `FinalEvidenceWire` already crosses Quantum-to-Shadow, while `VerificationContext.law0_digest` and `VaultTransaction` already accept the final commitment. Advancement should connect these existing seams:

1. fail-closed parse the existing 64-lowercase-hex Light receipt SHA-256 in Quantum;
2. recompute the Coordination boundary from the validated PA/Besa split with checked `u64` arithmetic;
3. continue a canonical `u64` transcript through Reasoning, enforcing adjacency and phase order;
4. carry that sidecar inside the existing `FinalEvidenceWire` schema and verify its dedicated SHA-256 before Shadow core work;
5. append Shadow Verification from the received head to zero, including valid `0 -> 0` closure when Quantum already ended at zero;
6. feed the complete terminal digest into the existing transaction binding and version the outward receipt if public proof is claimed.

The executable harness demonstrates the smallest final seam in its bounded cases: the current byte-identical receipt ID does not change when only LAW-0 changes, while a domain-separated connection of the current receipt ID and LAW-0 digest does change and detects substitution. This is a POC advancement probe, not current production integration. Adding fields to `FinalEvidenceWire` or `VerificationReceiptWire` remains a coordinated protocol-schema change, not merely a parser change.

Acceptance criterion: Shadow can validate one domain-separated transcript back to the Light origin; the published receipt identifies the verified terminal LAW-0 head and durable transaction; and missing, zero, reordered, replayed, cross-session, cross-cycle, or forged entries fail before verdict commitment or publication.

## Funding work packages

1. **Typed domain:** canonical candidate-count and unknown-state types, overflow/inexactness tests.
2. **Constitutional state machine:** state-derived transitions, phase typestate, terminal seal.
3. **Evidence binding:** canonical encoding, candidate/evidence/rule digests, hash-linked heads.
4. **Wire integration:** Light origin, Quantum continuation, Shadow closure without parallel sovereignty.
5. **Adversarial validation:** property tests, replay/cross-session tests, incomplete-chain tests, and production-binary integration.

## Architecture preservation

GCL continues to grant and bound authority. Light does not verify Shadow's verdict; Quantum does not inherit GCL sovereignty; Shadow does not invent upstream candidate counts. The parent supplies constitutional unity while the children retain their distinct jurisdictions.
