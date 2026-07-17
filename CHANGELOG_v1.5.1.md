# ESS-MAI v1.5.1 — Static-Evidence Corrections

## Baseline

Kjo release ndërtohet mbi `ess_mai_v1_5_0_PATCHED.zip/ess_mai`. ZIP-i origjinal v1.5.0 ruhet si rollback.

## Korrigjime me provë konkrete

1. **Quantum test/runtime correctness**
   - RAM floor llogaritet me `u64 / 10`, jo me `0.10f32`.
   - Katër fixtures LIM plotësojnë kontratën reale pa ulur pragjet.
   - Testi NPIM respekton semantikën: score i lartë = besueshmëri e lartë për eliminim negativ.
   - First Guardian izolon state-in global vetëm nën `#[cfg(test)]`.

2. **Shadow FFI safety**
   - Katër hyrjet publike që mund të lexojnë raw pointers janë `unsafe extern "C"` me kontratë `# Safety`.
   - ABI dhe emrat e simboleve C mbeten të pandryshuar.
   - `sovereign_validate_and_write` kthen `-6` kur vault-i mungon dhe `-7` kur persistimi dështon.
   - Lineage regjistrohet vetëm pasi shkrimi persistent ka përfunduar.
   - Testet e payload-it bosh presin `-5`, sipas kontratës së dokumentuar.

3. **Shadow persistence semantics**
   - WAL dhe SovereignLog deklarojnë shprehimisht `.truncate(false)`.
   - Riparimi i bishtit kryhet vetëm nga `set_len(good_len)` pas parse-it.

4. **Binary decision counting**
   - Refuzimet sovrane numërohen si `usize`, pastaj normalizohen në score.

## Të paprekura

- Arkitektura Light → Quantum → Shadow.
- Formula dhe pragjet NPRO/NPIM.
- `pub use`, seals dhe wire formats.
- LIM runtime.
- UI/Tauri dhe installer/CI.
- Cargo profile policy; profile-t child mbeten për vendim të veçantë sepse Cargo i injoron në workspace dhe `panic` nuk mund të ruhet per-package me overrides.

## Status validimi

Ky mjedis nuk ka Rust/Cargo. Janë kryer validime statike, hash-e, parse TOML/JSON dhe kontroll i scope-it. Ekzekuto `VALIDATE_V151.ps1` në Windows për vulën dinamike.
