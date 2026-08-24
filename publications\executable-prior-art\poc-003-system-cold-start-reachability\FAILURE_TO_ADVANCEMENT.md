# Failure to Advancement

## Observed failure

Generation zero has no request-bound positive or negative candidate. Besa can accurately attest complete emptiness, but Asht's ordinary relevance rule requires exact prior candidates before the route may advance. The final Shadow judgment/write path is therefore unreachable from the state that needs it to create the first durable knowledge.

```text
empty state
  → complete empty selection
  → exact prior-candidate requirement
  → fail-closed
  → downstream Shadow writer not reached
  → state remains empty
```

Fail-closed behavior is locally correct. The liveness problem is the circular precondition.

## Architectural invariants that must not be broken

- GCL remains the sole sovereign constitutional parent.
- Light coordinates but does not reason or decide.
- Quantum reasons but does not issue the final verdict or write persistent knowledge.
- Shadow remains the only final judge and persistent writer.
- Positive and negative candidate jurisdictions remain distinct.
- No harness, sibling, seed script, or external operator writes directly into the vault.
- Ordinary non-genesis cycles keep the existing Asht relevance rule.

## Narrow advancement hypothesis

Add a one-time, GCL-authorized genesis transaction mode routed through existing boundaries.

Required evidence:

1. `generation == 0`;
2. authenticated empty-state digest from Shadow;
3. current GCL directive and expected phase;
4. request/session/input/split/evidence digests;
5. nonce or capability that is single-use and replay-protected;
6. explicit typed absence of prior candidates;
7. terminal binding to the resulting Shadow transaction and receipt.

Required behavior:

1. Light requests genesis eligibility but cannot grant it.
2. GCL delegates a bounded one-cycle genesis jurisdiction.
3. Quantum may forward provisional reasoning plus typed absence; it still cannot decide.
4. Shadow independently verifies emptiness, evidence, session binding, and single-use status.
5. Shadow alone judges and commits through its existing transaction/WAL path.
6. The capability is consumed only after durable commit.
7. Every later request uses the ordinary Asht candidate rule.

Fail before any verdict/write on:

- a non-empty state;
- replayed or expired capability;
- wrong request, session, phase, or input digest;
- missing positive/negative separation;
- mismatched evidence;
- transaction or durability failure.

## Why this preserves the hierarchy

The special case is delegated by GCL and adjudicated by Shadow. It does not let Asht fabricate prior truth, let Light seed storage, or make the harness a peer authority. It provides a constitutionally bounded route to the existing writer while preserving sibling separation below and constitutional unity above.

## Variations for future experiments

### Variant A — single-use genesis capability

GCL issues a capability bound to an authenticated empty-state digest. Shadow consumes it during the first durable transaction. This is the narrowest candidate.

### Variant B — two-cycle provisional then canonical verification

Cycle one produces quarantined, non-authoritative candidate material; cycle two applies the ordinary exact relevance gate before promotion. Shadow remains the only writer. This costs an extra cycle and requires explicit quarantine semantics.

### Variant C — typed absence as negative evidence

Shadow records verified absence itself as a negative-knowledge event, then uses a separate GCL-bounded rule to obtain the first positive candidate. This may strengthen negative-knowledge provenance but does not alone solve the missing positive side.

### Variant D — externally signed genesis evidence

An external evidence package can be admitted only as input evidence, never as a direct store write. Shadow must verify, judge, and transact it through the same jurisdiction. This broadens the trust boundary and needs stronger signature/revocation policy.

None of these variants is claimed implemented. Each requires an executable POC with a non-empty-state rejection test, replay test, wrong-session test, failed-commit test, and proof that ordinary post-genesis behavior is unchanged.

## Funding-oriented work packages

1. formalize genesis typestate and GCL delegation;
2. connect authenticated empty-state evidence to the existing Shadow transaction seam;
3. implement single-use/replay protection;
4. test crash recovery and capability consumption around durable commit;
5. test the first ordinary post-genesis cycle;
6. fuzz wire/version boundaries and malformed evidence;
7. run a full private native system experiment after the surgical gate passes.

Experimental success plus experimental failure becomes an advancement method only when every boundary remains explicit and testable.
