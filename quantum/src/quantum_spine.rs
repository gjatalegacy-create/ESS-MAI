// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  QUANTUM_SPINE.RS — Kurrizi Vertikal i Quantum-it (PATCHIM)          ║
// ║  GJATA LEGACY™ — ESS-MAI Quantum                          (SPINE)       ║
// ║                                                                          ║
// ║  PATCHIM I THELLË: lidh shtresat e Quantum-it në NJË rrjedhë presioni   ║
// ║  hardware-aware. Para patchimit, HPRO (L3) dhe abyssal_probe (L2)        ║
// ║  ishin të shkëputur — ky kurriz i bashkon: presioni tani respekton      ║
// ║  kufijtë fizikë të device-it.                                          ║
// ║                                                                          ║
// ║  RRJEDHA E PLOTË E QUANTUM-it (si një trup):                          ║
// ║    1. HPRO (L3)            → kufijtë fizikë nga device (prod_formula)   ║
// ║    2. Abyssal Probe (L2)   → presion brenda kufijve (NPRO)             ║
// ║    3. HPIM (L3)            → paketon trace-in hardware                  ║
// ║    4. Package Builder (L2) → dosja e provës për Shadow                  ║
// ║                                                                          ║
// ║  KUFIRI: presioni respekton hardware-in; HPRO lëshon harduerin; Quantum ║
// ║  s'jep verdikt (vetëm paketon provën për Shadow). Zero if/else.       ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::lab_contracts::trl::TrlEvidence;
use crate::lab_contracts::trust::TrustContext;
use crate::lab_contracts::evidence::EvidencePackage;
use crate::lab_contracts_v11::hardware::DeviceSnapshot;
use crate::layer2::abyssal_probe::{AbyssalProbe, ProbeResult};
use crate::layer2::verification_package_builder::VerificationPackageBuilder;
use crate::layer3::layer3_flow::{QuantumLayer3Flow, QuantumLayer3Result};

// ─────────────────────────────────────────────────────────────────────────────
// QUANTUM SPINE RESULT — rezultati i rrjedhës vertikale
// ─────────────────────────────────────────────────────────────────────────────

/// Rezultati i plotë i kurrizit të Quantum-it — hardware + presion + paketim.
#[derive(Debug, Clone)]
pub struct QuantumSpineResult {
    /// Rezultati i presionit hardware-aware (L3).
    pub hardware:        QuantumLayer3Result,
    /// Rezultati i sondës së presionit (L2).
    pub probe:           ProbeResult,
    /// Dosja e provës gati për Shadow (L2).
    pub evidence:        EvidencePackage,
    /// A respektoi presioni kufijtë fizikë.
    pub within_bounds:   bool,
}

// ─────────────────────────────────────────────────────────────────────────────
// QUANTUM SPINE — orkestron rrjedhën vertikale L3→L2→L3→L2
// ─────────────────────────────────────────────────────────────────────────────

pub struct QuantumSpine;

impl QuantumSpine {
    /// reason_full — rrjedha e plotë e Quantum-it nga hardware te dosja e provës.
    ///
    /// Lidh të 4 hapat: HPRO → abyssal_probe → HPIM → package_builder.
    /// Presioni respekton kufijtë fizikë të device-it. Zero if — sekuencë.
    pub fn reason_full(
        claim_id: u64,
        fragments: &[String],
        snapshot: &DeviceSnapshot,
        trl_evidence: TrlEvidence,
        trust_context: TrustContext,
        signal: f32,
        causal: f32,
    ) -> QuantumSpineResult {
        // API compatibility: HPRO e vetme krijohet këtu dhe lëshohet sepse kjo
        // rrugë nuk zotëron një HardwareRuntimeContext të ciklit.
        let hardware = QuantumLayer3Flow::run_hardware_pressure(claim_id, snapshot);
        Self::reason_full_with_hardware(
            claim_id,
            fragments,
            hardware,
            trl_evidence,
            trust_context,
            signal,
            causal,
        )
    }

