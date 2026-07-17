# ESS-MAI v1.6.0 — Architectural Closure

Baseline: `v1.5.9_ess_mai`

## Implemented

- GCL process authority now exists in `PdSpineRequest` before Spine 9 and is
  retained by Layer 1, Layer 2 and Layer 3 receipts; every PD mode activates
  the complete Layer mask `111`.
- Final evidence now carries the full continuum and cognitive-activation
  material. Shadow independently recomputes `i + U → i₀ → 1Q`, the activation
  contract, GCL process, Spine activation, each Layer canonical source
  material, each Layer receipt and completion.
- GCL post-Layer authorization verifies continuity and emits SHA-256 action/law
  traces; Layers remain deep processing under GCL, not intersecting authority.
- PIM, strengthened NPIM and MPRO now form one final evidence package.
- Shadow main recomputes PIM/NPIM projections, NPIM blob binding, 16 MPRO
  measurements, vector/factic mass and all finite-number constraints.
- Light input SHA-256 now travels Light→Quantum→Shadow and is recomputed at both
  receiving boundaries.
- Final evidence, PD GCL process and Spine completion lineage are bound into
  Shadow `xy_digest` and exposed to Matrix as verified state context.
- PD Light now emits a typed `VerifiedPdDelivery`: Nura surface plus an iZ
  emotional-continuity signal. The old emotional UI writes a typed
  `[PD_LIGHT/IZ]` runtime command to Light stdout; the existing Tauri parser
  consumes it and projects it into the new UI.
- Light emotional spine is explicitly distinguished from Quantum PD Spine 9.
- NPIM hardening now affects the actual final package and Negative Knowledge,
  rather than only diagnostic output.

## Preserved

- Shadow remains binary-only and main-mediated.
- Legacy Shadow C remains the continuous observer of primitive evolution toward
  Legacy; it is distinct from final package verification.
- X and Y remain separate input/cause and output/effect fields.
- Quantum proposes; Shadow verifies; Matrix routes according to verified state
  and Knowledge context.
- No license change.

## Boundaries not invented

- No confidentiality cipher/key-management hierarchy was added because no
  authoritative RIS/NurAtomic contract defines its keys, custody or rotation.
- The generated-question semantic payload remains represented by the digest in
  the existing PD contract; no raw-payload disclosure format was invented.
- Matrix policy was not reweighted or redesigned.

## Verification status

Static audit passed. Cargo build, tests and clippy are pending execution through
`VALIDATE_V160.ps1` in the Windows GNU toolchain environment.
