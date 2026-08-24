# Claim Boundary

## Claims supported by execution

- The complete minimal v189 `gcl-constitution` crate closure compiles as an isolated offline Cargo workspace.
- All six extracted production files—the five-file GCL crate closure plus one Shadow receipt source—are byte-identical to their v189 origins by SHA-256 and size.
- `UncertaintyLedger::record` accepts caller-reported tuples satisfying `after <= before`.
- A direct caller-reported expansion such as `2 -> 5` returns an error.
- Rejection does not append a step, both for an empty ledger and for an already-populated ledger.
- The upstream reference chain `16 -> 3 -> 2 -> 1 -> 1 -> 0` is accepted and classified as collapsed.
- Cross-step reset `16 -> 1`, then `100 -> 50`, is accepted by the current API and makes `current_space` grow from 1 to 50.
- A terminal Verification step can be followed by Coordination and the ledger becomes uncollapsed.
- Negative and infinite possibility-space values are accepted in the reproduced cases.
- `chain_report` omits the `before` value of every step after the first.
- The byte-identical Shadow receipt algorithm produces a deterministic receipt identifier for the tested fixed inputs.
- Changing only the LAW-0 close report does not change the current Shadow receipt identifier because LAW-0 is not an input.
- The harness-only domain-separated connection of the existing receipt ID and LAW-0 digest changes when the LAW-0 digest is altered.

## Claims supported by static production mapping

- v189 has one physical source crate for the shared GCL constitutional types, re-exported by Light, Quantum, and Shadow.
- Light has a separate checked `u64` coordinate contraction and receipt verification; it does not instantiate `UncertaintyLedger` in the mapped production path.
- Light embeds `legacy:<receipt_sha256>` in the existing `lgc_seal` transported to Quantum; Quantum retains that string but does not currently consume the legacy segment as continuity evidence.
- Quantum creates and consumes a local four-step `UncertaintyLedger`; continuity is maintained by caller variable reuse, not by the ledger type.
- Shadow creates a new one-step Verification ledger with a constant local `1 -> 0` close marker and binds the SHA-256 of that report through the cycle digest, transaction ID, WAL serialization, and replay validation.
- Shadow separately creates a SHA-256 verification receipt, transports it to Quantum, and Light performs its corresponding cycle closure checks.
- The current receipt and response wire omit `law0_digest`, `cycle_digest`, and `transaction_id`; the durable transaction co-binds the receipt and local LAW-0 digest only as sibling commitments.
- Current Light-to-Quantum and Quantum-to-Shadow wire types do not carry a continuous LAW-0 possibility-space transcript.
- `FinalEvidenceWire` is an existing Quantum-to-Shadow carrier that can be versioned for a canonical continuity sidecar; this is a source-backed advancement seam, not current integration.

## Claims not supported

- This POC does not show that every ESS-MAI runtime state is recorded.
- It does not prove global LAW-0 enforcement or one continuous Light-to-Quantum-to-Shadow ledger.
- It does not execute the full v189 system or its three production binaries end to end.
- It does not prove that caller-reported numbers came from real candidate sets or from the module named by the caller.
- It does not prove semantic uncertainty reduction, truth, correctness, confidence calibration, Shannon-information reduction, convergence, or termination.
- `is_collapsed()` is not a verified truth verdict; the current predicate also accepts negative values as collapsed.
- The local Shadow digest is an integrity binding for a constant local close marker, not a per-cycle upstream transcript, origin authentication, or a global attestation.
- The current public receipt does not independently prove the LAW-0 digest or durable commit.
- The harness-only connection hash is an advancement feasibility result, not current production integration.
- `chain_report` is not a complete audit trail under the current API.
- Byte identity applies only to the six files listed in `EXTRACTION_IDENTITY.md`; it is not a claim that three runtime binaries are identical.
- The artifact does not establish production readiness, security certification, patent novelty, patent invalidity, freedom to operate, legal priority, or grant eligibility.

## Architectural invariant for advancement

```text
GCL authority
  -> bounded Coordination evidence
  -> bounded Reasoning evidence
  -> bounded Verification evidence
```

The receipt may carry continuity; it may not transfer sovereignty. No child phase may mint the parent authority or assume a sibling's jurisdiction.
