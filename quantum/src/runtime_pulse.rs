// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  RUNTIME_PULSE — PULSI REAL I ORGANEVE (Quantum, v1.4.3)                 ║
// ║  GJATA LEGACY™ | Arkitekt: Bledar Gjata                                  ║
// ║                                                                          ║
// ║  GJETJA (audit p.3/p.4, PRANUAR): gcl_presume regjistronte PRO/NPRO/     ║
// ║  MPRO/APRO/PIM/NPIM me `gati_gjithnje` — "apel simbolik". READY vinte    ║
// ║  nga konstantja, jo nga jeta. Tani: çdo organ RREH kur ekzekutohet       ║
// ║  realisht në pipeline (mark_ready në vendin e thirrjes), dështon me zë   ║
// ║  (mark_notready), ose deklarohet bosh (mark_degraded). presume lexon     ║
// ║  PULSIN — jo premtimin.                                                  ║
// ║                                                                          ║
// ║  LIGJET: zero if/else · zero unwrap · KURRË bllokues · pulsi informon,   ║
// ║  s'urdhëron (Shadow mbetet autoriteti; Quantum s'bëhet mur).             ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use std::sync::atomic::{AtomicU64, AtomicU8, Ordering};
use std::sync::{Mutex, OnceLock};
use crate::lab_contracts::gcl_presume::GclReadiness;

/// Organet që rrahin — një slot pulsi për secilin.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    Split,      // read_primitive_context — burimi i i₀ + Xi/Yi
    Hpro,       // HPRO/HPIM — prova fizike e Layer 3
    Pro,        // PRO — kandidatët optimalë
    Npro,       // NPRO — (−Xi)(−Yi)
    Srk,        // SRK — conservation + IBE + evidence_chain
    Apro,       // APRO — argumentimi
    Mpro,       // MPRO — 16 matjet dhe masa faktike
    Pim,        // PIM — paketimi pozitiv pas SRK
    Npim,       // NPIM — paketimi negativ
    Hcp,        // HCP_PRO — vendimi real i harduerit
    Thermal,    // sensori termik — leximi i fundit
    TokenForge, // dëshmitari i dytë SHA-256
    NkImport,   // importi i dijes negative nga Shadow
    PdContinuum,// i₀ + 1Q → output + iZ → next i₀
}

const N: usize = 14;

/// Organet që duhet të kenë mbaruar realisht para paketës finale.
/// Bitet 1..=9 janë HPRO, PRO, NPRO, SRK, APRO, MPRO, PIM, NPIM dhe HCP.
pub const REQUIRED_ACTION_MASK: u64 = 0x0000_0000_0000_03FE;
/// Rendi real i mbarimit në pipeline-in Quantum v1.6.2.
const REQUIRED_ACTION_ORDER: [u8; 9] = [1, 2, 3, 8, 4, 7, 5, 6, 9];

fn expected_action_word_count(stage: u8) -> Option<usize> {
    match stage {
        1 => Some(5),  // HPRO
        2 => Some(3),  // PRO
        3 => Some(4),  // NPRO
        4 => Some(6),  // SRK
        5 => Some(4),  // APRO
        6 => Some(21), // MPRO
        7 => Some(6),  // PIM
        8 => Some(5),  // NPIM
        9 => Some(5),  // HCP
        _ => None,
    }
}

/// Kodet e pulsit: 0=I-PA-RRAHUR, 1=READY, 2=NOT_READY, 3=BOSH-I-DEKLARUAR.
static ACTION_STATE: AtomicU64 = AtomicU64::new(0);
static ACTION_MASK: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionEvidence {
    pub stage: u8,
    /// Fjalët kanonike që organi prodhoi në përfundim. Faza 1 nuk i hash-on.
    pub evidence_words: Vec<u64>,
}

static ACTION_EVIDENCE: OnceLock<Mutex<Vec<ActionEvidence>>> = OnceLock::new();

fn action_ledger() -> &'static Mutex<Vec<ActionEvidence>> {
    ACTION_EVIDENCE.get_or_init(|| Mutex::new(Vec::with_capacity(N)))
}

