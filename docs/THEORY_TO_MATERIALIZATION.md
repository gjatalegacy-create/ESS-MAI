# Theory-to-materialization map

Status date: 2026-09-14

## Evidence classes

| Code | Meaning | What it may support |
| --- | --- | --- |
| `EXT` | Established external theory cited by a capsule | Context and prior-art boundary; not ESS-MAI invention |
| `GJ` | Bledar Gjata / ESS-MAI formulation or composition | Authorship of the project-specific contribution, subject to its claim boundary |
| `INNOV-P` | Bounded innovation proposition | A research proposition needing prior-art and experimental scrutiny; not a novelty opinion |
| `EXEC` | Behavior executed in the public capsule | Only the stated command, input, observable, and boundary |
| `SRC` | Public or declared source edge mapped but not executed end to end | Architectural presence, not runtime closure |
| `GAP` | Reproduced counterexample, omission, or unexecuted condition | Negative evidence and next-experiment input |

## End-to-end closure registry

This is the required trace. A row is `MATERIALIZED` only when every layer needed
for that row's bounded statement is present. A missing layer remains visible; it
is never filled by documentation alone.

| Theory | Formalization | Architectural object | Source module | Runtime behavior | Test | Artifact | Observed result | Status |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Safe systems must preserve refusal while making an initial legitimate state reachable | Empty set vs exact-pair predicate; post-Asht probe as observable | GCL → Besa → Asht → separate Shadow probe | POC 003 extracted crates and `src/system.rs` identified by its `SOURCE_MAP.md` | Empty state stops at exact-positive gate; exact-pair control advances | 84-test suite; three empty runs; one exact-pair control | POC 003 `RESULTS.md` and extraction manifest | Gap localized; no production commit | `PARTIAL` |
| A one-use genesis capability can resolve cold-start without broadening child authority | Single-use, session-bound, non-empty-rejecting, consume-after-commit rules | Proposed Shadow-owned genesis transaction | No implemented successor source | Not executed | Proposed replay/crash/non-empty/ordinary-cycle matrix | POC 003 `FAILURE_TO_ADVANCEMENT.md` only | Method documented; no runtime result | `DOCUMENTED_ONLY` |
| Local uncertainty non-expansion | `after <= before`, rejection atomicity | `UncertaintyLedger` local step | POC 004 `extracted/gcl-constitution/` | Expansion refused without append | Extracted/POC harness tests | POC 004 `RESULTS.md` | Local bounded property supported | `MATERIALIZED` |
| Global LAW-0 continuity across all phases and roles | One private typed head; monotone/phase/terminal/domain invariants; receipt binding | Cross-Light/Quantum/Shadow continuity transcript | Separate local seams mapped in POC 004 | Caller reset, phase regression, terminal reopening, invalid domain, incomplete report, receipt disconnection reproduced | Six counterexample-focused experiment tests within 19-test total | POC 004 `RESULTS.md`, `SHADOW_CONNECTION_FINDING.md` | Strong global claim does not hold in tested closure | `NOT_MATERIALIZED` |
| Copyable FFI representation need not copy authority | Hidden generation slot; atomic `fresh → consumed`; forged/unknown rejection | Rust-owned registry above `repr(C)` handle | POC 005 extracted focal source and authored adapter documented in `SOURCE_RUNTIME_MAP.md` | One post-authorization outcome per generation; direct write ends at `-8` hold | 15 tests; serial/concurrent/forgery cases; 3 runs | POC 005 `RESULTS.md`, `FINAL_VERDICT.txt` | Generation-local non-duplication supported; persistence jurisdiction preserved | `MATERIALIZED` |
| Authority must not duplicate across semantically equivalent fresh generations and restart | Durable semantic key + confirmed-verdict transaction + idempotent sink | Proposed Shadow transaction authority | No implemented successor source | Same payload can cross two fresh generations; restart not run | Counterexample present; crash/restart suite proposed | POC 005 `FAILURE_TO_ADVANCEMENT.md` | Strong semantic/durable theory unresolved and current semantic reading falsified | `NOT_MATERIALIZED` |
| Verified K− can re-enter and change later reasoning | Required-history gate; sealed import; match produces hard block | Shadow-origin negative asset → Quantum pre-PRO gate/filter | POC 006 extracted source/adapters in `SOURCE_RUNTIME_MAP.md` | Valid K− becomes ready and hard-blocks match; invalid required history blocks PRO | 30 tests and three principal runs | POC 006 `RESULTS.md`, `FINAL_VERDICT.txt` | Bounded detached re-entry supported | `MATERIALIZED` |
| K+ and K− are distinct governed sibling streams | Closed verdict domain; constructive and rigorous-negative admission laws | Separate constructive/negative branches beneath one superior verdict boundary | Public source mapped in POC 006 `POSITIVE_NEGATIVE_PARALLELISM.md` | Native focal branches are reported; only K− detached re-entry executes | Four native focal verdict filters plus detached K− suite | POC 006 source map and parallelism note | Domain separation supported; symmetric detached lifecycle absent | `PARTIAL` |
| A full negative commit receipt survives a detached process restart | Receipt binds asset, transaction, cycle, GCL version, export digest and commit; verify before PRO | Shadow commit receipt → sealed export → new Quantum process | Current public wire exposes Boolean persistence; typed receipt successor absent | Shadow terminate/reopen/re-import not executed | Successor process-level experiment proposed | POC 006 `FAILURE_TO_ADVANCEMENT.md` | No full receipt or detached restart proof | `NOT_MATERIALIZED` |

