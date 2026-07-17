# ESS-MAI v1.6.2 — Untrust Start to End

## Scope

Ky version mbyll Fazën 1 të Vulës së Gjallë si provë reale të përfundimit të organeve dhe zëvendëson identitetet e dobëta FNV të receipt-it dhe TokenForge me SHA-256 real.

## Implemented

- `runtime_pulse::begin_cycle()` zerohet si gjendje strukturore Untrust.
- HPRO, PRO, NPRO, NPIM, SRK, PIM, APRO, MPRO dhe HCP derdhin materialin e tyre kanonik vetëm pasi ekzekutohen.
- U përcaktua rendi kanonik real: `HPRO → PRO → NPRO → NPIM → SRK → PIM → APRO → MPRO → HCP`.
- TokenForge mbetet dëshmitar runtime dhe nuk ndot `action_state` e arsyetimit.
- Final evidence wire bart fjalët kanonike të secilit organ, jo një kontribut të gatshëm.
- Shadow rillogarit kontributin, fold-in, maskën, rendin dhe i kryqëzon rezultatet me PIM/NPIM/MPRO, kandidaturat PRO, HPRO dhe HCP.
- Living Trust kërkon maskën e saktë të nëntë organeve.
- `VerificationReceipt` u migrua nga FNV64 në SHA-256 kanonik 32-byte/64-hex.
- Receipt-i lidhet me SHA-256 e plotë të Living Trust.
- `token_forge` u migrua nga FNV64 në SHA-256 32-byte.
- Wire protocol u ngrit në versionin 5.
- Versionet Cargo/UI u ngritën në `1.6.2`.
- Testet stateful të `runtime_pulse` dhe `token_forge` serializohen me mutex vetëm nën `#[cfg(test)]`, që Cargo paralel të mos prodhojë dështime të rreme pa ndryshuar runtime-in.

## Preserved

- GCL mbetet autoritet mbi gjithë sistemin.
- Layer 1/2/3 mbeten thellim i procedimit nën GCL.
- PD Quantum mbetet procedues; PD Light mbetet korrier kontekstual.
- Shadow main mediation mbetet e paanashkalueshme.
- Legacy Shadow mbetet vëzhguesi paralel i evolucionit drejt Legacy.
- PIM/NPIM propozojnë dhe paketojnë; Shadow verifikon dhe Matrix vendos Knowledge.

## Release status

- Static verification: passed in packaging environment.
- Cargo build/check/test/clippy: pending execution through `VALIDATE_V162.ps1` on Windows GNU.
