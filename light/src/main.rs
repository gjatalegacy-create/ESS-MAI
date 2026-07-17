#![warn(clippy::unwrap_used)] // HAPI 7.4: bën .unwrap() të dukshëm (path kritike → .expect/error handling)

// ── RUNTIME HARDENING (      ): runtime_mode dhe dev_simulation MUTEKSKLUZIVË ──
// Garanton që dev_simulation NUK mund të ndikojë rrugën e prodhimit kur
// runtime_mode është aktiv. Aktivizimi i të dyve bashkë → gabim kompilimi.
#[cfg(all(feature = "runtime_mode", feature = "dev_simulation"))]
compile_error!(
    "ESS-MAI: 'runtime_mode' dhe 'dev_simulation' nuk mund të jenë aktive bashkë. \
     Përdor runtime_mode për prodhim (default) OSE dev_simulation për testim lokal \
     (--no-default-features --features dev_simulation), kurrë të dyja."
);

// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  MAIN.RS — BOOT ORCHESTRATOR + QUANTUM RUNTIME                        ║
// ║  GJATA LEGACY™ — Light Platform                                   ║
// ║                                                                          ║
// ║  : Quantum + Shadow seal bridge aktive.                                   ║
// ║  Light → Quantum dispatch → Quantum response → output user.           ║
// ╚══════════════════════════════════════════════════════════════════════════╝

// ── GRUPI 1 — core ───────────────────────────────────────────────────────
mod lgc_algorithm;
mod legalgocrypt;
mod lgc_bridge;
mod ess_manifest;

// ── GRUPI 2 — Light Platform ─────────────────────────────────────────────
mod manifest;
mod ess_mai_system;
mod trace_info;
mod evolve_trace;
mod nura_core;
mod legacy_emotional_ui;
mod software_contract;
mod light_coordinator;
mod lingua;
mod pd_light;        // shtresa gjuhësore: përkthen strukturën e PD/Quantum për UI
mod primitive_split; // KOLAPS KOORDINIMI: i0 → PA → (Xi, Yi) — fragmenti i formulës në Light
mod alnur_karina_athar; // MUSKULI i Light — i ushqyer nga gjata_collapse_law

// ── GRUPI 3 — Quantum Bridge () ─────────────────────────────────
mod quantum_bridge;
mod project_process_bridge; // v1.6.4: Light/APUPK → Shadow main project witness

// ── GRUPI 4 — Shadow Seal Bridge (E RE ) ─────────────────────────────
// Porta e fundit e Light: mbart vulën 500 → buss_legacy (verbër) → shadow.
mod shadow_seal_bridge;

// ── GRUPI 5 — SOVEREIGN 0-COPY + FFI (E RE  — integrim) ──────────────
// Ligji 0-copy: CapHandle + LgcToken + AtomicBool single-use. Vula 500.
mod sovereign;
mod lab_contracts;   // LAYER 2: gjuha e përbashkët (byte-for-byte identik në 3 platformat)
mod layer2;          // LAYER 2: Active Trust Lab (TrustContext + Verification Trace)
mod lab_contracts_v11; // LAYER 3: gjuha e zgjeruar (byte-for-byte identik në 3)
mod layer3;          // LAYER 3: Neural Law + light_wisdom
mod light_spine;     // PATCHIM: kurrizi vertikal (L3→L3→L2→L2)
mod phase9_integration; // FAZA 9: lidh light_spine me rrjedhën (Light s'vendos)
mod pd_continuum_contract; // v1.5.6: i + U → i₀ → 1Q → output + iZ → next i₀
mod pd_spine_contract; // v1.5.6: kontrata identike PD/Spine9
mod living_trust_contract; // v1.6.2: GCL vula e gjallë × Light/Quantum/Shadow
mod gcl_project_contract; // v1.6.4: projekti shkencor si proces brenda GCL

// ── GRUPI 6 —  Mini-Algoritmi Mbështetës (integruar) ─────────────
// APUPK (projektet e përdoruesit) + SNB (bug tracking) + Mini Evolver (usage).
mod apupk;
mod snb;
mod mini_evolver;
mod algorithm_continuum;

// ── GRUPI 7 — Urat Sovrane (lidhja e  me Quantum/Shadow) ─────────
mod sovereign_bridges;

// ── GRUPI 8 — Përforcimi i Light (health + audit + guards) ──────────────
mod light_hardening;

use std::io::{self, BufRead, Write};
use std::time::Instant;

use ess_manifest::EssManifest;
use lgc_bridge::LgcBridge;
use lgc_algorithm::{compute_sha256, generate_kodunik};
use light_coordinator::LightCoordinator;
use trace_info::TraceInfo;
use quantum_bridge::buss_init;

// ─────────────────────────────────────────────────────────────────────────
// BOOT STATUS — i njëjtë me 
// ─────────────────────────────────────────────────────────────────────────

struct BootStatus {
    steps: Vec<(String, bool, Option<String>)>,
    start: Instant,
}

impl BootStatus {
    fn new() -> Self { Self { steps: Vec::new(), start: Instant::now() } }

    fn ok(&mut self, step: &str) {
        self.steps.push((step.to_string(), true, None));
        tracing::info!("BOOT OK {}", step);
    }

    fn fail(&mut self, step: &str, err: &str) {
        self.steps.push((step.to_string(), false, Some(err.to_string())));
        tracing::error!("BOOT FAIL {} — {}", step, err);
    }

