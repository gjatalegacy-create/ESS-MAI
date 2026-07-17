// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  ESSMAI_FIRST_GUARDIAN — DËSHMIA E FRYMËS SË PARË (kontratë ×3)          ║
// ║  GJATA LEGACY™ | Paradigma: Gardiani i ESS-MAI | Arkitekt: Bledar Gjata  ║
// ║                                                                          ║
// ║  PARADIGMA (e nuhatur nga çdo gjetje-kurorë e këtij udhëtimi):           ║
// ║  Laku GCL i vdekur në urë. Lexuesit e verbër ndaj vulës. Thirrja që      ║
// ║  s'kompilonte. Termiku përjetësisht i ftohtë. TË GJITHA kishin një       ║
// ║  emërues: ORGANI JETONTE NË TEST, POR S'MERRTE FRYMË NË GJAK.            ║
// ║                                                                          ║
// ║  LIGJI I RI: në çdo ndezje, PARA se të shërbejë, sistemi kryen RITIN     ║
// ║  E FRYMËS — ushtron vetë organet e veta kritike NË KËTË PROCES, NË KËTË  ║
// ║  MAKINË (jo në CI, jo në premtim): çekiçin fnv mbi vektorin e njohur,    ║
// ║  rrethin encode→verify të urës, farkën e token-it, ligjin termik...      ║
// ║  Pastaj VULOS dëshminë në `first_guardian.txt` me vetë-CRC. Sistemi që   ║
// ║  merr frymë e provon frymën; ai që s'e provon dot, e DEKLARON me zë.     ║
// ║                                                                          ║
// ║  LIGJET: zero varësi · zero if/else · zero unwrap · KURRË bllokues       ║
// ║  (dëshmia informon; edhe DEGRADED, runtime vazhdon — por bota e di).     ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use super::pa_wire;

/// Një organ i ritit: emri + ushtrimi (fn pointer i pastër → bool).
#[derive(Clone, Copy)]
pub struct RiteCheck {
    pub name: &'static str,
    pub run:  fn() -> bool,
    /// v1.4.5 (audit p.1): riti KRITIK (themeli i vulave — fnv, ura).
    /// Dështimi i tij = DEGRADED_HARD (jo thjesht alarm), por runtime
    /// VAZHDON — fryma informon, Shadow mbetet i vetmi mur.
    pub critical: bool,
}

/// Dëshmia e frymës — kthehet te thirrësi, KURRË s'urdhëron.
#[derive(Debug)]
pub struct BreathReport {
    pub total:  usize,
    pub ok:     usize,
    pub failed: Vec<&'static str>,
    /// Ritet KRITIKE që dështuan (nënbashkësi e `failed`).
    pub critical_failed: Vec<&'static str>,
    /// Rreshti i vulosur që u shkrua (ose do të shkruhej) në ledger.
    pub sealed_line: String,
}

/// Shkalla e frymës — tre nivele, zero if/else.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BreathGrade { Plote, Degraded, DegradedHard }

impl BreathGrade {
    pub fn label(&self) -> &'static str {
        match self {
            BreathGrade::Plote        => "PLOTE",
            BreathGrade::Degraded     => "DEGRADED",
            BreathGrade::DegradedHard => "DEGRADED_HARD",
        }
    }
}

impl BreathReport {
    /// Fryma e plotë: çdo organ rrahu.
    pub fn full_breath(&self) -> bool { self.ok == self.total }
    /// Shkalla: kritik i dështuar ⇒ HARD; ndonjë dështim ⇒ DEGRADED; ndryshe PLOTE.
    pub fn grade(&self) -> BreathGrade {
        match (self.critical_failed.is_empty(), self.failed.is_empty()) {
            (false, _)    => BreathGrade::DegradedHard,
            (true, false) => BreathGrade::Degraded,
            (true, true)  => BreathGrade::Plote,
        }
    }
}

/// Gjendja e frymës së fundit — e lexueshëme nga gcl_presume (0=s'ka rit,
/// 1=PLOTE, 2=DEGRADED, 3=HARD). Fryma informon, kurrë s'urdhëron.
static FRYMA_E_FUNDIT: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);

/// Statusi për gcl_presume — READY vetëm nga fryma e PLOTË reale.
pub fn breath_status() -> super::gcl_presume::GclReadiness {
    use super::gcl_presume::GclReadiness;
    match FRYMA_E_FUNDIT.load(std::sync::atomic::Ordering::SeqCst) {
        1 => GclReadiness::Ready,
        2 => GclReadiness::Degraded("fryma DEGRADED — organ jo-kritik s'rrahu"),
        3 => GclReadiness::NotReady("fryma DEGRADED_HARD — themeli i vulave s'rrahu (fnv/urë)"),
        _ => GclReadiness::Degraded("riti i frymës s'ka ndodhur ende"),
    }
}

fn tani_ns() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0)
}

