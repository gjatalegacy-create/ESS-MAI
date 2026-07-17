# Audit v1.6.0 — GCL, Layers, Final Evidence and iZ

## Baseline

`v1.5.9_ess_mai`

## Authoritative interpretation retained

- GCL governs the entire process.
- Layer 1/2/3 deepen processing under GCL and cannot cross or replace GCL.
- PD Quantum activates Spine 9; PD Light is a contextual courier.
- Nura and old emotional UI are parallel consumers of verified iZ.
- Shadow main verifies final cycle evidence; Legacy Shadow continuously observes
  primitive evolution toward Legacy.
- X=input/cause and Y=output/effect.
- PIM/NPIM package module evidence before Shadow; Shadow verifies rather than
  discovering the evidence.

## Concrete disconnections found

1. The pre-v1.6.0 Layer receipts did not carry an explicit common GCL-process
   identity from activation through all Layers.
2. Shadow received projected metrics and `xy_mass`, but not enough complete MPRO
   evidence to independently recompute the sixteen measurements.
3. NPIM hardening was calculated and printed but the original profile was sent.
4. The Negative Knowledge blob and suggestion were not bound to final NPIM.
5. Light created input SHA-256 but the Light→Quantum wire omitted it.
6. PD Light returned only a Nura surface and had no typed parallel emotional-UI
   continuity message.
7. The first v1.6.0 evidence draft recomputed Layer receipts but still trusted
   precomputed continuum/activation digests; it did not carry enough material
   for independent Shadow recomputation.
8. The first emotional relay draft targeted `NEW_UI` inside `NuraCore`, but the
   real UI consumer listens to Light stdout through Tauri.

## Implemented closure

See `CHANGELOG_v1.6.0.md` and `ESS_MAI_V1_6_0_IMPLEMENTATION_MAP.md`.

## Static audit result

- No unmatched Rust delimiters across the source tree.
- Shared PD Spine contracts are byte-identical.
- Final wire codec order matches all five declared nested schemas.
- Shadow independently recomputes continuum stimulus, 1Q, PD activation,
  GCL process, Spine activation, each Layer source-material digest, the three
  Layer receipts and completion.
- The emotional relay uses the existing Light stdout→Tauri→new-UI transport.
- No direct Quantum→Shadow core dependency was reintroduced.
- No speculative Matrix weighting or Legacy merger was implemented.

## Boundaries where implementation stopped

1. No confidentiality cipher or key hierarchy was added. The architecture contains
   integrity identities and sovereign seals, but no concrete authority for key
   generation, rotation, custody or cipher-suite selection. Inventing one would have
   violated the evidence-only rule.
2. No raw PD generated-question payload was added to the public Shadow wire. The
   current PD contract exposes its digest, not a canonical serialisation of the raw
   semantic payload. v1.6.0 binds and verifies the digest but does not invent a new
   disclosure contract.
3. Matrix weighting and Knowledge-placement policy were not changed. Shadow receives
   stronger verified evidence, while the existing sovereign routing authority remains
   intact.

## Execution boundary

Cargo was unavailable in the packaging environment. `VALIDATE_V160.ps1` must
produce the final build/test/clippy evidence on Windows GNU.
