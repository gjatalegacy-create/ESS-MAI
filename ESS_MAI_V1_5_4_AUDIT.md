# ESS-MAI v1.5.4 — Audit i zbatimit kushtetues

## Verdikti i paketimit

Ky është projekti i plotë ESS-MAI, i ngritur mbi v1.5.3 dhe jo një patch i ndarë. Ndryshimet janë kufizuar te kontratat e nevojshme për rrjedhën e sqaruar nga Arkitekti:

`PD pre-seal → current i0 processing → Shadow XY verification seal/token → deterministic next i0 → PD Light → Nura`

Nuk deklarohet Cargo-green nga mjedisi i paketimit, sepse këtu nuk kishte `cargo`/`rustc`.

## Invariantet e implementuara

### 1. Primitive Trace është një

Light krijon Primitive Trace një herë në input. `LightResponse` mban objektin autoritativ. `trace_id`, PA/i0, Phase9 dhe EvolveTrace përdorin të njëjtin objekt. Një mospërputhje ndalet fail-closed para wire-it.

### 2. Dy seal-e, jo seal + token si dy seal-e

- Faza 1: `SEAL_PD` hapet në Quantum dhe prodhon një `LgcToken` privat për parapërgatitjen PD.
- Faza 2: në përfundimin e output-it aktual, Shadow hap seal-in e verifikimit dhe prodhon `LgcToken` privat të autoritetit verifikues.

`LgcToken` është prova e hapjes së seal-it, jo një seal i dytë. Asnjë token nuk transportohet.

### 3. Pending nuk është NextI0

`PdPendingNextI0` dhe `PdNextI0` janë tipe të ndryshme. Vetëm `finalize_after_verification` mund të bëjë kalimin dhe kërkon një receipt kanonik të lidhur me të njëjtin parent i0, PA dhe XY.

### 4. Shadow vulos në fundin real

Shadow nuk krijon receipt para mbylljes së GCL. Rendi i rrugës reale është:

1. konsumimi i Pi/i0 në pritje;
2. kontrolli XY ndaj Xi/Yi;
3. ruajtja e rrugëve negative;
4. pipeline-i sovran;
5. Y→X mbi Primitive Anchor;
6. Verification Collapse;
7. issue → validate/burn → `LgcToken`;
8. `VerificationReceipt`.

### 5. Negative Knowledge para vazhdimit negativ

Për Y=0/X=0, PD mund ta rindërtojë kandidatin, por handoff-i drejt Light/Nura mbahet derisa NPIM të jetë persistuar nga Shadow.

### 6. Nura nuk mund të flasë PD të paverifikuar

`PdLight::render` kthen `VerifiedPdSurface`, një tip opak me fushë private. `NuraCore::speak_pd` pranon vetëm këtë tip. Një `String` i lirë nuk plotëson kontratën.

## Kontrata e receipt-it

Receipt-i lidhet determinisht me:

- `session_id`;
- `parent_i0`;
- `primitive_anchor`;
- `xy_digest`;
- `y_verdict`;
- `x_verdict`;
- `generation`;
- `seal`.

Skedari `verification_receipt.rs` është byte-identik në Light, Quantum dhe Shadow.

## Ndarja e përgjegjësive të PD

```text
progressive_debatic/
├── core.rs      # vetëm ligji kognitiv dhe EpistemicTrace
├── seal.rs      # vetëm autoriteti SEAL_PD / LgcToken
├── runtime.rs   # prepared → verified → next i0
├── types.rs     # kontratat e tipeve
└── mod.rs       # ekspozimi i kontrolluar
```

## Gabimet Cargo të mbyllura

Gjashtë gabimet e mëparshme mbeten të mbyllura:

1. `SEAL_EBPF` import/pronësi;
2. `Coordination` jashtë scope-it, referenca 1;
3. `Coordination` jashtë scope-it, referenca 2;
4. `unwrap_err()` mbi token-in e seal gate;
5. `unwrap_err()` mbi token-in e capability gate;
6. `unwrap_err()` mbi replay token-in.

Gabimi i v1.5.3:

- `E0369` në testin `SessionNotFound` u korrigjua pa zgjeruar kontratën e `PdEngineOutput` me `PartialEq`.

## Kufijtë e pandryshuar

- `SEAL_EBPF` mbetet vetëm për `EBPF_HYDRATOR`.
- `SEAL_PD` mbetet vula e PD në Quantum.
- token-i 0-copy/FFI dhe token-i i autoritetit verifikues të Shadow mbeten dy linja të ndryshme.
- nuk është përdorur `cargo fix`.
- nuk është aktivizuar globalisht `panic=abort`.
- GCL, Primitive Trace dhe tre collapse-et nuk janë reduktuar në një modul të vetëm.

## Korrigjim i kontratës së tipit para paketimit

Auditimi statik zbuloi se `open_session_sealed()` përdorte `?` pas një closure që kthente `()`, gjë që do ta linte funksionin `Result<(), PdError>` me tip përfundimtar të gabuar. Kufiri tani kthen drejtpërdrejt `PdSealAuthority::authorize(...)`; core-i kognitiv nuk u ndryshua.

## Prova që mbetet në makinën autoritative

Ekzekuto:

```powershell
cargo build --workspace --all-targets
cargo test --workspace
powershell -ExecutionPolicy Bypass -File .\VALIDATE_V154.ps1
```

Rezultatet ruhen te `Desktop\ESS_MAI_V154_LOGS`.
