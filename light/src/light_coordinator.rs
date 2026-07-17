// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LIGHT_COORDINATOR.RS — ESS-MAI LIGHT PLATFORM                        ║
// ║  Gjata Legacy™  |  Arkitekt: Bledar Gjata                             ║
// ║                                                                          ║
// ║  Porta e hyrjes dhe daljes. Koordinues i pastër.                      ║
// ║  KURRË nuk vendos. Shadow vendos.                                     ║
// ║                                                                          ║
// ║  FLUKSI:                                                                ║
// ║    receive(input)                                                        ║
// ║      → lingua::parse_input()              [gjuhë → SystemQuery]       ║
// ║      → trace_info::TraceInfo::new()       [stamp fillestar]           ║
// ║      → nura_core::Domain::detect()         [domain routing]           ║
// ║      → software_contract::create()         [kontratë + seal]         ║
// ║      → evolve_trace::EvolveTrace::branch() [dega INPKNG]             ║
// ║      → LightResponse (gati për Quantum)                               ║
// ║    format_output(content, lang)                                         ║
// ║      → lingua::format_output()            [akademik → user]           ║
// ║                                                                          ║
// ║  Roli: COORDINATOR — orchestron, rout-on. Zero logjikë vendimi.      ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::trace_info::{TraceInfo, DomainHint};
use crate::nura_core::{NuraCore, Domain};
use crate::software_contract::SoftwareContract;
use crate::ess_mai_system::dsl;
use crate::lab_contracts::{PrimitiveSplit, GjataCollapseLaw, CollapsePhase};
use crate::lgc_algorithm::{
    compute_sha256, LegacyCoordination, LegacyLgcAlgorithm,
};
use crate::alnur_karina_athar::HandoffEvidence;
use crate::lingua::{LinguaCore, DetectedLang};

// ════════════════════════════════════════════════════════════════════════════
// §1  REQUEST — input nga user ose sistem
// ════════════════════════════════════════════════════════════════════════════

#[derive(Debug)]
pub struct LightRequest {
    /// Teksti i papërpunuar
    pub text:       String,

    /// Bytes raw — për trace_info::new()
    pub raw:        Vec<u8>,

    /// Session opsionale
    pub session_id: Option<String>,
}

impl LightRequest {
    pub fn new(text: &str) -> Self {
        Self {
            raw:        text.as_bytes().to_vec(),
            text:       text.to_string(),
            session_id: None,
        }
    }

    pub fn with_session(mut self, sid: &str) -> Self {
        self.session_id = Some(sid.to_string());
        self
    }
}

// ════════════════════════════════════════════════════════════════════════════
// §2  STATUS — çfarë ka ndodhur
// ════════════════════════════════════════════════════════════════════════════

#[derive(Debug, PartialEq)]
pub enum LightStatus {
    /// Input traçed, contract created, gati për Quantum
    Routed,
    /// Kontrata dështoi — gabim autorizimi ose modul i panjohur
    ContractFailed(String),
    /// EvolveTrace dështoi — suffix gabim ose limit arritur
    EvolveFailed(String),
    /// GCL/LGC/PA/LAW0/receipt dështoi para handoff-it.
    LegacyFailed(String),
}

impl LightStatus {
    pub fn label(&self) -> &str {
        match self {
            Self::Routed           => "ROUTED",
            Self::ContractFailed(_) => "CONTRACT_FAILED",
            Self::EvolveFailed(_)   => "EVOLVE_FAILED",
            Self::LegacyFailed(_)   => "LEGACY_LGC_FAILED",
        }
    }

    pub fn is_ok(&self) -> bool {
        matches!(self, Self::Routed)
    }
}

// ════════════════════════════════════════════════════════════════════════════
// §3  RESPONSE — rezultati i koordinimit
// ════════════════════════════════════════════════════════════════════════════

#[derive(Debug)]
pub struct LightResponse {
    /// T3_a3f4b2c1 — etiketa e së njëjtës gjurmë primitive më poshtë.
    pub trace_id:        String,

    /// Primitive Trace autoritative e krijuar NJË herë në hyrje. Ky objekt
    /// bartet deri te eksporti PA; main nuk lejohet të rikrijojë TraceInfo.
    pub primitive_trace: TraceInfo,

