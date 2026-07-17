# ESS-MAI — Gjurmë autoritative ndër versione

## Identiteti kushtetues

**Architecture Category:** RIS (Royal Intelligent System) — Governance in data and flexible hardware.

**Architecture Family:** NurAtomic — deterministic verified secure output from verified trusted input, by deterministic collapse mathematics over a split Primitive Trace.

**System:** ESS-MAI — Core Foundation Research Governance over a sovereign, traceable cognitive runtime architecture governed by GCL and Primitive Trace.

`GCL ↔ ESS-MAI`. GCL nuk është modul ndihmës. ESS-MAI është trupi ekzekutiv i GCL-së dhe GCL është rendi kushtetues i ESS-MAI.

---

## Versioni bazë: v1.5.6

### Filozofia e trashëguar

Formula e kontinuumit:

```text
i + U(user input) → i₀
i₀ + 1Q(question IQ) → iZ(PD)
PD(i₀) → output + iZ
output + iZ → next i₀
```

`i₀` është fillesa primitive. PD është completion-i `iZ`, jo vetë `i₀`. PD aktivizon Spine 9, merr kthimin e Layer 1/2/3, pret Shadow dhe vetëm pas verifikimit përfundon output-in dhe iZ.

### Gjendja Cargo e provuar për v1.5.6

Logjet Windows të datës 14 korrik 2026 provuan:

```text
cargo check --workspace --all-targets: exit 101
cargo test --workspace --all-targets --no-run: exit 101
blocking error: E0505
quantum/src/progressive_debatic/runtime.rs: completion huazohej dhe lëvizej brenda të njëjtit kufi autoriteti
```

v1.5.6 nuk u deklarua Cargo-green.

### Boshllëku i zbuluar

`PdEngineOutput` ruante mode-in dhe GeniusSignal, por `PdSpineRequest` i reduktonte në `response_kind` dhe digest-e të përgjithshme. Completion-i ruante vetëm `layer_mask`, `evidence_digest` dhe `mpro_mass`; nuk provonte lineage-in L1→L2→L3 dhe as nënndarjen e PD-së që e nisi ciklin.

---

# Versioni i implementuar: v1.5.7

## Objektivi

Të mbyllet kontrata e veprimit PD përpara token-it GCL:

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

## Filozofia e nënndarjeve të PD-së

### Intellect

Gjuha e zakonshme dhe detyrat e lehta. Plani bazë aktivizon Layer 1. Nuk ngre artificialisht kërkim të thellë.

### Philosophy

Përdoruesi ka shumë ide dhe inteligjencë, por i mungon strukturimi, mbrojtja oratorike ose formimi shkencor. Plani bazë aktivizon Layer 1 dhe Layer 2 për strukturim dhe kundërprovë.

### Scientific

Përdoruesi ka njohuri, formim, metodë dhe kryen kërkim ose bisedë shkencore. Plani bazë aktivizon Layer 1, Layer 2 dhe Layer 3.

### Novel

Një ide e re lind nga përdoruesi. Novel nuk hamendësohet nga fjalë kyçe dhe nuk shpallet nga PD pa trace. Në v1.5.7 kontrata Novel e autorizuar për kërkim lind nga `GeniusDetected`, i cili ruan origjinën e signal-it dhe trace-it.

### Genius

Genius nuk është mode paralel. Është aktivizues autonom kur sistemi ka grumbulluar informacion të mjaftueshëm për të kërkuar zgjidhjen e idesë Novel pa e lodhur më përdoruesin.

```text
Novel(User)
+ sufficient trace
+ structural coherence
+ research readiness
→ GeniusAutonomousResearch
→ Layer 1 + Layer 2 + Layer 3
```

Origjina Novel mbetet e përdoruesit; kërkimi dhe formimi i zgjidhjes kryhen nga sistemi.

## Implementimi real

### 1. Kontrata e aktivizimit të PD-së

Skedarë byte-identikë:

```text
light/src/pd_spine_contract.rs
quantum/src/pd_spine_contract.rs
shadow/src/pd_spine_contract.rs
```

Tipet e shtuara:

```text
PdCognitiveMode
PdActivationOrigin
PdActivationContract
PdLayerReceipt
```

`PdActivationContract` ruan:

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

Rregulli Genius:

```text
origin == GeniusAutonomousResearch
⇒ mode == Novel
⇒ genius_signal_digest != 0
⇒ required_layer_mask == Layer1|Layer2|Layer3
```

### 2. Lidhja e output-it PD me kontratën

Skedari:

```text
quantum/src/progressive_debatic/runtime.rs
```

Funksionet:

```text
map_mode()
activation_from_output()
ingest_for_spine_sealed()
```

`Continue` ruan mode-in real të `PdTurn`. `GeniusDetected` prodhon `Novel + GeniusAutonomousResearch` dhe lidh `signal_id`, `trace_ref`, `genius_score`, accumulated mass dhe structural coherence.

### 3. Zgjedhja fillestare pa hamendësim Novel

Skedari:

```text
quantum/src/main.rs
```

Mode-i fillestar merret vetëm nga territory i deklaruar:

```text
philosophy/philosophical → Philosophy
science/scientific/research → Scientific
çdo territory tjetër → Intellect
```

Novel nuk zgjidhet nga territory ose nga fjalë kyçe; ai lind kur `GeniusDetected` provon trace-in e idesë së përdoruesit.

### 4. Lineage i Layer 1/2/3

Skedari:

```text
quantum/src/main.rs
```

`PdLayerReceipt` krijohet për çdo shtresë:

```text
Layer 1 parent = 0
Layer 2 parent = Layer 1 result_digest
Layer 3 parent = Layer 2 result_digest
```

Të tre receipts lidhen me:

```text
activation_id
activation_contract_digest
layer identity
result material
completed state
```

`PdSpineCompletion::closes_all_layers()` refuzon completion-in nëse prishet activation identity ose zinxhiri L1→L2→L3.

### 5. GCL token-i kushtetues

Skedarët:

```text
quantum/src/progressive_debatic/seal.rs
quantum/src/progressive_debatic/types.rs
```

Tipet:

```text
GclActionAuthorizationToken
PdAuthorizedCompletion
```

GCL konsumon `PdSpineCompletion`; nuk e huazon dhe nuk e lëviz në të njëjtën kohë. Kandidati rilind si `PdAuthorizedCompletion`.

Token-i përmban:

```text
contract_version
action_sha256
law_trace_sha256
law_mask
verdict
issued_at
```

Ligjet e kontrolluara:

```text
continuum readiness
PD activation contract
Layer 1→2→3 lineage
Genius full-layer rule
ready-for-Shadow boundary
```

SHA-256 lidh veprimin konkret; GCL/ESS-MAI jep autoritetin. SHA-256 nuk zëvendëson GCL.

### 6. Zgjidhja e E0505

Në v1.5.6:

```text
&completion → autoriteti
completion → move brenda closure-it
```

Në v1.5.7:

```text
PdSpineCompletion candidate
→ GCL consumes candidate
→ PdAuthorizedCompletion
→ PendingNextI0
```

Nuk përdoret `clone()` për të maskuar kufirin e autoritetit. Clone-i i mbetur përdoret vetëm pas autorizimit për bartjen e të njëjtit payload në strukturat ekzistuese, jo gjatë borrow-it kushtetues.

### 7. SHA-256 kur formohet iZ

Skedari:

```text
quantum/src/progressive_debatic/runtime.rs
```

`PdIzCompletion` dhe `PdNextI0` tani mbajnë:

```text
iz_sha256: [u8; 32]
```

Materiali kanonik i iZ përfshin:

```text
continuum activation digest
question increment digest
verified output digest
Shadow verification receipt id
GCL action SHA-256
GCL law-trace SHA-256
```

Kështu iZ lidhet me inputin, 1Q, output-in e verifikuar, receipt-in e Shadow dhe token-in kushtetues GCL.

## Map i v1.5.7

