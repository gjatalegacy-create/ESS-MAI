# ESS-MAI — Authoritative Cross-Version Trace

## Constitutional Identity

**Architecture Category:** RIS (Royal Intelligent System) — Governance in data and flexible hardware.

**Architecture Family:** NurAtomic — deterministic verified secure output from verified trusted input, by deterministic collapse mathematics over a split Primitive Trace.

**System:** ESS-MAI — Core Foundation Research Governance over a sovereign, traceable cognitive runtime architecture governed by GCL and Primitive Trace.

`GCL ↔ ESS-MAI`. GCL is not an auxiliary module. ESS-MAI is the executive body of GCL, and GCL is the constitutional order of ESS-MAI.

---

## Baseline Version: v1.5.6

### Inherited Philosophy

Continuum formula:

```text
i + U(user input) → i₀
i₀ + 1Q(question IQ) → iZ(PD)
PD(i₀) → output + iZ
output + iZ → next i₀
```

`i₀` is the primitive origin. PD is the `iZ` completion, not `i₀` itself. PD activates Spine 9, receives the return from Layer 1/2/3, waits for Shadow, and only after verification finalizes the output and iZ.

### Verified Cargo State for v1.5.6

Windows logs dated July 14, 2026 established:

```text
cargo check --workspace --all-targets: exit 101
cargo test --workspace --all-targets --no-run: exit 101
blocking error: E0505
quantum/src/progressive_debatic/runtime.rs: completion was borrowed and moved within the same authority boundary
```

v1.5.6 was not declared Cargo-green.

### Identified Gap

`PdEngineOutput` retained the mode and GeniusSignal, but `PdSpineRequest` reduced them to `response_kind` and general-purpose digests. The completion retained only `layer_mask`, `evidence_digest`, and `mpro_mass`; it did not prove the L1→L2→L3 lineage or the PD subdivision that initiated the cycle.

---
# Implemented Version: v1.5.7

## Objective

To close the PD action contract before the GCL token:

```text
PD subdivision
→ typed activation contract
→ Spine 9
→ Layer 1 receipt
→ Layer 2 receipt bound to Layer 1
→ Layer 3 receipt bound to Layer 2
→ PD completion candidate
→ ESS-MAI/GCL laws
→ GCL SHA-256 authorization token
→ pre-seal
→ Shadow verification
→ verified output + iZ(SHA-256)
→ next i₀
```

## Philosophy of the PD Subdivisions

### Intellect

Ordinary language and lightweight tasks. The baseline plan activates Layer 1. It does not artificially elevate the request into deep research.

### Philosophy

The user possesses substantial ideas and intelligence but lacks structuring, rhetorical defense, or scientific formation. The baseline plan activates Layer 1 and Layer 2 for structuring and counter-evidence.

### Scientific

The user possesses knowledge, formation, and method, and conducts research or scientific dialogue. The baseline plan activates Layer 1, Layer 2, and Layer 3.

### Novel

A new idea originates from the user. Novel is not inferred from keywords and is not declared by PD without a trace. In v1.5.7, the Novel contract authorized for research originates from `GeniusDetected`, which preserves the origin of the signal and trace.

### Genius

Genius is not a parallel mode. It is an autonomous activator used when the system has accumulated sufficient information to pursue a solution to the Novel idea without further burdening the user.

```text
Novel(User)
+ sufficient trace
+ structural coherence
+ research readiness
→ GeniusAutonomousResearch
→ Layer 1 + Layer 2 + Layer 3
```

The Novel origin remains attributable to the user; research and solution formation are performed by the system.

## Actual Implementation

### 1. PD Activation Contract

Byte-identical files:

```text
light/src/pd_spine_contract.rs
quantum/src/pd_spine_contract.rs
shadow/src/pd_spine_contract.rs
```

Added types:

```text
PdCognitiveMode
PdActivationOrigin
PdActivationContract
PdLayerReceipt
```

`PdActivationContract` preserves:

```text
mode
origin
response_kind_digest
genius_signal_digest
trace_mass
structural_coherence
required_layer_mask
contract_digest
```

Genius rule:

```text
origin == GeniusAutonomousResearch
⇒ mode == Novel
⇒ genius_signal_digest != 0
⇒ required_layer_mask == Layer1|Layer2|Layer3
```

### 2. Binding PD Output to the Contract

File:

```text
quantum/src/progressive_debatic/runtime.rs
```

Functions:

```text
map_mode()
activation_from_output()
ingest_for_spine_sealed()
```

`Continue` preserves the actual mode of `PdTurn`. `GeniusDetected` produces `Novel + GeniusAutonomousResearch` and binds `signal_id`, `trace_ref`, `genius_score`, accumulated mass, and structural coherence.

### 3. Initial Selection without Inferring Novel

File:

```text
quantum/src/main.rs
```

The initial mode is derived only from the declared territory:

```text
philosophy/philosophical → Philosophy
science/scientific/research → Scientific
any other territory → Intellect
```

Novel is not selected from territory or keywords; it originates when `GeniusDetected` proves the trace of the user's idea.

### 4. Layer 1/2/3 Lineage

File:

```text
quantum/src/main.rs
```

A `PdLayerReceipt` is created for each layer:

```text
Layer 1 parent = 0
Layer 2 parent = Layer 1 result_digest
Layer 3 parent = Layer 2 result_digest
```

All three receipts are bound to:

```text
activation_id
activation_contract_digest
layer identity
result material
completed state
```

`PdSpineCompletion::closes_all_layers()` rejects the completion if the activation identity or the L1→L2→L3 chain is broken.

### 5. Constitutional GCL Token

Files:

```text
quantum/src/progressive_debatic/seal.rs
quantum/src/progressive_debatic/types.rs
```

Types:

```text
GclActionAuthorizationToken
PdAuthorizedCompletion
```

GCL consumes `PdSpineCompletion`; it does not borrow and move it simultaneously. The candidate is reconstituted as `PdAuthorizedCompletion`.

The token contains:

```text
contract_version
action_sha256
law_trace_sha256
law_mask
verdict
issued_at
```

Verified laws:

```text
continuum readiness
PD activation contract
Layer 1→2→3 lineage
Genius full-layer rule
ready-for-Shadow boundary
```

SHA-256 binds the concrete action; GCL/ESS-MAI confers authority. SHA-256 does not replace GCL.

### 6. Resolution of E0505

In v1.5.6:

```text
&completion → authority
completion → move inside the closure
```

In v1.5.7:

```text
PdSpineCompletion candidate
→ GCL consumes candidate
→ PdAuthorizedCompletion
→ PendingNextI0
```

`clone()` is not used to conceal the authority boundary. The remaining clone is used only after authorization to transport the same payload into existing structures, not during the constitutional borrow.

### 7. SHA-256 at iZ Formation

File:

```text
quantum/src/progressive_debatic/runtime.rs
```

`PdIzCompletion` and `PdNextI0` now carry:

```text
iz_sha256: [u8; 32]
```

The canonical iZ material includes:

```text
continuum activation digest
question increment digest
verified output digest
Shadow verification receipt id
GCL action SHA-256
GCL law-trace SHA-256
```

Accordingly, iZ is bound to the input, 1Q, verified output, Shadow receipt, and the constitutional GCL token.

## v1.5.7 Map

```text
User
  ↓
i + U → i₀
  ↓
PD Turn
  ├─ Intellect
  ├─ Philosophy
  ├─ Scientific
  └─ Novel (only from trace/Genius)
  ↓
PdActivationContract
  ↓
Spine 9
  ↓
Layer 1 Receipt
  ↓ parent digest
Layer 2 Receipt
  ↓ parent digest
Layer 3 Receipt
  ↓
PdSpineCompletion
  ↓
ESS-MAI/GCL law collapse
  ↓ verdict 1
GclActionAuthorizationToken
  ├─ action_sha256
  └─ law_trace_sha256
  ↓
PD pre-seal / PendingNextI0
  ↓
Quantum XY / Shadow Verified(Y) → Trust(X)
  ↓
PD Verified Output
  ↓
iZ + iz_sha256
  ↓
output + iZ → next i₀
```

## Static Simulations

### Intellect

Ordinary task → Intellect mode → Layer 1 is the contractual minimum. The current pipeline may produce all three layers, but GCL proves at least the required plan.

### Philosophy

Philosophy territory → Philosophy → Layer 1 and Layer 2 are required. Layer 2 must carry the parent digest of Layer 1.

### Scientific

Science/research territory → Scientific → all three layers are required and linked in order.

### Novel + Genius

`GeniusDetected` → Novel + GeniusAutonomousResearch → non-zero genius digest → all three layers are mandatory. Absence of any layer rejects the token.

### Candidate Swap

If Layer 2 or Layer 3 originates from another activation, `activation_id` or `activation_contract_digest` does not match and the completion is rejected.

### Broken Parent

If Layer 3 does not carry the parent digest of Layer 2, `closes_all_layers()` fails and no GCL token is produced.

## Verification Performed in This Package

- The `pd_spine_contract.rs` contract is byte-identical in Light, Quantum, and Shadow.
- The `pd_continuum_contract.rs` contract is byte-identical in Light, Quantum, and Shadow.
- Crate versions were raised to `1.5.7`.
- The borrow+move pattern that produced E0505 in v1.5.6 was eliminated.
- Call sites were statically inspected for initializers of modified types.
- A complete ZIP archive, rather than a patch, was created.

## Evidentiary Boundary

The packaging environment did not provide `cargo`, `rustc`, or `rustfmt`. Therefore, v1.5.7 is **not declared Cargo-green**. Authoritative verification must be performed on the user's Windows machine using `VALIDATE_V157.ps1`, and the logs must be recorded in this document in the subsequent version.

## Open Risks

- `iz_sha256` is formed and retained in Quantum objects; complete transport of all 32 bytes to PD Light must be verified in the next wire-contract audit.
- The existing pipeline produces all three layers even when the minimal mode plan requires fewer; execution optimization was not performed in v1.5.7 to avoid disrupting the operational architecture without Cargo evidence.
- Mode selection relies on the declared territory for Philosophy/Scientific; automatic semantic classification was not invented.
- In the next version, Shadow must verify `action_sha256` and `law_trace_sha256` as part of the final receipt, rather than only within the Quantum pre-seal.

---

This document shall not be renamed. Every subsequent version must append the version, philosophy, theory, formula, actual implementation, code locations, map, Cargo evidence, risks, and open matters.

---
# Evolution of v1.5.8

## Theory Name

**PD Receipt Lifetime Closure** — closure of the lifetime contract for recording the source of the PD handoff.

## Philosophy

Sovereign evidence does not change meaning because of a borrow/lifetime error. The error must be corrected at the narrowest proven boundary, without altering the receipt, the PD formula, or Shadow authority.

## Purpose

To eliminate `E0521` in `quantum/src/main.rs` without cloning, leaking, artificial allocation, or alteration of the flow:

```text
Shadow VerificationReceipt
→ Quantum recomputes receipt_id
→ rrjedha::note(site)
→ PD Light handoff
```

## Mathematical Model

```text
source ∈ S_static
S_static ⊂ S_str
rrjedha::note : S_static → Evidence
```

The proven call sites were constant literals:

```text
"main::export_pd_probe"
"main::export_pd_handoff"
```

Accordingly, the correct contract is:

```rust
source: &'static str
```

not `source: &str`.

## Runtime Flow

```text
PD closure
→ VerificationReceipt from Shadow
→ receipt_id is recomputed in Quantum
→ static source literal records the ledger only upon wire rejection
→ output + iZ → next i₀
```

## Contracts

- `VerificationReceipt` remained unchanged.
- `receipt_id(...)` remained unchanged.
- `PdContinuumClosure` remained unchanged.
- `rrjedha::note` continues to require `&'static str` for stable module identity.
- The SHA-256 manifest was normalized from absolute packaging paths to project-relative paths.

## Code Location

```text
quantum/src/main.rs
  export_pd_probe
  export_pd_handoff
  export_pd_verified_line

ESS_MAI_V1_5_7_FILELIST.sha256
VALIDATE_V157.ps1
AUDIT_V157_CARGO_PD_CONTINUITY.md
```

## Version Introduced

```text
Theory:         inherited from the v1.5.6/v1.5.7 PD flow
Implementation: v1.5.8
Verification:   static audit + Windows Cargo evidence for E0521
Release status: packaged as v1.5.8_ess_mai
```

## Verification Evidence

Windows logs established a single blocking error:

```text
error[E0521]: borrowed data escapes outside of function
quantum/src/main.rs
```

The call sites were traced and proven to be `&'static str` literals. The original manifest contained correct hashes but absolute Linux paths; the normalized manifest uses relative paths.

## Current Status

```text
Theory:         VERIFIED
Implementation: IMPLEMENTED
Verification:   CARGO_RECHECK_REQUIRED
Release:        RELEASED_AS_BASELINE_FOR_V1.5.9
```

---
# Implemented Version: v1.5.9

## Theory Name

**Shadow Main Necessary Precondition / Complete Mediation Contract**

## Philosophy

`Shadow lib.rs` owns the verification constitution, but it has no linkable voice outside the Shadow process. `main.rs` is not merely a recommended gateway; it is the structural precondition without which the sovereign code cannot be compiled as a target accessible to other crates.

```text
main.rs active
⇒ Shadow core exists at runtime

¬main.rs active
⇒ no linkable Shadow core
⇒ no ingest
⇒ no vault
⇒ no LgcToken
⇒ no VerificationReceipt
```

Quantum recognizes only the form of the evidence. It does not recognize its sovereign producer.

## Purpose

To close the actual v1.5.8 bypass through which Quantum could execute:

```text
shadow_lib::Shadow::new()
shadow.ingest_bridged(...)
shadow.on_negative(...)
```

by linking `shadow_platform` directly as an `rlib` and without executing `shadow/src/main.rs`.

## Mathematical Model

Let:

```text
Q = Quantum process
M = Shadow main.rs process
L = Shadow core/lib.rs
C = shadow_contracts
R = VerificationReceipt
```

v1.5.9 contract:

```text
Q ↛ L
Q → C → M → L
L → M → C → Q

R exists ⇒ M executed ∧ L verified ∧ token consumed
¬M ⇒ ¬communication(L) ∧ ¬R
```

Complete mediation:

```text
∀ access a to Shadow authority:
allowed(a) ⇔ mediated_by(M, a)
```

## Runtime Flow

