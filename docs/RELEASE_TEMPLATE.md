# ESS-MAI technical release template

Use this format for a serious public release. Replace every placeholder with a verifiable statement or `None`; do not silently omit failures.

## Version and identity

- Version: `<version>`
- Git tag: `<tag>`
- Commit: `<full commit SHA>`
- Release date: `<YYYY-MM-DD>`
- Covered public scope: `<paths>`
- Previous published release identity: `<tag observed at commit / version DOI>`

## Architecture status

State only what changed in the public architecture. Separate `MATERIALIZED`, `PARTIAL`, `DOCUMENTED_ONLY`, and `NOT_MATERIALIZED`. Do not describe a POC closure as closure of the full ESS-MAI architecture.

## New materialization

| Claim or hypothesis ID | Source path | Runtime path | Test | Observed result |
| --- | --- | --- | --- | --- |
| `<ID>` | `<path>` | `<command/path>` | `<test>` | `<bounded result>` |

## New tests and experiments

- Exact Cargo command: `<command>`
- Toolchain: `<rustc and cargo versions>`
- Environment: `<OS/architecture>`
- Result: `<passed/failed counts and exit status>`
- Independent reproduction: `<none/link>`

## New falsifiable claims

For each claim, state the claim, tested scope, and the smallest observation that would falsify it. Link to `docs/HYPOTHESES.md` or a versioned capsule claim boundary.

## Resolved contradictions

List the prior statement, the resolving evidence, and whether the resolution changed code, documentation, or only interpretation.

## Negative results and known failures

List expected and unexpected failures separately. A test that successfully reproduces a counterexample is still a negative architectural result where applicable.

## Known limitations

Include missing process boundaries, receipt gaps, platform limits, absent formal proof, absent independent review, non-reproducible binaries, or benchmark gaps as applicable.

## Reproducibility

Provide locked build/test/run commands, source hashes, artifact hashes, and clean-target instructions. State whether the release was reproduced outside the maintainer environment.

## Review requests

Name concrete reviewer profiles, exact files, commands, claims, and attack questions. Link to issues rather than asking for generic feedback.

## Provenance and citation

- GitHub release: `<URL>`
- Version DOI: `<DOI or pending—not minted>`
- Concept DOI: `<DOI>`
- Software Heritage SWHID: `<SWHID or pending—not archived>`
- Integrity manifest: `<path and SHA-256>`

## Claim boundary

State explicitly what this release does **not** prove: compilation is not scientific validation; archival is not peer review; a scanner is not security certification; a DOI is not a novelty decision; and a bounded POC is not proof that the full architecture is complete.
