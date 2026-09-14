# External discovery graph

Status date: 2026-09-14

This map describes how a reader can find and identify the public ESS-MAI material. It is not a quality ranking. An archive preserves, an index helps discovery, a repository supports development, a self-assessment records maintainer assertions, peer review evaluates a submission, and a grant portal receives a proposal. None of those functions should be substituted for another.

## Canonical public nodes

```text
GitHub repository and tagged release
        | source, history, issues, CI
        +----------------------+----------------------+
        |                                             |
        v                                             v
Zenodo version record                         Software Heritage snapshot
10.5281/zenodo.22750188                       swh:1:snp:d2d862ac5122383925807c53d2ec4cda7d05b46a
        | exact deposited version                     | content-derived archive identity
        v                                             |
Zenodo concept DOI                                    |
10.5281/zenodo.22074027                               |
        | all-versions/latest-version resolver         |
        +----------------------+----------------------+
                               v
                    OpenAIRE discovery/index record
```

- **GitHub is the development repository.** It exposes the public source, revision history, releases, issue templates and the repository's validation workflow. A public repository and a passing workflow run make work inspectable; they do not independently validate the project's scientific claims.[^1]
- **Zenodo is the deposited-record/archive layer.** DOI `10.5281/zenodo.22750188` identifies the v1.1.1 record. DOI `10.5281/zenodo.22074027` is the all-versions concept DOI and should be used when a citation is intended to follow the version set rather than freeze v1.1.1. Zenodo describes each version as a separate record with its own persistent identifier and files.[^2][^3]
- **Software Heritage is the source-code preservation layer.** `swh:1:snp:d2d862ac5122383925807c53d2ec4cda7d05b46a` is a snapshot-type SWHID (`snp`). A SWHID identifies an archived software object by intrinsic content-derived identity; a snapshot SWHID is not a review or endorsement.[^4][^5]
- **OpenAIRE is an indexing and discovery layer.** The project has a result URL keyed to the v1.1.1 DOI. OpenAIRE describes its graph and Explore service as research discovery infrastructure, including research-software records. Presence there does not mean OpenAIRE peer-reviewed or certified ESS-MAI.[^6][^7]

When naming an exact experimental release, cite the v1.1.1 DOI. When naming the evolving Zenodo version family, cite the concept DOI. When proving which archived repository snapshot is meant, cite the snapshot SWHID. When inviting code inspection or participation, link GitHub.

## What each external surface does—and does not do

| Surface | Function | Present public fact | Not implied |
| --- | --- | --- | --- |
| GitHub | Repository and collaboration | Public ESS-MAI repository, tags, releases, issues and Actions surface | Independent replication, scientific review, security certification |
| Zenodo v1.1.1 | Archive/record | Version DOI `10.5281/zenodo.22750188` | Journal publication, novelty finding, peer review |
| Zenodo concept DOI | Version-family identifier | Concept DOI `10.5281/zenodo.22074027` | That every version has identical files or results |
| Software Heritage | Source archive | Snapshot SWHID `swh:1:snp:d2d862ac5122383925807c53d2ec4cda7d05b46a` | Correctness, reproducibility, security or authorship adjudication |
| OpenAIRE | Index/discovery graph | Search/result route for the Zenodo DOI | Validation, eligibility, endorsement or peer review |
| ORCID | Researcher identity and work linking | A DOI can be added to an author's ORCID record | Verification of the work's claims or of unverified manually entered contributors |
| OSF | Registration/preprint infrastructure | Optional future venue for a genuinely distinct study plan or manuscript | A registration is not peer review; a preprint is explicitly pre-peer-review |
| HAL | Open archive | Optional future repository if its scope and depositor conditions are met | Journal acceptance or external validation |
| JOSS | Journal peer review | Future checklist only; no submission or acceptance is claimed | Readiness, review, acceptance or a JOSS paper DOI |
| OpenSSF Scorecard | Automated repository security signals | Future readiness target; no project result or badge is claimed | Security certification or a guarantee of safe software |
| OpenSSF Best Practices | Voluntary self-certification | Future readiness target; no enrollment or badge is claimed | Independent audit or peer review |
| EIC/NLnet | Grant portals and competitive selection | Candidate opportunity mapping only | Eligibility, fit, grant readiness, selection or funding |

