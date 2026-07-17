# ESS-MAI v1.6.1 — Harta e stimulimit të Vulës së Gjallë

## Modeli i stimuluar

```text
Mosbesim / no seal
  ↓
PRO ─┐
NPRO ├─ rotacion + XOR + wrapping-add → action_state
NPIM ┤
PIM  ┤        (zero SHA-256)
APRO ┤
MPRO ┘
  ↓
PIM + NPIM + MPRO final evidence
  ↓
Shadow main.rs complete mediation
  ↓
Shadow verifikon evidencat, X/Y, GCL, Spine dhe L-500
  ↓
SupremeVerdict
  ↓
ONE SHA-256 per platform mbi të njëjtin proof
  ├─ Shadow: prodhon
  ├─ Quantum: rillogarit
  └─ Light: rillogarit
  ↓
32-byte identity + fixed-point intensity + trust kind
  ↓
VerificationReceipt
  ↓
PD Quantum: output + iZ + Trust → next i0
  ↓
PD Light courier → Nura || Old Emotional UI → New UI
```

## Skenarët

### S1 — Besim ndërtimi

```text
verified=1, primitive=1
L-500=500
all system laws seal nonzero
action_state nonzero
legacy_score finite
```

Pritja:

- Shadow prodhon `TRUST_KIND_CONSTRUCTIVE`;
- Quantum prodhon të njëjtin SHA-256 dhe intensitet;
- receipt lidhet me `living_trust_digest`;
- PD finalizon `Released`;
- iZ dhe `next_i0` bartin farën e Besimit.

### S2 — Besim rigoroziteti negativ

```text
verified=0, primitive=0
L-500=500
negative evidence verified and persisted
action_state nonzero
```

Pritja:

- Shadow prodhon `TRUST_KIND_RIGOROUS_NEGATIVE`;
- negativa ruhet si aset;
- PD finalizon vetëm pasi ruajtja negative është provuar;
- `next_i0` lind si `RebuiltFromNegative`.

### S3 — Çift kushtetues i përzier

```text
(verified, primitive) ∈ {(1,0),(0,1)}
```

Pritja:

- `constitutional_kind = NONE`;
- nuk lind Vula e Gjallë;
- PD nuk prodhon iZ/next-i0 të besueshëm.

### S4 — Manipulim i action_state

Pritja:

- SHA-256 Shadow ≠ Quantum/Light;
- mospërputhja refuzohet para finalizimit PD.

### S5 — Manipulim i ligjeve aktive

Pritja:

- `system_laws_seal()` ndryshon;
- SHA-256 ×3 nuk përputhet;
- receipt dhe iZ nuk verifikohen.

### S6 — Manipulim i L-500

Pritja:

- `(flags & MASK) ^ XOR != 500`;
- `LivingTrustProof::is_admissible()` dështon;
- Shadow nuk lëshon Besim.

### S7 — Manipulim i intensitetit

Pritja:

- identiteti 32-byte mbetet identiteti i të njëjtit proof;
- `identity_digest()` ndryshon sepse lidh edhe intensitetin;
- receipt/iZ/next-i0 refuzojnë forcën e manipuluar.

### S8 — Manipulim i receipt-it

Pritja:

- `receipt_id` rillogaritet me `living_trust_digest`;
- receipt i vjetër nuk pranon Trust të ri dhe anasjelltas.

### S9 — Manipulim i handoff-it Quantum → Light

Pritja:

- CRC, receipt, Trust, output, iZ ose continuum mismatch;
- PD Light nuk e dërgon sinjalin as te Nura dhe as te UI emocionale.

## Kufijtë e simulimit

Nuk u simulua konfidencialiteti kriptografik, sepse nuk ekziston kontratë për
çelësat. U simulua integriteti, identiteti, lineage, complete mediation dhe
fail-closed ndër-platformik.
