# Release Provenance and Supersession

This public release candidate is capsule version `0.2.0`. It supersedes the earlier local-only capsule version `0.1.0` for publication metadata and licensing without rewriting the historical evidence.

The original local capsule's `evidence/artifact_hashes.sha256` file has SHA-256:

```text
dbc84d1402449d91ac995cf16eba6e1798a2c847ea51620e36a3a3b40579db88
```

That value records the exact prior local manifest; it is not presented as an independent public timestamp. The original 40 manifest rows and raw evidence logs are retained unchanged in the source working capsule.

## What changed in 0.2.0

- Bledar Gjata, ESS-MAI, and Gjata Legacy are made explicit in public metadata.
- Apache-2.0 is applied under the rights holder's explicit authorization.
- the canonical repository and POC path are recorded;
- the stale 16-test statement is corrected to the verified 19-test total;
- the public 2026-06-03 Business Magazine project record is linked without overstating claim-level priority;
- release status, version namespaces, and the non-disclosure boundary for the full v189 core are made explicit;
- fresh release-candidate validation evidence and a new complete artifact seal are added.

## What did not change

- the six extracted files listed in `EXTRACTION_MANIFEST.sha256`;
- the experimental Rust behavior;
- the historical raw evidence logs;
- the scientific outcome: local tuple enforcement succeeds, while global continuity, phase ordering, numeric-domain enforcement, and cross-platform binding remain only partially materialized.

The new `evidence/artifact_hashes.sha256` supersedes the old manifest only for capsule version 0.2.0. Both hashes remain useful because they show exactly which release metadata changed while production extracts stayed byte-identical.
