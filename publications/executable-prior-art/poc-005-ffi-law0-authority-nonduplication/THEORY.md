# Theory and falsification contract

## 1. Existing theory

Capability systems treat a capability as authority-bearing rather than as a
mere identifier. Linear or single-use authorization, nonce validation,
reference monitors, deny-by-default decisions, atomic state transitions, and
FFI wrappers all have substantial prior art. POC-005 does not claim invention
of those primitives.

## 2. ESS-MAI formulation

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

## 3. Materialization tested

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

## 4. Stronger theory not yet established

POC-005 does not establish:

- durable consumption across process restart;
- protection against fresh reissuance for the same semantic action;
- an idempotent knowledge sink;
- successful FFI-to-Vault commit;
- cryptographic unforgeability of the FNV module seal;
- formal verification or universal absence of bypass paths.

Those boundaries are conditions of scientific validity, not disclaimers added
after the experiment.

