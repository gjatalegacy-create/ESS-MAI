// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  BRIDGE_SHADOW/MOD.RS — Ura Quantum → Shadow (PERSOSMËRI)             ║
// ║  GJATA LEGACY™ — Quantum Platform                                     ║
// ║                                                                          ║
// ║  Lidhja me Shadow . Kontratat kufitare janë IDENTIKE me ato që     ║
// ║  Shadow pret te bridge/quantum_in.rs + bridge/light_in.rs:            ║
// ║   • QuantumInbound  — paketa propozuese (PIM/SRK/PRO/LIM)             ║
// ║   • LightInbound    — transporti + vula 500 (kanali i dytë)          ║
// ║   • SupremeOutcome  — përgjigja (PrimitiveKnowledge / ...)            ║
// ║                                                                          ║
// ║  DY ORIGJINA TË NDARA (si Shadow): propozimi dhe transporti+vula      ║
// ║  bashkohen VETËM brenda Shadow gjatë ingest. Quantum NUK vendos.       ║
// ║                                                                          ║
// ║  API kryesore e Shadow: Shadow::ingest_quantum(QuantumInbound)        ║
// ║  ose receive_from_light(QuantumInbound, LightInbound).                ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::pim::{PassPackage as PimPass, SuggestedVerdict};
use crate::srk_types::ReasoningPackage;
use crate::lim_types::LinearInfoMetric;
use crate::pro_types::ProResult;

// ─────────────────────────────────────────────────────────────────────────────
// VULA 500 — identike me Shadow (bridge/light_in.rs::seal_ok)
//   ((primitive_flags & 0xFFFF) ^ 0xA5A5) == 500
// ─────────────────────────────────────────────────────────────────────────────

const SEAL_XOR:       u32 = 0xA5A5;
const SEAL_MASK:      u32 = 0x0000_FFFF;
const SEAL_PRIMITIVE: u32 = 500;
/// Vula e maskuar siç udhëton: (500 & 0xFFFF) ^ 0xA5A5 = 0xA451.
pub const SEAL_PRIMITIVE_MASKED: u32 = (SEAL_PRIMITIVE & SEAL_MASK) ^ SEAL_XOR;
pub const SEAL_STRIPPED: u32 = 0x0000;

// ═════════════════════════════════════════════════════════════════════════════
// §1  QuantumInbound — IDENTIK me Shadow bridge/quantum_in.rs::QuantumInbound
// ═════════════════════════════════════════════════════════════════════════════

/// Paketa propozuese që Quantum i dërgon Shadow-it.
/// Fushat janë byte-for-byte ato që Shadow pret (verifikuar nga quantum_in.rs).
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
    pub primitive_flags:      u32,   // vula 500
    pub raw_bytes:            Vec<u8>,

    // Kontrata e kolapsit/PD — IDENTIKE me Shadow bridge::QuantumInbound.
    pub xy_x:                 Option<String>,
    pub xy_y:                 Option<String>,
    pub xy_mass:              Option<u32>,
    pub pd_binding_digest:    Option<u64>,
    pub pd_continuum_activation_digest: Option<u64>,
    pub pd_parent_i0:         Option<String>,
    pub collapse_refused:     Option<String>,
    pub negative_paths:       Vec<String>,

    pub hcp_input_id:         Option<u64>,
    pub hcp_token_gen:        Option<u64>,
    pub hcp_token_nonce:      Option<u64>,
    pub hcp_directive_bit:    Option<u8>,
}

impl QuantumInbound {
    /// A e mban vulën e primitivit?
    pub fn is_sealed(&self) -> bool {
        self.primitive_flags == SEAL_PRIMITIVE_MASKED
    }

    /// RUNTIME HARDENING (      , pa material GPT — shkruar direkt): fail-closed gate.
    /// Para se Quantum→Shadow, paketa duhet të ketë identitet + sesion + payload real.
    /// Asnjë silent success për paketë bosh/gjymtuar. Zero if — match i ndërthurur.
    pub fn is_valid(&self) -> bool {
        match self.package_id.is_empty() {
            true  => false,
            false => match self.session_id.is_empty() {
                true  => false,
                false => match self.raw_bytes.is_empty() {
                    true  => false,
                    false => self.territory.is_empty() == false,
                },
            },
        }
    }

