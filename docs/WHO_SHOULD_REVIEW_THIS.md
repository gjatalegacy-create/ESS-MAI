# Who should review this

Status date: 2026-09-14

ESS-MAI needs several bounded reviews, not one all-purpose endorsement. A successful reproduction can establish that a disclosed procedure behaves as reported in a stated environment. It cannot, by itself, establish scientific novelty, formal correctness, safety, deployability, legal compliance, peer-review status or funding eligibility.

## Review roles

| Reviewer | Minimum independence and expertise | Question to answer | Public output |
| --- | --- | --- | --- |
| Rust reproducibility reviewer | No authorship or financial stake; Rust/Cargo and Windows CI familiarity | Can the exact public release be obtained, integrity-checked, built and tested from clean state using only its instructions? | Environment, commit/tag/DOI/SWHID, commands, logs, deviations and pass/fail per step |
| Systems/formal-methods reviewer | Distributed systems, state machines, authorization or formal verification | Do the stated invariants follow from the models and executions? Which claims are tested, bounded, unproven or falsified? | Claim-to-evidence matrix, counterexamples and missing proof obligations |
| AI reasoning/planning reviewer | Symbolic or neuro-symbolic reasoning, planning and evaluation | How does the architecture compare with named baselines on causal/logical reasoning, abstraction, uncertainty and planning? | Baseline protocol, metrics, datasets, ablations, error taxonomy and reproducible results |
| Security/supply-chain reviewer | Secure Rust, threat modeling, CI/CD and artifact provenance | What are the trust boundaries and attack surfaces? Are release, dependency and workflow controls adequate? | Threat model, severity-ranked findings and remediation evidence |
| Research-software/open-science reviewer | Metadata, citation, FAIR practice and research-software packaging | Can a new user install, understand, cite, test and reuse the public scope? Are version and concept identifiers used correctly? | Metadata/packaging checklist and user-journey report |
| Domain/application reviewer | Expertise in one proposed real-world use case | Does the POC represent a real task and meaningful constraints, or only an internal demonstration? | Use-case requirements, acceptance criteria and external-data limitations |
| Ethics, fundamental-rights and AI-governance reviewer | Independent socio-technical, legal/policy and risk expertise | Are affected people, failure modes, transparency, privacy, fairness and accountability addressed for the intended use? | Impact/risk assessment with prohibited or conditional uses |
| Funding-call reviewer | Current EIC or NLnet proposal experience; no claim to speak for the funder | Does a draft answer the published call, eligibility and deliverable requirements? | Gap memo only—not an eligibility or award decision |

Recruit reviewers who can publish their identity, affiliation, conflict-of-interest statement, exact artifact identity, environment and methods. Prefer reviewers who did not author the relevant code or evidence. If a reviewer is not independent, label the activity “internal review” or “maintainer verification.”

## Recommended sequence

1. **Artifact identity:** freeze the item under review with the v1.1.1 DOI `10.5281/zenodo.22750188`, the Git tag/commit, checksums and—when reviewing the repository snapshot—`swh:1:snp:d2d862ac5122383925807c53d2ec4cda7d05b46a`.[^1][^2]
2. **Clean-room reproduction:** have an external Rust reviewer execute only documented steps and publish raw logs plus deviations.
3. **Claim audit:** have systems/formal-methods reviewers map every claim to code, tests, assumptions and counterexamples. Maintainer-run tests remain maintainer evidence until independently rerun.
4. **Comparative evaluation:** require named baselines, measurable tasks and ablations before using “advance,” “state of the art,” “deep reasoning,” “planning” or “trustworthy” as comparative conclusions.
5. **Security and governance:** perform threat modeling and supply-chain review before operational claims or public OpenSSF status signals.
6. **Application and societal review:** select one concrete use case, then review its user requirements, harms, data governance and regulatory context.
7. **Venue readiness:** only after the earlier work, evaluate JOSS or a grant call against its current official requirements.

Use these evidence labels consistently:

- **Maintainer-reported:** produced or asserted by the author/maintainer.
- **CI-observed:** produced by a named public workflow at a specified revision.
- **Independently reproduced:** rerun by an identified independent party with public methods and scope.
- **Expert-reviewed:** evaluated by an identified expert under a stated protocol; name the domain.
- **Peer reviewed:** use only after completion of a named journal or conference peer-review process.

Archive, index and discovery presence must never be relabeled as any of the last three categories.

## JOSS: future checklist, not ready today

JOSS is an appropriate **future** review target only if ESS-MAI becomes a mature research-software package with demonstrated research use. JOSS states that it uses formal, checklist-driven peer review, with reviewers checking the source, license, functionality, tests, documentation, community guidelines, paper and research significance.[^3][^4]

### Positive preparation already visible

- Public source repository with Apache-2.0 license.
- Contribution, support, security and conduct documents.
- Issue templates and a public POC validation workflow.
- Citation metadata, version DOI and concept DOI.
- Explicit limitations around maintainer evidence and private/public scope.

### Blocking or unresolved items

- **Public-history gate:** JOSS currently requires more than six months of public development with activity across that period. GitHub records this repository as created on 2026-07-14, and the currently checked-out Git commit graph begins on 2026-08-24, so this gate cannot be met on the status date.[^15][^5]
- **Research impact:** a DOI and repository are useful identifiers, not evidence that independent researchers use the software. Collect public citations, adoption, integrations or external research workflows.
- **External development:** the visible author/email variants appear to represent one maintainer. Seek genuine issue discussion, pull-request review and contributions; never manufacture activity.
- **Independent usability:** ask a colleague with no privileged setup to install and test the software, then publish the report.
- **Paper:** no JOSS `paper.md`/bibliography package is identified in the reviewed top-level materials. A future paper needs the current JOSS sections: Summary, Statement of need, State of the field, Software design, Research impact statement, AI usage disclosure, Acknowledgements, and References, without turning documentation into unsupported research claims.[^4][^5]
- **Scope and comparison:** explain the research application, intended users, relation to existing software, architectural trade-offs and why this is reusable software rather than a one-off demonstration.
- **Testability:** demonstrate objective installation and core-function verification across the supported scope; report coverage and limitations rather than extrapolating from selected POCs.
- **AI-use disclosure:** review and satisfy JOSS's current AI-usage policy at submission time.[^5]

Do not submit merely because a checklist can be completed. Re-evaluate after the public-history threshold and external-use evidence exist. Until JOSS creates a submission and begins review, say “future JOSS checklist”; after actual submission, use the precise live status; use “peer reviewed” only after acceptance.

## OpenSSF review team

Assign the following before displaying an OpenSSF signal:

- a maintainer to own repository settings and remediation;
- an independent security engineer to interpret Scorecard findings and false positives;
- a release engineer to document dependencies, signatures/provenance, CI permissions and build reproducibility; and
- a governance reviewer to examine review requirements, maintainer continuity and vulnerability response.

OpenSSF Scorecard is automated scoring intended to help users assess repository security signals; it is not a security audit.[^6] The Best Practices badge is a voluntary project self-certification with public answers; it is not peer review.[^7] Reviewers should preserve those labels even if a future score or badge is earned.

## DeepRAP 2026 review group

**Candidate alignment only.** The 2026 DeepRAP Challenge addresses reasoning, abstraction and planning beyond current paradigms, plus explainability and trustworthy cognitive AI. Expected outcomes include constrained-compute architectures, provable trustworthiness, TRL 4 real-world demonstrations, evaluation/certification methods, FAIR outputs and portfolio collaboration.[^8]

Before a go/no-go decision, convene:

- a cognitive-AI principal investigator to define the scientific advance and baselines;
- a planning/reasoning evaluation lead to design reproducible benchmarks and ablations;
- a formal-methods and AI-safety lead to specify what “provable trustworthiness” could mean;
- a real-world application partner with data, tasks and end-user access;
- an ethics/fundamental-rights lead;
- a FAIR/open-science and interoperability lead;
- an eligible-entity/consortium and Horizon Europe grants specialist; and
- a project manager to build work packages, budget, risks, IP/exploitation and portfolio commitments.

