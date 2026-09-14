# Badge and status-label policy

Status date: 2026-09-14

A badge is compact metadata, not a substitute for evidence. ESS-MAI badges must name exactly what the linked service establishes and must lead to a public evidence page. No badge may imply scientific validation, independent replication, eligibility, endorsement, peer review, security certification or grant readiness unless a named body has actually issued that exact status.

## Allowed now

| Signal | Required wording and link | Meaning boundary |
| --- | --- | --- |
| GitHub Actions | “POC build and tests”; link to the `poc-validation.yml` workflow | Status of the workflow run and selected branch only. A green run is not scientific validation or independent replication. GitHub describes workflow badges as passing/failing status displays.[^1] |
| License | “Apache-2.0”; link to the repository `LICENSE` file | Declared repository license only. It is not a compliance opinion about every dependency, dataset or third-party asset. |
| Zenodo concept DOI | Label “DOI — all versions” or plainly “DOI”; link to `10.5281/zenodo.22074027` | Persistent identifier for the version family. Do not call it a journal-publication DOI or peer-review badge. |
| Zenodo version DOI | Prefer a text link or label “v1.1.1 DOI”; link to `10.5281/zenodo.22750188` | Exact deposited version. Zenodo preservation and DOI registration do not establish correctness, novelty or peer review.[^2][^3] |
| Software Heritage | Label “Software Heritage snapshot” or “source archived”; link to `swh:1:snp:d2d862ac5122383925807c53d2ec4cda7d05b46a` | Archived snapshot identity. `snp` denotes a snapshot object; the SWHID is an intrinsic identifier, not a verdict.[^4] |

The current README's workflow, license, DOI and Software Heritage classes are acceptable in principle if each target continues to resolve and the nearby prose preserves these boundaries. Prefer the exact v1.1.1 DOI in release-specific contexts and the concept DOI in project-wide, all-version contexts.

## Discovery links are not merit badges

OpenAIRE may be linked as “Indexed in OpenAIRE” or “Discover in OpenAIRE,” preferably in a references/discovery section instead of the top badge strip. OpenAIRE describes Explore and its graph as discovery infrastructure containing research-software items.[^5] Do not label the link “OpenAIRE verified,” “validated,” “approved” or “peer reviewed.”

ORCID, OSF and HAL should not receive project badges unless a corresponding public identifier or deposit actually exists and resolves:

- An ORCID icon/link may identify a named researcher's author-confirmed public ORCID iD; it must not be inferred from a name or email. ORCID permits works to be added by DOI, and manually added information identifies its source.[^6]
- An OSF link must say whether it points to a registration or a preprint. OSF explicitly distinguishes registrations from preprints and says preprints are shared without formal peer review.[^7]
- A HAL link may say “Archived in HAL” only after a HAL deposit exists. HAL is an open archive for published and unpublished research, including software.[^8]

## Not allowed until externally true

Do not display any of the following today:

- “JOSS,” “JOSS review,” “submitted to JOSS,” “JOSS accepted” or a JOSS DOI badge;
- an OpenSSF Scorecard badge before Scorecard results are published for this repository and the badge resolves to its viewer page;
- an OpenSSF Best Practices passing, silver, gold or baseline badge before the project has completed the applicable public questionnaire and the service awards that level;
- EIC, Horizon Europe, EU, NLnet, Restack, CodeSupply or other funder logos/status labels unless the relevant body authorizes the usage and an actual award/status exists;
- “peer reviewed,” “independently verified,” “certified,” “validated,” “eligible,” “endorsed,” “grant ready” or equivalent wording based only on a DOI, archive, index, CI result, automated score or maintainer assertion;
- custom shields that visually imitate an external institution's award or badge.

A future JOSS submission may be described as “submitted” only after a submission exists; “under review” only while the JOSS process is active; and “peer reviewed” or “accepted” only after JOSS acceptance. JOSS states that it conducts formal peer review and issues a paper DOI upon acceptance.[^9]

## OpenSSF readiness, without a badge claim