    /// WORK→NURA / HOME→ALISAH / OUTSIDE→AMMY
    pub domain:          String,

    /// KODUNIK seal i kontratës
    pub contract_id:     String,

    /// LGC lightweight seal — dërguar te Quantum për verifikim
    pub lgc_seal:        String,

    /// Gjendja DSL aktuale
    pub algorithm_state: String,

    /// Rezyme evolve_trace: T3_a3f4b2c1→[x1234_inpkng_1]
    pub evolve_summary:  String,

    /// Gjuha e detektuar — për format_output pas Shadow
    pub user_lang:       DetectedLang,

    /// Confidence i detektimit gjuhësor [0,1]
    pub lang_confidence: f32,

    /// KOLAPS KOORDINIMI — (Xi, Yi): dy hapësirat primitive të lindura nga PA.
    /// Formula e gdhendur: i0 → PA → (Xi, Yi). Light koordinon, s'arsyeton.
    pub primitive_split: PrimitiveSplit,

    /// SHA-256 REAL i inputit — lind në Light (evolve_trace), Shadow vetëm vulos.
    pub input_sha256: String,

    /// Autoriteti unik LGC: receipt + EvolveTrace + muskuli evidence-bound.
    pub legacy: Option<LegacyCoordination>,

    /// Statusi
    pub status:          LightStatus,
}

impl LightResponse {
    /// Invarianti kushtetues i Primitive Trace: trace_id që udhëton drejt
    /// Quantum-it duhet të jetë etiketa e të njëjtit objekt që zbret si PA/i₀
    /// në Shadow. Një TraceInfo i rindërtuar do ta bëjë këtë `false`.
    pub fn primitive_continuity_valid(&self) -> bool {
        let trace_ok = self.trace_id == self.primitive_trace.label();
        match &self.legacy {
            Some(legacy) => {
                trace_ok
                    && legacy.receipt.verify_integrity().is_ok()
                    && legacy.receipt.trace_id == self.trace_id
                    && legacy.receipt.trace_origin == self.primitive_trace.id
                    && legacy.receipt.primitive_anchor == self.primitive_trace.id_hex()
                    && legacy.receipt.input_sha256 == self.input_sha256
                    && legacy.primitive_split == self.primitive_split
            }
            None => false,
        }
    }

    pub fn ready_for_handoff(&self) -> Result<(), String> {
        match self.status.is_ok() {
            true => {}
            false => return Err(format!("Light status: {}", self.status.label())),
        }
        match self.primitive_continuity_valid() {
            true => {}
            false => return Err(String::from("primitive continuity invalid")),
        }
        self.legacy
            .as_ref()
            .ok_or_else(|| "legacy coordination missing".to_string())?
            .ready_for_handoff()
            .map_err(|e| e.to_string())
    }

    pub fn complete_legacy_handoff(
        &mut self,
        evidence: &HandoffEvidence,
    ) -> Result<(), String> {
        self.legacy
            .as_mut()
            .ok_or_else(|| "legacy coordination missing".to_string())?
            .complete_handoff(evidence)
            .map_err(|e| e.to_string())
    }

    pub fn ready_for_dispatch(&self) -> Result<(), String> {
        match self.status.is_ok() {
            true => {}
            false => return Err(format!("Light status: {}", self.status.label())),
        }
        match self.primitive_continuity_valid() {
            true => {}
            false => return Err(String::from("primitive continuity invalid")),
        }
        self.legacy
            .as_ref()
            .ok_or_else(|| "legacy coordination missing".to_string())?
            .ready_for_dispatch()
            .map_err(|e| e.to_string())
    }

    pub fn print(&self) {
        println!("╔{:═<68}╗", "");
        println!("║  LIGHT COORDINATOR RESPONSE  {:>37}║", "");
        println!("╠{:═<68}╣", "");
        println!("║  trace_id    : {:<52}║", self.trace_id);
        println!("║  domain      : {:<52}║", self.domain);
        println!("║  contract_id : {:<52}║",
            &self.contract_id[..self.contract_id.len().min(52)]);
        println!("║  alg_state   : {:<52}║", self.algorithm_state);
        println!("║  evolve      : {:<52}║",
            &self.evolve_summary[..self.evolve_summary.len().min(52)]);
        println!("║  lang        : {} (conf:{:.2})  {:>33}║",
            self.user_lang.code(), self.lang_confidence, "");
        println!("║  status      : {:<52}║", self.status.label());
        println!("╚{:═<68}╝", "");
    }
}