Status vocabulary: `MATERIALIZED`, `PARTIAL`, `DOCUMENTED_ONLY`, and
`NOT_MATERIALIZED`. These labels never apply beyond the statement in their row.

## Materialization ledger

| POC / layer | Theory or proposition | Class | Evidence path | Current status | Boundary |
| --- | --- | --- | --- | --- | --- |
| 003 | Safe initialization must preserve safety and permit bounded progress | `EXT` | `publications/executable-prior-art/poc-003-system-cold-start-reachability/PRIOR_ART.md` | Established field | Literature list is focused, not exhaustive |
| 003 | Empty generation-zero state is blocked by an exact prior-candidate requirement | `GJ` + `EXEC` | `publications/executable-prior-art/poc-003-system-cold-start-reachability/RESULTS.md` | Reproduced 3/3 | Surgical selector; no production commit |
| 003 | Exact-pair availability is the bounded causal variable at the disclosed Asht gate | `EXEC` | `publications/executable-prior-art/poc-003-system-cold-start-reachability/CLAIM_BOUNDARY.md` | Harness control passes | Does not validate candidates as production knowledge |
| 003 | GCL-authorized one-shot genesis transaction | `INNOV-P` + `GAP` | `publications/executable-prior-art/poc-003-system-cold-start-reachability/FAILURE_TO_ADVANCEMENT.md` | Proposed | Not implemented; later blockers possible |
| 004 | Local non-expansion is weaker than global continuity or termination | `EXT` | `publications/executable-prior-art/poc-004-gcl-law0-global-continuity/PRIOR_ART.md` | Established field | Candidate count is not entropy |
| 004 | Current ledger rejects direct local expansion atomically | `EXEC` | `publications/executable-prior-art/poc-004-gcl-law0-global-continuity/RESULTS.md` | Materialized in capsule | Caller reports the values |
| 004 | Current ledger enforces global cross-step and phase continuity | `GAP` | `publications/executable-prior-art/poc-004-gcl-law0-global-continuity/THEORY.md` | Not enforced | Counterexamples are passing falsification tests |
| 004 | Shadow locally binds a LAW-0 report into transaction/WAL paths | `SRC` | `publications/executable-prior-art/poc-004-gcl-law0-global-continuity/SOURCE_MAP.md` and `publications/executable-prior-art/poc-004-gcl-law0-global-continuity/CLAIM_BOUNDARY.md` | Source-mapped | Full durability route not executed by the capsule |
| 004 | Canonical cross-platform transcript bound to public receipt | `INNOV-P` + `GAP` | `publications/executable-prior-art/poc-004-gcl-law0-global-continuity/FAILURE_TO_ADVANCEMENT.md` | Components present, connection unlinked | No global receipt claim |
| 005 | Capability authority is not equivalent to an identifier or copied representation | `EXT` | `publications/executable-prior-art/poc-005-ffi-law0-authority-nonduplication/PRIOR_ART.md` | Established field | Individual primitives not claimed |
| 005 | A copied `CapHandle` does not duplicate one generation's hidden Rust authority | `GJ` + `EXEC` | `publications/executable-prior-art/poc-005-ffi-law0-authority-nonduplication/RESULTS.md` | Materialized for tested surface | In-process, generation-local result |
| 005 | Successful local FFI validation does not confer Vault write authority | `GJ` + `EXEC` | `publications/executable-prior-art/poc-005-ffi-law0-authority-nonduplication/SOURCE_RUNTIME_MAP.md` | Explicit hold at `-8` | No committed write |
| 005 | Semantic non-duplication across fresh generations | `GAP` | `publications/executable-prior-art/poc-005-ffi-law0-authority-nonduplication/FINAL_VERDICT.txt` | Counterexample found | Two fresh generations cross the gate |
| 005 | Durable semantic key consumed at idempotent Shadow sink | `INNOV-P` + `GAP` | `publications/executable-prior-art/poc-005-ffi-law0-authority-nonduplication/FAILURE_TO_ADVANCEMENT.md` | Proposed | Restart and commit recovery unexecuted |
| 006 | Failures can be retained and reused as negative knowledge | `EXT` | `publications/executable-prior-art/poc-006-living-negative-knowledge-verified-reentry/PRIOR_ART.md` | Established and adjacent prior art exists | Focused search is not exhaustive |
| 006 | K+ and K− are distinct sibling evidence channels beneath governance | `GJ` + `SRC` | `publications/executable-prior-art/poc-006-living-negative-knowledge-verified-reentry/POSITIVE_NEGATIVE_PARALLELISM.md` | Closed source domain mapped; native focal tests reported | Not symmetric detached re-entry |
| 006 | Valid required K− history becomes ready and changes a matching candidate | `GJ` + `EXEC` | `publications/executable-prior-art/poc-006-living-negative-knowledge-verified-reentry/RESULTS.md` | Materialized in detached capsule | Exact copied import/filter plus authored guard |
| 006 | Missing, corrupt, or downgraded required history blocks reasoning | `EXEC` | `publications/executable-prior-art/poc-006-living-negative-knowledge-verified-reentry/EXPERIMENT_PROTOCOL.md` | Materialized negative controls | Does not prove hostile-machine security |
| 006 | Boolean persistence proves the committed asset and transaction | `GAP` | public root `shadow-contracts/src/lib.rs` and POC 006 `SOURCE_RUNTIME_MAP.md` | Not materialized | Boolean lacks asset/transaction/export binding |
| 006 | Typed commit receipt plus detached terminate/reopen/re-import | `INNOV-P` + `GAP` | `publications/executable-prior-art/poc-006-living-negative-knowledge-verified-reentry/FAILURE_TO_ADVANCEMENT.md` | Proposed successor experiment | No current cross-process recovery proof |
| Repo-wide | GCL-rooted separation of Light, Quantum, and Shadow | `GJ` + `SRC` | `README.md`, `Cargo.toml`, and public source directories | Public architecture under active research | Source presence is not full-system execution |

