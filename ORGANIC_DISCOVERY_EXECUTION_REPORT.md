# ESS-MAI Phase 16 organic-discovery execution report

Execution date: 2026-09-14

Repository: `https://github.com/gjatalegacy-create/ESS-MAI`

Phase 16 establishes legitimate discovery and external-review entry points. It
does not manufacture popularity and does not treat metadata, archives, CI,
automated scores, or grant-call vocabulary as external validation.

## Executed repository changes

- Published research-domain, hypothesis, theory-to-materialization,
  negative-result, review-status, reviewer, topic, external-platform, badge, and
  release-template documentation in commit `152a3ad`.
- Added README sections for Research Domains, Engineering Domains, Review
  Domains, Experimental Questions, Implementation Status, Falsifiable Claims,
  and Open Hypotheses.
- Added `CONTRIBUTING.md` section “Ways to Contribute Without Accepting the
  Theory.”
- Extended research-question and reproducibility issue forms to POC 003–006 and
  added a structured independent-review discussion form.
- Updated `CITATION.cff` and `codemeta.json` with narrower, factual research
  terminology. No DOI, ORCID, affiliation, acceptance, or review was invented.
- Left every file under `publications/executable-prior-art/` unchanged; the
  immutable v1.1.1 tag remains at `d38c2ba`.

## Final repository description

> Experimental Rust research software for explicit authority boundaries,
> negative knowledge, traceable state transitions, and reproducible
> falsification POCs.

Two ranked alternatives and the selection rationale are recorded in
`docs/DISCOVERABILITY_TOPIC_AUDIT.md`.

## Final GitHub topics

The exact 20-topic set was applied through the GitHub API:

`rust`, `systems-programming`, `research-software`, `software-architecture`,
`systems-engineering`, `systems-research`, `deterministic-systems`,
`runtime-verification`, `state-machines`, `reproducibility`, `open-science`,
`knowledge-representation`, `reasoning-systems`, `ai-governance`,
`formal-methods`, `hypothesis-testing`, `traceability`,
`executable-prior-art`, `negative-knowledge`, `ess-mai`.

`formal-methods` is retained only because the repository exposes bounded
invariants, transitions, counterexamples, and proof obligations suitable for
formal-methods review. No formal verification or model-checking result is
claimed.

## Rejected or removed topics

Rejected because they would overstate present evidence: `formal-verification`,
`model-checking`, `theorem-proving`, `formal-ai-governance`, and `sovereign-ai`.

Deferred until external evidence or a concrete review artifact exists:
`trustworthy-ai`, `explainable-ai`, `ai-safety`, `security`, `cybersecurity`,
`epistemic-reasoning`, `decision-systems`, and `scientific-software`.

Removed as too broad, redundant, or low-value under the 20-topic limit:
`artificial-intelligence`, `research`, `deterministic-ai`, `traceable-ai`,
`hierarchical-authority`, `bledar-gjata`, and `gjata-legacy`. Authorship and
organization remain prominent in the README, citation metadata, DOI record, and
repository identity.

The full evidence/risk matrix is in `docs/DISCOVERABILITY_TOPIC_AUDIT.md`.

## Labels configured

Thirty-seven operational labels were created or normalized:

- submission: `kind/research-question`, `kind/reproducibility`,
  `kind/technical-review`, `kind/counterexample`;
- POC/scope: `poc/003`–`poc/006`, `scope/cross-poc`,
  `scope/claim-boundary`, `scope/artifact-integrity`, `scope/build-tooling`;
- evidence/status: `evidence/independent`, `status/needs-triage`,
  `status/needs-evidence`, `status/needs-reproduction`, `status/reproduced`,
  `status/not-reproduced`, `status/inconclusive`;
- review domains: architecture, formal methods, Rust, systems, security,
  reproducibility, runtime, state machine, governance, knowledge
  representation, and adversarial review under `review/*`;
- work signals: `review-wanted`, `open-hypothesis`,
  `falsification-wanted`, `implementation-gap`, `documentation-vs-source`,
  `benchmark-needed`, and `test-needed`.

The existing `help wanted` label is used only for actionable public tasks;
`good first issue` is used only on issue #10, a genuinely bounded four-row
documentation cross-check. Meanings and rejected label candidates are in
`docs/LABEL_TAXONOMY.md`.

## Review issues created or upgraded

