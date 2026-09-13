# ESS-MAI Executable Prior Art — POC Collection

This directory is the canonical public collection of executable ESS-MAI proof-of-concept capsules.

## Canonical identity

- Author and architect: **Bledar Gjata**
- Project: **ESS-MAI**
- Organization and affiliation: **Gjata Legacy**
- Public contact: **gjata@legacy.al**
- Canonical repository: <https://github.com/gjatalegacy-create/ESS-MAI>
- License: **Apache-2.0**

## Publication boundary

This is one collection inside one canonical repository. Every published artifact here is a surgically bounded **POC**, never a demo. The full ESS-MAI v1.8.9 core is expressly excluded from this publication boundary. A POC may include the minimum source, harness, documentation, logs, and manifests required to make its stated claim inspectable and reproducible; it must not imply that unpublished engine code is present.

C01 and C02 may be consulted as historical references. They are not evidence for any claim in this collection. Evidence must come from the relevant POC's disclosed source, executable procedure, test output, or explicitly identified primary material.

## Advancement method

ESS-MAI records both successful and failed experiments:

```text
experimental success + experimental failure = advancement method
```

A success establishes only what the disclosed experiment actually demonstrates. A failure is preserved as a typed boundary or unresolved gap. Advancement means identifying the smallest architecture-preserving connection, constraint, or experiment that can close that gap. Neither result is to be hidden or inflated.

For POC 006, positive and negative knowledge are not opposing Boolean labels.
They are distinct sibling outcomes under GCL: `K+` reinforces or expands
supported routes, while `K−` excludes rigorously verified failure routes.
`HOLD` remains separate from both. Their governed parallel use is the
advancement mechanism; it does not collapse one authority into the other.

## Validated release-candidate entries

| POC | Directory | Version | Executed result |
| --- | --- | --- | --- |
| POC 003 | `poc-003-system-cold-start-reachability/` | 0.2.0 | build PASS; 84/84 tests; empty-state gap + positive control |
| POC 004 | `poc-004-gcl-law0-global-continuity/` | 0.2.0 | build PASS; 19/19 tests; supported behavior + counterexamples |
| POC 005 | `poc-005-ffi-law0-authority-nonduplication/` | 1.0.0 | build PASS; native 8/8; detached 1/1; one-shot authority + direct-write hold |
| POC 006 | `poc-006-living-negative-knowledge-verified-reentry/` | 1.0.0 | build PASS; native runs 12/12, 19/19 and 1/1 plus four focused verdict controls; detached 30/30; verified re-entry + receipt gap |

The machine-readable inventory is in `manifest.json`. An entry in a draft manifest is not by itself evidence that a capsule has been released, tagged, archived, or assigned a DOI.

`ZENODO_DEPOSIT_METADATA.json` describes the v1.1.0 collection deposit. A DOI is inserted into the repository only after Zenodo actually issues it. The preferred archival object is the POC-only release asset, whose SHA-256 is recorded in the GitHub release notes; the private v1.8.9 source is never part of that asset.

## Reading rule

Read each capsule's own claim boundary, extraction identity, reproducibility procedure, results, known failures, and artifact hashes together. No collection-level description expands a capsule's claim.