## CI materialization boundary

`.github/workflows/poc-validation.yml` triggers on changes to the workflow or `publications/executable-prior-art/**`, on `main` pushes, pull requests, and manual dispatch. It grants `contents: read`, verifies the collection hash manifest, and defines one Windows job for each POC.

The workflow executes:

```powershell
# POC 003
Set-Location publications/executable-prior-art/poc-003-system-cold-start-reachability
.\scripts\verify_extraction.ps1
cargo build --workspace --all-targets --locked
cargo test --workspace --all-targets --locked

# POC 004
Set-Location publications/executable-prior-art/poc-004-gcl-law0-global-continuity
.\verify_extraction.ps1
cargo build --workspace --all-targets --locked
cargo test --workspace --all-targets --locked

# POC 005
Set-Location publications/executable-prior-art/poc-005-ffi-law0-authority-nonduplication
cargo build --workspace --all-targets --release --locked
cargo test --workspace --all-targets --release --locked -- --test-threads=1
1..3 | ForEach-Object { cargo run -p poc005-ffi-law0-experiment --release --locked }

# POC 006
Set-Location publications/executable-prior-art/poc-006-living-negative-knowledge-verified-reentry
cargo build --workspace --all-targets --release --locked
cargo test --workspace --all-targets --release --locked -- --test-threads=1
1..3 | ForEach-Object { cargo run -p poc006-living-negative-experiment --release --locked }
```

For POC 005 and 006 the workflow also verifies each capsule's `artifact_hashes.sha256` and places `CARGO_TARGET_DIR` outside the capsule. POC 003 and 004 use their extraction scripts. Collection integrity is a separate job over `publications/executable-prior-art/artifact_hashes.sha256`.

These are declared CI procedures. A workflow file proves what automation is configured to run, not that a particular remote run succeeded. The recorded result files supply maintainer evidence; an independent reviewer should retain the commit, run URL, environment, and output when reproducing them.

## Promotion criteria

Move an entry from `INNOV-P` or `GAP` to `EXEC` only when the successor evidence identifies the exact source version, pre-registers observables and falsifiers, exercises positive and negative controls, records the environment and commands, preserves earlier counterexamples, and narrows its conclusion to the path actually run. Never promote a static map to end-to-end execution or a POC aggregate to closure of v1.8.9/full architecture.