```text
Light
  ↓ Primitive Anchor + Xi/Yi
Quantum
  ↓ PD + Spine 9 + Layer 1/2/3
  ↓ Quantum Collapse
ShadowCycleRequest (checksummed binary contract)
  ↓ process spawn
shadow_platform main.rs
  ↓ Phase9 no-bypass condition
  ↓ open persistent wisdom vault
  ↓ feed Light PA gate
  ↓ convert public wire → internal types
  ↓ Shadow::ingest_bridged
  ↓ Verification Collapse Y→X
  ↓ consume sovereign verification token
  ↓ produce VerificationReceipt
  ↓ persist NPIM Negative Knowledge in same Shadow instance
ShadowCycleResponse (checksummed binary contract)
  ↓
Quantum recomputes receipt_id
  ↓
PD finalize_after_verification
  ↓
output + iZ → next i₀
  ↓
PD Light → Nura
```

## Contracts

### Build Contract

```toml
shadow/Cargo.toml

autolib = false
autotests = false
autoexamples = true

[[bin]]
name = "shadow_platform"
path = "src/main.rs"
```

Shadow no longer produces:

```text
rlib
staticlib
```

`shadow/src/main.rs` includes:

```rust
include!("lib.rs");
```

Thus, the modules of `lib.rs` become part of the binary crate and are not exposed as a linkable library target.

### Public Form Contract

New crate:

```text
shadow-contracts/
```

It contains only:

- wire formats originating from Quantum;
- wire formats originating from Light;
- the Negative Knowledge wire format;
- the public form of `VerificationReceipt`;
- the minimal public verdict;
- a deterministic codec with version, frame type, length, and FNV-1a checksum;
- maximum bounds for frames, fields, and vectors.

It does not contain:

- `Shadow`;
- vault;
- pipeline;
- GCL authority;
- `LgcToken`;
- `seal_verified_output`;
- persistent-write API.

### Process Contract

Quantum resolves only the binary:

```text
ESSMAI_SHADOW_BIN=<absolute path>
```

or the sibling binary:

```text
target/debug/shadow_platform(.exe)
target/release/shadow_platform(.exe)
```

A missing binary, non-zero status, absent response, invalid checksum, or session mismatch results in fail-closed behavior.

### Identity Contract

The cycle request preserves the two origins separately:

```text
QuantumInboundWire
LightInboundWire
```

Before entering the core, Shadow main verifies:

```text
Quantum.session == Light.session
Quantum.territory == Light.territory
Quantum.primitive_flags == Light.primitive_flags
session != empty
```

The receipt remains bound to:

```text
session_id
parent_i0
primitive_anchor
xy_digest
pd_binding_digest
pd_continuum_activation_digest
Y
X
generation
seal
receipt_id
```

## Code Location

```text
Cargo.toml
shadow/Cargo.toml
quantum/Cargo.toml
shadow-contracts/Cargo.toml
shadow-contracts/src/lib.rs
shadow/src/main.rs
shadow/src/lib.rs
shadow/src/process_bridge.rs
quantum/src/main.rs
quantum/src/shadow_process_bridge.rs
```

## Version Introduced

```text
Theory:         v1.5.9 authoritative clarification
Implementation: v1.5.9
Verification:   static architecture audit in packaging environment
Release status: packaged; Cargo verification pending on Windows GNU
```

## Verification Evidence

Static evidence:

```text
quantum/Cargo.toml has no dependency on ../shadow
quantum source has no shadow_lib:: or shadow_platform:: core call
shadow/Cargo.toml has autolib=false
shadow/Cargo.toml has no [lib] target
shadow/src/main.rs includes lib.rs
Shadow core is invoked only from shadow/src/process_bridge.rs or interactive main
Quantum invokes shadow_platform as a child process
wire response is checksummed and session-bound
E0521 fix source: &'static str remains present
```

The shared contracts crate includes roundtrip tests and checksum-corruption tests. Unit tests inside Shadow modules remain compiled as binary-unit tests. The historical test `shadow/tests/integration.rs` is included by `shadow/src/main.rs` and executes within the binary target, without creating a linkable library. The example `shadow/examples/full_flow.rs` no longer accesses the core; it documents only the initiation of the mediated flow.

## Current Status

```text
Theory:         CONTRACT_DEFINED
Implementation: IMPLEMENTED
Runtime:        PROCESS_MEDIATED
Static audit:   PASSED
Cargo check:    PENDING_EXTERNAL_ENVIRONMENT
Cargo test:     PENDING_EXTERNAL_ENVIRONMENT
Release:        READY_FOR_WINDOWS_GNU_VERIFICATION
```

## v1.5.9 Map

```text
RIS governance
  ↓
NurAtomic architecture
  ↓
ESS-MAI
  ↓
Light Coordination Collapse
  ↓
Quantum Elimination Collapse
  ↓ public shadow_contracts only
Shadow main.rs necessary condition
  ↓ includes non-linkable lib.rs core
Shadow Verification Collapse
  ↓ checksummed VerificationReceipt
Quantum receipt re-verification
  ↓
PD output + iZ
  ↓
next i₀
  ↓
PD Light → Nura
```

## Evidentiary Boundary

The v1.5.9 packaging environment did not provide `cargo`, `rustc`, or `rustfmt`. For this reason, the status is not designated Cargo-green. Final evidence must be produced on Windows GNU with:

```text
cargo check --workspace --all-targets
cargo test --workspace --all-targets --no-run
cargo clippy --workspace --all-targets -- -W clippy::all
VALIDATE_V159.ps1
```

Every Cargo failure must be treated as concrete evidence for the subsequent cycle; speculative changes are not authorized.

---
# Evolution v1.5.9 → v1.6.0

## Theory Name

**GCL-Governed Deep Processing, Final Evidence Closure and iZ Dual-Surface Continuity**

## Philosophy

GCL is not a module that meets or intersects Layer 1, Layer 2, and Layer 3. GCL is the law declared over the entirety of ESS-MAI. PD Quantum opens the governed process; Spine 9 organizes it; Layer 1→2→3 only carries the same proceeding to greater depth. No Layer creates a new authority, verdict, or parallel identity.

PD Light is not a procedural copy of PD Quantum. It is the contextual courier of the verified iZ. From the same iZ, Nura and the legacy emotional UI emerge in parallel. Nura gives voice to the output; the legacy UI presents the state/animations and transmits the signal to the new UI. Neither reopens reasoning.

Shadow main verifies the final package of a cycle. Legacy Shadow C continues to observe the system and the progression of primitives toward Legacy. These are two distinct roles and were not merged.

## Purpose

v1.6.0 closes only proven discontinuities:

1. To prove that the same GCL process exists before Spine 9 and is preserved in every Layer 1→2→3 receipt.
2. To prevent only reduced masses from being sent to Shadow; PIM, NPIM, and MPRO must form a recomputable final package with evidence.
3. To make the SHA-256 of the Light input a real Light→Quantum→Shadow lineage.
4. To make PD Light a typed delivery of iZ to Nura and to the legacy emotional UI in parallel.
5. To keep Matrix bound to the final-evidence digest, the GCL process, and the Spine 9 completion without assigning Matrix the reasoning role of Quantum.
6. To distrust even ready-made continuum or activation digests: Shadow must recompute `i + U → i₀ → 1Q`, the PD cognitive contract, Spine 9 activation, Layer 1/2/3 receipts, and completion.

## Mathematical Model

```text
Light:
    i + U → i₀
    H₀ = SHA256(U)

GCL process authority:
    G = H(version, law_seal, system_laws_seal, phase,
          session, parent_i₀, continuum, activation, time)

PD / Spine 9:
    R₁ = H(L1, activation, G, parent=0, result₁)
    R₂ = H(L2, activation, G, parent=R₁, result₂)
    R₃ = H(L3, activation, G, parent=R₂, result₃)

    G(L1) = G(L2) = G(L3) = G

MPRO:
    mᵢ ∈ {0,1}, i=1..16
    positives = Σmᵢ
    vector_mass = positives / 16
    factic_mass = vector_mass × LIM_epistemic_mass

Continuum + activation evidence:
    S₀ = H(session, initial-i, U, i₀, time, "I_PLUS_U_TO_I0")
    Q₁ = H(S₀, 1Q, question, response-kind, time)
    A₀ = H(S₀, Q₁, "PD_CONTINUUM_ACTIVATION")
    Aₚ = H(mode, origin, response-kind, genius, masses, layers=111)

Final evidence:
    E = H(H₀, U, S₀, Q₁, A₀, Aₚ, G, SpineCompletion,
          PIM metrics + proof chain,
          NPIM metrics + arguments + argument-blob digest,
          MPRO[16] + vector/evidence/factic mass)

Shadow:
    recompute(H₀, S₀, Q₁, A₀, Aₚ, G, R₁, R₂, R₃,
              SpineCompletion, E, MPRO, PIM, NPIM)
    verify X(input/cause) and Y(output/effect)
    receipt.xy_digest binds E + G + SpineCompletion

PD Continuum:
    PD(i₀) → output + iZ
    GCL(output + iZ) → next i₀

Light delivery:
    PDLight(iZ) → Nura
                ∥ LegacyEmotionalUI → NewUI
```

`H` in the compact contracts is a deterministic contract digest. SHA-256 continues to be used for the Light input and for GCL authorization of the PD action. Wire packaging has a deterministic checksum and is subsequently bound within Shadow's sovereign receipt; it is not represented as confidentiality encryption.

## Runtime Flow

```text
User input U
  ↓
Light Coordination Collapse
  ├─ Primitive Trace / PA / Xi,Yi
  └─ SHA256(U)
  ↓
Quantum input gate
  └─ recompute SHA256(U); mismatch → fail-closed
  ↓
PD Quantum
  └─ creates PdSpineRequest with GclProcessAuthority G
  ↓
Spine 9
  ├─ Layer 1 receipt, bound to G
  ├─ Layer 2 receipt, bound to G and R₁
  └─ Layer 3 receipt, bound to G and R₂
  ↓
PD completion + GCL SHA-256 authorization/pre-seal
  ↓
PIM + NPIM + MPRO final evidence package
  ├─ PIM 5D + proof chain + suggestion
  ├─ NPIM strengthened profile + arguments + exact blob digest + suggestion
  ├─ MPRO sixteen binary measurements
  ├─ Light input SHA-256 + source bytes
  └─ G + Spine completion lineage
  ↓
Shadow main.rs necessary process boundary
  ├─ finite-number gate
  ├─ recompute Light SHA-256
  ├─ recompute i + U → i₀ stimulus and 1Q increment
  ├─ recompute PD cognitive activation (all three Layers = 111)
  ├─ recompute GCL process, Spine activation, L1→L2→L3, and completion
  ├─ recompute evidence-package digest
  ├─ recompute NPIM blob binding
  ├─ recompute MPRO vector/factic mass
  ├─ compare PIM/NPIM wire projections
  └─ call sovereign Shadow core only after all gates pass
  ↓
Shadow Verification Collapse
  ├─ X=input/cause
  ├─ Y=output/effect
  ├─ Matrix sees verified evidence/GCL/Spine lineage and current state
  ├─ verified negative → Negative Knowledge asset
  └─ verified positive → Knowledge routing under Shadow authority
  ↓
VerificationReceipt
  └─ xy_digest binds final evidence + GCL process + Spine completion
  ↓
PD Quantum finalizes output + detailed iZ → next i₀
  ↓
PD Light typed courier
  ├─ Nura → New UI content
  └─ Legacy Emotional UI → New UI emotion/animation signal

Parallel continuous path:
Legacy Shadow C observes primitive evolution toward Legacy during the system
lifecycle; it is not the final package verdict and was not removed.
```

## Contracts

### GCL Process Continuity Contract

`pd_spine_contract.rs` is byte-identical in Light, Quantum, and Shadow. It adds `GclProcessAuthority` and binds `gcl_process_digest` into each Layer receipt and the Spine completion. The post-Layer GCL seal now proves continuity of an authority that existed before the Layers; it does not introduce GCL after processing.

### Final Evidence Wire Contract

`shadow-contracts` protocol version 2 adds `FinalEvidenceWire`. It carries:

- `PdContinuumEvidenceWire`: the stimulus material `i + U → i₀`, the single `1Q` increment, their states, and the activation digest;
- `PdActivationEvidenceWire`: mode/origin, response kind, Genius signal, trace/coherence masses, the mandatory `111` mask, and contract digest;
- `PdSpineEvidenceWire`: GCL law/system seals, the GCL process, activation ID, the material and receipt of each Layer, plus Spine completion;
- Light input SHA-256 and exact input bytes;
- PIM fixed metrics, suggestion, and proof chain;
- strengthened NPIM metrics, suggestion, arguments, and argument-blob digest;
- sixteen MPRO measurements, positive count, total, vector mass, evidence mass, and factic mass;
- deterministic package digest.

Shadow main verifies the contract before creating `Shadow` or entering the core. It recomputes not only the package digests, but also the stimulus, 1Q, cognitive contract, GCL process, activation ID, the three receipts, and completion. NaN and ±Infinity are rejected at the process boundary.

### Negative Knowledge Contract

`NegativeKnowledgeWire` now carries `suggestion_code`. Shadow compares mass, frequency, suggestion, and blob digest against NPIM in the final package. The strengthened NPIM profile is now the profile actually transmitted and persisted; it no longer exists only as a log line.

### Input Identity Contract

The Light→Quantum payload now includes `input_sha256`. Quantum validates it against the received text before any reasoning. The final evidence carries the same SHA-256 and source bytes; Shadow recomputes it independently before entering the core.

### PD Light Delivery Contract

`PdLight::deliver` returns `VerifiedPdDelivery`:

```text
VerifiedPdSurface      → Nura → New UI content
PdUiContinuitySignal   → Legacy Emotional UI
                       → Light stdout `[PD_LIGHT/IZ]`
                       → Tauri EmotionalCommand
                       → New UI emotion/animation
```

Both channels originate only after receipt, continuum, output, and iZ have passed. `LegacyEmotionalUi` does not execute Layer 1/2/3, rejects a zero digest, and uses the existing real UI transport; it does not write to an unconsumed target.

### Matrix State Contract

`PassPackage` and `SystematizedCase` carry:

```text
final_evidence_digest
pd_gcl_process_digest
spine_completion_digest
```

Matrix receives these as verified state context. No new scoring weight or reasoning behavior was invented.

## Code Location

```text
light/src/light_coordinator.rs
light/src/quantum_bridge.rs
light/src/pd_light.rs
light/src/legacy_emotional_ui.rs
light/src/light_spine.rs
light/src/phase9_integration.rs
light/src/main.rs
light/src/pd_spine_contract.rs

quantum/src/bridge_light/mod.rs
quantum/src/main.rs
quantum/src/pd_spine_contract.rs
quantum/src/progressive_debatic/runtime.rs
quantum/src/progressive_debatic/seal.rs

shadow-contracts/src/lib.rs
shadow/src/main.rs
shadow/src/process_bridge.rs
shadow/src/pd_spine_contract.rs
shadow/src/bridge/quantum_in.rs
shadow/src/types.rs
shadow/src/shadow_matrix.rs
shadow/src/shadow_gateway.rs

Cargo.toml
light/Cargo.toml
quantum/Cargo.toml
shadow/Cargo.toml
shadow-contracts/Cargo.toml
ui/Cargo.toml
light/ui/src-tauri/Cargo.toml
```

