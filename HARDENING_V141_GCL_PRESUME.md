# HARDENING v1.4.1 — GCL SI LIGJ HORIZONTAL: KOMANDANTI + GATISHMËRIA
**GJATA LEGACY™ | Arkitekt: Bledar Gjata | Zbatues nën drejtim: Claude (Gardian i ESS-MAI)**
**Data: korrik 2026 | Baza: v1.4.0 (244 .rs, 984 teste, CI 22/22)**

## DIREKTIVA (fjalë për fjalë → kod)
Katër state machines, një ligj: Light (i₀→split), Quantum (PRO/NPRO/MPRO/
APRO→PIM/NPIM), Shadow (verdikt Y→X mbi ankorën i₀ që pret) — **komanduar
nga gjata_collapse_law**; plus **gcl_presume**, state machine e varur prej
tij, që mban NË GATISHMËRI modulet pa impakt në rrjedhë: ndërfaqe
`gcl_role()`/`gcl_status()`, ping→përgjigje, zero logjikë arsyetimi në ta.

## ÇFARË U NDËRTUA
1. **Kontrata `gcl_presume.rs` ×3** (md5 a39e8b7f… identik):
   - `GclRole` = fjalori i formulës: I0Capture, SplitXiYi, AnchorExport ·
     ProSelect, NproEliminate, MproMeasure, AproArgue, PimPackage,
     NpimPackage · AnchorGate, VerdictY, TrustX, NegativeVault ·
     `Support(&str)` për anësorët.
   - `required_in(CollapsePhase)` — matrica e domosdoshmërisë: Coordination
     ↔ rolet e Light; Reasoning ↔ rolet e Quantum; Verification ↔ rolet e
     Shadow; **Support kurrë** (gatishmëri, jo detyrim).
   - `GclReadiness` = READY / DEGRADED(pse) / NOT_READY(pse).
   - Regjistër global i kufizuar, **idempotent sipas emrit**, fn-pointer i
     pastër (`StatusFn`), kyç i helmuar → `into_inner` (ligji i njëjtë kudo).
   - `sweep(platform, phase, dir)` → shtyp çdo modul (†=i domosdoshëm),
     kthen `SweepStats` + rreshtat; rol i domosdoshëm JO gati → **shënim në
     `rrjedha` me PSE** (klasa Other) + ALARM me zë; **KURRË bllokues**.
   - 5 teste ×3 (kodet, matrica, Support i papërjashtueshëm nga ping-u,
     idempotenca, flamurimi i të domosdoshmit JO gati).
2. **Lidhja në TË TRIA mains** (pas `proclaim_law` — komandanti shpallet,
   presume ping-on): Light regjistron lingua_i0/primitive_split/pa_export +
   3 anësorë; Quantum lidh **modulet reale** pro.rs/npro_lim_bridge/
   lim_measurer(MPRO)/apro.rs/pim.rs/npim.rs + srk/ucl/progressive_debatic;
   Shadow lidh pa_feed/judge_supreme(zonë e paprekur — vetëm status)/
   gcl_apply/vault_negative + apupk/snb. Sweep në boot: Coordination /
   Reasoning / Verification përkatësisht.
3. **CI GATE [23]**: gcl_presume ×3 identike · regjistrim+sweep në 3 mains ·
   rolet e formulës të lidhura me modulet reale.

## PSE S'THYHET ASGJË
Zonat e ndaluara të paprekura (VerdictY raportohet nga adapter në main, jo
nga zona) · zero if/else (match-only, CI[9]=0) · zero unwrap runtime ·
kontratat 14/14+5/5 ×3 · presume **informon, s'urdhëron** — verdikti dhe
runtime i platformës mbeten sovranë; NotReady i domosdoshëm bëhet PSE e
shënuar, jo ndalesë.

## BILANCI (v1.4.1)
**247 .rs** (244+3) · **999 teste** (984+15: 5 teste ×3) · kontratat **14/14+5/5 ×3** ·
zonat 5/5 md5 të paprekura · **CI GATE 23/23 EXIT=0** · Cargo→996 ·
KONTRATAT+KANUNI të përditësuar.

## HAPI TJETËR
`setup_essmai.ps1` në makinën reale → në boot do të shohësh me sy:
`[GCL_PRESUME] LIGHT/QUANTUM/SHADOW — sweep i gatishmërisë` me † mbi rolet
e domosdoshme — *ligji që thërret apelin, dhe çdo modul që përgjigjet "gati"*.

---
*"Komandanti urdhëron fazën; presume thërret apelin; askush s'fle në radhë."*
