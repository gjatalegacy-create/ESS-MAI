# ESS-MAI organic discovery readiness

Assessment date: 2026-09-14

Public scope: `gjatalegacy-create/ESS-MAI`

Overall readiness: **80/100**

This score measures whether a technically relevant outsider can find, classify,
inspect, reproduce, criticize, and cite the public work. It does **not** measure
scientific validity, novelty, safety, product maturity, grant competitiveness,
popularity, or the completeness of the private ESS-MAI workspace. Stars, views,
followers, and unverifiable impressions are deliberately excluded.

## Scored dimensions

| Dimension | Score | Evidence for the score | What prevents a higher score |
| --- | ---: | --- | --- |
| GitHub classification | 90 | Factual one-sentence description; 20 audited topics; Rust detected as primary language; Apache-2.0; identity consistent in README and metadata | Topic utility needs real search/referral data and periodic review; `formal-methods` requires especially careful boundaries |
| Technical clarity | 78 | README now distinguishes architecture, experiments, implementation statuses, and forbidden inferences; capsule claim boundaries are explicit | The older root audit/history documents vary in terminology and evidentiary strength; a new reader still faces substantial volume |
| Research classification | 85 | `docs/RESEARCH_DOMAINS.md`, `HYPOTHESES.md`, and capsule prior-art files separate established theory, project contribution, proposition, evidence, and gap | Cross-disciplinary claims need independent specialists and a broader systematic literature review |
| Review accessibility | 90 | Reviewer personas, contribution routes, structured forms, issue #3 and scoped issues #4–#11 provide concrete starting tasks | No external reviewer is assigned and no independent review result is registered |
| Reproducibility | 84 | Locked Cargo commands, source/extraction checks, SHA-256 manifests, external target directories, repeatability runs, and successful public CI run 34817554348 | Windows-centered CI; no independent reproduction; POC 005/006 do not claim bit-for-bit executable reproducibility |
| Citation readiness | 92 | Valid-looking `CITATION.cff`, CodeMeta, exact v1.1.1 DOI, all-versions DOI, author/organization/license, and citation boundary | No ORCID is asserted; no journal paper or independent citation/adoption is established |
| External archival | 94 | Zenodo version DOI `10.5281/zenodo.22750188`, concept DOI `10.5281/zenodo.22074027`, Software Heritage snapshot SWHID, and OpenAIRE discovery route | Future Phase 16 commits are newer than the sealed v1.1.1 POC record; archive/version identities must remain explicit |
| OpenSSF readiness | 35 | License, security/contribution policies, issue/PR templates, least-privilege POC workflow, public history | No published Scorecard/Best Practices result; main is unprotected; secret scanning and Dependabot security updates are disabled; no pinned action SHAs, coverage, fuzzing, SAST, SBOM/provenance, signed-release evidence, or multi-maintainer review requirement |
| Discussion readiness | 82 | Discussions enabled; initial independent-review discussion #12 published; structured discussion form committed | Specialized category creation/pinning requires separate GitHub UI configuration; no outside conversation exists yet |
| Contributor readiness | 88 | `CONTRIBUTING.md` has evidence procedure and “Ways to Contribute Without Accepting the Theory”; forms, labels, support, conduct, and security paths exist | Newcomer installation remains substantial; attribution policy is stated but has not yet been exercised with an outside contribution |
| Theory falsifiability | 90 | Normalized registry gives statement, rationale, source, test, falsifier, evidence, and status; negative results remain first-class | Several advancement methods still lack successor implementations or independently designed attacks |
| Theory-to-code traceability | 84 | End-to-end registry traces theory → formalization → object → source → runtime → test → artifact → result; POCs include source maps and hashes | Some root architecture edges are source-mapped or documented only, and the private v1.8.9 workspace is outside public review |
| Security-review surface | 65 | `SECURITY.md`, integrity manifests, explicit authority/receipt threats, and adversarial issue #9 create a real entry path | No published threat model, external audit, CodeQL/SAST, fuzzing, dependency review, coverage, or security certification |
| Academic-review surface | 58 | Prior-art notes, hypothesis/materialization registries, citation metadata, DOI/archive records, negative-result disclosure | No paper, institutional validation, external use, peer review, benchmark suite, or six-month public-development history for JOSS |
| Engineering-review surface | 86 | Public Rust workspace, explicit subsystem roles, reproducible POCs, reviewer commands, open implementation gaps, and labels by specialty | Full-system build/runtime is not the same reviewed scope as the POCs; cross-platform and recovery characterization remain incomplete |

Arithmetic mean: `(1201 / 15) = 80.07`, reported conservatively as **80/100**.

## Strongest current signals

1. Four executable POCs expose both supporting observations and counterexamples.
2. Exact release/version identities exist across GitHub, Zenodo, and Software Heritage.
3. Claims now route to a normalized hypothesis and theory-to-materialization registry.
4. Outsiders can enter through reproduction, formalization, architecture,
   security, Rust, state-machine, negative-knowledge, documentation, or
   benchmark tasks without accepting the theory.
5. The repository explicitly refuses to reinterpret CI, DOI, archival, or a
   future automated score as peer review or scientific validation.

## Highest-value missing signals

1. One named, independent, clean-room reproduction of each POC with public logs.
2. One bounded formal model with a published counterexample or proof result and
   a documented relation to the Rust implementation.
3. An external security/threat-model review and a repository-hardening plan.
4. Cross-platform CI plus transparent coverage/fuzzing/static-analysis results.
5. A successor POC for each currently explicit gap: cold-start genesis, global
   LAW-0 transcript, durable semantic FFI identity, and full receipt/restart K−
   re-entry with a distinct K+ counterpart.
6. A predeclared benchmark protocol with named baselines and limitations.
7. Genuine outside use, citations, pull requests, and criticism accumulated over
   time; none should be manufactured.

## Reassessment rule

Raise a score only with a linkable change in evidence. Lower it when a signal
expires, a link breaks, a reproduction fails, a claim is narrowed, or a new gap
is found. A negative result may lower one maturity score while increasing
falsifiability and research credibility; preserve both effects rather than
averaging the contradiction away.
