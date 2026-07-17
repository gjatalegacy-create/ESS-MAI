// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAB_CONTRACTS/MOD.RS — Gjuha e Përbashkët e Tre Laboratorëve         ║
// ║  GJATA LEGACY™ — ESS-MAI (BYTE-FOR-BYTE IDENTIK në 3 platformat)      ║
// ║                                                                          ║
// ║  Unified Layer 2 Verification Laboratory Framework — themeli.          ║
// ║                                                                          ║
// ║  Kjo dosje është GJUHA, jo FORCA dhe jo ROLI:                          ║
// ║    • lab_contracts = gjuhë e përbashkët (kjo dosje)                    ║
// ║    • Layer 2       = forcë (specifike për platformë)                   ║
// ║    • Platforma     = rol (Light/Quantum/Shadow)                        ║
// ║                                                                          ║
// ║  Mban VETËM kontrata (tipe): matje, gjendje, evidencë, vendim, gjurmë,  ║
// ║  mesazh, histori verifikimi. ASNJË logjikë platforme. ASNJË presion     ║
// ║  (presioni jeton te PressureReport + Layer 2 i secilës platformë).     ║
// ║                                                                          ║
// ║  KUFIRI ABSOLUT: ky skedar duhet të jetë byte-for-byte identik në       ║
// ║  Light, Quantum dhe Shadow. Asnjë platformë s'lejohet ta ndryshojë.    ║
// ╚══════════════════════════════════════════════════════════════════════════╝

pub mod trust;     // TrustContext — konteksti i besimit (Light prodhon)
pub mod trl;       // TrlEvidence — dëshmia e matjes TRL (e pastër, minimale)
pub mod pressure;  // PressureReport — kontrata e presionit (jo te TrlEvidence)
pub mod evidence;  // EvidencePackage — dosja e provës (për Shadow)
pub mod verdict;   // VerificationState + LabVerdict — gjendje & vendim
pub mod memory;    // VerificationLedgerEntry + NegativeKnowledgeRecord
pub mod message;   // LabMessage — gjuha lab-to-lab
pub mod domains;  // CoreDomain — regjistri kanonik i domeneve bërthamë
pub mod collapse; // Kolapsi 3-fazor — formula e gdhendur si kontratë
pub mod gjata_collapse_law; // LIGJI KUSHTETUES: ESS-MAI == gjata_collapse_law (Law 0)
pub mod pa_wire;
pub mod verification_receipt;   // PACP në urë — SHA-256 kanonik mbi receipt-in sovran (byte-për-byte)
pub mod rrjedha;   // INTELIGJENCA E DËSHTIMEVE — pse + shënim + kufi (kurrë bllokuese)
pub mod gcl_presume; // GATISHMËRIA NËN LIGJ — ping/rol/status nën CollapsePhase (v1.4.1)
pub mod first_guardian; // DËSHMIA E FRYMËS SË PARË — riti i ndezjes (v1.4.4)

