// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  GJATA_COLLAPSE_LAW.RS — LIGJI KUSHTETUES I RUNTIME-it                   ║
// ║  GJATA LEGACY™ — ESS-MAI (BYTE-FOR-BYTE IDENTIK në 3 platformat)         ║
// ║                                                                          ║
// ║  ESS-MAI == gjata_collapse_law. Çdo modul është vetëm një FAZË e ligjit.║
// ║                                                                          ║
// ║  LIGJI (i plotë, i Arkitektit):                                          ║
// ║    i0 → Trace → PA → Xi ‖ Yi → COORDINATE COLLAPSE                       ║
// ║       → LIM → (-Xi)+(-Yi) → PRO → i ≈ XY → APRO → MPRO → XY             ║
// ║       → PIM / NPIM → Shadow → VERIFICATION COLLAPSE → D ∈ {0,1}         ║
// ║                                                                          ║
// ║  TRE KOLAPSET:                                                           ║
// ║    Light   → Coordinate Collapse   (gjurmon, ankoron, koordinon Xi/Yi)  ║
// ║    Quantum → Reasoning Collapse    (eliminon, krahason, propozon XY)     ║
// ║    Shadow  → Verification Collapse (provon ose rrëzon — kurrë krijon)   ║
// ║                                                                          ║
// ║  INVARIANTET KUSHTETUESE:                                                ║
// ║    Light:   "Asgjë nuk hyn pa u bërë Primitive_Anchor."                 ║
// ║    Quantum: "Asgjë nuk del pa kaluar Quantum_Collapse."                  ║
// ║    Shadow:  "Asgjë nuk pranohet pa Sovereign_Verification_Collapse."    ║
// ║                                                                          ║
// ║  LAW 0 (globale, E DETYRUAR NË KOD):                                     ║
// ║    "Every state must reduce uncertainty."                                ║
// ║    Çdo fazë regjistron hapësirën e mundësive PARA→PAS me numra REALË    ║
// ║    (kandidatë të matur, kurrë hamendje). PAS > PARA = LawViolation —    ║
// ║    fail-closed. Kolapsi s'është një hap: është sjellja e runtime-it.    ║
// ║                                                                          ║
// ║  ESS-MAI s'është probabilitet LLM. Është PROBABILITET FAKTIK:            ║
// ║    matje deterministike → raport vektorial → × evidencë e verifikuar.   ║
// ║                                                                          ║
// ║  LIGJE KODI: zero if/else (vetëm match), zero varësi, zero clone.        ║
// ╚══════════════════════════════════════════════════════════════════════════╝

// ─────────────────────────────────────────────────────────────────────────────
// DALLIM I PRERË (mos u ngatërro kurrë — dy ligje të veçanta):
//   • LAW 0  (ky ligj)     = "Every state must reduce uncertainty" —
//     ulja e detyruar e hapësirës së mundësive (UncertaintyLedger, kolapsi).
//   • LIGJI 0 (transporti) = zero-copy/move në urat FFI/bridge — pronësia
//     zhvendoset një herë, kurrë s'klonohet (ligj më i vjetër i transportit).
// Bashkëjetojnë: LIGJI 0 mbron TRANSPORTIN, LAW 0 drejton KOLAPSIN.
// ─────────────────────────────────────────────────────────────────────────────

use super::collapse::CollapsePhase;

/// Invariantet kushtetuese — tekste ligjore, jo konfigurim.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstitutionalInvariant {
    LightAnchored,
    QuantumCollapsed,
    ShadowVerified,
}

impl ConstitutionalInvariant {
    pub fn text(&self) -> &'static str {
        match self {
            Self::LightAnchored =>
                "Asgjë nuk hyn pa u bërë Primitive_Anchor.",
            Self::QuantumCollapsed =>
                "Asgjë nuk del pa kaluar Quantum_Collapse.",
            Self::ShadowVerified =>
                "Asgjë nuk pranohet pa Sovereign_Verification_Collapse.",
        }
    }

    pub fn phase(&self) -> CollapsePhase {
        match self {
            Self::LightAnchored    => CollapsePhase::Coordination,
            Self::QuantumCollapsed => CollapsePhase::Reasoning,
            Self::ShadowVerified   => CollapsePhase::Verification,
        }
    }
}