    /// reason_invalid — kthen arsyen specifike të refuzimit (reason code). Zero if.
    pub fn reason_invalid(&self) -> &'static str {
        match self.package_id.is_empty() {
            true  => "empty_package_id",
            false => match self.session_id.is_empty() {
                true  => "empty_session_id",
                false => match self.raw_bytes.is_empty() {
                    true  => "empty_raw_bytes",
                    false => match self.territory.is_empty() {
                        true  => "empty_territory",
                        false => "valid",
                    },
                },
            },
        }
    }

    /// Riprodhon transportin Light (IDENTIK me Shadow quantum_in::derive_light):
    /// proof_chain[i] = (score.to_bits() << 8) | (i & 0xFF).
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
}

// ═════════════════════════════════════════════════════════════════════════════
// §2  LightInbound — IDENTIK me Shadow bridge/light_in.rs::LightInbound
// ═════════════════════════════════════════════════════════════════════════════

/// Transporti + vula që mbërrin te Shadow përmes kanalit Light (origjina e dytë).
#[derive(Debug, Clone)]
pub struct LightInbound {
    pub session_id:      String,
    pub territory:       String,
    pub primitive_flags: u32,
    pub epistemic_mass:  f32,
    pub proof_chain:     Vec<u64>,
    pub payload:         Vec<u8>,
}

impl LightInbound {
    /// IDENTIK me Shadow light_in::seal_ok.
    pub fn seal_ok(&self) -> bool {
        ((self.primitive_flags & SEAL_MASK) ^ SEAL_XOR) == SEAL_PRIMITIVE
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// §3  SupremeOutcome — IDENTIK me Shadow bridge/shadow_out.rs::SupremeOutcome
// ═════════════════════════════════════════════════════════════════════════════

/// Përgjigja e Shadow-it (vendimi suprem). Mirror i kontratës së Shadow.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SupremeOutcome {
    PrimitiveKnowledge,      // 1 — vula 500 mbijetoi
    VerifiableNotPrimitive,  // 0 — verifikuar por jo primitiv
    ChainRejected,           // 0 — zinxhiri ra
}

impl SupremeOutcome {
    /// bit() — IDENTIK me Shadow: 1 vetëm për PrimitiveKnowledge.
    pub fn bit(self) -> u8 {
        (self == SupremeOutcome::PrimitiveKnowledge) as u8
    }

    pub fn label(self) -> &'static str {
        match self {
            SupremeOutcome::PrimitiveKnowledge     => "1:PRIMITIVE_KNOWLEDGE",
            SupremeOutcome::VerifiableNotPrimitive => "0:VERIFIABLE_NOT_PRIMITIVE",
            SupremeOutcome::ChainRejected          => "0:CHAIN_REJECTED",
        }
    }
}

/// Përgjigja e plotë (mirror i ShadowLightResponse).
#[derive(Debug, Clone)]
pub struct ShadowLightResponse {
    pub session_id: String,
    pub verdict:    SupremeOutcome,
    pub shadow_note: String,
    pub legacy_score: f32,
    pub y_verdict: u8,
    pub x_verdict: u8,
    pub verification_seal: u64,
    pub verification_generation: u64,
    pub verification_receipt_id: String,
    pub verification_parent_i0: String,
    pub verification_primitive_anchor: u64,
    pub verification_xy_digest: u64,
    pub verification_pd_binding_digest: u64,
    pub verification_pd_continuum_activation_digest: u64,
    pub verification_living_trust_digest: u64,
    pub reason_code: u32,
    pub failure_stage: &'static str,
    pub source_module: &'static str,
}

// ═════════════════════════════════════════════════════════════════════════════
// §4  URA — ndërton QuantumInbound nga output-i i plotë i Quantum
// ═════════════════════════════════════════════════════════════════════════════

