// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  SHADOW_RUNTIME_PULSE — PULSI REAL I AUTORITETIT (Shadow, v1.4.5)        ║
// ║  GJATA LEGACY™ | Arkitekt: Bledar Gjata                                  ║
// ║                                                                          ║
// ║  GJETJA (audit p.3, PRANUAR): gcl_presume në Shadow regjistronte         ║
// ║  judge_supreme/gcl_apply/negative_vault/trl_verdict/maturim me           ║
// ║  `gati_gjithnje` — deklarim, jo jetë. Quantum tashmë rreh; Shadow jo.    ║
// ║  Tani: çdo organ i verifikimit RREH kur ekzekutohet realisht.            ║
// ║                                                                          ║
// ║  LIGJET: zero varësi (vetëm std) · zero if/else · zero unwrap ·          ║
// ║  KURRË bllokues (pulsi informon; verdikti mbetet ligji i judge_supreme). ║
// ║  IZOMORF me quantum/runtime_pulse — i njëjti çelës jete, tjetër trup.    ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use std::sync::atomic::{AtomicU8, Ordering};
use crate::lab_contracts::gcl_presume::GclReadiness;

/// Organet e verifikimit që rrahin.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SStage {
    Judge,          // judge_supreme — verdikti Y
    GclApply,       // gcl_apply — Trust(X) trashëgon
    NegativeVault,  // apply_negative — (−Xi)(−Yi) në WAL
    TrlVerdict,     // TrlVerdict — prova faktike deri TRL4
    LegacyWritten,  // legacy u shkrua/pranua me gjurmë
    Maturation,     // run_maturation — pjekja e NK-së
}

const M: usize = 6;

/// 0=s'ka rrahur, 1=READY, 2=NOT_READY, 3=DEGRADED (bosh i deklaruar).
static PULSE: [AtomicU8; M] = [
    AtomicU8::new(0), AtomicU8::new(0), AtomicU8::new(0),
    AtomicU8::new(0), AtomicU8::new(0), AtomicU8::new(0),
];

fn idx(s: SStage) -> usize {
    match s {
        SStage::Judge         => 0,
        SStage::GclApply      => 1,
        SStage::NegativeVault => 2,
        SStage::TrlVerdict    => 3,
        SStage::LegacyWritten => 4,
        SStage::Maturation    => 5,
    }
}

pub fn mark_ready(s: SStage)    { PULSE[idx(s)].store(1, Ordering::SeqCst); }
pub fn mark_notready(s: SStage) { PULSE[idx(s)].store(2, Ordering::SeqCst); }
pub fn mark_degraded(s: SStage) { PULSE[idx(s)].store(3, Ordering::SeqCst); }

fn read(s: SStage) -> GclReadiness {
    match PULSE[idx(s)].load(Ordering::SeqCst) {
        1 => GclReadiness::Ready,
        2 => GclReadiness::NotReady("dështoi në runtime — shih rrjedha_ledger"),
        3 => GclReadiness::Degraded("bosh i deklaruar në runtime"),
        _ => GclReadiness::Degraded("s'ka rrahur ende në këtë proces"),
    }
}

pub fn judge_status()      -> GclReadiness { read(SStage::Judge) }
pub fn gcl_apply_status()  -> GclReadiness { read(SStage::GclApply) }
pub fn negvault_status()   -> GclReadiness { read(SStage::NegativeVault) }
pub fn trl_verdict_status()-> GclReadiness { read(SStage::TrlVerdict) }
pub fn legacy_status()     -> GclReadiness { read(SStage::LegacyWritten) }
pub fn maturation_status() -> GclReadiness { read(SStage::Maturation) }

#[cfg(test)]
mod shadow_pulse_tests {
    use super::*;

    #[test]
    fn judge_default_is_not_a_ready_lie() {
        // Para se judge të thirret, presume s'thotë READY.
        match judge_status() {
            GclReadiness::Ready => panic!("judge s'ka rrahur — READY do të ishte gënjeshtër"),
            _ => {}
        }
    }

    #[test]
    fn pulse_transitions_reflect_reality() {
        mark_ready(SStage::GclApply);
        assert!(gcl_apply_status().is_ready());
        mark_notready(SStage::NegativeVault);
        match negvault_status() {
            GclReadiness::NotReady(_) => {}
            other => panic!("pritej NOT_READY, erdhi {:?}", other),
        }
        mark_degraded(SStage::LegacyWritten);
        match legacy_status() {
            GclReadiness::Degraded(_) => {}
            other => panic!("pritej DEGRADED, erdhi {:?}", other),
        }
    }
}