/// Shkelje e Law 0 — pasiguria u RRIT. Fail-closed: kjo s'kalon kurrë heshtur.
#[derive(Debug, Clone, PartialEq)]
pub struct LawViolation {
    pub phase:  CollapsePhase,
    pub module: &'static str,
    pub before: f32,
    pub after:  f32,
}

impl LawViolation {
    pub fn report(&self) -> String {
        format!("LAW0 SHKELJE [{}::{}] pasiguria u rrit: {:.2} → {:.2}",
            self.phase.label(), self.module, self.before, self.after)
    }
}

/// Një hap i regjistruar i ligjit — hapësira e mundësive para→pas.
#[derive(Debug, Clone, PartialEq)]
pub struct LawStep {
    pub phase:  CollapsePhase,
    pub module: &'static str,
    pub before: f32,
    pub after:  f32,
}

/// LAW 0 LEDGER — regjistri monoton i kolapsit të pasigurisë.
/// Numrat janë REALË (|Xi|×|Yi|, eliminime, mbijetues, 1, 0) — kurrë hamendje.
#[derive(Debug, Default)]
pub struct UncertaintyLedger {
    steps: Vec<LawStep>,
}

impl UncertaintyLedger {
    pub fn new() -> Self {
        Self { steps: Vec::new() }
    }

    /// Regjistron një fazë. LAW 0: after ≤ before — ndryshe LawViolation.
    /// (Barazia lejohet: një fazë mund të mos ngushtojë, por KURRË s'zgjeron.)
    pub fn record(
        &mut self,
        phase:  CollapsePhase,
        module: &'static str,
        before: f32,
        after:  f32,
    ) -> Result<(), LawViolation> {
        match after <= before {
            true => {
                self.steps.push(LawStep { phase, module, before, after });
                Ok(())
            }
            false => Err(LawViolation { phase, module, before, after }),
        }
    }

    /// Hapësira aktuale (pas hapit të fundit). Bosh → f32::INFINITY (e pakolapsuar).
    pub fn current_space(&self) -> f32 {
        match self.steps.last() {
            Some(s) => s.after,
            None    => f32::INFINITY,
        }
    }

    /// A ka kolapsuar plotësisht runtime-i (hapësira → 0: D u vendos)?
    pub fn is_collapsed(&self) -> bool {
        match self.steps.last() {
            Some(s) => s.after <= 0.0,
            None    => false,
        }
    }

    pub fn steps(&self) -> &[LawStep] {
        &self.steps
    }

    /// Raporti i zinxhirit: "n²→xy→…→1→0" — dëshmia e gjallë e ligjit.
    pub fn chain_report(&self) -> String {
        let mut out = String::new();
        for (i, s) in self.steps.iter().enumerate() {
            match i {
                0 => out.push_str(&format!("{:.0}", s.before)),
                _ => {}
            }
            out.push_str(&format!("→{:.2}[{}]", s.after, s.module));
        }
        out
    }
}

/// Hapësira e mundësive e dy hapësirave primitive: |Xi| × |Yi|.
/// Njëra bosh → hapësira e çifteve = 0 (s'ka XY të mundshëm ende).
pub fn pair_space(xi_len: usize, yi_len: usize) -> f32 {
    (xi_len as f32) * (yi_len as f32)
}

/// PROBABILITETI FAKTIK (next-level, JO probabilitet LLM):
///   deterministik (raporti vektorial i 16 matjeve 0/1 të MPRO-s)
///   × evidencë e verifikuar (epistemic_mass i matur nga LIM).
/// Dy hyrje të matura → dalje e matur. Zero hamendje.
pub fn factic_probability(vector_ratio: f32, evidence_mass: f32) -> f32 {
    (vector_ratio.clamp(0.0, 1.0) * evidence_mass.clamp(0.0, 1.0)).clamp(0.0, 1.0)
}