```text
User
  ↓
i + U → i₀
  ↓
PD Turn
  ├─ Intellect
  ├─ Philosophy
  ├─ Scientific
  └─ Novel (vetëm nga trace/Genius)
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

## Simulime statike

### Intellect

Detyrë e zakonshme → mode Intellect → Layer 1 është minimumi kontraktual. Pipeline-i aktual mund të prodhojë të tri shtresat, por GCL provon të paktën planin e kërkuar.

### Philosophy

Territory philosophy → Philosophy → Layer 1 dhe Layer 2 kërkohen. Layer 2 duhet të ketë parent digest të Layer 1.

### Scientific

Territory science/research → Scientific → të tri shtresat kërkohen dhe lidhen në rend.

### Novel + Genius

`GeniusDetected` → Novel + GeniusAutonomousResearch → genius digest jo-zero → të tri shtresat janë të detyrueshme. Mungesa e një shtrese refuzon token-in.

### Candidate swap

Nëse Layer 2 ose Layer 3 vjen nga activation tjetër, `activation_id` ose `activation_contract_digest` nuk përputhet dhe completion-i refuzohet.

### Broken parent

Nëse Layer 3 nuk ka parent digest të Layer 2, `closes_all_layers()` dështon dhe GCL token nuk prodhohet.

## Verifikimi i kryer në këtë paketim

- Kontrata `pd_spine_contract.rs` është byte-identike në Light, Quantum dhe Shadow.
- Kontrata `pd_continuum_contract.rs` është byte-identike në Light, Quantum dhe Shadow.
- Versionet e crates u ngritën në `1.5.7`.
- U eliminua modeli borrow+move që prodhoi E0505 në v1.5.6.
- U kontrolluan statikisht call-sites për initializer-at e tipeve të ndryshuara.
- U krijua ZIP-i i plotë, jo patch.

## Kufiri i provës

Në mjedisin e paketimit nuk kishte `cargo`, `rustc` ose `rustfmt`. Prandaj v1.5.7 **nuk deklarohet Cargo-green**. Verifikimi autoritativ duhet të kryhet në makinën Windows të përdoruesit me `VALIDATE_V157.ps1` dhe logjet duhet të ruhen në këtë dokument në versionin pasardhës.

## Rreziqe të hapura

- `iz_sha256` formohet dhe mbahet në objektet Quantum; transporti i plotë i 32 byte-ve deri te PD Light duhet verifikuar në auditimin e ardhshëm të wire contract-it.
- Pipeline-i ekzistues prodhon të tri shtresat edhe kur plani minimal i mode-it kërkon më pak; optimizimi i ekzekutimit nuk u bë në v1.5.7 që të mos prishej arkitektura operative pa prova Cargo.
- Mode selection mbështetet në territory të deklaruar për Philosophy/Scientific; klasifikimi semantik automatik nuk u shpik.
- Shadow duhet të verifikojë në versionin e ardhshëm `action_sha256` dhe `law_trace_sha256` si pjesë e receipt-it final, jo vetëm pre-seal-it Quantum.

---

Ky dokument nuk riemërtohet. Çdo version pasues duhet të shtojë versionin, filozofinë, teorinë, formulën, implementimin real, vendndodhjen në kod, map-in, provat Cargo, rreziqet dhe çështjet e hapura.

---

# Evolucioni i v1.5.8

## Theory name

**PD Receipt Lifetime Closure** — mbyllja e kontratës së jetëgjatësisë për shënimin e burimit të handoff-it PD.

## Philosophy

Një dëshmi sovrane nuk ndryshon kuptim për shkak të një gabimi borrow/lifetime. Gabimi duhet të korrigjohet në kufirin më të ngushtë të provuar, pa prekur receipt-in, formulën e PD-së ose autoritetin Shadow.

## Purpose

Të eliminohej `E0521` në `quantum/src/main.rs` pa përdorur klonim, leak, alokim artificial ose ndryshim të rrjedhës:

```text
Shadow VerificationReceipt
→ Quantum rillogarit receipt_id
→ rrjedha::note(site)
→ PD Light handoff
```

## Mathematical model

```text
source ∈ S_static
S_static ⊂ S_str
rrjedha::note : S_static → Evidence
```

Call-site-et e provuara ishin literale konstante:

```text
"main::export_pd_probe"
"main::export_pd_handoff"
```

Prandaj kontrata e saktë është:

```rust
source: &'static str
```

jo `source: &str`.

## Runtime flow

```text
PD closure
→ VerificationReceipt nga Shadow
→ receipt_id rillogaritet në Quantum
→ source literal statik shënon ledger-in vetëm në refuzim wire
→ output + iZ → next i₀
```

## Contracts

- `VerificationReceipt` mbeti i pandryshuar.
- `receipt_id(...)` mbeti i pandryshuar.
- `PdContinuumClosure` mbeti i pandryshuar.
- `rrjedha::note` vazhdon të kërkojë `&'static str` për identitet moduli të qëndrueshëm.
- Manifesti SHA-256 u normalizua nga rrugë absolute paketimi në rrugë relative të projektit.

## Code location

```text
quantum/src/main.rs
  export_pd_probe
  export_pd_handoff
  export_pd_verified_line

ESS_MAI_V1_5_7_FILELIST.sha256
VALIDATE_V157.ps1
AUDIT_V157_CARGO_PD_CONTINUITY.md
```

## Version introduced

```text
Theory:        e trashëguar nga rrjedha PD v1.5.6/v1.5.7
Implementation: v1.5.8
Verification:   audit statik + prova Cargo e gabimit E0521 nga Windows
Release status: paketuar si v1.5.8_ess_mai
```

## Verification evidence

Logjet e Windows provuan një gabim të vetëm bllokues:

```text
error[E0521]: borrowed data escapes outside of function
quantum/src/main.rs
```

Call-site-et u ndoqën dhe u provuan si literale `&'static str`. Manifesti origjinal kishte hash-e të sakta, por rrugë absolute Linux; manifesti i normalizuar përdor rrugë relative.

## Current status

```text
Theory:        VERIFIED
Implementation: IMPLEMENTED
Verification:  CARGO_RECHECK_REQUIRED
Release:       RELEASED_AS_BASELINE_FOR_V1.5.9
```

---

# Versioni i implementuar: v1.5.9

## Theory name

**Shadow Main Necessary Precondition / Complete Mediation Contract**

## Philosophy

`Shadow lib.rs` zotëron kushtetutën e verifikimit, por nuk ka zë të linkueshëm jashtë procesit Shadow. `main.rs` nuk është thjesht një portë e rekomanduar; është kushti strukturor pa të cilin kodi sovran nuk kompilohet si target i aksesueshëm nga crate të tjera.

```text
main.rs aktiv
⇒ Shadow core ekziston në runtime

¬main.rs aktiv
⇒ asnjë Shadow core i linkueshëm
⇒ asnjë ingest
⇒ asnjë vault
⇒ asnjë LgcToken
⇒ asnjë VerificationReceipt
```

Quantum njeh vetëm formën e dëshmisë. Ai nuk njeh prodhuesin sovran të saj.

## Purpose

Të mbyllet bypass-i real i v1.5.8 ku Quantum mund të bënte:

```text
shadow_lib::Shadow::new()
shadow.ingest_bridged(...)
shadow.on_negative(...)
```

duke linkuar direkt `shadow_platform` si `rlib` dhe pa ekzekutuar `shadow/src/main.rs`.

## Mathematical model

Le të jenë:

```text
Q = procesi Quantum
M = procesi Shadow main.rs
L = Shadow core/lib.rs
C = shadow_contracts
R = VerificationReceipt
```

Kontrata v1.5.9:

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

## Runtime flow

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
Quantum rillogarit receipt_id
  ↓
PD finalize_after_verification
  ↓
output + iZ → next i₀
  ↓
PD Light → Nura
```

## Contracts

### Build contract

```toml
shadow/Cargo.toml

autolib = false
autotests = false
autoexamples = true

[[bin]]
name = "shadow_platform"
path = "src/main.rs"
```

Shadow nuk prodhon më:

```text
rlib
staticlib
```

`shadow/src/main.rs` përfshin:

```rust
include!("lib.rs");
```

Kështu modulet e `lib.rs` bëhen pjesë e crate-it binar dhe nuk janë target library i linkueshëm.

### Public form contract

Crate i ri:

```text
shadow-contracts/
```

Ai përmban vetëm:

- formatet wire të origjinës Quantum;
- formatet wire të origjinës Light;
- formatin wire të Negative Knowledge;
- formën publike të `VerificationReceipt`;
- verdiktin minimal publik;
- codec determinist me version, lloj frame, gjatësi dhe checksum FNV-1a;
- kufij maksimalë të frame/field/vector.

Ai nuk përmban:

- `Shadow`;
- vault;
- pipeline;
- GCL authority;
- `LgcToken`;
- `seal_verified_output`;
- API shkrimi persistent.

### Process contract

Quantum zgjidh vetëm binarin:

```text
ESSMAI_SHADOW_BIN=<absolute path>
```

ose binarin sibling:

```text
target/debug/shadow_platform(.exe)
target/release/shadow_platform(.exe)
```

Mungesa e binarit, status jo-zero, response i munguar, checksum i gabuar ose session mismatch japin fail-closed.

### Identity contract

Kërkesa e ciklit mban të dy origjinat të ndara:

```text
QuantumInboundWire
LightInboundWire
```

Shadow main verifikon para core-it:

```text
Quantum.session == Light.session
Quantum.territory == Light.territory
Quantum.primitive_flags == Light.primitive_flags
session != empty
```

Receipt-i vazhdon të lidhet me:

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

## Code location

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

## Version introduced

```text
Theory:         v1.5.9 authoritative clarification
Implementation: v1.5.9
Verification:   static architecture audit in packaging environment
Release status: packaged; Cargo verification pending on Windows GNU
```

## Verification evidence

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

The shared contracts crate includes roundtrip and checksum-corruption tests. Unit tests inside Shadow modules remain compiled as binary-unit tests. Testi historik `shadow/tests/integration.rs` përfshihet nga `shadow/src/main.rs` dhe ekzekutohet brenda target-it binar, pa krijuar library të linkueshme. Shembulli `shadow/examples/full_flow.rs` nuk prek më core-in; ai dokumenton vetëm nisjen e rrjedhës së ndërmjetësuar.

## Current status

```text
Theory:         CONTRACT_DEFINED
Implementation: IMPLEMENTED
Runtime:        PROCESS_MEDIATED
Static audit:   PASSED
Cargo check:    PENDING_EXTERNAL_ENVIRONMENT
Cargo test:     PENDING_EXTERNAL_ENVIRONMENT
Release:        READY_FOR_WINDOWS_GNU_VERIFICATION
```

## Map i v1.5.9

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

## Kufiri i provës

Mjedisi i paketimit të v1.5.9 nuk kishte `cargo`, `rustc` ose `rustfmt`. Për këtë arsye statusi nuk quhet Cargo-green. Prova përfundimtare duhet të prodhohet në Windows GNU me:

```text
cargo check --workspace --all-targets
cargo test --workspace --all-targets --no-run
cargo clippy --workspace --all-targets -- -W clippy::all
VALIDATE_V159.ps1
```

Çdo dështim i Cargo-s duhet trajtuar si provë konkrete për ciklin pasues; nuk autorizohet ndryshim spekulativ.

---

# Evolucioni v1.5.9 → v1.6.0

## Theory name

**GCL-Governed Deep Processing, Final Evidence Closure and iZ Dual-Surface Continuity**

## Philosophy

GCL nuk është një modul që takohet ose kryqëzohet me Layer 1, Layer 2 dhe
Layer 3. GCL është ligji i shpallur mbi të gjithë ESS-MAI. PD Quantum e hap
procesin e qeverisur; Spine 9 e organizon; Layer 1→2→3 vetëm e çojnë më thellë
të njëjtin procedim. Asnjë Layer nuk krijon autoritet të ri, verdict ose
identitet paralel.

PD Light nuk është kopje proceduese e PD Quantum. Ai është korrieri kontekstual
i iZ-së së verifikuar. Nga i njëjti iZ dalin paralelisht Nura dhe UI-ja e vjetër
emocionale. Nura i jep zë output-it; UI-ja e vjetër shfaq gjendjen/animacionet
dhe ia transmeton sinjalin UI-së së re. Asnjëra nuk rihap reasoning-un.

Shadow main verifikon paketën finale të një cikli. Legacy Shadow C vazhdon të
vëzhgojë sistemin dhe afrimin e primitivëve drejt Legacy. Këto janë dy role të
ndryshme dhe nuk u bashkuan.

## Purpose

v1.6.0 mbyll vetëm shkëputje të provuara:

1. Të provojë se i njëjti proces GCL ekziston para Spine 9 dhe ruhet në çdo
   receipt të Layer 1→2→3.
2. Të mos dërgohen te Shadow vetëm masa të reduktuara; PIM, NPIM dhe MPRO të
   formojnë një paketë finale të rillogaritshme me evidenca.
3. Të bëjë SHA-256 e inputit të Light një lineage real Light→Quantum→Shadow.
4. Të bëjë PD Light një dorëzim të tipizuar iZ drejt Nura-s dhe UI-së së vjetër
   emocionale, në paralel.
5. Të mbajë Matrix-in të lidhur me digest-in e evidencës finale, procesin GCL
   dhe completion-in e Spine 9 pa i dhënë Matrix-it rol arsyetues të Quantum.
6. Të mos i besohet as digest-it të gatshëm të continuum-it ose aktivizimit:
   Shadow të rillogarisë `i + U → i₀ → 1Q`, kontratën njohëse të PD-së,
   aktivizimin e Spine 9, receipts-at e Layer 1/2/3 dhe completion-in.

## Mathematical model

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

`H` në kontratat e vogla është digest deterministik i kontratës. SHA-256
vazhdon të përdoret për inputin Light dhe autorizimin GCL të veprimit PD.
Paketimi wire ka checksum deterministik dhe pastaj lidhet brenda receipt-it
sovran të Shadow; nuk paraqitet si enkriptim i fshehtësisë.

## Runtime flow

```text
User input U
  ↓
