# Theory under test

## Existing theory

Failure-driven reminding, explanation-based failure avoidance, positive and
negative knowledge sharing, structured negative-memory banks, provenance, and
runtime governance all predate this POC. The broad proposition that a system
can retain failures and use them later is prior art.

## Bledar Gjata / ESS-MAI contribution

The contribution tested here is the ESS-MAI-specific constitutional
composition of a typed negative asset, independent Shadow validation,
transactional Vault admission, sealed export, one-way downgrade detection,
mandatory Quantum readiness before PRO, and an observable later hard block.
Positive and negative results remain different sibling knowledge authorities
under GCL rather than becoming interchangeable labels.

## Bounded innovation proposition

The proposed innovation is **Living Negative Knowledge**: a rigorously
verified failure remains typed, bounded and operational across cycles, so it
can constrain later reasoning while preserving the distinct authority of the
producer, verifier, persistence owner and consumer. This POC materially
supports verified import and later influence; it does not yet prove the whole
receipt-bound lifecycle in one detached run.

## State model

Let `N` be a typed negative asset, `V` the Shadow validation predicate, `T`
the Shadow Vault transaction, `S` the exported sealed history, `R` the
Quantum re-entry predicate, and `P` permission to activate PRO.

```text
failure evidence
  -> N
  -> V(N)
  -> T.commit(N)
  -> S = seal(export(Vault))
  -> R(S)
  -> future influence
```

The bounded laws tested here are:

```text
V(N) = false                         => T.commit(N) is forbidden
history_required AND R(S) != READY  => P = false
R(S) = READY AND N matches candidate
                                     => hard_block(candidate) = true
```

Negative history is not treated as a Boolean label. It has typed content,
provenance-bearing fields, severity/confidence, process semantics, storage
behavior, and an observable effect on a later candidate.

## Parallel positive and negative knowledge

The closed v189 verdict domain has two distinct knowledge outcomes:

```text
Constructive       -> KnowledgeWrite::Primitive  -> K+
RigorousNegative   -> KnowledgeWrite::Negative   -> K-
insufficient proof -> HOLD / structural refusal  -> no fabricated knowledge
```

`K+` and `K-` remain separate sibling streams. Positive evidence reinforces
or expands supported routes; negative evidence constrains failed routes. Their
unity is constitutional, not semantic: both are admitted only through the
superior GCL/Shadow verdict boundary and both advance the next cycle by
different operations. See
[POSITIVE_NEGATIVE_PARALLELISM.md](POSITIVE_NEGATIVE_PARALLELISM.md).

## Authority structure

```text
GCL / ESS-MAI constitutional law
  ├─ Shadow authority
  │    validate -> transact -> persist -> deduplicate -> export
  └─ Quantum authority
       import -> verify seal/readiness -> filter -> permit/refuse PRO
```

The sibling authorities cooperate through a bounded contract. Quantum does
not become a Vault writer; Shadow does not activate PRO. A missing or invalid
handoff holds the state and produces a precise fail-closed refusal at the
consumer boundary.

## Scope of current materialization

The POC directly materializes the sealed re-entry and future-influence gate.
Native v189 tests independently materialize the typed asset, Shadow admission,
Vault transaction/deduplication, and WAL restart portions. The present wire
does not yet materialize a full cross-subsystem negative-commit receipt.

This distinction prevents a green compiler result from being mistaken for a
complete end-to-end constitutional receipt chain.

## Further research and architecture-preserving variants

The smallest closure is a typed `NegativeCommitReceiptWire` emitted only after
the existing Shadow transaction commits and verified by Quantum before use.
It should bind asset token, transaction/cycle identity, GCL identity, export
digest and persistence result without granting Quantum write authority.

Additional POC variations should execute real terminate/reopen recovery and a
symmetric positive-history re-entry path. The K+ and K- receipts may share
cycle infrastructure, but they must retain distinct admission laws, data
types, consumers and effects.
