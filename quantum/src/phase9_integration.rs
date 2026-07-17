// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  PHASE9_INTEGRATION.RS — Lidhja me Rrjedhën e Gjallë (Quantum)       ║
// ║  GJATA LEGACY™ — ESS-MAI Quantum                          (FAZA 9)      ║
// ║                                                                          ║
// ║  FAZA 9 — PATCHIM I RRJEDHËS SË GJALLË:                               ║
// ║  Lidh quantum_spine (HPRO→presion→paketim) me orchestrator-in real,     ║
// ║  duke respektuar TË 5 LIGJET SOVRANE.                                  ║
// ║                                                                          ║
// ║  LIGJI KRYESOR PËR QUANTUM — REASONING PURITY:                        ║
// ║   Quantum NUK shkruan KURRË knowledge persistent. Shkruan vetëm         ║
// ║   ephemeral (vdes me ciklin). Faza 9 e verifikon këtë para çdo lidhjeje.║
// ║                                                                          ║
// ║  Quantum prodhon EvidencePackage (dosje prove) → ia kalon Shadow-it.    ║
// ║   Quantum NUK jep verdikt final (Shadow vendos). Faza 9 e ruan këtë.   ║
// ║                                                                          ║
// ║  KUFIRI: Faza 9 lidh spine me rrjedhën; verifikon ligjet; s'shton       ║
// ║  vendim. Zero if/else klasik (match/boolean).                         ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::lab_contracts::trl::TrlEvidence;
use crate::lab_contracts::trust::TrustContext;
use crate::lab_contracts::evidence::EvidencePackage;
use crate::lab_contracts_v11::hardware::DeviceSnapshot;
use crate::laws::sovereign_laws::{ReasoningPurity, Destination, ModuleAuthority};
use crate::quantum_spine::{QuantumSpine, QuantumSpineResult};
use crate::layer3::layer3_flow::QuantumLayer3Result;

// ─────────────────────────────────────────────────────────────────────────────
// LIGJET E FAZËS 9 (Quantum)
// ─────────────────────────────────────────────────────────────────────────────

/// Identiteti i Quantum-it si shkrues (lejohet VETËM ephemeral).
pub const QUANTUM_WRITER_ID: &str = "quantum_engine";

/// Rezultati i kontrollit të ligjeve.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuantumLawCheck {
    Passed,
    ViolatedPurity,    // Quantum provoi të shkruajë persistent
    ViolatedAuthority, // veprim jashtë autoritetit
}