pub struct QuantumShadowBridge;

impl QuantumShadowBridge {
    /// Mbledh nga të katër fazat → QuantumInbound (kontrata e Shadow).
    ///   LIM → epistemic_mass
    ///   PRO → candidate_scores
    ///   SRK → ibe_score, conservation_ok, chain_coherence
    ///   PIM → 5 dimensionet, potentiality_score, suggested_verdict
    ///
    /// `sealed`: a mbeti inputi primitiv (vula 500) — vjen nga EvolveTrace.
    pub fn build(
        lim:        &LinearInfoMetric,
        pro:        &ProResult,
        srk:        &ReasoningPackage,
        pim:        &PimPass,
        session_id: &str,
        territory:  &str,
        sealed:     bool,
    ) -> QuantumInbound {
        // Vula 500: sealed → 0xA451, ndryshe → 0x0000 (branchless).
        let is_sealed = sealed as u32;
        let primitive_flags = is_sealed * SEAL_PRIMITIVE_MASKED
            + (1 - is_sealed) * SEAL_STRIPPED;

        // suggested_verdict: PIM → u8 ∈ {0,1,2}.
        let suggested_verdict = Self::map_verdict(&pim.shadow_suggestion.suggested_verdict);

        // ibe_score nga SRK best explanation.
        let ibe_score = srk.best_explanation.as_ref()
            .map(|e| e.ibe_score).unwrap_or(0.0);

        // conservation_ok: asnjë check violated.
        let conservation_ok = srk.conservation_checks.iter().all(|c| !c.violated);

        // candidate_scores nga PRO.
        let candidate_scores: Vec<f32> = pro.candidates.iter()
            .map(|c| c.score).collect();

        // raw_bytes: serializim minimal për KnowledgeVault.
        let raw_bytes = Self::serialize_minimal(lim, pim);

        QuantumInbound {
            package_id: pim.package_id.clone(),
            session_id: session_id.to_string(),
            territory: territory.to_string(),
            epistemic_mass: lim.epistemic_mass,
            evidence_density:     pim.potentiality.evidence_density,
            logical_coherence:    pim.potentiality.logical_coherence,
            causal_integrity:     pim.potentiality.causal_integrity,
            convergence_strength: pim.potentiality.convergence_strength,
            reproducibility:      pim.potentiality.reproducibility,
            potentiality_score:   pim.potentiality.potentiality_score,
            ibe_score,
            chain_coherence:      pim.potentiality.logical_coherence,
            conservation_ok,
            candidate_scores,
            suggested_verdict,
            primitive_flags,
            raw_bytes,
            xy_x: None,
            xy_y: None,
            xy_mass: None,
            pd_binding_digest: None,
            pd_continuum_activation_digest: None,
            pd_parent_i0: None,
            collapse_refused: None,
            negative_paths: Vec::new(),
            hcp_input_id: None,
            hcp_token_gen: None,
            hcp_token_nonce: None,
            hcp_directive_bit: None,
        }
    }

    /// PIM SuggestedVerdict → u8 (0=negative 1=hypothesis 2=fact).
    /// Branchless: tabelë indeksimi.
    fn map_verdict(v: &SuggestedVerdict) -> u8 {
        // EXPLORE/HOLD → 1 (hypothesis), CONSIDER_OPTIMAL → 2 (fact).
        const TABLE: [u8; 3] = [1, 1, 2];
        let idx = match v {
            SuggestedVerdict::ExploreFurther  => 0,
            SuggestedVerdict::HoldForTesting  => 1,
            SuggestedVerdict::ConsiderOptimal => 2,
        };
        TABLE[idx]
    }