## Version Introduced

```text
Theory:         authoritative clarification before v1.6.0
Contract:       v1.6.0
Implementation: v1.6.0
Static audit:   v1.6.0 packaging cycle
Release status: packaged for Windows GNU verification
```

## Verification Evidence

Static verification performed in the packaging environment:

```text
- all Rust source delimiters balanced
- PD Spine contract byte-identical in Light/Quantum/Shadow
- GCL process digest present in request, all three receipts, and completion
- no changed code introduced if/else in the implemented contracts
- wire codec order matches every declared schema:
  PdContinuumEvidenceWire(16), PdActivationEvidenceWire(8),
  PdLayerEvidenceWire(9), PdSpineEvidenceWire(24), FinalEvidenceWire(25)
- Shadow recomputes stimulus, 1Q, PD activation, GCL process, the canonical
  source material of every Layer, all Layer receipts, and Spine completion before
  entering sovereign core
- MPRO package requires exactly 16 binary measurements and recomputes masses
- NPIM bridge mass/frequency/suggestion/blob are compared by Shadow main
- Light input SHA-256 is required by Quantum input parser and recomputed twice
- Quantum retains no direct Shadow-core dependency
- Shadow remains binary-only and main-mediated
- PD Light dual delivery is typed and zero-digest fail-closed
- Legacy Shadow source path remains present and distinct
```

The implementation intentionally stops at two boundaries where the existing architecture has no concrete contract: no confidentiality cipher/key hierarchy was invented, and no raw generated-question payload was exposed beyond the digest already defined by the PD contract. Integrity remains enforced through SHA-256 lineage, deterministic digests, GCL bindings, and sovereign receipts.

The packaging environment did not contain `cargo`, `rustc`, or `rustfmt`. Therefore, Cargo build/test/clippy are not claimed as passed. The release includes `VALIDATE_V160.ps1` for authoritative Windows GNU evidence.

## Current Status

```text
Theory:              CONTRACT_DEFINED
GCL/Layers contract: IMPLEMENTED
Final evidence:      IMPLEMENTED
Input SHA lineage:   IMPLEMENTED
PD Light iZ courier: IMPLEMENTED_OVER_REAL_UI_TRANSPORT
Continuum/activation proof: IMPLEMENTED_AND_RECOMPUTED_BY_SHADOW
Shadow verification: IMPLEMENTED_AT_PROCESS_BOUNDARY
Legacy Shadow:       PRESERVED_AS_CONTINUOUS_OBSERVER
Static verification: PASSED
Cargo check:         PENDING_EXTERNAL_WINDOWS_GNU
Cargo tests:         PENDING_EXTERNAL_WINDOWS_GNU
Release:             PACKAGED_FOR_VERIFICATION
```

## v1.6.0 Map

```text
RIS governance
  ↓
NurAtomic architecture
  ↓
ESS-MAI / GCL governing field
  ↓
Light: trusted input coordination + PA + SHA256(U)
  ↓
PD Quantum creates one GCL process G
  ↓
Spine 9
  ├─ Layer 1 deepens G
  ├─ Layer 2 deepens G
  └─ Layer 3 deepens G
  ↓
PIM + NPIM + MPRO final evidence
  ↓
Shadow main complete mediation
  ↓
recompute evidence + verify X/Y + Matrix/Knowledge state
  ↓
VerificationReceipt binds E + G + Spine completion
  ↓
PD Quantum: output + detailed iZ → next i₀
  ↓
PD Light courier
  ├─ Nura → New UI
  └─ Old Emotional UI → New UI

Parallel:
Legacy Shadow C → continuous primitive-to-Legacy observation
```

## Boundaries Deliberately Not Crossed

- Layer 1/2/3 were not moved to Light.
- The Light emotional spine was not renamed into, or made equivalent to, PD Spine 9.
- Matrix was not assigned new speculative weights.
- Legacy Shadow was not removed or merged with the final Shadow receipt.
- PIM/NPIM/MPRO modules do not call Shadow separately; only the final package crosses the process boundary.
- No license was invented or added.

---
# Evolution v1.6.0 → v1.6.1

## Theory Name

**GCL — Living Trust Seal**

## Philosophy

ESS-MAI does not trust itself during processing. Distrust is the constitutive state: throughout PRO/NPRO/NPIM/PIM/APRO/MPRO, no Trust seal exists. The organs only converge their actions into a lightweight state. Trust originates only after Shadow issues the supreme verdict and only when the active laws, L-500, the verdict, and the evidence close within the same proof.

The seal is not a new authority above GCL. It is the final evidence that the same GCL process, deepened by Spine 9 and Layer 1/2/3, has reached a constitutional verdict. Shadow produces it; Quantum and Light recompute it.

## Purpose

- To make L-500 a runtime law bound to every Trust instance produced.
- To bind the actual work of the modules to the supreme verdict without per-module hashing.
- To produce a single SHA-256 identity of the entire proof at the culmination of the cycle.
- To separate the identity of Trust from its intensity.
- To bind Trust to `VerificationReceipt`, iZ, and `next_i0`.
- To require complete equality across Light ∥ Quantum ∥ Shadow.
- To preserve distrust as the default: without a seal, there is no trustworthy PD finalization.

## Mathematical Model

### Phase 1 — Action Convergence, Zero SHA-256

For every organ that actually completes:

```text
A₀ = 0
Aₙ₊₁ = ROTL(ROTL(Aₙ,11) + (CONVERGE(Eₙ) XOR STAGEₙ), 7)
```

where `CONVERGE` uses only rotation, XOR, and wrapping addition. State `A` is the progression toward the seal; it is not Trust.

Contributing organs:

```text
PRO → NPRO → NPIM → PIM → APRO → MPRO
```

The order is the actual execution order of the cycle. All contributions are bound within the final PIM/NPIM/MPRO package verified by Shadow.

### Phase 2 — The Single SHA-256 Pulse at Verdict

```text
proof = action_state
      || verdict.verified
      || verdict.primitive
      || verdict.knowledge_band
      || verdict.lgc_law
      || legacy_bits(verdict.lgc_law)
      || system_laws_seal(SYSTEM_LAWS)
      || sovereign_flags
      || sovereign_value_500
```

```text
living_trust_identity = SHA256(
    "GCL_LIVING_TRUST_V161"
    || contract_version
    || proof
)
```

SHA-256 does not contain the intensity. The identity establishes **which proof was sealed**; the intensity establishes **how much strength the Trust acquired**.

### Intensity

The existing Shadow formula is preserved:

```text
legacy_score = evidence_density     × 0.25
             + logical_coherence    × 0.20
             + causal_integrity     × 0.25
             + convergence_strength × 0.15
             + reproducibility      × 0.15
```

```text
intensity = round(clamp(legacy_score, 0, 1) × 10000)
```

### Constitutional Type

```text
(verified, primitive) = (1,1) → CONSTRUCTIVE_TRUST
(verified, primitive) = (0,0) → RIGOROUS_NEGATIVE_TRUST
any other pair                   → NO_TRUST
```

### Compact Binding

The existing contracts use a binding `u64`:

```text
living_trust_digest = FNV64(
    identity_sha256 || intensity || kind || sovereign_value || domain
)
```

This digest is not the seal and does not replace SHA-256. It binds the identity and strength to the receipt, iZ, and `next_i0`.

## Runtime Flow

```text
i + U
  ↓
Light: coordination + SHA256(input) → i0
  ↓
PD Quantum under GCL
  ↓
Spine 9 → Layer 1 → Layer 2 → Layer 3
  ↓
PRO/NPRO/NPIM/PIM/APRO/MPRO → action_state (zero SHA)
  ↓
final evidence package
  ↓
Shadow main.rs complete mediation
  ↓
recompute evidence + verify X/Y + Matrix/Knowledge
  ↓
SupremeVerdict
  ↓
L-500 + system laws + action_state + verdict
  ↓
Shadow SHA256 → LivingTrustSeal
  ↓
VerificationReceipt binds living_trust_digest
  ↓
Quantum recomputes identical SHA256
  ↓
PD: output + iZ + Trust → next i0
  ↓
Light recomputes identical SHA256 + receipt + iZ
  ↓
PD Light courier
  ├─ Nura → New UI
  └─ Old Emotional UI → New UI
```

In parallel:

```text
Legacy Shadow C → continuous observation of primitive → Legacy
```

Legacy Shadow is not the producer of the final receipt; it remains the continuous observer of the system.

## Contracts

### `living_trust_contract.rs`

Byte-identical contract in Light, Quantum, and Shadow:

- `LivingTrustProof`;
- `LivingTrustSeal`;
- constitutional types;
- L-500 constants;
- canonical SHA-256;
- fixed-point intensity;
- compact `identity_digest` binding.

### `VerificationReceipt`

`living_trust_digest` is part of the `receipt_id` material. A receipt cannot be detached from the Trust produced by Shadow.

### PD Continuum

`PdVerificationCompletion`, `PdIzCompletion`, and `PdNextI0` carry:

- the SHA-256 of Trust;
- intensity;
- type;
- sovereign value 500;
- binding digest.

`iz_sha256` is bound to the output, continuum, and Trust.

### Wire Contract

`shadow-contracts` uses `PROTOCOL_VERSION = 3`. The wire carries the proof required for Quantum and Light not to trust Shadow blindly, but to recompute the seal.

## Code Location

```text
light/src/living_trust_contract.rs
light/src/lab_contracts/verification_receipt.rs
light/src/pd_continuum_contract.rs
light/src/pd_light.rs
light/src/main.rs

quantum/src/living_trust_contract.rs
quantum/src/runtime_pulse.rs
quantum/src/main.rs
quantum/src/lab_contracts/verification_receipt.rs
quantum/src/pd_continuum_contract.rs
quantum/src/progressive_debatic/types.rs
quantum/src/progressive_debatic/runtime.rs

shadow/src/living_trust_contract.rs
shadow/src/shadow_gj_legacy.rs
shadow/src/sovereign_ffi_gate.rs
shadow/src/lab_contracts/verification_receipt.rs
shadow/src/pd_continuum_contract.rs
shadow/src/process_bridge.rs
shadow/src/types.rs
shadow/src/bridge/*

shadow-contracts/src/lib.rs
```

## Version Introduced

```text
Theory:         v1.6.1 authoritative paradigm
Contract:       v1.6.1
Implementation: v1.6.1
Static audit:   v1.6.1 packaging cycle
Release status: packaged for Windows GNU verification
```

## Verification Evidence

The stimulation audit covers:

1. `(1,1)` → constructive Trust;
2. `(0,0)` → rigorous negative Trust;
3. mixed pair → no Trust;
4. manipulation of `action_state`;
5. manipulation of active laws;
6. manipulation of L-500;
7. manipulation of intensity;
8. manipulation of the receipt;
9. manipulation of the Quantum→Light handoff.

Static packaging evidence is preserved in:

```text
AUDIT_V161_GCL_LIVING_TRUST.md
V161_SIMULATION_MAP.md
ESS_MAI_V1_6_1_IMPLEMENTATION_MAP.md
STATIC_AUDIT_V161.txt
V1_6_1_FROM_V1_6_0.diff
ESS_MAI_V1_6_1_FILELIST.sha256
VALIDATE_V161.ps1
```

## Current Status

```text
Theory:                    COMPLETE_AS_DESIGN
Action convergence:        IMPLEMENTED
L-500 runtime binding:     IMPLEMENTED
Supreme SHA-256 pulse:     IMPLEMENTED
Light/Quantum/Shadow ×3:   IMPLEMENTED
Receipt binding:           IMPLEMENTED
Trust → iZ → next_i0:      IMPLEMENTED
Static syntax/contracts:   PASSED_81_OF_81
Cargo check/test/clippy:   PENDING_EXTERNAL_WINDOWS_GNU
Release:                   PACKAGED_FOR_EXECUTIVE_VERIFICATION
```

## Boundaries Deliberately Not Crossed

- No new authority was created for Layers; they remain deepening mechanisms under GCL.
- PD Light was not made a processor of Spine 9.
- Legacy Shadow was not merged with final Shadow.
- The seal was not persisted as an independent Knowledge state.
- Existing `legacy_score` weights were not changed.
- No confidentiality encryption was invented without a concrete contract for keys, rotation, revocation, and storage authority.

---
# Evolution v1.6.1 → v1.6.2

## Theory Name

**GCL — Untrust Start to End / Complete Proof of Organ Convergence**

## Philosophy

ESS-MAI does not begin a cycle by trusting itself, its modules, or its platforms. The constitutional initial state is **Untrust**: no organ is regarded as complete, no contribution is regarded as proof, and no Trust Seal exists.

Distrust is not discharged by a module's declaration. It is discharged only when the organ:

1. executes its actual work;
2. emits its canonical evidence material;
3. is recorded in the constitutional order of the cycle;
4. is independently reverified by Shadow against the source material;
5. is bound to the final PIM/NPIM/MPRO package and to the X→Y relationship.

Trust originates only after the supreme verdict, as a single SHA-256 pulse of the **Living Seal**. The receipt and TokenForge use SHA-256 as separate integrity gates; they do not create additional supreme seals and do not replace Living Trust.

## Purpose

This evolution closes the critical v1.6.1 question:

```text
Is quantum_action_state a real convergence of module work,
or a declared/stub value?
```

v1.6.2 makes the answer verifiable:

- the nine required organs emit evidence at their actual completion points;
- the wire does not carry only a ready-made contribution;
- Shadow receives the canonical evidence words;
- Shadow recomputes every contribution, the mask, and the complete fold;
- Shadow cross-checks the ledger against the independent PIM, NPIM, MPRO, PRO, SRK, HPRO, and HCP structures;
- Living Trust is permitted only when the complete family of organs has been proven.

The secondary objective is to eliminate FNV64 from identities governing distrust/trust:

- `VerificationReceipt` transitions to SHA-256;
- `TokenForge` transitions to SHA-256;
- FNV remains only in non-sovereign checksums or legacy traces where content is independently reverified.

## Mathematical Model

### 1. Initial Untrust State

```text
A₀ = 0
M₀ = 0
L₀ = []
Trust₀ = ∅
```

where:

- `A` is `action_state`;
- `M` is the mask of completed organs;
- `L` is the evidence ledger;
- absence of the Seal is structural distrust.

