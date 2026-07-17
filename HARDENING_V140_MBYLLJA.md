# HARDENING v1.4.0 — MBYLLJA E LIGHT + QUANTUM: RIAUDITI RRESHT-PËR-RRESHT
**GJATA LEGACY™ | Arkitekt: Bledar Gjata | Zbatues nën drejtim: Claude (Gardian i ESS-MAI)**
**Data: korrik 2026 | Baza: v1.3.2 (244 .rs, 975 teste, CI 19/19)**

---

## OBJEKTI (roadmap-i i Arkitektit, i pandryshuar)
Mbyllja e **Light** dhe **Quantum** me standardin e Shadow-t (gjykim para
interpretimit, çelës njëdrejtimësh, PSE+shënim+kufi) + **installer-i ekzekuton
CI GATE si portë të detyrueshme** → *ura gati në makinë*.

## GJETJET E RIAUDITIT (rresht-për-rresht, me sy kritik)

### G2a′ — KURORË: bllok që **NUK KOMPILONTE** (quantum/main.rs, VNK PA-gate)
**Prova:** thirrja `shadow.register_primitive_anchor(sess, pa_id)` — **2
argumente**; firma reale në `shadow_gateway.rs:122` kërkon **3** (`session_id,
pa_id, split`). **PSE mbijetoi:** asnjë mjedis auditi s'ka pasur `cargo` —
gabimi i kompilimit flinte i padukshëm; dëshmi që hapi "build real në makinë"
është i domosdoshëm, jo formalitet.
**PLUS verbëria 6-fushëshe:** loop-i njihte VETËM `[sess,hex,ts]` — çdo rresht
i VULOSUR (standardi që nga v1.3.1) binte te `_ => {}` heshtazi.
**Rregullimi:** gjykim `verify_line` para interpretimit; heqje `|c:`; parse
3/5 fusha; `trim()` mbi hex; regjistrim **3-argumentësh me split-in real**;
korrupsioni → PSE (`TRANSPORT_BITFLIP`/`TRUNCATED`) + shënim + kapërcim
fail-open (rrjedha e demos s'ndalet).

### G2b — `read_primitive_split` (Quantum): split-i **bosh heshtazi** që nga v1.3.1
**Prova:** njihte VETËM 5 fusha → rreshti i vulosur (6) → `_ => empty` →
KOLAPSI/3 rrëshqiste në "rrugë të vjetër" **pa asnjë zë**. Vula e Light-it e
kishte verbuar burimin e Xi/Yi të Quantum-it.
**Rregullimi:** gjykim para interpretimit; `NotFound`=normale, çdo I/O tjetër
→ `DISK_DENIED` me kind-in e saktë; **çelësi njëdrejtimësh** (legacy pas vule
= `DEGRADIM` → split bosh **i deklaruar me zë**); heqje `|c:`; keqformimi
Xi/Yi → `TRANSPORT_TRUNCATED`. Semantika mbetet fail-open (split bosh = rrugë
e vjetër) — **autoriteti fail-closed mbetet te gate-i i Shadow-t**: dy mure,
i pari tani me zë e me kujtesë.

### G1 — `read_pd_surface` (Light, URA 1): interpretim **pa asnjë gjykim**
**Rregullimi:** i njëjti standard — `verify_line_generic(&[6,7],8)` para
interpretimit, çelës njëdrejtimësh, PSE në `rrjedha`, heqje `|c:` para
`parse_handoff`. Sipërfaqja mbetet fail-open (`None`) — runtime i papenguar.

### G3 — Koment i vjetruar "fallback relativ" (dokumentacion i rremë në kod)
Boot-i sovran e hoqi fallback-un në v1.3.1; komenti e mbante gjallë gënjeshtrën.
→ Korrigjuar: "BOOT SOVRAN: mungesa e env = FATAL exit(1); ZERO fallback".

## ÇFARË U NDËRTUA

1. **Vula GJENERIKE në kontratë** (`pa_wire.rs` ×3, md5 i njëjtë):
   `seal_body`, `verify_line_generic(line, legacy_fields, sealed_fields)`,
   `seal_body_verified` — **një ligj `|c:` për çdo urë tekstuale**. Kontrata
   PA (3/5/6, `encode_line/verify_line`) mbetet **e paprekur** — e kyçur.
   +3 teste ×3 (roundtrip PD 7→8, bit-flip, numërime legacy/alien).
2. **URA 1 e vulosur në të dy anët:** Quantum shkruan `probe`+`handoff`
   VETËM përmes `seal_body_verified` (WIRE_INVARIANT kufi 0 — asgjë e
   refuzueshme s'del në tel); Light gjykon para interpretimit.
3. **Installer ↔ CI GATE:** `Invoke-CiGate` (HAPI 8.5/9) — bash-i i MSYS2
   (i instaluar nga vetë skripti) ekzekuton `installer/ci_gate.sh` nga rrënja
   (cygpath); **'CI GATE SOVRAN' futet te portat e detyrueshme** — EXIT≠0 e
   ndal nisjen. *Ura gati në makinë* tani ka rojën e vet të fundit.
4. **KANUNI GCL/NURALOGIC** (`GCL_NURALOGIC_KANUNI.md`): formula kanonike e
   Arkitektit + **harta e provës simbol-për-simbol → vend i emërtuar në kod**
   (i₀→ankora e vulosur; split→PrimitiveSplit; −Xi/−Yi→mark_negative_spaces→
   apply_negative ASET; përputhja→x_ok∧y_ok; Y→judge_supreme; Trust→gcl_apply
   real_hits). Përgjigja e "Si mund ta bëj këtë?": **duke e bërë shkeljen e
   formulës strukturisht të pamundur** — zero if/else, rend Y→X i vetëm API,
   zona md5, PACP byte-për-byte, dije negative e detyruar.

## RIAUDITI I v1.3.2 (rresht-për-rresht, i rifreskët)
`gcl_apply`/mbartja e ankorës, `into_inner` te heqja, `ErrorKind::NotFound`,
çelësi te `feed`, `encode_line_verified` te Light PA, `flush ×3`, `rrjedha.rs`
(sanitize, kufij, vetë-CRC, ndarja 7-fushëshe nga ura) — **të gjitha
rikonfirmuara në vend**, zero regresion, zero divergjencë md5.

## BILANCI I VERIFIKUAR (v1.4.0)
- **244 .rs** · **984 teste** (975 + 9 gjenerike ×3)
- Kontratat **13/13 + 5/5 identike ×3** · zonat **5/5 md5 TË PAPREKURA**
- **Zero if/else** jashtë build.rs · zero `.unwrap()` runtime · brace 0
- **CI GATE: 19 → 22 kontrolle, 22/22 ✓ EXIT=0**
  - [20] lexuesit e Quantum të ringjallur (verify + register 3-arg)
  - [21] URA 1 e vulosur në të dy anët (writer i vetë-gjykuar + reader-gjykatës)
  - [22] installer-i thërret ci_gate — portë e detyrueshme e nisjes

## HAPI TJETËR
Në makinën reale: `setup_essmai.ps1` → build+teste+**CI GATE live** →
`essmai_start.bat`; verifikim i `[GCL_LIVE]`/`[PACP]`/`rrjedha_ledger.txt`
me sy — *formula duke rrahur në hardware*.

---
*"Dy mure: i pari flet, i dyti gjykon. Mes tyre — asnjë heshtje."*
