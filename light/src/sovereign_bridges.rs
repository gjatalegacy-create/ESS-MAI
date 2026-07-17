// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  SOVEREIGN_BRIDGES.RS — Urat Sovrane (Light ↔ Quantum/Shadow)        ║
// ║  GJATA LEGACY™ — Light Platform                                   ║
// ║                                                                          ║
// ║  Lidh 3 algoritmet e  me platformat e tjera, me mbrojtje        ║
// ║  sovrane (CapHandle një-përdorimësh):                                  ║
// ║                                                                          ║
// ║   APUPK  → Shadow (shadow_APUPK_memory) — pako projekti me vulë       ║
// ║   SNB    → Shadow (shadow_SNB)          — bug report me vulë          ║
// ║   Mini   → Quantum (knowledge trace)    — usage stats me vulë         ║
// ║                                                                          ║
// ║  Light VETËM përgatit dhe orienton — NUK vendos (Shadow vendos).       ║
// ║  Çdo kalim FFI mbrohet me CapHandle: kopja e handle-it është e kotë.   ║
// ║  Zero if/else — match/formula.                                        ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::sovereign::{
    SovereignGate, CapHandle, SovErr,
    SEAL_APUPK, SEAL_SNB, SEAL_MINI_EVOLVER, verify_500,
};
use crate::apupk::{ShadowApupkPackage, ApupkCoordinator, ProjectUpload};
use crate::snb::{SnbAlgorithm, ShadowBugReport};
use crate::mini_evolver::LightMiniEvolver;
use sha2::{Digest, Sha256};

// ─────────────────────────────────────────────────────────────────────────────
// SEALED PACKAGE — pako me vulë sovrane për kalim ndër-platformë
// ─────────────────────────────────────────────────────────────────────────────

/// Pako e vulosur që kalon nga Light te një platformë tjetër.
/// handle: capability një-përdorimësh. seal_500: vula primitive.
#[derive(Debug, Clone)]
pub struct SealedTransfer {
    pub handle:     CapHandle,
    pub seal_500:   u32,        // flags ku verify_500 == true
    pub destination: &'static str,
    pub payload_len: usize,
}