// ════════════════════════════════════════════════════════════════════════════
// SHPALLJA GLOBALE — gjata_collapse_law si STATE MACHINE SUPREME
//
// LIGJI URDHËRON. Tre muskujt e sistemit USHQEHEN prej tij dhe fizikisht
// NUK MUND të ndërtohen pa Direktivën Supreme (kapabilitet i vulosur):
//
//   1. alnur_karina_athar        (Light   — Coordinate Collapse)
//   2. ultimatum_collapse_law    (Quantum — Reasoning  Collapse)
//   3. verefied_diary_supremelaw (Shadow  — Verification Collapse)
//
// Hierarkia: 1 ligj i qartë → 3 muskuj → 0 devijim. Një herë e mirë.
// ════════════════════════════════════════════════════════════════════════════

/// Tre muskujt e shpallur globalisht — të ushqyer nga ky ligj.
pub const LAW_MUSCLES: [&str; 3] = [
    "alnur_karina_athar",
    "ultimatum_collapse_law",
    "verefied_diary_supremelaw",
];

/// FNV-1a 64 — vula deterministe e ligjit (zero varësi).
pub fn law_fnv1a64(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in bytes.iter() {
        h ^= *b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

/// Vula e ligjit — e njëjtë byte-për-byte në të tre platformat.
pub fn law_seal() -> u64 {
    law_fnv1a64(b"gjata_collapse_law")
}

/// MANIFESTI I LIGJIT — teksti që SISTEMI E LEXON në çdo nisje.
/// Ligji nuk është vetëm kod: është deklaratë e gjallë që çdo binar e shpall
/// në boot (Light, Quantum, Shadow) dhe çdo muskul e verifikon me vulë.
pub fn law_manifest() -> &'static str {
    "GJATA_COLLAPSE_LAW — ligji ekzekutiv i identitetit të ESS-MAI:\n\
     çdo Primitive Anchor DETYROHET të kalojë nëpër koordinim, eliminim,\n\
     arsyetim dhe verifikim deri në një gjendje të vetme të qëndrueshme\n\
     (D ∈ {0,1} ose XY).\n\
     Zinxhiri: i0→Trace→PA→Xi‖Yi→CoordinateCollapse→LIM→(-Xi)+(-Yi)→PRO→\n\
     i≈XY→APRO→MPRO→XY→PIM/NPIM→Shadow→VerificationCollapse→D∈{0,1}.\n\
     LAW 0: çdo gjendje ul pasigurinë (kurrë zgjerim; shkelje = fail-closed).\n\
     Muskujt: alnur_karina_athar (Light) · ultimatum_collapse_law (Quantum) ·\n\
     verefied_diary_supremelaw (Shadow). ESS-MAI == gjata_collapse_law.\n\
     Probabilitet FAKTIK (matje×evidencë) — kurrë probabilitet LLM.\n\
     NDARJE: 'Primitive Anchor' (PA) = HYRJA (Light, vulë 500); ndryshe nga\n\
     'UNANIM_PRIMITIVE' (Shadow, ∏ᵢXᵢ=1) = dija E VERIFIKUAR. PA hyn, \n\
     UNANIM_PRIMITIVE del i verifikuar — role të ndryshme, pa përplasje."
}

// ════════════════════════════════════════════════════════════════════════════
// REGJISTRI SOVRAN I LIGJEVE — TË GJITHA ligjet e ESS-MAI, të vulosura.
// Përforcimi final: asnjë ligj i shpërndarë nëpër komente — TË GJITHË këtu,
// të lexueshëm nga sistemi, të verifikueshëm me vulë, të shpallur në boot.
// ════════════════════════════════════════════════════════════════════════════

/// Një ligj i sistemit — kod i shkurtër, titull, tekst i plotë ekzekutiv.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SystemLaw {
    pub code:  &'static str,
    pub title: &'static str,
    pub text:  &'static str,
}

