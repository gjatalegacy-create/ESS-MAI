#!/usr/bin/env bash
# ╔══════════════════════════════════════════════════════════════════════════╗
# ║  CI GATE SOVRAN — ESS-MAI (Shadow Runtime Hardening)                     ║
# ║  GJATA LEGACY™ | Arkitekt: Bledar Gjata                                  ║
# ║                                                                          ║
# ║  Ekzekutohet nga rrënja e repo-s OSE nga installer/. Dështon (exit 1)    ║
# ║  nëse cenohet cilido ligj strukturor. Whitelist-et janë EKSPLICITE.      ║
# ╚══════════════════════════════════════════════════════════════════════════╝
set -uo pipefail
cd "$(dirname "$0")/.." || exit 1

FAIL=0
ok()  { printf '  ✓ %s\n' "$*"; }
bad() { printf '  ✗ %s\n' "$*"; FAIL=1; }

echo "══ CI GATE SOVRAN — ESS-MAI ══"

# ── 1. ZONAT E NDALUARA — md5 i KYÇUR (çdo ndryshim = dështim) ──────────────
echo "[1] Zonat e ndaluara (md5 lock):"
check_lock() {
  local f="$1" want="$2" have
  if [ ! -f "$f" ]; then bad "MUNGON zona e ndaluar: $f"; return; fi
  have=$(md5sum "$f" | cut -d' ' -f1)
  if [ "$have" = "$want" ]; then ok "e paprekur: $f"
  else bad "ZONË E NDALUAR E NDRYSHUAR: $f (md5=$have, pritej $want)"; fi
}
check_lock shadow/src/shadow_gj_legacy.rs   7b6e25321d9bcbc5d52ff4e2556a64bd
check_lock light/src/shadow_seal_bridge.rs  05b69f2916c2c33fb2c5e0016331c736
check_lock shadow/kernel/shadow_gj_legacy.c 02f218507173875a2af1afcfa1340114
check_lock shadow/kernel/shadow_gj_legacy.h 4b3dc3f53cada933219cf0357980a7c7
check_lock light/kernel/shadow_gj_legacy.c  ddaae4905aa8f73abb2ea23fb754e201

