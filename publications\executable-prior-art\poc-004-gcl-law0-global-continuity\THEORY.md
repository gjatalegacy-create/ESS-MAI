# Theory — GCL LAW-0 Continuous Uncertainty Collapse

## Constitutional placement

GCL is the parent constitutional authority. Coordination, Reasoning, and Verification are subordinate jurisdictions with distinct responsibilities. Their separation is preserved below GCL while their state transitions are unified above it.

```text
GCL
 ├─ Coordination / Light
 ├─ Reasoning / Quantum
 └─ Verification / Shadow
```

No phase becomes sovereign merely because it records a transition. A phase supplies evidence within its bounded jurisdiction; GCL defines the invariant that joins the phases.

## Operational meaning of uncertainty

Within this POC, uncertainty is a caller-reported count of remaining candidate possibilities. It is not Shannon entropy, probability, semantic truth, model confidence, or proof that the candidate set was measured correctly.

For a transcript `S = [s0, s1, ..., sn]`, each step contains:

```text
s = (phase, module, before, after)
```

The minimum safety invariant is:

```text
finite(before) && finite(after)
0 <= after <= before
```

For every step after the first, global continuity additionally requires:

```text
s[i].before == s[i-1].after
```

Phase governance requires:

```text
Coordination <= Reasoning <= Verification
```

and a terminal zero reached in Verification must be immutable.

## Safety versus progress

The production function deliberately permits equality. Therefore `after <= before` is a non-expansion safety property, not proof of strict reduction, convergence, or termination. A future termination claim needs a distinct strictly decreasing, well-founded measure or a bounded no-progress rule.

## What is materialized in v189

The shared `gcl-constitution` crate provides `LawStep`, `LawViolation`, `UncertaintyLedger`, `record`, `current_space`, `is_collapsed`, `steps`, and `chain_report`.

`record` materially checks:

```text
after <= before
```

and returns `LawViolation` before mutation when the tuple expands locally. The upstream tests demonstrate the reference chain and direct expansion rejection in the tested cases.

Quantum uses the ledger in its production main path and manually reuses the previous variables as the next `before`. Light has a separate checked integer contraction and receipt, but does not instantiate this ledger. Shadow uses the same ledger type for a fresh constant local terminal marker, hashes that report, and materially connects the digest to `cycle_digest`, `VaultTransaction`, `transaction_id`, and WAL persistence.

Shadow also has a separate materialized verification-receipt path: a single-use capability produces a SHA-256 receipt, the receipt is returned over the existing response wire, Quantum recomputes it, and Light closes the original cycle. The two successful paths meet only inside the durable transaction as sibling inputs. The outward receipt algorithm does not accept the LAW-0 digest, and the upstream possibility-space head does not enter Shadow.

## Falsification rule

The global theory is falsified for the current implementation if any accepted transcript can:

- set `next.before` to a value different from `previous.after`;
- regress or skip the governed phase order;
- reopen a terminal collapsed state;
- accept negative or non-finite candidate-space values;
- claim an audit chain while hiding a caller-supplied boundary;
- represent separate platform-local transcripts as one end-to-end proof.

The experiment reproduces the first five. The byte-identical Shadow receipt probe also reproduces the missing receipt binding. Static wire analysis establishes that the cross-platform mechanisms are present but their LAW-0 continuity connection is not materialized.

## Experimental verdict

```text
LOCAL_TUPLE_NON_EXPANSION_CHECK = MATERIALIZED
DIRECT_REJECTION_ATOMICITY      = MATERIALIZED
GLOBAL_STEP_CONTINUITY          = NOT_ENFORCED
PHASE_ORDER                     = NOT_ENFORCED
TERMINAL_IMMUTABILITY           = NOT_ENFORCED
FINITE_NONNEGATIVE_DOMAIN       = NOT_ENFORCED
SHADOW_LOCAL_DURABLE_BINDING    = MATERIALIZED
SHADOW_RECEIPT_TRANSPORT        = MATERIALIZED
LAW0_TO_PUBLIC_RECEIPT          = UNLINKED
CROSS_PLATFORM_TRANSCRIPT       = COMPONENTS_PRESENT_UNLINKED
OVERALL                         = PARTIAL_MATERIALIZATION
```

This result is intentionally not rounded up to full enforcement and not rounded down to absence. It identifies precisely which part of the theory exists in executable form.
