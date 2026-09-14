# Operational label taxonomy

This repository uses a small label set to route evidence, not to reward activity or
signal popularity. Labels describe the submission kind, the public POC or scope,
the evidence relationship, and the current review state. They do not certify a
claim, a reviewer, or the project.

Repository maintainers create these labels in GitHub before enabling automatic
label assignment in forms. Submitters may suggest labels, but maintainers apply
and remove evidence and status labels after inspecting the record.

## Submission kind

Use one primary `kind/*` label. Add `kind/counterexample` only when a submission
contains an executable or otherwise inspectable contradiction, rather than a
question or an unsuccessful command alone.

| Label | Purpose and usage |
| --- | --- |
| `kind/research-question` | A precise, evidence-bound question or proposed experiment. Applied automatically by the research-question form. |
| `kind/reproducibility` | A report that reruns a disclosed procedure or a declared variation. Applied automatically by the reproducibility form. |
| `kind/technical-review` | A structured review of a public claim, method, extraction, or boundary. Applied automatically by the independent technical-review discussion form. |
| `kind/counterexample` | A repeatable result that appears to contradict an exact bounded claim. Maintainers add it only after the claim and contradictory observation are both identified. |

## POC and scope

Apply every affected `poc/*` label. Use `scope/cross-poc` for a relationship
between capsules or for collection-level metadata; it does not replace the
individual POC labels when those are known.

| Label | Purpose and usage |
| --- | --- |
| `poc/003` | POC 003: system cold-start reachability. |
| `poc/004` | POC 004: GCL LAW-0 global continuity. |
| `poc/005` | POC 005: FFI LAW-0 authority non-duplication. |
| `poc/006` | POC 006: Living Negative Knowledge verified re-entry. |
| `scope/cross-poc` | Cross-capsule behavior, collection integrity, release metadata, or a comparison involving more than one POC. |
| `scope/claim-boundary` | The supported/not-supported wording, inference boundary, or attribution of a public claim is central to the submission. |
| `scope/artifact-integrity` | Manifests, hashes, extraction identity, archive contents, or provenance are central to the submission. |
| `scope/build-tooling` | Toolchain, platform, dependency cache, build, test harness, or CI behavior is central to the submission. |

## Evidence relationship

Evidence labels describe provenance, not result quality. Do not apply
`evidence/independent` solely because a submitter selected “independent” in a
form; confirm the disclosure and that the run did not reuse maintainer-generated
results as its observation.

| Label | Purpose and usage |
| --- | --- |
| `evidence/independent` | Work performed by a reviewer who discloses no project role or material involvement in producing the evidence under review. Maintainer-confirmed. |

## Requested review domain

Use one or, exceptionally, two `review/*` labels to route a concrete task to the
right expertise. These labels mean “review requested,” never “review completed.”

| Label | Purpose and usage |
| --- | --- |
| `review/architecture` | Cross-module jurisdiction, dependency, caller, handoff, or topology review. |
| `review/formal-methods` | Translation of a bounded invariant into a model, proof obligation, counterexample search, or model-checking plan. It does not claim existing formal verification. |
| `review/rust` | Rust ownership, type, unsafe/FFI, concurrency, serialization, or toolchain assumptions. |
| `review/systems` | Process, storage, recovery, durability, scheduling, resource, or end-to-end systems behavior. |
| `review/security` | Threat modeling, authority bypass, replay, confused-deputy, tampering, or supply-chain analysis. Sensitive findings still follow `SECURITY.md`. |
| `review/reproducibility` | Clean build, locked dependency, cross-platform, artifact-identity, or independent rerun review. |
| `review/runtime` | Runtime caller closure, evidence consumption, sink behavior, or observed transition review. |
| `review/state-machine` | Illegal, unreachable, regressive, terminal, replayed, or insufficiently typed transition analysis. |
| `review/governance` | Delegation, jurisdiction, accountability, policy-to-mechanism, or AI-governance boundary review. |
| `review/knowledge-representation` | K+/K− types, admission semantics, provenance, retrieval, re-entry, or conceptual consistency review. |
| `review/adversarial` | A deliberately hostile attempt to break a named claim under declared rules and controls. |

## Open-work signal

These labels describe why an issue is open. Use `help wanted` only when an
external contributor can act from the public record. Use `good first issue` only
when the task is genuinely bounded and approachable; neither label is added for
visibility alone.

| Label | Purpose and usage |
| --- | --- |
| `review-wanted` | A maintainer has supplied enough scope and evidence for an external review to begin. |
| `open-hypothesis` | A registered, falsifiable hypothesis remains unresolved. |
| `falsification-wanted` | The issue supplies a precise claim and actively requests a counterexample or disconfirming test. |
| `implementation-gap` | A theory-to-runtime layer is absent or only source-mapped; not a generic feature request. |
| `documentation-vs-source` | The task is to reconcile a specific public statement with observable source or runtime behavior. |
| `benchmark-needed` | A named comparative or performance conclusion lacks a predeclared protocol and measurements. |
| `test-needed` | A bounded property lacks a test that can distinguish support from contradiction. |

## Review state

Use one terminal or active `status/*` label at a time, except that
`status/needs-evidence` and `status/needs-reproduction` may coexist when both
deficiencies apply. A build failure or non-reproduction is not automatically a
falsification; environment and method differences must be resolved first.

| Label | Purpose and usage |
| --- | --- |
| `status/needs-triage` | New submission awaiting scope, safety, and completeness review. Applied automatically by the forms and removed after triage. |
| `status/needs-evidence` | The exact claim, identifier, commands, outputs, or comparison needed to inspect the submission is missing. Remove when supplied. |
| `status/needs-reproduction` | A material observation or counterexample needs another clean run or an independently implemented check. |
| `status/reproduced` | The stated bounded observation was reproduced with adequately recorded inputs and environment. This does not expand the capsule claim. |
| `status/not-reproduced` | A documented result was not reproduced under the reported method. This records an outcome; it does not by itself mean the claim is false. |
| `status/inconclusive` | Available evidence cannot distinguish claim failure from environment, method, or measurement differences. |

## Triage rules

1. Start with the form-assigned kind and `status/needs-triage` labels.
2. Add the affected POC and scope labels after checking the cited public paths.
3. Treat independence as a disclosed evidence relationship, not a badge.
4. Use `kind/counterexample` only for a bounded, inspectable contradiction with
   a stated falsification criterion.
5. Replace the triage label with the narrowest supported review-state label.
6. Do not create labels for promotion, engagement, priority theater, reviewer
   prestige, or unsupported “validated” claims. Security-sensitive reports are
   redirected to the private channel in the issue-template configuration and
   are not publicly labeled.

## Evaluated but not approved now

- `formalization` and `materialization` are too ambiguous alone; use
  `review/formal-methods`, `implementation-gap`, and an exact hypothesis ID.
- `claim-evidence-gap` overlaps `scope/claim-boundary` plus
  `status/needs-evidence`.
- `performance-review` and `api-review` should be created only after a public
  benchmark/API task has a precise scope; neither currently justifies a standing
  label.
- `ai-safety-review` is deferred until a concrete safety claim or safety-case
  issue exists; use `review/governance`, `review/security`, or
  `review/formal-methods` for today's bounded questions.
- `knowledge-representation` without the `review/` prefix could be mistaken for
  a result classification rather than a request for expertise.