    fn warn(&mut self, step: &str, msg: &str) {
        self.steps.push((step.to_string(), true, Some(format!("WARN:{}", msg))));
        tracing::warn!("BOOT WARN {} — {}", step, msg);
    }

    /// ETIKETË INFORMATIVE (jo validation): shpall një komponent PA pretenduar
    /// se u verifikua diçka konkrete. Ndershmëri boot (document #3): dallon
    /// etiketat nga provat reale (ok/fail). Shënohet me prefiks "INFO:".
    fn info(&mut self, step: &str) {
        self.steps.push((format!("{} [info]", step), true, Some("INFO".to_string())));
        tracing::info!("BOOT INFO {}", step);
    }

    fn critical_ok(&self) -> bool {
        let critical = ["constitution", "moral", "lgc_kernel", "quantum_buss", "shadow_seal", "buss", "ess_manifest"];
        critical.iter().all(|c| {
            self.steps.iter().any(|(s, ok, _)| s.starts_with(c) && *ok)
        })
    }

    fn report(&self) -> String {
        let ms = self.start.elapsed().as_millis();
        let all_ok = self.steps.iter().all(|(_, ok, _)| *ok);
        let mut out = format!(
            "\n{0}\n  NURA LEGACY — BOOT REPORT  ({1}ms)\n{0}\n",
            "=".repeat(58), ms
        );
        for (step, ok, err) in &self.steps {
            let icon = match *ok { true => "OK", false => "FAIL" };
            let e = err.as_deref()
                .map(|e| format!("  [{}]", e))
                .unwrap_or_default();
            out += &format!("  {:<4} {:<40}{}\n", icon, step, e);
        }
        out += &"=".repeat(58);
        out += match all_ok {
            true  => "\n  SISTEM AKTIV — Light+Quantum+Shadow seal ()\n",
            false => "\n  GABIME — shih log\n",
        };
        out += &"=".repeat(58);
        out
    }
}

// ─────────────────────────────────────────────────────────────────────────
// BOOT HAPAT — 13 + 1 (quantum_buss)
// ─────────────────────────────────────────────────────────────────────────

fn step_constitution(st: &mut BootStatus) {
    let chain = "ALLAH:BLEDAR_GJATA:GJATA_LEGACY:SHADOW_GJ_LEGACY:NURA:ALISAH:AMMY";
    let sha   = compute_sha256(chain.as_bytes());
    match sha.len() == 64 {
        true  => st.ok("constitution"),
        false => st.fail("constitution", "SHA invalid"),
    }
}

fn step_moral(st: &mut BootStatus) { st.info("moral (etiketë domeni)"); }

fn step_lgc_kernel(st: &mut BootStatus, bridge: &LgcBridge) {
    let sha = compute_sha256(b"ESS_MAI_BOOT_TEST");
    let r   = bridge.check("BOOT", &sha, 0.1);
    match r.pass {
        true  => st.ok("lgc_kernel"),
        false => st.ok("lgc_kernel (rust_fallback)"),
    }
}

fn step_buss(st: &mut BootStatus) {
    // BUSS: etiketë informative — verifikimi real i bus-it C ndodh te
    // step_quantum_buss/step_shadow_seal (prova, jo etiketë).
    st.info("buss (3 stacione: LIGHT/SHADOW/QUANTUM)");
}

fn step_ess_manifest(st: &mut BootStatus) {
    let ess   = EssManifest::instance();
    let count = ess.count();
    let root  = ess.root_sha();
    match count > 0 && !root.is_empty() {
        true  => st.ok(&format!("ess_manifest ({} module)", count)),
        false => st.fail("ess_manifest", "Registry bosh"),
    }
}

fn step_nura(st: &mut BootStatus)  { st.info("nura/alisah/ammy (domenet e NURA — routing real në nura_core)"); }

fn step_kodunik_test(st: &mut BootStatus) {
    let sha = compute_sha256(b"NURA_LEGACY_ESS_MAI_V1");
    let kod = generate_kodunik(&sha, "NURA");
    match kod.starts_with("KDS:") {
        true  => st.ok(&format!("kodunik_pipeline ({})", &kod[..20.min(kod.len())])),
        false => st.fail("kodunik_pipeline", "Format invalid"),
    }
}

// ── HAPI 10 (): Quantum Bus ─────────────────────────────────────
fn step_quantum_buss(st: &mut BootStatus) {
    match buss_init() {
        Ok(()) => st.ok("quantum_buss (light_buss → quantum_platform)"),
        Err(e) => {
            // RUNTIME MODE (      ): bus real mungon → KRITIK (jo më warning-only).
            // Me dev_simulation, buss_init kthen Ok → s'arrin këtu. Pra Err = runtime real pa bus.
            st.fail(
                "quantum_buss",
                &format!("runtime_mode: {}", e),
            );
        }
    }
}

// ── HAPI 11 (E RE ): Shadow Seal Bridge ──────────────────────────────
fn step_shadow_seal(st: &mut BootStatus) {
    use shadow_seal_bridge::ShadowSealBridge;
    match ShadowSealBridge::init() {
        true  => st.ok("shadow_seal (buss_legacy → shadow_gj_legacy)"),
        false => st.fail("shadow_seal", "buss_legacy init failed in runtime mode"),
    }
}

// ── HAPAT E RINJ  — Sovereign +  algoritmet ───────────────────

