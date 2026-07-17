// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  GCL_PRESUME — GATISHMËRIA NËN LIGJ (kontratë ×3, byte-identike)         ║
// ║  GJATA LEGACY™ | Arkitekt: Bledar Gjata                                  ║
// ║                                                                          ║
// ║  VENDI NË HIERARKI: gjata_collapse_law është KOMANDANTI — state machine  ║
// ║  horizontale që i komandon tre makinat e platformave (Light: Coordination║
// ║  → i₀→PA→split; Quantum: Reasoning → PRO/NPRO/MPRO/APRO→PIM/NPIM;        ║
// ║  Shadow: Verification → ankora+XY→Y→X). GCL_PRESUME VARET prej tij:      ║
// ║  merr CollapsePhase si urdhër dhe mban NË GATISHMËRI edhe modulet që     ║
// ║  s'kanë impakt në rrjedhën e kolapsit — ping→përgjigje, zero arsyetim.   ║
// ║                                                                          ║
// ║  NDËRFAQJA (siç e ka caktuar Arkitekti): çdo modul — edhe anësor —       ║
// ║  përgjigjet me gcl_role() + gcl_status(). Regjistrimi bëhet me fn        ║
// ║  pointer të pastër (zero gjendje, zero alokim të fshehtë).               ║
// ║                                                                          ║
// ║  LIGJET: zero varësi të jashtme · zero if/else (vetëm match) ·           ║
// ║  KURRË bllokues (sweep-i informon; NotReady i domosdoshëm → shënim       ║
// ║  në rrjedha me PSE, runtime vazhdon) · regjistri i kufizuar (idempotent).║
// ╚══════════════════════════════════════════════════════════════════════════╝

use std::sync::{Mutex, OnceLock};
use super::collapse::CollapsePhase;

/// Roli i një moduli ndaj ligjit GCL — fjalori i formulës, i gdhendur.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GclRole {
    // ── Light (Coordination) ────────────────────────────────────────────────
    /// i₀ — kapja e indicies primitive (lingua/gjurma).
    I0Capture,
    /// i₀ → Xi + Yi — ndarja kolaps në dy hapësira.
    SplitXiYi,
    /// Eksporti i ankorës së vulosur drejt urës (pa_wire, shkruesi).
    AnchorExport,
    // ── Quantum (Reasoning) ─────────────────────────────────────────────────
    /// PRO — gjetja e kandidatëve optimalë XiYi.
    ProSelect,
    /// NPRO — +(−Xi)+(−Yi): eliminimi brenda hapësirës përkatëse.
    NproEliminate,
    /// MPRO — matja: VLERËSIM N PROPOZIM → XY me score përputhshmërie.
    MproMeasure,
    /// APRO — argumentimi shkencor i kandidatëve (pro DHE npro).
    AproArgue,
    /// PIM — paketimi i degës pozitive drejt Shadow.
    PimPackage,
    /// NPIM — paketimi i degës negative drejt Shadow.
    NpimPackage,
    /// PD — kontinuumi kushtetues: i₀ + 1Q → output + iZ → next i₀.
    PdContinuum,
    // ── Shadow (Verification) ───────────────────────────────────────────────
    /// Ankora i₀ që PRET në Shadow — e bën të lejueshëm verdiktin e saktë.
    AnchorGate,
    /// Y ~ 0/1 — Siguria: output i verifikuar (i pari, gjithmonë).
    VerdictY,
    /// X ~ 0/1 — Besimi: inputi trashëgon verdiktin e Y-së.
    TrustX,
    /// (−Xi)(−Yi) si dije negative — ASET i përjetshëm në vault.
    NegativeVault,
    // ── Anësorët ────────────────────────────────────────────────────────────
    /// Modul pa impakt në rrjedhën e kolapsit — por I GATSHËM të përgjigjet.
    Support(&'static str),
}

