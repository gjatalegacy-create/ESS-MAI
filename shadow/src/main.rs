#![warn(clippy::unwrap_used)]
include!("lib.rs");
mod process_bridge;

// Testi historik i integrimit ruhet, por kompilohet BRENDA target-it binar.
// Kështu invariantët verifikohen pa ringjallur një target library të linkueshëm.
#[cfg(test)]
#[path = "../tests/integration.rs"]
mod integration_tests;

// ╔══════════════════════════════════════════════════════════════════════════╗
// ║ SHADOW MAIN.RS — KUSHTI I DOMOSDOSHËM I AUTORITETIT (v1.6.4)          ║
// ║ Gjata Legacy™ | ESS-MAI Shadow Platform                                ║
// ║                                                                          ║
// ║ Cargo vendos `autolib = false`: Shadow NUK prodhon rlib/staticlib.       ║
// ║ `include!("lib.rs")` e kompilon kushtetutën vetëm brenda këtij binari.   ║
// ║ Prandaj as Quantum dhe asnjë crate tjetër nuk mund të ndërtojë Shadow,   ║
// ║ të hapë vault-in, të thërrasë ingest-in ose të prodhojë receipt pa       ║
// ║ ekzekutuar main.rs.                                                       ║
// ║                                                                          ║
// ║ Quantum njeh vetëm `shadow_contracts`: formën wire të kërkesës dhe       ║
// ║ dëshmisë. Vlera reale lind këtu, pas PA gate + Verification Collapse +   ║
// ║ token-it sovran. Mungesa/dështimi i këtij procesi = fail-closed.         ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::lab_contracts::proclaim_law;
use crate::phase9_integration::Phase9Integration;
use std::io::BufRead as _;

/// Skedari i vault-it persistent — dija e Shadow mbijeton mes nisjeve.
const WISDOM_VAULT_FILE: &str = "shadow_wisdom.svlt";

/// Skedari i sinjaleve PA nga Light (kontrata 5-fushëshe e URA 2).
const PA_EXPORT_FILE: &str = "light_pa_export.txt";

/// ZGJIDHJA e cwd-dependencës (#8, ULTIMATUM): rruga e handoff-it/vault-it
/// zgjidhet nga env — ESSMAI_HANDOFF_DIR jep dosjen bazë të përbashkët për të
/// TRE binarët (Light shkruan, Shadow lexon të NJËJTIN path pavarësisht cwd).
/// ESSMAI_VAULT mbizotëron plotësisht rrugën e vault-it. Fallback = relativ
/// (sjellja e vjetër). Zero unwrap — match.
fn handoff_path(file: &str) -> String {
    match std::env::var("ESSMAI_HANDOFF_DIR") {
        Ok(dir) => {
            // BOOT SOVRAN: dosja krijohet po të mungojë dhe kanonikalizohet
            // në ABSOLUTE — zero varësi nga cwd, gjurmueshmëri e përsëritshme.
            match std::fs::create_dir_all(&dir) {
                Ok(())  => {}
                Err(e)  => {
                    eprintln!("[BOOT] FATAL (fail-closed): ESSMAI_HANDOFF_DIR='{dir}' s'u krijua dot ({e}).");
                    std::process::exit(1);
                }
            }
            let abs = match std::fs::canonicalize(&dir) {
                Ok(p)  => p,
                Err(e) => {
                    eprintln!("[BOOT] FATAL (fail-closed): ESSMAI_HANDOFF_DIR='{dir}' s'u kanonikalizua ({e}).");
                    std::process::exit(1);
                }
            };
            let joined = abs.join(file);
            joined.to_string_lossy().into_owned()
        }
        Err(_) => {
            // ZERO IMPROVIZIM: pa rrugë të deklaruar s'ka nisje — rrugët
            // relative (cwd) dëmtojnë determinizmin dhe gjurmën e auditimit.
            eprintln!("[BOOT] FATAL (fail-closed): ESSMAI_HANDOFF_DIR mungon.");
            eprintln!("[BOOT] Cakto dosjen e përbashkët të urave, p.sh.:");
            eprintln!("[BOOT]   Linux/macOS:  export ESSMAI_HANDOFF_DIR=/opt/essmai/handoff");
            eprintln!("[BOOT]   Windows:      $env:ESSMAI_HANDOFF_DIR='C:\\essmai\\handoff'");
            std::process::exit(1);
        }
    }
}

