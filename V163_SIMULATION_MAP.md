# v1.6.3 — Harta e stimulimit para implementimit

## Skenarët bazë

| ID | Stimulimi | Pritja kushtetuese |
|---|---|---|
| S01 | projekt i ri, ID/user/title/content/V500 valid | Shadow WAL durable, witness, Quantum process |
| S02 | project_id=0 | refuzim Light/Shadow |
| S03 | user_id=0 | refuzim |
| S04 | trace nuk lind nga ID+user+title | refuzim para WAL |
| S05 | Vula 500 e gabuar | refuzim para WAL |
| S06 | progress NaN/Infinity | refuzim para WAL |
| S07 | i njëjti project_id me user tjetër | ownership mismatch, zero WAL |
| S08 | riregjistrim i njëjtë | revision rritet, witness i ri |
| S09 | revision e vjetër në Quantum | Shadow replay/stale refuzim |
| S10 | content SHA ndryshe | Light ose Shadow refuzon |
| S11 | context SHA ndryshe | Light/Quantum/Shadow refuzon |
| S12 | payload SHA ndryshe | Quantum refuzon pa procedim |
| S13 | response e një kërkese tjetër | Light refuzon request SHA |
| S14 | pyetja/hipoteza ndryshohet pas TRL | evidence SHA mismatch |
| S15 | skedar deklarohet image por magic bytes tjetër | Shadow refuzon |
| S16 | mungon një nga 9 organet | action mask/replay refuzon |
| S17 | SRK proof chain nuk lidhet me PIM | Shadow refuzon |
| S18 | (Y,X)=(1,1), TRL4 + prova Novel | NOVEL_FACTUAL + constructive Trust |
| S19 | (Y,X)=(1,1), Novel proof e pamjaftueshme | HOLD + constructive Trust |
| S20 | (Y,X)=(0,0) | RIGOROUS_NEGATIVE + negative persistence |
| S21 | mixed pair | pa Living Trust; cikli nuk lirohet |
| S22 | status Novel ndryshohet në wire | verdict SHA/Living Trust mismatch |
| S23 | full SHA ka prefix të njëjtë por pjesë tjetër ndryshon | Living Trust mismatch; u64 nuk mjafton |
| S24 | APUPK fsync dështon | zero witness |
| S25 | dy Shadow project processes paralel | një fiton lock, tjetri fail-closed |
| S26 | stale lock pas crash | ndalim operatori, jo recovery i hamendësuar |
| S27 | positive/hold me NPIM blob | zero Negative Knowledge write |
| S28 | negative persist dështon | Quantum ndalon PD/iZ negativ |
| S29 | handoff PD me 35/37 fusha të vjetër | Light refuzon |
| S30 | handoff v1.6.3 45+CRC | Light rillogarit Trust dhe dorëzon |

## Kryqëzimet e hetuara

1. Light APUPK identity × Shadow durable memory.
2. Shadow witness × Light content SHA.
3. Light payload × Quantum input parser.
4. Digital Lab material × project evidence SHA.
5. Project evidence × FinalEvidence digest.
6. FinalEvidence × nine-organ Untrust replay.
7. APUPK revision/title/owner × scientific package.
8. NovelEvidence magic bytes × declared types.
9. Novel status × same SupremeVerdict.
10. Project verdict SHA × Living Trust.
11. Living Trust × VerificationReceipt.
12. Receipt × PD output/iZ/next i0.
13. PD handoff × Light/Nura/UI.
14. Negative status × Negative Knowledge persistence.
15. Shadow main process × internal lib/core.

## Rezultati i stimulimit

Implementimi u lejua vetëm për pikat me kontratë ekzistuese: APUPK, Digital Lab, FinalEvidence, Untrust ledger, GeniusNovel, ShadowEco, Living Trust, Receipt dhe PD Light. HMAC, key management, WAL migration dhe APUPK final-status event u lanë jashtë sepse nuk kishin autoritet konkret.