### 2. Required Family of Organs

The actual canonical order is:

```text
HPRO → PRO → NPRO → NPIM → SRK → PIM → APRO → MPRO → HCP
```

Organ codes occupy bits `1..9`; the complete mask is:

```text
REQUIRED_ACTION_MASK = 0x03FE
```

The canonical material schema is:

```text
HPRO = 5 words
PRO  = 3 words
NPRO = 4 words
SRK  = 6 words
APRO = 4 words
MPRO = 21 words
PIM  = 6 words
NPIM = 5 words
HCP  = 5 words
```

### 3. Lightweight Contribution of an Organ

For canonical evidence `Eₘ = [e₁, …, eₙ]` of organ `m`:

```text
Cₘ = converge_words(Eₘ) XOR stage_word(m)
```

`converge_words` uses only rotation, XOR, and wrapping addition. No SHA-256 is performed in Phase 1.

### 4. Convergence Fold

```text
Aₖ₊₁ = ROTL₇(ROTL₁₁(Aₖ) + Cₘ)
Mₖ₊₁ = Mₖ OR bit(m)
Lₖ₊₁ = Lₖ || {m, Eₘ}
```

Convergence is accepted only when:

```text
M_final = 0x03FE
order(L_final) = REQUIRED_ACTION_ORDER
schema(L_final) = REQUIRED_ACTION_WORD_COUNTS
replay(L_final) = A_final
```

### 5. Complete Proof

Completion of an organ is not established merely by its presence in the ledger. Shadow cross-checks the material against independently transported evidence:

```text
PRO  ↔ candidate scores
NPRO ↔ NPIM negative package
SRK  ↔ conservation/IBE/evidence chain ↔ PIM proof chain
HPRO ↔ HPRO measurements inside MPRO
APRO ↔ APRO measurements inside MPRO
MPRO ↔ 16 measurements + vector/factic masses
PIM  ↔ positive profile + proof count
NPIM ↔ negative profile + argument count
HCP  ↔ id + generation + nonce + directive + sealed state
```

Therefore:

```text
COMPLETE_PROOF ⇔
    exact_mask
  ∧ exact_order
  ∧ exact_schema
  ∧ replayed_action_state
  ∧ all_cross_bindings
  ∧ X/Y verification
```

### 6. Living Seal

Only after `judge_supreme` produces a constitutional verdict:

```text
LivingTrustSHA256 = SHA256(
    domain
  || version
  || action_state
  || action_mask
  || required_action_mask
  || verdict.verified
  || verdict.primitive
  || verdict.knowledge_band
  || verdict.lgc_law
  || legacy_bits(verdict)
  || system_laws_seal
  || sovereign_seal_500
)
```

Intensity remains a distinct fixed-point strength; it is not included in the Seal identity, but is subsequently bound to the receipt, iZ, and `next_i0`.

### 7. Sovereign Receipt

```text
ReceiptSHA256 = SHA256(
    receipt_domain
  || receipt_version
  || session
  || parent_i0
  || primitive_anchor
  || xy_digest
  || pd_binding_digest
  || continuum_activation_digest
  || LivingTrustSHA256
  || Y
  || X
  || generation
  || sovereign_seal
)
```

The receipt no longer uses FNV64 as a security identity.

### 8. TokenForge

TokenForge produces a 32-byte SHA-256 runtime witness. It is not a reasoning organ and is not permitted to alter `action_state` or the nine-organ mask.

### 9. iZ and the Subsequent Cycle

```text
Untrust(start)
  → evidence × organ
  → verified convergence
  → Shadow verdict
  → Living Trust
  → VerificationReceipt
  → output + iZ + Trust
  → next i0
  → Untrust(next cycle)
```

Earned Trust is a continuity seed, but the subsequent cycle begins again with zero ledger/mask/state. Trust is not inherited as an unconditional privilege.

## Runtime Flow

```text
Light: i + U → SHA256(input) → i0
  ↓
Quantum begin_cycle()
  action_state=0, action_mask=0, ledger=[]
  ↓
PD Quantum under GCL
  ↓
Spine 9 → Layer 1 → Layer 2 → Layer 3
  ↓
HPRO completes → mark_action(HPRO, canonical evidence)
PRO  completes → mark_action(PRO, canonical evidence)
NPRO completes → mark_action(NPRO, canonical evidence)
NPIM completes → mark_action(NPIM, canonical evidence)
SRK  completes → mark_action(SRK, proof-carrying evidence)
PIM  completes → mark_action(PIM, canonical evidence)
APRO completes → mark_action(APRO, canonical evidence)
MPRO completes → mark_action(MPRO, 16 measurements + masses)
HCP  completes → mark_action(HCP, canonical evidence)
  ↓
Quantum verifies exact mask/order/schema/replay
  ↓
PIM/NPIM/MPRO final evidence + raw action ledger
  ↓
Shadow main.rs complete mediation
  ↓
Shadow recomputes every contribution, mask, and fold
  ↓
Shadow cross-binds ledger with module evidence
  ↓
Shadow verifies X=input/cause and Y=output/effect
  ↓
judge_supreme + L-500 + active laws
  ↓
one Living Trust SHA-256 pulse
  ↓
SHA-256 VerificationReceipt
  ↓
Quantum recomputes receipt and Living Trust
  ↓
PD finalize: output + iZ + Trust → next i0
  ↓
PD Light recomputes and acts only as contextual courier
  ├─ Nura → New UI
  └─ Old Emotional UI → New UI
```

In parallel, Legacy Shadow C continues observing primitives toward the Legacy state and does not become a duplicate of `judge_supreme`.

## Contracts

### `runtime_pulse.rs`

Defines:

- cycle reset;
- the nine mandatory organs;
- canonical order;
- word schema;
- SHA-free fold;
- source-material ledger;
- deterministic replay;
- separation of TokenForge from action convergence.

### `shadow-contracts`

`PROTOCOL_VERSION = 5` transports:

- `action_state`;
- `action_mask`;
- `required_action_mask`;
- ledger entries `{stage, evidence_words}`;
- final PIM/NPIM/MPRO material;
- Light input proof;
- GCL/PD/Spine material;
- HPRO/HCP and other evidence required for reverification.

### `living_trust_contract.rs`

Byte-identical contract in Light, Quantum, and Shadow. Living Trust is accepted only when the received mask and required mask are both exactly equal to `0x03FE`.

### `verification_receipt.rs`

Byte-identical contract in Light, Quantum, and Shadow, version `0x0001_0602`, with a 32-byte/64-hex SHA-256 identity and binding to the complete SHA-256 of Living Trust.

### `token_forge.rs`

SHA-256 runtime token, separated from the reasoning organs and from the supreme Living Trust pulse.

### GCL Action Authorization

The existing `GclActionAuthorizationToken` continues to authorize pre-seal/pending iZ through `action_sha256` and `law_trace_sha256`. A second incompatible token was not invented merely for the name “Untrust.”

## Code Location

```text
quantum/src/runtime_pulse.rs
quantum/src/main.rs
quantum/src/token_forge.rs
quantum/src/living_trust_contract.rs
quantum/src/lab_contracts/verification_receipt.rs
quantum/src/progressive_debatic/runtime.rs
quantum/src/progressive_debatic/types.rs

shadow-contracts/src/lib.rs

shadow/src/process_bridge.rs
shadow/src/shadow_gj_legacy.rs
shadow/src/sovereign_ffi_gate.rs
shadow/src/types.rs
shadow/src/bridge/quantum_in.rs
shadow/src/living_trust_contract.rs
shadow/src/lab_contracts/verification_receipt.rs

light/src/living_trust_contract.rs
light/src/lab_contracts/verification_receipt.rs
light/src/pd_light.rs
light/src/main.rs

VALIDATE_V162.ps1
AUDIT_V162_UNTRUST_START_TO_END.md
V162_SIMULATION_MAP.md
ESS_MAI_V1_6_2_IMPLEMENTATION_MAP.md
STATIC_AUDIT_V162.txt
```

## Version Introduced

```text
Theory:         v1.6.2 authoritative paradigm
Contract:       v1.6.2
Implementation: v1.6.2
Static audit:   v1.6.2 packaging cycle
Cargo proof:    pending VALIDATE_V162 on Windows GNU
Release status: packaged for executive verification
```

## Verification Evidence

Stimulation and audit cover:

1. a cycle with zero state/mask/ledger;
2. the exact order of the nine organs;
3. absence of one organ;
4. an extraneous or repeated organ;
5. manipulated order;
6. invalid word schema;
7. manipulated evidence words;
8. declared `action_state` that does not match replay;
9. declared mask that does not match the ledger;
10. PRO ↔ candidate mismatch;
11. SRK ↔ PIM proof-chain mismatch;
12. HPRO/APRO/MPRO mismatch;
13. NPIM mismatch;
14. HCP mismatch;
15. manipulation of Living Trust;
16. manipulation of the SHA-256 Receipt;
17. an attempt by TokenForge to contaminate action convergence;
18. continuity Trust → receipt → iZ → next_i0;
19. preservation of complete mediation by Shadow main.rs;
20. separation of PD Light, Nura, the emotional UI, and Legacy Shadow.

Evidence is preserved in:

```text
AUDIT_V162_UNTRUST_START_TO_END.md
V162_SIMULATION_MAP.md
ESS_MAI_V1_6_2_IMPLEMENTATION_MAP.md
STATIC_AUDIT_V162.txt
V1_6_2_FROM_V1_6_1.diff
ESS_MAI_V1_6_2_FILELIST.sha256
VALIDATE_V162.ps1
```

## Current Status

```text
Theory:                         COMPLETE_AS_DESIGN
Structural Untrust start:       IMPLEMENTED
Nine-organ real convergence:    IMPLEMENTED
SRK full proof citizenship:     IMPLEMENTED
Canonical evidence ledger:      IMPLEMENTED
Shadow replay:                  IMPLEMENTED
Shadow module cross-binding:    IMPLEMENTED
Exact Living Trust mask:        IMPLEMENTED
Receipt SHA-256:                IMPLEMENTED
Stateful Cargo test isolation:  IMPLEMENTED_WITH_TEST_ONLY_MUTEX
TokenForge SHA-256:             IMPLEMENTED
Trust → receipt → iZ:           IMPLEMENTED
Static syntax/contracts:        PASSED_115_OF_115
Cargo build/check/test/clippy:   PENDING_EXTERNAL_WINDOWS_GNU
Release:                        PACKAGED_FOR_EXECUTIVE_VERIFICATION
```

## Boundaries Deliberately Not Crossed

- Cargo-green was retained as a **release gate**, not a runtime bit. There is not yet a signed build-attestation contract with authority, key, format, rotation, and revocation.
- HMAC was not added because no sovereign key-management contract exists.
- Remaining FNV checksums in frame/package/legacy trace were not represented as Trust, Receipt, or Token. Their content is independently reverified by Shadow.
- Remote physical attestation beyond the existing HPRO/HCP evidence was not invented.
- TokenForge was not made a reasoning organ and was not permitted to alter the nine-organ mask.
- Layer 1/2/3 did not receive authority parallel to GCL.
- PD Light was not made a processor of PD Quantum/Spine 9.

---
# Evolution v1.6.2 → v1.6.3 — GCL Scientific Project Continuum

## Version Identity

```text
Version:        ESS-MAI v1.6.3
Baseline:       ESS-MAI v1.6.2
Theory:         GCL Scientific Project Continuum
Authority:      GCL + Shadow main supreme mediation
Scope:          user scientific/innovative projects and Novel factualization
Cargo status:   PENDING external Windows GNU validation
```

This version does not create a parallel laboratory, a second verdict, or Quantum access to Shadow storage. It closes the existing user-project organs within one constitutional flow:

```text
Light/APUPK identity
→ Shadow durable project context
→ Light
→ Quantum scientific processing
→ FinalEvidence + nine-organ Untrust
→ Shadow main verification
→ same SupremeVerdict
→ Seal 500 + Living Trust + VerificationReceipt
→ Quantum PD / output+iZ / next i0
→ PD Light / Nura / UI
```

## Philosophy

A scientific project is not declared Novel because Quantum names it as such, because Digital Lab produces a TRL, or because documentation exists. Each organ produces only its own portion of the proof:

- Light originates the project identity, APUPK trace, input SHA-256, and Seal 500;
- Shadow preserves the project identity and issues only a context witness;
- Quantum processes the hypothesis, assumptions, evidence, TRL, SRK, PIM/NPIM/MPRO, and the nine Untrust organs;
- Shadow recomputes the material, compares it with the durable project, and issues the supreme verdict;
- GCL seals the identity of the entire project entity within Living Trust;
- PD/iZ transports only the verified result.

Version law:

> The project originated in Light, the project stored in Shadow, the question processed by Quantum, the evidence adjudicated by Shadow, and the status incorporated into Living Trust must be proven to be the same object.

## Why v1.6.3 Was Required

### 1. E0425 Was Not Merely a Demonstration Error

Cargo showed that `run_integrated_lab_demo()` was compiled without `dev_harness`, while `persist_negative()` was dev-only. This revealed that Digital Lab had two mixed identities:

```text
hard-coded demonstration
versus
real scientific-project processing organ
```

v1.6.3 separates them:

- `run_lab_demo` and `run_integrated_lab_demo` remain dev-only;
- `LabSystemBridge::run_integrated` is used in the actual Quantum path for project material;
- standalone `persist_negative` remains dev-only;
- negative production is persisted only after the full Shadow cycle.

### 2. E0063 Revealed Contract Drift

`PassPackage` had evolved to include:

```text
quantum_action_state
quantum_action_mask
quantum_required_action_mask
```

but the Shadow fixture did not include the two masks. They were not filled with zero. `REQUIRED_ACTION_MASK` was used because a “strong” package must declare and prove all mandatory organs. The fixture also declares `scientific_project: None`; it tests inner Shadow, not the end-to-end Novel flow.

### 3. Positive and Negative Paths Were Asymmetric

In v1.6.2, positive Digital Lab evidence could terminate at the printed “→ PIM” line, while the negative demonstration could request separate persistence. This compromised role separation. v1.6.3 places both positive and negative material in the same final package and leaves the decision to Shadow.

### 4. Novel Was a Component, Not a Continuum

The system already contained:

- Light and Shadow APUPK;
- Digital Lab;
- Governance and Raw Cognitive Trace;
- SRK, PIM, NPIM, MPRO;
- GeniusNovel;
- `ShadowEco::classify_with_factualization`;
- `judge_supreme`;
- Living Trust.

However, no single cryptographic identity bound them together. This version creates that binding.

## Mathematical and Cryptographic Model

Let the user's project be:

