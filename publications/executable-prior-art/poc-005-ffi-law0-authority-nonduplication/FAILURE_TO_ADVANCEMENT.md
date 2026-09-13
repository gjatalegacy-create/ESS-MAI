# Failure-to-advancement method

## Observed gap

`sovereign_validate_and_write` consumes valid authority, derives traceable
lineage, and then returns `CompatibilityHold (-8)`. It has no
`LawConfirmedVerdict`, so allowing a direct write would violate the current
authority hierarchy.

## Architecture-preserving closure

Do not make the FFI function a second Vault writer. Connect the already
consumed authority to the existing superior path:

```text
borrowed FFI payload
  -> one-shot CapHandle validation
  -> opaque LgcToken remains inside Rust
  -> canonical action/payload digest
  -> Shadow verification
  -> LawConfirmedVerdict
  -> VaultTransaction::build
  -> durable transaction / idempotent sink
  -> typed commit receipt
```

Required rules:

1. `LgcToken` never crosses FFI and never becomes `Copy`.
2. The consumed generation, canonical action digest, parent authority, and
   verdict receipt must be bound into one transaction identity.
3. The sink must be idempotent.
4. Consumption needed across restart must be recorded in monotonic durable
   state before reporting success.
5. Any missing verdict, lineage, transaction state, or sink authority remains
   HOLD/fail-closed.
6. Replay evidence may become a candidate negative asset only through the
   existing Shadow validation and transaction path.

## Advancement tests

- valid capability + valid committed verdict -> exactly one committed write;
- copied handle -> no second commit;
- fresh handle for the same authorization identity -> no second admission;
- crash after prepare, before commit -> deterministic recovery;
- crash after commit, before response -> idempotent response without duplicate;
- verdict mismatch or absent receipt -> HOLD, zero write;
- concurrent calls -> one transaction ID and one sink effect;
- all tests preserve the separation `GCL > Shadow verdict > Vault sink`.

This method advances the successful bounded result without weakening the
architecture to satisfy Cargo.

