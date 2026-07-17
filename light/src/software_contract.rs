// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  SOFTWARE_CONTRACT.RS — ESS-MAI LIGHT PLATFORM                        ║
// ║  Gjata Legacy™  |  Arkitekt: Bledar Gjata                             ║
// ║                                                                          ║
// ║  ENFORCER — krijon dhe verifikon kontratat ndërmjet moduleve.         ║
// ║  Thirret kur dy module komunikojnë — kontrata duhet ekzistuar.        ║
// ║                                                                          ║
// ║  ALGORITMI:                                                             ║
// ║    payload = "caller:target:action:platform:ts"                        ║
// ║    sha = lgc_algorithm::compute_sha256(payload)                        ║
// ║    id  = lgc_algorithm::generate_kodunik(sha, suffix)                 ║
// ║    lgc = "LGC::CONTRACT|caller|target|action|sha[..16]|ts"            ║
// ║                                                                          ║
// ║  LIDHJET:                                                               ║
// ║    lgc_algorithm  → compute_sha256, generate_kodunik                  ║
// ║    ess_mai_system → communication::is_authorized (auth check)         ║
// ║                                                                          ║
// ║  Roli: ENFORCER — verifikon. Kurrë nuk vendos logjikë biznesi.       ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::lgc_algorithm::{
    compute_sha256, generate_kodunik, is_canonical_sha256, verify_kodunik,
    LegacyLgcReceipt,
};
use crate::ess_mai_system::communication::is_authorized;
use crate::ess_mai_system::limits::CONTRACT_MAX_AGE_SECS;

use std::time::{SystemTime, UNIX_EPOCH};

// ════════════════════════════════════════════════════════════════════════════
// §1  MODULET E NJOHURA — sistemi njeh vetveten
// ════════════════════════════════════════════════════════════════════════════

const KNOWN_MODULES: &[&str] = &[
    // LIGHT
    "light_coordinator", "trace_info", "evolve_trace",
    "nura_core", "software_contract", "ess_mai_system",
    "manifest", "light_buss", "lgc_algorithm",
    "legalgocrypt", "lgc_bridge", "ess_manifest",
    // SHADOW
    "shadow_judiciary", "shadow_kernel", "nk_system",
    "legacy_continuum", "vok_store", "vnk_store", "shadow_buss",
    // QUANTUM
    "quantum_platform", "lim_engine", "pro_engine",
    "srk_engine", "pim_engine", "convergence",
    "ring_bridge", "ebpf_hydrator",
];