fn wisdom_vault_path() -> String {
    match std::env::var("ESSMAI_VAULT") {
        // GJETJA v1.4.2 (audit i jashtëm A5, i verifikuar): mbizotërimi
        // mbahej i papërpunuar — './vault' mbetej RELATIV, autoriteti varej
        // nga cwd. Tani prindi kanonikalizohet në ABSOLUTE (i njëjti ligj si
        // handoff_path); rrugë e pavlefshme = FATAL, kurrë relative heshtazi.
        Ok(p) => {
            let pb = std::path::PathBuf::from(&p);
            // v1.4.3 (audit p.2, PRANUAR): "absolut nga cwd" S'ËSHTË absolut.
            // ESSMAI_VAULT duhet ABSOLUT i deklaruar — ose HIQE env-in dhe
            // default-i zgjidhet brenda ESSMAI_HANDOFF_DIR (tashmë sovran).
            match pb.is_absolute() {
                true  => {}
                false => {
                    eprintln!("[BOOT] FATAL (fail-closed): ESSMAI_VAULT='{p}' është RELATIV — varet nga cwd.");
                    eprintln!("[BOOT] Jep rrugë ABSOLUTE (p.sh. /opt/essmai/vault/shadow.svlt | C:\\essmai\\vault\\shadow.svlt)");
                    eprintln!("[BOOT] OSE hiqe ESSMAI_VAULT — default-i banon brenda ESSMAI_HANDOFF_DIR.");
                    std::process::exit(1);
                }
            }
            let parent = match pb.parent() {
                Some(par) if !par.as_os_str().is_empty() => par.to_path_buf(),
                _ => std::path::PathBuf::from("."),
            };
            match std::fs::create_dir_all(&parent) {
                Ok(())  => {}
                Err(e)  => {
                    eprintln!("[BOOT] FATAL (fail-closed): ESSMAI_VAULT='{p}' — prindi s'u krijua ({e}).");
                    std::process::exit(1);
                }
            }
            let fname = match pb.file_name() {
                Some(f) => f.to_owned(),
                None    => {
                    eprintln!("[BOOT] FATAL (fail-closed): ESSMAI_VAULT='{p}' — pa emër skedari.");
                    std::process::exit(1);
                }
            };
            match std::fs::canonicalize(&parent) {
                Ok(abs) => abs.join(fname).to_string_lossy().into_owned(),
                Err(e)  => {
                    eprintln!("[BOOT] FATAL (fail-closed): ESSMAI_VAULT prindi '{}' s'u kanonikalizua ({e}).",
                        parent.display());
                    std::process::exit(1);
                }
            }
        }
        Err(_) => handoff_path(WISDOM_VAULT_FILE), // handoff_path = tashmë absolut+FATAL
    }
}

fn pa_export_path() -> String {
    handoff_path(PA_EXPORT_FILE)
}