impl GclRole {
    pub fn code(&self) -> &'static str {
        match self {
            GclRole::I0Capture     => "I0_CAPTURE",
            GclRole::SplitXiYi     => "SPLIT_XI_YI",
            GclRole::AnchorExport  => "ANCHOR_EXPORT",
            GclRole::ProSelect     => "PRO_SELECT",
            GclRole::NproEliminate => "NPRO_ELIMINATE",
            GclRole::MproMeasure   => "MPRO_MEASURE",
            GclRole::AproArgue     => "APRO_ARGUE",
            GclRole::PimPackage    => "PIM_PACKAGE",
            GclRole::NpimPackage   => "NPIM_PACKAGE",
            GclRole::PdContinuum   => "PD_I0_TO_IZ",
            GclRole::AnchorGate    => "ANCHOR_GATE",
            GclRole::VerdictY      => "VERDICT_Y",
            GclRole::TrustX        => "TRUST_X",
            GclRole::NegativeVault => "NEGATIVE_VAULT",
            GclRole::Support(_)    => "SUPPORT",
        }
    }

    /// Matrica e domosdoshmërisë: cilat role DUHET të jenë Ready në cilën
    /// fazë të komandantit. Support s'është i domosdoshëm KURRË — vetëm
    /// i gatshëm të përgjigjet (kjo është pikërisht detyra e presume-it).
    pub fn required_in(&self, phase: CollapsePhase) -> bool {
        match (phase, self) {
            (CollapsePhase::Coordination,
                GclRole::I0Capture | GclRole::SplitXiYi | GclRole::AnchorExport) => true,
            (CollapsePhase::Reasoning,
                GclRole::ProSelect | GclRole::NproEliminate | GclRole::MproMeasure
                | GclRole::AproArgue | GclRole::PimPackage | GclRole::NpimPackage
                | GclRole::PdContinuum) => true,
            (CollapsePhase::Verification,
                GclRole::AnchorGate | GclRole::VerdictY
                | GclRole::TrustX  | GclRole::NegativeVault) => true,
            _ => false,
        }
    }
}

/// Përgjigjja e ping-ut — gjendja e modulit ndaj ligjit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GclReadiness {
    Ready,
    Degraded(&'static str),
    NotReady(&'static str),
}

impl GclReadiness {
    pub fn code(&self) -> &'static str {
        match self {
            GclReadiness::Ready       => "READY",
            GclReadiness::Degraded(_) => "DEGRADED",
            GclReadiness::NotReady(_) => "NOT_READY",
        }
    }
    pub fn is_ready(&self) -> bool {
        match self { GclReadiness::Ready => true, _ => false }
    }
}

/// Ping-u është fn pointer i pastër — zero gjendje, zero arsyetim në modul.
pub type StatusFn = fn() -> GclReadiness;

/// Ndihmës statusi i RI (v1.4.2): gatishmëri e kushtëzuar nga prania e një
/// env-i (p.sh. `hw_kernel` sinjalizohet me ESSMAI_HW=1). Modul anësor mund
/// ta përdorë për të raportuar Degraded pa u bërë kurrë bllokues.
pub fn readiness_from_env(var: &str, on_missing: &'static str) -> GclReadiness {
    match std::env::var(var) {
        Ok(_)  => GclReadiness::Ready,
        Err(_) => GclReadiness::Degraded(on_missing),
    }
}

struct Entry {
    name:   &'static str,
    role:   GclRole,
    status: StatusFn,
}

fn registry() -> &'static Mutex<Vec<Entry>> {
    static R: OnceLock<Mutex<Vec<Entry>>> = OnceLock::new();
    R.get_or_init(|| Mutex::new(Vec::new()))
}

/// Regjistrim IDEMPOTENT sipas emrit (regjistri i kufizuar — s'rritet me
/// thirrje të përsëritura; kyçi i helmuar rikuperohet me ligjin e njëjtë).
pub fn register(name: &'static str, role: GclRole, status: StatusFn) {
    let mut guard = match registry().lock() {
        Ok(g)         => g,
        Err(poisoned) => poisoned.into_inner(),
    };
    match guard.iter().any(|e| e.name == name) {
        true  => {} // tashmë i regjistruar — heshtje e qëllimshme (idempotent)
        false => guard.push(Entry { name, role, status }),
    }
}