// ════════════════════════════════════════════════════════════════════════════
// §4  LIGHT COORDINATOR
// ════════════════════════════════════════════════════════════════════════════

pub struct LightCoordinator {
    /// NURA unified entity (NURA+ALISAH+AMMY)
    pub core:   NuraCore,
    /// Lingua — adaptive language engine
    pub lingua: LinguaCore,
}

impl LightCoordinator {
    pub fn new() -> Self {
        Self {
            core:   NuraCore::new(),
            lingua: LinguaCore::new(),
        }
    }

    /// PORTA KRYESORE — çdo input hyn këtu.
    ///
    /// Hapat e koordinimit (zero vendim):
    ///   0. lingua parse_input → SystemQuery (gjuhë + intent + terms)
    ///   1. stamp → trace_info (FNV mbi raw)
    ///   2. detect domain → nura_core + SystemQuery.domain_hint()
    ///   3. set domain hint → trace_info
    ///   4. create contract → software_contract
    ///   5. init evolve_trace + branch INPKNG
    ///   6. return LightResponse (me user_lang për output)
    pub fn receive(&self, req: LightRequest) -> LightResponse {
        // HAPI 0 — gjuha natyrore hyn vetëm një herë në Lingua.
        let query = self.lingua.parse_input(&req.text);
        let user_lang = query.lang.clone();
        let lang_conf = query.confidence;
        let domain_hint0 = query.domain_hint();

        // HAPI 1 — i₀ stampohet vetëm një herë; domain-i ndryshon vetëm hint-in,
        // jo identitetin, timestamp-in ose sequence-in e Primitive Trace.
        let raw_trace = TraceInfo::new(&req.raw);
        let domain = self.core.route(&query.normalized);
        let domain_hint = match &domain {
            Domain::Work => DomainHint::Work,
            Domain::Home => DomainHint::Home,
            Domain::Outside => DomainHint::Outside,
        };
        let final_hint = match domain_hint == DomainHint::Unknown {
            true => domain_hint0,
            false => domain_hint,
        };
        let trace = raw_trace.with_domain(final_hint);
        let trace_id = trace.label();
        let domain_label = domain.label().to_string();

        // HAPI 2 — GCL është autoriteti; LGC është algoritmi Legacy që zbaton
        // koordinimin e autorizuar. Këtu ndodh thirrja e vetme production për
        // PA → Xi/Yi, LegalGoCrypt V2, KODUNIK dhe receipt-in autoritativ.
        let directive = GjataCollapseLaw::issue(CollapsePhase::Coordination);
        let legacy = match LegacyLgcAlgorithm::coordinate_under_gcl(
            directive,
            &req.raw,
            &trace,
            &query,
        ) {
            Ok(value) => value,
            Err(error) => {
                return LightResponse {
                    trace_id,
                    primitive_trace: trace,
                    domain: domain_label,
                    contract_id: String::from("NO_CONTRACT"),
                    lgc_seal: String::new(),
                    algorithm_state: dsl::AlgorithmState::Failed(
                        error.to_string(),
                    )
                    .label()
                    .to_string(),
                    evolve_summary: String::from("LEGACY_LGC_NOT_ROUTED"),
                    user_lang,
                    lang_confidence: lang_conf,
                    primitive_split: PrimitiveSplit {
                        xi: Vec::new(),
                        yi: Vec::new(),
                    },
                    input_sha256: compute_sha256(&req.raw),
                    legacy: None,
                    status: LightStatus::LegacyFailed(error.to_string()),
                };
            }
        };

        let primitive_split = legacy.primitive_split.clone();
        let input_sha256 = legacy.receipt.input_sha256.clone();
        let evolve_summary = legacy.evolve.summary();
        let algorithm_state = legacy.evolve.state.label().to_string();

        // HAPI 3 — kontrata nuk gjeneron KODUNIK të dytë. Ajo lidhet me të
        // njëjtin KODUNIK dhe receipt që prodhoi algoritmi Legacy nën GCL.
        let contract = match SoftwareContract::create_bound(
            "light_coordinator",
            "quantum_platform",
            "route_input",
            "LIGHT",
            &legacy.receipt,
        ) {
            Ok(value) => value,
            Err(error) => {
                return LightResponse {
                    trace_id,
                    primitive_trace: trace,
                    domain: domain_label,
                    contract_id: String::from("NO_CONTRACT"),
                    lgc_seal: String::new(),
                    algorithm_state: dsl::AlgorithmState::Failed(
                        error.to_string(),
                    )
                    .label()
                    .to_string(),
                    evolve_summary,
                    user_lang,
                    lang_confidence: lang_conf,
                    primitive_split,
                    input_sha256,
                    legacy: Some(legacy),
                    status: LightStatus::ContractFailed(error.to_string()),
                };
            }
        };

        match SoftwareContract::enforce(&contract) {
            Ok(()) => LightResponse {
                trace_id,
                primitive_trace: trace,
                domain: domain_label,
                contract_id: contract.id,
                lgc_seal: contract.lgc_seal,
                algorithm_state,
                evolve_summary,
                user_lang,
                lang_confidence: lang_conf,
                primitive_split,
                input_sha256,
                legacy: Some(legacy),
                status: LightStatus::Routed,
            },
            Err(error) => LightResponse {
                trace_id,
                primitive_trace: trace,
                domain: domain_label,
                contract_id: contract.id,
                lgc_seal: contract.lgc_seal,
                algorithm_state: dsl::AlgorithmState::Failed(
                    error.to_string(),
                )
                .label()
                .to_string(),
                evolve_summary,
                user_lang,
                lang_confidence: lang_conf,
                primitive_split,
                input_sha256,
                legacy: Some(legacy),
                status: LightStatus::ContractFailed(error.to_string()),
            },
        }
    }

