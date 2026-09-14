# POC-005 falsifiable claim

## CLAIM

> Does the existing ESS-MAI v189 FFI gate cause one freshly issued,
> generation-bound `CapHandle` to cross authority validation at most once,
> while refusing copied replay, forged nonce, and unknown generation, and while
> withholding a direct knowledge write when no committed superior verdict is
> present?

The tested claim is local to one issued generation and to the disclosed
`legacy_ffi_compat`/test surface. It is deliberately capable of producing a
negative result for the stronger semantic interpretation.

## REQUIRED OBSERVABLES

- the C-visible handle remains bitwise copyable;
- the first valid non-empty call reaches the post-authority boundary (`-8`);
- second and third uses of the same generation return `-1`;
- among eight concurrent copies, exactly one reaches `-8` and seven return
  `-1`;
- a forged nonce returns `-3` without consuming the genuine slot;
- an unknown generation returns `-2`;
- an empty payload returns `-5`, after which the same handle returns `-1`;
- two freshly issued generations for the same module and payload may both
  reach `-8`, exposing the semantic-reissuance boundary;
- no direct FFI knowledge write is reported as successful.

## FORBIDDEN INFERENCES

This POC does not establish durable consumption across restart, semantic
idempotency across fresh generations, cryptographic unforgeability of the FNV
seal, an end-to-end FFI-to-Vault write, absence of every bypass, production
deployment of the legacy FFI surface, or closure of the complete v189
architecture.

It also does not claim invention of capabilities, nonces, atomic
compare-and-swap, linear authorization, reference monitors, or fail-safe
defaults.

## FALSIFICATION CONDITION

The narrow claim is falsified if any copied use of the same generation reaches
the post-authority boundary more than once, if a forged/unknown handle is
accepted, if an invalid payload leaves reusable authority, or if the current
surface silently reports a successful knowledge write without the committed
verdict path.

The stronger semantic claim is not materialised if two fresh generations for
the same semantic operation can each pass the local gate. The experiment
actively searches for and publishes that counterexample.