static PULSE: [AtomicU8; N] = [
    AtomicU8::new(0), AtomicU8::new(0), AtomicU8::new(0),
    AtomicU8::new(0), AtomicU8::new(0), AtomicU8::new(0),
    AtomicU8::new(0), AtomicU8::new(0), AtomicU8::new(0),
    AtomicU8::new(0), AtomicU8::new(0), AtomicU8::new(0),
    AtomicU8::new(0), AtomicU8::new(0),
];

fn idx(s: Stage) -> usize {
    match s {
        Stage::Split       => 0,
        Stage::Hpro        => 1,
        Stage::Pro         => 2,
        Stage::Npro        => 3,
        Stage::Srk         => 4,
        Stage::Apro        => 5,
        Stage::Mpro        => 6,
        Stage::Pim         => 7,
        Stage::Npim        => 8,
        Stage::Hcp         => 9,
        Stage::Thermal     => 10,
        Stage::TokenForge  => 11,
        Stage::NkImport    => 12,
        Stage::PdContinuum => 13,
    }
}

fn stage_bit(stage: Stage) -> u64 { 1u64 << idx(stage) }



/// Nis një cikël të ri: mosbesimi është gjendja e paracaktuar.
/// Asnjë vulë nuk trashëgohet; statuset dhe afrimi i aksioneve kthehen në zero.
pub fn begin_cycle() {
    for pulse in &PULSE {
        pulse.store(0, Ordering::SeqCst);
    }
    ACTION_STATE.store(0, Ordering::SeqCst);
    ACTION_MASK.store(0, Ordering::SeqCst);
    match action_ledger().lock() {
        Ok(mut ledger) => ledger.clear(),
        Err(poisoned) => poisoned.into_inner().clear(),
    }
}

fn stage_word(stage: Stage) -> u64 {
    (idx(stage) as u64 + 1).wrapping_mul(0x9E37_79B9_7F4A_7C15)
}

/// Konvergjencë e lehtë: zero SHA-256. Aksionet përzihen vetëm me rotacion,
/// XOR dhe mbledhje wrapping; SHA-256 lind një herë te verdikti suprem.
pub fn converge_words(words: &[u64]) -> u64 {
    words.iter().enumerate().fold(0x6A09_E667_F3BC_C909u64, |state, (index, word)| {
        state.rotate_left(((index as u32) % 47) + 5)
            ^ word.rotate_left(((index as u32) % 31) + 1)
            .wrapping_add(0x3C6E_F372_FE94_F82B ^ index as u64)
    })
}

/// Regjistron mbarimin real të një organi dhe derdh aksionin e tij në gjendjen
/// e përbashkët. `action_state` është vetëm afrimi drejt vulës, jo Besimi.
pub fn mark_action(stage: Stage, evidence_words: &[u64]) {
    mark_ready(stage);
    let contribution = converge_words(evidence_words) ^ stage_word(stage);
    let _ = ACTION_STATE.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |current| {
        Some(current.rotate_left(11).wrapping_add(contribution).rotate_left(7))
    });
    ACTION_MASK.fetch_or(stage_bit(stage), Ordering::SeqCst);
    let evidence = ActionEvidence {
        stage: idx(stage) as u8,
        evidence_words: evidence_words.to_vec(),
    };
    match action_ledger().lock() {
        Ok(mut ledger) => ledger.push(evidence),
        Err(poisoned) => poisoned.into_inner().push(evidence),
    }
}

pub fn action_state() -> u64 {
    ACTION_STATE.load(Ordering::SeqCst)
}

pub fn action_mask() -> u64 {
    ACTION_MASK.load(Ordering::SeqCst)
}

pub fn action_evidence() -> Vec<ActionEvidence> {
    match action_ledger().lock() {
        Ok(ledger) => ledger.clone(),
        Err(poisoned) => poisoned.into_inner().clone(),
    }
}

fn stage_word_from_index(stage: u8) -> u64 {
    (u64::from(stage) + 1).wrapping_mul(0x9E37_79B9_7F4A_7C15)
}

pub fn replay_action_evidence(evidence: &[ActionEvidence]) -> (u64, u64) {
    evidence.iter().fold((0u64, 0u64), |(state, mask), entry| {
        let contribution = converge_words(&entry.evidence_words)
            ^ stage_word_from_index(entry.stage);
        let next = state.rotate_left(11)
            .wrapping_add(contribution)
            .rotate_left(7);
        (next, mask | (1u64 << u32::from(entry.stage)))
    })
}