    /// FORMAT OUTPUT — pas Shadow verdict + SHA256 + KODUNIK
    /// Konverton output të sistemit në gjuhë akademike tek user
    pub fn format_output(
        &self,
        content:  &str,
        response: &LightResponse,
        verified: bool,
    ) -> String {
        self.lingua.format_output(
            content,
            &response.user_lang,
            &crate::lingua::Intent::Statement, // default — Shadow do specifikon
            verified,
        )
    }

    /// Status i koordinatorit
    pub fn status(&self) -> CoordinatorStatus {
        let cs = self.core.status();
        CoordinatorStatus {
            matter_total:   cs.matter_total,
            matter_online:  cs.matter_online,
            signals_active: cs.signals_active,
            devices_active: cs.devices_active,
            lingua_tier:    self.lingua.tier.label().to_string(),
        }
    }
}

impl Default for LightCoordinator {
    fn default() -> Self { Self::new() }
}

// ════════════════════════════════════════════════════════════════════════════
// §5  COORDINATOR STATUS
// ════════════════════════════════════════════════════════════════════════════

#[derive(Debug)]
pub struct CoordinatorStatus {
    pub matter_total:   usize,
    pub matter_online:  usize,
    pub signals_active: usize,
    pub devices_active: usize,
    pub lingua_tier:    String,
}

impl CoordinatorStatus {
    pub fn print(&self) {
        println!(
            "LightCoordinator → matter:{}/{} signals:{} devices:{} lingua:{}",
            self.matter_online, self.matter_total,
            self.signals_active, self.devices_active,
            self.lingua_tier,
        );
    }
}

// ════════════════════════════════════════════════════════════════════════════
// §6  QUANTUM DISPATCH —  entry points për Quantum Platform
//     Shtuara mbi §4 ekzistues. Zero fshirje.
// ════════════════════════════════════════════════════════════════════════════

use crate::quantum_bridge::{
    full_quantum_cycle, QuantumResult, QBridgeError,
};

/// Rezultati i ciklit të plotë Light → Quantum → Light
#[derive(Debug)]
pub enum DispatchResult {
    /// Quantum u përgjigj me sukses
    Verified(QuantumResult),
    /// Quantum nuk u përgjigj brenda timeout
    Timeout(String),
    /// Gabim i bus ose kontratës
    BridgeError(String),
}