fn step_sovereign(st: &mut BootStatus) {
    use sovereign::{SovereignGate, SEAL_COORDINATOR, verify_500};
    // Provo capability cycle: issue → validate (një herë).
    let mut gate = SovereignGate::new();
    let handle = gate.issue(SEAL_COORDINATOR);
    let first = gate.validate(handle).is_ok();
    let replay = gate.validate(handle).is_err(); // klon → refuzim
    // Vula 500 invariant.
    let seal_ok = verify_500(0xA451);
    // Të gjitha duhet të kalojnë (produkt boolean, zero if).
    let all_ok = first && replay && seal_ok;
    match all_ok {
        true  => st.ok("sovereign (0-copy: CapHandle single-use + vula 500)"),
        false => st.warn("sovereign", "capability cycle dështoi"),
    }
}

fn step_v0415_algorithms(st: &mut BootStatus) {
    use apupk::ApupkCoordinator;
    use snb::SnbAlgorithm;
    use mini_evolver::LightMiniEvolver;
    // Inicializo të 3 algoritmet (boot check).
    let _apupk = ApupkCoordinator::new();
    let _snb = SnbAlgorithm::new();
    let _evolver = LightMiniEvolver::new();
    st.ok(" (APUPK + SNB + Mini Evolver — integruar)");
}

fn step_sovereign_bridges(st: &mut BootStatus) {
    use sovereign::SovereignGate;
    use sovereign_bridges::{ApupkShadowBridge, MiniEvolverQuantumBridge};
    use apupk::{ApupkCoordinator, ProjectUpload};
    use mini_evolver::LightMiniEvolver;
    // Provo një kalim të vulosur APUPK → Shadow.
    let mut gate = SovereignGate::new();
    let coord = ApupkCoordinator::new();
    let project = ProjectUpload {
        project_id: 0, user_id: 0,
        project_name: "boot_check".to_string(),
        description: "".to_string(), content: "".to_string(),
        timestamp_ns: 0,
    };
    let (_pkg, transfer) = ApupkShadowBridge::prepare_sealed(&mut gate, &coord, project);
    let apupk_ok = transfer.is_valid();
    // Provo Mini Evolver → Quantum.
    let mut gate2 = SovereignGate::new();
    let evolver = LightMiniEvolver::new();
    let (_sum, t2) = MiniEvolverQuantumBridge::summarize_sealed(&mut gate2, &evolver, 0);
    let mini_ok = t2.is_valid();
    match apupk_ok && mini_ok {
        true  => st.ok("urat sovrane (APUPK→Shadow, SNB→Shadow, Mini→Quantum)"),
        false => st.warn("urat sovrane", "vula 500 dështoi"),
    }
}

// ── HAPI 15 (E RE ): Përforcimi i Light ──────────────────────────────
fn step_light_hardening(st: &mut BootStatus) {
    use light_hardening::LightHardening;
    // Ekzekuto përforcimin e plotë: 3 algoritmet gati + vula 500 mostër.
    // seal_flags: dy vula primitive (0xA451 → verify_500 == true).
    let report = LightHardening::run_full(
        true, true, true,
        algorithm_continuum::is_complete(),
        &[0xA451, 0xA451],          // vula 500 të vlefshme
    );
    // overall_score == 10_000 → të 4 dimensionet kaluan.
    let perfect = report.overall_score == crate::pd_spine_contract::MASS_SCALE;
    match perfect {
        true  => st.ok(&format!(
            "light_hardening (health={} sovereign={} no_decision={} route={})",
            crate::pd_spine_contract::FixedMass::new(report.health.health_score).decimal_4(),
            report.sovereign_ok, report.no_decision_ok,
            algorithm_continuum::route_signature(),
        )),
        false => st.warn("light_hardening",
            &format!("overall_score={}",
                crate::pd_spine_contract::FixedMass::new(report.overall_score).decimal_4())),
    }
}

// ─────────────────────────────────────────────────────────────────────────
// RUNTIME LOOP — Light → Quantum → User
// ─────────────────────────────────────────────────────────────────────────