/// Rreshti i një moduli në sweep — për shtypje, teste dhe raporte.
#[derive(Debug, Clone, Copy)]
pub struct SweepLine {
    pub name:      &'static str,
    pub role:      GclRole,
    pub readiness: GclReadiness,
    pub required:  bool,
}

/// Statistika e sweep-it — kthehet te thirrësi, KURRË s'urdhëron.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SweepStats {
    pub total:              usize,
    pub ready:              usize,
    pub degraded:           usize,
    pub required_not_ready: usize,
}

/// SWEEP-i i gatishmërisë nën urdhrin e komandantit (CollapsePhase):
/// ping-on ÇDO modul të regjistruar, shtyp gjendjen, dhe për rolet e
/// DOMOSDOSHME të fazës që s'janë Ready → shënim në rrjedha me PSE
/// (klasa Other — gatishmëria s'është korrupsion transporti). Best-effort,
/// zero bllokim: verdikti i ekzekutimit mbetet i platformës.
pub fn sweep(
    platform:    &'static str,
    phase:       CollapsePhase,
    rrjedha_dir: &str,
) -> (SweepStats, Vec<SweepLine>) {
    let guard = match registry().lock() {
        Ok(g)         => g,
        Err(poisoned) => poisoned.into_inner(),
    };
    let mut lines: Vec<SweepLine> = Vec::with_capacity(guard.len());
    let mut stats = SweepStats { total: 0, ready: 0, degraded: 0, required_not_ready: 0 };
    println!("  [GCL_PRESUME] {} — {} : sweep i gatishmërisë ({} module)",
        platform, phase.label(), guard.len());
    for e in guard.iter() {
        let readiness = (e.status)();
        let required  = e.role.required_in(phase);
        stats.total += 1;
        match readiness {
            GclReadiness::Ready       => { stats.ready += 1; }
            GclReadiness::Degraded(_) => { stats.degraded += 1; }
            GclReadiness::NotReady(_) => {}
        }
        let flamuri = match required { true => "†", false => " " };
        println!("  [GCL_PRESUME]   {}{} ({}) → {}",
            flamuri, e.name, e.role.code(), readiness.code());
        match (required, readiness.is_ready()) {
            (true, false) => {
                stats.required_not_ready += 1;
                let arsyeja = match readiness {
                    GclReadiness::Degraded(a) | GclReadiness::NotReady(a) => a,
                    GclReadiness::Ready => "", // e paarritshme në këtë degë
                };
                let _ = super::rrjedha::note(
                    rrjedha_dir, platform, "gcl_presume::sweep",
                    super::rrjedha::RrjedhaClass::Other,
                    &format!("rol i domosdoshëm JO gati: {} ({}) në {} — {}",
                        e.name, e.role.code(), phase.label(), arsyeja));
                eprintln!("  [GCL_PRESUME] ALARM: {} ({}) i domosdoshëm për {} → {}",
                    e.name, e.role.code(), phase.label(), readiness.code());
            }
            _ => {}
        }
        lines.push(SweepLine { name: e.name, role: e.role, readiness, required });
    }
    println!("  [GCL_PRESUME] {} — përfundim: {}/{} READY, {} DEGRADED, {} të domosdoshëm JO gati",
        platform, stats.ready, stats.total, stats.degraded, stats.required_not_ready);
    (stats, lines)
}