    fn serialize_minimal(lim: &LinearInfoMetric, pim: &PimPass) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&lim.epistemic_mass.to_le_bytes());
        bytes.extend_from_slice(&pim.potentiality.potentiality_score.to_le_bytes());
        bytes.extend_from_slice(pim.package_id.as_bytes());
        bytes
    }

    /// Ndan paketën në (propozim, transport) — IDENTIK me Shadow bridge::split.
    /// Quantum prodhon të dyja origjinat; Shadow i bashkon brenda.
    pub fn split(q: &QuantumInbound) -> (QuantumInbound, LightInbound) {
        let light = q.derive_light();
        (q.clone(), light)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// PREVIEW SEAL — kontroll lokal (e vërteta vendoset te Shadow)
// ─────────────────────────────────────────────────────────────────────────────

/// (flags & 0xFFFF) ^ 0xA5A5 == 500 → primitiv. Vetëm parashikim.
pub fn preview_seal(flags: u32) -> bool {
    let cleared = (flags != 0) as u32;
    let value = (flags & SEAL_MASK) ^ SEAL_XOR;
    let matches = (value == SEAL_PRIMITIVE) as u32;
    (cleared * matches) == 1
}

// ═════════════════════════════════════════════════════════════════════════════
// §5  URA NPIM → SHADOW — Negative Knowledge (IDENTIK me NegativeContext)
// ═════════════════════════════════════════════════════════════════════════════

/// Konteksti negativ që mbërrin te Shadow — IDENTIK me Shadow types.rs::NegativeContext:
///   { failure_reason_code: u32, process: String, mass_at_fail: f32, frequency: u32 }
#[derive(Debug, Clone)]
pub struct NegativeInbound {
    pub failure_reason_code: u32,
    pub process:             String,
    pub mass_at_fail:        f32,
    pub frequency:           u32,
    pub arguments_blob:      Vec<u8>,   // argumentat e paketuara (raw)
    pub suggestion_code:     u8,        // 0=weak 1=hold 2=confirm
}

impl NegativeInbound {
    /// Parametrat për on_negative(input_id, reason, process, mass, frequency, bytes, ts).
    pub fn as_on_negative_args(&self) -> (u32, &str, f32, u32, &[u8]) {
        (self.failure_reason_code, &self.process, self.mass_at_fail,
         self.frequency, &self.arguments_blob)
    }
}

pub struct NpimShadowBridge;

impl NpimShadowBridge {
    /// build — nga NegativeKnowledgePackage (NPIM) → NegativeInbound (Shadow).
    pub fn build(
        npim: &crate::npim::NegativeKnowledgePackage,
        territory: &str,
    ) -> NegativeInbound {
        use crate::npim::NegativeSuggestion;

        // failure_reason_code: koduar nga negativiteti (branchless).
        // Përdor elimination_certainty * 1000 si kod (deterministik).
        let failure_reason_code = (npim.negativity.elimination_certainty * 1000.0) as u32;

        // process: territory + origjina.
        let process = format!("{}:{}", territory, npim.origin);

        // mass_at_fail: negativity_score si "masa" e dështimit.
        let mass_at_fail = npim.negativity.negativity_score;

        // frequency: numri i argumentave (sa eliminime u gjetën).
        let frequency = npim.arguments.len() as u32;

        // arguments_blob: serializim minimal i argumentave.
        let arguments_blob = Self::serialize_arguments(npim);

        // suggestion_code: 0/1/2 (branchless tabelë via match).
        let suggestion_code = match npim.suggestion {
            NegativeSuggestion::WeakElimination => 0,
            NegativeSuggestion::HoldNegative    => 1,
            NegativeSuggestion::ConfirmNegative => 2,
        };

        NegativeInbound {
            failure_reason_code,
            process,
            mass_at_fail,
            frequency,
            arguments_blob,
            suggestion_code,
        }
    }

    fn serialize_arguments(npim: &crate::npim::NegativeKnowledgePackage) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&npim.negativity.negativity_score.to_le_bytes());
        bytes.extend_from_slice(&npim.negativity.failure_density.to_le_bytes());
        bytes.extend_from_slice(&npim.negativity.contradiction_strength.to_le_bytes());
        for arg in &npim.arguments {
            bytes.extend_from_slice(arg.operator.as_bytes());
            bytes.push(b'|');
            bytes.extend_from_slice(arg.cause.as_bytes());
            bytes.push(b'\n');
        }
        bytes
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// HCP DECISION BRIDGE — vendimet e HCP_PRO → Shadow heart_byte (RREGULLIM     )
// ═════════════════════════════════════════════════════════════════════════════
//
// DEFEKTI I RREGULLUAR: para këtij rregullimi, HCP_PRO (Quantum) mint+burn-onte
// token, dhe heart_byte (Shadow) regjistronte te ledger — POR të dy anët s'ishin
// të lidhura. HcpDecisionRecord thirrej vetëm në teste me vlera hardcoded.
//
// Ky bridge i lidh: vendimi i HCP_PRO → HcpDecisionInbound (i tipizuar, sipas
// shabllonit NegativeInbound) → kalon te Shadow → HcpDecisionRecord → heart_byte.

/// Vendimi i HCP_PRO që mbërrin te Shadow — i TIPIZUAR (jo String i papërpunuar).
/// Pasqyron EKZAKT HcpDecisionRecord te Shadow: {input_id, token_gen, token_nonce,
/// directive_bit}. Ndjek shabllonin e NegativeInbound (tipizim i fortë).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HcpDecisionInbound {
    /// Identifikuesi i inputit/ciklit ku u mor vendimi.
    pub input_id:      u64,
    /// gen i token-it të HCP_PRO (nga SovereignGate::issue → handle.gen).
    pub token_gen:     u64,
    /// nonce/seal i token-it (module_seal i HCP_PRO).
    pub token_nonce:   u64,
    /// Drejtimi i vendimit (1=push deeper, 0=pull back).
    pub directive_bit: u8,
}