Their first task is to disprove fit where possible: test whether the work goes beyond internal deterministic POCs, reaches the call's cognitive capabilities and measurable outcomes, and can plausibly reach TRL 4. The EIC Pathfinder Challenges deadline is **28 October 2026**.[^9] The work programme allows certain eligible single entities or qualifying consortia and sets country/consortium rules; only the Funding & Tenders Portal and responsible authorities can determine whether an applicant and proposal are admissible or eligible.[^10][^11]

No reviewer should describe ESS-MAI as DeepRAP-ready, eligible, endorsed or competitive based on repository materials alone.

## NLnet review group

As of the status date, NLnet's current open-call cycle lists Restack and CodeSupply with a deadline of **3 November 2026 at 12:00 CET**.[^12][^13]

Use a small fit panel:

- an open-internet/digital-commons maintainer;
- a software packaging, SBOM or supply-chain metadata specialist;
- a downstream user or infrastructure operator; and
- a reviewer familiar with NLnet's current eligibility and Generative AI policy.

The panel should reject a generic “fund the AI architecture” framing. Restack explicitly says AI development/application/integration is complementary or orthogonal to its supported work.[^14] A possible future proposal must isolate an operational open-internet commons deliverable. CodeSupply may be worth testing only if the work directly advances current, correct, reusable packaging/supply-chain metadata or accessible datasets, rather than merely possessing `CITATION.cff` and CodeMeta files.[^12]

The current NLnet notice also says mostly LLM-generated work is not eligible and advises applicants intending to use LLMs to wait for a revised policy.[^13] Review provenance and policy compliance before preparing anything. A favorable internal fit memo is not an eligibility decision, grant review, selection or award.

## Sources

[^1]: Zenodo, ESS-MAI v1.1.1 record: https://doi.org/10.5281/zenodo.22750188
[^2]: Software Heritage, ESS-MAI snapshot: https://archive.softwareheritage.org/swh:1:snp:d2d862ac5122383925807c53d2ec4cda7d05b46a
[^3]: Journal of Open Source Software documentation: https://joss.readthedocs.io/en/latest/
[^4]: JOSS, review checklist: https://joss.readthedocs.io/en/latest/review_checklist.html
[^5]: JOSS, “Submitting a paper”: https://joss.readthedocs.io/en/latest/submitting.html
[^6]: OpenSSF, Scorecard: https://openssf.org/scorecard/
[^7]: OpenSSF, Best Practices Badge: https://openssf.org/projects/best-practices-badge/
[^8]: European Innovation Council, EIC Work Programme 2026, DeepRAP section: https://eic.ec.europa.eu/document/download/52598755-1351-4b54-b46b-e2682d0a3aec_en?filename=EIC-Work-Programme-2026.pdf
[^9]: European Innovation Council, EIC Pathfinder overview: https://eic.ec.europa.eu/eic-funding-opportunities/eic-pathfinder_en
[^10]: European Innovation Council, EIC Pathfinder Challenges 2026: https://eic.ec.europa.eu/eic-funding-opportunities/eic-pathfinder/eic-pathfinder-challenges-2026_en
[^11]: EU Funding & Tenders Portal, DeepRAP topic `HORIZON-EIC-2026-PATHFINDERCHALLENGES-01-03`: https://ec.europa.eu/info/funding-tenders/opportunities/portal/screen/opportunities/topic-details/HORIZON-EIC-2026-PATHFINDERCHALLENGES-01-03
[^12]: NLnet, “Apply for funding”: https://nlnet.nl/funding.html
[^13]: NLnet, “Apply for funding before November 3rd 2026”: https://nlnet.nl/news/2026/20260903-call.html
[^14]: NLnet, Restack: https://nlnet.nl/restack/
[^15]: GitHub REST API, ESS-MAI repository metadata (`created_at`) and local public Git commit history: https://api.github.com/repos/gjatalegacy-create/ESS-MAI
