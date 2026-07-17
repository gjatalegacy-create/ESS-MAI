# Audit v1.6.2 — Untrust Start to End

## Authoritative map

```text
GCL governs the whole cycle
  ↓
begin_cycle: action_state=0, action_mask=0, ledger=[]
  ↓
HPRO → PRO → NPRO → NPIM → SRK → PIM → APRO → MPRO → HCP
  ↓
canonical evidence words → lightweight fold (zero SHA)
  ↓
PIM/NPIM/MPRO final package
  ↓
Shadow main.rs complete mediation
  ↓
recompute ledger + cross-bind module evidence + verify X/Y
  ↓
judge_supreme
  ↓
one Living Trust SHA-256 pulse over action_state + verdict + laws + L-500
  ↓
SHA-256 VerificationReceipt
  ↓
PD output + iZ + Trust → next i0
  ↓
PD Light courier → Nura and old emotional UI → new UI
```

## Detection

### Real convergence before v1.6.2

`ACTION_STATE` was real and received PRO, NPRO, APRO, MPRO, PIM and NPIM. HPRO, SRK and HCP existed in the runtime flow but were not all represented as independent required organs. In addition, TokenForge was mixed into the reasoning action state although its own contract says it is a non-deterministic runtime witness.

### Real disconnects

1. The Living Trust mask did not prove the exact full organ family.
2. Shadow received a ready-made `contribution`; it could replay the fold but could not recompute the contribution from canonical module material.
3. The ledger did not enforce the actual runtime order or per-stage schema.
4. `VerificationReceipt` used FNV64 as its identity.
5. `token_forge` used FNV64.
6. Cargo-green remained unproven in the packaging environment.

## Implemented closure

### Phase 1: structural Untrust

`begin_cycle()` resets pulse statuses, state, mask and ledger. Absence of completed action is the default distrust state. No trust seal exists here.

Each required module calls `mark_action` at the real completion point. `mark_action` stores:

```text
stage code + canonical evidence words
```

The contribution is recomputed by:

```text
contribution = converge_words(evidence_words) XOR stage_word(stage)
action_state = rotl(rotl(action_state,11) + contribution,7)
```

No SHA-256 is used in this phase.

### Exact organ contract

```text
Required mask: bits 1..9 = 0x03FE
Canonical order:
HPRO(1), PRO(2), NPRO(3), NPIM(8), SRK(4), PIM(7), APRO(5), MPRO(6), HCP(9)
```

The expected evidence-word schema is versioned and fail-closed:

```text
HPRO=5, PRO=3, NPRO=4, SRK=6, APRO=4,
MPRO=21, PIM=6, NPIM=5, HCP=5
```

### Shadow maximum verification

Shadow does not trust a prepared contribution. It receives the evidence words, recalculates every contribution, replays the ordered fold, reconstructs the mask and rejects duplicates, missing organs, extra organs, wrong order or wrong schema.

It then cross-binds the action ledger with the independently transported data:

- PRO count/first/last score ↔ candidate scores;
- NPRO completion ↔ NPIM argument count and bounded measures;
- SRK evidence-chain count ↔ PIM proof chain;
- HPRO physical booleans ↔ HPRO quartet inside MPRO;
- APRO quartet ↔ APRO MPRO measurements;
- MPRO 16 measurements and all derived masses ↔ final evidence;
- PIM 5D profile and proof count ↔ final evidence;
- NPIM negative profile and argument count ↔ final evidence;
- HCP id/generation/nonce/directive/sealed ↔ HCP wire.

This proves that `quantum_action_state` is derived from the declared real outputs, not from a single placeholder value.

### Phase 2: earned Trust

The Living Trust identity remains one SHA-256 pulse at supreme verdict. Receipt SHA-256 and TokenForge SHA-256 are separate gate/witness identities; they do not replace or multiply the Living Trust pulse.

`VerificationReceipt` now hashes the canonical receipt domain, version, length-delimited session/parent identities, PA, XY, PD binding, continuum activation, full Living Trust SHA-256, Y/X, generation and sovereign seal.

### iZ and token boundary

The existing `GclActionAuthorizationToken` already provides SHA-256 `action_sha256` and `law_trace_sha256` for the pending iZ/pre-seal boundary. v1.6.2 does not invent a second incompatible “untrust token”. The structural lifecycle is:

```text
cycle start: no proof → Untrust
module evidence accumulates → Untrust is progressively discharged
GCL pre-seal/action token → pending iZ is authorized for Shadow verification
Shadow verdict + Living Trust → final iZ/next_i0
next cycle begins at zero again
```

## Boundaries not invented

- Cargo-green is a release gate, not inserted into runtime Trust material, because no signed build-attestation authority, key, format or revocation contract exists.
- HMAC was not introduced because no sovereign key-management contract exists.
- FNV package/frame checksums remain non-sovereign transport checksums. They are not accepted as Trust, receipt or token identity; Shadow independently revalidates the evidence content.
- The ledger proves canonical evidence flow and cross-binding. Physical remote attestation beyond existing HPRO/HCP evidence was not invented.

## Test-state isolation

`runtime_pulse` dhe `token_forge` përdorin gjendje globale të procesit. Testet e tyre u serializuan me mutex vetëm nën `#[cfg(test)]`, në mënyrë që ekzekutimi paralel i Cargo-s të mos përziejë ciklet ose numëruesit. Kjo nuk prek kodin e prodhimit.

## Status

```text
Theory:                    COMPLETE_AS_DESIGN
Action convergence:        IMPLEMENTED_AND_REPLAYABLE
Nine-organ exact mask:     IMPLEMENTED
SRK full citizenship:      IMPLEMENTED
Shadow cross-binding:      IMPLEMENTED
Receipt SHA-256:           IMPLEMENTED
TokenForge SHA-256:        IMPLEMENTED
Trust → receipt → iZ:      IMPLEMENTED
Static audit:              PASSED_115_OF_115
Cargo-green:               PENDING_VALIDATE_V162
Release:                   PACKAGED_FOR_EXECUTIVE_VERIFICATION
```