OpenSSF Scorecard and OpenSSF Best Practices are separate signals. Scorecard automatically generates repository-security signals. Best Practices is voluntary self-certification through a public questionnaire; it is not an independent audit.[^10][^11]

### Evidence already visible in this checkout

- Public GitHub repository and revision history.
- Apache-2.0 `LICENSE`.
- `README.md`, `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `SECURITY.md` and `SUPPORT.md`.
- Issue and pull-request templates.
- A least-privilege `contents: read` validation workflow for the disclosed POC collection.
- Citation and software metadata (`CITATION.cff` and `codemeta.json`).

These are readiness inputs, not proof that any OpenSSF criterion is met.

### Unknown or incomplete until evidenced

- No OpenSSF Scorecard workflow or published Scorecard result is present in this checkout.
- Branch protection, mandatory code review, token permissions across all workflows, dependency update tooling, code scanning, secret scanning and maintainer 2FA cannot be established from repository files alone.
- The workflow uses version tags such as `actions/checkout@v4`, not immutable full-length commit pinning, which Scorecard evaluates under pinned dependencies.
- Fuzzing, sanitizer/dynamic analysis, coverage measurement and a documented coverage threshold are not visible in the inspected top-level workflow.
- Cryptographic release signing, provenance/SBOM publication, reproducible-build evidence, dependency vulnerability response and governance/bus-factor controls need explicit public evidence.
- The visible commit identities appear attributable to one maintainer; do not claim independent review or a multi-maintainer bus factor from name/email variants.

Before publishing a Scorecard badge, install or run the official Scorecard action, review every result, decide whether public result publication is appropriate, remediate high-value gaps, and link the badge to the public viewer. The official Scorecard repository says a badge is available when results are published and that it updates with repository changes.[^12]

Before applying for a Best Practices badge, answer every criterion conservatively with a public evidence URL, use “unknown” where evidence is absent, and let the badge service determine the displayed level. The program supports both baseline levels and the passing/silver/gold series.[^11]

## Display rules

1. Keep no more than four primary badges at the top of the README: workflow, license, DOI and source archive.
2. Link every badge to first-party evidence, not to a generic home page or image renderer.
3. Use explicit alt text. “Software Heritage snapshot” is better than “archived”; “v1.1.1 DOI” is better than an unexplained “published.”
4. Put OpenAIRE, ORCID and other discovery identities in a “Public references” section unless space or accessibility requires another layout.
5. Put maintainer-run results in prose/tables labeled “maintainer-verified” or “maintainer-reported.” Reserve “independently reproduced” for a named external reproduction with a public report and scope.
6. Recheck links and claims for every release. Remove stale, revoked or no-longer-resolving signals.
7. A badge never overrides a nearby limitation statement.

## Sources

[^1]: GitHub Docs, “Adding a workflow status badge”: https://docs.github.com/en/actions/how-tos/monitor-workflows/add-a-status-badge
[^2]: Zenodo, ESS-MAI v1.1.1 record: https://doi.org/10.5281/zenodo.22750188
[^3]: Zenodo, “About records”: https://help.zenodo.org/docs/deposit/about-records/
[^4]: Software Heritage, SWHID documentation: https://docs.softwareheritage.org/devel/swh-model/persistent-identifiers.html
[^5]: OpenAIRE Explore: https://explore.openaire.eu/
[^6]: ORCID, “Add works using an identifier”: https://support.orcid.org/hc/en-us/articles/360022298153-Add-works-using-an-identifier
[^7]: OSF Support, “OSF Projects Transition FAQs”: https://help.osf.io/article/725-faqs
[^8]: HAL, “About HAL”: https://about.hal.science/en/
[^9]: Journal of Open Source Software documentation: https://joss.readthedocs.io/en/latest/
[^10]: OpenSSF, Scorecard: https://openssf.org/scorecard/
[^11]: OpenSSF, Best Practices Badge: https://openssf.org/projects/best-practices-badge/
[^12]: OpenSSF, Scorecard repository and badge instructions: https://github.com/ossf/scorecard#scorecard-badges