impl HcpDecisionInbound {
    /// new — krijon mesazhin nga vendimi i HCP_PRO. Zero if.
    pub fn new(input_id: u64, token_gen: u64, token_nonce: u64, directive_bit: u8) -> Self {
        Self { input_id, token_gen, token_nonce, directive_bit }
    }

    /// as_record_args — parametrat për HcpDecisionRecord::new te Shadow. Zero if.
    /// (input_id, token_gen, token_nonce, directive_bit)
    pub fn as_record_args(&self) -> (u64, u64, u64, u8) {
        (self.input_id, self.token_gen, self.token_nonce, self.directive_bit)
    }
}

/// Ura HCP_PRO → Shadow: ndërton mesazhin e tipizuar nga vendimi i orkestrimit.
pub struct HcpShadowBridge;

impl HcpShadowBridge {
    /// from_parallel — nga ParallelActivation (HCP_PRO L3) → HcpDecisionInbound.
    ///
    /// Thirret PAS burn-it të suksesshëm të token-it. directive_bit nxirret nga
    /// urdhri i harduerit (PushDeeper=1, ndryshe 0). Zero if — match.
    pub fn from_parallel(
        input_id: u64,
        activation: &crate::layer3::hcp_pro_l3::ParallelActivation,
        token_nonce: u64,
    ) -> HcpDecisionInbound {
        use crate::hcp_pro::HardwareDirective;
        // directive_bit: PushDeeper → 1, Hold/PullBack → 0. Zero if — match.
        let directive_bit = match activation.hw_directive {
            HardwareDirective::PushDeeper => 1u8,
            HardwareDirective::Hold       => 0u8,
            HardwareDirective::PullBack    => 0u8,
        };
        HcpDecisionInbound::new(input_id, activation.token_gen, token_nonce, directive_bit)
    }

    /// from_l2_decision — nga PressureAdjustedDecision (HCP_PRO L2) → HcpDecisionInbound.
    ///
    /// directive_bit nxirret nga urdhri final (GoDeeper=1, Stop=0). Zero if — match.
    pub fn from_l2_decision(
        input_id: u64,
        decision: &crate::layer2::hcp_pro_l2::PressureAdjustedDecision,
        token_nonce: u64,
    ) -> HcpDecisionInbound {
        use crate::hcp_pro::OperatorOrder;
        let directive_bit = match decision.final_order {
            OperatorOrder::GoDeeper => 1u8,
            OperatorOrder::Stop     => 0u8,
        };
        HcpDecisionInbound::new(input_id, decision.base.token_gen, token_nonce, directive_bit)
    }