fn interactive_main() {
    // ── BOOT: sistemi LEXON ligjin — autoriteti shpallet para çdo veprimi ──
    proclaim_law("SHADOW");

    // ══ GCL_PRESUME (v1.4.1): gatishmëria nën ligj — komandanti (CollapsePhase)
    // urdhëron, presume ping-on; edhe modulet anësore përgjigjen me rol+status,
    // zero arsyetim shtesë në to. Sweep-i informon, KURRË s'bllokon nisjen.
    {
        use crate::lab_contracts::gcl_presume as presume;
        use crate::lab_contracts::CollapsePhase;
        fn gati_gjithnje() -> presume::GclReadiness { presume::GclReadiness::Ready }
        fn gati_env() -> presume::GclReadiness {
            match std::env::var("ESSMAI_HANDOFF_DIR") {
                Ok(_)  => presume::GclReadiness::Ready,
                Err(_) => presume::GclReadiness::NotReady("ESSMAI_HANDOFF_DIR mungon"),
            }
        }
    presume::register("pa_feed(pa_wire)",     presume::GclRole::AnchorGate,    gati_env);
    // v1.4.5 (audit p.3): READY nga PULSI runtime, jo gati_gjithnje.
    presume::register("judge_supreme(zonë)",  presume::GclRole::VerdictY,      crate::shadow_runtime_pulse::judge_status);
    presume::register("gcl_apply(Trust X)",   presume::GclRole::TrustX,        crate::shadow_runtime_pulse::gcl_apply_status);
    presume::register("vault_negative(WAL)",  presume::GclRole::NegativeVault, crate::shadow_runtime_pulse::negvault_status);
    presume::register("shadow_apupk",         presume::GclRole::Support("apupk"),      gati_gjithnje);
    presume::register("shadow_snb",           presume::GclRole::Support("snb"),        gati_gjithnje);
        // ZGJERIMI v1.4.2 (audit A6): NK-maturim + TRL-verdikt hyjnë në apel.
        presume::register("nk_maturim(vault)",    presume::GclRole::Support("nk-piqet"),  crate::shadow_runtime_pulse::maturation_status);
        presume::register("trl_verdict(shadow_lab)", presume::GclRole::Support("trl"),   crate::shadow_runtime_pulse::trl_verdict_status);
        let dir_rrjedhe = std::env::var("ESSMAI_HANDOFF_DIR")
            .unwrap_or_else(|_| std::env::temp_dir().to_string_lossy().into_owned());
        let _ = presume::sweep("SHADOW", CollapsePhase::Verification, &dir_rrjedhe);

        // ══ FRYMA E PARË (v1.4.4): autoriteti provon çekiçin, urën dhe
        // matricën e ligjit para se të gjykojë — dëshmi e vulosur në ndezje.
        use crate::lab_contracts::first_guardian as fryma;
        fn org_env() -> bool {
            match std::env::var("ESSMAI_HANDOFF_DIR") { Ok(_) => true, Err(_) => false }
        }
        fn org_matrica_gcl() -> bool {
            use crate::lab_contracts::gcl_presume::GclRole;
            use crate::lab_contracts::CollapsePhase;
            GclRole::VerdictY.required_in(CollapsePhase::Verification)
                && !GclRole::Support("x").required_in(CollapsePhase::Verification)
                && !GclRole::VerdictY.required_in(CollapsePhase::Coordination)
        }
        presume::register("fryma_e_pare", presume::GclRole::Support("frymë"),
            crate::lab_contracts::first_guardian::breath_status);
        let _ = fryma::attest("SHADOW", &dir_rrjedhe, &[
            fryma::RiteCheck { name: "fnv_known_vector",  run: fryma::known_vector_lock,  critical: true },
            fryma::RiteCheck { name: "pa_wire_roundtrip", run: fryma::wire_roundtrip_lock, critical: true },
            fryma::RiteCheck { name: "handoff_env",       run: org_env,        critical: false },
            fryma::RiteCheck { name: "gcl_matrica_ligjit", run: org_matrica_gcl, critical: false },
        ]);
    }

    // ── AUTORITETI NGRIHET: vault persistent (WAL replay) me fallback ──────
    // HARDENING (fail-closed): dështimi i diskut NDAL autoritetin — pa
    // persistencë s'ka wisdom të vërtetë (dija duhet të mbijetojë procesin).
    let vault_path = wisdom_vault_path();
    let shadow = match Shadow::with_disk(&vault_path) {
        Ok(s) => {
            println!("  [WISDOM] vault persistent i hapur: {} (dija mbijeton)",
                vault_path);
            s
        }
        Err(e) => {
            // HARDENING (fail-closed): autoriteti PA persistencë NUK është
            // autoritet i plotë — dija s'mbijeton, lineage humbet. Asnjë
            // degradim i heshtur në RAM: rregullo diskun/lejet dhe rinis.
            eprintln!("  [WISDOM] FATAL (fail-closed): vault-i persistent s'u hap ({e:?})");
            eprintln!("  [WISDOM] Autoriteti pa disk NUK ngrihet — rregullo rrugën '{}' dhe rinis.", vault_path);
            std::process::exit(1);
        }
    };

    print_wisdom(&shadow.stats());

    // ── FAZA 9 (AKTIVIZUAR) — verifikim RUNTIME i ligjit kushtetues të Shadow-it:
    // asnjë veprim s'mund të ANASHKALOJË (bypass/skip) verifikimin sovran. Kjo e
    // lidh phase9_integration me rrjedhën e nisjes dhe e bën ligjin të gjallë në
    // runtime (paralel me Light "s'vendos" dhe Quantum "prodhon provë").
    match Phase9Integration::verify_no_bypass("boot") {
        true  => println!("  [FAZA9]    ligji i mos-anashkalimit AKTIV (asnjë bypass i verifikimit sovran)"),
        false => {
            eprintln!("  [FAZA9]    FATAL: u tentua anashkalim në boot — fail-closed");
            return; // fail-closed: bypass i verifikimit = ndalim
        }
    }

    println!("\n  KOMANDAT: wisdom | feed <session_id> | <session_id> | neg | law | presume | exit");
    println!("  (çdo rresht tjetër = session_id → konsumohet sinjali PA i Light-it)\n");

    // ── EKZEKUTUESI: loop stdin — çdo rresht → thirrje te AUTORITETI ───────
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let raw = match line {
            Ok(l)  => l,
            Err(e) => {
                eprintln!("  [MAIN] stdin i palexueshëm: {e} — ekzekutuesi ndalon");
                break; // fail-closed: hyrje e prishur → dalje e pastër
            }
        };
        let cmd = raw.trim();
        match cmd.is_empty() {
            true  => continue,
            false => {}
        }

        // Zbërthimi i komandës: fjala e parë + argumenti opsional.
        let mut parts = cmd.splitn(2, char::is_whitespace);
        let head = parts.next().unwrap_or("");
        let tail = parts.next().unwrap_or("").trim();

        match head {
            "exit" | "quit" => {
                println!("  [MAIN] ekzekutuesi mbyllet — autoriteti (vault) mbetet i vulosur.");
                break;
            }
            "law" => proclaim_law("SHADOW"),
            // v1.4.3 (audit p.9): apeli NË RUNTIME me kërkesë — GCL rreh live.
            "presume" => {
                let d = std::env::var("ESSMAI_HANDOFF_DIR")
                    .unwrap_or_else(|_| std::env::temp_dir().to_string_lossy().into_owned());
                let _ = crate::lab_contracts::gcl_presume::sweep(
                    "SHADOW",
                    crate::lab_contracts::CollapsePhase::Verification,
                    &d);
            }
            "wisdom" | "status" => print_wisdom(&shadow.stats()),
            "neg" => {
                // Autoriteti eksporton dijen negative (bytes të vulosur).
                let bytes = shadow.export_negative_knowledge();
                println!("  [WISDOM] dije negative e eksportuar: {} bytes (ASET, kurrë e fshirë)",
                    bytes.len());
            }
            "feed" => match tail.is_empty() {
                true  => println!("  [MAIN] përdorimi: feed <session_id>"),
                false => feed_session(&shadow, tail),
            },
            // Çdo fjalë tjetër = session_id → rruga kryesore reale.
            other => feed_session(&shadow, other),
        }
    }
}