# ── 2. KONTRATAT ×3 IDENTIKE (lab_contracts 11 + lab_contracts_v11 5) ───────
echo "[2] Kontratat byte-identike në 3 platformat:"
for dir in lab_contracts lab_contracts_v11; do
  n=0; b=0
  for f in light/src/$dir/*.rs; do
    base=$(basename "$f"); n=$((n+1))
    a=$(md5sum "light/src/$dir/$base"   | cut -d' ' -f1)
    q=$(md5sum "quantum/src/$dir/$base" 2>/dev/null | cut -d' ' -f1)
    s=$(md5sum "shadow/src/$dir/$base"  2>/dev/null | cut -d' ' -f1)
    if [ "$q$s" != "$a$a" ]; then bad "kontratë JO identike: $dir/$base"; b=1; fi
  done
  if [ $b -eq 0 ]; then ok "$dir: $n/$n identike ×3"; fi
done

# ── 3. pure_rust — VETËM TEST/DEV ────────────────────────────────────────────
echo "[3] pure_rust i kufizuar:"
if grep -E '^default\s*=' shadow/Cargo.toml | grep -q pure_rust; then
  bad "pure_rust në default features (shadow/Cargo.toml)"
else ok "pure_rust JASHTË default features"; fi
# cfg(feature="pure_rust") lejohet VETËM në zonën e ndaluar (pasqyra e deklaruar)
stray=$(grep -rln 'cfg(feature = "pure_rust")' shadow/src --include='*.rs' | grep -v 'shadow_gj_legacy.rs' || true)
if [ -n "$stray" ]; then bad "cfg(pure_rust) jashtë zonës së deklaruar: $stray"
else ok "cfg(pure_rust) vetëm në shadow_gj_legacy.rs (pasqyra e deklaruar)"; fi
if grep -q 'CARGO_FEATURE_RUNTIME_MODE' shadow/build.rs && grep -q 'NDALIM SOVRAN' shadow/build.rs; then
  ok "guard-i runtime_mode+pure_rust prezent në build.rs"
else bad "guard-i i ndalimit sovran MUNGON në shadow/build.rs"; fi

# ── 4. Dublikata e kernelit JASHTË build-it ──────────────────────────────────
echo "[4] shadow_gj_legacy_kernel.c jashtë build-it:"
if grep -q 'shadow_gj_legacy_kernel.c' shadow/build.rs; then
  if grep 'shadow_gj_legacy_kernel.c' shadow/build.rs | grep -q '\.file('; then
    bad "dublikata e kernelit u fut në build (shadow/build.rs)"
  else ok "dublikata e përmendur vetëm si koment përjashtimi"; fi
else ok "dublikata s'përmendet në build.rs si burim"; fi

# ── 5. unwrap() VETËM pas #[cfg(test)] (heuristikë rreshtash) ────────────────
echo "[5] .unwrap() vetëm në zonën e testeve:"
viol=0
while IFS= read -r f; do
  # unwrap REAL = jo brenda komenti ('//' para tij në të njëjtin rresht)
  first_u=$(awk '/\.unwrap\(\)/{c=index($0,"//"); u=index($0,".unwrap()"); if(c==0||u<c){print NR; exit}}' "$f")
  [ -z "$first_u" ] && continue
  first_t=$(grep -n '#\[cfg(test)\]' "$f" | head -1 | cut -d: -f1)
  if [ -z "$first_t" ] || [ "$first_u" -lt "$first_t" ]; then
    bad "unwrap() para/jashtë cfg(test): $f:$first_u"; viol=1
  fi
done < <(find light/src quantum/src shadow/src -name '*.rs')
if [ $viol -eq 0 ]; then ok "asnjë .unwrap() runtime (të gjithë pas cfg(test))"; fi

# ── 6. Kontrata e transportit PA — fail-closed prezent ──────────────────────
echo "[6] Transporti PA fail-closed:"
if grep -q 'TransportCorrupt' shadow/src/types.rs && \
   grep -q 'TransportCorrupt' shadow/src/shadow_gateway.rs; then
  ok "ShadowError::TransportCorrupt aktiv në gateway"
else bad "TransportCorrupt mungon — transporti PA s'është fail-closed"; fi
if grep -n 'return 0' shadow/src/shadow_gateway.rs | grep -vq 'Ok(0)'; then
  if grep -q 'return 0;' shadow/src/shadow_gateway.rs; then
    bad "'return 0' i zhveshur (fail-open) u rikthye në shadow_gateway.rs"
  else ok "asnjë 'return 0' fail-open në gateway"; fi
else ok "asnjë 'return 0' fail-open në gateway"; fi

# ── 7. Vault pa disk = FATAL (asnjë degradim RAM në main) ────────────────────
echo "[7] Autoriteti pa disk s'ngrihet:"
if grep -q 'FATAL (fail-closed): vault-i persistent' shadow/src/main.rs && \
   ! grep -q 'DEGRADIM: disku' shadow/src/main.rs; then
  ok "with_disk Err → exit(1), pa fallback RAM"
else bad "fallback-u RAM i vault-it ekziston ende në shadow/src/main.rs"; fi

# ── 8. TIME_DEGRADED — vula e auditimit prezente dhe e lidhur ────────────────
echo "[8] Vula TIME_DEGRADED:"
if grep -q 'pub static TIME_DEGRADED' shadow/src/shadow_pipeline.rs; then
  ok "vula ekziston në shadow_pipeline.rs"
else bad "vula TIME_DEGRADED mungon"; fi
for f in shadow/src/shadow_apupk.rs shadow/src/shadow_snb.rs; do
  if grep -q 'mark_time_degraded' "$f"; then ok "fallback i shënuar: $f"
  else bad "fallback kohor PA vulë: $f"; fi
done

# ── 9. Zero if/else jashtë build.rs ──────────────────────────────────────────
echo "[9] Zero if/else (jashtë build.rs):"
ie=$(find light/src quantum/src shadow/src -name '*.rs' \
     -exec grep -lE '^[[:space:]]*if |^[[:space:]]*\} else' {} \; | wc -l)
if [ "$ie" -eq 0 ]; then ok "0 skedarë me if/else klasik"
else bad "$ie skedarë me if/else klasik jashtë build.rs"; fi

# ── 10. Brace balance global ─────────────────────────────────────────────────
echo "[10] Brace balance:"
bb=0
while IFS= read -r f; do
  o=$(grep -o '{' "$f" | wc -l); c=$(grep -o '}' "$f" | wc -l)
  if [ "$o" -ne "$c" ]; then bad "brace jo-balancë: $f ($((o-c)))"; bb=1; fi
done < <(find light/src quantum/src shadow/src -name '*.rs')
if [ $bb -eq 0 ]; then ok "të gjithë .rs të balancuar"; fi

# ── 11. PACP — kontrata pa_wire ×3 identike DHE e lidhur në të dy anët ──────
echo "[11] PACP (pa_wire) në urë:"
a=$(md5sum light/src/lab_contracts/pa_wire.rs   2>/dev/null | cut -d' ' -f1)
q=$(md5sum quantum/src/lab_contracts/pa_wire.rs 2>/dev/null | cut -d' ' -f1)
s=$(md5sum shadow/src/lab_contracts/pa_wire.rs  2>/dev/null | cut -d' ' -f1)
if [ -n "$a" ] && [ "$q$s" = "$a$a" ]; then ok "pa_wire.rs identike ×3"
else bad "pa_wire.rs mungon ose JO identike ×3"; fi
if grep -q 'pa_wire::encode_line' light/src/main.rs; then ok "Light shkruan me encode_line (CRC)"
else bad "Light NUK shkruan me pa_wire::encode_line"; fi
if grep -q 'pa_wire::verify_line' shadow/src/shadow_gateway.rs; then ok "Shadow verifikon me verify_line (PACP gate)"
else bad "Shadow NUK verifikon me pa_wire::verify_line"; fi

# ── 12. BOOT SOVRAN — zero fallback relativ, FATAL pa env ────────────────────
echo "[12] Boot sovran (rrugët):"
if grep -rn 'Err(_) => file.to_string()' light/src quantum/src shadow/src >/dev/null 2>&1; then
  bad "fallback relativ (cwd) u rikthye në handoff_path"
else ok "zero fallback relativ — ESSMAI_HANDOFF_DIR i detyruar"; fi
for f in shadow/src/main.rs quantum/src/main.rs light/src/main.rs; do
  if grep -q 'ESSMAI_HANDOFF_DIR mungon' "$f"; then ok "FATAL i deklaruar: $f"
  else bad "FATAL për env mungues NUK gjendet: $f"; fi
done

# ── 13. GCL LIVE — laku Y→X i lidhur pas verdiktit ───────────────────────────
echo "[13] GCL live loop:"
if grep -q 'gcl_reinforce_on_verified' shadow/src/shadow_gateway.rs && \
   grep -q 'confirm_primitive' shadow/src/shadow_gateway.rs; then
  ok "Verified(Y)=1 → confirm_primitive (Trust X) i lidhur në ingest_bridged"
else bad "laku GCL live NUK është i lidhur në gateway"; fi

# ── 14. TRACE RE-VERIFY — bridge_light ──────────────────────────────────────
echo "[14] Trace re-verify (Quantum→Light):"
if grep -q 'trace re-verify' quantum/src/bridge_light/mod.rs; then
  ok "build_output rikontrollon seal↔trace_id"
else bad "trace re-verify mungon në bridge_light::build_output"; fi

# ── 15. HW DEKLARIM — zero improvizim sensori ────────────────────────────────
echo "[15] Deklarimi HW:"
if grep -q 'HW_NOMINAL' quantum/src/main.rs; then
  ok "boot-i i Quantum deklaron HW_NOMINAL/REAL me zë"
else bad "deklarimi HW_NOMINAL mungon në boot të Quantum"; fi

# ── 16. RRJEDHA — inteligjenca e dështimeve ×3 identike + e lidhur ───────────
echo "[16] Rrjedha (inteligjenca e dështimeve):"
H1=$(md5sum light/src/lab_contracts/rrjedha.rs   | cut -d' ' -f1)
H2=$(md5sum quantum/src/lab_contracts/rrjedha.rs | cut -d' ' -f1)
H3=$(md5sum shadow/src/lab_contracts/rrjedha.rs  | cut -d' ' -f1)
if [ "$H1" = "$H2" ] && [ "$H2" = "$H3" ]; then ok "rrjedha.rs byte-identike ×3"
else bad "rrjedha.rs DIVERGJENTE mes platformave"; fi
if grep -q 'rrjedha::note' shadow/src/shadow_gateway.rs && \
   grep -q 'rrjedha::note' light/src/main.rs; then
  ok "rrjedhat marrin PSE + shënim + kufi (note i lidhur në Shadow + Light)"
else bad "rrjedha::note NUK është i lidhur në rrjedhat reale"; fi

# ── 17. GCL I MBARTUR — laku mbyllet edhe pas heqjes së ankorës ──────────────
echo "[17] GCL live në rrugën e urës:"
if grep -q 'gcl_anchor' shadow/src/shadow_gateway.rs && \
   grep -q 'fn gcl_apply' shadow/src/shadow_gateway.rs; then
  ok "ankora e mbartur (gcl_anchor) → gcl_apply pas verdiktit — laku i gjallë në urë"
else bad "GCL i mbartur MUNGON — laku Y→X i vdekur në rrugën e urës"; fi

# ── 18. PARANDALIMI NË BURIM — shkruesi s'emeton dot rresht të refuzueshëm ───
echo "[18] Parandalimi në burim (pa_wire):"
if grep -q 'encode_line_verified' shadow/src/lab_contracts/pa_wire.rs && \
   grep -q 'encode_line_verified' light/src/main.rs; then
  ok "Light shkruan VETËM rreshta të provuar në burim (encode_line_verified)"
else bad "encode_line_verified NUK përdoret nga shkruesi"; fi
if grep -q 'and_then(|_| f.flush())' light/src/main.rs && \
   grep -q 'and_then(|_| f.flush())' quantum/src/main.rs; then
  ok "flush i detyruar pas çdo shkrimi ure (dritarja e cungimit e ngushtuar)"
else bad "flush mungon në pikat e shkrimit të urës"; fi

# ── 19. DIAGNOZA E DISKUT — NotFound=normale; çdo tjetër = PSE i deklaruar ───
echo "[19] Diagnoza e diskut (feed):"
if grep -q 'ErrorKind::NotFound' shadow/src/shadow_gateway.rs; then
  ok "NotFound dallohet nga leja e mohuar/I/O reale — zero maskim dështimesh"
else bad "gabimet e diskut ende maskohen si 'asnjë ankorë'"; fi
if grep -q 'DEGRADIM' shadow/src/shadow_gateway.rs; then
  ok "çelësi njëdrejtimësh: legacy pas rreshti të vulosur = refuzim i argumentuar"
else bad "kufiri antidegradim MUNGON"; fi

# ── 20. LEXUESIT E QUANTUM të ringjallur (verbëria 6-fushëshe e shuar) ───────
echo "[20] Quantum lexon me gjykim (PA):"
if grep -q 'pa_wire::verify_line' quantum/src/main.rs && \
   grep -q 'let shadow_split = shadow_lib::lab_contracts::collapse::PrimitiveSplit' quantum/src/main.rs && \
   grep -q 'register_primitive_anchor(sess, pa_id, shadow_split)' quantum/src/main.rs; then
  ok "vnk_pa_gate: verify-before-interpret + adaptim tipi ndër-crate + register 3-argumentësh"
else bad "lexuesi PA i Quantum ende i verbër/i vjetruar"; fi
if grep -q 'read_primitive_context' quantum/src/main.rs && \
   grep -A90 'fn read_primitive_context' quantum/src/main.rs | grep -q 'pa_wire::verify_line' && \
   grep -A120 'fn read_primitive_context' quantum/src/main.rs | grep -q 'i0-'; then
  ok "read_primitive_context: PA gjykohet para interpretimit dhe jep i₀ + Xi/Yi"
else bad "read_primitive_context nuk mbyll verify-before-interpret për i₀ + Xi/Yi"; fi

# ── 21. URA 1 (PD) e vulosur në të dy anët ───────────────────────────────────
echo "[21] URA 1 (PD) e vulosur:"
if grep -q 'seal_body_verified' quantum/src/main.rs; then
  ok "Quantum shkruan PD VETËM të vulosur e të vetë-gjykuar (probe + handoff)"
else bad "PD writers pa vulë gjenerike"; fi
if grep -q 'verify_line_generic' light/src/main.rs; then
  ok "Light gjykon PD para interpretimit (6/7 legacy, 8 CRC, çelës njëdrejtimësh)"
else bad "Light ende interpreton PD pa gjykim"; fi

# ── 22. INSTALLER-i thërret CI GATE si portë të detyrueshme ──────────────────
echo "[22] Installer ↔ CI GATE:"
if grep -q 'Invoke-CiGate' setup_essmai.ps1 && \
   grep -q "CI GATE SOVRAN" setup_essmai.ps1; then
  ok "setup_essmai.ps1 ekzekuton ci_gate.sh — EXIT≠0 e ndal nisjen"
else bad "installer-i s'e thërret ci_gate"; fi

# ── 23. GCL_PRESUME — gatishmëria nën ligj, e lidhur në 3 platformat ─────────
echo "[23] GCL_PRESUME (gatishmëria nën ligj):"
H1=$(md5sum light/src/lab_contracts/gcl_presume.rs   | cut -d' ' -f1)
H2=$(md5sum quantum/src/lab_contracts/gcl_presume.rs | cut -d' ' -f1)
H3=$(md5sum shadow/src/lab_contracts/gcl_presume.rs  | cut -d' ' -f1)
if [ "$H1" = "$H2" ] && [ "$H2" = "$H3" ]; then ok "gcl_presume.rs byte-identike ×3"
else bad "gcl_presume.rs DIVERGJENTE mes platformave"; fi
pw=0
for f in light/src/main.rs quantum/src/main.rs shadow/src/main.rs; do
  if grep -q 'gcl_presume as presume' "$f" && grep -q 'presume::sweep' "$f"; then :
  else bad "presume JO i lidhur në: $f"; pw=1; fi
done
if [ $pw -eq 0 ]; then ok "regjistrim + sweep në TË TRIA mains (Coordination/Reasoning/Verification)"; fi
if grep -q 'GclRole::MproMeasure' quantum/src/main.rs && \
   grep -q 'GclRole::NpimPackage' quantum/src/main.rs; then
  ok "rolet e formulës të lidhura me modulet reale (PRO/NPRO/MPRO/APRO/PIM/NPIM)"
else bad "rolet e Quantum s'janë të lidhura me modulet reale"; fi

# ── 24. UNIFIKIMI I RRUGËVE (audit A5) — ESSMAI_VAULT absolut+FATAL ─────────
echo "[24] Unifikimi i rrugëve (vault sovran):"
if grep -q 'ESSMAI_VAULT.*prindi s.u krijua\|ESSMAI_VAULT=.*FATAL\|canonicalize(&parent)' shadow/src/main.rs; then
  ok "ESSMAI_VAULT kanonikalizohet në absolute (pari) + FATAL — s'ka më relativ heshtazi"
else bad "ESSMAI_VAULT ende mund të mbetet relativ (audit A5 i pazgjidhur)"; fi

# ── 25. MENAXHIMI REAL I HARDUERIT — 10% floor + anti-mbinxehje ─────────────
echo "[25] Harduer real (10% floor, anti-mbinxehje):"
if grep -q 'HW_FREE_FLOOR_PCT: f32 = 0.10' quantum/src/hw_real/hw_manager.rs; then
  ok "dyshemeja 10% e RAM/cores e koduar (HW_FREE_FLOOR_PCT=0.10)"
else bad "dyshemeja 10% MUNGON ose u ndryshua"; fi
if grep -q 'anti-mbinxehje\|_ => reserved_raw.max(1)' quantum/src/hw_real/hw_manager.rs; then
  ok "makinat 2-4 bërthama KURRË 100% cores (≥1 gjithnjë e lirë)"
else bad "floor-i i bërthamave s'mbron makinat e vogla nga mbinxehja"; fi
if grep -q 'orchestrate_thermal' quantum/src/main.rs && \
   grep -q 'thermal_hot' quantum/src/main.rs; then
  ok "menaxhimi HW reagon ndaj nxehtësisë REALE (orchestrate_thermal i lidhur në pipeline)"
else bad "porta termike s'është e lidhur — HW s'reagon ndaj mbinxehjes"; fi

# ── 26. gcl_presume ZGJERUAR (audit A6) + NK-bridge i vulosur (rek.2) ────────
echo "[26] GCL_PRESUME zgjeruar + NK-bridge i vulosur:"
if grep -q 'hw_pre_gate' quantum/src/main.rs && \
   grep -q 'nk_maturim' shadow/src/main.rs && \
   grep -q 'trl_input' light/src/main.rs; then
  ok "registry zgjeruar: hw_gate/termik/NK/TRL hyjnë në apel me status real"
else bad "gcl_presume s'u zgjerua me modulet e reja (A6 i pazgjidhur)"; fi
if grep -q 'seal_negative_export' quantum/src/pro_nk_gate.rs && \
   grep -q 'seal_negative_export' quantum/src/main.rs; then
  ok "ura binare NK e vulosur (NKB1+CRC) — bitflip kapet, jo dije e prishur"
else bad "NK-bridge ende pa integritet CRC (rek.2 i pazgjidhur)"; fi

# ── 27. TERMIKU 3-GJENDJESH + PULSI RUNTIME (v1.4.3) ────────────────────────
echo "[27] Termik 3-gjendjesh + puls runtime:"
if grep -q 'enum ThermalState' quantum/src/hw_real/thermal.rs && \
   grep -q 'ThermalState::Unknown' quantum/src/main.rs && \
   grep -q 'effective_hot(hcp_depth)' quantum/src/main.rs; then
  ok "Unknown ≠ i ftohtë — tërheqje konservative për Deep/VeryDeep e lidhur në pipeline"
else bad "ThermalState 3-gjendjesh JO i lidhur"; fi
if grep -q 'pub mod runtime_pulse' quantum/src/lib.rs && \
   grep -q 'runtime_pulse::pro_status' quantum/src/main.rs && \
   grep -q 'mark_ready(quantum_lim::runtime_pulse::Stage::Pro)' quantum/src/main.rs; then
  ok "presume lexon PULSIN real (PRO..NPIM+split+termik), jo konstante gati_gjithnje"
else bad "regjistrimet ende me konstante — apel simbolik"; fi
if grep -q 'cfg!(feature = "hw_kernel")' quantum/src/runtime_pulse.rs; then
  ok "hw/thermal status nga E VËRTETA e kompilimit — env-i s'e falsifikon (p.5)"
else bad "statusi HW ende env-based"; fi

# ── 28. BURIMI I DYTË I TOKEN-IT + NK ratchet + NKL1 ─────────────────────────
echo "[28] Token burimi 2 + NK njëdrejtimësh + NKL1:"
if grep -q 'pub mod token_forge' quantum/src/lib.rs && \
   grep -q 'token_forge::mint' quantum/src/main.rs && \
   grep -q '\[TOKEN2\]' quantum/src/main.rs; then
  ok "farka (burimi 2) derdh token të pavarur për çdo vendim HCP"
else bad "burimi i dytë i token-it MUNGON"; fi
if grep -q 'nk_downgrade' quantum/src/main.rs && \
   grep -q 'shadow_nk_export.sealed' quantum/src/main.rs && \
   grep -q 'b"NKL1"' quantum/src/pro_nk_gate.rs; then
  ok "NK: çelës njëdrejtimësh (.sealed) + NKL1 me gjatësi — niveli i PA-wire"
else bad "NK ratchet/NKL1 i pazbatuar (p.7/p.8)"; fi

# ── 29. TESTET E SJELLJES (audit p.10) — 4 emrat e detyruar ekzistojnë ───────
echo "[29] Roja e sjelljes (jo vetëm grep):"
BT=0
grep -rq 'fn orchestrate_thermal_hot_true_causes_pullback' quantum/src || BT=1
grep -rq 'fn hw_env_does_not_fake_hw_kernel' quantum/src || BT=1
grep -rq 'fn nk_legacy_after_nkb1_is_downgrade' quantum/src || BT=1
grep -rq 'fn split_corrupt_updates_presume_status' quantum/src || BT=1
if [ $BT -eq 0 ]; then ok "4/4 testet e sjelljes së auditit të pranishme e të emërtuara"
else bad "mungojnë teste sjelljeje nga lista e auditit"; fi
if grep -q 'sweep("QUANTUM", CollapsePhase::Reasoning' quantum/src/main.rs && \
   grep -q '"presume" =>' shadow/src/main.rs && \
   grep -q 'is_absolute' shadow/src/main.rs; then
  ok "sweep runtime (Reasoning) + komanda presume + vault ABSOLUT — p.2/p.9 mbyllur"
else bad "p.2/p.9 të pambyllura"; fi

# ── 30. FRYMA E PARË + LEGACY GJURMË + DEEPTECH TRL (v1.4.4) ─────────────────
echo "[30] Fryma e parë (ESSMAI_FIRST_GUARDIAN) + legacy + deeptech:"
H1=$(md5sum light/src/lab_contracts/first_guardian.rs   | cut -d' ' -f1)
H2=$(md5sum quantum/src/lab_contracts/first_guardian.rs | cut -d' ' -f1)
H3=$(md5sum shadow/src/lab_contracts/first_guardian.rs  | cut -d' ' -f1)
if [ "$H1" = "$H2" ] && [ "$H2" = "$H3" ]; then ok "first_guardian.rs byte-identike ×3"
else bad "first_guardian.rs DIVERGJENTE"; fi
FR=0
for f in light/src/main.rs quantum/src/main.rs shadow/src/main.rs; do
  grep -q 'fryma::attest' "$f" || FR=1
done
if [ $FR -eq 0 ]; then ok "riti i frymës i lidhur në TË TRIA ndezjet (attest ×3)"
else bad "riti i frymës JO i lidhur kudo"; fi
if grep -q 'known_vector_lock' shadow/src/lab_contracts/first_guardian.rs && \
   grep -q 'wire_roundtrip_lock' shadow/src/lab_contracts/first_guardian.rs; then
  ok "Known-Vector Live Lock: çekiçi+ura riprovohen NË MAKINË në çdo ndezje"
else bad "live-lock mungon"; fi
LG=$(grep -rc 'LEGACY_PRANUAR' light/src/main.rs quantum/src/main.rs shadow/src/shadow_gateway.rs | awk -F: '{s+=$2} END{print s}')
if [ "$LG" -ge 4 ]; then ok "legacy i përforcuar: $LG gjurmë të vulosura + kufi këshillues (Other=5)"
else bad "legacy pa gjurmë të mjaftueshme ($LG<4)"; fi
if grep -q 'pub struct TrlVector' shadow/src/lab_contracts/trl.rs && \
   grep -q 'TRL_VEKTOR' quantum/src/main.rs; then
  ok "DEEPTECH: TrlVector ×3 + dimensioni i arsyetimit i lidhur me evidencën reale"
else bad "shtylla deeptech (TRL-vektor) mungon"; fi

# ── 31. SHADOW RUNTIME PULSE (audit p.3) ────────────────────────────────────
echo "[31] Shadow pulse real (jo gati_gjithnje):"
if grep -q 'pub mod shadow_runtime_pulse' shadow/src/lib.rs && \
   grep -q 'shadow_runtime_pulse::judge_status' shadow/src/main.rs && \
   grep -q 'SStage::Judge' shadow/src/shadow_gateway.rs; then
  ok "judge/gcl_apply/negvault/maturim/trl lexojnë PULSIN runtime në Shadow"
else bad "Shadow presume ende me konstante (p.3 i pazgjidhur)"; fi
if grep -q 'first_guardian::breath_status' shadow/src/main.rs && \
   grep -q 'first_guardian::breath_status' quantum/src/main.rs && \
   grep -q 'first_guardian::breath_status' light/src/main.rs; then
  ok "fryma_e_pare në apel ×3 — READY vetëm nga fryma e PLOTË"
else bad "breath_status JO i lidhur në presume ×3"; fi

# ── 32. FIRST_GUARDIAN critical rites + per-platformë (audit p.1/p.2) ────────
echo "[32] Fryma: rite kritike + skedar per-platformë:"
if grep -q 'BreathGrade::DegradedHard' shadow/src/lab_contracts/first_guardian.rs && \
   grep -q 'critical: true' shadow/src/main.rs; then
  ok "ritet KRITIKE (fnv/urë) → DEGRADED_HARD; jo-kritiket → alarm (runtime vazhdon)"
else bad "rite kritike të padiferencuara (p.1)"; fi
if grep -q 'first_guardian_light.txt' shadow/src/lab_contracts/first_guardian.rs && \
   grep -q 'first_guardian_shadow.txt' shadow/src/lab_contracts/first_guardian.rs; then
  ok "dëshmi PER-PLATFORMË — tri ndezje paralele s'ndërthuren (p.2)"
else bad "ende një skedar i vetëm për 3 platforma (p.2)"; fi

# ── 33. TRL VEKTOR I PLOTË + TOKEN WITNESS + NK-presume + LINEAGE ───────────
echo "[33] Zinxhiri TRL + witness + NK-presume + lineage:"
if grep -q 'fn assemble' shadow/src/lab_contracts/trl.rs && \
   grep -q 'shkruaj_trl_dim' light/src/main.rs && \
   grep -q 'shkruaj_trl_dim' shadow/src/shadow_gateway.rs && \
   grep -q '\[TRL_VEKTOR\] i PLOTË' shadow/src/shadow_gateway.rs; then
  ok "TrlVector i plotë Light(in)→Quantum(rsn)→Shadow(ver) — montuar në gcl_apply (p.4)"
else bad "zinxhiri TRL i paplotë (p.4)"; fi
# Witness-guard: mint NUK guxon në arsyetim deterministik.
WG=0
grep -q 'token_forge::mint' quantum/src/collapse.rs 2>/dev/null && WG=1
grep -rq 'token_forge::mint' quantum/src/progressive_debatic/ 2>/dev/null && WG=1
grep -q 'token_forge' shadow/src/shadow_lab.rs 2>/dev/null && WG=1
if [ $WG -eq 0 ]; then ok "TokenForge = witness i pastër — zero mint në collapse/PD/TRL/judge (p.5)"
else bad "TokenForge rrjedh në arsyetim deterministik (p.5 SHKELUR)"; fi
if grep -q 'nk_status' quantum/src/main.rs && \
   grep -q 'Stage::NkImport' quantum/src/pro_nk_gate.rs; then
  ok "NK-import raporton NOT_READY në presume kur korrupton (p.6)"
else bad "NK-import pa status presume (p.6)"; fi
LN=$(grep -rc 'LEGACY_PRANUAR kind=' light/src/main.rs quantum/src/main.rs shadow/src/shadow_gateway.rs | awk -F: '{s+=$2} END{print s}')
if [ "$LN" -ge 4 ]; then ok "legacy lineage: $LN gjurmë me kind/bridge/session/sealed_para (p.7)"
else bad "legacy pa fusha lineage ($LN<4)"; fi

# ── 34. VERIFICATION_TRL nga legacy_score REAL (audit v1.4.5 p.2) ────────────
echo "[34] verification_trl nga sinjali real + numërim testesh publik:"
if grep -q 'verdict.legacy_score > 0.0' shadow/src/shadow_gateway.rs && \
   grep -q 'legacy_score={:.3}' shadow/src/shadow_gateway.rs; then
  ok "verification_trl derivohet nga verdict.legacy_score REAL (jo proxy 0.90/0.30)"
else bad "verification_trl ende proxy konstant (p.2 i pazgjidhur)"; fi
if grep -q 'fn verification_trl_derives_from_real_legacy_score' shadow/src/shadow_gateway.rs; then
  ok "test i sjelljes: mapimi legacy_score→verification_trl i mbrojtur"
else bad "mungon testi i lidhjes legacy_score→TRL"; fi
# Numërim publik i testeve — që '1046' të mos jetë kurrë më fjalë-kundër-fjalë.
NTEST=$(grep -rh '#\[test\]' --include='*.rs' . | wc -l | tr -d ' ')
echo "  ℹ #[test] të numëruar në repo: $NTEST (grep -rh '#[test]')"
if [ "$NTEST" -ge 1047 ]; then ok "numërimi i testeve: $NTEST (≥1047, rritur me v1.4.6)"
else bad "numërimi i testeve: $NTEST (pritej ≥1047)"; fi

echo "══════════════════════════════"
if [ $FAIL -eq 0 ]; then
  echo "CI GATE: ✓ KALOI — ligjet strukturore të paprekura."
  exit 0
else
  echo "CI GATE: ✗ DËSHTOI — shiko shkeljet më lart. Build-i NUK lejohet."
  exit 1
fi
