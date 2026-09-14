# Public research review status

Review date: 2026-09-14

Review scope: repository-visible source, POC 003–006 documentation and evidence metadata, and `.github/workflows/poc-validation.yml`

Execution performed for this review: documentation and source inspection only; no Cargo command was run

## Overall status

The public repository presents an evidence-aware experimental program with four bounded capsules. Its strongest support is for local, explicitly tested properties and reproduced counterexamples. The review does not support describing v1.8.9 or the full ESS-MAI architecture as closed, fully materialized, production-certified, formally verified, independently replicated, or security validated.

| Area | Review status | Basis |
| --- | --- | --- |
| Claim boundaries | Present for POC 003–006 | Capsule `CLAIM.md` or `CLAIM_BOUNDARY.md` files distinguish supported and unsupported inferences |
| Positive and negative evidence | Present | Each capsule publishes successful behavior and gaps; POC 006 explicitly separates K+ and K− sibling domains |
| Theory/prior-art separation | Present, focused rather than exhaustive | Each capsule includes a `PRIOR_ART.md`; POC 005/006 explicitly disclaim invention of broad primitives/concepts |
| Source provenance | Present but capsule-specific | POC 003/004 extraction identities; POC 005/006 focal-file identity and artifact hashes |
| Executable procedure | Present | Capsule reproducibility documents and public CI commands |
| CI configuration | Present for capsule integrity/build/test; repeatability runs for 005/006 | `.github/workflows/poc-validation.yml` |
| Independent replication | Not established by inspected files | Results are described as maintainer evidence; no independent run was performed in this review |
| Bit-for-bit binary reproducibility | Not established for POC 005/006 | Their result files record equal behavior with differing executable hashes across build roots |
| Full system E2E closure | Not established | Capsules exclude complete private v1.8.9 and record unexecuted or unlinked edges |

## POC status summary

| POC | Narrow reviewed result | Materialized channel | Negative result / gap | Review classification |
| --- | --- | --- | --- | --- |
| 003 | Empty cold start stops at exact-positive relevance; exact-pair control reaches post-Asht probe | `EXEC` for surgical path and control | No production commit; genesis route only proposed | Bounded causal POC; partial system materialization |
| 004 | Local non-expansion and rejection atomicity coexist with global discontinuity counterexamples | `EXEC` locally; `SRC` for mapped Shadow binding | Phase/order/domain/report/receipt continuity gaps | Partial theory materialization |
| 005 | Same-generation copied representation does not duplicate hidden in-process authority; write remains held | `EXEC` for declared FFI/test surface | Semantic reissuance, restart durability, and Vault sink absent | Narrow POC ready; global theory not materialized |
| 006 | Valid required K− history becomes ready and changes a matching candidate; invalid required history blocks PRO | `EXEC` for detached import/filter; `SRC` for broader chain | Boolean-only receipt, no detached restart, no symmetric K+ re-entry | Narrow POC ready; global theory not materialized |

## CI metadata reviewed

The workflow:

- triggers on `main` pushes and pull requests affecting `.github/workflows/poc-validation.yml` or `publications/executable-prior-art/**`, and supports manual dispatch;
- runs on `windows-latest` with read-only repository contents permission;
- verifies the collection `artifact_hashes.sha256` in a separate integrity job;
- runs extraction checks for POC 003 and 004;
- runs locked workspace build/test commands for all four POCs;
- uses release mode, single-threaded tests, external target directories, capsule hash checks, and three principal experiment launches for POC 005 and 006.

Configured commands, copied exactly in substance from `.github/workflows/poc-validation.yml`, are:

```powershell
# publications/executable-prior-art/poc-003-system-cold-start-reachability
.\scripts\verify_extraction.ps1
cargo build --workspace --all-targets --locked
cargo test --workspace --all-targets --locked

# publications/executable-prior-art/poc-004-gcl-law0-global-continuity
.\verify_extraction.ps1
cargo build --workspace --all-targets --locked
cargo test --workspace --all-targets --locked

# publications/executable-prior-art/poc-005-ffi-law0-authority-nonduplication
cargo build --workspace --all-targets --release --locked
cargo test --workspace --all-targets --release --locked -- --test-threads=1
1..3 | ForEach-Object { cargo run -p poc005-ffi-law0-experiment --release --locked }

# publications/executable-prior-art/poc-006-living-negative-knowledge-verified-reentry
cargo build --workspace --all-targets --release --locked
cargo test --workspace --all-targets --release --locked -- --test-threads=1
1..3 | ForEach-Object { cargo run -p poc006-living-negative-experiment --release --locked }
```

Review caveat: inspecting a workflow confirms configured procedure, not remote execution status. A future release review should cite the immutable commit/tag and the corresponding GitHub Actions run URL, then compare its outputs with the capsule result files.

## Evidence hierarchy used in review

1. Executed capsule outputs and tests, limited to their inputs and assertions.
2. Integrity/extraction identity for the exact disclosed source.
3. Static source/runtime maps for declared but unexecuted production edges.
4. Repository architecture and metadata for project organization and attribution.
5. External literature and publisher records for context and chronology, not runtime proof.

No lower tier is promoted into a higher tier by prose. In particular, a source map does not prove execution, a CI file does not prove a completed run, an article does not prove a mechanism, and a DOI does not prove novelty.

## Open review actions

- Independently reproduce each capsule from a pinned public commit and retain complete logs and environment metadata.
- For POC 003, execute a successor genesis experiment through an actual Shadow transaction and first ordinary post-genesis cycle.
- For POC 004, test a versioned canonical transcript across real process boundaries and post-commit receipt verification.
- For POC 005, test durable semantic idempotency, crash recovery, and the authorized Shadow sink.
- For POC 006, test typed receipt verification across Shadow terminate/reopen and the subsequent Quantum process, then test K+ re-entry as a distinct sibling path.
- Add reproducible-build controls before making any bit-for-bit binary claim.
- Keep legal novelty, patentability, freedom-to-operate, security certification, and production readiness outside technical POC conclusions unless separately evaluated by qualified reviewers.

## Review verdict

The inspected material supports describing ESS-MAI as active experimental research with partial, capsule-bounded materialization. It supports treating positive and negative outcomes as sibling evidence streams under explicit governance. It does not support a claim that the complete v1.8.9 system or repo-wide architecture is closed.
