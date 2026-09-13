# ESS-MAI Executable Prior Art v1.1.0

**Author:** Bledar Gjata  
**Organization:** Gjata Legacy  
**License:** Apache-2.0  
**Release date:** 2026-09-13

This release adds two surgically bounded theory POCs to the existing POC 003
and POC 004 collection. It does not publish the complete private ESS-MAI v1.8.9
source tree.

## POC-005 — FFI LAW-0 Authority Non-Duplication

Positive result: a C-compatible handle can be copied, while its Rust-owned
authorization is consumed once. Native v189 focal tests pass 8/8; the detached
workspace test passes 1/1. Eight concurrent copies produce one
post-authorization result and seven replay refusals.

Negative result: the authorized non-empty legacy FFI call ends at
`CompatibilityHold (-8)`; a direct knowledge write is not materialized.

Advancement: bind the one-shot gate to an already committed constitutional
verdict and durable monotonic transaction state. Do not bypass the hold.

## POC-006 — Living Negative Knowledge: Verified Re-entry

Positive result: valid required negative history imports as `READY`; the known
failure is hard-blocked at score zero. Native runs pass 12/12, 19/19 and 1/1;
four additional focused verdict filters each pass 1/1 and are not added into an
inflated unique-test total. The detached workspace passes 30/30.

Negative result: the cross-subsystem wire principally exposes
`negative_persisted: bool`, not a full independently verifiable commit receipt.
The reproduced FNV seal is not claimed as cryptographic integrity.

Advancement: transport a versioned `NegativeCommitReceiptWire` created only
after the existing Shadow Vault commit and independently verified by Quantum
before the existing PRO activation point. Shadow remains the sole writer.

Parallel authority result: v189 source and focused native tests expose two
closed sibling outcomes under GCL/Shadow. `Constructive` maps to
`KnowledgeWrite::Primitive` (`K+`), while `RigorousNegative` maps to
`KnowledgeWrite::Negative` (`K−`). Positive knowledge reinforces supported
routes; negative knowledge excludes verified failures. `HOLD` is neither.
POC-006 executes the negative re-entry half and does not claim a symmetric
positive-history re-entry experiment.

Public trace: Business Magazine published the broad ESS-MAI “Dija Negative”
concept under Bledar Gjata's name on 2026-06-03. The record predates the
2026-06-19 arXiv v1 of Wang's Negative Knowledge paper by 16 calendar days.
This is recorded as bounded public-disclosure chronology, not as proof that the
full Living Negative mechanism was already disclosed or implemented.

## Claim boundary

Negative Knowledge, capability systems, single-use gates, atomic consumption,
and runtime governance all have relevant prior art. This release claims only
the disclosed ESS-MAI source composition and its measured partial
materialization. It is executable prior-art evidence, not a legal novelty or
patentability opinion.

## Integrity roots

```text
POC-005 artifact_hashes.sha256 SHA-256:
26cba0e9c9f01996500ed5be50fd45de275c9028c06ee239e26d38f8a32a72b8

POC-006 artifact_hashes.sha256 SHA-256:
ec061d227521e4ecd66426dfd1fa2841c72b60fb0c1c543013f063adc746ede4
```

The POC-only archive hash and Git commit are added to the GitHub release body
after the final archive is generated from the tagged collection.
