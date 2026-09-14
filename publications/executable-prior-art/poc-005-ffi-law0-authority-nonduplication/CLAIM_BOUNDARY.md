# Claim boundary

## Supported

- The v189 `CapHandle` is C-compatible and copyable.
- The authoritative consumption state stays in a Rust-owned `AtomicBool`.
- A valid generation/nonce can pass the gate once.
- Bit-identical replays of that generation are refused.
- A forged nonce is refused without consuming the genuine handle.
- The current non-empty legacy direct-write route ends in
  `CompatibilityHold (-8)`.
- The result is reproducible in native v189 tests and in a detached Cargo POC.
- The strict public capsule passes all-target release build, 15/15 tests and
  3/3 independent principal runs.

## Not supported

- “Capabilities,” “single-use tokens,” atomic compare-and-swap, or reference
  monitors were invented by ESS-MAI.
- The handle is cryptographically unforgeable merely because its nonce is FNV.
- A fresh token cannot be issued for the same semantic authorization.
- The public FFI exports have a non-test in-repository default-production
  caller; none was found in the declared v189 caller search.
- Consumption survives restart.
- The legacy FFI surface currently performs an end-to-end Vault write.
- No bypass exists anywhere outside the tested closure.
- The mechanism is formally verified, certified, production-secure, or a
  completed patent search.

## Bounded contribution statement

The evidence supports a project-specific implementation contribution: a
copyable C ABI handle is separated from Rust-owned one-shot authority, and a
successful consumption is prevented from becoming a direct knowledge write
without the superior GCL/Shadow verdict path. Any novelty claim must be limited
to this precise constitutional composition and evaluated by qualified counsel
against the cited and uncited art.

`CLAIM_BOUNDARY=SEALED` applies only to this POC's local question. It does not
declare the full ESS-MAI v189 architecture closed.
