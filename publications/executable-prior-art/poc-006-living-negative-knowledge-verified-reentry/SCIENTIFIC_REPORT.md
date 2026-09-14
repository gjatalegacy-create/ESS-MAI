# Scientific report — POC-006

## Research question

Does ESS-MAI v189 contain an executable path in which verified negative
history changes later reasoning, while missing, corrupt, or downgraded required
history prevents PRO activation?

## Method

The supplied theory texts and project documents were treated initially as
references. A claim was promoted to evidence only when it could be tied to
source identity, a caller/consumer path, and a native or detached Cargo test.
The 2026-06-03 media page was promoted only as evidence of public disclosure
and attribution, never as evidence that the runtime mechanism was then
materialized.

Four focal files were copied byte-for-byte from v189 into a minimal independent
workspace. Narrow adapters supplied only the module types required to compile
those files. The experiment varied whether prior history was required and
whether the history was valid, corrupt, absent, or downgraded. Its observable
outputs were runtime readiness, Vault entry count, PRO activation permission,
hard-block status, and score.

The pre-registered falsification conditions were:

- required invalid history still permits PRO;
- valid history fails to import;
- a known imported failure has no observable effect;
- POC material is falsely reported as a full cross-subsystem receipt.

## Evidence chain

```text
PUBLIC TRACE
  -> 2026-06-03 publisher record: broad ESS-MAI “Dija Negative” concept
LAW
  -> required negative history must be verified before positive activation
TYPE
  -> NegativeKnowledgeAsset / sealed NK blob / GclReadiness
CALLER
  -> Shadow admission and export; Quantum guarded activation
TRANSITION
  -> absent|corrupt|downgraded -> DEGRADED|NOT_READY -> refusal
EVIDENCE
  -> native 12 + 19 + 1 tests; strict detached 30/30; 3/3 runtime receipts; hashes
CONSUMER
  -> NkGate::post_filter and the PRO activation boundary
SINK
  -> hard block for known failure, or fail-closed refusal
TEST
  -> first cycle / valid / corrupt / missing / downgrade
```

## Findings

The verified-re-entry claim passed in its bounded scope. A valid one-entry
history imported as `READY`; its known failure was hard-blocked at score zero.
Each required-history falsification prevented activation before candidate
reasoning.

The full receipt claim failed. Source inspection found the cross-subsystem wire
principally carries `negative_persisted: bool`, which cannot prove transaction,
asset, export, and consumer identity as one chain. This is the reproduced
advancement gap, not a compilation defect.

## Prior-art consequence

Negative Knowledge itself is direct prior art.[^1][^2] Runtime governance and
constitutive execution gates are also established neighboring work.[^3][^4]
POC-006 consequently claims neither invention of failure memory nor exclusive
ownership of fail-closed governance. Its defensible contribution is the tested
ESS-MAI source composition and its explicit boundary.

The Business Magazine record publicly attributes the broad Negative Knowledge
concept to Bledar Gjata and ESS-MAI on 2026-06-03.[^5] It predates Wang's
2026-06-19 arXiv v1 record by 16 days, but it does not contain the full
claim-by-claim Living Negative mechanism. The media record and executable
evidence are therefore preserved as different evidence classes.

## Independent verdict

```text
TYPED NEGATIVE ASSET: MATERIALIZED IN TESTED SCOPE
SHADOW VAULT TRANSACTION / DEDUPE / WAL: MATERIALIZED IN TESTED SCOPE
SEALED QUANTUM RE-ENTRY: MATERIALIZED IN TESTED SCOPE
REQUIRED-HISTORY FAIL-CLOSED GATE: MATERIALIZED IN POC SCOPE
FUTURE INFLUENCE ON KNOWN FAILURE: MATERIALIZED IN POC SCOPE
FULL CROSS-SUBSYSTEM COMMIT RECEIPT: NOT MATERIALIZED
CRYPTOGRAPHIC SECURITY OF FNV SEAL: NOT CLAIMED
NARROW_VERIFIED_REENTRY: POC_READY
GLOBAL_LIVING_NEGATIVE_LIFECYCLE: NOT_MATERIALISED
OVERALL: PARTIAL_POC
```

## Advancement

The strongest closure is a versioned `NegativeCommitReceiptWire` emitted only
after the existing Shadow transaction commits and independently verified by
Quantum before the existing activation point. It binds transaction, asset,
GCL law/version, exported history, and runtime cycle with SHA-256 identities.
This makes the current Boolean assertion auditable without transferring write
authority or collapsing the architecture.

In parallel, the existing closed `Constructive` and `RigorousNegative`
branches should carry distinct positive and negative commit receipts under one
GCL cycle receipt. Shared transport must not collapse their types, admission
laws, consumers, or effects.

## Sources

[^1]: Hanchun Wang, “Negative Knowledge as Failure-aware Shared Memory for AutoResearch,” arXiv:2606.21024. [Primary preprint](https://arxiv.org/abs/2606.21024).
[^2]: Hanchun Wang, Negative Knowledge reference implementation. [Source repository](https://github.com/hch-wang/Negative_Knowledge).
[^3]: “Constitutive Governance for Agentic Systems,” arXiv:2605.24538. [Primary preprint](https://arxiv.org/abs/2605.24538).
[^4]: “LATTICE: Runtime Governance for Agentic AI,” *Frontiers in Artificial Intelligence* (2026). [Publisher version](https://www.frontiersin.org/journals/artificial-intelligence/articles/10.3389/frai.2026.1800407/full).
[^5]: Eni Muça, “A jemi drejt një AI sovrane? Bledar Gjata dhe vizioni ambicioz pas ESS-MAI,” *Business Magazine Albania*, 2026-06-03. [Publisher record](https://businessmag.al/a-jemi-drejt-nje-ai-sovrane-bledar-gjata-dhe-vizioni-ambicioz-pas-ess-mai/).