/// EKZEKUTIM: përcjell te autoriteti — lib.feed_primitive_anchors —
/// dhe shfaq atë që autoriteti ktheu. Zero logjikë vendimi këtu.
///
/// FAZA 9 (AKTIVE): para çdo ushqimi, verifikohet ligji i mos-anashkalimit —
/// asnjë veprim s'kalon pa verifikim sovran. Fail-closed nëse tentohet bypass.
fn feed_session(shadow: &Shadow, session_id: &str) {
    match Phase9Integration::verify_no_bypass(session_id) {
        true  => {}
        false => {
            eprintln!("  [FAZA9]    REFUZIM: session='{}' përmban tentativë anashkalimi — fail-closed",
                session_id);
            return; // fail-closed: emri i sesionit tenton bypass/skip → ndalim
        }
    }
    let pa_path = pa_export_path();
    // HARDENING: korrupsioni i transportit PA → REFUZIM fail-closed i sesionit.
    let n = match shadow.feed_primitive_anchors(&pa_path, session_id) {
        Ok(k)  => k,
        Err(e) => {
            eprintln!("  [PA-GATE] REFUZIM (fail-closed): {e} — sesioni ndalon këtu");
            return;
        }
    };
    match n {
        0 => println!("  [PA-GATE] session={} — asnjë ankorë e re ({} mungon ose s'ka rresht për të)",
            session_id, pa_path),
        k => println!("  [PA-GATE] session={} — {} ankorë PA+(Xi,Yi) NË PRITJE për propozimin XY",
            session_id, k),
    }
}

/// Shfaqja e dijes së autoritetit — main vetëm e lexon, kurrë s'e ndryshon.
fn print_wisdom(s: &ShadowStats) {
    println!("  ┌── WISDOM (autoriteti raporton) ─────────────────────────────");
    println!("  │ verdikte:  total={} · pranuar={} · bllokuar={} · frozen={}",
        s.total, s.passed, s.blocked, s.frozen);
    println!("  │ dije:      primitive={} · legacy={} · hipoteza={} · fakte={}",
        s.primitive_count, s.legacy_count, s.hypothesis_count, s.fact_count);
    println!("  │ negative:  {} rrugë të eliminuara (ASET — përshpejtojnë eliminimet)",
        s.negative_count);
    match s.time_degraded {
        true  => println!("  │ KUJDES:    TIME_DEGRADED — ora reale dështoi; lineage me fallback monotonik"),
        false => {}
    }
    println!("  └─────────────────────────────────────────────────────────────");
}


fn main() {
    match process_bridge::dispatch_from_args() {
        Some(code) => std::process::exit(code),
        None => interactive_main(),
    }
}