impl SealedTransfer {
    /// is_valid — a është vula 500 e saktë? Zero if.
    pub fn is_valid(&self) -> bool {
        verify_500(self.seal_500)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// APUPK → SHADOW BRIDGE
// ─────────────────────────────────────────────────────────────────────────────

pub struct ApupkShadowBridge;

impl ApupkShadowBridge {
    /// prepare_sealed — APUPK përgatit pako për Shadow me vulë sovrane.
    ///
    /// Light lëshon CapHandle (SEAL_APUPK), e bashkëngjit pakos.
    /// Shadow validon handle-in para se ta pranojë.
    pub fn prepare_sealed(
        gate: &mut SovereignGate,
        coordinator: &ApupkCoordinator,
        project: ProjectUpload,
    ) -> (ShadowApupkPackage, SealedTransfer) {
        let package = coordinator.prepare_package_for_shadow(project);

        // Lësho capability për këtë kalim.
        let handle = gate.issue(SEAL_APUPK);

        let transfer = SealedTransfer {
            handle,
            seal_500: 0xA451,  // (0xA451 & 0xFFFF) ^ 0xA5A5 = 500
            destination: "shadow_APUPK_memory",
            payload_len: package.project_content.len(),
        };

        (package, transfer)
    }

    /// validate_at_shadow — Shadow validon handle-in (një herë).
    /// Kthen Ok kur handle valid, Err kur klon/replay.

    /// register_with_shadow_main — ruan APUPK vetëm përmes procesit sovran
    /// Shadow dhe merr një dëshmitar të kufizuar. CapHandle mbetet kontroll
    /// lokal i Light-it; në kufirin e procesit udhëton Vula 500 + materiali.
    pub fn register_with_shadow_main(
        gate: &SovereignGate,
        package: &ShadowApupkPackage,
        transfer: &SealedTransfer,
    ) -> Result<shadow_contracts::ProjectContextWitnessWire, String> {
        match transfer.destination == "shadow_APUPK_memory" {
            true => {}
            false => return Err("APUPK destination e pavlefshme".to_string()),
        }
        Self::validate_at_shadow(gate, transfer)
            .map_err(|e| format!("APUPK capability u refuzua: {e:?}"))?;
        let request = shadow_contracts::ProjectRegistrationRequestWire {
            project_id: package.trace.project_id,
            user_id: package.trace.user_id,
            trace_id: package.trace.trace_id,
            project_title: package.project_title.clone(),
            initial_trace: package.trace.initial_trace.clone(),
            project_content: package.project_content.clone(),
            progress_percentage: package.initial_progress.progress_percentage,
            last_update_ns: package.initial_progress.last_update_ns,
            notes: package.initial_progress.notes.clone(),
            light_sovereign_flags: transfer.seal_500,
        };
        crate::project_process_bridge::register_project(request)
    }

    /// v1.6.5 — rruga default e magazinës/bisedës së projektit.
    /// APUPK dhe Vula 500 mbeten identike; Quantum vetëm orienton workspace-in
    /// dhe nuk aktivizon procedimin e plotë shkencor.
    #[allow(clippy::too_many_arguments)]
    pub fn route_project_workspace_under_gcl(
        gate: &mut SovereignGate,
        coordinator: &ApupkCoordinator,
        project: ProjectUpload,
        domain: &str,
        lang_code: &str,
        evolve_summary: &str,
        hypothesis: &str,
        assumptions: &[String],
        documentation_description: &str,
        files: &[(u8, Vec<u8>)],
    ) -> Result<shadow_contracts::ProjectContextWitnessWire, String> {
        let (witness, quantum_payload) = Self::prepare_project_handoff_under_gcl(
            gate,
            coordinator,
            project,
            domain,
            lang_code,
            evolve_summary,
            hypothesis,
            assumptions,
            documentation_description,
            files,
        )?;
        crate::project_process_bridge::orient_project_workspace_with_quantum_main(
            quantum_payload, &witness,
        ).map_err(|e| format!("Light→Quantum Project Workspace handoff dështoi: {e}"))?;
        Ok(witness)
    }

    /// Rruga legacy e v1.6.4: APUPK identity → Shadow witness → Quantum
    /// procedim i plotë shkencor. Mbahet e ndarë dhe nuk ndryshohet nga porta
    /// e re e magazinimit/bisedës.
    #[allow(clippy::too_many_arguments)]
    pub fn route_scientific_project_under_gcl(
        gate: &mut SovereignGate,
        coordinator: &ApupkCoordinator,
        project: ProjectUpload,
        domain: &str,
        lang_code: &str,
        evolve_summary: &str,
        hypothesis: &str,
        assumptions: &[String],
        documentation_description: &str,
        files: &[(u8, Vec<u8>)],
    ) -> Result<shadow_contracts::ProjectContextWitnessWire, String> {
        let (witness, quantum_payload) = Self::prepare_project_handoff_under_gcl(
            gate,
            coordinator,
            project,
            domain,
            lang_code,
            evolve_summary,
            hypothesis,
            assumptions,
            documentation_description,
            files,
        )?;
        crate::project_process_bridge::execute_project_with_quantum_main(
            quantum_payload, &witness,
        ).map_err(|e| format!("Light→Quantum main project handoff dështoi: {e}"))?;
        Ok(witness)
    }

    #[allow(clippy::too_many_arguments)]
    fn prepare_project_handoff_under_gcl(
        gate: &mut SovereignGate,
        coordinator: &ApupkCoordinator,
        project: ProjectUpload,
        domain: &str,
        lang_code: &str,
        evolve_summary: &str,
        hypothesis: &str,
        assumptions: &[String],
        documentation_description: &str,
        files: &[(u8, Vec<u8>)],
    ) -> Result<(shadow_contracts::ProjectContextWitnessWire, String), String> {
        let project_title = project.project_name.clone();
        let project_content = project.content.clone();
        let (package, transfer) = Self::prepare_sealed(gate, coordinator, project);
        let witness = Self::register_with_shadow_main(gate, &package, &transfer)?;
        let input_sha256 = format!("{:x}", Sha256::digest(project_content.as_bytes()));
        match witness.content_sha256 == input_sha256 {
            true => {}
            false => return Err(
                "ProjectContextWitness nuk lidhet me SHA-256 e inputit Light".to_string(),
            ),
        }
        // Light është pronari i kufirit kushtetues. UI nuk furnizon kontratë
        // ose Vulë: ato derivojnë vetëm nga rrjedha APUPK dhe witness-i real.
        // Domain-i mbetet V164 sepse v1.6.5 nuk ndryshon GCL/Living Trust.
        let contract_id = "GCL:SCIENTIFIC_PROJECT:V164";
        let lgc_seal = format!(
            "LGC::C|flags:0x{:08X}", witness.light_sovereign_flags,
        );
        let quantum_payload = crate::quantum_bridge::build_project_payload(
            &package.trace.trace_id.to_string(),
            domain,
            contract_id,
            &lgc_seal,
            lang_code,
            evolve_summary,
            &input_sha256,
            &project_content,
            &witness,
            &project_title,
            hypothesis,
            assumptions,
            documentation_description,
            files,
        );
        Ok((witness, quantum_payload))
    }

    pub fn validate_at_shadow(
        gate: &SovereignGate,
        transfer: &SealedTransfer,
    ) -> Result<(), SovErr> {
        // 1. Vula 500 duhet të jetë e saktë (match mbi bool).
        let seal_ok = transfer.is_valid();
        // 2. CapHandle validohet një herë.
        match seal_ok {
            true => gate.validate(transfer.handle).map(|_token| ()),
            false => Err(SovErr::SealMismatch(transfer.seal_500 as u64)),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// SNB → SHADOW BRIDGE
// ─────────────────────────────────────────────────────────────────────────────

pub struct SnbShadowBridge;

impl SnbShadowBridge {
    /// prepare_sealed — SNB përgatit bug report për Shadow me vulë.
    ///
    /// Vetëm kur ka bug (prepare_for_shadow_snb kthen Some).
    /// Zero if — match mbi Option.
    pub fn prepare_sealed(
        gate: &mut SovereignGate,
        snb: &SnbAlgorithm,
    ) -> Option<(ShadowBugReport, SealedTransfer)> {
        // SNB kthen Some vetëm kur ka bug.
        match snb.prepare_for_shadow_snb() {
            None => None,
            Some(report) => {
                let handle = gate.issue(SEAL_SNB);
                let transfer = SealedTransfer {
                    handle,
                    seal_500: 0xA451,
                    destination: "shadow_SNB",
                    payload_len: report.description.len(),
                };
                Some((report, transfer))
            }
        }
    }

    /// validate_at_shadow — Shadow validon bug report.

    pub fn validate_at_shadow(
        gate: &SovereignGate,
        transfer: &SealedTransfer,
    ) -> Result<(), SovErr> {
        let seal_ok = transfer.is_valid();
        match seal_ok {
            true => gate.validate(transfer.handle).map(|_token| ()),
            false => Err(SovErr::SealMismatch(transfer.seal_500 as u64)),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// MINI EVOLVER → QUANTUM BRIDGE
// ─────────────────────────────────────────────────────────────────────────────

/// Pako e knowledge usage që Mini Evolver dërgon te Quantum.
#[derive(Debug, Clone)]
pub struct KnowledgeUsageSummary {
    pub knowledge_id:    u64,
    pub usage_count:     usize,
    pub avg_linear_mass: f32,
    pub avg_vector_score: f32,
}

pub struct MiniEvolverQuantumBridge;

impl MiniEvolverQuantumBridge {
    /// summarize_sealed — Mini Evolver përmbledh usage për Quantum me vulë.
    ///
    /// Quantum përdor këto statistika për të prioritizuar dijen e përdorur shpesh.
    pub fn summarize_sealed(
        gate: &mut SovereignGate,
        evolver: &LightMiniEvolver,
        knowledge_id: u64,
    ) -> (KnowledgeUsageSummary, SealedTransfer) {
        // avg_linear_mass: None → 0.0 (branchless via unwrap_or).
        let avg_linear_mass = evolver.get_average_linear_mass(knowledge_id).unwrap_or(0.0);
        let avg_vector_score = evolver.get_average_vector_score(knowledge_id).unwrap_or(0.0);
        let usage_count = evolver.get_usage_count(knowledge_id);

        let summary = KnowledgeUsageSummary {
            knowledge_id,
            usage_count,
            avg_linear_mass,
            avg_vector_score,
        };

        let handle = gate.issue(SEAL_MINI_EVOLVER);
        let transfer = SealedTransfer {
            handle,
            seal_500: 0xA451,
            destination: "quantum_knowledge_trace",
            payload_len: usage_count,
        };

        (summary, transfer)
    }

    /// validate_at_quantum — Quantum validon usage summary.
    pub fn validate_at_quantum(
        gate: &SovereignGate,
        transfer: &SealedTransfer,
    ) -> Result<(), SovErr> {
        let seal_ok = transfer.is_valid();
        match seal_ok {
            true => gate.validate(transfer.handle).map(|_token| ()),
            false => Err(SovErr::SealMismatch(transfer.seal_500 as u64)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn project() -> ProjectUpload {
        ProjectUpload {
            project_id: 1,
            user_id: 100,
            project_name: "Test Project".to_string(),
            description: "desc".to_string(),
            content: "project content here".to_string(),
            timestamp_ns: 1000,
        }
    }

    #[test]
    fn apupk_sealed_transfer_valid() {
        let mut gate = SovereignGate::new();
        let coord = ApupkCoordinator::new();
        let (_pkg, transfer) = ApupkShadowBridge::prepare_sealed(&mut gate, &coord, project());
        // Vula 500 është e saktë.
        assert!(transfer.is_valid());
        assert_eq!(transfer.destination, "shadow_APUPK_memory");
    }

    #[test]
    fn apupk_handle_single_use() {
        let mut gate = SovereignGate::new();
        let coord = ApupkCoordinator::new();
        let (_pkg, transfer) = ApupkShadowBridge::prepare_sealed(&mut gate, &coord, project());
        // Hera e parë: OK.
        assert!(ApupkShadowBridge::validate_at_shadow(&gate, &transfer).is_ok());
        // Hera e dytë (klon/replay): refuzim.
        assert_eq!(
            ApupkShadowBridge::validate_at_shadow(&gate, &transfer).unwrap_err(),
            SovErr::AlreadyConsumed
        );
    }

    #[test]
    fn snb_no_bug_no_transfer() {
        let mut gate = SovereignGate::new();
        let snb = SnbAlgorithm::new(); // pa bug
        // Pa bug → None.
        assert!(SnbShadowBridge::prepare_sealed(&mut gate, &snb).is_none());
    }

    #[test]
    fn snb_with_bug_sealed() {
        let mut gate = SovereignGate::new();
        let mut snb = SnbAlgorithm::new();
        snb.record_module("light_coordinator");
        snb.report_bug("apupk", "test bug", 5);
        // Me bug → Some me vulë.
        let result = SnbShadowBridge::prepare_sealed(&mut gate, &snb);
        assert!(result.is_some());
        let (_report, transfer) = result.unwrap();
        assert!(transfer.is_valid());
        assert_eq!(transfer.destination, "shadow_SNB");
    }

    #[test]
    fn mini_evolver_summary_sealed() {
        let mut gate = SovereignGate::new();
        let mut evolver = LightMiniEvolver::new();
        evolver.on_algorithm_step(42, "module_a", 0.8, 0.9);
        evolver.on_algorithm_step(42, "module_b", 0.6, 0.7);
        let (summary, transfer) = MiniEvolverQuantumBridge::summarize_sealed(&mut gate, &evolver, 42);
        // Usage count = 2.
        assert_eq!(summary.usage_count, 2);
        // Avg llogaritur.
        assert!((summary.avg_linear_mass - 0.7).abs() < 1e-5);
        assert!(transfer.is_valid());
    }

    #[test]
    fn invalid_seal_rejected() {
        let mut gate = SovereignGate::new();
        let coord = ApupkCoordinator::new();
        let (_pkg, mut transfer) = ApupkShadowBridge::prepare_sealed(&mut gate, &coord, project());
        // Korrupto vulën 500.
        transfer.seal_500 = 0x1234;
        // Validimi refuzon (vula e pavlefshme).
        assert!(ApupkShadowBridge::validate_at_shadow(&gate, &transfer).is_err());
    }

    #[test]
    fn distinct_seals_per_bridge() {
        // Çdo urë përdor vulën e vet moduli.
        assert_ne!(SEAL_APUPK, SEAL_SNB);
        assert_ne!(SEAL_SNB, SEAL_MINI_EVOLVER);
    }
}
