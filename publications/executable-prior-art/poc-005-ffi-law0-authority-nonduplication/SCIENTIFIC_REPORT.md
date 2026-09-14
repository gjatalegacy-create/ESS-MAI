# Scientific report — POC-005

## Research question

Does the current ESS-MAI v189 source materially preserve one-shot authority
when a C-compatible handle is copied, and does it prevent that authority from
bypassing the superior constitutional verdict path?

## Method

The audit began with the two supplied theory documents as references. Claims
were promoted to evidence only after locating the relevant v189 call path,
executing its native tests, copying the focal files byte-for-byte into an
independent Cargo closure, and attacking that closure through the public C ABI.

The independent variables were handle generation, nonce correctness, replay
count, payload presence, and concurrency. The observed variable was the ABI
return code. The pre-registered falsification conditions were:

- more than one post-authorization result for copies of one generation;
- acceptance of a forged nonce;
- a direct persistence success without a committed verdict;
- resurrection after the first consumption.

## Evidence chain

```text
LAW
  -> CapHandle copy is not authority copy
TYPE
  -> CapHandle + private CapSlot + opaque LgcToken
CALLER
  -> sovereign_validate_and_write
TRANSITION
  -> AtomicBool CAS(true,false)
EVIDENCE
  -> native 8/8 + strict detached 15/15 + 3/3 runtime receipts
CONSUMER
  -> sovereign_commit
SINK
  -> CompatibilityHold, no direct Vault write
TEST
  -> forged / serial replay / concurrent replay / empty payload
```

## Findings

The bounded non-duplication claim passed. One issued generation produced one
post-authorization result in both serial and concurrent attack cases. A forged
nonce did not consume the genuine handle. Consumption did not resurrect.

The end-to-end write claim failed by design: the only non-empty authorized call
returned `-8`. This is evidence of architectural mediation, not evidence that
the desired FFI-to-Vault path is complete.

## Prior-art consequence

Capability authority, sealed capabilities, single-use gateways, Rust
ownership, atomic transitions, fail-safe defaults, and complete mediation are
established.[^1][^2][^3] Recent work further demonstrates that same-identifier
consumption is insufficient against fresh semantic reissuance without durable
state.[^4] Accordingly, POC-005 makes no “first capability” or universal replay
claim.

## Independent verdict

```text
BOUNDED THEORY: MATERIALIZED
DIRECT FFI KNOWLEDGE WRITE: NOT MATERIALIZED
SEMANTIC REPLAY ACROSS FRESH ISSUANCE: NOT TESTED / NOT ESTABLISHED
ARCHITECTURAL ADVANCEMENT PATH: IDENTIFIED
LOCAL_PROPERTY: POC_READY
GLOBAL_THEORY: NOT_MATERIALISED
OVERALL: PARTIAL_POC
```

The strongest honest statement is that ESS-MAI materializes a software
separation between a copyable FFI identifier and once-consumable Rust
authority, and preserves the superior verdict boundary by holding the direct
write. Closing the path requires a verdict-bound durable transaction, not
weakening the hold.

## Sources

[^1]: Watson et al., CHERI capability architecture. [Primary paper](https://www.cl.cam.ac.uk/research/security/ctsrd/pdfs/201505-ssp2015-cheri-compartment.pdf).
[^2]: Rust Project, FFI safety obligations. [Official documentation](https://doc.rust-lang.org/nomicon/ffi.html).
[^3]: Saltzer and Schroeder, fail-safe defaults and complete mediation, 1975. [Author source](https://www.mit.edu/~Saltzer/publications/pubs.html).
[^4]: Xu et al., durable authorization state and semantic replay, arXiv:2608.01710. [Primary preprint](https://arxiv.org/abs/2608.01710).