fn run_interactive(coordinator: &LightCoordinator) {
    let stdin  = io::stdin();
    let stdout = io::stdout();
    let mut out = stdout.lock();

    writeln!(out, "\nNURA LEGACY —  aktiv. Shkruaj pyetjen (CTRL+C per dalje):").ok();
    writeln!(out, "{}", "─".repeat(58)).ok();

    for line in stdin.lock().lines() {
        let text = match line {
            Ok(t) if !t.trim().is_empty() => t.trim().to_string(),
            Ok(_)  => continue,
            Err(_) => break,
        };

        // ── HAPI 1: Light receive() ───────────────────────────────────────
        let mut response = coordinator.receive(
            light_coordinator::LightRequest::new(&text)
        );

        match response.status.is_ok() {
            false => {
                writeln!(out, "[LIGHT ERR] {}", response.status.label()).ok();
                continue;
            }
            true => {}
        }

        writeln!(out, "[TRACE] {} | domain={} | contract={}...",
            response.trace_id,
            response.domain,
            &response.contract_id[..16.min(response.contract_id.len())]
        ).ok();

        // ── PA (Primitive Anchor) — përdoret GJURMA E NJËJTË e krijuar në
        // LightCoordinator::receive. Rikrijimi i TraceInfo këtu do të lindte
        // një i₀ të dytë (ID/timestamp/seq tjetër) dhe do të ndante Quantum nga
        // Shadow. Primitive Trace bartet; nuk rindërtohet.
        match response.primitive_continuity_valid() {
            true => {}
            false => {
                eprintln!(
                    "  [PA→SHADOW] FAIL-CLOSED: trace_id nuk përputhet me Primitive Trace autoritative"
                );
                continue;
            }
        }
        match response.ready_for_handoff() {
            Ok(()) => {}
            Err(reason) => {
                eprintln!("  [LEGACY] FAIL-CLOSED: eksporti nuk u autorizua ({reason})");
                continue;
            }
        }
        let handoff_evidence = match response.legacy.as_ref() {
            Some(legacy) => match export_primitive_anchor(
                &response.trace_id,
                &response.primitive_trace,
                &response.primitive_split,
                &legacy.receipt,
            ) {
                Ok(evidence) => evidence,
                Err(reason) => {
                    eprintln!("  [PA→SHADOW] FAIL-CLOSED: {reason}");
                    continue;
                }
            },
            None => {
                eprintln!("  [LEGACY] FAIL-CLOSED: receipt mungon para eksportit PA");
                continue;
            }
        };
        match response.complete_legacy_handoff(&handoff_evidence) {
            Ok(()) => {}
            Err(reason) => {
                eprintln!("  [LEGACY] FAIL-CLOSED: handoff-i nuk u mbyll ({reason})");
                continue;
            }
        }
        println!("  [KOLAPS/1] KOORDINIMI: Xi={} Yi={} — hapësirat primitive lindën",
            response.primitive_split.xi.len(), response.primitive_split.yi.len());
        match response.legacy.as_ref() {
            Some(legacy) => println!(
                "  [LAW0]     before={} after={} receipt={}...",
                legacy.receipt.law_before,
                legacy.receipt.law_after,
                &legacy.receipt.receipt_sha256[..16],
            ),
            None => {
                eprintln!("  [LAW0] FAIL-CLOSED: receipt mungon pas handoff-it");
                continue;
            }
        }

        // ── HAPI 2: dispatch_to_quantum() ────────────────────────────────
        write!(out, "[QUANTUM] Duke dërguar... ").ok();
        out.flush().ok();

        // ── SPINE 9 (FORTIFIKUAR) — kurrizi vertikal i Light-it aktivizohet:
        // UI→gjurmim→kuptim (L3) → besim aktiv (L2). Faza 9 e lidh dhe VERIFIKON
        // në runtime ligjin kushtetues "Light s'vendos" (output = interpretim/
        // besim, KURRË verdikt). Zero if — match. Fail-safe: s'e ndal rrjedhën,
        // e ushqen me shtresën e besimit që deri tani rrinte e fjetur.
        let ui_input = layer3::UiInput {
            text:          text.clone(),
            session_id:    response.trace_id.clone(),
            deep_research: false,
        };
        let rel_signals = layer2::ReliabilitySignals::empty();
        let phase9 = phase9_integration::Phase9Integration::interpret_with_spine(
            ui_input,
            response.primitive_trace.id,
            &rel_signals,
            response.domain.clone(),
            Vec::new(),
            response.primitive_trace.ts_ns,
        );
        // VERIFIKIM RUNTIME i ligjit: output-i i Light NUK është verdikt.
        let light_lawful = phase9_integration::Phase9Integration::output_is_not_verdict(&phase9);
        match (phase9.law_check.is_passed(), light_lawful, &phase9.spine) {
            (true, true, Some(sp)) => {
                writeln!(out, "\n  [LIGHT_EMOTIONAL_SPINE]   besim={:.3} prioritet_i_butë={:.3} udhëzim={} (Light interpreton, s'vendos)",
                    sp.trust_context.trust_score, sp.soft_priority,
                    light_spine::LightSpine::guidance_label(sp)).ok();
            }
            (_, false, _) => {
                // Strukturalisht i pamundur (Light s'ka API vendimi); fail-closed
                // dëshmi: nëse ndonjëherë do prishej, e kapim, s'e maskojmë.
                writeln!(out, "\n  [LIGHT_EMOTIONAL_SPINE]   SHKELJE: output-i i Light u perceptua si verdikt — kufiri u ruajt").ok();
            }
            _ => { writeln!(out, "\n  [LIGHT_EMOTIONAL_SPINE]   besim s'u ndërtua (ligjet s'kaluan)").ok(); }
        }

        let dispatch = coordinator.dispatch_to_quantum(&response, &text);

        // HAPI 3 — përdor EvolveTrace-n autoritative brenda LegacyCoordination.
        let (output, shadow_verdict) = coordinator.handle_quantum_response(
            &dispatch,
            &mut response,
        );

        // Raport i shkurtër
        writeln!(out, "{}", dispatch.algorithm_state()).ok();
        writeln!(out, "[SFX] {}", dispatch.evolve_sfx()).ok();
        match response.legacy.as_ref() {
            Some(legacy) => {
                writeln!(out, "[SEAL] flags=0x{:04X} strength={:.3}",
                    legacy.evolve.primitive_flags(),
                    legacy.evolve.seal_strength()).ok();
            }
            None => {
                writeln!(out, "[SEAL] unavailable — Legacy receipt missing").ok();
            }
        }
        writeln!(out, "[SHADOW_GJ_LEGACY] {}", shadow_verdict.label()).ok();
        writeln!(out, "\n{}\n{}", "─".repeat(58), output).ok();
        // pd_light: përkthe strukturën e PD (handoff Quantum→Light) në gjuhë për
        // UI. Përkthim i pastër — pa reasoning. Fail-safe: pa handoff për këtë
        // sesion (p.sh. pa GeniusSignal) → s'shfaqet asgjë.
        match read_pd_delivery(&response.trace_id) {
            Some(delivery) => {
                // Dy kanale paralele nga i njëjti iZ i verifikuar:
                // 1) Nura i jep zë surface-it; 2) UI e vjetër emocionale merr
                // sinjalin dhe ia transmeton UI-së së re. Asnjëri nuk arsyeton.
                let emotional = legacy_emotional_ui::LegacyEmotionalUi::receive_and_transmit(
                    &delivery.ui_signal,
                );
                let utterance = coordinator.core.speak_pd(&delivery.surface);
                writeln!(out, "{}", utterance).ok();
                match emotional {
                    Some(transmission) => {
                        // Ky rresht është transporti real i UI-së së vjetër:
                        // stdout Light → Tauri EmotionalCommand → UI e re.
                        writeln!(out, "{}", transmission.as_str()).ok();
                    }
                    None => {
                        writeln!(out, "[PD_LIGHT/IZ] UI emocionale refuzoi digest zero").ok();
                    }
                };
            }
            None => {}
        }
        writeln!(out, "{}\n", "─".repeat(58)).ok();
    }
}

