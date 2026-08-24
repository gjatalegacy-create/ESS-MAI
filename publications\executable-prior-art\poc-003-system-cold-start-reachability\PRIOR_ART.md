# Focused Prior Art and Advancement Note

Review date: 2026-08-24

Scope: fail-safe defaults, initialization/liveness, state-machine phase control, evidence-carrying boundaries, and transactional genesis relevant to the bounded cold-start reachability result.

This is a focused technical comparison, not an exhaustive patent search, novelty opinion, freedom-to-operate analysis, patentability conclusion, or legal advice.

## 1. Established external foundations

- [Saltzer and Schroeder, “The Protection of Information in Computer Systems,” 1975](https://doi.org/10.1109/PROC.1975.9939) establishes fail-safe defaults: access should depend on explicit permission. Fail-closed behavior is established prior work.
- [Dijkstra, “Self-stabilizing Systems in Spite of Distributed Control,” 1974](https://doi.org/10.1145/361179.361202) treats recovery toward legitimate behavior from arbitrary initial state. Initialization and liveness from state boundaries are established concerns.
- [Strom and Yemini, “Typestate: A Programming Language Concept for Enhancing Software Reliability,” 1986](https://doi.org/10.1109/TSE.1986.6312929) restricts operations according to state. Phase/state-constrained APIs are established.
- [Necula, “Proof-Carrying Code,” 1997](https://doi.org/10.1145/263699.263712) establishes producer-supplied evidence checked by a consumer before acceptance. Evidence-carrying boundaries are established.
- [Gray, “The Transaction Concept: Virtues and Limitations,” 1981](https://doi.org/10.1007/3-540-10293-5_1) describes atomic transactional state change and recovery principles. Durable all-or-nothing update is established.
- [Lamport, “The Temporal Logic of Actions,” 1994](https://lamport.azurewebsites.net/pubs/acm-toplas.pdf) distinguishes initial-state predicates, next-state actions, safety, and liveness. A safe refusal does not by itself establish useful progress.

The broad ideas of secure initialization, one-time capabilities, fail-closed gates, state machines, proof objects, and transactional commits are not claimed as new.

## 2. Pre-existing ESS-MAI v1.8.9 materialization

`EXECUTED_IN_PUBLIC_CAPSULE`:

- GCL phase directives and wrong-phase rejection;
- Light Alnur and Besa state-machine behavior;
- Quantum UCL and Asht state-machine behavior;
- public Shadow wire contracts and bounded selection protocol;
- production Light/Quantum clients that invoke a separate Shadow process;
- request-bound relevance that requires exact positive and negative candidates;
- selection-hold integrity and replay refusal.

`STATIC_PRIVATE_SOURCE_MAPPED`:

- downstream Shadow judgment;
- positive and negative write construction;
- `VaultTransaction`, WAL, and terminal receipt paths.

The static items are not executed by this public capsule and are not upgraded to public end-to-end evidence.

## 3. Contribution of this POC

The contribution is a reproducible surgical experiment that:

1. publishes 20 byte-identical whole-file extracts and one exact Shadow projection excerpt without publishing the full core;
2. connects real extracted Light and Quantum state machines to a separate surgical Shadow selection process;
3. reproduces the empty-state reachability failure three times;
4. adds an exact-pair positive causal control through the same Asht gate;
5. distinguishes a safe local refusal from an unresolved system liveness condition;
6. records precisely that no production Shadow commit was executed.

This is a bounded experimental contribution, not a broad claim to the established concepts above.

## 4. Experimental success and failure

### Success

- locked offline Cargo build succeeds;
- 84/84 tests pass;
- GCL parent/phase checks reject wrong jurisdiction;
- both real selection clients cross a process boundary;
- empty selection is carried without fabrication;
- Asht fails closed rather than inventing knowledge;
- exact-pair control passes the same gate.

### Failure / unresolved gap

- generation-zero empty state cannot reach the downstream final-Shadow point;
- ordinary relevance requires knowledge that only a later path can durably create;
- no production judgment, transaction, WAL commit, or terminal receipt occurs;
- the public POC does not establish complete native E2E success.

The positive control narrows the observed cause to the missing exact candidates in this disclosed route; it does not prove there are no later private-runtime failures.

## 5. Bledar Gjata / ESS-MAI proposed advancement

The authored advancement is a GCL-governed one-time genesis route through existing Shadow authority:

- authenticated empty-state proof;
- request/session/phase/evidence binding;
- single-use replay-protected capability;
- explicit typed absence rather than fabricated prior knowledge;
- final judgment and persistent write only in Shadow;
- existing transaction/WAL path;
- automatic closure after first durable commit;
- ordinary Asht rule restored post-genesis.

The intended architectural distinction is:

```text
not: bypass gate → seed vault directly
but: GCL-delegated genesis evidence → Shadow verifies/judges/transacts → capability closes
```

This is proposed future materialization. It is not current production integration.

## Search limits

The review is focused, English-language, and source-led. It does not cover every patent family, non-English disclosure, thesis, repository, standard, commercial system, or unpublished work. No claim of “patent killing,” guaranteed legal priority, or complete novelty is made.