```text
P = {project_id, user_id, title, content, domain, hypothesis, assumptions, docs, files}
```

### Light/APUPK Identity

```text
trace_id = fold31(project_id + user_id, title)
input_sha = SHA256(content)
V500 = ((flags & 0xFFFF) XOR 0xA5A5) = 500
```

### Shadow Context Witness

After durable WAL persistence:

```text
C_S = SHA256(
    version
    || project_id
    || user_id
    || trace_id
    || revision
    || title
    || input_sha
    || Light_V500_flags
)
```

### Quantum Evidence

```text
E_Q = SHA256(
    version
    || C_S
    || title
    || domain
    || hypothesis
    || assumptions
    || GCL_process_digest
    || TRL level/pass/confidence/reproducibility
    || lab_test_id
    || findings
    || documentation_description
    || ordered evidence files
)
```

### Shadow Project Verdict

```text
V_S = SHA256(
    version
    || project_id
    || project_status
    || C_S[32]
    || E_Q[32]
    || factualized
    || TRL
    || proof_score
    || rejection_code
)
```

### Living Trust

```text
Trust = SHA256(
    action_state
    || action_mask
    || required_action_mask
    || SupremeVerdict(Y,X,band,law)
    || system laws
    || Seal500 flags/value
    || E_Q[32]
    || V_S[32]
)
```

The project's `u64` digests are retained only as compatibility/diagnostic indices. Living Trust uses complete SHA-256 values.

## Project Contract ×3

The following byte-identical files were added:

```text
light/src/gcl_project_contract.rs
quantum/src/gcl_project_contract.rs
shadow/src/gcl_project_contract.rs
```

The contract contains:

- `GCL_PROJECT_CONTRACT_VERSION = 0x0001_0603`;
- `project_trace_id`;
- `seal_is_500`;
- `ProjectContextMaterial`;
- `ProjectEvidenceMaterial`;
- `ProjectVerdictMaterial`;
- `context_sha256`;
- `evidence_sha256`;
- `verdict_sha256_or_zero`;
- SHA-256 parser/formatter;
- canonical transport of file evidence.

The contract does not produce a verdict. It canonicalizes the material recomputed by each platform.

## Protocol v8

`shadow-contracts` was raised to protocol 8 and added:

```text
ProjectRegistrationRequestWire
ProjectRegistrationResponseWire
ProjectContextWitnessWire
LightProjectIntakeRequestWire
LightProjectIntakeResponseWire
QuantumProjectExecutionRequestWire
QuantumProjectExecutionResponseWire
ProjectEvidenceFileWire
ScientificProjectWire
```

`ScientificProjectWire` is an `Option` within `FinalEvidenceWire`. The project does not open a bypass channel; it passes through the same final package.

`ShadowVerdictWire` carries:

- project ID/status;
- context/evidence compatibility indices;
- complete context SHA-256;
- complete evidence SHA-256;
- factualized flag;
- TRL;
- proof score;
- rejection.

## Light Flow

### Project Intake

Entry point:

```text
light-platform --project-route-once REQUEST RESPONSE
```

Light validates the form, creates `ProjectUpload`, APUPK trace, and Seal 500.

### Shadow APUPK Registration

Light starts only the Shadow process:

```text
shadow_platform --project-register-once REQUEST RESPONSE
```

Shadow returns `ProjectContextWitnessWire`. Light recomputes:

- project_id;
- user_id;
- trace_id;
- Light flags;
- content SHA;
- context SHA;
- Seal 500.

A mismatching witness is rejected.

### Light→Quantum Process Boundary

Real projects do not use the legacy 2048-byte bus. Light constructs `QuantumProjectExecutionRequestWire`, binds the payload with SHA-256, and starts:

```text
quantum-platform --project-process-once REQUEST RESPONSE
```

The response must carry the SHA-256 of the request frame. This prevents stale/swapped responses.

Light does not declare Novel and does not gain access to Quantum's internal reasoning.

## Shadow APUPK Durability

The APUPK WAL was raised to version 2:

```text
shadow_apupk_v163.wal
```

The record contains:

- project/user/trace;
- initial trace;
- project title;
- Light sovereign flags;
- content;
- progress;
- timestamps;
- notes;
- revision derived from replay.

Before WAL persistence, Shadow verifies:

- non-zero trace and non-empty initial trace;
- non-empty content;
- non-empty title;
- Seal 500;
- trace formula;
- finite progress;
- unchanged ownership of project_id.

`store_durable` requires the WAL and uses `append_checked`, which performs write, flush, and fsync. RAM changes only after durability succeeds. Without this, Shadow does not issue a witness.

Because Shadow executes as a one-shot process, an inter-process lock using `create_new` was added. This protects the WAL from parallel writers. A stale lock after a crash is not removed automatically; operator intervention is required because no recovery attestation exists.

## Quantum Flow

### Project Process Entry

Quantum:

1. reads the frame;
2. recomputes the request SHA;
3. decodes the payload;
4. recomputes the payload SHA;
5. binds project ID/trace/context to `ScientificProjectInput`;
6. only then invokes `run`.

### Real Digital Lab

For the real project, Quantum invokes:

```text
LabSystemBridge::run_integrated(
    title,
    domain,
    content,
    hypothesis,
    assumptions,
    Governance,
    RawCognitiveTrace,
    trace_id
)
```

Digital Lab does not issue a verdict. It produces TRL evidence, findings, test ID, and trace.

### Same Untrust Process

The project passes through the existing organs:

```text
HPRO → PRO → NPRO → NPIM → SRK → PIM → APRO → MPRO → HCP
```

`action_state`, `action_mask`, `required_action_mask`, and the ledger remain prerequisites of Living Trust. The project does not replace this process.

### Final Package

Quantum constructs `ScientificProjectWire`, computes E_Q, and places it in `FinalEvidenceWire`. The authority of PIM/NPIM/MPRO packaging, PD/Spine, and input SHA remains unchanged.

## Shadow Verification

Shadow main performs, in order:

1. frame decoding;
2. Quantum/Light identity checks;
3. finite-value checks;
4. FinalEvidence digest/replay;
5. input SHA verification;
6. project context/evidence SHA verification;
7. APUPK durable comparison;
8. PD Continuum and Spine 9 checks;
9. nine-organ ledger replay/cross-binding;
10. PIM/NPIM/MPRO checks;
11. core ingest;
12. the same `judge_supreme`.

### APUPK Cross-Check

Before entering the core, Shadow compares:

```text
project_id
user_id
trace_id
revision
project_title
Light sovereign flags
stored content SHA-256
```

Quantum receives only the witness; it does not receive `&ShadowApupkMemory`, vault access, or Knowledge.

### Novel Factualization

Inside `judge_supreme`:

- file kind is recomputed from magic bytes;
- `NovelEvidence` is constructed;
- for `(Y,X)=(1,1)`, `ShadowEco::classify_with_factualization` is used;
- for a non-sovereign pair, factual innovation is not declared;
- status becomes a derivative of the supreme verdict and Novel evidence.

Statuses:

```text
(0,0)                         → RIGOROUS_NEGATIVE
(1,1) + factual innovation    → NOVEL_FACTUAL
any other project case        → HOLD
no project                    → NONE
```

Novel is not a parallel verdict. Its status is incorporated into `SupremeVerdict`, V_S, and Living Trust.

## Negative Knowledge

v1.6.3 clearly separates:

```text
positive/hold/Novel → zero Negative Knowledge write
rigorous negative   → mandatory persistence
```

Standalone `--negative-once` is `dev_harness` only. Production persistence occurs in `run_cycle`, after the verdict. If a verified negative is not persisted, Quantum does not release the negative PD/iZ path.

## Seal 500

Seal 500 continues to originate in Light. The project preserves the flags in APUPK, the witness, and the scientific package. Shadow verifies that:

- the flags match APUPK;
- they decode to 500;
- they enter Living Trust;
- Novel status cannot replace Seal 500.

## PD Continuum and UI

Quantum exports a v1.6.3 handoff with 45 body fields + CRC. New fields:

- project evidence SHA;
- project ID;
- project context SHA;
- status;
- factualized flag;
- TRL;
- proof score;
- rejection.

Light expects 46 sealed fields, parses the 45 body fields, and recomputes:

- ProjectVerdict SHA;
- Living Trust;
- receipt binding;
- project-status consistency.

Only then does PD Light deliver to Nura and the emotional signal. PD Light does not reprocess Digital Lab or Layers.

## Concrete Cargo Corrections

### E0425

```text
run_lab_demo                  dev_harness
run_integrated_lab_demo       dev_harness
persist_negative              dev_harness
negative imports              dev_harness
```

The real scientific path does not depend on the demonstration helper.

### E0063

The `strong_pkg` fixture carries:

```text
quantum_action_mask = REQUIRED_ACTION_MASK
quantum_required_action_mask = REQUIRED_ACTION_MASK
scientific_project = None
```

This closes the test contract without weakening the invariant.

## Principal Changes by File

### Light

- `gcl_project_contract.rs`: Project contract.
- `project_process_bridge.rs`: process transport and response binding.
- `sovereign_bridges.rs`: APUPK→Shadow→Quantum flow.
- `quantum_bridge.rs`: project payload.
- `pd_light.rs`: Novel status and Trust recomputation.
- `main.rs`: project route and wire count.

### Quantum

- `gcl_project_contract.rs`: Project contract.
- `bridge_light/mod.rs`: parse/validate project input.
- `main.rs`: project entry, Digital Lab, package, Shadow verification, PD handoff.
- `shadow_process_bridge.rs`: dev-boundary closure.

### Shadow

- `gcl_project_contract.rs`: Project contract.
- `process_bridge.rs`: APUPK/project mediation.
- `shadow_apupk.rs`: durable context store.
- `sovereign_log.rs`: checked WAL append.
- `shadow_gj_legacy.rs`: project adjudication in the supreme judge.
- `types.rs` and bridges: project evidence/result transport.
- `tests/integration.rs`: semantic fixture update.

### Contracts/UI/Version

- `shadow-contracts`: protocol v8.
- manifests and Tauri configurations: v1.6.3.
- UI receives status through PD Light, not through a Novel bypass.

## Stimulation Audit

The following scenarios were simulated:

- valid and invalid identities;
- ownership swap;
- revision replay;
- content/context/payload SHA swap;
- hypothesis modification after testing;
- magic-byte mismatch;
- missing action mask/ledger;
- SRK/PIM mismatch;
- Novel/Hold/RigorousNegative;
- Living Trust/status manipulation;
- APUPK fsync failure;
- parallel writer;
- negative persistence failure;
- old and new PD schemas.

The complete map is recorded in `V163_SIMULATION_MAP.md`.

## Evidence from This Environment

```text
Static architecture checks:      90 PASS / 0 FAIL
Rust files structurally scanned: 277
Cargo.toml parsed:               7
JSON parsed:                     3
C objects built:                 18
C compilers:                     GCC + Clang
C warnings/errors:               0
Release manifest entries:        414 / 0 mismatch / relative only
Project contract ×3:             byte-identical
Living Trust contract ×3:        byte-identical
PD Continuum ×3:                 byte-identical
PD Spine ×3:                     byte-identical
```

These are static/contractual evidence and C execution evidence. They are not Cargo evidence.

## Non-Implementation Boundaries

### Wire Authentication/Encryption

The public frame retains its existing checksum. SHA-256 protects material identity and reverification, but it is not a MAC. HMAC/encryption was not invented without:

- key authority;
- provisioning;
- storage;
- rotation;
- revocation;
- recovery.

### APUPK WAL

The WAL is durable and CRC-guarded, not MAC-protected or encrypted. A Shadow key was not invented.

### Migration

The v1.6.2 WAL is not migrated automatically. v1.6.3 uses `shadow_apupk_v163.wal`; earlier projects must be registered again. Without an authorized schema-migration contract, this is the fail-closed choice.

### APUPK Final Status

Novel/Hold/Negative resides in SupremeVerdict/LivingTrust/PD handoff. No status event was created in the APUPK WAL because no authorized contract existed for semantics, versioning, and status conflict.

### Cargo

This environment did not provide `cargo`, `rustc`, `rustfmt`, or PowerShell. `VALIDATE_V163.ps1` is the release gate. The version remains:

```text
Theory:                 COMPLETE_AS_DESIGN
Contract:               IMPLEMENTED
Light project intake:   RUNTIME_CONNECTED
Shadow context:         RUNTIME_CONNECTED_DURABLE
Quantum science:        RUNTIME_CONNECTED
Shadow Novel verdict:   RUNTIME_CONNECTED
Trust/Receipt/PD:       RUNTIME_CONNECTED
Static/C verification:  PASSED
Cargo green:            PENDING_EXTERNAL_WINDOWS_GNU
Release:                PACKAGED_FOR_EXECUTIVE_VALIDATION
```

## Evidence Artifacts

```text
CHANGELOG_v1.6.3.md
AUDIT_V163_SCIENTIFIC_PROJECT_CONTINUUM.md
V163_SIMULATION_MAP.md
ESS_MAI_V1_6_3_IMPLEMENTATION_MAP.md
STATIC_AUDIT_V163.txt
V1_6_3_FROM_V1_6_2.diff
CHANGED_FILES_V163.txt
ESS_MAI_V1_6_3_FILELIST.sha256
VALIDATE_V163.ps1
```

---
# Evolution v1.6.3 → v1.6.4 — Minimal UI, TRL within GCL, and Multi-Stage Shadow Verification

## Architectural Decision

v1.6.4 clearly separates the project repository from the reasoning/verification organs:

```text
Legacy UI
├── accepts project upload
└── reflects the system's emotional state

Light
├── validates bounded material
├── creates trace/APUPK
├── produces the GCL boundary and Seal 500
└── coordinates Shadow witness → Quantum process

Quantum
├── processes only under GCL/PD/Spine 9
├── produces TRL 0–3 evidence
└── sends the final package to Shadow main

Shadow
├── verifies GCL/Spine/identity
├── verifies SHA-256 and the Light Seal
├── verifies evidence-file magic bytes
├── verifies TRL support
├── verifies Y and X
└── only after (Y=1,X=1) may factualize TRL4
```

No TRL authority was created. TRL remains evidence within the same GCL process and does not replace Shadow's supreme verdict.

## Old UI Minimal Boundary

