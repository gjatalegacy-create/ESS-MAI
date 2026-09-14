# Theory and falsification contract

## 1. Existing theory

Capability systems treat a capability as authority-bearing rather than as a
mere identifier. Linear or single-use authorization, nonce validation,
reference monitors, deny-by-default decisions, atomic state transitions, and
FFI wrappers all have substantial prior art. POC-005 does not claim invention
of those primitives.

This category includes capability-as-authority, one-shot or linear
authorization, atomic state transition, reference-monitor mediation,
deny-by-default handling, and unsafe FFI contracts. They are prior art, not
renamed ESS-MAI inventions.

## 2. Bledar Gjata / ESS-MAI contribution

The project-specific contribution tested here is the constitutional
composition: a freely copyable C representation is kept below a Rust-owned
one-shot authority; after local consumption, the compatibility path is still
forbidden from becoming knowledge unless the superior confirmed-verdict and
Shadow transaction authority exists.

The contribution is an arrangement and authority boundary, not the invention
of the individual primitives.

## 3. Bounded innovation proposition

The research proposition is that **representation may cross the FFI boundary
without transferring or multiplying the authoritative consumption state**, and
that successful local authentication does not inherit the authority of the
parent knowledge-admission system. The disclosed experiment materially
supports the first half for one generation and the hold boundary. It does not
yet support durable semantic non-duplication.

## 4. ESS-MAI formulation

Let:

- `H = (generation, nonce)` be the C-visible `CapHandle`;
- `S[g] = (valid, expected_nonce)` be the Rust-owned slot for generation `g`;
- `A(g,t)` mean that authority for generation `g` exists at time `t`.

The focal rule is:

```text
H is Copy
S is not exported

validate(H):
  require S[H.generation] exists
  require H.nonce == S[H.generation].expected_nonce
  require CAS(S.valid, true -> false) succeeds
```

For one issued generation:

```text
sum(successful_CAS(g)) <= 1
A(g,t0) = true
successful_CAS(g,t1) => for every t > t1: A(g,t) = false
```

This is **authority non-duplication for one issued generation**. It is not a
proof against a trusted issuer creating a fresh generation for the same
semantic action.

## 5. Current materialization tested

The experiment tests four adversarial classes:

1. bit-identical copy and serial replay;
2. forged nonce;
3. unknown generation;
4. eight concurrent bit-identical copies.

The expected post-authorization result is `-8`, not `0`, because the current
legacy surface requires a committed verdict before persistence. Therefore the
negative test is part of the falsification contract:

```text
direct_ffi_write_success == false
compatibility_hold == true
```

## 6. Further research and variants

POC-005 does not establish:

- durable consumption across process restart;
- protection against fresh reissuance for the same semantic action;
- an idempotent knowledge sink;
- successful FFI-to-Vault commit;
- cryptographic unforgeability of the FNV module seal;
- formal verification or universal absence of bypass paths.

Those boundaries are conditions of scientific validity, not disclaimers added
after the experiment.

Architecture-preserving successor variations include a durable semantic
authorization key, idempotent transaction sink, restart/replay test, and a
typed handoff from the compatibility gate to the existing confirmed-verdict
transaction path. Each must preserve the fact that the FFI child cannot mint
or assume its parent's verdict/persistence authority.
