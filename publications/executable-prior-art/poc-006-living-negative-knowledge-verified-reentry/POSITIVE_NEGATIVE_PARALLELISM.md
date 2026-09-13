# Positive and Negative Knowledge as parallel advancement streams

## Constitutional rule

ESS-MAI does not model positive and negative knowledge as interchangeable
Boolean labels. They are sibling epistemic outcomes under the same superior
GCL/Shadow constitutional boundary:

```text
                         GCL / ESS-MAI
                               |
                    LawConfirmedVerdict
                       /               \
              Constructive       RigorousNegative
                   |                    |
          KnowledgeWrite::        KnowledgeWrite::
              Primitive              Negative
                   \                    /
                    \-- next-cycle --/
                         advancement
```

Source evidence in v189:

- `shadow/src/types.rs:582-584` defines the closed verdict kinds
  `Constructive` and `RigorousNegative`;
- `shadow/src/types.rs:650-675` admits `(1,1)` as constructive, admits
  `(0,0)` only with real negative proof, and refuses mixed pairs;
- `shadow/src/shadow_gj_legacy.rs:348-367` maps the sealed constructive
  verdict to `KnowledgeWrite::Primitive` and the sealed rigorous-negative
  verdict to `KnowledgeWrite::Negative`;
- `shadow/src/shadow_commit.rs:7-25` defines both write variants in one
  closed knowledge write-set;
- `shadow/src/knowledge_vault.rs:591-633` applies both variants inside the
  same committed Vault transaction while preserving their distinct storage
  semantics.

The four focused native verdict tests passed: constructive closure, required
negative proof, refusal of mixed pairs, and refusal of out-of-domain bits.

## Parallel does not mean identical

```text
K+ != K-
Authority(K+) subset Authority(GCL)
Authority(K-) subset Authority(GCL)
```

The positive stream records what survived the constructive verification path.
The negative stream records what was rigorously disproved, with its failure
context and limitations. Neither stream may impersonate the other. A
structural refusal, missing proof, unknown state, or wrong receiving authority
is `HOLD`/refusal—not fabricated positive or negative knowledge.

The v189 code is intentionally asymmetric where meaning requires it. For
example, constructive verdicts may produce GCL reinforcement, while rigorous
negative verdicts do not use that same reinforcement operation. Shared parent
authority therefore preserves constitutional unity without erasing semantic
difference.

## Advancement formulation

The following is an architectural formulation derived from the source
topology; it is not claimed as a line-for-line implementation of every
next-cycle operation:

```text
VerifiedEvidence_t
  --GCL confirmation-->
    K+_t  |  K-_t  |  HOLD_t

CandidateSpace_(t+1)
  = Expand(CandidateSpace_t, K+_t)
    minus Exclude(CandidateSpace_t, K-_t)
```

Positive knowledge expands or reinforces supported routes. Negative knowledge
constrains routes already shown to fail. Both reduce uncertainty and therefore
advance success, but by different operations.

## What POC-006 actually tests

POC-006 executes the negative re-entry half:

```text
verified K-_t
  -> sealed history
  -> next-cycle import
  -> known failure hard-blocked
```

It also proves the negative controls: required history that is missing,
corrupt, or downgraded blocks PRO before reasoning. The positive write branch
and the closed two-verdict domain are source-evidenced and natively tested, but
POC-006 does not claim to execute a symmetric positive-history re-entry
experiment.

The next strong end-to-end POC variation should carry both typed receipts in
one cycle protocol:

```text
PositiveCommitReceiptWire  ||  NegativeCommitReceiptWire
                  \        /
                   GCL cycle receipt
                         |
              verified next-cycle use
```

The two receipts may share version, cycle, GCL, transaction, and digest
infrastructure, but they must retain different types, admission laws,
consumers, and effects.