**Theory name:** Old UI Minimal Upload and Emotion Boundary  
**Philosophy:** the surface must not assume constitutional responsibility from Light, Quantum, or Shadow.  
**Purpose:** to allow the user to submit a project and observe the emotional state without creating a bypass.  
**Mathematical model:** `UI(material) → Light intake`; `System state → EmotionalCommand → UI`.  
**Runtime flow:** `upload_project → light-platform --project-route-once`; `reflect_system_emotion` only reflects actual output.  
**Contracts:** `LightProjectIntakeRequestWire` does not contain `user_id`, `project_id`, timestamp, `contract_id`, `lgc_seal`, TRL, or verdict.  
**Code location:** `light/ui/src-tauri/src/main.rs`, `light/ui/src/main.js`, `light/ui/src/index.html`.  
**Version introduced:** v1.6.4.  
**Verification evidence:** static guards in `VALIDATE_V164.ps1`; absence of direct Shadow/Quantum calls in the UI.  
**Current status:** `RUNTIME_CONNECTED_PENDING_CARGO`.

### Change from v1.6.3

The placeholder commands were removed:

```text
explore_input
get_output
upload_knowledge_dialog
ready_for_shadow = true
```

The legacy UI now exposes only:

```text
upload_project
reflect_system_emotion
```

It does not create `user_id`, `project_id`, timestamp, sovereign trace, GCL contract, Seal 500, TRL, or verdict. Identity, time, trace, and the actual witness originate in Light/APUPK/Shadow.

## Light-Owned Project Intake

**Theory name:** Light-Owned Scientific Project Intake  
**Philosophy:** Light coordinates and anchors; the UI does not declare authority.  
**Purpose:** to create the single path from upload to GCL.  
**Mathematical model:** `U_project + Light(APUPK,500) → ProjectContextWitness → Quantum(i₀_project)`.  
**Runtime flow:** `--project-route-once → APUPK prepare → Shadow register → witness SHA check → Quantum process`.  
**Contracts:** `LightProjectIntakeRequestWire`, `ProjectRegistrationRequestWire`, `ProjectContextWitnessWire`, `QuantumProjectExecutionRequestWire`.  
**Code location:** `light/src/project_process_bridge.rs`, `light/src/sovereign_bridges.rs`.  
**Version introduced:** v1.6.3; authority boundary corrected in v1.6.4.  
**Verification evidence:** UI authority fields removed; Light derives `GCL:SCIENTIFIC_PROJECT:V164` and seal material from the APUPK witness.  
**Current status:** `RUNTIME_CONNECTED_PENDING_CARGO`.

## GCL TRL Separation

**Theory name:** GCL-Bounded TRL Separation  
**Philosophy:** TRL measures evidence maturity; it is not law, verdict, or a parallel flow.  
**Purpose:** to keep Quantum reasoning separated from Shadow factualization.  
**Mathematical model:**

```text
Quantum evidence: TRL ∈ {0,1,2,3}
Shadow factualization: TRL4 ⇔ GCL_bound ∧ evidence_verified ∧ Y=1 ∧ X=1 ∧ NovelProof
```

**Runtime flow:** `Quantum Digital Lab → ScientificProjectWire(TRL≤3) → ShadowLab → GeniusNovel → optional TRL4`.  
**Contracts:** `QUANTUM_MAX_TRL = 3`, `SHADOW_FACTUAL_TRL = 4`, protocol v9.  
**Code location:** `shadow-contracts/src/lib.rs`, `shadow/src/shadow_lab.rs`, `shadow/src/shadow_genius_novel.rs`, `shadow/src/shadow_eco.rs`.  
**Version introduced:** v1.6.4 as an explicit runtime boundary.  
**Verification evidence:** inbound TRL4 is rejected by the wire shape and by Shadow's GCL stage; tests were added.  
**Current status:** `IMPLEMENTED_PENDING_CARGO`.

## Shadow Multi-Stage Project Verification

**Theory name:** Shadow Multi-Stage Project Verification  
**Philosophy:** Shadow does not trust a ready-made project verdict; it verifies each boundary sequentially.  
**Purpose:** to avoid mixing identity, evidence, TRL, and factualization into one opaque decision.  
**Mathematical model:**

```text
S1 = GCL_identity ∧ Spine_complete ∧ Seal500 ∧ SHA_canonical
S2 = declared_file_kind ≡ detected_magic_kind
S3 = ShadowLab(TRL≤3, confidence, reproducibility)
S4 = (Y,X)=(1,1) ∧ GeniusNovel(real documentation)
TRL4 = S1 ∧ S2 ∧ S3 ∧ S4
```

**Runtime flow:** `process_bridge validation → judge_supreme → verify_project_gcl_stage → verify_project_file_kinds → ShadowLab → GeniusNovel`.  
**Contracts:** same `ScientificProjectContext`, same PD GCL digest, same Spine completion, same witness, and same project evidence SHA.  
**Code location:** `shadow/src/process_bridge.rs`, `shadow/src/shadow_gj_legacy.rs`.  
**Version introduced:** existing stages consolidated and explicitly separated in v1.6.4.  
**Verification evidence:** GCL mismatch, zero Spine, invalid seal, noncanonical SHA, and inbound TRL4 fail closed.  
**Current status:** `IMPLEMENTED_PENDING_CARGO`.

## Compilation Corrections from the v1.6.3 Audit

### PD Handoff

The order of the 45-field formatter in Quantum and in the PD Light test was corrected:

```text
project_evidence_sha256 → {}
project_id              → {:016x}
project_context_sha256  → {}
```

This closes `E0277 String: LowerHex` without altering the semantic parser schema.

### Shadow Fixtures

`shadow/src/bridge/mod.rs` and `shadow/src/bridge/shadow_callable.rs` declare:

```rust
scientific_project: None
```

This closes `E0063` without inventing a project within legacy fixtures.

## Domains and Protocol

- `shadow-contracts::PROTOCOL_VERSION = 9`.
- `ESS_MAI_GCL_PROJECT_CONTEXT_V164`.
- `ESS_MAI_GCL_SCIENTIFIC_PROJECT_EVIDENCE_V164`.
- `ESS_MAI_GCL_SCIENTIFIC_PROJECT_VERDICT_V164`.
- `GCL_LIVING_TRUST_V164`.
- `GCL_LIVING_TRUST_TO_IZ_V164`.
- `ESS_MAI_FINAL_EVIDENCE_V164`.
- `PD_LIGHT_IZ_UI_CONTINUITY_V164`.

The `gcl_project_contract.rs` and `living_trust_contract.rs` contracts remain byte-identical in Light, Quantum, and Shadow.

## Package Evidence

v1.6.4 artifacts:

```text
CHANGELOG_v1.6.4.md
AUDIT_V164_UI_LIGHT_GCL_TRL_SHADOW.md
ESS_MAI_V1_6_4_IMPLEMENTATION_MAP.md
V164_SIMULATION_MAP.md
STATIC_AUDIT_V164.txt
CHANGED_FILES_V164.txt
V1_6_4_FROM_V1_6_3.diff
ESS_MAI_V1_6_4_FILELIST.sha256
VALIDATE_V164.ps1
```

## Version Status

```text
Old UI role separation:       IMPLEMENTED
UI → Light project route:     RUNTIME_CONNECTED
Light-owned GCL boundary:     IMPLEMENTED
Quantum TRL bound ≤ 3:        CONTRACT_ENFORCED
Shadow multi-stage checks:    IMPLEMENTED
Shadow-only TRL4:             CONTRACT_ENFORCED
PD 45-field format repair:    IMPLEMENTED
Shadow fixture repair:        IMPLEMENTED
Static verification:          PASSED
Cargo green:                  PENDING_EXTERNAL_WINDOWS_GNU
Release:                      PACKAGED_FOR_EXECUTIVE_VALIDATION
```

This environment did not provide `cargo`, `rustc`, or PowerShell. Accordingly, v1.6.4 is not marked `VERIFIED` or `RELEASED` by Cargo without execution of `VALIDATE_V164.ps1` on Windows GNU / Rust 1.96.0.

---
# Evolution v1.6.4 → v1.6.5 — Project Workspace in Quantum and the Legacy Path

## Architectural Decision

v1.6.5 does not turn Quantum into a repository and does not create parallel memory. It adds only a **bounded router for the Project domain**, after the project has been registered in APUPK by Light through Shadow main.

```text
Default project path
UI upload
→ Light --project-route-once
→ APUPK + Shadow ProjectContextWitness
→ Quantum --project-workspace-once
→ Storage / Conversation / Storage+Conversation orientation

Legacy scientific path
Light --project-route-legacy-once
→ Quantum --project-process-once
→ GCL / PD / Spine 9 / TRL 0–3 / Shadow verification
```

Normal user input, Quantum's stdin flow, PD Continuum, and the other bridges are not redirected into Project Workspace.

## Quantum Project Workspace Orientation

**Theory name:** Quantum Project Workspace Orientation  
**Philosophy:** a project requires a clear orientation toward repository storage and conversation, but the orientation must not assume the authority of GCL, tokens, or Shadow.  
**Purpose:** to separate projects from normal inputs and from complete scientific processing.  
**Mathematical model:**

```text
P_valid = APUPK_bound ∧ project_id≠0 ∧ trace_id≠0 ∧ SHA_context ∧ SHA_request
Route(P) ∈ {STORAGE, CONVERSATION, STORAGE∧CONVERSATION}
Authority(Route(P)) = ∅
TokenMutation(Route(P)) = 0
```

**Runtime flow:** `--project-workspace-once → validate execution envelope → deserialize ScientificProjectInput → orient → SHA-256 record identities`.  
**Contracts:** uses `QuantumProjectExecutionRequestWire` only as the existing envelope; the new response is `ESSMAI_Q_PROJECT_WORKSPACE_V165`.  
**Code location:** `quantum/src/project_workspace_router.rs`, `quantum/src/main.rs`.  
**Version introduced:** v1.6.5.  
**Verification evidence:** the module does not import `LgcToken`, `LgcGate`, `CapHandle`, `token_forge`, `ForgeToken`, or `SEAL_*`; the workspace function does not invoke `run`.  
**Current status:** `IMPLEMENTED_PENDING_CARGO`.

### Three Orientations

```text
PROJECT_STORAGE
PROJECT_CONVERSATION
PROJECT_STORAGE_AND_CONVERSATION
```

The explicit domains `project-storage`, `project-chat`, and `project-workspace` select the path. When the domain is not explicit, the form of the material and the presence of a conversation turn determine the orientation deterministically.

### Record Identities

Quantum produces:

```text
workspace_sha256
material_sha256
conversation_turn_sha256
```

These are domain-separated record identities and **are not tokens, receipts, or verdicts**. The response declares:

```text
authority=NONE
token_policy=UNCHANGED
legacy_route=--project-process-once
```

## Light Dual Project Route

**Theory name:** Light Dual Project Route  
**Philosophy:** Light maintains a single APUPK/GCL boundary and then selects the destination explicitly.  
**Purpose:** to avoid duplicating or altering Seal 500 between the new and legacy paths.  
**Runtime flow:**

```text
prepare_project_handoff_under_gcl
├── route_project_workspace_under_gcl
└── route_scientific_project_under_gcl
```

**Contracts:** both paths use the same `ProjectContextWitnessWire`, `content_sha256`, `GCL:SCIENTIFIC_PROJECT:V164`, and `light_sovereign_flags`.  
**Code location:** `light/src/sovereign_bridges.rs`, `light/src/project_process_bridge.rs`.  
**Version introduced:** v1.6.5.  
**Verification evidence:** default flag `--project-route-once`; legacy flag `--project-route-legacy-once`; the Workspace response is reverified field by field.  
**Current status:** `RUNTIME_CONNECTED_PENDING_CARGO`.

## Why GCL and Tokens Remain V164

v1.6.5 does not alter the project's constitutional material, verdict, Living Trust, or iZ. Therefore, the following were not changed:

```text
GCL:SCIENTIFIC_PROJECT:V164
ESS_MAI_GCL_PROJECT_CONTEXT_V164
ESS_MAI_GCL_SCIENTIFIC_PROJECT_EVIDENCE_V164
ESS_MAI_GCL_SCIENTIFIC_PROJECT_VERDICT_V164
GCL_LIVING_TRUST_V164
GCL_LIVING_TRUST_TO_IZ_V164
shadow-contracts PROTOCOL_VERSION = 9
```

Changing these domains solely for workspace orientation would create new Living Trust/receipt identities and would endanger token continuity. v1.6.5 adds only:

```text
ESS_MAI_QUANTUM_PROJECT_WORKSPACE_V165
```

This domain does not enter the GCL token, ForgeToken, capability gate, receipt, or SupremeVerdict.

## Legacy Path

The v1.6.4 path was not removed:

```text
light-platform --project-route-legacy-once REQUEST RESPONSE
quantum-platform --project-process-once REQUEST RESPONSE
```

It continues complete scientific processing and remains separated from the Workspace gateway. The `run_project_process_once` function preserves its previous behavior.

## Shadow and Repository Ownership

Shadow remains the owner of persistent APUPK storage. The default order is:

```text
Light identity + Seal 500
→ Shadow project-register-once
→ ProjectContextWitness
→ Quantum workspace orientation
```

Quantum does not write a new vault. This preserves a single repository and prevents fragmentation of project identity.

## v1.6.5 Evidence

```text
CHANGELOG_v1.6.5.md
AUDIT_V165_QUANTUM_PROJECT_WORKSPACE.md
ESS_MAI_V1_6_5_IMPLEMENTATION_MAP.md
V165_SIMULATION_MAP.md
STATIC_AUDIT_V165.txt
CHANGED_FILES_V165.txt
V1_6_5_FROM_V1_6_4.diff
ESS_MAI_V1_6_5_FILELIST.sha256
VALIDATE_V165.ps1
```

## Version Status

```text
Project-only Quantum split:        IMPLEMENTED
Default storage/chat orientation:  RUNTIME_CONNECTED
Legacy scientific route:          PRESERVED
Normal Quantum stdin route:        UNCHANGED
Shadow APUPK persistence:          UNCHANGED
LGC/Forge/capability token files:  BYTE_IDENTICAL_WITH_V164
GCL/Living Trust domains:          UNCHANGED_V164
Rust syntax tree audit:            PASSED
Cargo/clippy/fmt:                  PENDING_EXTERNAL_WINDOWS_GNU
Release:                           PACKAGED_FOR_EXECUTIVE_VALIDATION
```

This environment did not provide `cargo`, `rustc`, or PowerShell. `VALIDATE_V165.ps1` is the executive gate that must establish Cargo-green status and zero warning debt on Windows GNU / Rust 1.96.0.

---

# ESS-MAI v1.6.8 — Light Legacy GCL Runtime Integration

## Scope

v1.6.8 focuses only on the Light Platform path that coordinates the primitive input under GCL and transfers the same identity toward Quantum and Shadow:

```text
i₀
→ Primitive Trace
→ SHA-256
→ Primitive Anchor
→ GCL Coordination Directive
→ LGC Legacy Algorithm
→ Primitive Split (Xi, Yi)
→ LegalGoCrypt V2
→ KODUNIK + Legacy Receipt
→ durable PA/receipt/commit evidence
→ Quantum handoff
```