pub fn action_convergence_complete() -> bool {
    let evidence = action_evidence();
    let schema_ok = evidence.len() == REQUIRED_ACTION_ORDER.len()
        && evidence.iter().zip(REQUIRED_ACTION_ORDER).all(|(entry, expected_stage)| {
            entry.stage == expected_stage
                && expected_action_word_count(entry.stage)
                    == Some(entry.evidence_words.len())
        });
    let replayed = replay_action_evidence(&evidence);
    action_state() != 0
        && action_mask() == REQUIRED_ACTION_MASK
        && replayed == (action_state(), action_mask())
        && schema_ok
}

/// Organi rrahu me sukses në runtime.
pub fn mark_ready(s: Stage)    { PULSE[idx(s)].store(1, Ordering::SeqCst); }
/// Organi dështoi realisht (detajet → rrjedha_ledger në vendin e ngjarjes).
pub fn mark_notready(s: Stage) { PULSE[idx(s)].store(2, Ordering::SeqCst); }
/// Organi u ekzekutua por prodhoi bosh të deklaruar (rrugë e vjetër).
pub fn mark_degraded(s: Stage) { PULSE[idx(s)].store(3, Ordering::SeqCst); }

fn read(s: Stage) -> GclReadiness {
    match PULSE[idx(s)].load(Ordering::SeqCst) {
        1 => GclReadiness::Ready,
        2 => GclReadiness::NotReady("dështoi në runtime — shih rrjedha_ledger"),
        3 => GclReadiness::Degraded("bosh i deklaruar në runtime (rrugë e vjetër)"),
        _ => GclReadiness::Degraded("s'ka rrahur ende në këtë proces"),
    }
}

// ── Statuset për gcl_presume (fn pointer të pastër, një për organ) ──────────
pub fn split_status() -> GclReadiness { read(Stage::Split) }
pub fn hpro_status()  -> GclReadiness { read(Stage::Hpro) }
pub fn pro_status()   -> GclReadiness { read(Stage::Pro) }
pub fn npro_status()  -> GclReadiness { read(Stage::Npro) }
pub fn srk_status()   -> GclReadiness { read(Stage::Srk) }
pub fn apro_status()  -> GclReadiness { read(Stage::Apro) }
pub fn mpro_status()  -> GclReadiness { read(Stage::Mpro) }
pub fn pim_status()   -> GclReadiness { read(Stage::Pim) }
pub fn npim_status()  -> GclReadiness { read(Stage::Npim) }
pub fn hcp_status()   -> GclReadiness { read(Stage::Hcp) }
pub fn pd_continuum_status() -> GclReadiness { read(Stage::PdContinuum) }
/// v1.4.5 (audit p.6): korrupsioni NK s'është më "pa NK" — është NOT_READY.
pub fn nk_status()    -> GclReadiness { read(Stage::NkImport) }

/// hw_kernel: E VËRTETA e kompilimit, JO env-i (audit p.5, PRANUAR —
/// ESSMAI_HW as e falsifikon dot, as e fsheh dot kernelin real).
pub fn hw_kernel_status() -> GclReadiness {
    match cfg!(feature = "hw_kernel") {
        true  => GclReadiness::Ready,
        false => GclReadiness::Degraded("hw_kernel joaktiv në kompilim — sensorë nominalë (env-i NUK e falsifikon)"),
    }
}

/// Termiku: kompilimi × pulsi i leximit të fundit — fakt, jo deklarim.
pub fn thermal_status() -> GclReadiness {
    match cfg!(feature = "hw_kernel") {
        false => GclReadiness::Degraded("hw_kernel joaktiv — leximi termik nominal"),
        true  => read(Stage::Thermal),
    }
}

