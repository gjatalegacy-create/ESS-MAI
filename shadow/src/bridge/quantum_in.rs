// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  BRIDGE/QUANTUM_IN.RS — URA Quantum → Shadow (FAZA 3)                 ║
// ║  Gjata Legacy™ | Bledar Gjata                                        ║
// ║                                                                          ║
// ║  Pasqyron `ShadowPassPackage` të Quantum-it dhe e adapton në           ║
// ║  `PassPackage` të brendshëm — fushat PROPOZUESE me ZHVENDOSJE (Ligji 0).║
// ║  session/territory/vula NUK hyjnë në PassPackage: ato mbërrijnë via    ║
// ║  Light (origjina e dytë). `derive_light()` riprodhon transportin Light.║
// ╚══════════════════════════════════════════════════════════════════════════╝

use super::clamp01;
use super::light_in::LightInbound;
use crate::types::{PassPackage, ScientificProjectContext};

/// Pasqyrë e `ShadowPassPackage` (output-i i urës Quantum → Shadow).
/// Shadow s'varet nga crate-i i Quantum-it; kjo është kontrata si tip kufitar.
#[derive(Debug, Clone)]
pub struct QuantumInbound {
    pub package_id:           String,
    pub session_id:           String,
    pub territory:            String,

    pub epistemic_mass:       f32,

    // 5 dimensionet e potencialitetit (PIM)
    pub evidence_density:     f32,
    pub logical_coherence:    f32,
    pub causal_integrity:     f32,
    pub convergence_strength: f32,
    pub reproducibility:      f32,

    pub potentiality_score:   f32,
    pub ibe_score:            f32,
    pub chain_coherence:      f32,
    pub conservation_ok:      bool,

    pub candidate_scores:     Vec<f32>,
    pub suggested_verdict:    u8,    // ∈ {0,1,2}
    pub primitive_flags:      u32,   // vula 500 e mbartur
    pub raw_bytes:            Vec<u8>,

    // ── HCP heart_byte mbartja nëpër wire (mbyllja e gapit 8-versionesh) ──────
    // DEFEKTI: më parë QuantumInbound s'kishte asnjë fushë për të mbartur vendimin
    // e HCP_PRO te Shadow → beat_from_inbound thirrej VETËM në testet e veta.
    // Tani vendimi (input_id, gen, nonce, directive) mbartet eksplicit. Të 4
    // janë Option: Some(...) vetëm kur HCP_PRO mori vendim real; None ndryshe.
    // ingest_bridged regjistron beat_from_inbound kur të 4 janë Some; main.rs është ura.
    // ── KOLAPSI (formula XiYi→XY e mbartur nëpër wire) ──────────────────────
    // Të gjitha Option: Some(...) vetëm kur Quantum kolapsoi realisht;
    // None → sjellja e vjetër e paprekur (backward-safe, si modeli HCP).
    pub xy_x:                 Option<String>,
    pub xy_y:                 Option<String>,
    pub xy_mass:              Option<u32>,
    /// Digest-i i kandidatit PD që duhet të mbetet i njëjtë deri te Nura.
    pub pd_binding_digest:      Option<u64>,
    /// Digest-i i aktivizimit kushtetues i + U → i₀ + 1Q.
    pub pd_continuum_activation_digest: Option<u64>,
    /// Parent i₀ i kandidatit PD; ndahet nga session_id për ciklet pasuese.
    pub pd_parent_i0:           Option<String>,
    /// Some(arsyeja) kur kolapsi u REFUZUA ndershëm (XiYi=∅) → kontribut D=0.
    pub collapse_refused:     Option<String>,
    /// Rrugët negative (-Xi/-Yi) në wire "term~space~reason" — ASET për NKR.
    pub negative_paths:       Vec<String>,

    pub hcp_input_id:         Option<u64>,
    pub hcp_token_gen:        Option<u64>,
    pub hcp_token_nonce:      Option<u64>,
    pub hcp_directive_bit:    Option<u8>,

