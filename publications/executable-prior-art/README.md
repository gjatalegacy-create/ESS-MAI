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

## Validated release-candidate entries

| POC | Directory | Version | Executed result |
| --- | --- | --- | --- |
| POC 003 | `poc-003-system-cold-start-reachability/` | 0.2.0 | build PASS; 84/84 tests; empty-state gap + positive control |
| POC 004 | `poc-004-gcl-law0-global-continuity/` | 0.2.0 | build PASS; 19/19 tests; supported behavior + counterexamples |

The machine-readable inventory is in `manifest.json`. An entry in a draft manifest is not by itself evidence that a capsule has been released, tagged, archived, or assigned a DOI.

`ZENODO_DEPOSIT_METADATA.json` is a prepared metadata template for a manual Zenodo Software record. The archival object must be the single final collection ZIP, not an automatically generated archive of the whole repository. No DOI is inserted until Zenodo actually reserves or issues one.

## Reading rule

Read each capsule's own claim boundary, extraction identity, reproducibility procedure, results, known failures, and artifact hashes together. No collection-level description expands a capsule's claim.
