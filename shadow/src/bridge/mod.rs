// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  BRIDGE — URAT REALE Quantum→Shadow & Light→Shadow (FAZA 3)           ║
// ║  Gjata Legacy™ | Bledar Gjata                                        ║
// ║                                                                          ║
// ║  Shadow s'varet nga crate-et e Quantum/Light. Këtu pasqyrohen KONTRATAT ║
// ║  e tyre si tipa kufitarë (`QuantumInbound`, `LightInbound`) dhe         ║
// ║  adaptohen → tipat e brendshëm (`PassPackage`, `LightEnvelope`).       ║
// ║                                                                          ║
// ║  DY ORIGJINA, TË NDARA: PassPackage (propozim) dhe LightEnvelope        ║
// ║  (transport+vulë) bashkohen VETËM brenda ShadowPassage gjatë ingest.   ║
// ╚══════════════════════════════════════════════════════════════════════════╝

pub mod light_in;
pub mod quantum_in;
#[cfg(test)]
pub mod shadow_out;
#[cfg(test)]
pub mod shadow_callable;   // vetëm test: production hyn përmes main/process_bridge

pub use light_in::LightInbound;
pub use quantum_in::QuantumInbound;
#[cfg(test)]
pub use shadow_out::{
    DefaultLightShadowBridge, LightShadowBridge, ShadowLightResponse, SupremeOutcome,
};
#[cfg(test)]
pub use shadow_callable::{
    ShadowCallable, ShadowEndpoint, LightVerdict, LightResponse,
};

#[cfg(test)]
use crate::types::{LightEnvelope, PassPackage};

/// Normalizim i përbashkët i skorëve në [0,1].
pub(crate) fn clamp01(x: f32) -> f32 {
    x.clamp(0.0, 1.0)
}

/// Ndan një paketë të vetme Quantum në (PassPackage, LightEnvelope), duke
/// modeluar rrjedhën reale Quantum→Light→Shadow: Light mbart
/// session/territory/vula/proof_chain te kanali i dytë.
///
/// ZERO-COPY REAL (      ): përdor `split_zero_copy()` që ZHVENDOS të gjitha fushat
/// pa asnjë klon. Para: `derive_light()` klononte session_id/territory/raw_bytes,
/// pastaj `into_pass_package()` zhvendoste — dyfishim i panevojshëm (payload-i i
/// Light s'lexohet kurrë në pipeline). Tani: një destrukturim, zhvendosje totale.
/// record_hcp_heart është lidhja production dhe thirret nga ingest_bridged,
/// që vetë thirret vetëm nga process_bridge i main.rs. Helper-i split më poshtë
/// ruhet vetëm për testet e adapterit dhe nuk përfaqëson hyrje production.
pub(crate) fn record_hcp_heart(q: &QuantumInbound) {
    match (q.hcp_input_id, q.hcp_token_gen, q.hcp_token_nonce, q.hcp_directive_bit) {
        (Some(id), Some(gen), Some(nonce), Some(bit)) => {
            let _ = crate::ess_mai_heart_byte::EssMaiHeartByte::beat_from_inbound(id, gen, nonce, bit);
        }
        _ => {}
    }
}

#[cfg(test)]
pub(crate) fn split_and_record_heart(q: QuantumInbound) -> (PassPackage, LightEnvelope) {
    record_hcp_heart(&q);
    let (pkg, light) = q.split_zero_copy();
    (pkg, light.into_envelope())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> QuantumInbound {
        QuantumInbound {
            package_id:           "pkg-x".to_string(),
            session_id:           "sess-x".to_string(),
            territory:            "INDUSTRIAL".to_string(),
            epistemic_mass:       0.9,
            evidence_density:     0.8,
            logical_coherence:    0.9,
            causal_integrity:     0.85,
            convergence_strength: 0.88,
            reproducibility:      0.92,
            potentiality_score:   0.9,
            ibe_score:            0.86,
            chain_coherence:      0.9,
            conservation_ok:      true,
            candidate_scores:     vec![0.9, 0.8],
            suggested_verdict:    1,
            primitive_flags:      0xA451,
            raw_bytes:            b"state".to_vec(),
            xy_x: None, xy_y: None, xy_mass: None, pd_binding_digest: None, pd_continuum_activation_digest: None,
            pd_parent_i0: None,
            collapse_refused: None, negative_paths: Vec::new(),
            hcp_input_id:         None,
            hcp_token_gen:        None,
            hcp_token_nonce:      None,
            hcp_directive_bit:    None,
            final_evidence_digest: 0,
            quantum_action_state: 0x1122_3344_5566_7788,
            quantum_action_mask: shadow_contracts::REQUIRED_ACTION_MASK,
            quantum_required_action_mask: shadow_contracts::REQUIRED_ACTION_MASK,
            pd_gcl_process_digest: 0,
            spine_completion_digest: 0,
            scientific_project: None,
        }
    }

    #[test]
    fn split_produces_two_aligned_origins() {
        let q = sample();
        let (pkg, light) = split_and_record_heart(q);
        // PassPackage mban propozimin; LightEnvelope mban transportin.
        assert_eq!(pkg.package_id, "pkg-x");
        assert_eq!(pkg.raw_bytes, b"state");
        assert_eq!(light.session_id, "sess-x");
        assert_eq!(light.territory, "INDUSTRIAL");
        assert_eq!(light.primitive_flags, 0xA451);
        assert_eq!(light.payload, b"state");      // payload == raw_bytes (si Light)
        assert_eq!(light.proof_chain.len(), 2);    // gjurmë jo-bosh → e gjurmueshme
    }

    // ── TEST OBLIGATIV (HAPI C): mbyllja e gapit 8-versionesh ────────────────
    // Kur QuantumInbound mbart një vendim REAL të HCP_PRO (të 4 fushat Some),
    // split() duhet ta regjistrojë si heart_byte te ledger-i ekzistues. Kjo
    // vërteton semantikën e adapterit test; rruga production testohet te ingest_bridged, jo
    // vetëm nga testet e veta — pikërisht lidhja që mungonte 8 versione.
    #[test]
    fn split_fires_heart_byte_when_hcp_present() {
        use crate::ess_mai_heart_byte::EssMaiHeartByte;

        // input_id UNIK (s'përdoret asgjëkund tjetër te shadow/src) → vetëm kjo
        // thirrje mund ta fusë te ledger; kausaliteti i izoluar plotësisht.
        const UNIQUE_ID: u64 = 0x00C0_FFEE;

        let mut q = sample();
        q.hcp_input_id      = Some(UNIQUE_ID);
        q.hcp_token_gen     = Some(7);                 // sikur nga gate.issue (Quantum)
        q.hcp_token_nonce   = Some(0xABCD_0001);
        q.hcp_directive_bit = Some(1);                 // push deeper

        // Para: ky id s'ekziston te ledger (unik → garantuar absent në fillim).
        assert!(!EssMaiHeartByte::verify_beat(UNIQUE_ID));
        let before = EssMaiHeartByte::total_beats();

        let (_pkg, _light) = split_and_record_heart(q);                 // TEST ADAPTER → beat_from_inbound

        let after = EssMaiHeartByte::total_beats();
        // total_beats u rrit (vendimi HCP arriti te ledger përmes split).
        assert!(after > before, "adapteri test s'prodhoi heart_byte për HCP të pranishëm");
        // Dhe pikërisht ID-ja jonë u regjistrua → split() e krijoi (kausalitet i ngushtë).
        assert!(EssMaiHeartByte::verify_beat(UNIQUE_ID));
    }
}
