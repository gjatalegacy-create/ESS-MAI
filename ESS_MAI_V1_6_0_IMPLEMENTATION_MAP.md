# ESS-MAI v1.6.0 — Implementation Map

```text
Light U
 ├─ Primitive Trace / PA / Xi,Yi
 ├─ SHA256(U)
 └─ Light→Quantum contract carries SHA256(U)
      ↓
Quantum input gate recomputes SHA256(U)
      ↓
PD Quantum creates:
 ├─ Continuum evidence: i+U→i₀→1Q
 ├─ Cognitive activation evidence (all Layers=111)
 └─ PdSpineRequest + GclProcessAuthority G
      ↓
Spine 9
 ├─ L1 receipt {activation,G,parent=0}
 ├─ L2 receipt {activation,G,parent=L1}
 └─ L3 receipt {activation,G,parent=L2}
      ↓
GCL SHA-256 action/law authorization + PD pre-seal
      ↓
PIM + strengthened NPIM + MPRO[16]
      ↓ FinalEvidenceWire
Shadow main process boundary
 ├─ finite gate
 ├─ Light SHA-256 recomputation
 ├─ continuum stimulus + 1Q recomputation
 ├─ PD activation-contract recomputation
 ├─ GCL process + Spine activation recomputation
 ├─ L1/L2/L3 canonical source-material + receipt recomputation
 ├─ Spine completion recomputation
 ├─ PIM projection verification
 ├─ NPIM mass/frequency/suggestion/blob verification
 ├─ MPRO 16-measurement recomputation
 └─ GCL/Spine lineage verification
      ↓
Shadow core: X/Y verification + Matrix + Knowledge
      ↓
Receipt.xy_digest binds final evidence + G + Spine completion
      ↓
PD Quantum output+iZ→next i0
      ↓
PD Light courier
 ├─ Nura→New UI content
 └─ Legacy Emotional UI→Light stdout `[PD_LIGHT/IZ]`
                     →Tauri EmotionalCommand→New UI emotion

Parallel and preserved:
Legacy Shadow C continuously observes primitive evolution toward Legacy.
```

## Primary contracts

| Contract | Location | Authority |
|---|---|---|
| GCL-governed Spine 9 | `*/src/pd_spine_contract.rs` | GCL / PD Quantum |
| Continuum/activation/final evidence wire | `shadow-contracts/src/lib.rs` | Public shape + deterministic recomputation only |
| Process verification | `shadow/src/process_bridge.rs` | Shadow main |
| Receipt binding | `shadow/src/shadow_gateway.rs` | Shadow sovereign core |
| iZ courier | `light/src/pd_light.rs` | PD Light contextual |
| Emotional relay | `light/src/legacy_emotional_ui.rs` | Old UI→new UI |

## Non-invented boundaries

- No cipher/key-management layer was introduced without a governing RIS/NurAtomic contract.
- The generated-question semantic payload remains represented by its existing digest;
  v1.6.0 does not expose or invent a new raw-payload contract.
- Matrix policy remains authoritative and unchanged; only the evidence reaching it is
  more completely recomputable.