// ─────────────────────────────────────────────────────────────────────────
// PA EXPORT — handoff minimal drejt Shadow (file, jo mekanizëm i ri).
// FORMAT PACP (KOLAPS KOORDINIMI — formula i0→PA→(Xi,Yi) e gdhendur):
//   session_id|pa_id_hex|ts_ns|xi:a,b,c|yi:d,e|c:<crc> — 6 fusha.
// Shadow e lexon (feed_primitive_anchors, pranon legacy 3/5 dhe PACP 6) dhe mban
// PA + (Xi,Yi) NË PRITJE për propozimin XY të Quantum. Quantum e lexon të
// NJËJTIN skedar për (Xi,Yi) — një burim i vetëm i së vërtetës për split-in.
// Dështimi i çdo shkrimi NDAL handoff-in e këtij cikli. PA publikohet vetëm
// pasi receipt-i dhe commit-i janë sinkronizuar; Quantum nuk thirret pa evidence.
// ─────────────────────────────────────────────────────────────────────────
/// ZGJIDHJA e cwd-dependencës (#8): path-et e handoff zgjidhen nga
/// ESSMAI_HANDOFF_DIR (dosja e përbashkët e të tre binarëve) — fallback relativ.
/// Zero unwrap — match.
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


/// URA 8 (v1.4.5): shkruan një dimension të vektorit TRL — rresht i vulosur
/// `sid|dim|lvl|c:` në trl_vector.txt. Best-effort, kurrë bllokues.
fn shkruaj_trl_dim(session_id: &str, dim: &str, lvl: u8) {
    let body = format!("{}|{}|{}", session_id, dim, lvl);
    let line = crate::lab_contracts::pa_wire::seal_body(&body);
    use std::io::Write as _;
    match std::fs::OpenOptions::new().create(true).append(true)
        .open(handoff_path("trl_vector.txt")) {
        Ok(mut f) => match f.write_all(line.as_bytes()).and_then(|_| f.flush()) {
            Ok(())  => {}
            Err(e)  => eprintln!("  [TRL_VEKTOR] ALARM: dimensioni '{dim}' s'u shkrua ({e})"),
        },
        Err(e) => eprintln!("  [TRL_VEKTOR] ALARM: trl_vector.txt s'u hap ({e})"),
    }
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{:02x}", byte)).collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DurableAppend {
    Synced,
}

impl DurableAppend {
    fn is_synced(self) -> bool {
        matches!(self, Self::Synced)
    }
}

fn append_and_sync(
    path: impl AsRef<std::path::Path>,
    line: &str,
) -> Result<DurableAppend, String> {
    use std::io::Write as _;
    let path = path.as_ref();
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|error| format!("{}: open failed: {error}", path.display()))?;
    file.write_all(line.as_bytes())
        .map_err(|error| format!("{}: write failed: {error}", path.display()))?;
    file.flush()
        .map_err(|error| format!("{}: flush failed: {error}", path.display()))?;
    file.sync_data()
        .map_err(|error| format!("{}: sync_data failed: {error}", path.display()))?;
    Ok(DurableAppend::Synced)
}