- [#3 — Independent reproduction index, POC 003–006](https://github.com/gjatalegacy-create/ESS-MAI/issues/3) (upgraded from the POC 003/004 index)
- [#4 — Formal-methods authority-transition review](https://github.com/gjatalegacy-create/ESS-MAI/issues/4)
- [#5 — Global LAW-0 state-machine review](https://github.com/gjatalegacy-create/ESS-MAI/issues/5)
- [#6 — Semantic/durable FFI authority review](https://github.com/gjatalegacy-create/ESS-MAI/issues/6)
- [#7 — Full K− receipt and detached restart falsification](https://github.com/gjatalegacy-create/ESS-MAI/issues/7)
- [#8 — Cold-start genesis architecture criticism](https://github.com/gjatalegacy-create/ESS-MAI/issues/8)
- [#9 — Adversarial artifact/receipt/authority substitution](https://github.com/gjatalegacy-create/ESS-MAI/issues/9)
- [#10 — README/capsule consistency check](https://github.com/gjatalegacy-create/ESS-MAI/issues/10)
- [#11 — Falsifiable benchmark-protocol design](https://github.com/gjatalegacy-create/ESS-MAI/issues/11)

Every issue identifies public starting files, a bounded task, useful output, and
claim limits. No issue requires access to the private workspace.

## GitHub Discussions

GitHub Discussions was enabled. The initial public discussion is:

- [#12 — ESS-MAI — Independent Technical Review Requested](https://github.com/gjatalegacy-create/ESS-MAI/discussions/12)

It explicitly invites disagreement and asks which claim is unsupported, which
invariant can be broken, what cannot be reproduced, what should be removed,
proved, benchmarked, or rewritten, and what evidence would change a conclusion.

Default `Ideas` and `Q&A` categories are available. A structured
`independent-technical-review` discussion form is committed. Specialized
Architecture, Formal Methods, Hypotheses & Falsification, Research Questions,
Reproducibility, Rust, Security, and Theory → Materialization categories are
prepared for GitHub UI configuration; category creation has no public REST or
GraphQL mutation.

## Academic and research-software discovery assets

- `CITATION.cff`: author Bledar Gjata, Gjata Legacy affiliation, Apache-2.0,
  repository URL, bounded abstract/keywords, and exact v1.1.1 reference.
- `codemeta.json`: machine-readable software identity, author, language,
  license, research classification, and POC 003–006 components.
- Zenodo exact v1.1.1 DOI: `10.5281/zenodo.22750188`.
- Zenodo all-versions DOI: `10.5281/zenodo.22074027`.
- Software Heritage repository snapshot:
  `swh:1:snp:d2d862ac5122383925807c53d2ec4cda7d05b46a`.
- OpenAIRE discovery route for the version DOI.
- A future JOSS checklist is documented, but ESS-MAI is **not ready** and no
  submission or peer-review claim was made.

The 3 June 2026 media trace remains attribution/context for the broad ESS-MAI
negative-knowledge concept. It is not substituted for later source hashes,
Cargo results, or mechanism-complete proof.

## External-platform readiness

`docs/EXTERNAL_DISCOVERY_GRAPH.md` distinguishes:

- GitHub as repository/collaboration/CI;
- Zenodo as DOI deposit/version archive;
- Software Heritage as content-addressed source preservation;
- OpenAIRE as indexing/discovery;
- OpenSSF Scorecard as future automated security-signal assessment;
- OpenSSF Best Practices as future public self-certification;
- JOSS as a future formal peer-review route;
- ORCID, OSF, and HAL as conditional identity/deposit routes;
- EIC Pathfinder Challenges DeepRAP and NLnet as opportunity assessments, not
  endorsements or eligibility decisions.

No new third-party account, grant application, JOSS submission, OSF/HAL record,
OpenSSF enrollment, or badge was created. Grant names were not inserted as
GitHub topics because that would manufacture relevance. DeepRAP has candidate
thematic alignment and serious TRL 4, benchmark, trustworthiness, team, and
eligibility gaps; the current official deadline is 28 October 2026. NLnet's
current Restack/CodeSupply cycle requires a separately scoped fit assessment,
with 3 November 2026 12:00 CET recorded as the present deadline.

## Badge readiness

Allowed and currently used: POC workflow status, Apache-2.0, Zenodo concept DOI,
and Software Heritage archive. Their meanings are bounded in
`docs/BADGE_POLICY.md`.

Not displayed because not earned: OpenSSF Scorecard, OpenSSF Best Practices,
JOSS status, code coverage, “peer reviewed,” “certified,” “validated,” or any
grant/funder badge.

## Search surfaces established

Substantive pages now exist for Rust runtime verification, Rust state machines,
reproducible Cargo experiments, negative knowledge, K+/K− sibling evidence,
AI-governance relevance, formal-methods evaluation, evidence traceability,
authority boundaries, architecture criticism, and research-software citation.
The exact query-to-content and overclaim matrix is in
`docs/DISCOVERABILITY_TOPIC_AUDIT.md`; no keyword-only page was added.

## Missing signals

- independent reproduction and external technical review;
- a machine-checked model/proof and implementation-refinement argument;
- external security/threat-model review;
- protected branch/review requirement, dependency automation, static analysis,
  secret scanning, fuzzing, coverage, SBOM/provenance, and signed-release proof;
- cross-platform CI and bit-for-bit reproducible binaries;
- predeclared comparative benchmarks;
- six months of public development plus independent research use for JOSS;
- legal-entity, consortium, work-package, TRL 4, benchmark, ethics, exploitation,
  and proposal evidence for DeepRAP.

## Next highest-value organic actions

1. Obtain one genuine independent reproduction through issue #3 and preserve
   PASS, PARTIAL, NOT REPRODUCED, FALSIFIED, or INCONCLUSIVE with equal rigor.
2. Complete one formal-methods attack from issue #4 or #5.
3. Complete one adversarial threat-model review from issue #9 before displaying
   any security-quality badge.
4. Add cross-platform CI and conservative supply-chain controls in a separately
   reviewed change; run OpenSSF locally first and publish its negative findings.
5. Build successor POCs for the four named implementation gaps without
   rewriting the immutable v1.1.1 evidence.
6. Define a single real application task and baseline protocol before making
   DeepRAP capability or TRL claims.
7. Earn external use, citations, issues, and pull requests through useful
   evidence—not unsolicited promotion, exchanges, fake accounts, or purchased
   metrics.

Current measured readiness is recorded in
`ORGANIC_DISCOVERY_READINESS.md` as 80/100, with evidence and deductions for
every dimension.