impl DispatchResult {
    pub fn is_verified(&self) -> bool {
        matches!(self, Self::Verified(r) if r.is_verified)
    }

    pub fn content(&self) -> &str {
        match self {
            Self::Verified(r) => &r.content,
            Self::Timeout(s)  => s.as_str(),
            Self::BridgeError(s) => s.as_str(),
        }
    }

    pub fn evolve_sfx(&self) -> &str {
        match self {
            Self::Verified(r) => &r.evolve_sfx,
            _                 => dsl::SFX_NEGATIVE,
        }
    }

    pub fn algorithm_state(&self) -> &str {
        match self {
            Self::Verified(r) => &r.algorithm_state,
            Self::Timeout(_)  => "TIMEOUT",
            Self::BridgeError(_) => "BRIDGE_ERROR",
        }
    }

    /// algorithm_state_tag — qasje E TIPIZUAR te gjendja (RREGULLIM     ).
    ///
    /// Lidh accessor-in e tipizuar algorithm_state_typed() (që ishte i papërdorur)
    /// me një konsumator real. Kthen enum në vend të &str të lirë. Zero if — match.
    pub fn algorithm_state_tag(&self) -> crate::quantum_bridge::AlgorithmStateTag {
        use crate::quantum_bridge::AlgorithmStateTag;
        match self {
            Self::Verified(r)    => r.algorithm_state_typed(),
            Self::Timeout(_)     => AlgorithmStateTag::Unknown,
            Self::BridgeError(_) => AlgorithmStateTag::Unknown,
        }
    }
}

impl LightCoordinator {

    /// DISPATCH_TO_QUANTUM — dërgon LightResponse + text tek Quantum.
    ///
    /// Thirret pas receive() kur status == Routed.
    /// Bllokon deri sa Quantum u përgjigj ose timeout (5s default).
    ///
    /// Hapat:
    ///   1. Ekstrakt contract_id + lgc_seal nga response
    ///   2. full_quantum_cycle() → send + receive
    ///   3. Kthe DispatchResult
    pub fn dispatch_to_quantum(
        &self,
        response: &LightResponse,
        text:     &str,
    ) -> DispatchResult {

        // Porta e vetme e daljes: status + lineage + receipt + handoff durable.
        match response.ready_for_dispatch() {
            Ok(()) => {}
            Err(reason) => return DispatchResult::BridgeError(reason),
        }

        let lang_code = response.user_lang.code().to_string();

        match full_quantum_cycle(
            &response.trace_id,
            &response.domain,
            &response.contract_id,
            &response.lgc_seal,   // lgc_seal shtohet në §3 më poshtë
            &lang_code,
            &response.evolve_summary,
            &response.input_sha256,
            text,
            None,  // timeout default 5s
        ) {
            Ok(result) => DispatchResult::Verified(result),
            Err(QBridgeError::Timeout(ms)) =>
                DispatchResult::Timeout(format!("TIMEOUT:{}ms", ms)),
            Err(e) =>
                DispatchResult::BridgeError(e.to_string()),
        }
    }