Light Coordination Collapse
  ├─ Primitive Trace / PA / Xi,Yi
  └─ SHA256(U)
  ↓
Quantum input gate
  └─ rillogarit SHA256(U); mismatch → fail-closed
  ↓
PD Quantum
  └─ krijon PdSpineRequest me GclProcessAuthority G
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
  ├─ recompute i + U → i₀ stimulus dhe 1Q increment
  ├─ recompute PD cognitive activation (të tre Layers = 111)
  ├─ recompute GCL process, Spine activation, L1→L2→L3 dhe completion
  ├─ recompute evidence package digest
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

### GCL process continuity contract

`pd_spine_contract.rs` is byte-identical in Light, Quantum and Shadow. It adds
`GclProcessAuthority` and binds `gcl_process_digest` into each Layer receipt and
Spine completion. The post-Layer GCL seal now proves continuity of an authority
that existed before Layers; it does not introduce GCL after processing.

### Final evidence wire contract

`shadow-contracts` protocol version 2 adds `FinalEvidenceWire`. It carries:

- `PdContinuumEvidenceWire`: materialin e stimulus-it `i + U → i₀`, incrementin
  e vetëm `1Q`, gjendjet e tyre dhe activation digest-in;
- `PdActivationEvidenceWire`: mode/origin, response-kind, Genius signal,
  trace/coherence masses, maskën e detyrueshme `111` dhe contract digest-in;
- `PdSpineEvidenceWire`: GCL law/system seals, procesin GCL, activation ID,
  materialin dhe receipt-in e secilit Layer, plus Spine completion;
- Light input SHA-256 and exact input bytes;
- PIM fixed metrics, suggestion and proof chain;
- strengthened NPIM metrics, suggestion, arguments and argument-blob digest;
- sixteen MPRO measurements, positive count, total, vector mass, evidence mass
  and factic mass;
- deterministic package digest.

Shadow main verifies the contract before creating `Shadow` or entering core.
Ai rillogarit jo vetëm digest-et e paketës, por edhe stimulus-in, 1Q,
kontratën njohëse, GCL process, activation ID, tre receipts-at dhe completion-in.
NaN dhe ±Infinity refuzohen në kufirin e procesit.

### Negative Knowledge contract

`NegativeKnowledgeWire` now carries `suggestion_code`. Shadow compares mass,
frequency, suggestion and blob digest against NPIM in the final package. The
strengthened NPIM profile is now the profile actually sent and persisted; it no
longer exists only as a log line.

### Input identity contract

The Light→Quantum payload now includes `input_sha256`. Quantum validates it
against the received text before any reasoning. The final evidence carries the
same SHA-256 and source bytes; Shadow recomputes it independently before core.

### PD Light delivery contract

`PdLight::deliver` returns `VerifiedPdDelivery`:

```text
VerifiedPdSurface      → Nura → New UI content
PdUiContinuitySignal   → Legacy Emotional UI
                       → Light stdout `[PD_LIGHT/IZ]`
                       → Tauri EmotionalCommand
                       → New UI emotion/animation
```

Të dy kanalet lindin vetëm pasi kalojnë receipt, continuum, output dhe iZ.
`LegacyEmotionalUi` nuk ekzekuton Layer 1/2/3, refuzon digest zero dhe përdor
transportin ekzistues real të UI-së; nuk shkruan në një target pa konsumator.

### Matrix state contract

`PassPackage` and `SystematizedCase` carry:

```text
final_evidence_digest
pd_gcl_process_digest
spine_completion_digest
```

Matrix receives these as verified state context. No new scoring weight or
reasoning behavior was invented.

## Code location

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

## Version introduced

```text
Theory:         authoritative clarification before v1.6.0
Contract:       v1.6.0
Implementation: v1.6.0
Static audit:   v1.6.0 packaging cycle
Release status: packaged for Windows GNU verification
```

## Verification evidence

Static verification performed in the packaging environment:

```text
- all Rust source delimiters balanced
- PD Spine contract byte-identical in Light/Quantum/Shadow
- GCL process digest present in request, all three receipts and completion
- no changed code introduced if/else in the implemented contracts
- wire codec order matches every declared schema:
  PdContinuumEvidenceWire(16), PdActivationEvidenceWire(8),
  PdLayerEvidenceWire(9), PdSpineEvidenceWire(24), FinalEvidenceWire(25)
- Shadow recomputes stimulus, 1Q, PD activation, GCL process, the canonical
  source material of every Layer, all Layer receipts and Spine completion before
  entering sovereign core
- MPRO package requires exactly 16 binary measurements and recomputes masses
- NPIM bridge mass/frequency/suggestion/blob are compared by Shadow main
- Light input SHA-256 is required by Quantum input parser and recomputed twice
- Quantum retains no direct Shadow-core dependency
- Shadow remains binary-only and main-mediated
- PD Light dual delivery is typed and zero-digest fail-closed
- Legacy Shadow source path remains present and distinct
```

The implementation intentionally stops at two boundaries where the existing
architecture has no concrete contract: no confidentiality cipher/key hierarchy
was invented, and no raw generated-question payload was exposed beyond the
digest already defined by the PD contract. Integrity remains enforced through
SHA-256 lineage, deterministic digests, GCL bindings and sovereign receipts.

The packaging environment did not contain `cargo`, `rustc` or `rustfmt`.
Therefore Cargo build/test/clippy are not claimed as passed. The release includes
`VALIDATE_V160.ps1` for the authoritative Windows GNU proof.

## Current status

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

## Map i v1.6.0

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

## Boundaries deliberately not crossed

- Layer 1/2/3 were not moved to Light.
- Light emotional spine was not renamed into or made equivalent to PD Spine 9.
- Matrix was not given new speculative weights.
- Legacy Shadow was not removed or merged with final Shadow receipt.
- PIM/NPIM/MPRO modules do not call Shadow separately; only the final package
  crosses the process boundary.
- No license was invented or added.

---

# Evolucioni v1.6.0 → v1.6.1

## Theory name

**GCL — Vula e Gjallë e Besimit**

## Philosophy

ESS-MAI nuk i beson vetes gjatë përpunimit. Mosbesimi është gjendja
konstitutive: gjatë PRO/NPRO/NPIM/PIM/APRO/MPRO nuk ekziston vulë Besimi.
Organet vetëm konvergojnë aksionet e tyre në një gjendje të lehtë. Besimi lind
vetëm pasi Shadow jep verdiktin suprem dhe vetëm kur ligjet aktive, L-500,
verdikti dhe evidencat mbyllen në të njëjtin proof.

