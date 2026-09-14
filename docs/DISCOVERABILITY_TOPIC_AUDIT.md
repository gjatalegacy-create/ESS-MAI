# ESS-MAI discoverability topic audit

Status date: 2026-09-14  
Scope: public repository `gjatalegacy-create/ESS-MAI` at `ab87e3e` before the Phase 16 documentation commit  
Rule: a topic is selected only when a public source, experiment, or repository practice supports it. A topic is classification metadata, not validation.

GitHub permits at most 20 topics and recommends terms that describe a repository's purpose, subject area, community, or language.[^1] The set below therefore favors established engineering and research vocabulary over project-internal names. The project-specific `ess-mai` and the central experimental term `negative-knowledge` are retained because they provide exact-identity and exact-concept routes.

## Selected topics

| Topic | Why it applies | Public source evidence | Target community | Discovery value | Overclaim risk and control |
| --- | --- | --- | --- | --- | --- |
| `rust` | The workspace and all four executable POCs use Rust and Cargo. | `Cargo.toml`; `light/`, `quantum/`, `shadow/`, `shadow-contracts/`; POC `Cargo.toml` files | Rust engineers | Very high | Low; GitHub also identifies Rust as the primary language. |
| `systems-programming` | The source defines process boundaries, wire contracts, persistence paths, FFI, and fail-closed runtime behavior. | `shadow-contracts/src/lib.rs`; `shadow/src/sovereign_ffi.rs`; `quantum/src/bridge_shadow.rs` | systems programmers | High | Medium; this describes implementation concerns, not production maturity. |
| `research-software` | The repository exposes hypotheses, claim boundaries, reproducibility procedures, citation metadata, and versioned experiments. | `CITATION.cff`; `codemeta.json`; `publications/executable-prior-art/` | research-software engineers | High | Low if kept explicitly experimental. |
| `software-architecture` | Light, Quantum, and Shadow are separate bounded roles connected by public contracts. | `Cargo.toml`; `KONTRATAT_URAVE.md`; `shadow-contracts/src/lib.rs` | architecture reviewers | High | Medium; the topic does not imply architectural closure. |
| `systems-engineering` | The project joins component boundaries, runtime handoffs, failure modes, persistence, and reproducible build procedures. | `README.md`; `LIGHT_RUNTIME_CLOSURE.md`; `QUANTUM_RUNTIME_CLOSURE.md`; `SHADOW_RUNTIME_CLOSURE.md` | systems engineers | High | Medium; historical closure documents are claims/references unless independently exercised. |
| `systems-research` | The public POCs isolate reachability, continuity, authority consumption, and verified-history re-entry as falsifiable systems questions. | POC 003–006 `CLAIM_BOUNDARY.md`, `RESULTS.md`, and `REPRODUCIBILITY.md` | experimental-systems researchers | High | Low when results stay bounded to the disclosed POCs. |
| `deterministic-systems` | Tests and contracts use explicit inputs, deterministic encodings, hashes, transitions, and repeatability runs. | `.github/workflows/poc-validation.yml`; `shadow-contracts/src/lib.rs`; POC 005–006 `REPRODUCIBILITY.md` | deterministic/reliable-systems reviewers | Medium | Medium; deterministic tested paths do not prove total-system determinism. |
| `runtime-verification` | Runtime inputs, receipts, evidence digests, transition conditions, and refusal paths are checked during execution. | `shadow-contracts/src/lib.rs`; `shadow/src/verification.rs`; POC 004 and 006 | runtime-verification researchers | High | Medium; this is an implemented connection and review domain, not a claim of formal verification. |
| `state-machines` | Named phases and explicit allowed/refused transitions appear in the public source and experiments. | `quantum/src/bridge_light/mod.rs`; `quantum/src/gcl_state_machine.rs`; POC 003–004 | state-machine reviewers | High | Low; no model-checker use is implied. |
| `reproducibility` | Locked Cargo commands, source hashes, manifests, external CI, and repeatability runs are published. | `.github/workflows/poc-validation.yml`; each POC's `REPRODUCIBILITY.md`; `artifact_hashes.sha256` | reproducibility reviewers | Very high | Low; maintainer reruns are explicitly not independent replication. |
| `open-science` | Apache-2.0 POCs, negative results, version DOIs, and content-addressed archive records are public. | `LICENSE`; `publications/executable-prior-art/`; `CITATION.cff`; Zenodo/SWH links in `README.md` | open-science practitioners | High | Medium; archival publication is not peer review. |
| `knowledge-representation` | The public contracts and POC 006 model typed negative knowledge, evidence, receipts, and distinct constructive/negative streams. | `shadow-contracts/src/lib.rs`; POC 006 `POSITIVE_NEGATIVE_PARALLELISM.md` | knowledge-representation researchers | High | Medium; this is a candidate research connection, not disciplinary validation. |
| `reasoning-systems` | Quantum contains reasoning-stage modules and exposes evidence to a separate verdict authority; public POCs test parts of that topology. | `quantum/src/`; `shadow-contracts/src/lib.rs`; `README.md` | AI reasoning and cognitive-architecture reviewers | High | Medium; the topic does not assert state-of-the-art reasoning performance. |
| `ai-governance` | ESS-MAI tests explicit authority boundaries, delegated verdict jurisdiction, evidence requirements, and fail-closed decisions. | `GCL_NURALOGIC_KANUNI.md`; `KONTRATAT_URAVE.md`; POC 003–006 claim boundaries | AI-governance researchers | High | High; documented as “relevant to” governance, not as a compliant governance framework. |
| `formal-methods` | The repository publishes invariants, state/transition claims, counterexamples, and falsification conditions that are suitable objects for formal-methods review. | `docs/HYPOTHESES.md`; POC 004 `THEORY.md`; POC 005–006 `CLAIM.md` | formal-methods researchers | High | High; ESS-MAI does **not** currently claim mechanized proof, model checking, or formal verification. |
| `hypothesis-testing` | POCs deliberately pair positive controls with counterexamples and explicit falsification criteria. | POC 003–006 `EXPERIMENT_PROTOCOL.md`/`POC_PROTOCOL.md`, `RESULTS.md`, `CLAIM_BOUNDARY.md` | empirical-method and theory critics | High | Low. |
| `traceability` | Public wire objects, source maps, extraction identities, hashes, receipts, and manifests connect claims to artifacts. | `shadow-contracts/src/lib.rs`; POC `SOURCE_MAP.md`; `manifest.json` | audit and traceability reviewers | High | Low within disclosed boundaries. |
| `executable-prior-art` | The named collection publishes source, prior-art boundaries, hashes, commands, successes, failures, and advancement methods. | `publications/executable-prior-art/README.md`; Zenodo version DOI | open-source and prior-art researchers | High | Medium; the phrase is descriptive and does not assert a patent-office novelty decision. |
| `negative-knowledge` | Negative knowledge is a central, expressly bounded concept and the direct subject of POC 006. | POC 006 `THEORY.md`, `PRIOR_ART.md`, `RESULTS.md`, `POSITIVE_NEGATIVE_PARALLELISM.md` | epistemic reasoning and failure-memory researchers | Very high | High; the repository expressly does not claim invention of the broad idea. |
| `ess-mai` | Exact project identity is consistent across code, releases, DOI metadata, and archives. | `README.md`; `CITATION.cff`; `codemeta.json` | people seeking the named project | High for exact search | Low; one identity topic is proportionate. |