    /// Paketimi final PIM/NPIM/MPRO i verifikuar në kufirin Shadow main.rs.
    pub final_evidence_digest: u64,
    pub quantum_action_state: u64,
    /// Maska e organeve që derdhën evidencë reale në action_state.
    pub quantum_action_mask: u64,
    /// Maska kushtetuese që duhet të jetë e plotë para verdict-it.
    pub quantum_required_action_mask: u64,
    /// Procesi GCL që qeverisi PD→Spine9→Layer1→2→3.
    pub pd_gcl_process_digest: u64,
    pub spine_completion_digest: u64,
    pub scientific_project: Option<ScientificProjectContext>,
}

impl QuantumInbound {
    /// Adapton fushat PROPOZUESE → `PassPackage` me ZHVENDOSJE.
    /// Skorët normalizohen në [0,1]; `suggested_verdict` ngujohet në {0,1,2}.
    pub fn into_pass_package(self) -> PassPackage {
        PassPackage {
            package_id:           self.package_id,
            epistemic_mass:       clamp01(self.epistemic_mass),
            evidence_density:     clamp01(self.evidence_density),
            logical_coherence:    clamp01(self.logical_coherence),
            causal_integrity:     clamp01(self.causal_integrity),
            convergence_strength: clamp01(self.convergence_strength),
            reproducibility:      clamp01(self.reproducibility),
            potentiality_score:   clamp01(self.potentiality_score),
            ibe_score:            clamp01(self.ibe_score),
            chain_coherence:      clamp01(self.chain_coherence),
            conservation_ok:      self.conservation_ok,
            candidate_scores:     self.candidate_scores,
            suggested_verdict:    self.suggested_verdict.min(2),
            final_evidence_digest: self.final_evidence_digest,
            quantum_action_state: self.quantum_action_state,
            quantum_action_mask: self.quantum_action_mask,
            quantum_required_action_mask: self.quantum_required_action_mask,
            pd_gcl_process_digest: self.pd_gcl_process_digest,
            spine_completion_digest: self.spine_completion_digest,
            scientific_project: self.scientific_project,
            raw_bytes:            self.raw_bytes,
        }
    }

    /// Riprodhon transportin Light (si `to_light_shadow_envelope` i Quantum-it):
    /// derive_light — modelon KANALIN E DYTË (Light) që mbart provën te Shadow.
    ///
    /// SQARIM #10 (ndershmëri arkitekturore): ky funksion KLONON qëllimisht
    /// session_id/territory/raw_bytes sepse modelon një kanal TË DYTË të pavarur
    /// (Light transport), jo bashkimin kryesor. Ligji ZERO-COPY vlen për rrugën
    /// kryesore: split() + ShadowPassage::new() përdorin MOVE (zhvendosje, pa klon).
    /// Ky kanal sekondar ka kosto klonimi TË VETËDIJSHME (audit/transport i ndarë)
    /// — projektim i qëllimshëm, JO placeholder. Për volum shumë të lartë ekziston
    /// opsioni i migrimit te Arc<[u8]> për raw_bytes (optimizim opsional, jo i domosdoshëm).
    /// `proof_chain[i] = (score.to_bits() << 8) | (i & 0xFF)`.
    ///
    /// PËRDORIM: vetëm kur ke nevojë për kanalin Light VEÇMAS dhe `q` mbetet gjallë.
    /// Për rrugën kryesore split(), përdor `split_zero_copy()` (zero klon — LIGJI 0).
    pub fn derive_light(&self) -> LightInbound {
        let proof_chain: Vec<u64> = self
            .candidate_scores
            .iter()
            .enumerate()
            .map(|(i, s)| ((s.to_bits() as u64) << 8) | (i as u64 & 0xFF))
            .collect();

        LightInbound {
            session_id:      self.session_id.clone(),
            territory:       self.territory.clone(),
            primitive_flags: self.primitive_flags,
            epistemic_mass:  self.epistemic_mass,
            proof_chain,
            payload:         self.raw_bytes.clone(),
        }
    }