Vula nuk është autoritet i ri mbi GCL. Ajo është dëshmia përfundimtare se i
njëjti proces GCL, i thelluar nga Spine 9 dhe Layer 1/2/3, ka mbërritur në një
verdikt kushtetues. Shadow e prodhon; Quantum dhe Light e rillogarisin.

## Purpose

- Të bëjë L-500 ligj runtime të lidhur me çdo Besim të prodhuar.
- Të lidhë punën reale të moduleve me verdiktin suprem pa hash per-modul.
- Të prodhojë një identitet SHA-256 të gjithë proof-it në kulmin e ciklit.
- Të ndajë identitetin e Besimit nga intensiteti i tij.
- Të lidhë Besimin me `VerificationReceipt`, iZ dhe `next_i0`.
- Të kërkojë barazi të plotë Light ∥ Quantum ∥ Shadow.
- Të ruajë mosbesimin si default: pa vulë, pa finalizim PD të besueshëm.

## Mathematical model

### Faza 1 — konvergjenca e aksioneve, zero SHA-256

Për çdo organ të përfunduar realisht:

```text
A₀ = 0
Aₙ₊₁ = ROTL(ROTL(Aₙ,11) + (CONVERGE(Eₙ) XOR STAGEₙ), 7)
```

ku `CONVERGE` përdor vetëm rotacion, XOR dhe mbledhje wrapping. Gjendja `A`
është afrimi drejt vulës; ajo nuk është Besimi.

Organet që kontribuojnë:

```text
PRO → NPRO → NPIM → PIM → APRO → MPRO
```

Rendi është rendi real i ekzekutimit të ciklit. Të gjitha kontributet lidhen në
paketën finale PIM/NPIM/MPRO që Shadow verifikon.

### Faza 2 — pulsi i vetëm SHA-256 në verdikt

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

SHA-256 nuk përmban intensitetin. Identiteti tregon **çfarë proof-i u vulos**;
intensiteti tregon **sa forcë fitoi Besimi**.

### Intensiteti

Formula ekzistuese e Shadow ruhet:

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

### Lloji kushtetues

```text
(verified, primitive) = (1,1) → CONSTRUCTIVE_TRUST
(verified, primitive) = (0,0) → RIGOROUS_NEGATIVE_TRUST
çdo çift tjetër                  → NO_TRUST
```

### Lidhja kompakte

Kontratat ekzistuese përdorin një `u64` lidhës:

```text
living_trust_digest = FNV64(
    identity_sha256 || intensity || kind || sovereign_value || domain
)
```

Ky digest nuk është vula dhe nuk zëvendëson SHA-256. Ai lidh identitetin dhe
forcën me receipt, iZ dhe `next_i0`.

## Runtime flow

```text
i + U
  ↓
Light: koordinim + SHA256(input) → i0
  ↓
PD Quantum nën GCL
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

Paralelisht:

```text
Legacy Shadow C → vëzhgim i vazhdueshëm primitive → Legacy
```

Legacy Shadow nuk është prodhuesi i receipt-it final; ai mbetet vëzhguesi i
vazhdueshëm i sistemit.

## Contracts

### `living_trust_contract.rs`

Kontratë byte-identike në Light, Quantum dhe Shadow:

- `LivingTrustProof`;
- `LivingTrustSeal`;
- llojet kushtetuese;
- konstantet L-500;
- SHA-256 kanonik;
- fixed-point intensity;
- lidhja kompakte `identity_digest`.

### `VerificationReceipt`

`living_trust_digest` është pjesë e materialit të `receipt_id`. Një receipt nuk
mund të shkëputet nga Besimi që Shadow prodhoi.

### PD Continuum

`PdVerificationCompletion`, `PdIzCompletion` dhe `PdNextI0` bartin:

- SHA-256 e Besimit;
- intensitetin;
- llojin;
- vlerën sovrane 500;
- digest-in lidhës.

`iz_sha256` lidhet me output-in, continuum-in dhe Besimin.

### Wire contract

`shadow-contracts` përdor `PROTOCOL_VERSION = 3`. Wire-i bart proof-in e
nevojshëm që Quantum dhe Light të mos i besojnë verbërisht Shadow-it, por ta
rillogarisin vulën.

## Code location

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

## Version introduced

```text
Theory:         v1.6.1 authoritative paradigm
Contract:       v1.6.1
Implementation: v1.6.1
Static audit:   v1.6.1 packaging cycle
Release status: packaged for Windows GNU verification
```

## Verification evidence

Auditimi i stimulimit mbulon:

1. `(1,1)` → Besim ndërtimi;
2. `(0,0)` → Besim rigoroziteti negativ;
3. çift i përzier → pa Besim;
4. manipulim të `action_state`;
5. manipulim të ligjeve aktive;
6. manipulim të L-500;
7. manipulim të intensitetit;
8. manipulim të receipt-it;
9. manipulim të handoff-it Quantum→Light.

Provat statike të paketimit ruhen në:

```text
AUDIT_V161_GCL_LIVING_TRUST.md
V161_SIMULATION_MAP.md
ESS_MAI_V1_6_1_IMPLEMENTATION_MAP.md
STATIC_AUDIT_V161.txt
V1_6_1_FROM_V1_6_0.diff
ESS_MAI_V1_6_1_FILELIST.sha256
VALIDATE_V161.ps1
```

## Current status

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

## Boundaries deliberately not crossed

- Nuk u krijua autoritet i ri për Layers; ato mbeten thellim nën GCL.
- Nuk u bë PD Light procesues i Spine 9.
- Nuk u bashkua Legacy Shadow me Shadow final.
- Nuk u persistua vula si Knowledge state i pavarur.
- Nuk u ndryshuan peshat ekzistuese të `legacy_score`.
- Nuk u shpik enkriptim konfidencial pa kontratë konkrete të çelësave,
  rotacionit, revokimit dhe autoritetit të ruajtjes.

---

# Evolucioni v1.6.1 → v1.6.2

## Theory name

**GCL — Untrust Start to End / Prova e Plotë e Konvergjencës së Organeve**

## Philosophy

ESS-MAI nuk nis një cikël duke i besuar vetes, moduleve ose platformave.
Gjendja kushtetuese e fillimit është **Untrust**: asnjë organ nuk konsiderohet i
përfunduar, asnjë kontribut nuk konsiderohet provë dhe asnjë Vulë Besimi nuk
ekziston.

Mosbesimi nuk hiqet nga deklarata e modulit. Ai shkarkohet vetëm kur organi:

1. ekzekuton punën e vet reale;
2. derdh materialin kanonik të evidencës;
3. regjistrohet në rendin kushtetues të ciklit;
4. verifikohet përsëri nga Shadow mbi materialin burimor;
5. lidhet me paketën përfundimtare PIM/NPIM/MPRO dhe me marrëdhënien X→Y.

Besimi lind vetëm pas verdiktit suprem, si një puls i vetëm SHA-256 i
**Vulës së Gjallë**. Receipt-i dhe TokenForge përdorin SHA-256 si porta të
veçanta integriteti; ato nuk krijojnë vula të tjera supreme dhe nuk
zëvendësojnë Living Trust.

## Purpose

Ky evolucion mbyll pyetjen kritike të v1.6.1:

```text
A është quantum_action_state konvergjencë reale e punës së moduleve,
apo një vlerë e deklaruar/stub?
```

v1.6.2 e bën përgjigjen të verifikueshme:

- nëntë organet e kërkuara derdhin evidencë në pikat reale të përfundimit;
- wire-i nuk bart vetëm një kontribut të gatshëm;
- Shadow merr fjalët kanonike të evidencës;
- Shadow rillogarit çdo kontribut, maskën dhe të gjithë fold-in;
- Shadow e kryqëzon ledger-in me strukturat e pavarura të PIM, NPIM, MPRO,
  PRO, SRK, HPRO dhe HCP;
- Living Trust lejohet vetëm kur familja e plotë e organeve është provuar.

Qëllimi dytësor është eliminimi i FNV64 nga identitetet që qeverisin
mosbesimin/besimin:

- `VerificationReceipt` kalon në SHA-256;
- `TokenForge` kalon në SHA-256;
- FNV mbetet vetëm në checksum-e ose trace legacy jo-sovrane, ku përmbajtja
  riverifikohet në mënyrë të pavarur.

## Mathematical model

### 1. Gjendja fillestare Untrust

```text
A₀ = 0
M₀ = 0
L₀ = []
Trust₀ = ∅
```

ku:

- `A` është `action_state`;
- `M` është maska e organeve të përfunduara;
- `L` është ledger-i i evidencës;
- mungesa e Vulës është mosbesimi strukturor.

### 2. Familja e detyrueshme e organeve

Rendi kanonik real është:

```text
HPRO → PRO → NPRO → NPIM → SRK → PIM → APRO → MPRO → HCP
```

Kodet e organeve janë bitet `1..9`; maska e plotë është:

```text
REQUIRED_ACTION_MASK = 0x03FE
```

Skema e materialit kanonik është:

```text
HPRO = 5 fjalë
PRO  = 3 fjalë
NPRO = 4 fjalë
SRK  = 6 fjalë
APRO = 4 fjalë
MPRO = 21 fjalë
PIM  = 6 fjalë
NPIM = 5 fjalë
HCP  = 5 fjalë
```

### 3. Kontributi i lehtë i një organi

Për evidencën kanonike `Eₘ = [e₁, …, eₙ]` të organit `m`:

```text
Cₘ = converge_words(Eₘ) XOR stage_word(m)
```

`converge_words` përdor vetëm rotacion, XOR dhe mbledhje wrapping. Në Fazën 1
nuk kryhet SHA-256.

### 4. Fold-i i konvergjencës

```text
Aₖ₊₁ = ROTL₇(ROTL₁₁(Aₖ) + Cₘ)
Mₖ₊₁ = Mₖ OR bit(m)
Lₖ₊₁ = Lₖ || {m, Eₘ}
```

Konvergjenca pranohet vetëm kur:

```text
M_final = 0x03FE
order(L_final) = REQUIRED_ACTION_ORDER
schema(L_final) = REQUIRED_ACTION_WORD_COUNTS
replay(L_final) = A_final
```

### 5. Prova e plotë

Përfundimi i organit nuk vërtetohet vetëm nga prania në ledger. Shadow e
kryqëzon materialin me evidencat e transportuara në mënyrë të pavarur:

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

Pra:

```text
PROVA_E_PLOTË ⇔
    exact_mask
  ∧ exact_order
  ∧ exact_schema
  ∧ replayed_action_state
  ∧ all_cross_bindings
  ∧ X/Y verification