## Rejected or deferred topics

| Candidate | Decision | Reason |
| --- | --- | --- |
| `formal-verification` | Rejected | No mechanized proof or verified implementation is disclosed. Runtime tests are not formal verification. |
| `model-checking` | Rejected | No TLA+, Alloy, SPIN, PRISM, Lean, Coq, Isabelle, or equivalent model-checking/proof artifact is present. |
| `theorem-proving` | Rejected | No machine-checked theorem exists in the public repository. |
| `trustworthy-ai` | Deferred | The architecture is relevant to trustworthiness questions, but independent evaluation and benchmark evidence are absent. |
| `explainable-ai` | Deferred | Trace and evidence fields are inspectable, but no user-facing explanation method or evaluation is established. |
| `ai-safety` | Deferred | Safety researchers are invited to review fail-closed and authority claims, but the repository has no safety case or independent safety evaluation. |
| `security` / `cybersecurity` | Deferred | Security boundaries and a reporting policy exist, but no external security audit or production security claim exists. Use review labels rather than a broad topic for now. |
| `formal-ai-governance` | Rejected | This compound label could imply a formalized or validated governance framework that is not yet present. |
| `evidence-based-ai` | Rejected | Ambiguous term with possible clinical-policy connotations; the repository's more precise phrase is evidence-governed reasoning. |
| `epistemic-reasoning` | Deferred | Relevant, but lower-signal than `knowledge-representation` and `reasoning-systems`, and the 20-topic budget is full. |
| `decision-systems` | Deferred | Shadow verdict paths are relevant, but the topic is broader than the demonstrated POC scope. |
| `scientific-software` | Deferred | `research-software` is the clearer description; ESS-MAI is not yet a mature scientific-computing tool. |
| `experimental-software` / `experimental-research` | Deferred | The README already says experimental; the terms are less discriminating than the selected systems and hypothesis topics. |
| `artificial-intelligence` | Deferred | Too broad for useful reviewer discovery; the selected reasoning, governance, and knowledge topics are more precise. |
| `sovereign-ai` | Rejected as classification | It is project language rather than an established implementation or validation category and risks promotional interpretation. |
| `deterministic-ai` / `traceable-ai` | Deferred | Project-relevant but less established than `deterministic-systems` and `traceability`. |
| `hierarchical-authority` | Deferred | Central internal architecture phrase; discoverability is served more accurately through `software-architecture`, `ai-governance`, and the README. |
| `bledar-gjata` / `gjata-legacy` | Removed from topic budget | Authorship and organization remain prominent in citation metadata and the README; topic slots should chiefly serve technical discovery. |
| `research` | Removed | Too broad; `research-software` and `systems-research` are more useful. |

