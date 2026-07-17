# ESS-MAI v1.6.2 — Simulation Map

## S0 — Default distrust

```text
begin_cycle
state=0, mask=0, ledger=[]
Living Trust inadmissible
```

Expected: final evidence is rejected.

## S1 — Complete constructive cycle

All nine organs complete in canonical order. Shadow replays the same state/mask, verifies `(Y,X)=(1,1)`, L-500 and laws.

Expected: `CONSTRUCTIVE_TRUST`, SHA-256 receipt, Released next-i0.

## S2 — Complete rigorous negative cycle

All nine organs complete and Shadow verifies `(Y,X)=(0,0)`.

Expected: `RIGOROUS_NEGATIVE_TRUST`, negative asset persisted, rebuilt next-i0.

## S3 — SRK absent

Remove stage 4 from ledger.

Expected: mask differs from `0x03FE`; no Living Trust.

## S4 — Stub contribution attempt

Change evidence words and also send a self-consistent declared state.

Expected: Shadow recomputes the contribution from the words, then cross-binding to PIM/NPIM/MPRO/HCP fails.

## S5 — Duplicate or reordered stage

Duplicate NPRO as SRK or exchange PIM/SRK order.

Expected: duplicate/order guard rejects final evidence.

## S6 — TokenForge contamination

Attempt to include TokenForge stage in action convergence.

Expected: exact action mask/order rejects it. TokenForge may be READY but is not a reasoning organ.

## S7 — Receipt substitution

Change parent i0, PD binding, Living Trust SHA, Y/X, generation or seal.

Expected: SHA-256 receipt mismatch at Quantum and Light.

## S8 — Living Trust substitution

Change action state/mask, verdict, laws or L-500.

Expected: Shadow/Quantum/Light recomputation differs; no handoff to Nura.

## S9 — Non-finite input

Inject NaN or Infinity in scalar/candidate evidence.

Expected: Shadow rejects before core judgment.

## S10 — Cargo release gate

Run `VALIDATE_V162.ps1`.

Expected: release remains pending on any non-zero build/check/test/clippy exit. `CARGO_GREEN=TRUE` is written only after all required commands and file hashes pass.