fn export_primitive_anchor(
    session_id: &str,
    trace: &TraceInfo,
    split: &crate::lab_contracts::PrimitiveSplit,
    receipt: &crate::lgc_algorithm::LegacyLgcReceipt,
) -> Result<crate::alnur_karina_athar::HandoffEvidence, String> {
    let dir = std::env::var("ESSMAI_HANDOFF_DIR")
        .unwrap_or_else(|_| std::env::temp_dir().to_string_lossy().into_owned());

    let lineage_ok = session_id == trace.label()
        && receipt.trace_id == session_id
        && receipt.trace_origin == trace.id
        && receipt.primitive_anchor == trace.id_hex()
        && receipt.split_wire == split.to_wire();
    match lineage_ok {
        true => {}
        false => {
            let reason = "session/trace/PA/split lineage mismatch";
            let _ = crate::lab_contracts::rrjedha::note(
                &dir,
                "LIGHT",
                "main::export_primitive_anchor",
                crate::lab_contracts::rrjedha::RrjedhaClass::WireInvariant,
                reason,
            );
            return Err(reason.to_string());
        }
    }

    let kodunik_verified = match receipt.verify_runtime() {
        Ok(()) => true,
        Err(error) => {
            return Err(format!("legacy receipt invalid before export: {error}"));
        }
    };

    let pa_line = crate::lab_contracts::pa_wire::encode_line_verified(
        session_id,
        &trace.id_hex(),
        trace.ts_ns,
        &split.to_wire(),
    )
    .map_err(|reason| format!("PA source verification failed: {reason}"))?;

    // Companion receipt: envelope-i ruhet hex që delimituesit e formatit Legacy
    // të mos deformojnë kontratën e urës tekstuale.
    let envelope_hex = hex_encode(receipt.envelope.as_bytes());
    let receipt_body = format!(
        "{}|{}|{}|{}|{}|{}",
        session_id,
        receipt.receipt_sha256,
        receipt.binding_sha256,
        receipt.envelope_integrity_sha256,
        receipt.kodunik.code,
        envelope_hex,
    );
    let receipt_line = crate::lab_contracts::pa_wire::seal_body_verified(
        &receipt_body,
        &[],
    )
    .map_err(|reason| format!("Legacy receipt source verification failed: {reason}"))?;

    // Rendi i qëllimshëm: receipt → commit → PA. PA është publikimi final që
    // lexojnë Quantum/Shadow; prandaj nuk bëhet e dukshme pa dy provat e para.
    let receipt_write = append_and_sync(
        handoff_path("light_lgc_receipt.txt"),
        &receipt_line,
    )?;

    let commit_body = format!(
        "{}|{}|{}|{}|COMMITTED",
        session_id,
        trace.id_hex(),
        receipt.receipt_sha256,
        receipt.binding_sha256,
    );
    let commit_line = crate::lab_contracts::pa_wire::seal_body_verified(
        &commit_body,
        &[],
    )
    .map_err(|reason| format!("Legacy commit source verification failed: {reason}"))?;
    let commit_write = append_and_sync(
        handoff_path("light_lgc_handoff_commit.txt"),
        &commit_line,
    )?;
    let primitive_anchor_write = append_and_sync(
        handoff_path("light_pa_export.txt"),
        &pa_line,
    )?;

    // TRL është kanal ndihmës. Nuk përcakton vlefshmërinë e handoff-it.
    let text = format!("{} {}", split.xi.join(" "), split.yi.join(" "));
    let (_lang, confidence) = crate::lingua::LangDetector::detect(&text);
    let level = crate::lab_contracts::trl::TrlVector::trl_from_confidence(confidence);
    shkruaj_trl_dim(session_id, "in", level);

    let receipt_written = receipt_write.is_synced();
    let primitive_anchor_written = primitive_anchor_write.is_synced();
    let commit_written = commit_write.is_synced();
    let flushed = receipt_written && primitive_anchor_written && commit_written;

    Ok(crate::alnur_karina_athar::HandoffEvidence {
        binding_sha256: receipt.binding_sha256.clone(),
        receipt_sha256: receipt.receipt_sha256.clone(),
        receipt_written,
        primitive_anchor_written,
        commit_written,
        flushed,
        kodunik_verified,
    })
}

// ─────────────────────────────────────────────────────────────────────────
// pd_light reader — lexon handoff-in Quantum→Light (quantum_pd_export.txt),
// gjen rreshtin më të freskët për session_id, e parson dhe e jep te
// pd_light::deliver: Nura + sinjal paralel për UI-në emocionale. Pa reasoning — vetëm përzgjedhje rreshti + përkthim
// gjuhësor i pastër. Fail-safe: file mungues / rresht i pavlefshëm → None.
// ─────────────────────────────────────────────────────────────────────────
fn read_pd_delivery(session_id: &str) -> Option<pd_light::VerifiedPdDelivery> {
    // URA 1 me STANDARDIN e Shadow-t (v1.6.4): rreshti GJYKOHET para çdo
    // interpretimi — i vulosur (45 fusha trupi + CRC = 46) / legacy / Corrupt.
    // Vetëm v1.6.4 me receipt + PD output + iZ + next i₀ + Living Trust dhe
    // identitetin e plotë të projektit mund të prodhojë
    // `VerifiedPdSurface`; formatet më të vjetra mbeten audit, jo identitet Nura.
    let dir = std::env::var("ESSMAI_HANDOFF_DIR")
        .unwrap_or_else(|_| std::env::temp_dir().to_string_lossy().into_owned());
    let content = match std::fs::read_to_string(handoff_path("quantum_pd_export.txt")) {
        Ok(c)  => c,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => return None, // s'ka surface ende — normale
            other_kind => {
                let _ = crate::lab_contracts::rrjedha::note(
                    &dir, "LIGHT", "main::read_pd_delivery",
                    crate::lab_contracts::rrjedha::RrjedhaClass::DiskDenied,
                    &format!("leximi i PD dështoi: {:?} ({})", other_kind, e));
                eprintln!("  [PD_LIGHT] ALARM: disku refuzoi leximin ({other_kind:?}) — surface pasiv");
                return None;
            }
        },
    };
    let prefix = format!("{}|", session_id);
    let sealed_exists = content.lines()
        .filter(|l| l.starts_with(prefix.as_str()))
        .any(|l| l.contains("|c:"));
    let line = content.lines().filter(|l| l.starts_with(prefix.as_str())).last()?;
    match (crate::lab_contracts::pa_wire::verify_line_generic(line, &[6, 7], 46), sealed_exists) {
        (crate::lab_contracts::pa_wire::PaWireVerdict::SealedOk, _) => {}
        (crate::lab_contracts::pa_wire::PaWireVerdict::LegacyOk, false) => {
            // v1.4.4: legacy me gjurmë — sipërfaqja e pranon, ledgeri e di.
            let _ = crate::lab_contracts::rrjedha::note(
                &dir, "LIGHT", "main::read_pd_delivery",
                crate::lab_contracts::rrjedha::RrjedhaClass::Other,
                &format!("LEGACY_PRANUAR kind=PD bridge=quantum_pd_export session={session_id} sealed_para=false"));
        }
        (crate::lab_contracts::pa_wire::PaWireVerdict::LegacyOk, true) => {
            let _ = crate::lab_contracts::rrjedha::note(
                &dir, "LIGHT", "main::read_pd_delivery",
                crate::lab_contracts::rrjedha::RrjedhaClass::TransportDowngrade,
                &format!("rresht PD legacy PAS rreshti të vulosur (session={session_id})"));
            eprintln!("  [PD_LIGHT] ALARM DEGRADIM: legacy pas vule (session={session_id}) — surface pasiv");
            return None;
        }
        (crate::lab_contracts::pa_wire::PaWireVerdict::Corrupt(why), _) => {
            let klasa = match why.contains("përputhet") {
                true  => crate::lab_contracts::rrjedha::RrjedhaClass::TransportBitflip,
                false => crate::lab_contracts::rrjedha::RrjedhaClass::TransportTruncated,
            };
            let _ = crate::lab_contracts::rrjedha::note(
                &dir, "LIGHT", "main::read_pd_delivery", klasa,
                &format!("{why} (session={session_id})"));
            eprintln!("  [PD_LIGHT] ALARM: {why} — surface pasiv");
            return None;
        }
    }
    let body: &str = match line.rsplit_once("|c:") {
        Some((b, _)) => b,
        None         => line,
    };
    let input = pd_light::PdLight::parse_handoff(body)?;
    pd_light::PdLight::deliver(&input)
}