// ════════════════════════════════════════════════════════════════════════════
// TESTE — kontrata provohet në SECILËN platformë (×3, byte-identike).
// KUJDES: regjistri është global në binarin e testeve — emrat unikë,
// pohimet kërkojnë rreshtin TONË, kurrë totale absolute të brishta.
// ════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod gcl_presume_tests {
    use super::*;

    fn gati() -> GclReadiness { GclReadiness::Ready }
    fn jo_gati() -> GclReadiness { GclReadiness::NotReady("provë e qëllimshme") }

    fn tdir() -> String { std::env::temp_dir().to_string_lossy().into_owned() }

    #[test]
    fn role_codes_are_stable_and_greppable() {
        assert_eq!(GclRole::MproMeasure.code(), "MPRO_MEASURE");
        assert_eq!(GclRole::NpimPackage.code(), "NPIM_PACKAGE");
        assert_eq!(GclRole::Support("x").code(), "SUPPORT");
    }

    #[test]
    fn required_matrix_follows_the_law() {
        use CollapsePhase::*;
        // Çdo fazë kërkon SAKTËSISHT rolet e veta — kurrë të fqinjit.
        assert!(GclRole::SplitXiYi.required_in(Coordination));
        assert!(!GclRole::SplitXiYi.required_in(Reasoning));
        assert!(GclRole::PimPackage.required_in(Reasoning));
        assert!(!GclRole::PimPackage.required_in(Verification));
        assert!(GclRole::TrustX.required_in(Verification));
        assert!(!GclRole::TrustX.required_in(Coordination));
    }

    #[test]
    fn pd_continuum_is_required_in_reasoning() {
        use CollapsePhase::*;
        assert!(GclRole::PdContinuum.required_in(Reasoning));
        assert!(!GclRole::PdContinuum.required_in(Coordination));
        assert!(!GclRole::PdContinuum.required_in(Verification));
        assert_eq!(GclRole::PdContinuum.code(), "PD_I0_TO_IZ");
    }

    #[test]
    fn support_is_never_required_but_always_pingable() {
        use CollapsePhase::*;
        let s = GclRole::Support("anësor");
        assert!(!s.required_in(Coordination));
        assert!(!s.required_in(Reasoning));
        assert!(!s.required_in(Verification));
        register("test_presume_support_unik", s, gati);
        let (_, lines) = sweep("TEST", Reasoning, &tdir());
        let yni = lines.iter().find(|l| l.name == "test_presume_support_unik")
            .expect("moduli anësor duhet të përgjigjet në sweep");
        assert!(yni.readiness.is_ready());
        assert!(!yni.required);
    }

    #[test]
    fn register_is_idempotent_by_name() {
        const NAME: &str = "test_presume_idem_unik";
        register(NAME, GclRole::Support("a"), gati);
        let (_, lines1) = sweep("TEST", CollapsePhase::Coordination, &tdir());
        register(NAME, GclRole::Support("a"), gati);
        let (_, lines2) = sweep("TEST", CollapsePhase::Coordination, &tdir());

        // Regjistri është global dhe testet mund të ecin paralelisht; prandaj
        // kontrata matet mbi emrin tonë, jo mbi totalin global të brishtë.
        let count1 = lines1.iter().filter(|line| line.name == NAME).count();
        let count2 = lines2.iter().filter(|line| line.name == NAME).count();
        assert_eq!(count1, 1, "regjistrimi i parë duhet të krijojë vetëm një hyrje");
        assert_eq!(count2, 1, "regjistrimi i dyfishtë s'duhet të krijojë hyrje të dytë");
    }

    #[test]
    fn required_not_ready_is_flagged_and_noted() {
        register("test_presume_verdikt_unik", GclRole::VerdictY, jo_gati);
        let (stats, lines) = sweep("TEST", CollapsePhase::Verification, &tdir());
        let yni = lines.iter().find(|l| l.name == "test_presume_verdikt_unik")
            .expect("rreshti ynë duhet të ekzistojë");
        assert!(yni.required, "VerdictY është i domosdoshëm në Verification");
        assert!(!yni.readiness.is_ready());
        assert!(stats.required_not_ready >= 1, "sweep-i duhet ta numërojë");
        // Dhe në fazë tjetër i njëjti modul s'është i domosdoshëm (s'bllokon).
        let (_, lines2) = sweep("TEST", CollapsePhase::Coordination, &tdir());
        let yni2 = lines2.iter().find(|l| l.name == "test_presume_verdikt_unik")
            .expect("prapë i pranishëm");
        assert!(!yni2.required);
    }
}