/// RUNTIME HARDENING (      , nga GPT runtime_patch): token i pastër kontrate.
/// Pengon delimiter-injection: pa '|;=:\n\r\0' dhe pa hapësira anësore. Zero if — match.
fn is_clean_contract_token(value: &str) -> bool {
    let trimmed = value.trim();
    match trimmed.is_empty() {
        true => false,
        false => {
            value == trimmed
                && !value.chars().any(|c| matches!(c, '|' | ';' | '=' | ':' | '\n' | '\r' | '\0'))
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// §2  CONTRACT VIOLATION — gabimet e kontratës
// ════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, PartialEq)]
pub enum ContractViolation {
    /// Moduli thirres nuk njihet nga sistemi
    CallerUnknown(String),

    /// Moduli i thirrur nuk njihet nga sistemi
    TargetUnknown(String),

    /// Thirja nuk është e autorizuar (communication matrix)
    Unauthorized { caller: String, target: String },

    /// Seal KODUNIK invalid ose format gabim
    SealInvalid(String),

    /// Kontrata ka skaduar (> CONTRACT_MAX_AGE_SECS)
    Expired(String),

    /// LGC format gabim
    LgcMalformed(String),
}

impl std::fmt::Display for ContractViolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CallerUnknown(m)        => write!(f, "VIOLATION:CALLER_UNKNOWN:{}", m),
            Self::TargetUnknown(m)        => write!(f, "VIOLATION:TARGET_UNKNOWN:{}", m),
            Self::Unauthorized{caller,target}
                => write!(f, "VIOLATION:UNAUTHORIZED:{}→{}", caller, target),
            Self::SealInvalid(s)          => write!(f, "VIOLATION:SEAL_INVALID:{}", s),
            Self::Expired(id)             => write!(f, "VIOLATION:EXPIRED:{}", id),
            Self::LgcMalformed(s)         => write!(f, "VIOLATION:LGC_MALFORMED:{}", s),
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// §3  CONTRACT — kontrata e vulosur
// ════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub struct Contract {
    /// KODUNIK seal — KDS:SHA8-CTR6-SUFFIX
    pub id:       String,

    /// Thirruesi (caller)
    pub caller:   String,

    /// I thirruri (target)
    pub target:   String,

    /// Veprimi i autorizuar
    pub action:   String,

    /// Platforma ku ndodh komunikimi
    pub platform: String,

    /// Timestamp sekonda UNIX
    pub ts:       u64,

    /// LGC lightweight seal
    pub lgc_seal: String,

    /// Aktiv (false nëse u anulua manualisht)
    pub valid:    bool,

    /// FIX #6: fingerprint kanonik mbi fushat — zbulon modifikim pas krijimit.
    /// hash(caller|target|action|platform|ts|id). enforce() e rikomputon e krahason.
    pub fingerprint: String,

    /// SHA mbi të cilën është vulosur KODUNIK-u.
    pub authority_sha256: String,

    /// Receipt LGC V2 kur kontrata ripërdor autoritetin Legacy.
    pub legacy_receipt_sha256: Option<String>,
}

impl Contract {
    /// compute_fingerprint — hash kanonik mbi fushat e kontratës (#6).
    /// Çdo ndryshim i caller/target/action/platform/ts/id → fingerprint tjetër.
    pub fn compute_fingerprint(
        caller: &str, target: &str, action: &str,
        platform: &str, ts: u64, id: &str,
    ) -> String {
        let canonical = format!("{caller}|{target}|{action}|{platform}|{ts}|{id}");
        compute_sha256(canonical.as_bytes())
    }

    /// current_fingerprint — rikomputon fingerprint nga gjendja AKTUALE. Zero if.
    pub fn current_fingerprint(&self) -> String {
        Self::compute_authority_fingerprint(
            &self.caller, &self.target, &self.action,
            &self.platform, self.ts, &self.id, &self.authority_sha256,
            self.legacy_receipt_sha256.as_deref(),
        )
    }

    pub fn compute_authority_fingerprint(
        caller: &str,
        target: &str,
        action: &str,
        platform: &str,
        ts: u64,
        id: &str,
        authority_sha256: &str,
        legacy_receipt_sha256: Option<&str>,
    ) -> String {
        let base = Self::compute_fingerprint(caller, target, action, platform, ts, id);
        compute_sha256(
            format!(
                "{}|{}|{}",
                base,
                authority_sha256,
                legacy_receipt_sha256.unwrap_or("LEGACY_NONE"),
            )
            .as_bytes(),
        )
    }
    /// Label human-readable: light_coordinator→quantum_platform::route_input
    pub fn label(&self) -> String {
        format!("{}→{}::{}", self.caller, self.target, self.action)
    }

    /// A ka skaduar?
    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        now > self.ts + CONTRACT_MAX_AGE_SECS
    }
}

// ════════════════════════════════════════════════════════════════════════════
// §4  SOFTWARE CONTRACT — factory + enforcer
// ════════════════════════════════════════════════════════════════════════════

pub struct SoftwareContract;

impl SoftwareContract {
    /// KRIJON kontratë ndërmjet dy moduleve.
    ///
    /// Thirrur nga light_coordinator ose çdo modul tjetër
    /// para se të dërgojë mesazh tek moduli tjetër.
    ///
    /// Hapat:
    ///   1. Verifikon caller dhe target janë module të njohura
    ///   2. Verifikon autorizimin (communication matrix)
    ///   3. Gjeneron payload SHA256
    ///   4. Vulos me KODUNIK (lgc_algorithm)
    ///   5. Krijon LGC lightweight seal
    ///   6. Kthen Contract ose ContractViolation
    pub fn create(
        caller:   &str,
        target:   &str,
        action:   &str,
        platform: &str,
    ) -> Result<Contract, ContractViolation> {

        // §4.0 — RUNTIME HARDENING (      ): tokens pa delimiter-injection.
        // Konvertuar `if !is_clean` → match (zero if/else). Zero if — loop+match.
        for (label, value) in [
            ("caller", caller),
            ("target", target),
            ("action", action),
            ("platform", platform),
        ] {
            match is_clean_contract_token(value) {
                true => {}
                false => return Err(ContractViolation::LgcMalformed(
                    format!("invalid {label} token: {value}")
                )),
            }
        }

        // §4.1 — Caller i njohur?
        match KNOWN_MODULES.contains(&caller) {
            false => return Err(ContractViolation::CallerUnknown(caller.to_string())),
            true => {}
        }

        // §4.2 — Target i njohur?
        match KNOWN_MODULES.contains(&target) {
            false => return Err(ContractViolation::TargetUnknown(target.to_string())),
            true => {}
        }

        // §4.3 — Autorizim (communication matrix)
        match is_authorized(caller, target) {
            false => return Err(ContractViolation::Unauthorized {
                caller: caller.to_string(),
                target: target.to_string(),
            }),
            true => {}
        }

        // §4.4 — Timestamp
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // §4.5 — Payload + SHA256
        let payload = format!("{}:{}:{}:{}:{}", caller, target, action, platform, ts);
        let sha = compute_sha256(payload.as_bytes());

        // §4.6 — KODUNIK suffix: prime 3 chars caller + prime 3 chars target
        let c_sfx = &caller[..3.min(caller.len())].to_uppercase();
        let t_sfx = &target[..3.min(target.len())].to_uppercase();
        let suffix = format!("{}{}", c_sfx, t_sfx);

        let id = generate_kodunik(&sha, &suffix);

        // §4.7 — LGC lightweight seal
        let lgc_seal = format!(
            "LGC::CONTRACT|{}|{}|{}|{}|{}",
            caller, target, action, &sha[..16], ts
        );

        // FIX #6: fingerprint kanonik mbi fushat (integritet kundër modifikimit).
        let fingerprint = Contract::compute_authority_fingerprint(
            caller, target, action, platform, ts, &id, &sha, None,
        );

        Ok(Contract {
            id,
            caller:   caller.to_string(),
            target:   target.to_string(),
            action:   action.to_string(),
            platform: platform.to_string(),
            ts,
            lgc_seal,
            valid: true,
            fingerprint,
            authority_sha256: sha,
            legacy_receipt_sha256: None,
        })
    }

    /// Krijon kontratën e handoff-it duke ripërdorur KODUNIK-un autoritativ LGC.
    pub fn create_bound(
        caller: &str,
        target: &str,
        action: &str,
        platform: &str,
        receipt: &LegacyLgcReceipt,
    ) -> Result<Contract, ContractViolation> {
        receipt
            .verify_runtime()
            .map_err(|e| ContractViolation::SealInvalid(e.to_string()))?;
        for (label, value) in [
            ("caller", caller),
            ("target", target),
            ("action", action),
            ("platform", platform),
        ] {
            if !is_clean_contract_token(value) {
                return Err(ContractViolation::LgcMalformed(format!(
                    "invalid {label} token: {value}"
                )));
            }
        }
        if !KNOWN_MODULES.contains(&caller) {
            return Err(ContractViolation::CallerUnknown(caller.to_string()));
        }
        if !KNOWN_MODULES.contains(&target) {
            return Err(ContractViolation::TargetUnknown(target.to_string()));
        }
        if !is_authorized(caller, target) {
            return Err(ContractViolation::Unauthorized {
                caller: caller.to_string(),
                target: target.to_string(),
            });
        }
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let id = receipt.kodunik.code.clone();
        let authority_sha256 = receipt.binding_sha256.clone();
        let legacy_receipt_sha256 = Some(receipt.receipt_sha256.clone());
        let lgc_seal = format!(
            "LGC::CONTRACT|{}|{}|{}|{}|{}|flags:0xA451|legacy:{}",
            caller,
            target,
            action,
            &authority_sha256[..16],
            ts,
            receipt.receipt_sha256,
        );
        let fingerprint = Contract::compute_authority_fingerprint(
            caller,
            target,
            action,
            platform,
            ts,
            &id,
            &authority_sha256,
            legacy_receipt_sha256.as_deref(),
        );
        Ok(Contract {
            id,
            caller: caller.to_string(),
            target: target.to_string(),
            action: action.to_string(),
            platform: platform.to_string(),
            ts,
            lgc_seal,
            valid: true,
            fingerprint,
            authority_sha256,
            legacy_receipt_sha256,
        })
    }

    /// VERIFIKON kontratën ekzistuese.
    ///
    /// Thirrur para ekzekutimit të veprimit të kontratës.
    /// Kthen Ok(()) nëse kontrata është e vlefshme,
    /// ose Err(ContractViolation) nëse ka problem.
    pub fn enforce(contract: &Contract) -> Result<(), ContractViolation> {
        // §4.8 — Kontrata aktive?
        match contract.valid {
            false => return Err(ContractViolation::SealInvalid(contract.id.clone())),
            true => {}
        }

        // §4.9 — Ka skaduar?
        match contract.is_expired() {
            true => return Err(ContractViolation::Expired(contract.id.clone())),
            false => {}
        }

        // §4.10 — KODUNIK format i saktë?
        match contract.id.starts_with("KDS:") {
            false => return Err(ContractViolation::SealInvalid(
                format!("expected KDS: prefix, got: {}", &contract.id[..8.min(contract.id.len())])
            )),
            true => {}
        }

        // §4.11 — LGC format i saktë?
        match contract.lgc_seal.starts_with("LGC::CONTRACT|") {
            false => return Err(ContractViolation::LgcMalformed(
                format!("expected LGC::CONTRACT| prefix: {}", &contract.lgc_seal[..20.min(contract.lgc_seal.len())])
            )),
            true => {}
        }

        // §4.12 — RUNTIME HARDENING (      , GPT runtime_patch): KODUNIK verifikohet
        // me SHA reale (HMAC nga registry). Zbulon ID të falsifikuar. Zero if — match.
        let payload = format!(
            "{}:{}:{}:{}:{}",
            contract.caller, contract.target, contract.action, contract.platform, contract.ts
        );
        let payload_sha = compute_sha256(payload.as_bytes());
        if !is_canonical_sha256(&contract.authority_sha256) {
            return Err(ContractViolation::SealInvalid(
                "authority SHA-256 invalid".to_string(),
            ));
        }
        match &contract.legacy_receipt_sha256 {
            None if contract.authority_sha256 != payload_sha => {
                return Err(ContractViolation::SealInvalid(
                    "legacy contract authority mismatch".to_string(),
                ));
            }
            Some(receipt_sha) if !is_canonical_sha256(receipt_sha) => {
                return Err(ContractViolation::SealInvalid(
                    "legacy receipt SHA-256 invalid".to_string(),
                ));
            }
            _ => {}
        }
        match verify_kodunik(&contract.id, &contract.authority_sha256) {
            true => {}
            false => return Err(ContractViolation::SealInvalid(
                format!("KODUNIK mismatch — kontratë e pavlefshme: {}", contract.id)
            )),
        }

        // §4.13 — LGC seal duhet të jetë identik me gjendjen aktuale (recompute+compare).
        let expected_lgc_seal = match &contract.legacy_receipt_sha256 {
            Some(receipt_sha) => format!(
                "LGC::CONTRACT|{}|{}|{}|{}|{}|flags:0xA451|legacy:{}",
                contract.caller,
                contract.target,
                contract.action,
                &contract.authority_sha256[..16],
                contract.ts,
                receipt_sha,
            ),
            None => format!(
                "LGC::CONTRACT|{}|{}|{}|{}|{}",
                contract.caller,
                contract.target,
                contract.action,
                &payload_sha[..16],
                contract.ts,
            ),
        };
        match contract.lgc_seal == expected_lgc_seal {
            true => {}
            false => return Err(ContractViolation::LgcMalformed(
                format!("seal mismatch — kontratë e modifikuar: {}", contract.id)
            )),
        }

        // §4.14 — INTEGRITET (#6): fingerprint-i aktual përkon me atë të ruajtur?
        // Nëse caller/target/action/platform/ts/id u modifikuan pas krijimit →
        // fingerprint ndryshon → kontrata refuzohet. Zero if — match.
        let recomputed = contract.current_fingerprint();
        match recomputed == contract.fingerprint {
            false => return Err(ContractViolation::SealInvalid(
                format!("fingerprint mismatch — kontratë e modifikuar: {}", contract.id)
            )),
            true => {}
        }

        Ok(())
    }
}