// ════════════════════════════════════════════════════════════════════════════
// PATCHIMI PËRFORCUES (v1.4.4): KNOWN-VECTOR LIVE LOCK
// CI-ja e provon në repo; RITI e riprovon NË MAKINË, në çdo ndezje —
// drift kompilatori/arkitekture mbi çekiçin fnv kapet aty ku rreh metali.
// ════════════════════════════════════════════════════════════════════════════

/// Çekiçi fnv1a64 mbi vektorin e njohur — i njëjtë kudo, përgjithmonë.
pub fn known_vector_lock() -> bool {
    pa_wire::fnv1a64(b"a") == 0xaf63_dc4c_8601_ec8c
}

/// Rrethi i plotë i urës: encode i vetë-gjykuar → verify i gjykatësit.
pub fn wire_roundtrip_lock() -> bool {
    match pa_wire::encode_line_verified(
        "FRYMA", "00000000000000f1", 1, "xi:zjarr,jete|yi:makine") {
        Err(_)  => false,
        Ok(l)   => match pa_wire::verify_line(l.trim_end_matches('\n')) {
            pa_wire::PaWireVerdict::SealedOk => true,
            _                                => false,
        },
    }
}

/// RITI I FRYMËS: ushtro organet, vulos dëshminë, shpalle me zë.
/// Formati i rreshtit (i vetë-vulosur me të njëjtin ligj '|c:'):
///   ts_ns|platform|FRYMA|ok/total|te_deshtuarit_csv_ose_-|c:%016x
pub fn attest(
    platform: &'static str,
    dir:      &str,
    checks:   &[RiteCheck],
) -> BreathReport {
    println!("  [FIRST_GUARDIAN] {} — riti i frymës së parë ({} organe)…",
        platform, checks.len());
    let mut ok: usize = 0;
    let mut failed: Vec<&'static str> = Vec::new();
    let mut critical_failed: Vec<&'static str> = Vec::new();
    for c in checks.iter() {
        let rrahu = (c.run)();
        match rrahu {
            true  => { ok += 1; println!("  [FIRST_GUARDIAN]   ✓ {}", c.name); }
            false => {
                failed.push(c.name);
                match c.critical {
                    true  => critical_failed.push(c.name),
                    false => {}
                }
                eprintln!("  [FIRST_GUARDIAN]   ✗ {} — organi S'RRAHU në këtë makinë", c.name);
            }
        }
    }
    let te_deshtuarit = match failed.is_empty() {
        true  => "-".to_string(),
        false => failed.join(","),
    };
    let raporti_paraprak = BreathReport {
        total: checks.len(), ok,
        failed: failed.clone(), critical_failed: critical_failed.clone(),
        sealed_line: String::new(),
    };
    let shkalla = raporti_paraprak.grade();
    let body = format!("{}|{}|FRYMA|{}/{}|{}|{}",
        tani_ns(), platform, ok, checks.len(), te_deshtuarit, shkalla.label());
    let sealed_line = pa_wire::seal_body(&body);

    // v1.4.5 (audit p.2): SKEDAR PER-PLATFORMË — tri ndezje paralele
    // s'ndërthuren kurrë; append+flush mbetet, platforma edhe në trup.
    let emri = match platform {
        "LIGHT"   => "first_guardian_light.txt",
        "QUANTUM" => "first_guardian_quantum.txt",
        "SHADOW"  => "first_guardian_shadow.txt",
        _         => "first_guardian_test.txt",
    };
    let path = format!("{}/{}", dir.trim_end_matches(['/', '\\']), emri);
    use std::io::Write as _;
    match std::fs::OpenOptions::new().create(true).append(true).open(&path) {
        Ok(mut f) => match f.write_all(sealed_line.as_bytes()).and_then(|_| f.flush()) {
            Ok(())  => {}
            Err(e)  => eprintln!("  [FIRST_GUARDIAN] ALARM: dëshmia s'u shkrua ({e}) — fryma mbetet, letra humbi"),
        },
        Err(e) => eprintln!("  [FIRST_GUARDIAN] ALARM: {} s'u hap ({e})", path),
    }

    // Fryma e cunguar shënohet edhe në rrjedha (PSE + kufi), me zë.
    // v1.4.5: KRITIKU i dështuar = WIRE_INVARIANT (kufi 0) — themeli i
    // vulave s'rrahu; jo-kritiku = Other. Runtime VAZHDON në të dyja.
    match shkalla {
        BreathGrade::Plote => println!(
            "  [FIRST_GUARDIAN] {} — FRYMA E PARË ✓ ({}/{}) — dëshmia u vulos",
            platform, ok, checks.len()),
        BreathGrade::Degraded => {
            let _ = super::rrjedha::note(
                dir, platform, "first_guardian::attest",
                super::rrjedha::RrjedhaClass::Other,
                &format!("FRYMA DEGRADED: {}/{} — organet: {}", ok, checks.len(), te_deshtuarit));
            eprintln!("  [FIRST_GUARDIAN] {} — FRYMA DEGRADED ({}/{}) — runtime vazhdon, bota e di",
                platform, ok, checks.len());
        }
        BreathGrade::DegradedHard => {
            let _ = super::rrjedha::note(
                dir, platform, "first_guardian::attest",
                super::rrjedha::RrjedhaClass::WireInvariant,
                &format!("FRYMA DEGRADED_HARD: themeli i vulave s'rrahu — {}",
                    critical_failed.join(",")));
            eprintln!("  [FIRST_GUARDIAN] {} — ⛔ FRYMA DEGRADED_HARD ({}/{}) — fnv/ura s'rrahën; runtime vazhdon i shënuar, Shadow mbetet muri",
                platform, ok, checks.len());
        }
    }
    let kodi: u8 = match shkalla {
        BreathGrade::Plote        => 1,
        BreathGrade::Degraded     => 2,
        BreathGrade::DegradedHard => 3,
    };
    FRYMA_E_FUNDIT.store(kodi, std::sync::atomic::Ordering::SeqCst);

    BreathReport { total: checks.len(), ok, failed, critical_failed, sealed_line }
}