```

### 6. Vula e Gjallë

Vetëm pasi `judge_supreme` prodhon verdikt kushtetues:

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

Intensiteti mbetet forcë e veçantë fixed-point; nuk futet në identitetin e
Vulës, por lidhet më pas me receipt, iZ dhe `next_i0`.

### 7. Receipt-i sovran

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

Receipt-i nuk përdor më FNV64 si identitet sigurie.

### 8. TokenForge

TokenForge prodhon dëshmitar runtime SHA-256 32-byte. Ai nuk është organ i
arsyetimit dhe nuk lejohet të ndryshojë `action_state` ose maskën e nëntë
organeve.

### 9. iZ dhe cikli tjetër

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

Besimi i fituar është farë e vazhdimësisë, por cikli tjetër nis përsëri me
ledger/mask/state zero. Besimi nuk trashëgohet si privilegj i pakushtëzuar.

## Runtime flow

```text
Light: i + U → SHA256(input) → i0
  ↓
Quantum begin_cycle()
  action_state=0, action_mask=0, ledger=[]
  ↓
PD Quantum nën GCL
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
Shadow recomputes every contribution, mask and fold
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

Paralelisht, Legacy Shadow C vazhdon vëzhgimin e primitivëve drejt gjendjes
Legacy dhe nuk bëhet dublikatë e `judge_supreme`.

## Contracts

### `runtime_pulse.rs`

Përcakton:

- zeroimin e ciklit;
- nëntë organet e detyrueshme;
- rendin kanonik;
- skemën e fjalëve;
- fold-in pa SHA;
- ledger-in e materialit burimor;
- replay-in determinist;
- ndarjen e TokenForge nga action convergence.

### `shadow-contracts`

`PROTOCOL_VERSION = 5` transporton:

- `action_state`;
- `action_mask`;
- `required_action_mask`;
- ledger-in `{stage, evidence_words}`;
- materialin final PIM/NPIM/MPRO;
- provën e inputit Light;
- materialin GCL/PD/Spine;
- HPRO/HCP dhe evidencat e tjera të nevojshme për riverifikim.

### `living_trust_contract.rs`

Kontratë byte-identike në Light, Quantum dhe Shadow. Living Trust pranohet
vetëm kur maska e marrë dhe maska e kërkuar janë të barabarta saktësisht me
`0x03FE`.

### `verification_receipt.rs`

Kontratë byte-identike në Light, Quantum dhe Shadow, version `0x0001_0602`, me
identitet SHA-256 32-byte/64-hex dhe lidhje me SHA-256 e plotë të Living Trust.

### `token_forge.rs`

Token runtime SHA-256, i ndarë nga organet e arsyetimit dhe nga pulsi suprem i
Living Trust.

### GCL action authorization

`GclActionAuthorizationToken` ekzistues vazhdon të autorizojë pre-seal/pending
iZ me `action_sha256` dhe `law_trace_sha256`. Nuk u shpik një token i dytë i
papajtueshëm vetëm për emrin “Untrust”.

## Code location

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

## Version introduced

```text
Theory:         v1.6.2 authoritative paradigm
Contract:       v1.6.2
Implementation: v1.6.2
Static audit:   v1.6.2 packaging cycle
Cargo proof:    pending VALIDATE_V162 on Windows GNU
Release status: packaged for executive verification
```

## Verification evidence

Stimulimi dhe auditimi mbulojnë:

1. cikël me state/mask/ledger zero;
2. rendin e saktë të nëntë organeve;
3. mungesën e një organi;
4. organ të tepërt ose të përsëritur;
5. rend të manipuluar;
6. skemë të gabuar të fjalëve;
7. fjalë evidence të manipuluara;
8. `action_state` të deklaruar që nuk përputhet me replay-in;
9. maskë të deklaruar që nuk përputhet me ledger-in;
10. mospërputhje PRO ↔ kandidatët;
11. mospërputhje SRK ↔ PIM proof chain;
12. mospërputhje HPRO/APRO/MPRO;
13. mospërputhje NPIM;
14. mospërputhje HCP;
15. manipulim të Living Trust;
16. manipulim të Receipt-it SHA-256;
17. tentativë që TokenForge të ndotë action convergence;
18. vazhdimësinë Trust → receipt → iZ → next_i0;
19. ruajtjen e complete mediation të Shadow main.rs;
20. ndarjen e PD Light, Nura, UI-së emocionale dhe Legacy Shadow.

Provat ruhen në:

```text
AUDIT_V162_UNTRUST_START_TO_END.md
V162_SIMULATION_MAP.md
ESS_MAI_V1_6_2_IMPLEMENTATION_MAP.md
STATIC_AUDIT_V162.txt
V1_6_2_FROM_V1_6_1.diff
ESS_MAI_V1_6_2_FILELIST.sha256
VALIDATE_V162.ps1
```

## Current status

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

## Boundaries deliberately not crossed

- Cargo-green u mbajt si **release gate** dhe jo si bit runtime. Nuk ekziston
  ende kontratë e nënshkruar build-attestation me autoritet, çelës, format,
  rotacion dhe revokim.
- Nuk u shtua HMAC, sepse nuk ekziston kontratë sovrane e menaxhimit të
  çelësave.
- FNV checksum-et e mbetura në frame/package/legacy trace nuk u paraqitën si
  Trust, Receipt ose Token. Përmbajtja e tyre riverifikohet nga Shadow.
- Nuk u shpik remote physical attestation përtej evidencave ekzistuese HPRO/HCP.
- TokenForge nuk u bë organ reasoning dhe nuk u lejua të ndryshojë maskën e
  nëntë organeve.
- Layer 1/2/3 nuk morën autoritet paralel ndaj GCL.
- PD Light nuk u bë procesues i PD Quantum/Spine 9.

---

# Evolucioni v1.6.2 → v1.6.3 — GCL Scientific Project Continuum

## Identiteti i versionit

```text
Version:        ESS-MAI v1.6.3
Baseline:       ESS-MAI v1.6.2
Theory:         GCL Scientific Project Continuum
Authority:      GCL + Shadow main supreme mediation
Scope:          user scientific/innovative projects and Novel factualization
Cargo status:   PENDING external Windows GNU validation
```

Ky version nuk krijon një laborator paralel, një verdict të dytë ose akses të Quantum-it në magazinat e Shadow-it. Ai mbyll organet ekzistuese të projekteve të përdoruesit në një rrjedhë të vetme kushtetuese:

```text
Light/APUPK identity
→ Shadow durable project context
→ Light
→ Quantum scientific processing
→ FinalEvidence + nine-organ Untrust
→ Shadow main verification
→ same SupremeVerdict
→ Vula 500 + Living Trust + VerificationReceipt
→ Quantum PD / output+iZ / next i0
→ PD Light / Nura / UI
```

## Filozofia

Një projekt shkencor nuk shpallet Novel sepse Quantum e quan të tillë, sepse Digital Lab prodhon një TRL ose sepse ekziston dokumentacion. Secili organ prodhon vetëm pjesën e vet të provës:

- Light lind identitetin e projektit, gjurmën APUPK, SHA-256 e inputit dhe Vulën 500;
- Shadow ruan identitetin e projektit dhe lëshon vetëm një dëshmitar konteksti;
- Quantum procedon hipotezën, supozimet, provat, TRL, SRK, PIM/NPIM/MPRO dhe të nëntë organet Untrust;
- Shadow rillogarit materialin, e krahason me projektin durable dhe jep verdiktin suprem;
- GCL vulos identitetin e të gjithë qenies së projektit brenda Living Trust;
- PD/iZ bart vetëm rezultatin e verifikuar.

Ligji i versionit:

> Projekti që lindi në Light, projekti që u ruajt në Shadow, pyetja që Quantum procedoi, evidenca që Shadow gjykoi dhe statusi që hyri në Living Trust duhet të provohen si i njëjti objekt.

## Pse u kërkua v1.6.3

### 1. E0425 nuk ishte vetëm gabim demo

Cargo tregoi se `run_integrated_lab_demo()` kompilohej pa `dev_harness`, ndërsa `persist_negative()` ishte dev-only. Kjo zbuloi se Digital Lab kishte dy identitete të përziera:

```text
hard-coded demonstration
versus
real scientific-project processing organ
```

v1.6.3 i ndan:

- `run_lab_demo` dhe `run_integrated_lab_demo` mbeten dev-only;
- `LabSystemBridge::run_integrated` përdoret në rrugën reale Quantum për materialin e projektit;
- `persist_negative` standalone mbetet dev-only;
- negative production ruhet vetëm pas full Shadow cycle.

### 2. E0063 tregoi contract drift

`PassPackage` kishte evoluar me:

```text
quantum_action_state
quantum_action_mask
quantum_required_action_mask
```

por fixture-i Shadow nuk kishte dy maskat. Ato nuk u mbushën me zero. U përdor `REQUIRED_ACTION_MASK`, sepse një paketë “strong” duhet të deklarojë dhe të provojë të gjithë organet e detyrueshme. Fixture-i deklaron gjithashtu `scientific_project: None`; ai teston inner Shadow, jo rrjedhën Novel end-to-end.

### 3. Rruga pozitive dhe negative ishin asimetrike

Në v1.6.2 evidenca pozitive Digital Lab mund të ndalej në printimin “→ PIM”, ndërsa negative demo mund të kërkonte persistim të veçuar. Kjo cenonte ndarjen e roleve. v1.6.3 vendos si pozitivet, si negativet në të njëjtën paketë finale dhe ia lë Shadow-it vendimin.

### 4. Novel ishte komponent, jo continuum

Ekzistonin:

- APUPK Light dhe Shadow;
- Digital Lab;
- Governance dhe Raw Cognitive Trace;
- SRK, PIM, NPIM, MPRO;
- GeniusNovel;
- `ShadowEco::classify_with_factualization`;
- `judge_supreme`;
- Living Trust.

Por nuk ekzistonte një identitet i vetëm kriptografik që t’i lidhte. Ky version e krijon atë lidhje.

## Modeli matematikor dhe kriptografik

Le të jetë projekti i përdoruesit:

```text
P = {project_id, user_id, title, content, domain, hypothesis, assumptions, docs, files}
```

### Identiteti Light/APUPK

```text
trace_id = fold31(project_id + user_id, title)
input_sha = SHA256(content)
V500 = ((flags & 0xFFFF) XOR 0xA5A5) = 500
```

### Dëshmitari Shadow i kontekstit

Pas WAL durable:

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

### Evidenca Quantum

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

### Verdikti i projektit Shadow

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
    || Vula500 flags/value
    || E_Q[32]
    || V_S[32]
)
```

Digest-et `u64` të projektit ruhen vetëm si indekse compatibility/diagnostike. Living Trust përdor SHA-256 të plota.

## Kontrata e projektit ×3

U shtua byte-identik:

```text
light/src/gcl_project_contract.rs
quantum/src/gcl_project_contract.rs
shadow/src/gcl_project_contract.rs
```

Ai mban:

- `GCL_PROJECT_CONTRACT_VERSION = 0x0001_0603`;
- `project_trace_id`;
- `seal_is_500`;
- `ProjectContextMaterial`;
- `ProjectEvidenceMaterial`;
- `ProjectVerdictMaterial`;
- `context_sha256`;
- `evidence_sha256`;
- `verdict_sha256_or_zero`;
- parser/formatter SHA-256;
- file evidence canonical transport.

Kontrata nuk prodhon verdict. Ajo kanonizon materialin që secila platformë rillogarit.

## Protocol v8

`shadow-contracts` u ngrit në protocol 8 dhe shtoi:

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

`ScientificProjectWire` është `Option` brenda `FinalEvidenceWire`. Projekti nuk hap kanal të anashkalimit; kalon në të njëjtën paketë finale.

`ShadowVerdictWire` mban:

- project ID/status;
- context/evidence compatibility indices;
- full context SHA-256;
- full evidence SHA-256;
- factualized;
- TRL;
- proof score;
- rejection.

## Rrjedha Light

### Project intake

Entry point:

```text
light-platform --project-route-once REQUEST RESPONSE
```

Light validon formën, krijon `ProjectUpload`, APUPK trace dhe Vula 500.

### Shadow APUPK registration

Light nis vetëm procesin Shadow:

```text
shadow_platform --project-register-once REQUEST RESPONSE
```

Shadow kthen `ProjectContextWitnessWire`. Light rillogarit:

- project_id;
- user_id;
- trace_id;
- Light flags;
- content SHA;
- context SHA;
- Vula 500.

Një witness që nuk përputhet refuzohet.

### Light→Quantum process boundary

Projektet reale nuk përdorin bus-in legacy 2048-byte. Light ndërton `QuantumProjectExecutionRequestWire`, lidh payload-in me SHA-256 dhe nis:

```text
quantum-platform --project-process-once REQUEST RESPONSE
```

Përgjigjja duhet të mbajë SHA-256 e frame-it të kërkesës. Kjo ndalon stale/swap të response-it.

Light nuk deklaron Novel dhe nuk merr akses në reasoning intern të Quantum-it.

## Shadow APUPK durability

APUPK WAL u ngrit në version 2:

```text
shadow_apupk_v163.wal
```

Rekordi mban:

- project/user/trace;
- initial trace;
- project title;
- Light sovereign flags;
- content;
- progress;
- timestamps;
- notes;
- revision nga replay-i.

Para WAL verifikohen:

- trace jo-zero dhe initial trace jo-bosh;
- content jo-bosh;
- title jo-bosh;
- Vula 500;
- formula e trace-it;
- progres finite;
- ownership i pandryshuar i project_id.

`store_durable` kërkon WAL dhe përdor `append_checked`, i cili kryen write, flush dhe fsync. RAM ndryshon vetëm pasi durabiliteti kalon. Pa këtë, Shadow nuk jep witness.

Për shkak se Shadow ekzekutohet si one-shot process, u shtua dry ndër-proces me `create_new`. Kjo mbron WAL-in nga shkrues paralelë. Një stale lock pas crash-it nuk hiqet automatikisht; kërkohet operator, sepse nuk ka attestation për recovery.

## Rrjedha Quantum

### Project process entry

Quantum:

1. lexon frame-in;
2. rillogarit request SHA;
3. dekodon payload-in;
4. rillogarit payload SHA;
5. lidh project ID/trace/context me `ScientificProjectInput`;
6. vetëm pastaj thërret `run`.

### Digital Lab real

Për projektin real thirret:

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

Digital Lab nuk jep verdict. Ai prodhon TRL evidence, findings, test ID dhe trace.

### Same Untrust process

Projekti kalon organet ekzistuese:

```text
HPRO → PRO → NPRO → NPIM → SRK → PIM → APRO → MPRO → HCP
```

`action_state`, `action_mask`, `required_action_mask` dhe ledger-i mbeten kushti i Living Trust. Projekti nuk e zëvendëson këtë proces.

### Final package

Quantum ndërton `ScientificProjectWire`, llogarit E_Q dhe e vendos në `FinalEvidenceWire`. Paketimi PIM/NPIM/MPRO, PD/Spine dhe input SHA mbeten të pandryshuara në autoritet.

## Shadow verification

Shadow main kryen në rend:

1. decode të frame-it;
2. identity checks Quantum/Light;
3. finite checks;
4. FinalEvidence digest/replay;
5. input SHA;
6. project context/evidence SHA;
7. APUPK durable comparison;
8. PD Continuum dhe Spine 9 checks;
9. nine-organ ledger replay/cross-binding;
10. PIM/NPIM/MPRO checks;
11. ingest në core;
12. same `judge_supreme`.

### APUPK cross-check

Përpara core-it krahasohen:

```text
project_id
user_id
trace_id
revision
project_title
Light sovereign flags
stored content SHA-256
```

Quantum merr vetëm witness; nuk merr `&ShadowApupkMemory`, vault ose Knowledge.

### Novel factualization

Brenda `judge_supreme`:

- file kind rillogaritet nga magic bytes;
- ndërtohet `NovelEvidence`;
- për (Y,X)=(1,1) përdoret `ShadowEco::classify_with_factualization`;
- për çift jo-sovran nuk shpallet factual innovation;
- statusi bëhet derivat i verdict-it suprem dhe Novel evidence.

Statuset:

```text
(0,0)                         → RIGOROUS_NEGATIVE
(1,1) + factual innovation    → NOVEL_FACTUAL
çdo rast tjetër i projektit   → HOLD
pa projekt                    → NONE
```

Novel nuk është verdict paralel. Statusi futet në `SupremeVerdict`, V_S dhe Living Trust.

## Negative Knowledge

v1.6.3 ndan qartë:

```text
positive/hold/Novel → zero Negative Knowledge write
rigorous negative  → persist i detyrueshëm
```

Standalone `--negative-once` është `dev_harness` only. Production persist ndodh në `run_cycle`, pas verdict-it. Nëse negative e verifikuar nuk ruhet, Quantum nuk liron rrugën PD/iZ negative.

## Vula 500

Vula 500 vazhdon të lindë në Light. Projekti ruan flags-at në APUPK, witness dhe scientific package. Shadow kontrollon se:

- flags-at përputhen me APUPK;
- dekodohen në 500;
- hyjnë në Living Trust;
- statusi Novel nuk mund të zëvendësojë Vulën 500.

## PD Continuum dhe UI

Quantum eksporton handoff v1.6.3 me 45 fusha trupi + CRC. Fushat e reja:

- project evidence SHA;
- project ID;
- project context SHA;
- status;
- factualized;
- TRL;
- proof score;
- rejection.

Light pret 46 fusha të vulosura, pars-on 45 fushat e trupit dhe rillogarit:

- ProjectVerdict SHA;
- Living Trust;
- receipt binding;
- project status consistency.

Vetëm pastaj PD Light ia jep Nura-s dhe sinjalit emocional. PD Light nuk riprocedon Digital Lab ose Layers.

## Rregullimet konkrete të Cargo

### E0425

```text
run_lab_demo                  dev_harness
run_integrated_lab_demo       dev_harness
persist_negative              dev_harness
negative imports              dev_harness
```

Rruga reale shkencore nuk varet nga demo helper-i.

### E0063

Fixture-i `strong_pkg` mban:

```text
quantum_action_mask = REQUIRED_ACTION_MASK
quantum_required_action_mask = REQUIRED_ACTION_MASK
scientific_project = None
```

Kjo mbyll kontratën e testit pa dobësuar invariantin.

## Ndryshimet kryesore sipas skedarëve

### Light

- `gcl_project_contract.rs`: kontrata Project.
- `project_process_bridge.rs`: process transport dhe response binding.
- `sovereign_bridges.rs`: APUPK→Shadow→Quantum flow.
- `quantum_bridge.rs`: project payload.
- `pd_light.rs`: Novel status dhe Trust recomputation.
- `main.rs`: project route dhe wire count.

### Quantum

- `gcl_project_contract.rs`: kontrata Project.
- `bridge_light/mod.rs`: parse/validate project input.
- `main.rs`: project entry, Digital Lab, package, Shadow verification, PD handoff.
- `shadow_process_bridge.rs`: dev boundary closure.

### Shadow

- `gcl_project_contract.rs`: kontrata Project.
- `process_bridge.rs`: APUPK/project mediation.
- `shadow_apupk.rs`: durable context store.
- `sovereign_log.rs`: checked WAL append.
- `shadow_gj_legacy.rs`: project adjudication in supreme judge.
- `types.rs` dhe bridges: project evidence/result transport.
- `tests/integration.rs`: semantic fixture update.

### Contracts/UI/version

- `shadow-contracts`: protocol v8.
- manifests dhe Tauri configs: v1.6.3.
- UI merr statusin përmes PD Light, jo përmes një bypass Novel.

## Auditimi mbi stimulim

U simuluan:

- identitete të vlefshme dhe të pavlefshme;
- ownership swap;
- revision replay;
- content/context/payload SHA swap;
- ndryshim i hipotezës pas testit;
- magic-byte mismatch;
- action mask/ledger mungesë;
- SRK/PIM mismatch;
- Novel/Hold/RigorousNegative;
- Living Trust/status manipulation;
- APUPK fsync failure;
- writer parallel;
- negative persist failure;
- PD schema të vjetër dhe të ri.

Harta e plotë është në `V163_SIMULATION_MAP.md`.

## Provat e këtij ambienti

```text
Static architecture checks:     90 PASS / 0 FAIL
Rust files structurally scanned: 277
Cargo.toml parsed:              7
JSON parsed:                    3
C objects built:                18
C compilers:                    GCC + Clang
C warnings/errors:              0
Release manifest entries:       414 / 0 mismatch / relative only
Project contract ×3:            byte-identical
Living Trust contract ×3:       byte-identical
PD Continuum ×3:                byte-identical
PD Spine ×3:                    byte-identical
```

Këto janë prova statike/kontraktuale dhe C execution proof. Nuk janë Cargo proof.

## Kufijtë e mos-implementimit

### Wire authentication/encryption

Frame-i public vazhdon të ketë checksum ekzistues. SHA-256 mbron identitetin dhe riverifikimin e materialit, por nuk është MAC. Nuk u shpik HMAC/encryption pa:

- key authority;
- provisioning;
- storage;
- rotation;
- revocation;
- recovery.

### APUPK WAL

WAL është durable dhe CRC-guarded, jo MAC/encrypted. Nuk u shpik një çelës Shadow.

### Migration

WAL v1.6.2 nuk migrohet automatikisht. v1.6.3 përdor `shadow_apupk_v163.wal`; projektet e vjetra duhet të riregjistrohen. Pa schema migration të autorizuar, kjo është zgjedhja fail-closed.

### APUPK final status

Novel/Hold/Negative është në SupremeVerdict/LivingTrust/PD handoff. Nuk u krijua status event në APUPK WAL, sepse nuk kishte kontratë të autorizuar për semantikën, versionimin dhe konfliktin e statusit.

### Cargo

Ky ambient nuk kishte `cargo`, `rustc`, `rustfmt` ose PowerShell. `VALIDATE_V163.ps1` është release gate. Versioni mbetet:

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

## Artefaktet e provës

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

# Evolucioni v1.6.3 → v1.6.4 — UI minimale, TRL brenda GCL dhe Shadow shumëhapësh

## Vendimi arkitekturor

v1.6.4 e ndan qartë magazinën e projektit nga organet e reasoning/verifikimit:

```text
UI e vjetër
├── pranon upload-in e projektit
└── pasqyron emocionin e sistemit