// ════════════════════════════════════════════════════════════════════════════
// TESTE — sjellje, jo grep (audit p.10, PRANUAR)
// ════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod runtime_pulse_tests {
    use super::*;
    use std::sync::{Mutex as TestMutex, MutexGuard};

    // Gjendja e pulsit është globale për procesin; testet Rust ekzekutohen
    // paralelisht si parazgjedhje. Ky lock izolon vetëm provat dhe parandalon
    // që një test të ndryshojë ledger-in e një testi tjetër.
    static TEST_LOCK: TestMutex<()> = TestMutex::new(());

    fn test_guard() -> MutexGuard<'static, ()> {
        match TEST_LOCK.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }

    #[test]
    fn action_state_lind_vetem_pas_aksionit_real() {
        let _guard = test_guard();
        begin_cycle();
        assert_eq!(action_state(), 0);
        assert_eq!(action_mask(), 0);
        assert!(!action_convergence_complete());
        mark_action(Stage::Pro, &[4, 11, 22]);
        let after_pro = action_state();
        assert_ne!(after_pro, 0);
        mark_action(Stage::Npro, &[4, 33, 44]);
        assert_ne!(action_state(), after_pro);
        assert!(!action_convergence_complete());
    }

    #[test]
    fn trust_requires_every_real_reasoning_organ() {
        let _guard = test_guard();
        begin_cycle();
        for (stage, words) in [
            (Stage::Hpro, 5usize),
            (Stage::Pro, 3),
            (Stage::Npro, 4),
            (Stage::Npim, 5),
            (Stage::Srk, 6),
            (Stage::Pim, 6),
            (Stage::Apro, 4),
            (Stage::Mpro, 21),
            (Stage::Hcp, 5),
        ] {
            mark_action(stage, &vec![stage_word(stage); words]);
        }
        assert_eq!(action_mask(), REQUIRED_ACTION_MASK);
        assert!(action_convergence_complete());
        let evidence = action_evidence();
        assert_eq!(replay_action_evidence(&evidence), (action_state(), action_mask()));
    }

    #[test]
    fn token_forge_readiness_does_not_enter_reasoning_action_state() {
        let _guard = test_guard();
        begin_cycle();
        mark_ready(Stage::TokenForge);
        assert_eq!(action_state(), 0);
        assert_eq!(action_mask(), 0);
        assert!(matches!(read(Stage::TokenForge), GclReadiness::Ready));
    }

    #[test]
    fn default_pulse_is_degraded_not_ready_lie() {
        let _guard = test_guard();
        // Para se organi të rrahë, presume s'thotë kurrë READY.
        match apro_status() {
            GclReadiness::Ready => panic!("APRO s'ka rrahur — READY do të ishte gënjeshtër"),
            _ => {}
        }
    }

    #[test]
    fn split_corrupt_updates_presume_status() {
        let _guard = test_guard();
        // Sjellja e kërkuar nga auditi: split i prishur → presume NOT_READY.
        mark_notready(Stage::Split);
        match split_status() {
            GclReadiness::NotReady(_) => {}
            other => panic!("split i prishur duhej NOT_READY, erdhi {:?}", other),
        }
        mark_ready(Stage::Split);
        assert!(split_status().is_ready(), "pas rrahjes së suksesshme → READY");
        mark_degraded(Stage::Split);
        match split_status() {
            GclReadiness::Degraded(_) => {}
            other => panic!("bosh i deklaruar duhej DEGRADED, erdhi {:?}", other),
        }
    }

    #[cfg(not(feature = "hw_kernel"))]
    #[test]
    fn hw_env_does_not_fake_hw_kernel() {
        let _guard = test_guard();
        // Sjellja e kërkuar nga auditi: env-i s'e falsifikon dot kernelin.
        std::env::set_var("ESSMAI_HW", "1");
        match hw_kernel_status() {
            GclReadiness::Ready => panic!("ESSMAI_HW=1 e falsifikoi hw_kernel — SHKELJE"),
            _ => {} // Degraded — e vërteta e kompilimit fiton
        }
        match thermal_status() {
            GclReadiness::Ready => panic!("termiku READY pa kernel — SHKELJE"),
            _ => {}
        }
        std::env::remove_var("ESSMAI_HW");
    }

    #[cfg(feature = "hw_kernel")]
    #[test]
    fn hw_kernel_compiled_reports_ready() {
        let _guard = test_guard();
        assert!(hw_kernel_status().is_ready());
    }
}
