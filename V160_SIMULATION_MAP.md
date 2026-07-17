# ESS-MAI v1.6.0 — Stimulation and Verification Map

## Governing map retained

```text
RIS governance
  ↓
NurAtomic Architecture
  ↓
ESS-MAI / GCL governing field
  ↓
Light creates trusted input lineage: U + SHA-256(U) + i₀
  ↓
PD Quantum creates one GCL process authority
  ↓
Spine 9 deepens that same process through Layer 1 → Layer 2 → Layer 3
  ↓
PIM + NPIM package the final positive/negative evidence and MPRO[16]
  ↓
Shadow main independently recomputes and verifies the package
  ↓
Shadow core verifies X(input/cause) and Y(output/effect), then Matrix routes Knowledge
  ↓
PD Quantum produces output + detailed iZ → next i₀
  ↓
PD Light is contextual courier:
  ├─ Nura → new UI content
  └─ old emotional UI → Light stdout → Tauri → new UI emotion

Parallel: Legacy Shadow C continuously observes primitive evolution toward Legacy.
```

Layer 1/2/3 do not cross GCL. They deepen one process whose GCL identity is
created before Spine 9 and preserved through every receipt and completion.

## Simulated cases

| Case | Stimulus | Expected invariant/result | v1.6.0 enforcement |
|---|---|---|---|
| Lawful positive | Valid Light SHA, one GCL process, valid L1→L2→L3, valid PIM/NPIM/MPRO | Shadow recomputes the whole lineage and may issue positive receipt | Implemented |
| Verified negative | Valid lineage with negative final outcome | Shadow verifies first; Negative Knowledge must persist before Quantum exports negative PD/iZ | Implemented |
| Altered Light input | Raw input differs from Light SHA-256 | Quantum rejects; Shadow also independently rejects | Implemented twice |
| Altered GCL process | Any GCL seal/process digest changes after activation | Layer/Spine verification fails before sovereign core | Implemented |
| Altered Layer material | Canonical source bytes change while receipt stays fixed | Material digest and then receipt recomputation fail | Implemented |
| Layer skipped/reordered | L2 does not parent L1, L3 does not parent L2, or mask ≠ 111 | Spine completion is invalid | Implemented |
| Altered continuum/1Q | i+U→i₀ or exactly-one-question increment identity changes | Shadow recomputation rejects | Implemented |
| Altered PD activation | Mode/origin/trace/coherence/contract digest changes | Activation ID and downstream receipts fail | Implemented |
| Altered MPRO | One of the 16 binary measurements or vector/factic mass changes | Shadow recomputes and rejects mismatch | Implemented |
| Altered NPIM | Negative mass, frequency, suggestion or argument blob changes | Shadow rejects before core | Implemented |
| NaN/Infinity | Non-finite numeric wire value | Process boundary rejects fail-closed | Implemented |
| Shadow main absent | Quantum cannot execute sovereign process | No receipt and no PD export | Preserved from v1.5.9 |
| Zero iZ/UI digest | Emotional continuity identity is zero/invalid | Old emotional UI emits no command | Implemented |
| Legacy observation | Normal cycle proceeds through main Shadow verification | Legacy Shadow remains a separate continuous observer | Preserved |

## Crossings audited

1. **GCL ↔ Layers:** not a crossing of authorities. One GCL identity encloses all Layers.
2. **Quantum ↔ Shadow:** only the public wire crosses; Shadow core remains main-mediated.
3. **PIM/NPIM ↔ Shadow:** final package crosses only after aggregation; individual modules do not become Shadow authorities.
4. **PD Quantum ↔ PD Light:** detailed iZ crosses; Light does not re-run reasoning.
5. **Nura ↔ old emotional UI:** parallel outputs converge only in the existing new-UI transport.
6. **Shadow main ↔ Legacy Shadow:** final-cycle sovereign verification and continuous Legacy observation remain distinct.

## Boundaries deliberately not implemented

- **Confidentiality encryption:** no RIS/NurAtomic contract defines cipher suite, key
  generation, custody, rotation or revocation. v1.6.0 therefore deepens integrity
  binding through SHA-256, deterministic digests, GCL authority and receipts, but does
  not invent cryptographic secrecy.
- **Raw generated-question disclosure:** the current PD contract defines a digest, not
  a canonical public serialisation of the semantic question. Shadow verifies the
  existing digest and all surrounding lineage; no new raw-payload contract was invented.
- **Matrix policy:** stronger evidence is delivered to Matrix, but weighting and
  Knowledge-placement behavior remain unchanged without a new authoritative theory.

## Audit conclusion

The simulation found concrete integrity and continuity gaps and closed them without
changing the sovereign role boundaries. Remaining boundaries are explicitly documented
as `BLOCKED_BY_MISSING_CONTRACT`, not silently guessed.