/// TË DHJETË ligjet e ESS-MAI — regjistri i plotë (rendi = pesha).
pub const SYSTEM_LAWS: [SystemLaw; 10] = [
    SystemLaw { code: "LAW-0",  title: "Ulja e Pasigurisë",
        text: "Çdo gjendje DUHET të ulë hapësirën e mundësive; after>before = LawViolation, fail-closed." },
    SystemLaw { code: "LIGJI-0", title: "Transporti Zero-Copy",
        text: "Në urat FFI/bridge pronësia ZHVENDOSET një herë (move); klonimi i të dhënave të transportit i ndaluar." },
    SystemLaw { code: "L-FC",   title: "Fail-Closed Uniform",
        text: "Çdo kufi i jashtëm ose pranon SAKTË ose refuzon ME EMËR (reason code); korrupsioni kurrë s'bëhet 0/\"\" e butë." },
    SystemLaw { code: "L-500",  title: "Vula Sovrane 500",
        text: "Primitivi mban vulën 500 (0xA451 e maskuar); humbja e vulës = fragmentim; verdikti kalon VETËM nëpër rrugën e vulës." },
    SystemLaw { code: "L-PA",   title: "Primitive Anchor Gate",
        text: "Asgjë nuk hyn pa u bërë Primitive_Anchor; PA+(Xi,Yi) pret në Shadow propozimin XY; mospërputhja = SealInvalid." },
    SystemLaw { code: "L-HW10", title: "Dyshemeja 10% e Hardware-it",
        text: "≥10% e hardware-it GJITHMONË i lirë; kërkesa mbi kufi SHKURTOHET me fleksibilitet — kurrë refuzim, kurrë bllokim." },
    SystemLaw { code: "L-MT",   title: "Match-Totalitet",
        text: "Zero if/else në rrugët e verdiktit/vulës (vetëm match total, build.rs përjashtim) — çdo degë e emërtuar, e verifikueshme statikisht." },
    SystemLaw { code: "L-CX3",  title: "Kontratat Byte-Identike ×3",
        text: "lab_contracts dhe lab_contracts_v11 janë byte-për-byte identike në Light, Quantum, Shadow — një gjuhë, tre trupa." },
    SystemLaw { code: "L-NC",   title: "Zero-Clone ku Shmanget",
        text: "Pronësia lëviz, nuk dyfishohet; clone lejohet vetëm i justifikuar dhe i dokumentuar në vend." },
    SystemLaw { code: "L-FP",   title: "Probabiliteti Faktik",
        text: "Dalja = matje deterministe (vektor 16) × evidencë e matur (LIM) — KURRË probabilitet stokastik LLM." },
];

/// Vula e regjistrit — FNV mbi bashkimin e të gjithë kodeve+teksteve.
/// E njëjtë byte-për-byte në 3 platformat; ndryshimi i një ligji e ndryshon.
pub fn system_laws_seal() -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for law in SYSTEM_LAWS.iter() {
        for b in law.code.as_bytes().iter()
            .chain(law.title.as_bytes().iter())
            .chain(law.text.as_bytes().iter()) {
            h ^= *b as u64;
            h = h.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    h
}

/// ROLI EKZEKUTIV i një platforme — deklaratë kushtetuese: ROL, JO AGJENT.
/// Light/Quantum/Shadow NUK janë agjentë (s'mendojnë, s'vendosin si subjekte
/// autonome, s'improvizojnë, s'debatojnë si vetë). Janë ARKITEKTURA +
/// MAKINA GJENDJESH + ROLE EKZEKUTIVE, të lidhura nga një ligj i vetëm.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlatformRole {
    pub platform: &'static str,
    pub role:     &'static str,
    pub verb:     &'static str,
}

/// Deklarata globale — tre role ekzekutive, ZERO agjentë.
pub const PLATFORM_ROLES: [PlatformRole; 3] = [
    PlatformRole { platform: "LIGHT",   role: "Koordinuese (state machine)",
        verb: "koordinon: gjurmon, ankoron, ndan Xi‖Yi — nuk arsyeton" },
    PlatformRole { platform: "QUANTUM", role: "Kolapsuese (state machine)",
        verb: "propozon: eliminon, krahason, kolapson në XY — nuk vendos" },
    PlatformRole { platform: "SHADOW",  role: "Verifikuese/Autoritet (state machine)",
        verb: "autorizon: verifikon X e Y veç, vulos D∈{0,1} — nuk krijon ide" },
];