impl QuantumLawCheck {
    pub fn label(self) -> &'static str {
        match self {
            QuantumLawCheck::Passed            => "LAWS_PASSED",
            QuantumLawCheck::ViolatedPurity    => "VIOLATED_REASONING_PURITY",
            QuantumLawCheck::ViolatedAuthority => "VIOLATED_MODULE_AUTHORITY",
        }
    }

    /// is_passed — a kaluan ligjet? Zero if.
    pub fn is_passed(self) -> bool {
        match self {
            QuantumLawCheck::Passed => true,
            _ => false,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// PHASE 9 INTEGRATION (Quantum)
// ─────────────────────────────────────────────────────────────────────────────

/// Rezultati i arsyetimit me spine të lidhur.
#[derive(Debug, Clone)]
pub struct Phase9ReasonResult {
    pub law_check: QuantumLawCheck,
    pub spine:     Option<QuantumSpineResult>,  // Some nëse ligjet kaluan
    pub ready_for_shadow: bool,
}

/// Rezultati i orkestrimit HCP_PRO me ligje (PËRFORCIM     ).
#[derive(Debug, Clone)]
pub struct Phase9HcpResult {
    pub law_check:        QuantumLawCheck,
    /// Sa familje u ngritën paralel.
    pub active_count:     usize,
    /// Mesazhi i tipizuar për Shadow heart_byte (Some nëse ligjet kaluan).
    pub inbound:          Option<crate::bridge_shadow::HcpDecisionInbound>,
    pub ready_for_shadow: bool,
}

pub struct Phase9Integration;

impl Phase9Integration {
    /// reason_with_spine — lidh quantum_spine me rrjedhën, duke respektuar ligjet.
    ///
    /// Verifikon Reasoning Purity (Quantum → vetëm ephemeral) dhe Module
    /// Authority PARA se spine të aktivizohet. Zero if — match mbi ligjet.
    pub fn reason_with_spine(
        claim_id: u64,
        fragments: &[String],
        snapshot: &DeviceSnapshot,
        trl_evidence: TrlEvidence,
        trust_context: TrustContext,
        signal: f32,
        causal: f32,
    ) -> Phase9ReasonResult {
        // LIGJ 1: Reasoning Purity — Quantum shkruan VETËM ephemeral.
        let law_check = Self::check_quantum_laws();

        // Spine ekzekutohet VETËM nëse ligjet kalojnë. Zero if — match.
        let (spine, ready) = match law_check.is_passed() {
            true => {
                let result = QuantumSpine::reason_full(
                    claim_id, fragments, snapshot, trl_evidence, trust_context, signal, causal,
                );
                let r = QuantumSpine::is_ready_for_shadow(&result);
                (Some(result), r)
            }
            false => (None, false),
        };

        Phase9ReasonResult {
            law_check,
            spine,
            ready_for_shadow: ready,
        }
    }

    /// Production path: përdor hardware-in e matur/qeverisur nga i njëjti
    /// cycle runtime. Nuk ekzekuton HPRO dhe nuk krijon snapshot/lease të dytë.
    pub fn reason_with_runtime_hardware(
        claim_id: u64,
        fragments: &[String],
        hardware: QuantumLayer3Result,
        trl_evidence: TrlEvidence,
        trust_context: TrustContext,
        signal: f32,
        causal: f32,
    ) -> Phase9ReasonResult {
        let law_check = Self::check_quantum_laws();
        let (spine, ready) = match law_check.is_passed() {
            true => {
                let result = QuantumSpine::reason_full_with_hardware(
                    claim_id,
                    fragments,
                    hardware,
                    trl_evidence,
                    trust_context,
                    signal,
                    causal,
                );
                let ready = QuantumSpine::is_ready_for_shadow(&result);
                (Some(result), ready)
            }
            false => (None, false),
        };

        Phase9ReasonResult {
            law_check,
            spine,
            ready_for_shadow: ready,
        }
    }

    /// check_quantum_laws — verifikon që Quantum respekton Reasoning Purity.
    ///
    /// Quantum shkruan ephemeral (lejohet) por JO persistent. Verifikon edhe
    /// autoritetin e modulit. Zero if — match mbi rezultatet boolean.
    fn check_quantum_laws() -> QuantumLawCheck {
        // Reasoning Purity: Quantum → Ephemeral (lejohet).
        let purity_ephemeral = ReasoningPurity::enforce(QUANTUM_WRITER_ID, Destination::Ephemeral);
        // Reasoning Purity: Quantum → Persistent (NUK lejohet — duhet false).
        let purity_persistent = ReasoningPurity::enforce(QUANTUM_WRITER_ID, Destination::PersistentStore);

        // Quantum respekton ligjin: ephemeral OK AND persistent i bllokuar.
        let purity_ok = purity_ephemeral & !purity_persistent;

        // Module Authority: QuantumOrchestrator lejohet të bëjë "pipeline".
        // (Emri i regjistruar në AUTHORITY_MAP është "QuantumOrchestrator".)
        let authority_ok = ModuleAuthority::enforce("QuantumOrchestrator", "pipeline");

        // Match mbi (purity_ok, authority_ok) — zero if.
        match (purity_ok, authority_ok) {
            (false, _)    => QuantumLawCheck::ViolatedPurity,
            (true, false) => QuantumLawCheck::ViolatedAuthority,
            (true, true)  => QuantumLawCheck::Passed,
        }
    }

    /// produces_evidence_not_verdict — konfirmon që Quantum prodhon provë, jo verdikt.
    ///
    /// EvidencePackage është dosje prove (për Shadow), JO verdikt final.
    /// Faza 9 e ruan këtë kufi. Zero if.
    pub fn produces_evidence_not_verdict(result: &Phase9ReasonResult) -> bool {
        // Match mbi spine — nëse ekziston, është EvidencePackage (provë).
        match &result.spine {
            Some(spine) => Self::is_evidence_package(&spine.evidence),
            None => false,
        }
    }

    /// is_evidence_package — verifikon që output-i është dosje prove. Zero if.
    fn is_evidence_package(evidence: &EvidencePackage) -> bool {
        // Dosja ka pretendim + arsyetim + presion (jo verdikt 0/1).
        let has_claim = !evidence.claim.is_empty();
        let has_reasoning = !evidence.reasoning.is_empty();
        has_claim & has_reasoning
    }

    /// orchestrate_hcp_with_laws — lidh HCP_PRO L3 me ligjet + bridge (PËRFORCIM     ).
    ///
    /// PËRFORCIM koherence: HCP_PRO ngre familjet paralel (L3), vendimi vuloset me
    /// token, dhe prodhon HcpDecisionInbound për Shadow heart_byte. Verifikon
    /// Reasoning Purity para se vendimi të kalojë. Zero if — match.
    pub fn orchestrate_hcp_with_laws(
        gate: &mut crate::sovereign::lgc_gate::SovereignGate,
        envelope: &crate::hw_real::territories::PressureEnvelope,
        thermal: &crate::hw_real::thermal::ThermalReading,
        family_signals: &[crate::layer3::hcp_pro_l3::FamilySignals],
        hpro_result: &crate::layer3::hpro::HproResult,
        input_id: u64,
        token_nonce: u64,
    ) -> Phase9HcpResult {
        use crate::layer3::hcp_pro_l3::HcpProL3;
        use crate::bridge_shadow::HcpShadowBridge;

        // LIGJ: Reasoning Purity — HCP_PRO orkestron (ephemeral), s'shkruan persistent.
        let law_check = Self::check_quantum_laws();

        // HCP_PRO aktivizon familjet paralel VETËM nëse ligjet kalojnë. Zero if — match.
        let outcome = match law_check.is_passed() {
            true => {
                let activation = HcpProL3::activate_parallel(gate, envelope, thermal, family_signals, hpro_result);
                // Vendimi → bridge inbound për Shadow heart_byte.
                let inbound = HcpShadowBridge::from_parallel(input_id, &activation, token_nonce);
                let ready = HcpProL3::ready_to_package(&activation);
                Some((activation.active_count, inbound, ready))
            }
            false => None,
        };

        // Shpërndaj rezultatin. Zero if — match mbi Option.
        let (active_count, inbound, ready_for_shadow) = match outcome {
            Some((ac, inb, r)) => (ac, Some(inb), r),
            None => (0, None, false),
        };

        Phase9HcpResult {
            law_check,
            active_count,
            inbound,
            ready_for_shadow,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn spine9_wired_produces_evidence_not_verdict() {
        // Dëshmi që SPINE9 i lidhur në Quantum prodhon PROVË (jo verdikt).
        let snap = DeviceSnapshot::idle();
        let frags = vec!["flamuri eshte kuq".to_string(), "kuq e zi".to_string()];
        let trl = TrlEvidence::pending(0xA451);
        let trust = TrustContext::empty(0xA451, "letersi".to_string());
        let r = Phase9Integration::reason_with_spine(
            0xA451, &frags, &snap, trl, trust, 0.5, 0.6);
        assert!(r.law_check.is_passed());                          // Reasoning Purity OK
        assert!(Phase9Integration::produces_evidence_not_verdict(&r)); // PROVË, jo verdikt
        assert!(r.spine.is_some());                               // spine u aktivizua
    }

    use super::*;
    use crate::lab_contracts::trust::TrustGuidance;

    fn strong_device() -> DeviceSnapshot {
        DeviceSnapshot {
            ram_free_bytes: 4_000_000_000, ram_total_bytes: 8_000_000_000,
            cpu_load: 0.3, cores_active: 4, cores_total: 8,
            energy_margin: 0.9, on_battery: false,
            temp_c: 40.0, temp_limit_c: 90.0, timestamp: 100,
        }
    }

    fn ready_trl() -> TrlEvidence {
        let mut e = TrlEvidence::pending(1);
        e.trl_level = 3; e.trl_passed = true; e.confidence = 0.8; e.timestamp = 100;
        e
    }

    fn sample_trust() -> TrustContext {
        TrustContext {
            trace_id: 1, domain: "physics".to_string(),
            trust_score: 0.7, reliability_score: 0.6, verification_score: 0.5,
            history_refs: vec![1], guidance: TrustGuidance::Recommended,
        }
    }

    #[test]
    fn spine_runs_respecting_purity() {
        let fragments = vec!["solid idea".to_string()];
        let result = Phase9Integration::reason_with_spine(
            1, &fragments, &strong_device(), ready_trl(), sample_trust(), 0.7, 0.6,
        );
        // Quantum respekton Reasoning Purity → spine ekzekutoi.
        assert!(result.law_check.is_passed());
        assert!(result.spine.is_some());
    }

    #[test]
    fn quantum_purity_verified() {
        // Quantum lejohet ephemeral, jo persistent — ligji kalon.
        let check = Phase9Integration::check_quantum_laws();
        assert_eq!(check, QuantumLawCheck::Passed);
    }

    #[test]
    fn produces_evidence_not_verdict() {
        let fragments = vec!["idea".to_string()];
        let result = Phase9Integration::reason_with_spine(
            1, &fragments, &strong_device(), ready_trl(), sample_trust(), 0.5, 0.5,
        );
        // Quantum prodhon dosje prove, JO verdikt final.
        assert!(Phase9Integration::produces_evidence_not_verdict(&result));
    }

    #[test]
    fn ready_for_shadow_flag() {
        let fragments = vec!["x".to_string()];
        let result = Phase9Integration::reason_with_spine(
            1, &fragments, &strong_device(), ready_trl(), sample_trust(), 0.5, 0.5,
        );
        // Dosja gati për Shadow (Quantum s'vendos, vetëm përgatit).
        assert!(result.ready_for_shadow);
    }

    #[test]
    fn law_check_passed_logic() {
        assert!(QuantumLawCheck::Passed.is_passed());
        assert!(!QuantumLawCheck::ViolatedPurity.is_passed());
    }

    #[test]
    fn hcp_orchestration_reaches_bridge_via_laws() {
        use crate::sovereign::lgc_gate::SovereignGate;
        use crate::hw_real::territories::{PressureEnvelope, DepthHint};
        use crate::layer3::hcp_pro_l3::{FamilySignals, ProFamily};
        use crate::layer3::hpro::HproEngine;

        let mut gate = SovereignGate::new();
        let env = PressureEnvelope {
            locked_bytes: 1_000_000, released_bytes: 500_000, total_reserved: 1_500_000,
            demand_bytes: 800_000, under_pressure: false, depth_hint: DepthHint::Deep, ts_ns: 100,
        };
        let families = vec![
            FamilySignals::new(ProFamily::Pro, [0.8, 0.7, 0.9, 0.6]),
        ];
        let hpro = HproEngine::run(&strong_device());

        let thermal = crate::hw_real::thermal::ThermalReading::for_test(false);
        let result = Phase9Integration::orchestrate_hcp_with_laws(
            &mut gate, &env, &thermal, &families, &hpro, 5555, 0xABCD,
        );
        // Ligjet kaluan → HCP_PRO orkestroi + prodhoi mesazh për Shadow.
        assert!(result.law_check.is_passed());
        assert!(result.inbound.is_some());
        // Mesazhi mban input_id real.
        let inbound = result.inbound.unwrap();
        assert_eq!(inbound.input_id, 5555);
        assert_eq!(inbound.directive_bit, 1); // PushDeeper (deep, no pressure)
    }
}
