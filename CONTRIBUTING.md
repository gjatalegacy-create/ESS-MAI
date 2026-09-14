# Contributing to ESS-MAI public research

Thank you for examining ESS-MAI. The most useful contributions are evidence-bound: independent reproductions, precise documentation corrections, counterexamples, claim-boundary analysis, and small tests that clarify a disclosed POC.

## Public contribution scope

Contributions may address:

- POC 003, POC 004, POC 005, or POC 006 reproducibility;
- disclosed Rust source and Cargo behavior;
- extraction identity and artifact-integrity checks;
- prior-art attribution or citation corrections;
- experimental success, failure, and advancement-method documentation;
- narrowly scoped tests that do not alter the ESS-MAI authority hierarchy.

The full private ESS-MAI v1.8.9 workspace is not part of this repository's public POC boundary. Do not request or submit private credentials, unpublished source, personal data, or third-party confidential material.

## Start with evidence

Before opening an issue or pull request:

1. read the relevant `README.md`, `CLAIM_BOUNDARY.md`, `PRIOR_ART.md`, and `REPRODUCIBILITY.md`;
2. verify the capsule's SHA-256 manifests;
3. run the documented locked Cargo build and test commands;
4. record the operating system, `rustc --version`, `cargo --version`, exact command, and complete result;
5. distinguish observed behavior from interpretation or a proposed future variant.

Use the repository's **Reproducibility report** or **Research question** issue form whenever possible.

## Ways to Contribute Without Accepting the Theory

You do not need to agree with Gjata Collapse Law, the ESS-MAI authority model, or the project's terminology to make a valuable contribution. A result that narrows, contradicts, or disproves a claim is useful when its evidence and scope are clear.

Legitimate entry points include:

- **Review only:** identify an overbroad claim, ambiguous term, missing citation, or unsupported documentation statement without changing code.
- **Reproduce:** run an exact locked Cargo procedure and report the environment, command, exit status, and output—even when the result differs from the maintainer record.
- **Report failure:** preserve a minimal failing input or counterexample and state which bounded claim it affects.
- **Challenge theory:** select an ID from `docs/HYPOTHESES.md`, explain the challenged premise, and provide a falsification argument or test design.
- **Test a hypothesis:** add the smallest test that distinguishes the claim from a plausible alternative explanation.
- **Review Rust:** inspect ownership, concurrency, FFI, process, serialization, persistence, and state assumptions in public source.
- **Review security:** look for authority bypass, artifact substitution, replay, receipt forgery, confused-deputy behavior, or fail-open paths. Report sensitive findings through `SECURITY.md`.
- **Review formalization:** translate a bounded invariant into a precise model and identify assumptions that the executable test leaves implicit. Do not label a model “verified” without a real proof or model-check result.
- **Improve documentation:** reconcile terminology and claims with exact source paths and test observations.
- **Add a test or benchmark:** define the property, baseline, environment, limits, and falsification threshold before reporting a number.
- **Reproduce an experiment independently:** avoid relying on maintainer build caches or unpublished files, and disclose every deviation from the documented procedure.

When a contribution disproves or materially narrows an ESS-MAI claim, the project should preserve that negative result in `docs/NEGATIVE_RESULTS.md` and attribute the contributor in the issue, pull request, release notes, or resulting research artifact as appropriate. Attribution never converts a result into endorsement by either party.

## Sealed POC rule

Do not silently rewrite a sealed POC in place. A correction that changes covered bytes must preserve the earlier hashes and be proposed as a clearly versioned correction or successor capsule. Evidence logs, success counts, failures, and claim boundaries must remain mutually consistent.

## Pull-request requirements

A pull request should:

- describe the exact public claim or reproducibility problem it addresses;
- avoid expanding a claim beyond the disclosed experiment;
- preserve GCL as the constitutional authority root and preserve the bounded jurisdictions below it;
- include the smallest relevant change;
- pass the affected Cargo build and tests;
- update documentation and integrity records when the proposed versioning method requires it;
- contain no generated targets, binaries, caches, secrets, local absolute paths, or private workspace material.

## Communication standard

Be precise, respectful, and evidence-first. Criticism, failed reproduction, and counterexamples are welcome when they include enough information to inspect. Promotional claims, personal attacks, and unverifiable assertions are not useful research contributions.

Security-sensitive findings should be reported privately according to [SECURITY.md](SECURITY.md).