/// Deklarata e shprehur: ESS-MAI s'ka agjentë — vetëm role ekzekutive.
pub fn roles_declaration() -> String {
    let mut out = String::from(
        "ESS-MAI: TRE ROLE EKZEKUTIVE (arkitektura + makina gjendjesh), ZERO AGJENTË.\n");
    for r in PLATFORM_ROLES.iter() {
        out.push_str(&format!("  {} — {} · {}\n", r.platform, r.role, r.verb));
    }
    out
}

/// Raporti i regjistrit — sistemi i LEXON të dhjetë ligjet.
pub fn law_registry_report() -> String {
    let mut out = String::new();
    for law in SYSTEM_LAWS.iter() {
        out.push_str(&format!("  [{}] {} — {}\n", law.code, law.title, law.text));
    }
    out
}

/// Shpallja e ligjit në boot — sistemi e LEXON dhe e dëshmon me vulë.
pub fn proclaim_law(platform: &str) {
    println!("╔══ [LIGJI I LEXUAR] platforma={} vula={:016x} ══╗", platform, law_seal());
    for line in law_manifest().lines() {
        println!("║ {}", line);
    }
    println!("╠══ REGJISTRI: {} LIGJE AKTIVE · vula {:016x} ══╣",
        SYSTEM_LAWS.len(), system_laws_seal());
    println!("╠══ {} ROLE EKZEKUTIVE (JO AGJENTË) ══╣", PLATFORM_ROLES.len());
    println!("╚══ ESS-MAI == gjata_collapse_law — ligji është gjallë ══╝");
}

/// DIREKTIVA SUPREME — kapabiliteti që ligji i jep NJË muskuli për NJË fazë.
/// Pa të, muskuli nuk ekziston. Me fazë të gabuar, muskuli refuzohet.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SupremeDirective {
    pub phase: CollapsePhase,
    pub seal:  u64,
}

/// STATE MACHINE SUPREME — gjata_collapse_law vetë.
pub struct GjataCollapseLaw;

impl GjataCollapseLaw {
    /// Lëshon direktivën për fazën — akti urdhërues i ligjit.
    pub fn issue(phase: CollapsePhase) -> SupremeDirective {
        SupremeDirective { phase, seal: law_seal() }
    }

    /// Verifikon direktivën: vula e ligjit + faza e kërkuar.
    pub fn verify(d: &SupremeDirective, required: CollapsePhase) -> bool {
        d.seal == law_seal() && d.phase == required
    }
}