Light
├── validon materialin e kufizuar
├── krijon trace/APUPK
├── prodhon kufirin GCL dhe Vulën 500
└── koordinon Shadow witness → Quantum process

Quantum
├── procedon vetëm nën GCL/PD/Spine 9
├── prodhon evidence TRL 0–3
└── dërgon paketën finale te Shadow main

Shadow
├── verifikon GCL/Spine/identitetin
├── verifikon SHA-256 dhe Vulën Light
├── verifikon magic bytes të provave
├── verifikon mbështetjen TRL
├── verifikon Y dhe X
└── vetëm pas (Y=1,X=1) mund të faktualizojë TRL4
```

Nuk u krijua një autoritet TRL. TRL mbetet evidencë brenda të njëjtit GCL process dhe nuk zëvendëson verdict-in suprem të Shadow.

## Old UI Minimal Boundary

**Theory name:** Old UI Minimal Upload and Emotion Boundary  
**Philosophy:** sipërfaqja nuk duhet të marrë përgjegjësi kushtetuese nga Light, Quantum ose Shadow.  
**Purpose:** të lejojë përdoruesin të dorëzojë projektin dhe të shohë gjendjen emocionale, pa bypass.  
**Mathematical model:** `UI(material) → Light intake`; `System state → EmotionalCommand → UI`.  
**Runtime flow:** `upload_project → light-platform --project-route-once`; `reflect_system_emotion` vetëm pasqyron output-in real.  
**Contracts:** `LightProjectIntakeRequestWire` nuk përmban `user_id`, `project_id`, timestamp, `contract_id`, `lgc_seal`, TRL ose verdict.  
**Code location:** `light/ui/src-tauri/src/main.rs`, `light/ui/src/main.js`, `light/ui/src/index.html`.  
**Version introduced:** v1.6.4.  
**Verification evidence:** guard-et statike në `VALIDATE_V164.ps1`; mungesë e thirrjeve direkte Shadow/Quantum në UI.  
**Current status:** `RUNTIME_CONNECTED_PENDING_CARGO`.

### Ndryshimi nga v1.6.3

U hoqën komandat placeholder:

```text
explore_input
get_output
upload_knowledge_dialog
ready_for_shadow = true
```

UI-ja e vjetër tani ekspozon vetëm:

```text
upload_project
reflect_system_emotion
```

Ajo nuk krijon `user_id`, `project_id`, timestamp, trace sovran, kontratë GCL, Vulë 500, TRL ose verdict. Identiteti, koha, trace-i dhe witness-i real lindin në Light/APUPK/Shadow.

## Light-Owned Project Intake

**Theory name:** Light-Owned Scientific Project Intake  
**Philosophy:** Light koordinon dhe ankoron; UI nuk shpall autoritet.  
**Purpose:** të krijojë rrugën e vetme nga upload-i te GCL.  
**Mathematical model:** `U_project + Light(APUPK,500) → ProjectContextWitness → Quantum(i₀_project)`.  
**Runtime flow:** `--project-route-once → APUPK prepare → Shadow register → witness SHA check → Quantum process`.  
**Contracts:** `LightProjectIntakeRequestWire`, `ProjectRegistrationRequestWire`, `ProjectContextWitnessWire`, `QuantumProjectExecutionRequestWire`.  
**Code location:** `light/src/project_process_bridge.rs`, `light/src/sovereign_bridges.rs`.  
**Version introduced:** v1.6.3; authority boundary corrected in v1.6.4.  
**Verification evidence:** UI authority fields removed; Light derives `GCL:SCIENTIFIC_PROJECT:V164` and seal material from the APUPK witness.  
**Current status:** `RUNTIME_CONNECTED_PENDING_CARGO`.

## GCL TRL Separation

**Theory name:** GCL-Bounded TRL Separation  
**Philosophy:** TRL mat pjekurinë e provës; nuk është ligj, verdict ose rrjedhë paralele.  
**Purpose:** të mbajë Quantum reasoning të ndarë nga Shadow factualization.  
**Mathematical model:**

```text
Quantum evidence: TRL ∈ {0,1,2,3}
Shadow factualization: TRL4 ⇔ GCL_bound ∧ evidence_verified ∧ Y=1 ∧ X=1 ∧ NovelProof
```

**Runtime flow:** `Quantum Digital Lab → ScientificProjectWire(TRL≤3) → ShadowLab → GeniusNovel → optional TRL4`.  
**Contracts:** `QUANTUM_MAX_TRL = 3`, `SHADOW_FACTUAL_TRL = 4`, protocol v9.  
**Code location:** `shadow-contracts/src/lib.rs`, `shadow/src/shadow_lab.rs`, `shadow/src/shadow_genius_novel.rs`, `shadow/src/shadow_eco.rs`.  
**Version introduced:** v1.6.4 as an explicit runtime boundary.  
**Verification evidence:** inbound TRL4 rejected by wire shape and by Shadow’s GCL stage; tests added.  
**Current status:** `IMPLEMENTED_PENDING_CARGO`.

## Shadow Multi-Stage Project Verification

**Theory name:** Shadow Multi-Stage Project Verification  
**Philosophy:** Shadow does not trust a ready-made project verdict; it verifies each boundary in sequence.  
**Purpose:** to avoid mixing identity, evidence, TRL and factualization in one opaque decision.  
**Mathematical model:**

```text
S1 = GCL_identity ∧ Spine_complete ∧ Seal500 ∧ SHA_canonical
S2 = declared_file_kind ≡ detected_magic_kind
S3 = ShadowLab(TRL≤3, confidence, reproducibility)
S4 = (Y,X)=(1,1) ∧ GeniusNovel(real documentation)
TRL4 = S1 ∧ S2 ∧ S3 ∧ S4
```

**Runtime flow:** `process_bridge validation → judge_supreme → verify_project_gcl_stage → verify_project_file_kinds → ShadowLab → GeniusNovel`.  
**Contracts:** same `ScientificProjectContext`, same PD GCL digest, same Spine completion, same witness and project evidence SHA.  
**Code location:** `shadow/src/process_bridge.rs`, `shadow/src/shadow_gj_legacy.rs`.  
**Version introduced:** existing stages consolidated and explicitly separated in v1.6.4.  
**Verification evidence:** GCL mismatch, zero Spine, invalid seal, noncanonical SHA and inbound TRL4 fail closed.  
**Current status:** `IMPLEMENTED_PENDING_CARGO`.

## Korrigjimet e kompilimit nga auditimi v1.6.3

### PD handoff

U korrigjua rendi i formatter-it 45-fushësh në Quantum dhe testin e PD Light:

```text
project_evidence_sha256 → {}
project_id              → {:016x}
project_context_sha256  → {}
```

Kjo mbyll `E0277 String: LowerHex` pa ndryshuar skemën semantike të parser-it.

### Shadow fixtures

`shadow/src/bridge/mod.rs` dhe `shadow/src/bridge/shadow_callable.rs` deklarojnë:

```rust
scientific_project: None
```

Kjo mbyll `E0063` pa shpikur një projekt në fixture-at legacy.

## Domain-et dhe protokolli

- `shadow-contracts::PROTOCOL_VERSION = 9`.
- `ESS_MAI_GCL_PROJECT_CONTEXT_V164`.
- `ESS_MAI_GCL_SCIENTIFIC_PROJECT_EVIDENCE_V164`.
- `ESS_MAI_GCL_SCIENTIFIC_PROJECT_VERDICT_V164`.
- `GCL_LIVING_TRUST_V164`.
- `GCL_LIVING_TRUST_TO_IZ_V164`.
- `ESS_MAI_FINAL_EVIDENCE_V164`.
- `PD_LIGHT_IZ_UI_CONTINUITY_V164`.

Kontratat `gcl_project_contract.rs` dhe `living_trust_contract.rs` mbeten byte-identike në Light, Quantum dhe Shadow.

## Provat e paketës

Artefaktet e v1.6.4:

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

## Statusi i versionit

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

Ky ambient nuk kishte `cargo`, `rustc` ose PowerShell. Për këtë arsye v1.6.4 nuk shënohet `VERIFIED` ose `RELEASED` nga Cargo pa ekzekutimin e `VALIDATE_V164.ps1` në Windows GNU / Rust 1.96.0.

---

# Evolucioni v1.6.4 → v1.6.5 — Project Workspace në Quantum dhe rruga legacy

## Vendimi arkitekturor

v1.6.5 nuk e kthen Quantum-in në magazinë dhe nuk krijon memory paralele. Ai shton vetëm një **orientues të kufizuar për pjesën Project**, pasi projekti është regjistruar në APUPK nga Light përmes Shadow main.

```text
Rruga default e projektit
UI upload
→ Light --project-route-once
→ APUPK + Shadow ProjectContextWitness
→ Quantum --project-workspace-once
→ Storage / Conversation / Storage+Conversation orientation