// ════════════════════════════════════════════════════════════════════════════
// TESTE — riti provohet në SECILËN platformë (×3, byte-identike)
// ════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod first_guardian_tests {
    use super::*;

    fn rreh() -> bool { true }
    fn s_rreh() -> bool { false }
    fn tdir() -> String { std::env::temp_dir().to_string_lossy().into_owned() }

    #[test]
    fn live_locks_hold_on_this_machine() {
        // Patchimi përforcues: çekiçi dhe rrethi i urës rrahin KËTU.
        assert!(known_vector_lock(), "fnv1a64 devijoi nga vektori i njohur — drift!");
        assert!(wire_roundtrip_lock(), "encode→verify s'u mbyll — ura e thyer!");
    }

    #[test]
    fn full_breath_when_all_organs_beat() {
        let checks = [
            RiteCheck { name: "organ_a", run: rreh, critical: true },
            RiteCheck { name: "organ_b", run: rreh, critical: false },
        ];
        let r = attest("TEST", &tdir(), &checks);
        assert!(r.full_breath());
        assert_eq!((r.ok, r.total), (2, 2));
        assert!(r.failed.is_empty());
    }

    #[test]
    fn degraded_breath_names_the_dead_organ() {
        let checks = [
            RiteCheck { name: "i_gjalli", run: rreh, critical: false },
            RiteCheck { name: "i_vdekuri", run: s_rreh, critical: false },
        ];
        let r = attest("TEST", &tdir(), &checks);
        assert!(!r.full_breath());
        assert_eq!(r.failed, vec!["i_vdekuri"], "i vdekuri EMËROHET, s'fshihet");
        assert!(r.critical_failed.is_empty(), "jo-kritiku s'e nxin themelin");
        assert_eq!(r.grade(), BreathGrade::Degraded);
        assert!(r.sealed_line.contains("|1/2|"));
        assert!(r.sealed_line.contains("i_vdekuri"));
        assert!(r.sealed_line.contains("|DEGRADED|c:"), "shkalla në trupin e vulosur");
    }

    #[test]
    fn critical_failure_grades_hard_and_status_notready() {
        let r = attest("TEST", &tdir(), &[
            RiteCheck { name: "themeli", run: s_rreh, critical: true },
            RiteCheck { name: "anësori", run: rreh,  critical: false },
        ]);
        assert_eq!(r.grade(), BreathGrade::DegradedHard);
        assert_eq!(r.critical_failed, vec!["themeli"]);
        assert!(r.sealed_line.contains("|DEGRADED_HARD|c:"));
        use super::super::gcl_presume::GclReadiness;
        match breath_status() {
            GclReadiness::NotReady(_) => {}
            other => panic!("HARD duhej NOT_READY në presume, erdhi {:?}", other),
        }
        // Fryma e plotë pas saj e kthen statusin në READY (jeta flet e fundit).
        let r2 = attest("TEST", &tdir(), &[
            RiteCheck { name: "themeli2", run: rreh, critical: true },
        ]);
        assert_eq!(r2.grade(), BreathGrade::Plote);
        assert!(breath_status().is_ready());
    }

    #[test]
    fn attestation_line_is_self_sealed() {
        // Dëshmia ha ushqimin e vet: trupi para '|c:' riprodhon CRC-në.
        let r = attest("TEST", &tdir(), &[RiteCheck { name: "x", run: rreh, critical: true }]);
        let t = r.sealed_line.trim_end_matches('\n');
        let (body, crc_hex) = t.rsplit_once("|c:").expect("fusha c: e pranishme");
        let want = u64::from_str_radix(crc_hex, 16).expect("hex i vlefshëm");
        assert_eq!(pa_wire::fnv1a64(body.as_bytes()), want, "vetë-CRC e dëshmisë");
        assert!(body.contains("|FRYMA|"));
    }
}