// ════════════════════════════════════════════════════════════════════════════
// TESTE — ligji provohet identikisht nga të tre platformat
// ════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn law0_monotone_chain_collapses_to_zero() {
        let mut l = UncertaintyLedger::new();
        // Light: 4 terma → n²=16 hapësirë; koordinimi → |Xi|×|Yi|=3×1=3.
        l.record(CollapsePhase::Coordination, "primitive_anchor", 16.0, 3.0).unwrap();
        // NPRO eliminon 1 term Xi → 2×1=2.
        l.record(CollapsePhase::Reasoning, "npro", 3.0, 2.0).unwrap();
        // PRO i≈XY: 1 çift mbijeton.
        l.record(CollapsePhase::Reasoning, "pro", 2.0, 1.0).unwrap();
        // MPRO: XY i vetëm i matur.
        l.record(CollapsePhase::Reasoning, "mpro", 1.0, 1.0).unwrap();
        // Shadow: D u vendos → 0.
        l.record(CollapsePhase::Verification, "kolaps3", 1.0, 0.0).unwrap();
        assert!(l.is_collapsed());
        assert_eq!(l.steps().len(), 5);
        assert!(l.chain_report().contains("[kolaps3]"));
    }

    #[test]
    fn law0_violation_is_fail_closed() {
        let mut l = UncertaintyLedger::new();
        let v = l.record(CollapsePhase::Reasoning, "x", 2.0, 5.0);
        match v {
            Err(viol) => {
                assert!(viol.report().contains("SHKELJE"));
                assert_eq!(l.steps().len(), 0); // asgjë s'u regjistrua
            }
            Ok(_) => panic!("rritja e pasigurisë DUHET të refuzohet"),
        }
    }

    #[test]
    fn pair_space_real_counting() {
        assert_eq!(pair_space(3, 1), 3.0);
        assert_eq!(pair_space(0, 5), 0.0);
        assert_eq!(pair_space(4, 4), 16.0);
    }

    #[test]
    fn factic_probability_is_measured_not_guessed() {
        // 9/16 vektorial × 0.80 evidencë = 0.45 faktik.
        let f = factic_probability(0.5625, 0.80);
        assert!((f - 0.45).abs() < 1e-6);
        // Evidencë zero → faktik zero (pa evidencë s'ka probabilitet faktik).
        assert_eq!(factic_probability(1.0, 0.0), 0.0);
        assert_eq!(factic_probability(2.0, 0.5), 0.5); // clamp
    }

    #[test]
    fn constitutional_invariants_bound_to_phases() {
        assert_eq!(ConstitutionalInvariant::LightAnchored.phase(),
                   CollapsePhase::Coordination);
        assert_eq!(ConstitutionalInvariant::QuantumCollapsed.phase(),
                   CollapsePhase::Reasoning);
        assert_eq!(ConstitutionalInvariant::ShadowVerified.phase(),
                   CollapsePhase::Verification);
        assert!(ConstitutionalInvariant::LightAnchored.text().contains("Primitive_Anchor"));
    }

    #[test]
    fn supreme_directive_issue_verify_roundtrip() {
        let d = GjataCollapseLaw::issue(CollapsePhase::Reasoning);
        assert!(GjataCollapseLaw::verify(&d, CollapsePhase::Reasoning));
        assert!(!GjataCollapseLaw::verify(&d, CollapsePhase::Coordination));
        let fake = SupremeDirective { phase: CollapsePhase::Reasoning, seal: 0 };
        assert!(!GjataCollapseLaw::verify(&fake, CollapsePhase::Reasoning));
    }

    #[test]
    fn platforms_are_roles_not_agents() {
        assert_eq!(PLATFORM_ROLES.len(), 3);
        let plats: Vec<&str> = PLATFORM_ROLES.iter().map(|r| r.platform).collect();
        assert!(plats.contains(&"LIGHT") && plats.contains(&"QUANTUM") && plats.contains(&"SHADOW"));
        let d = roles_declaration();
        assert!(d.contains("ZERO AGJENTË"));
        assert!(d.contains("verifikon X e Y veç")); // dy-kanalësh, jo cross-elimination
    }

    #[test]
    fn system_laws_registry_complete_and_sealed() {
        assert_eq!(SYSTEM_LAWS.len(), 10);
        let codes: Vec<&str> = SYSTEM_LAWS.iter().map(|l| l.code).collect();
        for c in ["LAW-0","LIGJI-0","L-FC","L-500","L-PA","L-HW10","L-MT","L-CX3","L-NC","L-FP"] {
            assert!(codes.contains(&c), "ligji {} mungon nga regjistri", c);
        }
        assert!(system_laws_seal() != 0);
        let rep = law_registry_report();
        assert!(rep.contains("Probabiliteti Faktik") && rep.contains("10%"));
    }

    #[test]
    fn law_manifest_readable_and_sealed() {
        let m = law_manifest();
        assert!(m.contains("GJATA_COLLAPSE_LAW"));
        assert!(m.contains("D ∈ {0,1}"));
        assert!(m.contains("alnur_karina_athar"));
        assert!(law_seal() != 0);
    }

    #[test]
    fn three_muscles_declared_globally() {
        assert_eq!(LAW_MUSCLES.len(), 3);
        assert!(LAW_MUSCLES.contains(&"alnur_karina_athar"));
        assert!(LAW_MUSCLES.contains(&"ultimatum_collapse_law"));
        assert!(LAW_MUSCLES.contains(&"verefied_diary_supremelaw"));
    }

    #[test]
    fn empty_ledger_uncollapsed_infinite_space() {
        let l = UncertaintyLedger::new();
        assert!(!l.is_collapsed());
        assert!(l.current_space().is_infinite());
    }
}