    /// split_zero_copy — RRUGA KRYESORE (      ): ndan `self` në (PassPackage,
    /// LightInbound) me ZHVENDOSJE TOTALE, ZERO KLON. Konsumon `self` një herë.
    ///
    /// VENDIM ARKITEKTUROR (verifikuar në kod): `LightEnvelope.payload` NUK lexohet
    /// KURRË në pipeline-in e Shadow (vetëm `PassPackage.raw_bytes` zhvendoset te
    /// judge_supreme). Prandaj `raw_bytes` shkon TE PassPackage (ku përdoret), dhe
    /// payload-i i Light merr `Vec::new()` — asnjë dyfishim, asnjë klon. Kjo zëvendëson
    /// derive_light().clone() te split(): session_id/territory/raw_bytes ZHVENDOSEN.
    /// Zero if — destrukturim + ndërtim direkt.
    pub fn split_zero_copy(self) -> (PassPackage, LightInbound) {
        // proof_chain ndërtohet PARA zhvendosjes (lexon candidate_scores me referencë).
        let proof_chain: Vec<u64> = self
            .candidate_scores
            .iter()
            .enumerate()
            .map(|(i, s)| ((s.to_bits() as u64) << 8) | (i as u64 & 0xFF))
            .collect();

        // Destrukturim i plotë — çdo fushë zhvendoset një herë, pa klon.
        let QuantumInbound {
            package_id, session_id, territory, epistemic_mass,
            evidence_density, logical_coherence, causal_integrity,
            convergence_strength, reproducibility, potentiality_score,
            ibe_score, chain_coherence, conservation_ok,
            candidate_scores, suggested_verdict, primitive_flags, raw_bytes,
            // HCP: konsumohen te split() (beat PARA split_zero_copy), këtu shpërfillen.
            hcp_input_id: _, hcp_token_gen: _, hcp_token_nonce: _, hcp_directive_bit: _,
            // KOLAPSI: konsumohen te ingest_bridged (verifikimi XY↔XiYi + NKR),
            // këtu (split i pastër transporti) shpërfillen eksplicit.
            xy_x: _, xy_y: _, xy_mass: _, pd_binding_digest: _, pd_continuum_activation_digest: _, pd_parent_i0: _,
            collapse_refused: _, negative_paths: _,
            final_evidence_digest, quantum_action_state, quantum_action_mask,
            quantum_required_action_mask, pd_gcl_process_digest, spine_completion_digest,
            scientific_project,
        } = self;

        // PassPackage merr fushat propozuese + raw_bytes (ku lexohet realisht). MOVE.
        let pkg = PassPackage {
            package_id,
            epistemic_mass:       clamp01(epistemic_mass),
            evidence_density:     clamp01(evidence_density),
            logical_coherence:    clamp01(logical_coherence),
            causal_integrity:     clamp01(causal_integrity),
            convergence_strength: clamp01(convergence_strength),
            reproducibility:      clamp01(reproducibility),
            potentiality_score:   clamp01(potentiality_score),
            ibe_score:            clamp01(ibe_score),
            chain_coherence:      clamp01(chain_coherence),
            conservation_ok,
            candidate_scores,
            suggested_verdict:    suggested_verdict.min(2),
            final_evidence_digest,
            quantum_action_state,
            quantum_action_mask,
            quantum_required_action_mask,
            pd_gcl_process_digest,
            spine_completion_digest,
            scientific_project,
            raw_bytes,  // MOVE — pa klon (shkon te rruga që e lexon)
        };

        // LightInbound merr session/territory/proof_chain me MOVE. payload bosh:
        // s'lexohet kurrë në pipeline → asnjë klon i raw_bytes. Zero-copy real.
        let light = LightInbound {
            session_id,       // MOVE
            territory,        // MOVE
            primitive_flags,
            epistemic_mass,
            proof_chain,
            payload:          Vec::new(),  // s'përdoret në Shadow → pa klon raw_bytes
        };

        (pkg, light)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> QuantumInbound {
        QuantumInbound {
            package_id:           "pkg-7".to_string(),
            session_id:           "sess-7".to_string(),
            territory:            "INDUSTRIAL".to_string(),
            epistemic_mass:       0.92,
            evidence_density:     0.80,
            logical_coherence:    0.90,
            causal_integrity:     0.85,
            convergence_strength: 0.88,
            reproducibility:      0.95,
            potentiality_score:   0.91,
            ibe_score:            0.87,
            chain_coherence:      0.93,
            conservation_ok:      true,
            candidate_scores:     vec![0.9, 0.8, 0.7],
            suggested_verdict:    1,
            primitive_flags:      0xA451,
            raw_bytes:            b"quantum-state".to_vec(),
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
    fn into_pass_package_maps_propose_fields() {
        let q = sample();
        let p = q.into_pass_package();
        assert_eq!(p.package_id, "pkg-7");
        assert!((p.epistemic_mass - 0.92).abs() < 1e-6);
        assert_eq!(p.candidate_scores, vec![0.9, 0.8, 0.7]);
        assert_eq!(p.suggested_verdict, 1);
        assert_eq!(p.raw_bytes, b"quantum-state");
    }

    #[test]
    fn out_of_range_scores_are_clamped() {
        let mut q = sample();
        q.epistemic_mass = 1.7;
        q.ibe_score = -0.3;
        q.suggested_verdict = 9; // jashtë {0,1,2}
        let p = q.into_pass_package();
        assert!((p.epistemic_mass - 1.0).abs() < 1e-6);
        assert!((p.ibe_score - 0.0).abs() < 1e-6);
        assert_eq!(p.suggested_verdict, 2); // ngujuar
    }

    #[test]
    fn derive_light_builds_proof_chain_and_carries_seal() {
        let q = sample();
        let l = q.derive_light();
        assert_eq!(l.session_id, "sess-7");
        assert_eq!(l.territory, "INDUSTRIAL");
        assert_eq!(l.primitive_flags, 0xA451);
        assert!(l.seal_ok());
        assert_eq!(l.proof_chain.len(), 3); // një hash për çdo candidate
        assert_eq!(l.payload, b"quantum-state");
        // proof_chain jo-bosh → envelope i gjurmueshëm te ligji i lineage-it.
        let expected0 = ((0.9f32.to_bits() as u64) << 8) | 0;
        assert_eq!(l.proof_chain[0], expected0);
    }

    #[test]
    fn split_zero_copy_moves_raw_bytes_to_pkg_and_leaves_light_payload_empty() {
        // ZERO-COPY REAL (      ): raw_bytes ZHVENDOSET te PassPackage (ku lexohet),
        // payload-i i Light mbetet bosh (s'lexohet kurrë → asnjë klon).
        let q = sample();
        let (pkg, light) = q.split_zero_copy();
        // raw_bytes te pkg (rruga që e përdor judge_supreme).
        assert_eq!(pkg.raw_bytes, b"quantum-state");
        assert_eq!(pkg.package_id, "pkg-7");
        assert_eq!(pkg.suggested_verdict, 1);
        // session/territory zhvendosur te Light.
        assert_eq!(light.session_id, "sess-7");
        assert_eq!(light.territory, "INDUSTRIAL");
        assert_eq!(light.primitive_flags, 0xA451);
        assert!(light.seal_ok());
        // proof_chain i ndërtuar para zhvendosjes.
        assert_eq!(light.proof_chain.len(), 3);
        // payload-i i Light BOSH — zero klon i raw_bytes (vendim arkitektonik).
        assert!(light.payload.is_empty());
    }
}
