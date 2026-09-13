# Prior-art analysis

Search date: 2026-09-10. This is a focused technical comparison, not an
exhaustive patent search, freedom-to-operate opinion, or legal novelty opinion.

## Established foundations

Capability systems already define capabilities as unforgeable tokens of
authority and support constrained delegation. CHERI implements capability
integrity with guarded manipulation and tagged memory, and sealed capabilities
are immutable and non-dereferenceable.[^1] CHERIoT also describes software
capabilities as sealed, shareable tokens validated by a more privileged
component.[^2]

Rust ownership is established language machinery, while Rust's official FFI
guidance makes clear that raw pointer contracts and foreign declarations remain
unsafe obligations outside the compiler's ordinary checks.[^3]

Fail-safe defaults and complete mediation are classical protection principles:
access should be based on explicit permission and every access should be
checked for authority.[^4]

Recent agent-governance work also predates this release. A Technical Disclosure
Commons publication dated 2026-08-13 describes a fail-closed gateway redeeming
single-use capability tokens.[^5] More importantly, CapLease shows that
identifier-local single-use tokens do not prevent **semantic replay** through
fresh issuance; durable authorization state and an idempotent sink are needed
for the stronger property.[^6]

## Comparison

| Dimension | Prior art | POC-005 evidence |
|---|---|---|
| capability as authority | established | not claimed |
| sealed/unforgeable capability | established in CHERI/CHERIoT | POC uses a software registry and nonce; no hardware tag claim |
| single-use gateway token | established | exact v189 implementation tested |
| atomic one-shot transition | established primitive | `AtomicBool::compare_exchange` used |
| deny-by-default / mediation | established | direct FFI write remains held without verdict |
| same-identifier replay | known problem with known solutions | bounded test passes |
| fresh semantic reissuance | explicitly identified by CapLease | not solved by POC-005 alone |
| GCL constitutional composition | project-specific formulation | partial: one-shot gate + verdict hold; commit connection absent |

## Defensible contribution boundary

No direct prior art was found in this focused search for the **exact**
ESS-MAI-specific composition of:

1. a freely copyable two-word C handle;
2. non-exported Rust authority consumed once;
3. capability lineage derivation;
4. refusal to translate consumed authority into knowledge without a
   `LawConfirmedVerdict`;
5. intended handoff to the separate Shadow transaction authority.

That statement is not a claim that the composition is legally novel. The
individual mechanisms are known, and the current materialization is partial.

## Sources

[^1]: Robert N. M. Watson et al., “CHERI: A Hybrid Capability-System Architecture for Scalable Software Compartmentalization,” IEEE Symposium on Security and Privacy, 2015. [Primary paper](https://www.cl.cam.ac.uk/research/security/ctsrd/pdfs/201505-ssp2015-cheri-compartment.pdf).
[^2]: CHERIoT Platform, “Understanding CHERI capabilities” and “Sealing pointers for tamper proofing.” [Official guide](https://cheriot.org/book/concepts.html).
[^3]: Rust Project, “Foreign Function Interface” and “Ownership and Lifetimes.” [Official FFI guide](https://doc.rust-lang.org/nomicon/ffi.html), [official ownership guide](https://doc.rust-lang.org/nomicon/ownership.html).
[^4]: Jerome H. Saltzer and Michael D. Schroeder, “The Protection of Information in Computer Systems,” *Proceedings of the IEEE* 63(9), 1975, DOI: 10.1109/PROC.1975.9939. [Author bibliography](https://www.mit.edu/~Saltzer/publications/pubs.html).
[^5]: Jason Edward Plumb, “Evidence-Partitioned Autonomy Authorization for LLM Agents: Single-Use Capability Tokens…,” Technical Disclosure Commons, 2026-08-13. [Defensive publication](https://www.tdcommons.org/dpubs_series/11356/).
[^6]: Jinghan Xu et al., “Beyond Single-Use Tokens: Durable Authorization State for Replay-Resistant LLM Agent Actions,” arXiv:2608.01710, submitted 2026-08-03. [Primary preprint](https://arxiv.org/abs/2608.01710).