    /// HANDLE_QUANTUM_RESPONSE — përpunon QuantumResult.
    ///
    /// Thirret pas dispatch_to_quantum() me sukses.
    /// Hapat:
    ///   1. Shton degën sipas verdict, me koherencë nga Quantum (potentiality).
    ///      → kjo rillogarit vulën 500 te EvolveTrace.
    ///   2. Dorëzon vulën te Shadow përmes buss_legacy (i verbër).
    ///      → shadow_gj_legacy vendos 0/1 (primitive_knowledge ose jo).
    ///   3. Formaton output për user.
    ///
    /// Kthen: (output_string, shadow_verdict).
    pub fn handle_quantum_response(
        &self,
        dispatch: &DispatchResult,
        response: &mut LightResponse,
    ) -> (String, crate::shadow_seal_bridge::ShadowVerdict) {
        use crate::shadow_seal_bridge::{ShadowSealBridge, ShadowVerdict};

        let suffix = dispatch.evolve_sfx();
        let coherence = Self::coherence_from_dispatch(dispatch);
        let lgc_seal = response.lgc_seal.clone();
        let bridge = ShadowSealBridge;

        // EvolveTrace autoritative është ajo që lindi brenda receipt-it LGC.
        // Nuk ndërtohet trace e dytë pas Quantum-it.
        let verdict = match response.legacy.as_mut() {
            Some(legacy) => match legacy.evolve.branch_scored(suffix, coherence) {
                Ok(_) => bridge.dispatch_seal(&mut legacy.evolve, &lgc_seal, 0.10),
                Err(error) => {
                    eprintln!("  [EVOLVE] FAIL-CLOSED: {error}");
                    ShadowVerdict::VerifiableNotPrimitive
                }
            },
            None => {
                eprintln!("  [LEGACY] FAIL-CLOSED: receipt/EvolveTrace mungon pas dispatch-it");
                ShadowVerdict::VerifiableNotPrimitive
            }
        };

        let content = dispatch.content();
        let verified = dispatch.is_verified()
            && verdict == ShadowVerdict::PrimitiveKnowledge;
        let output = self.format_output(content, response, verified);
        (output, verdict)
    }

    /// Koherenca [0,1] nga rezultati i Quantum-it.
    /// OPTIMAL → e lartë (mbetet primitiv); NEGATIVE/HOLD/DISMISSED → e ulët.
    fn coherence_from_dispatch(dispatch: &DispatchResult) -> f32 {
        match dispatch {
            DispatchResult::Verified(r) => {
                // potentiality është tashmë [0,1]; OPTIMAL e afron me 1.0
                let base = r.potentiality.clamp(0.0, 1.0);
                // verdikti rrit ose ul: OPTIMAL → +, ndryshe → toward 0
                let verdict_factor = match r.verdict.as_str() {
                    "OPTIMAL" => 1.0,
                    "HOLD"    => 0.85,
                    _         => 0.50,   // NEGATIVE / DISMISSED → fragmentim
                };
                (base * verdict_factor).clamp(0.0, 1.0)
            }
            // Timeout/BridgeError → s'ka propozim → fragmentim total
            _ => 0.0,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn receive_uses_one_legacy_authority_and_one_kodunik() {
        let coordinator = LightCoordinator::new();
        let response = coordinator.receive(LightRequest::new(
            "Analizo motorin industrial të kuq dhe sensorin termik",
        ));
        assert!(response.status.is_ok(), "{}", response.status.label());
        assert!(response.primitive_continuity_valid());
        let legacy = response.legacy.as_ref().expect("Legacy coordination");
        assert_eq!(response.contract_id, legacy.receipt.kodunik.code);
        assert_eq!(response.input_sha256, legacy.receipt.input_sha256);
        assert_eq!(response.primitive_split, legacy.primitive_split);
        response
            .ready_for_handoff()
            .expect("coordinate collapse must be complete before export");
        assert!(response.ready_for_dispatch().is_err());
    }

    #[test]
    fn durable_evidence_is_required_before_dispatch() {
        let coordinator = LightCoordinator::new();
        let mut response = coordinator.receive(LightRequest::new(
            "Krijo raportin final të laboratorit",
        ));
        let receipt = response
            .legacy
            .as_ref()
            .expect("Legacy coordination")
            .receipt
            .clone();
        let evidence = HandoffEvidence {
            binding_sha256: receipt.binding_sha256,
            receipt_sha256: receipt.receipt_sha256,
            receipt_written: true,
            primitive_anchor_written: true,
            commit_written: true,
            flushed: true,
            kodunik_verified: true,
        };
        response
            .complete_legacy_handoff(&evidence)
            .expect("evidence-bound handoff");
        response
            .ready_for_dispatch()
            .expect("dispatch must open only after durable evidence");
    }

    #[test]
    fn input_without_primitive_material_fails_closed() {
        let coordinator = LightCoordinator::new();
        let response = coordinator.receive(LightRequest::new("dhe ose por"));
        assert!(matches!(&response.status, LightStatus::LegacyFailed(_)));
        assert!(response.legacy.is_none());
        assert!(response.ready_for_dispatch().is_err());
    }
}