## Repository description candidates

1. **Recommended — strongest combined accuracy and discoverability:** “Experimental Rust research software for explicit authority boundaries, negative knowledge, traceable state transitions, and reproducible falsification POCs.”
2. **Most engineering-focused:** “Rust systems-research repository with bounded runtime authorities, evidence-carrying contracts, explicit state transitions, and reproducible Cargo POCs.”
3. **Most research-focused:** “Experimental Rust architecture connecting governed reasoning and negative knowledge to falsifiable, hash-traceable Cargo experiments.”

The first description is compact, states the stack, research scope, principal concepts, and the review mechanism, and avoids maturity or validation claims.

## Legitimate search-surface matrix

| Query family | Genuine connection? | Substantive landing content | Boundary |
| --- | --- | --- | --- |
| Rust runtime verification | Yes | POC 004/006; `shadow-contracts/src/lib.rs`; `docs/THEORY_TO_MATERIALIZATION.md` | Runtime checks and experiments, not formal verification. |
| Rust state-machine research | Yes | POC 003/004; `quantum/src/gcl_state_machine.rs`; `docs/HYPOTHESES.md` | Only disclosed transitions are evidenced. |
| Reproducible AI reasoning | Partial | locked Cargo POCs, CI, hashes, repeatability receipts | POC reproducibility does not establish full-system reproducibility. |
| Negative knowledge AI | Yes, bounded | POC 006 theory/prior art/results and K+/K− topology | Broad negative-knowledge theory is prior art; the tested composition is narrower. |
| AI reasoning traceability | Partial | contracts, source maps, evidence fields, manifests | No independent trace-quality evaluation. |
| AI governance architecture | Candidate connection | authority hierarchy, jurisdiction boundaries, fail-closed tests | Not a legal/compliance framework and not externally validated. |
| Formal methods for AI governance | Evaluation route | explicit invariants, claim registry, falsification conditions | No mechanized formalization yet. |
| Evidence-governed reasoning architecture | Yes as project formulation | README, contracts, POC 003–006 | ESS-MAI term, not a recognized academic classification. |
| Knowledge verification system | Partial | Shadow verification and receipt paths; POC 006 | Does not prove truth or universal knowledge correctness. |
| Authority-based computing | Candidate connection | GCL hierarchy and POC 005 authority-consumption experiment | Project-specific composition; no novelty judgment. |
| Software architecture criticism | Yes | `docs/WHO_SHOULD_REVIEW_THIS.md`, issue forms, public module topology | Criticism is invited; closure is not presumed. |

No keyword-only pages are proposed. Each discovery phrase routes to source, an experiment, a falsification condition, or an explicitly identified gap.

## Configuration

The approved metadata command is:

```powershell
gh repo edit gjatalegacy-create/ESS-MAI `
  --description "Experimental Rust research software for explicit authority boundaries, negative knowledge, traceable state transitions, and reproducible falsification POCs." `
  --add-topic rust,systems-programming,research-software,software-architecture,systems-engineering,systems-research,deterministic-systems,runtime-verification,state-machines,reproducibility,open-science,knowledge-representation,reasoning-systems,ai-governance,formal-methods,hypothesis-testing,traceability,executable-prior-art,negative-knowledge,ess-mai
```

Because `--add-topic` does not remove older topics, an API replacement is preferable when applying the audited exact set.

## Sources

[^1]: GitHub Docs, “Classifying your repository with topics,” topic purpose, public visibility, syntax, and 20-topic limit: https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/classifying-your-repository-with-topics