GCL remains the sole governing authority. LGC Algorithm and LegalGoCrypt operate as the executable Legacy algorithm under GCL. Light does not reason and does not issue verdicts. Quantum and Shadow authority boundaries were not changed.

## Codex Trace Findings Corrected

The inherited Codex patch was incomplete. The following defects were proven and corrected:

```text
1. LegacyLgcAlgorithm existed without a production caller.
2. LightCoordinator still called EvolveTrace and primitive_split directly.
3. AlnurKarinaAthar API changed, while callers still used the removed symbolic API.
4. LightResponse gained a legacy field, but constructors did not initialize it.
5. SoftwareContract still generated a second KODUNIK.
6. main.rs rebuilt LAW0 and EvolveTrace in parallel paths.
7. PA export remained passive/fail-open.
8. Handoff evidence was constructed from literal true values.
9. The write helper accepted PathBuf while runtime passed String paths.
10. KODUNIK HMAC did not bind the complete emitted code.
11. receipt raw_len was mutable metadata outside the receipt digest.
12. Existing Alnur state-machine tests had been removed.
```

## Implemented Runtime Changes

### Single Legacy Authority

`LightCoordinator::receive` now calls one production entry point:

```text
LightCoordinator
→ LegacyLgcAlgorithm::coordinate_under_gcl
→ primitive_split
```

The direct parallel split/Evolve/Alnur path was removed from the coordinator.

### One Primitive Identity

The same `TraceInfo`, input SHA-256, Primitive Anchor, Xi/Yi split, EvolveTrace, KODUNIK and Legacy receipt are carried through the Light cycle. `main.rs` no longer creates a second EvolveTrace or a second LAW0 calculation.

### One KODUNIK

`SoftwareContract::create_bound` reuses the KODUNIK and binding SHA-256 produced by the Legacy receipt. A second contract KODUNIK is not created for the same cycle.

### LegalGoCrypt V2

LegalGoCrypt V2 binds:

```text
trace identity
input SHA-256
Primitive Anchor
Xi/Yi lengths and digests
split digest
LAW0 before/after
GCL law seal
system-laws seal
Legacy V1 digest
contract lineage
bridge lineage
KODUNIK attestation
integrity SHA-256
```

The original LegalGoCrypt V1 generator/parser remains as a compatibility component and as the embedded Legacy payload of V2. It was not rebuilt from zero.

### Evidence-Bound AlnurKarinaAthar

The Light state machine now advances only through typed evidence:

```text
TraceEvidence
→ PrimitiveAnchorEvidence
→ PrimitiveSplitEvidence
→ CoordinateCollapseEvidence
→ HandoffEvidence
```

A durable handoff requires receipt, commit and PA publication evidence, synchronization evidence and KODUNIK verification.

### Fail-Closed Durable Handoff

The production export path now:

```text
verify receipt
→ seal receipt wire
→ sync receipt
→ seal and sync commit
→ publish and sync PA last
→ create HandoffEvidence from actual write results
→ complete Alnur handoff
→ open Quantum dispatch gate
```

A mismatch or I/O failure blocks that cycle before Quantum dispatch. PA is published last so Quantum/Shadow cannot see the new session anchor before the companion Legacy evidence has been synchronized.

### KODUNIK Hardening

The HMAC now binds the complete KODUNIK code in addition to SHA-256, counter and timestamp. Changing the suffix or any emitted-code byte invalidates the attestation.

### Receipt Hardening

The receipt digest now binds `raw_len` together with the LegalGoCrypt envelope and Xi/Yi wire. `raw_len` is no longer unauthenticated descriptive metadata.

## Tests Added or Restored

The v1.6.8 Light code includes tests for:

```text
lawful evidence-bound Alnur sequence
illegal order and duplicate events
false durable handoff evidence
single Legacy authority and single KODUNIK
no dispatch before durable handoff
dispatch after evidence completion
empty primitive material fail-closed
valid GCL phase requirement
Legacy receipt and split continuity
raw_len receipt tampering
KODUNIK full-code tampering
durable file writes and sync
mismatched split writes nothing
```

## Ghost-Flow Decisions

The two incomplete Codex planning documents were removed from the final package. The following existing APIs were retained because they have an explicit compatibility or diagnostic role:

```text
LegalGoCrypt V1 generator/parser/validator
SoftwareContract::create legacy API
EvolveTrace::from_trace compatibility/test bridge
EvolveTrace::from_trace_with_input compatibility/test API
LgcAlgorithm::stats diagnostics
LightStatus::EvolveFailed compatibility status
```

They must not be classified as removable solely from a `dead_code` warning. No Quantum, Shadow, UI, shared wire contract, CI gate or MD5-locked file was changed.

## Static Verification Performed in This Build Environment

```text
All Rust files parsed with tree-sitter-rust: PASS
Rust files with syntax ERROR/missing nodes: 0
Single production caller for coordinate_under_gcl: PASS
Single production caller path for primitive_split: PASS
All LightResponse constructors initialize legacy: PASS
Removed old coordinate_collapse / zero-argument hand_off calls: PASS
Removed duplicate EvolveTrace/LAW0 path from main: PASS
Forbidden areas changed (Quantum/Shadow/UI/installer): 0
```

This build environment does not contain `cargo` or `rustc`. Therefore, Cargo compilation, tests, Clippy, feature builds and runtime execution are not claimed as completed here.

## Required Executive Validation

Run from the `ess_mai` directory on the Windows GNU/Rust machine:

```powershell
cargo check -p light-platform --all-targets
cargo test -p light-platform --no-fail-fast
cargo check --workspace --all-targets
cargo test --workspace --no-fail-fast
cargo build --workspace
cargo build --release --workspace
```

Then separately record the known global acceptance gates:

```text
cargo clippy --workspace --all-targets -- -D warnings
Light c_kernel release
Quantum hw_kernel release
UI/Tauri build
installer/ci_gate.sh
```

Do not classify a pre-existing global failure as a new v1.6.8 Light regression without comparing it to the v1.6.7 baseline.

## Known Scope Limitations

```text
The companion Legacy receipt and commit files are produced and used by the
Light dispatch gate. Quantum and Shadow were intentionally not modified in
this version, so they continue to consume the existing PA/bridge contracts.

The KODUNIK registry is process-local. The V2 envelope contains a self-contained
HMAC attestation, but live `verify_runtime` also requires the current-process
registry entry. Cross-restart receipt replay is not claimed in v1.6.8.

Append-only receipt/commit records from a cycle that fails before final PA
publication remain audit artifacts. They do not open the Light dispatch gate.
```

## Progress Status

```text
Targeted Light GCL/LGC integration design:       100%
Production call-graph integration:                100% statically connected
Legacy receipt/KODUNIK/PA fail-closed path:       100% implemented in source
Targeted test coverage written:                    90%
Static syntax and flow verification:              100% passed
Cargo compile/test proof:                           0% in this environment / pending laptop execution
Global v1.6.8 release acceptance:                  60% pending Cargo, Clippy, CI and UI gates
```

The source-level Light intervention is complete for executive Cargo validation. v1.6.8 must not be called release-green until the commands above pass and the existing v1.6.7 global gate conflicts are resolved authoritatively.

# ESS-MAI v1.6.8 — Quantum HPRO/HCP_PRO Real Hardware Runtime

**Data e ndërhyrjes:** 2026-07-17  
**Projekti aktiv:** `v1.6.8_ess_mai`  
**Baseline historik Cargo:** `v1.6.7_ess_mai`  
**Platforma e fokusit:** Quantum  
**Statusi i këtij seksioni:** `STATICALLY_INTEGRATED_PENDING_CARGO_AND_REAL_DEVICE_SMOKE`

## Kufiri autoritativ

Kjo ndërhyrje nuk ndryshon autoritetet kushtetuese:

```text
GCL    = autoriteti i vetëm qeverisës
Light  = Coordination Collapse
Quantum = Elimination Collapse
Shadow = Verification Collapse + verdict
```

HPRO dhe HCP_PRO nuk japin verdict, nuk ndryshojnë GCL, nuk ndryshojnë `i₀`, SHA-256 lineage, PD pre-seal/closure, Spine 9 ose rendin `Layer 1 → Layer 2 → Layer 3`. Ndryshimi kufizohet te qeverisja fizike e workload-it Quantum.

## Baseline Cargo i v1.6.7 që udhëhoqi ndërhyrjen

Gjurmët e ruajtura të v1.6.7 provonin:

```text
cargo check --workspace --all-targets      PASS
Quantum default/release compile            PASS
Quantum hw_kernel build                    PASS
real test suite                            PARTIALLY RED
clippy -D warnings                         RED
CI gate                                    RED
```

Warning-et e provuara në Quantum përfshinin importe të papërdorura në HPIM/HCP_PRO/hw_real, `Ev` në binary, `mut ucl_step`, `gati_env` dhe helper-in privat `byte_at`. Kjo ndërhyrje i trajtoi vetëm warning-et me provë të pastër dhe nuk përdori `cargo fix`.

Dështimet historike të SHA fixtures, numërimit të vulave, `first_guardian`, gate-eve PA/pulse dhe CI-së nuk u maskuan ose ndryshuan, sepse nuk janë shkaku i HPRO/HCP_PRO.

## Gjendja para ndërhyrjes

```text
ResourceTerritory / hw_real
→ snapshot ose fallback
→ LimHwRealBridge
→ HPRO bounds + StabilityPlan
→ hardware_released = true (simbolik)
→ PRO zgjedh deri n_parallel operatorë, por i ekzekuton serialisht
→ HCP_PRO prodhon PushDeeper/Hold/PullBack
→ direktiva shkon në trace/Shadow, por nuk ndryshon lease-in
```

Problemet e provuara:

1. `DeviceSnapshot` i HPRO në rrjedhën kryesore përmbante konstante CPU/energji/bateri/temperaturë.
2. HPRO deklaronte release me literal, përpara se workload-i ta konsumonte hardware-in.
3. `HwManager::adjust()` nuk ishte production path dhe përdorte kufi artificial cores.
4. `ram_multiplier()` nuk kishte efekt production.
5. `n_parallel` kufizonte zgjedhjen, jo ekzekutimin paralel real.
6. HCP_PRO nuk ndryshonte RAM budget, worker count ose operator admission.
7. HCP L2/L3 mund të krijonin vendime/tokenë të ndarë nga aplikimi real.
8. Fallback-u i cooling-ut mund të dukej si veprim fizik i suksesshëm.
9. Snapshot real në Windows mund të ngatërrohej me mungesën e sysfs cooling.

## Plani i zbatuar

```text
1. Një matje kanonike për cikël.
2. MeasurementState i tipizuar.
3. Lease real logjik i workload-it me 10% RAM floor dhe core reserve.
4. HPRO krijon lease aktiv, jo vetëm bounds.
5. HCP_PRO vendos dhe aplikon lease para PRO-së.
6. PRO konsumon lease-in për fragment working set, operator admission dhe workers.
7. HCP L2 konsumon të njëjtin application/token dhe mund të mbyllë PRO.
8. HCP L3 respekton Layer 2 dhe kufizon familjet para MPRO.
9. Release receipt prodhohet pas join/drop të workload-it.
10. Fallback-u etiketohet dhe nuk pretendon cooling fizik.
```

## Rrjedha production pas ndërhyrjes

```text
main.rs
→ HardwareRuntimeContext::capture
→ ResourceTerritory::snapshot (një herë)
→ MeasurementState / MeasurementSource
→ DeviceSnapshot kanonik
→ HproEngine::acquire
→ HwManager::govern
→ active HwLease
→ HcpPro::orchestrate_and_apply
→ HwManager::adjust me RAM/cores reale
→ lease_after + operator_admission_mask
→ fragment working-set admission
→ QuantumLayer3Flow::from_hpro
→ Spine 9 / Abyssal Probe
→ HCP_PRO L2 final order
→ LimHwRealBridge::finalize_with_runtime
→ ProEngine::activate
→ bounded scoped workers + deterministic merge
→ NPRO/SRK/PIM/APRO
→ HCP_PRO L3 family gate, i mbyllur nga Layer 2
→ MPRO
→ Shadow evidence/verification flow i pandryshuar
→ drop working buffers
→ HardwareRuntimeContext::release
→ HardwareReleaseReceipt
```

## Kontrolli real i workload-it

### RAM

`HwLease::admit_fragment_prefix()` kufizon prefix-in e fragmenteve përpara clone/vectorization. Ky është kontroll i working set-it të ESS-MAI dhe jo pretendim për rezervim të fortë RAM në nivel OS.

### CPU/paralelizëm

PRO përdor `std::thread::scope` vetëm për llogaritjen e pastër të operatorëve të pranuar. `KnowledgeVault` dhe NK post-filter mbeten sekuenciale. Rezultatet bashkohen sipas indeksit kanonik të operatorit.

### HCP_PRO

```text
PushDeeper → kërkon RAM/parallel më të lartë, por kalon sërish në HwManager
Hold       → riverifikon grantën ndaj snapshot-it aktual
PullBack   → ul RAM/parallel dhe ngushton operator admission
unsealed   → operator admission = 0 (fail-closed)
```

Një ngushtim i operator admission ka përparësi ndaj një rritjeje nominale të lease-it: sistemi nuk raporton `PushDeeper` nëse puna reale është ngushtuar.

### Layer 2 dhe Layer 3

Layer 2 përdor të njëjtin `HcpApplicationResult`; nuk mint-on token të dytë. Një Abyssal Probe që jep `Stop` mbyll operator admission para PRO-së.

Layer 3 përdor të njëjtin application/token dhe nuk mund të rihapë një rrjedhë që Layer 2 e mbylli. Familjet PRO/NPRO/HPRO/APRO kufizohen nga lease parallel dhe ligjet para hyrjes në MPRO.

### Release

`HardwareReleaseReceipt` derivon nga lease-i aktiv, worker-at e bashkuar dhe buffer-at e regjistruar në cikël. Rruga e suksesit bën release eksplicit; early return përdor pronësinë Rust dhe `Drop` të context-it.

## Matja reale kundrejt fallback-ut

```text
Measured        = snapshot real + sensor real
Degraded        = snapshot real + termik matematik/paplotë
NominalFallback = hw_kernel joaktiv; vlera të deklaruara nominale
Unavailable     = snapshot nuk u mor; kufij konservativë
```

`kernel_hw_available()` tani tregon provider-in real të snapshot-it. `kernel_cooling_available()` është aftësi e veçantë. Mungesa e sysfs/fan control në Windows nuk e zhvlerëson snapshot-in real.

