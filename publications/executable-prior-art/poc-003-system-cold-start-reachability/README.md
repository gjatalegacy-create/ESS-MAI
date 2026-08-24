# ESS-MAI SYSTEM POC 003 — Surgical Cold-Start Reachability

Author and architect: **Bledar Gjata**  
Project: **ESS-MAI**  
Organization / affiliation: **Gjata Legacy**  
Public contact: **gjata@legacy.al**  
License: **Apache-2.0**  
Capsule version: **0.2.0**  
Source baseline: **ESS-MAI v1.8.9**

Artifact class: `SYSTEM_POC`

Experimental classification: `PARTIAL_SYSTEM_MATERIALIZATION_WITH_CAUSAL_CONTROL`

## Falsifiable claim

From generation zero—an empty Shadow knowledge state—the bounded real authority path should be able to traverse:

```text
GCL parent authority
  → Light Coordination state machine
  → production Besa selection client
  → separate Shadow selection process
  → Quantum Reasoning state machine
  → production Asht selection client
  → separate Shadow selection process
  → post-Asht final-Shadow reachability point
```

The claim fails if the empty state is accepted by Besa but Asht requires an exact prior candidate and prevents the route from reaching the final-Shadow probe.

## Executed result

`cargo build --workspace --all-targets --locked --offline` passes. The complete disclosed suite passes **84/84** tests:

- 11 GCL constitutional tests;
- 13 public Shadow-contract tests;
- 44 Quantum surgical-closure tests;
- 10 system-module unit tests;
- 6 cold-start integration tests.

Three fresh empty-state executions reproduced the same result:

```text
GCL parent authority verified                    PASS
Light/Besa → separate Shadow process             PASS
Besa complete empty selection                    PASS
Quantum/Asht → separate Shadow process           PASS
Asht exact-positive relevance at generation zero FAIL-CLOSED
post-Asht final-Shadow probe reached              NO
production Shadow commit executed                 NO
```

A separate positive control supplied one exact positive and one exact negative candidate through the harness selector. The same Asht path then passed and reached the post-Asht probe. The control did **not** execute a production Shadow commit.

This isolates the bounded causal finding:

```text
empty exact-candidate set  → reachability gap reproduced
exact-pair control present → gate passes
```

It does not establish that a full native v1.8.9 cycle succeeds.

## What is production-exact and what is new

The capsule contains **20 whole-file byte-identical production extracts**: 18 Rust source files and two existing Cargo manifests. It also contains one 124-line byte-identical excerpt of Shadow's read-only bounded selection function.

New POC glue provides:

- the reduced Cargo workspace and two surgical crate manifests;
- module-compatibility shells that create no authority;
- an empty-store Shadow shell around the exact selection excerpt;
- a separate surgical `shadow_platform` process;
- experiment orchestration, positive-control mode, CLI, and integration tests.

The new Shadow process is a bounded selector for this POC. It is not represented as the full production Shadow binary, writer, transaction engine, WAL, or verdict core.

Run `scripts/verify_extraction.ps1` to verify all 20 whole-file extracts and the exact excerpt.

## Architectural interpretation

GCL remains the sovereign constitutional parent. Light, Quantum, and Shadow exercise separate bounded jurisdictions; none inherits GCL sovereignty. The positive control does not grant Quantum or the harness decision authority. It only tests whether the existing request-bound relevance gate is reachable when its required evidence exists.

The observed gap is a generation-zero cycle:

```text
empty Shadow state
  → Besa lawfully accepts complete emptiness
  → Asht requires prior exact positive and negative candidates
  → final Shadow judgment/transaction is not reached
  → the ordinary downstream writer cannot create the first candidates
```

## Architecture-preserving advancement method

The narrow next hypothesis is a **GCL-authorized, one-time genesis transaction route** through the existing Shadow jurisdiction—not a direct seed, not a permanent Asht relaxation, and not a new peer authority.

It should:

1. verify and durably attest that the knowledge state is genuinely empty;
2. bind the one-time permission to the current request, session, GCL directive, and evidence;
3. carry typed “generation-zero / no prior candidate” evidence past the ordinary prior-candidate gate;
4. leave judgment and all persistent writes exclusively in Shadow;
5. use the existing Shadow transaction/WAL path;
6. consume the genesis permission after the first successful commit;
7. restore the ordinary Asht relevance rule for every later cycle;
8. fail closed on replay, non-empty state, cross-session use, missing evidence, or commit failure.

This method is proposed future work. It is not implemented or claimed successful by this capsule, and resolving it may reveal further downstream blockers.

## Scope boundary

This public capsule is surgical. It excludes the complete private v1.8.9 workspace, the full Light/Quantum/Shadow binaries, embedded private material, C01/C02, generated targets, and the full Shadow storage core.

The earlier private native experiment is historical local context only. Its 1,288-pass/1-fail package audit is not relabeled as evidence from this public capsule.

This POC is technical evidence, not an exhaustive patent search, novelty opinion, freedom-to-operate opinion, security certification, or external validation.

## Start here

- `CLAIM_BOUNDARY.md` — exactly what execution supports;
- `RESULTS.md` — success, failure, and positive-control matrix;
- `PRIOR_ART.md` — established work, existing ESS-MAI materialization, contribution, and proposed advancement;
- `FAILURE_TO_ADVANCEMENT.md` — architecture-preserving method and variants;
- `SOURCE_MAP.md` — production-exact versus new glue;
- `REPRODUCIBILITY.md` — locked/offline commands;
- `PUBLICATION_MANIFEST.md` — allowlist, exclusions, and integrity model.