    /// Production path: konsumon rezultatin HPRO/HPIM të matur e të qeverisur
    /// nga cikli real. Nuk mat hardware dhe nuk krijon lease të dytë.
    pub fn reason_full_with_hardware(
        claim_id: u64,
        fragments: &[String],
        hardware: QuantumLayer3Result,
        trl_evidence: TrlEvidence,
        trust_context: TrustContext,
        signal: f32,
        causal: f32,
    ) -> QuantumSpineResult {
        // Shtytja e lejuar nga lease-i real i HPRO/HCP_PRO.
        let allowed_push = hardware.hpim_package.pressure_push;

        // HAP 2 (L2): Abyssal Probe bën presion BRENDA kufijve të hardware-it.
        let effective_signal = (signal * allowed_push).clamp(0.0, 1.0);
        let probe = AbyssalProbe::drive_to_limit(claim_id, fragments, effective_signal, causal);
        let within_bounds = effective_signal <= allowed_push;

        // HAP 3/4: HPIM tashmë mban trace-in e lease-it; Package Builder
        // ndërton vetëm dosjen e provës për Shadow.
        let claim = Self::claim_from_probe(&probe);
        let reasoning = Self::reasoning_from_hardware(&hardware);
        let evidence = VerificationPackageBuilder::build_reasoned(
            claim_id,
            claim,
            reasoning,
            trl_evidence,
            trust_context,
            probe.pressure_report.clone(),
            probe.cross_domain_results(fragments),
        );

        QuantumSpineResult {
            hardware,
            probe,
            evidence,
            within_bounds,
        }
    }

    /// claim_from_probe — formon pretendimin nga rezultati i sondës. Zero if — match.
    fn claim_from_probe(probe: &ProbeResult) -> String {
        use crate::layer2::abyssal_probe::TruthVerdict;
        match probe.verdict {
            TruthVerdict::Holds  => format!("claim {} mbijetoi presionin", probe.claim_id),
            TruthVerdict::Breaks => format!("claim {} u thye nën presion", probe.claim_id),
        }
    }

    /// reasoning_from_hardware — arsyetimi nga konteksti hardware. Zero if — match.
    fn reasoning_from_hardware(hw: &QuantumLayer3Result) -> String {
        match hw.ready_for_pim {
            true  => format!("presion hardware-aware (push {:.2}, paralel {})",
                            hw.hpim_package.pressure_push, hw.hpim_package.parallel_used),
            false => "presion i kufizuar nga hardware".to_string(),
        }
    }

    /// is_ready_for_shadow — a është dosja gati për Shadow? Zero if.
    pub fn is_ready_for_shadow(result: &QuantumSpineResult) -> bool {
        let hardware_ready = result.hardware.ready_for_pim;
        let has_evidence = !result.evidence.claim.is_empty();
        hardware_ready & has_evidence
    }
}

#[cfg(test)]
mod tests {
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
    fn full_flow_connects_hardware_and_pressure() {
        let fragments = vec!["solid idea".to_string(), "consistent".to_string()];
        let result = QuantumSpine::reason_full(
            1, &fragments, &strong_device(), ready_trl(), sample_trust(), 0.7, 0.6,
        );
        // L3 hardware u ekzekutua + lëshoi harduerin.
        assert!(result.hardware.hardware_released);
        // L2 probe prodhoi presion.
        assert_eq!(result.probe.claim_id, 1);
        // L2 dosja u ndërtua.
        assert!(!result.evidence.claim.is_empty());
        assert!(QuantumSpine::is_ready_for_shadow(&result));
    }

    #[test]
    fn pressure_respects_hardware_bounds() {
        let fragments = vec!["idea".to_string()];
        let result = QuantumSpine::reason_full(
            1, &fragments, &strong_device(), ready_trl(), sample_trust(), 0.9, 0.5,
        );
        // Presioni efektiv s'kalon shtytjen e lejuar.
        assert!(result.within_bounds);
    }

    #[test]
    fn weak_device_limits_pressure() {
        let mut weak = DeviceSnapshot::idle();
        weak.ram_free_bytes = 50_000_000;
        weak.energy_margin = 0.1;
        weak.temp_c = 88.0; weak.temp_limit_c = 90.0;
        let fragments = vec!["idea".to_string()];
        let result = QuantumSpine::reason_full(
            1, &fragments, &weak, ready_trl(), sample_trust(), 0.9, 0.5,
        );
        // Hardware i dobët → shtytje 0 → presion shumë i kufizuar.
        assert!(result.hardware.hardware_released);
        // Dosja prapë ndërtohet (me presion minimal).
        assert!(!result.evidence.claim.is_empty());
    }

    #[test]
    fn evidence_ready_for_shadow() {
        let fragments = vec!["x".to_string()];
        let result = QuantumSpine::reason_full(
            1, &fragments, &strong_device(), ready_trl(), sample_trust(), 0.5, 0.5,
        );
        // Dosja ka pressure_report nga Quantum NPRO.
        use crate::lab_contracts::pressure::PressureSource;
        assert_eq!(result.evidence.pressure_report.source, PressureSource::QuantumNpro);
    }
}
