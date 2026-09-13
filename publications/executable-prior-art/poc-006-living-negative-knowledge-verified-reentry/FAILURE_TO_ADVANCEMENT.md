# Failure-to-advancement method

## Observed failure

The wire-level statement `negative_persisted: bool` answers only whether a
producer asserted persistence. It cannot independently prove:

- which negative asset was committed;
- which transaction committed it;
- which GCL/version governed admission;
- whether the exported history contains the same committed asset;
- whether the consumer verified the same identity before PRO activation.

The detached POC also reproduces an FNV-based seal. It detects the tested
single-byte corruption, but it must not be promoted to a cryptographic
integrity claim.

## Authority-preserving closure

Introduce a versioned transport type such as:

```text
NegativeCommitReceiptWire {
    schema_version,
    cycle_id,
    transaction_id,
    asset_digest_sha256,
    negative_class,
    gcl_law_version,
    vault_commit_status,
    exported_history_digest_sha256,
    history_required,
}
```

The type is a receipt, not a new authority.

1. Shadow remains the sole authority that validates and commits the asset.
2. The receipt is created only after the existing Vault transaction commits.
3. The exported history and receipt bind to the same SHA-256 digest and cycle.
4. The existing bridge transports the versioned receipt without interpreting
   or rewriting it.
5. Quantum independently verifies schema, digest, GCL version, sealed-history
   identity, and readiness before calling the existing PRO activation point.
6. Any absent, mismatched, corrupt, or downgraded receipt becomes a typed hold;
   at the activation boundary it becomes an authoritative fail-closed refusal.

No second writer is created. No sibling assumes its parent's or sibling's
authority. Cargo must encode this law through types and tests; the law must not
be weakened merely to obtain a green build.

## Required tests

- Boolean `true` without receipt must not activate PRO.
- Receipt without a committed Vault transaction must be impossible or refused.
- Asset digest and exported-history digest mismatch must be refused.
- Restart/WAL recovery must reproduce the same transaction and asset identity.
- Duplicate evidence must increment the intended frequency/access semantics
  without creating a contradictory second identity.
- Missing, corrupt, unknown-version, and downgraded receipts must block before
  reasoning.
- A valid receipt and valid history must preserve the currently passing path.
- An unrelated candidate must remain available for reasoning.

Alternative wire encodings are valid if these invariants and authority
boundaries remain unchanged.