Fallback-u `kernel_colddown_execute()` nuk prodhon më fan/cooling/throttle të rremë dhe kthen kod jo-suksesi.

## Stimulimi statik/matematik

Skenarët e simuluar mbi formulat e implementuara:

```text
MEASURED + DEEP + kapacitet
→ PushDeeper
→ RAM/parallel rriten vetëm brenda floor/core reserve

NOMINAL_FALLBACK + DEEP
→ PullBack konservativ
→ pa thellim mbi hardware të pamatur

MEASURED + SHALLOW + normal
→ Hold
→ lease riverifikohet

MEASURED + presion/Hot
→ PullBack
→ RAM/parallel/operator admission ngushtohen

Layer 2 = Stop
→ Layer 3 sealed=false për family admission
→ asnjë familje nuk rihapet
```

Ky stimulim nuk është provë sensorike. Prova reale e Windows/Linux kërkon ekzekutim me `hw_kernel` në pajisje.

## Skedarët Quantum të ndryshuar

Ndryshimet kryesore ekzekutive:

```text
quantum/src/hw_real/runtime.rs              NEW
quantum/src/hw_real/hw_manager.rs
quantum/src/hw_real/ffi.rs
quantum/src/hw_real/thermal.rs
quantum/src/hw_real/mod.rs
quantum/src/layer3/hpro.rs
quantum/src/hcp_pro.rs
quantum/src/lim_hw_real_bridge.rs
quantum/src/pro.rs
quantum/src/pro_types.rs
quantum/src/layer2/hcp_pro_l2.rs
quantum/src/layer3/hcp_pro_l3.rs
quantum/src/layer3/hpim.rs
quantum/src/layer3/layer3_flow.rs
quantum/src/quantum_spine.rs
quantum/src/phase9_integration.rs
quantum/src/main.rs
```

Ndryshime të tjera janë vetëm sinkronizim i `ProResult` test fixtures ose heqje e warning-eve të provuara Quantum.

Nuk u ndryshuan:

```text
light/**
shadow/**
shadow-contracts/**
ui/**
installer/ci_gate.sh
PD contracts
wire contracts
GCL domains
SHA-256 lineage
version identifiers
```

## Verifikimi i kryer në këtë ambient

```text
Rust files parsed                         279
Tree-sitter Rust syntax error files       0
Known modified-call arity mismatches      0
Known struct literal missing fields       0
Classic if/else additions in patch        0
Production hardware_released literal      0
Artificial u32::MAX.min(64) production    0
HPRO/HCP production caller chain          PRESENT
Prohibited platform changes               0
```

Ky ambient nuk përmban `cargo` ose `rustc`; prandaj nuk deklarohen:

```text
cargo check green
cargo test green
hw_kernel runtime green
release acceptance green
```

## Validimi ekzekutiv i detyrueshëm

Në Windows GNU, nga rrënja e v1.6.8:

```powershell
cargo check -p quantum-platform --all-targets
cargo test -p quantum-platform --no-fail-fast
cargo check -p quantum-platform --features hw_kernel
cargo test -p quantum-platform --features hw_kernel --no-fail-fast
cargo build --release -p quantum-platform --features hw_kernel
cargo check --workspace --all-targets
cargo test --workspace --no-fail-fast
```

Pastaj duhet një smoke test real që regjistron:

```text
measurement_state
measurement_source
RAM total/free
CPU load
cores
energy/battery
thermal source/state
initial lease
applied HCP directive
lease before/after
PRO workers/fragments/working-set
L2 final order
L3 active families
release receipt
```

## Kufizimet e mbetura

1. Lease-i qeveris workload-in e ESS-MAI; nuk është rezervim OS-level, CPU affinity ose hard memory lock.
2. Temperatura Windows mund të jetë matematike dhe etiketohet `Degraded`.
3. Cooling fizik varet nga backend/capabilities; nuk pretendohet universal.
4. HCP L3 kufizon hyrjen e familjeve në MPRO; disa llogaritje NPRO/APRO prodhohen më herët në rrjedhën ekzistuese.
5. Worker panic bashkohet dhe output-i i tij refuzohet; API-ja ekzistuese `ProResult` nuk ekspozon ende error të tipizuar për worker panic.
6. Cargo dhe runtime real nuk u ekzekutuan në këtë ambient.

## Progresi i v1.6.8

Milestone-et e këtij scope-i:

```text
1. Baseline/call graph                         COMPLETE
2. Gjurmët Cargo v1.6.7                       COMPLETE
3. Gjurmët e Codex dhe ghost-flow analysis    COMPLETE
4. Snapshot kanonik                           IMPLEMENTED
5. Measurement-state separation               IMPLEMENTED
6. HPRO active lease                          IMPLEMENTED
7. HCP_PRO real application                   IMPLEMENTED
8. PRO lease/working-set/parallel consumption IMPLEMENTED
9. HCP L2/L3 continuity                       IMPLEMENTED
10. Release receipt                           IMPLEMENTED
11. Targeted tests authored                   IMPLEMENTED_NOT_EXECUTED
12. ess_mai.md documentation                  COMPLETE
13. Cargo validation                          PENDING_EXTERNAL
14. Real hw_kernel smoke                      PENDING_EXTERNAL
```

**Progresi i implementimit të scope-it HPRO/HCP_PRO:** `12/14 milestone = 85.7%`.  
Ky numër nuk është release readiness. Dy milestone-et e mbetura janë prova ekzekutive që mund të zbulojnë nevojë për patch shtesë.

**Verdikti aktual i v1.6.8:**

```text
LIGHT LEGACY FLOW          STATICALLY INTEGRATED, CARGO PENDING
QUANTUM HPRO/HCP RUNTIME   STATICALLY INTEGRATED, CARGO PENDING
REAL HARDWARE CONTROL      IMPLEMENTED AT WORKLOAD LEVEL, DEVICE SMOKE PENDING
FULL RELEASE ACCEPTANCE    NOT YET GREEN
```

---

# v1.6.9 — Shadow Multi-Verification Runtime dhe Main-Mediated Authority

## Scope-i autoritativ

v1.6.9 vazhdon drejtpërdrejt mbi v1.6.8 dhe ndryshon vetëm Shadow Platform. Light, Quantum, Shadow Contracts, UI, installer-i, PD, Spine 9, rendi Layer 1→2→3, i₀, SHA-256 lineage dhe autoriteti final i ShadowGjLegacy nuk ndryshojnë.

Qëllimi është që Shadow të mos gjykojë vetëm një herë ose të mbajë gjykatat e pasura si helper pa caller. Gjendja reale tani kalon në një rrjedhë të vetme shumë-verifikimi, sistematizohet në precedent/memory/wisdom dhe pastaj futet si portë e detyrueshme para Judiciary.

## Gjurmët e provuara para ndërhyrjes

```text
main.rs -> process_bridge -> ingest_bridged -> ingest_unsealed -> run_pipeline
```

Ishte rrjedha production, ndërsa:

```text
ShadowSpine
-> NightWatch
-> FinalVerdictCourt (6 courts)
-> VerificationMemoryIndex
-> WisdomWarehouse
```

kishte vetëm caller-a test/helper. Judiciary mund të mesatarizonte gjendjet e nodave pa kërkuar që kjo rrjedhë multi-verifikimi të ekzistonte. HCP heart regjistrohej në një helper split pa caller production, ndërsa adapterët `shadow_out` dhe `shadow_callable` kompiloheshin edhe pse nuk ishin ura reale ndër-procesore.

## Rrjedha production pas v1.6.9

```text
main.rs
-> process_bridge
-> Shadow::with_disk
-> ingest_bridged
   -> HCP heart nga inbound real
   -> PA / XiYi / GCL verification
   -> ingest_unsealed
      -> staged ShadowVerificationRuntime
      -> ShadowSpine::adjudicate_runtime
         -> Evidence/Adversarial/TRL/CrossDomain/NegativeKnowledge Courts
         -> NightWatch opsional
         -> VerificationMemoryIndex
         -> WisdomWarehouse
         -> MultiVerificationAttestation
      -> run_pipeline
         -> Router / Matrix / route nodes
         -> NightWatch i njëjtë për Deep Consensus
         -> S.MULTI_VERIFY
         -> S.JUDICIARY (fail-closed nga multi gate)
         -> sovereign laws
         -> ShadowGjLegacy — i vetmi verdict final
         -> KnowledgeVault
      -> commit i memory/wisdom vetëm pas suksesit
```

## Main.rs si ura e vetme drejt kushtetutës Shadow

Kufiri tani provohet edhe nga build-i:

```text
Cargo.toml: autolib = false
Cargo.toml: asnjë [lib]
main.rs: include!("lib.rs")
main.rs: mod process_bridge
build.rs: fail-closed nëse një invariant mungon
```

`lib.rs` mbetet kushtetuta source që kompilohet vetëm brenda binarit Shadow. Nuk prodhohet rlib/staticlib/dylib. Adapterët direct-call `shadow_out` dhe `shadow_callable` kompilohen vetëm me `cfg(test)`; production hyn vetëm nga `main.rs/process_bridge`.

## Multi-verifikimi dhe sistematizimi i gjendjes

`ShadowVerificationRuntime` mban:

```text
VerificationMemoryIndex
WisdomWarehouse
```

Çdo cikël përdor staging. Ledger-i, negative knowledge dhe wisdom angazhohen vetëm nëse pipeline-i sovran përfundon me sukses. Një gabim pas multi-verifikimit nuk lejon që memoria të regjistrojë një ngjarje që nuk u materializua.

`S.MULTI_VERIFY` u shtua pas discriminant-it historik `Judiciary`, që vlerat ekzistuese numerike të nodave të mos ndryshojnë. Në rrjedhë, ai futet përpara Judiciary. Rrugët minimale u rritën:

```text
Fast:     4 noda
Standard: 7 noda
Deep:    10 noda pa Watch, 11 me Watch
```

Judiciary shumëzon rezultatin e vet me bitin e multi-verifikimit. Mungesa ose dështimi i `S.MULTI_VERIFY` jep zero dhe nuk mund të fshihet nga një mesatare e lartë e nodave të tjera.

## Evidence adapter pa wire të dytë

Nuk u ndryshua kontrata wire. `EvidencePackage` ndërtohet brenda Shadow nga gjendja ekzistuese:

```text
PassPackage (Quantum)
+
LightEnvelope (Light)
```

Ai përdor vetëm prova reale të disponueshme: GCL/spine/action digests, scores, proof chain, vula Light, scientific project kur ekziston, final evidence digest dhe NightWatch report kur ekziston. Cross-domain nuk shpiket; aktivizohet vetëm kur projekti real ka domain të ndryshëm nga territory i Light.

Court ledger tani përdor SHA-256 real të `TrlEvidence.evidence_hash`; pseudo-hash-i tekstual `CLM...TRL...SRV...` u hoq.

## Cargo baseline i përdorur

Gjurmët historike të v1.6.7/v1.6.8 u përdorën si referencë:

```text
workspace cargo check          PASS historik
Shadow C-kernel/release build  PASS historik
real Shadow tests              jo plotësisht green
Clippy / CI acceptance         RED historik
```

Problemet historike të testit — bridge payload, GCL preconditions, vault promotion, seal counts dhe pulse global-state interference — nuk u ndryshuan pa log të ri që provon shkakun në v1.6.9.

## Verifikimi i kryer në ambientin aktual

```text
Rust files parsed                  279
Tree-sitter syntax error files       0
Changed core files                  11
Added/deleted core files              0
Changes outside Shadow               0
Main/lib boundary checks           PASS
ShadowSpine production caller      PASS
Mandatory multi gate               PASS
Active HCP-heart ingress           PASS
Direct adapters test-only          PASS
Actual evidence SHA in ledger      PASS
```

Ky ambient nuk ka `cargo`/`rustc`; prandaj nuk deklarohen Cargo-green, test-green, Clippy-green ose release-green.

## Progresi i v1.6.9

```text
1. Shadow call graph / Cargo panorama             COMPLETE
2. Main/lib authority boundary                    IMPLEMENTED
3. ShadowSpine production activation              IMPLEMENTED
4. Six-court multi-verification state              IMPLEMENTED
5. Optional NightWatch continuity                  IMPLEMENTED
6. Verification memory + wisdom staging            IMPLEMENTED
7. Mandatory S.MULTI_VERIFY gate                   IMPLEMENTED
8. Judiciary fail-closed integration               IMPLEMENTED
9. Active HCP heart ingress                        IMPLEMENTED
10. Direct adapter production removal              IMPLEMENTED
11. Real SHA evidence ledger                       IMPLEMENTED
12. Targeted behavioral tests authored             IMPLEMENTED_NOT_EXECUTED
13. Shadow Cargo/test/Clippy validation            PENDING_EXTERNAL
14. Full workspace/release acceptance              PENDING_EXTERNAL
```

**Scope implementation progress:** `12/14 = 85.7%`.

Ky numër nuk është release readiness. Dy milestone-et e mbetura janë prova Cargo dhe acceptance reale në laptop.

## Komandat e detyrueshme për validim

Nga rrënja `v1.6.9_ess_mai/ess_mai`:

```powershell
cargo check -p shadow_platform --all-targets
cargo test -p shadow_platform --no-default-features --features pure_rust --no-fail-fast
cargo build -p shadow_platform
cargo build --release -p shadow_platform
cargo clippy -p shadow_platform --all-targets -- -D warnings
cargo check --workspace --all-targets
cargo test --workspace --no-fail-fast
```

Në Windows, build-i real C kërkon toolchain GNU. `pure_rust` është vetëm test/dev dhe build.rs refuzon kombinimin me `runtime_mode`.

## Kufizimet e mbetura

1. VerificationMemory/Wisdom e re është process-lifetime; final knowledge mbetet durable në KnowledgeVault, por precedent index nuk rehidrohet ende pas restart-it.
2. EvidencePackage është adapter i brendshëm mbi wire-in ekzistues; nuk u krijua kontratë wire e dytë.
3. NightWatch mungues është neutral; NightWatch i pranishëm dhe i dështuar mbyll multi gate.
4. Dështimet historike Cargo/test nuk janë maskuar dhe mund të kërkojnë patch shtesë pas logut real.

**Verdikti aktual:**

```text
SHADOW MAIN MEDIATION       STATICALLY ENFORCED
SHADOW MULTI-VERIFICATION   PRODUCTION-INTEGRATED
SHADOW STATE SYSTEMIZATION  PROCESS-RUNTIME INTEGRATED
FINAL AUTHORITY             SHADOW_GJ_LEGACY UNCHANGED
CARGO / RELEASE ACCEPTANCE  PENDING LAPTOP EVIDENCE
```
