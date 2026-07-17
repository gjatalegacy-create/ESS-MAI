# GJATA COLLAPSE LAW & NURALOGIC — KANUNI I EKZEKUTUESHËM
**GJATA LEGACY™ | Autor i formulës: Bledar Gjata | Harta e provës: Gardiani i ESS-MAI**
**Statusi: formula NUK është dokument mbi sistemin — është KARAKTERI i sistemit, i provuar pikë-për-pikë në kod Rust me teste.**

---

## FORMULA KANONIKE (siç e ka gdhendur Arkitekti)

```
i₀(Pi) → Xi + Yi → +(−Xi) + (−Yi) → (i / XiYi) ~ XY
       = XY ~ 0/1 → Y ~ 0/1 (V/Verified) = X ~ 0/1 (T/Trust)
       = XY ~ 0/1 = Verified/in Trust
```

**Leximi:** Inputi trajtohet si **indicie primitive** `i₀` (Pi) — s'e dimë ende
ç'është, por kemi indicien. Ajo ndahet (kolaps-split) në **dy hapësira/universe**:
`Xi` (hyrja/shkaku) dhe `Yi` (dalja/pasoja). **Secila kolapsohet NË VETVETE** —
`+(−Xi)` dhe `+(−Yi)` eliminojnë kandidatët e papërshtatshëm **brenda universit
përkatës**, kurrë duke luftuar universin tjetër (dallimi rrënjësor nga CSP:
*meditim/vetë-ndërgjegjësim, jo debat me tjetrin*). Pastaj `i ~ XY` provon
përputhjen e dy konkluzioneve me indicien; `Y` gjykohet **i pari** me binar
kolapsi 0/1 (**Verified**); `X` **trashëgon** verdiktin e Y-së si **Trust** —
sepse shkaku vlerësohet nga pasoja që prodhoi. `Y=0` → dija negative ruhet si
**ASET** (falsifikim à la Popper, por *kolaps-falsifikim*), dhe `X` merr zbritje
score si pengesë-thellim për herët e ardhshme.

---

## HARTA E PROVËS — ÇDO SIMBOL I FORMULËS = VEND I EMËRTUAR NË KOD