ORCID can improve author-to-work discovery by linking a DOI to the author's record, but no ORCID iD should be invented or inferred.[^8] OSF should be used only for a distinct registration or manuscript workflow: OSF distinguishes a registration from a preprint and states that a preprint is shared without formal peer review.[^9] HAL describes itself as a multidisciplinary open archive that includes software; a HAL deposit would be another archive/discovery surface, not a review outcome.[^10]

## Future formal-review route: JOSS

JOSS is a journal with formal peer review, but ESS-MAI should be represented only as a **future checklist candidate**. Current JOSS screening expects, among other things, more than six months of public development history, demonstrated research impact, a full-featured maintainable research package, public contribution paths, tests and documentation, and a paper in the repository.[^11][^12]

The visible checkout history begins on 2026-07-15, less than six months before this status date. The repository has an Apache-2.0 license, contribution/support/security files and a public validation workflow, but those positive signals do not cure the history gate. No JOSS submission, pre-review issue, reviewer assignment or acceptance is asserted. See `WHO_SHOULD_REVIEW_THIS.md` for a staged independent-review plan.

## Future quality-signal route: OpenSSF

OpenSSF Scorecard automatically evaluates repository security signals. The OpenSSF Best Practices program is different: projects voluntarily self-certify their answers, with some automated checking and public scrutiny.[^13][^14] ESS-MAI has not claimed either signal.

Prudent next steps are to run a private/local gap assessment first, document every failing or unknown check, and add a public Scorecard workflow or Best Practices project only after the maintainer deliberately opts in. A score or badge, if later earned and linked to its evidence page, still must not be described as a security audit or scientific validation.

## Grant-portal watchlist

### EIC Pathfinder Challenges 2026 — DeepRAP

**Judgment: candidate thematic alignment, not application readiness.**

The official DeepRAP scope calls for advances in deep reasoning, abstraction or planning, with transparent and explainable rationales and trustworthy operation. It expects proposals to deliver models or architectures handling knowledge and uncertainty, provable trustworthiness mechanisms, a cognitive-AI demonstration reaching TRL 4 on complex real-world tasks, new evaluation/certification methods, FAIR outputs, and portfolio collaboration.[^15]

ESS-MAI's public descriptions of deterministic transitions, bounded authority, traceability, fail-closed behavior and negative knowledge are relevant vocabulary for reasoning and trustworthiness. The disclosed POCs may be useful preliminary evidence. That is only a paper-screen alignment. The repository does not by itself establish:

- a state-of-the-art advance against named DeepRAP baselines;
- deep abstraction or long-horizon planning performance;
- multimodal and uncertainty-aware model training/deployment under constrained compute;
- a TRL 4 integrated cognitive-AI system on complex real-world tasks and scaled simulations;
- provable coverage of explainability, fairness, security, fundamental rights and EU AI Act obligations;
- shared benchmarks, an open evaluation platform, interoperability work or portfolio pilots;
- the required scientific, engineering, application, ethics and exploitation team; or
- legal-entity/consortium eligibility, budget, work packages, IP plan and portal completeness.

The EIC work programme gives **28 October 2026** as the Pathfinder Challenges deadline and describes grants up to EUR 4 million, with higher amounts only if duly justified. It permits an eligible single legal entity or qualifying consortia under the stated country rules.[^16][^17] Treat the Funding & Tenders topic page as the controlling submission surface and recheck its current call conditions before any decision.[^18] Nothing in this document is an eligibility determination, proposal, endorsement or promise of funding.

### NLnet — current open calls

**Judgment: weak as a general AI proposal; potentially narrower fit only for an open-internet or software-supply-chain work package.**

