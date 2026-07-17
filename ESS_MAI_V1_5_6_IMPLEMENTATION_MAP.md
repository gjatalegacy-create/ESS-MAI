# ESS-MAI v1.5.6 — Harta autoritative e implementimit

## Formula GCL e mbyllur

```text
i + U(user input) → i₀
PD(i₀) → output + iZ
output + iZ → next i₀
```

PD nuk është `Support`, nuk është thjesht `iZ` dhe nuk prodhon drejtpërdrejt `next i₀`.
PD nxit i₀-në, aktivizon Spine 9, rimerr tri shtresat, përgatit kandidatin,
pret verifikimin e Shadow dhe vetëm atëherë mbyll dy rezultate të ndara:
`PD verified output` dhe `iZ`. `next i₀` derivohet vetëm nga të dyja.

## Rrjedha ekzekutive

```text
Light PA / user input
  → session_id + Primitive Anchor real (parent i₀)
  → Quantum PD: PdTurn → trace → GeniusSignal? → PdCognitivePackage
  → PdContinuumActivation: i + U → i₀ + 1Q
  → PD aktivizon Spine 9
  → Layer 1 (HCP_PRO)
  → Layer 2 (EvidencePackage)
  → Layer 3 (Phase9/hardware release)
  → PdSpineCompletion kthehet te PD
  → vetëm tani: PendingNextI0 + PD pre-seal
  → MPRO 16-vlerësime, masë u32 fixed-point [0..10_000]
  → XY / Elimination Collapse
  → Shadow: Verified(Y) → Trust(X)
  → receipt i lidhur me kandidatin PD + aktivizimin i₀+1Q
  → PD Quantum finalize
       ├─ PdVerifiedOutput
       ├─ PdIzCompletion
       └─ derive_next_i0_id(output, iZ)
  → PD Light rindërton dhe verifikon të njëjtën formulë
  → Nura
  → UI e re (tekst)
```

Në paralel:

```text
runtime state → old UI EmotionalCommand → avatar/color/motion/animation → new UI
```

Kanali emocional nuk ndryshon dhe nuk ndërmjetëson tekstin e Nura-s.

## Kontratat identike

Këto skedarë janë byte-identikë në Light, Quantum dhe Shadow:

- `pd_continuum_contract.rs`
- `pd_spine_contract.rs`
- `lab_contracts/verification_receipt.rs`
- `lab_contracts/gcl_presume.rs`

Kontratat mbajnë të njëjtat tipe, versione, digest-e, njësi dhe semantikë.
Autoritetet mbeten të ndara: Quantum mat/eliminon; Shadow verifikon; Light koordinon/projekton.

## Kufijtë fail-closed

1. Pa Primitive Anchor real nga Light nuk shpiket `parent_i₀` nga `session_id`.
2. Pa `1Q` real aktivizimi PD–Spine 9 nuk është gati.
3. Pa mbylljen e Layer 1/2/3 nuk krijohet pre-seal.
4. Receipt-i lidhet me kandidatin PD dhe aktivizimin e kontinuumit.
5. Ndryshimi i output-it ndryshon `pd_output_digest`, `iZ` dhe `next i₀`.
6. PD Light refuzon handoff të vjetër ose të manipuluar pa `output + iZ`.
7. Nura merr vetëm `VerifiedPdSurface`.

## Algoritmi në Light

Deklarimi runtime është:

```text
MINI_ALGORITHM → ALGORITHM → PD_I0_TO_IZ
```

PD është `GclRole::PdContinuum` dhe është i domosdoshëm në fazën Reasoning.
Nuk regjistrohet më si `Support("debatik")`.