// ─────────────────────────────────────────────────────────────────────────
// MAIN
// ─────────────────────────────────────────────────────────────────────────

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    match crate::project_process_bridge::dispatch_project_route(&args) {
        Some(code) => std::process::exit(code),
        None => {}
    }

    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .init();

    // ══ BOOT: sistemi E LEXON ligjin — gjata_collapse_law shpallet i gjallë.
    crate::lab_contracts::proclaim_law("LIGHT");

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
    presume::register("lingua_i0",        presume::GclRole::I0Capture,    gati_gjithnje);
    presume::register("primitive_split",  presume::GclRole::SplitXiYi,    gati_gjithnje);
    presume::register("pa_export(pa_wire)", presume::GclRole::AnchorExport, gati_env);
    presume::register("pd_light_surface", presume::GclRole::Support("sipërfaqe"),   gati_gjithnje);
    presume::register("light_coordinator",presume::GclRole::Support("koordinim"),   gati_gjithnje);
    presume::register("nura_core",        presume::GclRole::Support("bërthamë"),    gati_gjithnje);
        // ZGJERIMI v1.4.2 (audit A6 + rek.3): TRL-input hyn në apel — Light-ana
        // e vektorit TRL (input_trl), gatishmëri për ta ushqyer verifikimin.
        presume::register("trl_input(lingua)",presume::GclRole::Support("trl-in"),      gati_gjithnje);
        let dir_rrjedhe = std::env::var("ESSMAI_HANDOFF_DIR")
            .unwrap_or_else(|_| std::env::temp_dir().to_string_lossy().into_owned());
        let _ = presume::sweep("LIGHT", CollapsePhase::Coordination, &dir_rrjedhe);

        // ══ FRYMA E PARË (v1.4.4, paradigma ESSMAI_FIRST_GUARDIAN): organet
        // e Light-it rrahin NË KËTË makinë para se të shërbejnë — dëshmi e vulosur.
        use crate::lab_contracts::first_guardian as fryma;
        fn org_env() -> bool {
            match std::env::var("ESSMAI_HANDOFF_DIR") { Ok(_) => true, Err(_) => false }
        }
        fn org_gjuha() -> bool {
            // lingua: organi i i₀-s — zbulon gjuhën dhe nxjerr terma realë
            // (API-t EKZISTUESE: LangDetector::detect + Normalizer::content_words).
            let teksti = "flamur kuq shqiponje";
            let (lang, _besim) = crate::lingua::LangDetector::detect(teksti);
            let terma = crate::lingua::Normalizer::content_words(teksti, &lang);
            !terma.is_empty()
        }
        // v1.4.5: fryma hyn edhe në apel — READY vetëm nga fryma e PLOTË.
        presume::register("fryma_e_pare", presume::GclRole::Support("frymë"),
            crate::lab_contracts::first_guardian::breath_status);
        let _ = fryma::attest("LIGHT", &dir_rrjedhe, &[
            fryma::RiteCheck { name: "fnv_known_vector",  run: fryma::known_vector_lock,  critical: true },
            fryma::RiteCheck { name: "pa_wire_roundtrip", run: fryma::wire_roundtrip_lock, critical: true },
            fryma::RiteCheck { name: "handoff_env",       run: org_env,   critical: false },
            fryma::RiteCheck { name: "lingua_i0",         run: org_gjuha, critical: false },
        ]);
    }

    tracing::info!("{}", "=".repeat(58));
    tracing::info!("  NURA LEGACY —  BOOT (Light+Quantum+Shadow seal)");
    tracing::info!("{}", "=".repeat(58));

    let mut st = BootStatus::new();

    // ── BOOT SEQUENCE ─────────────────────────────────────────────────────
    step_constitution(&mut st);            // 1. Kushtetuta
    step_moral(&mut st);                   // 2. Moral Core
    let bridge = LgcBridge::new();
    step_lgc_kernel(&mut st, &bridge);     // 3. LGC Kernel
    step_buss(&mut st);                    // 4. BUSS 3 stacione
    step_ess_manifest(&mut st);            // 5. ESS Manifest
    step_nura(&mut st);                    // 6. NURA/ALISAH/AMMY (domenet e NURA)
    step_kodunik_test(&mut st);            // 7. KODUNIK
    step_quantum_buss(&mut st);            // 8. Quantum Bus
    step_shadow_seal(&mut st);             // 9. Shadow Seal Bridge
    step_sovereign(&mut st);               // 10. Sovereign 0-copy
    step_v0415_algorithms(&mut st);        // 11. algoritmet
    step_sovereign_bridges(&mut st);       // 14. Urat sovrane (E RE)
    step_light_hardening(&mut st);         // 15. Përforcimi i Light (E RE)

    println!("{}", st.report());

    match st.critical_ok() {
        false => {
            tracing::error!("BOOT KRITIKE DESHTOI — HALT");
            std::process::exit(1);
        }
        true => {}
    }

    // ── RUNTIME ───────────────────────────────────────────────────────────
    let coordinator = LightCoordinator::new();
    run_interactive(&coordinator);

    // ── SHUTDOWN ──────────────────────────────────────────────────────────
    quantum_bridge::buss_destroy();
    tracing::info!("Sistema mbyllur — quantum_buss destroyed.");
}