Rruga legacy shkencore
Light --project-route-legacy-once
→ Quantum --project-process-once
→ GCL / PD / Spine 9 / TRL 0–3 / Shadow verification
```

Inputi normal i përdoruesit, rrjedha stdin e Quantum, PD Continuum dhe urat e tjera nuk ridrejtohen në Project Workspace.

## Quantum Project Workspace Orientation

**Theory name:** Quantum Project Workspace Orientation  
**Philosophy:** projekti ka nevojë për një orientim të qartë drejt magazinës dhe bisedës, por orientimi nuk duhet të marrë autoritetin e GCL, tokenit ose Shadow-it.  
**Purpose:** të ndajë projektet nga inputet normale dhe nga procedimi i plotë shkencor.  
**Mathematical model:**

```text
P_valid = APUPK_bound ∧ project_id≠0 ∧ trace_id≠0 ∧ SHA_context ∧ SHA_request
Route(P) ∈ {STORAGE, CONVERSATION, STORAGE∧CONVERSATION}
Authority(Route(P)) = ∅
TokenMutation(Route(P)) = 0
```

**Runtime flow:** `--project-workspace-once → validate execution envelope → deserialize ScientificProjectInput → orient → SHA-256 record identities`.  
**Contracts:** përdor `QuantumProjectExecutionRequestWire` vetëm si envelope ekzistues; përgjigjja e re është `ESSMAI_Q_PROJECT_WORKSPACE_V165`.  
**Code location:** `quantum/src/project_workspace_router.rs`, `quantum/src/main.rs`.  
**Version introduced:** v1.6.5.  
**Verification evidence:** moduli nuk importon `LgcToken`, `LgcGate`, `CapHandle`, `token_forge`, `ForgeToken` ose `SEAL_*`; funksioni workspace nuk thërret `run`.  
**Current status:** `IMPLEMENTED_PENDING_CARGO`.

### Tri orientimet

```text
PROJECT_STORAGE
PROJECT_CONVERSATION
PROJECT_STORAGE_AND_CONVERSATION
```

Domain-et eksplicite `project-storage`, `project-chat` dhe `project-workspace` zgjedhin rrugën. Kur domain-i nuk është eksplicit, forma e materialit dhe prania e turnit të bisedës japin orientimin deterministik.

### Identitetet e rekordeve

Quantum prodhon:

```text
workspace_sha256
material_sha256
conversation_turn_sha256
```

Këto janë identitete domain-separated të rekordeve dhe **nuk janë token, receipt ose verdict**. Përgjigjja deklaron:

```text
authority=NONE
token_policy=UNCHANGED
legacy_route=--project-process-once
```

## Light Dual Project Route

**Theory name:** Light Dual Project Route  
**Philosophy:** Light mban një kufi APUPK/GCL të vetëm, pastaj zgjedh qartë destinacionin.  
**Purpose:** të mos kopjojë ose ndryshojë Vulën 500 midis rrugës së re dhe asaj legacy.  
**Runtime flow:**

```text
prepare_project_handoff_under_gcl
├── route_project_workspace_under_gcl
└── route_scientific_project_under_gcl
```

**Contracts:** të dyja rrugët përdorin të njëjtin `ProjectContextWitnessWire`, `content_sha256`, `GCL:SCIENTIFIC_PROJECT:V164` dhe `light_sovereign_flags`.  
**Code location:** `light/src/sovereign_bridges.rs`, `light/src/project_process_bridge.rs`.  
**Version introduced:** v1.6.5.  
**Verification evidence:** default flag `--project-route-once`; legacy flag `--project-route-legacy-once`; përgjigjja Workspace riverifikohet fushë për fushë.  
**Current status:** `RUNTIME_CONNECTED_PENDING_CARGO`.

## Pse GCL dhe tokenët mbeten V164

v1.6.5 nuk ndryshon materialin kushtetues të projektit, verdict-in, Living Trust ose iZ. Për këtë arsye nuk u ndryshuan:

```text
GCL:SCIENTIFIC_PROJECT:V164
ESS_MAI_GCL_PROJECT_CONTEXT_V164
ESS_MAI_GCL_SCIENTIFIC_PROJECT_EVIDENCE_V164
ESS_MAI_GCL_SCIENTIFIC_PROJECT_VERDICT_V164
GCL_LIVING_TRUST_V164
GCL_LIVING_TRUST_TO_IZ_V164
shadow-contracts PROTOCOL_VERSION = 9
```

Ndryshimi i këtyre domaineve vetëm për orientimin e workspace-it do të krijonte identitete të reja të Living Trust/receipt dhe do të rrezikonte vazhdimësinë e tokenëve. v1.6.5 shton vetëm:

```text
ESS_MAI_QUANTUM_PROJECT_WORKSPACE_V165
```

Ky domain nuk hyn në GCL token, ForgeToken, capability gate, receipt ose SupremeVerdict.

## Rruga legacy

Rruga e v1.6.4 nuk u hoq:

```text
light-platform --project-route-legacy-once REQUEST RESPONSE
quantum-platform --project-process-once REQUEST RESPONSE
```

Ajo vazhdon procedimin e plotë shkencor dhe është e ndarë nga porta Workspace. Funksioni `run_project_process_once` ruan sjelljen e mëparshme.

## Shadow dhe magazina

Shadow mbetet pronari i magazinës persistente APUPK. Rendi default është:

```text
Light identity + Vula 500
→ Shadow project-register-once
→ ProjectContextWitness
→ Quantum workspace orientation
```

Quantum nuk shkruan një vault të ri. Kjo mban një magazinë të vetme dhe shmang ndarjen e identitetit të projektit.

## Provat e v1.6.5

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

## Statusi i versionit

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

Ky ambient nuk kishte `cargo`, `rustc` ose PowerShell. `VALIDATE_V165.ps1` është porta ekzekutive që duhet të provojë Cargo-green dhe zero warning debt në Windows GNU / Rust 1.96.0.
