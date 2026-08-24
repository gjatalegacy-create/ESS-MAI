# Publication Manifest

Artifact ID: `ESS-MAI-SYSTEM-POC-003`

Capsule version: `0.2.0`

Class: `SYSTEM_POC`

Status: `PUBLIC_RELEASE_CANDIDATE_NOT_YET_TAGGED`

Experimental status: `PARTIAL_SYSTEM_MATERIALIZATION_WITH_CAUSAL_CONTROL`

Author / architect: `Bledar Gjata`

Project: `ESS-MAI`

Organization / affiliation: `Gjata Legacy`

Public contact: `gjata@legacy.al`

License: `Apache-2.0`

Canonical repository: `https://github.com/gjatalegacy-create/ESS-MAI`

Canonical path: `publications/executable-prior-art/poc-003-system-cold-start-reachability`

## Publication allowlist

- root `Cargo.toml`, `Cargo.lock`, `LICENSE`, `NOTICE.md`, and `CITATION.cff`;
- root POC documents;
- `EXTRACTION_MANIFEST.sha256`;
- `scripts/verify_extraction.ps1`;
- the four disclosed crates under `crates/`;
- sanitized text evidence under `evidence/`;
- `evidence/artifact_hashes.sha256`.

Every file under `crates/` must be either:

1. a whole-file extract named in `EXTRACTION_MANIFEST.sha256`;
2. the identified excerpt carrier;
3. explicitly documented new POC glue.

## Production/glue split

```text
WHOLE_FILE_PRODUCTION_EXTRACTS=20
PRODUCTION_EXCERPTS=1
PRIVATE_FULL_CORE_INCLUDED=false
SURGICAL_SHADOW_PROCESS=new POC glue
PRODUCTION_SHADOW_COMMIT_EXECUTED=false
```

## Exclusions

- full private v1.8.9 workspace or source-structure manifest;
- complete production Light, Quantum, or Shadow source trees;
- full private Light `lgc_algorithm.rs`;
- full private Shadow `knowledge_vault.rs`;
- C01/C02;
- `target/`, `.git/`, caches, build/runs directories;
- `*.exe`, `*.dll`, `*.obj`, `*.pdb`, `*.rlib`, `*.rmeta`;
- `*.hold`, `*.wal`, vault files, handoff directories;
- credentials, private keys, tokens, environment dumps, and private absolute paths;
- DOI, tag, commit, release, or archive identifiers that do not yet exist.

## Integrity

- Extract identity: `EXTRACTION_MANIFEST.sha256`.
- Complete capsule identity: `evidence/artifact_hashes.sha256`.

The complete seal excludes itself, generated artifacts, and repository metadata.

## Publication truth

Creating a local seal is not public disclosure by itself. The GitHub commit/tag and any Zenodo DOI must be recorded only after the external service actually creates them. This file does not claim an immutable release or DOI.