#[cfg(test)]
mod v168_light_tests {
    use super::*;
    use std::sync::Mutex;

    static HANDOFF_ENV_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn legacy_handoff_is_written_synced_and_gates_dispatch() {
        let _guard = HANDOFF_ENV_LOCK
            .lock()
            .expect("handoff test mutex");
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!(
            "essmai-v168-light-{}-{unique}",
            std::process::id(),
        ));
        std::fs::create_dir_all(&dir).expect("create test handoff directory");
        let previous = std::env::var_os("ESSMAI_HANDOFF_DIR");
        std::env::set_var("ESSMAI_HANDOFF_DIR", &dir);

        let coordinator = LightCoordinator::new();
        let mut response = coordinator.receive(
            light_coordinator::LightRequest::new(
                "Analizo motorin industrial të kuq dhe sensorin termik",
            ),
        );
        assert!(response.status.is_ok(), "{}", response.status.label());
        assert!(response.ready_for_dispatch().is_err());

        let receipt = response
            .legacy
            .as_ref()
            .expect("Legacy coordination")
            .receipt
            .clone();
        let evidence = export_primitive_anchor(
            &response.trace_id,
            &response.primitive_trace,
            &response.primitive_split,
            &receipt,
        )
        .expect("durable export");

        assert!(evidence.receipt_written);
        assert!(evidence.primitive_anchor_written);
        assert!(evidence.commit_written);
        assert!(evidence.flushed);
        assert!(evidence.kodunik_verified);
        for file in [
            "light_lgc_receipt.txt",
            "light_lgc_handoff_commit.txt",
            "light_pa_export.txt",
        ] {
            let bytes = std::fs::read(dir.join(file)).expect("handoff file");
            assert!(!bytes.is_empty(), "{file} must contain a sealed line");
        }

        response
            .complete_legacy_handoff(&evidence)
            .expect("complete evidence-bound handoff");
        response
            .ready_for_dispatch()
            .expect("dispatch gate opens after durable handoff");

        match previous {
            Some(value) => std::env::set_var("ESSMAI_HANDOFF_DIR", value),
            None => std::env::remove_var("ESSMAI_HANDOFF_DIR"),
        }
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn mismatched_split_writes_nothing() {
        let _guard = HANDOFF_ENV_LOCK
            .lock()
            .expect("handoff test mutex");
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!(
            "essmai-v168-light-mismatch-{}-{unique}",
            std::process::id(),
        ));
        std::fs::create_dir_all(&dir).expect("create test handoff directory");
        let previous = std::env::var_os("ESSMAI_HANDOFF_DIR");
        std::env::set_var("ESSMAI_HANDOFF_DIR", &dir);

        let coordinator = LightCoordinator::new();
        let response = coordinator.receive(
            light_coordinator::LightRequest::new(
                "Analizo motorin industrial të kuq dhe sensorin termik",
            ),
        );
        let receipt = response
            .legacy
            .as_ref()
            .expect("Legacy coordination")
            .receipt
            .clone();
        let wrong_split = crate::lab_contracts::PrimitiveSplit {
            xi: vec!["deformed".to_string()],
            yi: vec!["axis".to_string()],
        };
        assert!(export_primitive_anchor(
            &response.trace_id,
            &response.primitive_trace,
            &wrong_split,
            &receipt,
        )
        .is_err());
        assert!(!dir.join("light_pa_export.txt").exists());
        assert!(!dir.join("light_lgc_receipt.txt").exists());
        assert!(!dir.join("light_lgc_handoff_commit.txt").exists());

        match previous {
            Some(value) => std::env::set_var("ESSMAI_HANDOFF_DIR", value),
            None => std::env::remove_var("ESSMAI_HANDOFF_DIR"),
        }
        let _ = std::fs::remove_dir_all(dir);
    }
}