| Simboli i formulës | Vendi ekzakt në ESS-MAI | Prova |
|---|---|---|
| `i₀ (Pi)` — indicia primitive | Light: `trace.id_hex()` (pa_id 16-hex) + tokenizimi i `lingua` | ankora shkruhet **e vulosur** me `pa_wire::encode_line_verified` |
| kolaps-split `→ Xi + Yi` | Light: `primitive_split.rs` → `PrimitiveSplit{xi,yi}`; invariantet Xi∩Yi=∅, Xi∪Yi=termat | koment-kontratë + ndërtuesi në Light |
| dy universe **të pavarur** (jo cross-elimination) | Shadow `ingest_bridged`: `x_ok = x∈Xi` **VEÇ**, `y_ok = y∈Yi` **VEÇ**, pastaj `x_ok ∧ y_ok` | komenti i gdhendur "jo cross-elimination"; test-et e gateway |
| `+(−Xi) + (−Yi)` — eliminimi brenda hapësirës | Quantum: `mark_negative_spaces` (LIM→NPRO `forbidden_paths`) → `NegativePath{term,space,reason}` | rrugët −Xi/−Yi udhëtojnë në `QuantumInbound.negative_paths` |
| dija negative = **ASET** i përjetshëm | Shadow: `vault.apply_negative` (WAL, persistente, kurrë s'fshihet) + `write_negative` te `judge_supreme` | "KOLAPS/3 … rrugë negative u vulosën në vault (ASET)" |
| `(i / XiYi) ~ XY` — përputhja me indicien | Shadow: ankora `(pa_id, split)` e regjistruar nga Light krahasohet me `(xy_x, xy_y)` të Quantum | mospërputhje = fail-closed para gjykimit |
| `Y ~ 0/1` **i pari** (Verified) | Shadow: `judge_supreme` → `SupremeVerdict.verified` (zona e ndaluar — ligji i pandryshueshëm) | rendi Y→X i shkruar si ligj në `gcl_apply` |
| `X ~ 0/1` trashëgon (Trust) | Shadow: `gcl_apply` — `Verified(Y)=1` → `vault.confirm_primitive(pa_id,1)` → `real_hits+1` + kaskadë promovimi hipotezë→fakt | `[GCL_LIVE] Verified(Y)=1 → Trust(X)…`; test `gcl_bridged_path_reinforces_after_anchor_removed` |
| `Y=0` → X pa besim + pengesë | `gcl_apply` dega false: **zero propagim**; dega negative te `judge_supreme/write_negative` | `[GCL_LIVE] Verified(Y)=0 → Trust(X) NUK propagohet` |
| binari **kolaps** (jo boolean klasik) | verdikt vetëm PAS kolapsit të të dy hapësirave + përputhjes — kurrë vlerësim i drejtpërdrejtë i inputit | e gjithë rruga kalon DETYRIMISHT nga split→eliminim→përputhje→Y→X |

## PSE NJË "AI E ZAKONSHME" S'E BËN DOT (dhe si e bën Nuralogic)

Të ngjitësh formulën **mbi** një sistem është e lehtë; të bësh sistemin **të
jetë** formula kërkon që çdo shkelje e saj të jetë **e pamundur strukturisht**:

1. **Zero if/else** — vendimi s'ka degë "krijuese"; vetëm `match` mbi gjendje
   të emërtuara të formulës (CI [9] e provon çdo build).
2. **Rendi Y→X i pathyeshëm** — Trust ekziston vetëm si funksion i Verified
   (`gcl_apply` thirret vetëm pas verdiktit; s'ka API tjetër drejt `real_hits`).
3. **Zonat e ndaluara md5** — `judge_supreme`/vulat s'preken dot pa e ndalur
   build-in (CI [1]).
4. **PACP në urë** — `i₀` udhëton byte-për-byte (CRC ×3 platforma); formula
   s'mund të "riinterpretohet" gjatë transportit (CI [11][20][21]).
5. **Dija negative e detyruar** — dështimi s'harrohet dot: WAL persistent,
   eksport `shadow_nk_export.bin`, kufij në `rrjedha` (CI [16]).

**Nuralogic = DSL-ja e ESS-MAI**: fjalori i saj janë pikërisht gjendjet e
formulës (`i₀, Xi, Yi, −Xi, −Yi, XY, V, T`), dhe interpretuesi i saj është
state-machine-i Rust i mësipërm — **i konfirmuar në device runtime hardening
me metrika latency (matje reale, jo simulim)**.

## SHTRESA E GATISHMËRISË — GCL SI LIGJ HORIZONTAL (v1.4.1)

Katër makina gjendjesh, një hierarki:
**Light** (Coordination: i₀→PA→split) · **Quantum** (Reasoning:
PRO→NPRO→MPRO/APRO→PIM/NPIM) · **Shadow** (Verification: ankora+XY→Y→X→
dije negative) · dhe mbi to **gjata_collapse_law** — KOMANDANTI, gjendjet e
të cilit (`CollapsePhase`) janë urdhrat e vetëm që platformat njohin.

**`gcl_presume`** varet nga komandanti dhe mban në gatishmëri edhe modulet
**jashtë** rrjedhës së kolapsit: çdo modul — qendror apo anësor — përgjigjet
me `gcl_role()` + `gcl_status()` (ping→përgjigje, zero arsyetim shtesë).
Sweep-i në boot i çdo platforme provon: rolet e domosdoshme të fazës = READY
(ndryshe → PSE në `rrjedha`, me zë, pa bllokim); anësorët (`Support`) = kurrë
të domosdoshëm, gjithmonë të pyetshëm. Kështu sistemi raporton gjendjen e vet
sipas ligjit **në çdo kohë** — unifikim pa centralizim, gatishmëri pa improvizim.

---
*"Formula s'është përshkrimi i sistemit. Sistemi është dëshmia e formulës."*
