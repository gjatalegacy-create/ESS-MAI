# Theory under test

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

## Scope of materialization

The POC directly materializes the sealed re-entry and future-influence gate.
Native v189 tests independently materialize the typed asset, Shadow admission,
Vault transaction/deduplication, and WAL restart portions. The present wire
does not yet materialize a full cross-subsystem negative-commit receipt.

This distinction prevents a green compiler result from being mistaken for a
complete end-to-end constitutional receipt chain.