As of the status date, NLnet lists Restack and CodeSupply as open, with the next deadline **3 November 2026 at 12:00 CET**.[^19][^20] Restack targets deployable, scalable, secure internet commons and explicitly says AI development, application or integration is complementary/orthogonal to the work it supports.[^21] Therefore an ESS-MAI proposal centered on a cognitive-AI architecture is not yet a demonstrated Restack fit.

A narrower, separately scoped deliverable could be assessed if it directly strengthens open internet infrastructure or a reusable digital commons—for example, deployment hardening, interoperability, packaging or reproducible infrastructure. CodeSupply is more plausibly adjacent to the repository's `CITATION.cff`, CodeMeta and release-integrity metadata, but its stated focus is software-supply-chain data, packaging metadata and democratic access to datasets. Existing metadata files alone do not establish fit.[^19]

Before considering NLnet, define a concise milestone-based FOSS work package, name downstream users, compare existing alternatives, show public benefit and sustainability, and audit the project's AI-development history against NLnet's current Generative AI policy. NLnet's September 2026 call notice says mostly LLM-generated work is not eligible and advises AI-using applicants to await the revised policy.[^20] No proposal has been submitted and no eligibility, selection or grant readiness is claimed.

## Sources

[^1]: GitHub, public ESS-MAI repository: https://github.com/gjatalegacy-create/ESS-MAI
[^2]: Zenodo, ESS-MAI v1.1.1 record: https://doi.org/10.5281/zenodo.22750188
[^3]: Zenodo, “Manage versions”: https://help.zenodo.org/docs/deposit/manage-versions/
[^4]: Software Heritage, ESS-MAI snapshot: https://archive.softwareheritage.org/swh:1:snp:d2d862ac5122383925807c53d2ec4cda7d05b46a
[^5]: Software Heritage, SWHID documentation: https://docs.softwareheritage.org/devel/swh-model/persistent-identifiers.html
[^6]: OpenAIRE Explore, ESS-MAI v1.1.1 result route: https://explore.openaire.eu/search/result?pid=10.5281%2Fzenodo.22750188
[^7]: OpenAIRE Explore: https://explore.openaire.eu/
[^8]: ORCID, “Add works using an identifier”: https://support.orcid.org/hc/en-us/articles/360022298153-Add-works-using-an-identifier
[^9]: OSF Support, “OSF Projects Transition FAQs”: https://help.osf.io/article/725-faqs
[^10]: HAL, “About HAL”: https://about.hal.science/en/
[^11]: JOSS, “Submitting a paper”: https://joss.readthedocs.io/en/latest/submitting.html
[^12]: JOSS, review checklist: https://joss.readthedocs.io/en/latest/review_checklist.html
[^13]: OpenSSF, Scorecard: https://openssf.org/scorecard/
[^14]: OpenSSF, Best Practices Badge: https://openssf.org/projects/best-practices-badge/
[^15]: European Innovation Council, EIC Work Programme 2026, DeepRAP section: https://eic.ec.europa.eu/document/download/52598755-1351-4b54-b46b-e2682d0a3aec_en?filename=EIC-Work-Programme-2026.pdf
[^16]: European Innovation Council, EIC Pathfinder Challenges 2026: https://eic.ec.europa.eu/eic-funding-opportunities/eic-pathfinder/eic-pathfinder-challenges-2026_en
[^17]: European Innovation Council, EIC Pathfinder overview: https://eic.ec.europa.eu/eic-funding-opportunities/eic-pathfinder_en
[^18]: EU Funding & Tenders Portal, DeepRAP topic `HORIZON-EIC-2026-PATHFINDERCHALLENGES-01-03`: https://ec.europa.eu/info/funding-tenders/opportunities/portal/screen/opportunities/topic-details/HORIZON-EIC-2026-PATHFINDERCHALLENGES-01-03
[^19]: NLnet, “Apply for funding”: https://nlnet.nl/funding.html
[^20]: NLnet, “Apply for funding before November 3rd 2026”: https://nlnet.nl/news/2026/20260903-call.html
[^21]: NLnet, Restack: https://nlnet.nl/restack/