// ── Ri-eksportim i sheshtë (gjuha e plotë në një vend) ──────────────────────
pub use trust::{TrustContext, TrustGuidance};
pub use trl::TrlEvidence;
pub use pressure::{PressureReport, PressureSource};
pub use evidence::{EvidencePackage, EvidenceKind};
pub use verdict::{VerificationState, LabVerdict};
pub use memory::{VerificationLedgerEntry, NegativeKnowledgeRecord};
pub use message::{LabMessage, LabNode, LabPayloadKind};
pub use domains::{CoreDomain, CORE_DOMAIN_COUNT, ALL_CORE_DOMAINS, normalize_domain_key};
pub use collapse::{CollapsePhase, PrimitiveSplit, NegativePath, CollapseOutcome};
pub use gjata_collapse_law::{UncertaintyLedger, LawViolation, ConstitutionalInvariant, pair_space, factic_probability, GjataCollapseLaw, SupremeDirective, LAW_MUSCLES, law_manifest, proclaim_law, SystemLaw, SYSTEM_LAWS, system_laws_seal, law_registry_report, PlatformRole, PLATFORM_ROLES, roles_declaration};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trl_evidence_pending_is_pending() {
        let e = TrlEvidence::pending(1);
        assert!(e.is_pending());
        assert_eq!(e.trl_level, 0);
        assert!(!e.meets_level(1));
    }

    #[test]
    fn trl_evidence_meets_level() {
        let mut e = TrlEvidence::pending(1);
        e.trl_level = 3;
        e.trl_passed = true;
        e.confidence = 0.8;
        e.timestamp = 100;
        assert!(e.meets_level(3));
        assert!(e.meets_level(2));
        assert!(!e.meets_level(4));
        assert!(!e.is_pending());
    }

    #[test]
    fn trust_context_empty_unknown() {
        let t = TrustContext::empty(5, "physics".to_string());
        assert!(!t.has_history());
        assert_eq!(t.guidance, TrustGuidance::Unknown);
        assert_eq!(t.combined_strength(), 0.0);
    }

    #[test]
    fn trust_context_combined_strength() {
        let t = TrustContext {
            trace_id: 1, domain: "d".to_string(),
            trust_score: 1.0, reliability_score: 1.0, verification_score: 1.0,
            history_refs: vec![1, 2], guidance: TrustGuidance::Recommended,
        };
        assert!(t.has_history());
        // 1.0*0.40 + 1.0*0.25 + 1.0*0.35 = 1.0
        assert!((t.combined_strength() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn pressure_report_withstood() {
        let mut p = PressureReport::none(1, PressureSource::QuantumNpro);
        assert!(p.withstood()); // pa kontradikta/dobësi/dështime
        p.contradiction_score = 0.5;
        assert!(!p.withstood()); // shumë kontradikta
    }

    #[test]
    fn pressure_report_failure_breaks_withstood() {
        let mut p = PressureReport::none(1, PressureSource::QuantumNpro);
        p.failure_modes.push("breaks under load".to_string());
        assert!(!p.withstood());
    }

    #[test]
    fn evidence_package_completeness() {
        let pkg = EvidencePackage {
            claim_id: 1,
            claim: "X works".to_string(),
            reasoning: "because Y".to_string(),
            trl_evidence: {
                let mut e = TrlEvidence::pending(1);
                e.trl_level = 3; e.trl_passed = true; e.confidence = 0.8; e.timestamp = 1;
                e
            },
            trust_context: TrustContext::empty(1, "d".to_string()),
            pressure_report: PressureReport::none(1, PressureSource::QuantumNpro),
            evidence_chain: vec!["hash1".to_string()],
            cross_domain_checks: vec!["domain2".to_string()],
            kind: EvidenceKind::Documented,
        };
        // Të 5 komponentët presentë → completeness 1.0
        assert!((pkg.completeness() - 1.0).abs() < 1e-6);
        assert!(pkg.has_physical_evidence());
        assert_eq!(pkg.cross_domain_count(), 1);
    }

    #[test]
    fn verification_state_ordering() {
        assert!(VerificationState::EvidenceVerified.at_least(VerificationState::Reasoned));
        assert!(!VerificationState::Reasoned.at_least(VerificationState::EvidenceVerified));
        assert!(VerificationState::EvidenceVerified.is_verified());
        assert!(!VerificationState::PressureTested.is_verified());
        assert_eq!(VerificationState::Unverified.rank(), 0);
        assert_eq!(VerificationState::OperationallyVerified.rank(), 5);
    }

    #[test]
    fn lab_verdict_bit_and_accept() {
        let acc = LabVerdict::Accepted { trl: 3, state: VerificationState::EvidenceVerified, reason: "ok".to_string() };
        assert!(acc.is_accepted());
        assert_eq!(acc.bit(), 1);
        let rej = LabVerdict::Rejected { reason: "weak".to_string(), weaknesses: vec![] };
        assert!(!rej.is_accepted());
        assert_eq!(rej.bit(), 0);
        let need = LabVerdict::NeedsMoreEvidence { missing: vec!["video".to_string()] };
        assert_eq!(need.bit(), 0);
    }

    #[test]
    fn ledger_entry_passing() {
        let entry = VerificationLedgerEntry {
            claim_id: 1, state: VerificationState::EvidenceVerified,
            evidence_hash: "h".to_string(), pressure_score: 0.8,
            cross_domain_score: 0.7, final_verdict: 1, recorded_at: 100,
        };
        assert!(entry.is_passing());
        assert!(entry.strength() > 0.5);
    }

    #[test]
    fn negative_knowledge_valuable() {
        let mut nk = NegativeKnowledgeRecord::new(
            1, "destabilizes".to_string(), "npro->contradiction".to_string(), true, 100,
        );
        assert!(!nk.is_valuable()); // ende pa konsultime
        nk.access_count = 3;
        assert!(nk.is_valuable()); // verifikuar + konsultuar
    }

    #[test]
    fn lab_message_seal_and_route() {
        let msg = LabMessage::new(
            1, LabNode::Quantum, LabNode::Shadow,
            LabPayloadKind::EvidencePackage, 1, 100,
        );
        assert!(msg.seal_ok());      // vula 500
        assert!(msg.is_valid_route()); // Quantum != Shadow
        assert_eq!(msg.seal, 0xA451);
    }

    #[test]
    fn lab_message_same_node_invalid_route() {
        let msg = LabMessage::new(
            1, LabNode::Light, LabNode::Light,
            LabPayloadKind::TrustContext, 1, 100,
        );
        assert!(!msg.is_valid_route());
    }
}
