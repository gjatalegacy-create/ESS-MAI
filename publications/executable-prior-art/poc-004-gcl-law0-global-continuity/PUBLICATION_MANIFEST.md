# Publication Manifest

Artifact: `gcl_law0_global_uncertainty_continuity`

Capsule version: `0.2.0`

Author and architect: `Bledar Gjata`

Organization: `Gjata Legacy`

Canonical repository: `https://github.com/gjatalegacy-create/ESS-MAI`

Canonical repository path: `publications/executable-prior-art/poc-004-gcl-law0-global-continuity`

Class: `THEORY_POC`

Local verification status: `LOCALLY_REPRODUCED_PARTIAL_MATERIALIZATION`

Public release status: `RELEASE_CANDIDATE_NOT_YET_TAGGED`

License: `Apache-2.0`

## Publication allowlist

The intended source capsule consists of:

- root Cargo manifest and lockfile;
- Apache-2.0 `LICENSE` and `NOTICE.md`;
- `.gitignore`;
- `extracted/gcl-constitution/Cargo.toml`;
- the four extracted Rust source files;
- `extracted/shadow_verification_receipt.rs`;
- `experiment/Cargo.toml` and `experiment/src/main.rs`;
- `EXTRACTION_MANIFEST.sha256` and `verify_extraction.ps1`;
- the named root Markdown documentation listed below;
- `CITATION.cff`;
- historical text evidence `evidence/00_...` through `evidence/09_...`;
- fresh release-candidate evidence `evidence/10_release_candidate_validation_2026-08-24.txt`;
- `evidence/artifact_hashes.sha256`.

Named root Markdown files:

```text
CLAIM_BOUNDARY.md
EXPERIMENT_PROTOCOL.md
EXTRACTION_IDENTITY.md
FAILURE_TO_ADVANCEMENT.md
NOTICE.md
POC_PROTOCOL.md
PRIOR_ART.md
PRIORITY_TIMELINE.md
PUBLICATION_MANIFEST.md
README.md
RELEASE_PROVENANCE.md
REPRODUCIBILITY.md
RESULTS.md
SHADOW_CONNECTION_FINDING.md
SOURCE_MAP.md
THEORY.md
WORKLOG.md
```

## Exclusions

- `target/` and all compiled artifacts;
- `.git/` metadata;
- v189 full source, archives, and unrelated ESS-MAI documentation;
- machine-specific caches;
- secrets, credentials, private keys, tokens, and environment dumps;
- any DOI, repository URL, archive ID, or timestamp evidence that does not yet exist.

## Provenance split

```text
EXTRACTED_PRODUCTION_SOURCE = byte-identical five-file GCL crate closure
EXTRACTED_SHADOW_SOURCE     = byte-identical receipt algorithm
EXPERIMENTAL_GLUE           = new experiment and harness-only connection probe
DOCUMENTATION               = new bounded publication analysis
```

## Integrity

- Per-origin extraction hashes: `EXTRACTION_MANIFEST.sha256`.
- Publication file hashes: `evidence/artifact_hashes.sha256`.
- v189 before/after tree digest: `evidence/05_source_integrity.txt`.

Generated `target/` content is deliberately not part of the cryptographic publication identity.

## Release state

- Apache-2.0 has been selected by the rights holder for this public capsule.
- Public disclosure of this surgical POC and the public contact `gjata@legacy.al` has been explicitly authorized.
- The full v189 source workspace is not part of this capsule and must not be uploaded.
- No release tag, Zenodo deposit, or DOI may be claimed until it actually exists.
- The 2026-06-03 Business Magazine URL and publication date are independently retrievable, but a claim-by-claim mapping from that article to this bounded LAW-0 POC has not been established.

The remaining release work is mechanical verification, resealing, repository commit, immutable tag, and optional independent archival deposit. Until those steps complete, this file deliberately uses `RELEASE_CANDIDATE_NOT_YET_TAGGED`.