    /// from_orchestration — nga OrchestrationDecision (HCP_PRO L1) → HcpDecisionInbound.
    ///
    /// RRUGA REALE E PRODHIMIT (main.rs::run): `HcpPro::orchestrate` prodhon vendimin
    /// e harduerit të vulosur me token (mint+burn → token_gen REAL nga gate). Kjo
    /// është ura që e mbart atë vendim te Shadow heart_byte. directive_bit nxirret
    /// nga urdhri i harduerit (PushDeeper=1, Hold/PullBack=0) — IDENTIK me
    /// from_parallel. Zero if — match.
    pub fn from_orchestration(
        input_id: u64,
        decision: &crate::hcp_pro::OrchestrationDecision,
        token_nonce: u64,
    ) -> HcpDecisionInbound {
        use crate::hcp_pro::HardwareDirective;
        let directive_bit = match decision.hw_directive {
            HardwareDirective::PushDeeper => 1u8,
            HardwareDirective::Hold       => 0u8,
            HardwareDirective::PullBack    => 0u8,
        };
        HcpDecisionInbound::new(input_id, decision.token_gen, token_nonce, directive_bit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lim::LinearInfoMetricEngine;
    use crate::lim_hw_real_bridge::LimHwRealBridge;
    use crate::pro::ProEngine;
    use crate::pro_nk_gate::KnowledgeVault;
    use crate::srk::ScientificReasoningKernel;
    use crate::pim::PassInfoMetric;
    use crate::lim_types::EpistemicState;

    fn full_pipeline(turns: &[&str], sealed: bool) -> QuantumInbound {
        let mut lim = LinearInfoMetricEngine::new("s1", "science");
        for t in turns { lim.collect(t); }
        let scaffold = lim.pre_pro_scan().expect("scaffold");
        let frags: Vec<String> = lim.collector().fragments().to_vec();
        let lim_metric = scaffold.lim.clone();

        let hw = LimHwRealBridge::finalize(scaffold);
        let mut vault = KnowledgeVault::new();
        let pro = ProEngine::activate(&hw, &frags, &mut vault);

        let srk = ScientificReasoningKernel::new("s1", "science");
        let reasoning = srk.process(&pro);

        let state = EpistemicState::PotentialHypothesis {
            confidence_range: (0.36, 0.98), stability: 0.6,
            trl_candidate: 2, pro_mode: "deep".to_string(),
        };
        let pass = PassInfoMetric::measure(&reasoning, &state, "s1");

        QuantumShadowBridge::build(
            &lim_metric, &pro, &reasoning, &pass, "s1", "science", sealed,
        )
    }

    #[test]
    fn inbound_sealed_carries_500() {
        let q = full_pipeline(&[
            "deterministic elimination replaces probabilistic generation in reasoning",
            "knowledge that survives the filters becomes verifiable primitive knowledge",
        ], true);
        assert!(q.is_sealed());
        assert_eq!(q.primitive_flags, 0xA451);
        assert!(preview_seal(q.primitive_flags));
    }

    #[test]
    fn inbound_stripped_when_fragmented() {
        let q = full_pipeline(&[
            "the reactor is stable under load conditions here now",
            "the reactor is not stable when load increases suddenly today",
        ], false);
        assert!(!q.is_sealed());
        assert_eq!(q.primitive_flags, 0x0000);
    }

    #[test]
    fn derive_light_carries_seal_and_proof() {
        let q = full_pipeline(&[
            "quantum reasoning eliminates wrong paths through deterministic filters",
            "the shadow authority verifies each output with a binary decision",
        ], true);
        let light = q.derive_light();
        assert_eq!(light.primitive_flags, q.primitive_flags);
        assert!(light.seal_ok());
        assert_eq!(light.proof_chain.len(), q.candidate_scores.len());
        // proof_chain[i] = (score.to_bits() << 8) | i — IDENTIK me Shadow.
        match q.candidate_scores.is_empty() {
            false => {
                let expected0 = ((q.candidate_scores[0].to_bits() as u64) << 8) | 0;
                assert_eq!(light.proof_chain[0], expected0);
            }
            true => {}
        }
    }

    #[test]
    fn split_gives_two_origins() {
        let q = full_pipeline(&[
            "deterministic elimination produces verifiable knowledge here now",
            "each candidate passes through the negative knowledge filter first",
        ], true);
        let (pkg, light) = QuantumShadowBridge::split(&q);
        assert_eq!(pkg.package_id, q.package_id);
        assert_eq!(light.session_id, q.session_id);
        assert_eq!(light.payload, q.raw_bytes);
    }

    #[test]
    fn seal_constants_match_shadow() {
        assert_eq!(SEAL_PRIMITIVE_MASKED, 0xA451);
        assert!(preview_seal(0xA451));
        assert!(!preview_seal(0x0000));
        assert!(!preview_seal(500)); // pa XOR
    }

    #[test]
    fn suggested_verdict_in_range() {
        let q = full_pipeline(&[
            "quantum platform eliminates wrong reasoning paths deterministically now",
            "surviving knowledge becomes primitive verifiable knowledge over time",
        ], true);
        assert!(q.suggested_verdict <= 2);
    }

    // ── from_orchestration: vendim REAL i HcpPro::orchestrate → bridge i tipizuar ──
    // Vërteton që rruga reale e prodhimit (main.rs::run) prodhon token REAL (nga
    // gate, jo hardcoded) dhe që ura mbart saktë (input_id, token_gen, nonce, bit).
    #[test]
    fn from_orchestration_carries_real_token_and_directive() {
        use crate::hcp_pro::{HcpPro, HardwareDirective};
        use crate::sovereign::lgc_gate::SovereignGate;
        use crate::hw_real::territories::{PressureEnvelope, DepthHint};
        use crate::lim_types::Operator;

        // Vendim REAL i orkestrimit: token mint+burn + urdhër harduerik nga presioni.
        let mut gate = SovereignGate::new();
        let env = PressureEnvelope {
            locked_bytes: 1_000_000, released_bytes: 500_000, total_reserved: 1_500_000,
            demand_bytes: 800_000, under_pressure: false, depth_hint: DepthHint::Deep, ts_ns: 100,
        };
        let signals = [
            (Operator::Reverse,  0.8f32),
            (Operator::Mirror,   0.7f32),
            (Operator::Contrast, 0.6f32),
            (Operator::Trace,    0.9f32),
        ];
        let decision = HcpPro::orchestrate(&mut gate, &env, &signals);
        assert!(decision.sealed); // token u mint+burn me sukses

        let input_id = 0xD00Du64;
        let nonce = 0x4843_5050_524F_0001u64; // S_HCP_PRO
        let inbound = HcpShadowBridge::from_orchestration(input_id, &decision, nonce);

        // token_gen vjen nga gate (REAL), jo literal.
        assert_eq!(inbound.input_id, input_id);
        assert_eq!(inbound.token_gen, decision.token_gen);
        // mapja e directive-it është IDENTIKE me from_parallel (robuste ndaj decide_hardware).
        let expected_bit = match decision.hw_directive {
            HardwareDirective::PushDeeper => 1u8,
            HardwareDirective::Hold       => 0u8,
            HardwareDirective::PullBack    => 0u8,
        };
        assert_eq!(inbound.directive_bit, expected_bit);
        // as_record_args jep saktësisht 4 vlerat për Shadow heart_byte.
        let (id, gen, n, bit) = inbound.as_record_args();
        assert_eq!((id, gen, n, bit), (input_id, decision.token_gen, nonce, expected_bit));
    }
}
