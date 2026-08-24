# Contributing to ESS-MAI public research

Thank you for examining ESS-MAI. The most useful contributions are evidence-bound: independent reproductions, precise documentation corrections, counterexamples, claim-boundary analysis, and small tests that clarify a disclosed POC.

## Public contribution scope

Contributions may address:

- POC 003 or POC 004 reproducibility;
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

