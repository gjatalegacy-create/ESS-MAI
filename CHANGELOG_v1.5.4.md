# ESS-MAI v1.5.4 — Deterministic PD Continuation and Verified Nura Surface

## Constitutional flow implemented

`GCL ↔ ESS-MAI` remains the governing constitution. The implementation preserves the three sovereign collapses:

- Light: Coordination Collapse — Primitive Trace / Pi-i0 → Xi → Yi.
- Quantum: Elimination Collapse — LIM → PRO → APRO → MPRO → PIM/NPIM.
- Shadow: Verification Collapse — waiting Pi-i0 + XY → Y 0/1 → X 0/1.

## Progressive Debatic split to the responsibility boundary

Progressive Debatic is no longer a single mixed core:

- `core.rs` owns only cognitive trace/evolution and PD decisions.
- `seal.rs` owns `SEAL_PD`, `LgcGate`, and the private 0-copy `LgcToken` authorization.
- `runtime.rs` owns the two-stage lifecycle and cannot turn a prepared candidate into a next i0 without the final Shadow receipt.

The first PD seal authorizes semantic pre-preparation only. It creates `PdPendingNextI0`; it does not publish a question.

## Shadow completion receipt

Shadow now creates the final verification receipt only after:

1. the sovereign pipeline has produced Y/X;
2. the waiting Primitive Anchor has closed the GCL Y→X loop;
3. Verification Collapse has been recorded;
4. the Shadow verification capability has been issued, validated, burned, and converted to an internal `LgcToken`.

The token never crosses the boundary. The portable receipt is bound to:

- session/current i0;
- parent i0;
- Primitive Anchor;
- XY digest;
- Y and X verdict bits;
- capability generation;
- verification seal.

The canonical receipt formula is byte-identical in Light, Quantum, and Shadow.

## Deterministic next i0

Quantum PD can release the next i0 only when both stages are valid:

`PD pre-seal + verified current output receipt → next i0`

- Y=1 and X=1 releases the prepared continuation.
- Y=0 and X=0 rebuilds the continuation from the disproved basis.
- any mismatched, tampered, unanchored, or non-constitutional receipt is rejected.
- the negative continuation is not exported until NPIM has been persisted in Negative Knowledge.

## Quantum → Light → Nura

Quantum exports a receipt-bound PD handoff only after Shadow completion. Light re-computes the canonical receipt and rejects downgrade/tampering. `PdLight` creates an opaque `VerifiedPdSurface`; Nura accepts only that type, not a free `String`.

Thus Nura remains the named user-facing identity, not a reasoning or verification authority.

## Primitive Trace continuity

The second `TraceInfo::new(...)` in Light runtime was removed. `LightResponse` carries the original Primitive Trace created at input. The same object now supplies:

- Quantum `trace_id`;
- PA/i0 export to Shadow;
- Phase9 identity/timestamp;
- post-Quantum EvolveTrace.

Light checks the continuity invariant before PA export and refuses a hybrid i0.

## Cargo evidence fixes retained

- the two `Coordination` scope errors remain corrected;
- all three opaque-`LgcToken` `unwrap_err()` compilation errors remain corrected without adding `Debug`, `Clone`, `Copy`, or `Send`;
- `SEAL_EBPF` remains Quantum-owned and limited to `EBPF_HYDRATOR`;
- `SEAL_PD` remains distinct and guards PD before mutation;
- the v1.5.3 `SessionNotFound` E0369 test is corrected with `matches!`, without adding `PartialEq` to `PdEngineOutput`.

## Build profiles

Release profile ownership was moved to the workspace root. `panic="unwind"` is explicit; global `panic="abort"` is not enabled.

## Static contract correction before packaging

The staged `open_session_sealed()` boundary initially applied `?` after an authorization closure returning `()`. Static type-flow review caught that this would return `()` from a `Result<(), PdError>` function. The boundary now returns `PdSealAuthority::authorize(...)` directly; the cognitive core remains unchanged.

## Verification boundary

The packaging environment did not contain Cargo/Rust. Static Rust syntax, TOML, field-completeness, shared-contract hashes, flow ordering, and archive integrity were checked. Cargo verification must be performed with `VALIDATE_V154.ps1` on the authoritative host.
