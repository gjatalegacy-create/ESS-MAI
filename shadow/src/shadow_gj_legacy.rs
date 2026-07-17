// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  SHADOW_GJ_LEGACY.RS — AUTORITETI SUPREM (Rust wrapper)          ║
// ║  Gjata Legacy™  |  Arkitekt: Bledar Gjata                             ║
// ║                                                                          ║
// ║  Thirr kernelin C shadow_gj_legacy.c. I VETMI që di ç'është 500.     ║
// ║  I VETMI që shkruan në KnowledgeVault.                                ║
// ║                                                                          ║
// ║ :                                                                 ║
// ║    • FFI i zgjeruar: lgc_freeze/lgc_unfreeze/lgc_get_stats/seal_check  ║
// ║    • Ligjet sovrane zbatohen PARA (enforce_sovereign_laws) → këtu      ║
// ║      verified = judiciary_ok (refuzimi sovran s'mbërrin këtu).         ║
// ║    • LIGJI 0 (zero-copy): judge_supreme KONSUMON passage dhe ZHVENDOS  ║
// ║      raw_bytes në vault (asnjë klon i payload-it).                    ║
// ║    • feature `pure_rust` → pasqyrë Rust e kernelit (testim pa C FFI)   ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::knowledge_lineage::{ledger, Lineage};
use crate::luvik::Luvik;
use crate::knowledge_vault::KnowledgeVault;
use crate::types::{
    KnowledgeBand, NegativeContext, PassPackage, ScientificProjectContext, ShadowError,
    ShadowPassage, SupremeVerdict,
};
use std::sync::Arc;

// ════════════════════════════════════════════════════════════════════════════
// §0  KERNEL — pamje e njësuar mbi C FFI (default) ose pasqyrë Rust (pure_rust)
// ════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, Default)]
pub struct KernelStats {
    pub total:   u64,
    pub passed:  u64,
    pub blocked: u64,
    pub frozen:  i32,
}

// Konstantet e vulës (identike me shadow_gj_legacy.c)
const SGL_SEAL_XOR:       u32 = 0xA5A5;
const SGL_SEAL_MASK:      u32 = 0x0000_FFFF;
const SGL_SEAL_PRIMITIVE: u32 = 500;

// ─────────────────────────────────────────────────────────────────────────────
// VARIANTI REAL — C FFI (kerneli C është autoriteti)
// ─────────────────────────────────────────────────────────────────────────────
#[cfg(not(feature = "pure_rust"))]
mod kernel {
    use super::KernelStats;

    #[repr(C)]
    struct LgcRequest {
        req_id:    [u8; 48],
        caller:    [u8; 64],
        sha256:    [u8; 65],
        ram_usage: f32,
        flags:     u32,
        ts:        u64,
    }

    #[repr(C)]
    struct LgcResult {
        req_id:   [u8; 48],
        decision: u32, /* 0=BLOCK 1=PASS 2=REFINE 3=WAIT */
        state:    u32,
        sha256:   [u8; 65],
        reason:   [u8; 128],
        ts:       u64,
    }

    #[repr(C)]
    struct LgcStats {
        total:   u64,
        passed:  u64,
        blocked: u64,
        state:   i32,
        frozen:  i32,
    }

    extern "C" {
        fn lgc_init() -> i32;
        fn lgc_check(req: *const LgcRequest, res: *mut LgcResult) -> i32;
        fn lgc_freeze(stage: i32) -> i32;
        fn lgc_unfreeze() -> i32;
        fn lgc_get_stats(stats: *mut LgcStats);
        fn shadow_lgc_seal_check(flags: u32) -> i32;
    }

    pub fn init() -> i32 {
        // SAFETY: `lgc_init` nuk merr pointera dhe inicializon singleton-in C.
        unsafe { lgc_init() }
    }
    pub fn freeze(stage: i32) -> i32 {
        // SAFETY: `stage` kalon me vlerë; kerneli C nuk mban referenca Rust.
        unsafe { lgc_freeze(stage) }
    }
    pub fn unfreeze() -> i32 {
        // SAFETY: funksioni C nuk merr argumente dhe nuk prek memorie Rust.
        unsafe { lgc_unfreeze() }
    }
    pub fn seal_check(flags: u32) -> bool {
        // SAFETY: `flags` kalon me vlerë dhe ABI-ja deklarohet identikisht në C.
        unsafe { shadow_lgc_seal_check(flags) != 0 }
    }

    pub fn stats() -> KernelStats {
        let mut s = LgcStats { total: 0, passed: 0, blocked: 0, state: 0, frozen: 0 };
        // SAFETY: `s` është i inicializuar, unik dhe valid për shkrim gjatë thirrjes.
        unsafe { lgc_get_stats(&mut s) };
        KernelStats { total: s.total, passed: s.passed, blocked: s.blocked, frozen: s.frozen }
    }

    /// Thirr autoritetin: kthen 1 nëse PASS (vula 500 mbijetoi & jo i ngrirë).
    pub fn check(flags: u32, ram_usage: f32, ts_ns: u64, input_id: u64) -> u8 {
        let mut req = LgcRequest {
            req_id: [0u8; 48], caller: [0u8; 64], sha256: [0u8; 65],
            ram_usage, flags, ts: ts_ns,
        };
        let caller = b"SHADOW_GJ_LEGACY";
        let n = caller.len().min(63);
        req.caller[..n].copy_from_slice(&caller[..n]);
        req.req_id[..8].copy_from_slice(&input_id.to_le_bytes());

        let mut res = LgcResult {
            req_id: [0u8; 48], decision: 0, state: 0,
            sha256: [0u8; 65], reason: [0u8; 128], ts: 0,
        };
        // SAFETY: `req` dhe `res` janë repr(C), të inicializuar dhe jetojnë gjatë thirrjes.
        let rc = unsafe { lgc_check(&req, &mut res) };
        ((rc == 0) & (res.decision == 1)) as u8
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// VARIANTI PURE-RUST — pasqyrë identike e logjikës së kernelit (testim pa C)
// ─────────────────────────────────────────────────────────────────────────────
#[cfg(feature = "pure_rust")]
mod kernel {
    use super::{KernelStats, SGL_SEAL_MASK, SGL_SEAL_PRIMITIVE, SGL_SEAL_XOR};
    use std::sync::atomic::{AtomicI32, AtomicU64, Ordering};

    static TOTAL:   AtomicU64 = AtomicU64::new(0);
    static PASSED:  AtomicU64 = AtomicU64::new(0);
    static BLOCKED: AtomicU64 = AtomicU64::new(0);
    static FROZEN:  AtomicI32 = AtomicI32::new(0);

    pub fn init() -> i32 {
        TOTAL.store(0, Ordering::SeqCst);
        PASSED.store(0, Ordering::SeqCst);
        BLOCKED.store(0, Ordering::SeqCst);
        FROZEN.store(0, Ordering::SeqCst);
        0
    }

    pub fn freeze(stage: i32) -> i32 {
        // Zero if/else — formulë: nëse stage==0 → 1, ndryshe → stage.
        // (stage==0) është 1 kur stage=0 (→ 0+1=1), 0 ndryshe (→ stage+0=stage).
        let is_zero = (stage == 0) as i32;
        let frozen_stage = stage + is_zero;
        FROZEN.store(frozen_stage, Ordering::SeqCst);
        0
    }

    pub fn unfreeze() -> i32 {
        FROZEN.store(0, Ordering::SeqCst);
        0
    }

    pub fn seal_check(flags: u32) -> bool {
        let cleared = flags != 0;
        let value = (flags & SGL_SEAL_MASK) ^ SGL_SEAL_XOR;
        cleared && value == SGL_SEAL_PRIMITIVE
    }

    pub fn stats() -> KernelStats {
        KernelStats {
            total:   TOTAL.load(Ordering::SeqCst),
            passed:  PASSED.load(Ordering::SeqCst),
            blocked: BLOCKED.load(Ordering::SeqCst),
            frozen:  FROZEN.load(Ordering::SeqCst),
        }
    }

    pub fn check(flags: u32, _ram_usage: f32, _ts_ns: u64, _input_id: u64) -> u8 {
        let live   = (FROZEN.load(Ordering::SeqCst) == 0) as u8;
        let sealed = seal_check(flags) as u8;
        let pass   = live & sealed;
        TOTAL.fetch_add(1, Ordering::SeqCst);
        PASSED.fetch_add(pass as u64, Ordering::SeqCst);
        BLOCKED.fetch_add((1 - pass) as u64, Ordering::SeqCst);
        pass
    }
}

// ════════════════════════════════════════════════════════════════════════════
// §1  SUPREME JUDGE
// ════════════════════════════════════════════════════════════════════════════


#[derive(Debug, Clone, Copy)]
struct ProjectAdjudication {
    eco_class: crate::shadow_eco::EpistemicClass,
    project_id: u64,
    project_status: u8,
    project_context_digest: u64,
    project_evidence_digest: u64,
    project_context_sha256: [u8; 32],
    project_evidence_sha256: [u8; 32],
    novel_factualized: bool,
    novel_trl_level: u8,
    novel_proof_score: u32,
    novel_rejection: u8,
}

impl ProjectAdjudication {
    fn none(eco_class: crate::shadow_eco::EpistemicClass) -> Self {
        Self {
            eco_class,
            project_id: 0,
            project_status: shadow_contracts::PROJECT_STATUS_NONE,
            project_context_digest: 0,
            project_evidence_digest: 0,
            project_context_sha256: [0u8; 32],
            project_evidence_sha256: [0u8; 32],
            novel_factualized: false,
            novel_trl_level: 0,
            novel_proof_score: 0,
            novel_rejection: 0,
        }
    }
}

pub struct ShadowGjLegacy;

impl ShadowGjLegacy {
    /// Inicializon kernelin — thirr 1 herë në startup.
    pub fn init() -> Result<(), ShadowError> {
        let rc = kernel::init();
        match rc != 0 {
            true => return Err(ShadowError::SealInvalid(format!("lgc_init failed: {}", rc))),
            false => {}
        }
        Ok(())
    }

    /// Ngrirja sovrane (administrative) — pasqyrohet te kerneli.
    pub fn freeze(stage: i32) -> i32 { kernel::freeze(stage) }
    pub fn unfreeze() -> i32 { kernel::unfreeze() }

    /// Statistikat e autoritetit (total/passed/blocked/frozen).
    pub fn get_stats() -> KernelStats { kernel::stats() }

    /// Kontroll i pastër i vulës 500 (pa state) — observueshmëri.
    pub fn seal_check(flags: u32) -> bool { kernel::seal_check(flags) }

    /// Vendimi suprem — i vetmi që shikon 500, i vetmi që shkruan vault.
    ///
    /// LIGJI 0 (zero-copy): KONSUMON `passage` dhe ZHVENDOS raw_bytes (payload)
    /// në vault — asnjë klon i payload-it. Ligjet sovrane janë zbatuar PARA,
    /// pra këtu `verified = judiciary_ok` (allow ≡ 1).
    ///
    ///   verified  = judiciary_ok                  (ZERO if/else)
    ///   primitive = verified AND seal_500_pass
    pub fn judge_supreme(
        passage: ShadowPassage,
        vault:   &Arc<KnowledgeVault>,
        ts_ns:   u64,
    ) -> Result<SupremeVerdict, ShadowError> {
        // ── 1) Lexime (vetëm &; vlera Copy) ───────────────────────────────
        let input_id     = passage.input_id;
        let flags        = passage.primitive_flags;
        let judiciary_ok = passage.judiciary_verdict();           /* 0/1 */

        let mass         = passage.package.epistemic_mass;
        let band         = passage.package.knowledge_band();
        let legacy_score = Self::legacy_score_compute(&passage.package);
        let action_state = passage.package.quantum_action_state;
        let action_mask = passage.package.quantum_action_mask;
        let required_action_mask = passage.package.quantum_required_action_mask;

        // Inputet e maturimit (Copy) — llogariten para zhvendosjes.
        let vector_points = (passage.package.evidence_density     * 0.25
                           + passage.package.logical_coherence    * 0.20
                           + passage.package.causal_integrity     * 0.25
                           + passage.package.convergence_strength * 0.15
                           + passage.package.reproducibility      * 0.15)
                           .clamp(0.0, 1.0);
        let temporal_score = passage.package.reproducibility.clamp(0.0, 1.0);

        // ── 2) Autoriteti C/Rust mbi vulën 500 (+ jo i ngrirë) ────────────
        let ram_usage = (1.0 - mass).clamp(0.0, 1.0);
        let c_pass    = kernel::check(flags, ram_usage, ts_ns, input_id);

        // ── 3) verified / primitive — aritmetikë e pastër (ZERO if/else) ──
        let verified  = judiciary_ok;          /* ligjet sovrane kaluan ⇒ allow=1 */
        let primitive = verified & c_pass;

        // ── 4) LGC LAW bitmask (gjendje e dijes) ──────────────────────────
        let law1 = (primitive as u32) * 0x01;                    /* PRIMITIVE   */
        let law2 = (verified  as u32) * 0x02;                    /* VERIFIED    */
        let legacy_active = (primitive as u32) & ((legacy_score > 0.50) as u32);
        let law3 = legacy_active * 0x04;                         /* LEGACY PATH */
        let law5 = (1u32 - verified as u32) * 0x10;              /* NEGATIVE    */
        let lgc_law = law1 | law2 | law3 | law5;

        // ── 5) GJURMUESHMËRIA: ndërto lineage-in nga ZINXHIRI para konsumimit.
        //       proof_chain nga Light = prejardhja algoritmike upstream.
        let lineage = Lineage::derive_chain(input_id, &passage.verdicts, &passage.light.proof_chain);
        let admission = Luvik::admit(&lineage); // PORTA SOVRANE E SHKRIMIT

        // ── 6) PORTA LUVIK — Novel nuk gjykohet para lineage-it. ─────────
        // Pa gjurmë, projekti mbetet HOLD/TRACE_REJECTED dhe GeniusNovel nuk
        // thirret fare. Kjo mban rendin: GCL → evidence → lineage → Novel.
        match admission {
            Err(reject) => {
                let project = Self::project_hold_for_trace_rejection(&passage.package);
                crate::shadow_destfake::destfake().on_reject(reject);
                let ShadowPassage { package, light, .. } = passage;
                let _ = (package.raw_bytes, light.territory); // purge, zero persistence
                let dsl = Self::emit_dsl(0, 0, band, 0x20);
                let verdict = SupremeVerdict {
                    verified: 0,
                    primitive: 0,
                    knowledge_band: band,
                    lgc_law: 0x20,
                    legacy_score,
                    lgc_dsl: dsl,
                    living_trust_sha256: [0u8; 32],
                    living_trust_intensity: 0,
                    living_trust_kind: 0,
                    living_trust_action_state: action_state,
                    living_trust_action_mask: action_mask,
                    living_trust_required_action_mask: required_action_mask,
                    living_trust_system_laws_seal: 0,
                    living_trust_sovereign_flags: flags,
                    living_trust_sovereign_value: 0,
                    project_id: project.project_id,
                    project_status: project.project_status,
                    project_context_digest: project.project_context_digest,
                    project_evidence_digest: project.project_evidence_digest,
                    project_context_sha256: project.project_context_sha256,
                    project_evidence_sha256: project.project_evidence_sha256,
                    novel_factualized: false,
                    novel_trl_level: project.novel_trl_level,
                    novel_proof_score: 0,
                    novel_rejection: project.novel_rejection,
                };
                return Self::seal_living_trust(verdict);
            }
            Ok(_) => {}
        }

        // P5.1 / v1.6.4: vetëm pas Luvik, projekti gjykohet brenda të njëjtit
        // judge_supreme. Nuk ka degë Novel paralele dhe nuk ka verdict Quantum.
        let project = Self::adjudicate_project_under_gcl(
            &passage.package, verified, primitive, band, ts_ns,
        )?;
        let eco_bit = match project.eco_class {
            crate::shadow_eco::EpistemicClass::ExactScienceTrl4 => 0x40u32,
            _ => 0x00u32,
        };
        let lgc_law_eco = lgc_law | eco_bit;
        let lgc_dsl = Self::emit_dsl(primitive, verified, band, lgc_law_eco);

        // ── 7) ZHVENDOS origjinat pas gjykimit (zero-copy në vault) ───────
        let ShadowPassage { package, light, path, primitive_flags, .. } = passage;
        let raw_bytes = package.raw_bytes;
        let territory = light.territory;

        // ── 8) Shkruan në KnowledgeVault (i vetmi shkrues persistent) ─────
        match (primitive == 1, verified == 1) {
            (true, _) => {
                vault.write_primitive(input_id, vector_points, temporal_score, mass, primitive_flags, path, territory, raw_bytes, ts_ns)?;
                // Provo kristalizimin Legacy (mbijetesa e zgjatur; fresh s'promovon).
                // RUNTIME HARDENING (v1.0.7): rezultati bëhet eksplicit (numër i promovuar).
                // Proces sekondar — s'ndikon vendimin suprem, por s'injorohet i heshtur.
                let _promoted_to_legacy = vault.try_promote_to_legacy();
            }
            (false, true) => {
                vault.write_verified(input_id, raw_bytes, band, mass, ts_ns)?;
            }
            (false, false) => {
                // Dije Negative me KONTEKST. Shkaku: judiciary=0 | vula e hequr.
                let reason = (1u32 - judiciary_ok as u32) * 0x01
                           | (1u32 - c_pass       as u32) * 0x100;
                let ctx = NegativeContext {
                    failure_reason_code: reason,
                    process:             territory,
                    mass_at_fail:        mass,
                    frequency:           1,
                };
                vault.write_negative(input_id, raw_bytes, ctx, ts_ns)?;
            }
        }

        // Gjurma e plotë regjistrohet → dija e ruajtur është plotësisht e gjurmueshme.
        ledger().record(lineage);

        let verdict = SupremeVerdict {
            verified,
            primitive,
            knowledge_band: band,
            lgc_law:        lgc_law_eco,
            legacy_score,
            lgc_dsl,
            living_trust_sha256: [0u8; 32],
            living_trust_intensity: 0,
            living_trust_kind: 0,
            living_trust_action_state: action_state,
            living_trust_action_mask: action_mask,
            living_trust_required_action_mask: required_action_mask,
            living_trust_system_laws_seal: 0,
            living_trust_sovereign_flags: flags,
            living_trust_sovereign_value: 0,
            project_id: project.project_id,
            project_status: project.project_status,
            project_context_digest: project.project_context_digest,
            project_evidence_digest: project.project_evidence_digest,
            project_context_sha256: project.project_context_sha256,
            project_evidence_sha256: project.project_evidence_sha256,
            novel_factualized: project.novel_factualized,
            novel_trl_level: project.novel_trl_level,
            novel_proof_score: project.novel_proof_score,
            novel_rejection: project.novel_rejection,
        };
        Self::seal_living_trust(verdict)
    }

    fn project_hold_for_trace_rejection(package: &PassPackage) -> ProjectAdjudication {
        match package.scientific_project.as_ref() {
            Some(project) => ProjectAdjudication {
                eco_class: crate::shadow_eco::EpistemicClass::HypothesisPotential,
                project_id: project.project_id,
                project_status: shadow_contracts::PROJECT_STATUS_HOLD,
                project_context_digest: crate::gcl_project_contract::digest64(
                    &project.context_sha256,
                ),
                project_evidence_digest: crate::gcl_project_contract::digest64(
                    &project.evidence_sha256,
                ),
                project_context_sha256: crate::gcl_project_contract::parse_sha256(
                    &project.context_sha256,
                ).unwrap_or([0u8; 32]),
                project_evidence_sha256: crate::gcl_project_contract::parse_sha256(
                    &project.evidence_sha256,
                ).unwrap_or([0u8; 32]),
                novel_factualized: false,
                novel_trl_level: project.trl_level.min(3),
                novel_proof_score: 0,
                novel_rejection: shadow_contracts::NOVEL_REJECTION_TRACE_REJECTED,
            },
            None => ProjectAdjudication::none(crate::shadow_eco::EpistemicClass::Negative),
        }
    }

    /// v1.6.4 — Gjykimi i projektit shkencor brenda GCL.
    /// Shadow nuk beson llojin e skedarit të deklaruar: e rillogarit me magic bytes.
    /// Factualization lejohet vetëm kur vendimi sovran është (Y=1,X=1).
    fn adjudicate_project_under_gcl(
        package: &PassPackage,
        verified: u8,
        primitive: u8,
        band: KnowledgeBand,
        ts_ns: u64,
    ) -> Result<ProjectAdjudication, ShadowError> {
        let scientific = match package.scientific_project.as_ref() {
            Some(project) => project,
            None => {
                let trl = crate::shadow_lab::TrlInput {
                    trl_level: (package.epistemic_mass.clamp(0.0, 1.0) * 5.0) as u8,
                    trl_passed: package.causal_integrity > 0.5,
                    confidence: package.logical_coherence,
                    reproducibility: package.reproducibility,
                    measurable: package.evidence_density > 0.5,
                    reproducible: package.reproducibility > 0.5,
                };
                let input = crate::shadow_eco::ClassificationInput {
                    verified,
                    primitive,
                    trl,
                    is_existing_science: band == KnowledgeBand::Scientific,
                };
                return Ok(ProjectAdjudication::none(
                    crate::shadow_eco::ShadowEco::classify(&input).class,
                ));
            }
        };

        // SHADOW / GCL — katër porta të ndara, pa autoritet paralel:
        // 1) lidhja kushtetuese e projektit me GCL/Spine;
        // 2) prova fizike dhe magic bytes;
        // 3) verifikimi TRL nga ShadowLab;
        // 4) vetëm pas (Y=1,X=1), factualization TRL4 nga GeniusNovel.
        Self::verify_project_gcl_stage(package, scientific)?;
        Self::verify_project_file_kinds(scientific)?;
        let trl = crate::shadow_lab::TrlInput {
            trl_level: scientific.trl_level,
            trl_passed: scientific.trl_passed,
            confidence: (scientific.trl_confidence as f32
                / shadow_contracts::MASS_SCALE as f32).clamp(0.0, 1.0),
            reproducibility: (scientific.trl_reproducibility as f32
                / shadow_contracts::MASS_SCALE as f32).clamp(0.0, 1.0),
            measurable: !scientific.findings.trim().is_empty(),
            reproducible: scientific.trl_reproducibility >= 5_000,
        };
        let input = crate::shadow_eco::ClassificationInput {
            verified,
            primitive,
            trl,
            is_existing_science: band == KnowledgeBand::Scientific,
        };
        let evidence = crate::shadow_genius_novel::NovelEvidence {
            project_id: scientific.project_id,
            files: scientific.files.iter()
                .map(|(_, bytes)| crate::shadow_genius_novel::EvidenceFile::from_bytes(bytes.clone()))
                .collect(),
            description: scientific.description.clone(),
            trl,
        };

        let sovereign_pair = verified == 1 && primitive == 1;
        let (eco, novel, innovation) = match sovereign_pair {
            true => crate::shadow_eco::ShadowEco::classify_with_factualization(
                &input, &evidence, ts_ns,
            ),
            false => {
                let base = crate::shadow_eco::ShadowEco::classify(&input);
                let novel = crate::shadow_genius_novel::GeniusNovel::adjudicate(&evidence);
                (base, novel, None)
            }
        };

        // Statusi final nuk krijon autoritet paralel: është derivat i të njëjtit
        // verified/primitive + NovelEvidence brenda verdictit suprem.
        let project_status = match (verified, primitive, innovation.is_some()) {
            (0, 0, _) => shadow_contracts::PROJECT_STATUS_RIGOROUS_NEGATIVE,
            (1, 1, true) => shadow_contracts::PROJECT_STATUS_NOVEL_FACTUAL,
            _ => shadow_contracts::PROJECT_STATUS_HOLD,
        };
        let rejection = match (sovereign_pair, novel.rejection) {
            (false, _) => shadow_contracts::NOVEL_REJECTION_SOVEREIGN_PAIR_NOT_VERIFIED,
            (true, crate::shadow_genius_novel::NovelRejection::None) => shadow_contracts::NOVEL_REJECTION_NONE,
            (true, crate::shadow_genius_novel::NovelRejection::NoVisualProof) => shadow_contracts::NOVEL_REJECTION_NO_VISUAL_PROOF,
            (true, crate::shadow_genius_novel::NovelRejection::DescriptionTooShort) => shadow_contracts::NOVEL_REJECTION_DESCRIPTION_TOO_SHORT,
            (true, crate::shadow_genius_novel::NovelRejection::UnknownFormats) => shadow_contracts::NOVEL_REJECTION_UNKNOWN_FORMATS,
            (true, crate::shadow_genius_novel::NovelRejection::TrlNotReady) => shadow_contracts::NOVEL_REJECTION_TRL_NOT_READY,
        };

        let project_context_sha256 = crate::gcl_project_contract::parse_sha256(
            &scientific.context_sha256,
        ).ok_or_else(|| ShadowError::TransportCorrupt(
            "ProjectContextWitness SHA-256 nuk është kanonik".to_string(),
        ))?;
        let project_evidence_sha256 = crate::gcl_project_contract::parse_sha256(
            &scientific.evidence_sha256,
        ).ok_or_else(|| ShadowError::TransportCorrupt(
            "ScientificProject evidence SHA-256 nuk është kanonik".to_string(),
        ))?;

        Ok(ProjectAdjudication {
            eco_class: eco.class,
            project_id: scientific.project_id,
            project_status,
            project_context_digest: crate::gcl_project_contract::digest64(
                &scientific.context_sha256,
            ),
            project_evidence_digest: crate::gcl_project_contract::digest64(
                &scientific.evidence_sha256,
            ),
            project_context_sha256,
            project_evidence_sha256,
            novel_factualized: sovereign_pair && innovation.is_some(),
            novel_trl_level: novel.trl_level,
            novel_proof_score: (novel.proof_score.clamp(0.0, 1.0)
                * shadow_contracts::MASS_SCALE as f32).round() as u32,
            novel_rejection: rejection,
        })
    }

    /// Porta 1 e Shadow për projektin: identiteti dhe TRL hyrës duhet të
    /// mbeten brenda të njëjtit GCL/Spine. Quantum mund të sjellë maksimumi
    /// TRL3; TRL4 nuk pranohet nga wire-i dhe lind vetëm në portën 4 të Shadow.
    fn verify_project_gcl_stage(
        package: &PassPackage,
        project: &ScientificProjectContext,
    ) -> Result<(), ShadowError> {
        let context_sha_ok = crate::gcl_project_contract::parse_sha256(
            &project.context_sha256,
        ).is_some();
        let evidence_sha_ok = crate::gcl_project_contract::parse_sha256(
            &project.evidence_sha256,
        ).is_some();
        let content_sha_ok = crate::gcl_project_contract::parse_sha256(
            &project.content_sha256,
        ).is_some();
        let sovereign_value =
            (project.light_sovereign_flags & SGL_SEAL_MASK) ^ SGL_SEAL_XOR;
        let trl_is_quantum_bounded =
            project.trl_level <= shadow_contracts::QUANTUM_MAX_TRL;
        let trl_pass_is_coherent = !project.trl_passed || project.trl_level > 0;
        let mass_is_bounded = project.trl_confidence <= shadow_contracts::MASS_SCALE
            && project.trl_reproducibility <= shadow_contracts::MASS_SCALE;
        let same_gcl = package.pd_gcl_process_digest != 0
            && project.gcl_process_digest == package.pd_gcl_process_digest;
        let spine_complete = package.spine_completion_digest != 0;
        let identity_present = project.project_id != 0
            && project.user_id != 0
            && project.trace_id != 0;
        let light_seal_ok = sovereign_value == SGL_SEAL_PRIMITIVE;

        match (
            identity_present,
            same_gcl,
            spine_complete,
            light_seal_ok,
            context_sha_ok,
            evidence_sha_ok,
            content_sha_ok,
            trl_is_quantum_bounded,
            trl_pass_is_coherent,
            mass_is_bounded,
        ) {
            (true, true, true, true, true, true, true, true, true, true) => Ok(()),
            _ => Err(ShadowError::TransportCorrupt(
                "ScientificProject: GCL/Spine/TRL identity gate dështoi".to_string(),
            )),
        }
    }

    fn verify_project_file_kinds(
        project: &ScientificProjectContext,
    ) -> Result<(), ShadowError> {
        let all_match = project.files.iter().all(|(declared, bytes)| {
            let detected = crate::media_formats::FormatDetector::detect_kind(bytes);
            match (*declared, detected) {
                (shadow_contracts::PROJECT_EVIDENCE_IMAGE, crate::media_formats::MediaKind::Image)
                | (shadow_contracts::PROJECT_EVIDENCE_VIDEO, crate::media_formats::MediaKind::Video)
                | (shadow_contracts::PROJECT_EVIDENCE_DOCUMENT, crate::media_formats::MediaKind::Document)
                | (shadow_contracts::PROJECT_EVIDENCE_UNKNOWN, crate::media_formats::MediaKind::Unknown) => true,
                _ => false,
            }
        });
        match all_match {
            true => Ok(()),
            false => Err(ShadowError::TransportCorrupt(
                "NovelEvidence: lloji i deklaruar nuk përputhet me magic bytes".to_string(),
            )),
        }
    }

    /// FAZA 2 — pulsi i vetëm SHA-256 në kulmin e verdiktit.
    /// Kur çifti nuk është kushtetues, verdict-i mbetet i vlefshëm por pa Besim.
    fn seal_living_trust(
        mut verdict: SupremeVerdict,
    ) -> Result<SupremeVerdict, ShadowError> {
        let system_laws_seal =
            crate::lab_contracts::gjata_collapse_law::system_laws_seal();
        let sovereign_value =
            (verdict.living_trust_sovereign_flags & SGL_SEAL_MASK) ^ SGL_SEAL_XOR;
        let l500_contract_sync = SGL_SEAL_XOR
                == crate::living_trust_contract::SOVEREIGN_SEAL_XOR
            && SGL_SEAL_MASK
                == crate::living_trust_contract::SOVEREIGN_SEAL_MASK
            && SGL_SEAL_PRIMITIVE
                == crate::living_trust_contract::SOVEREIGN_SEAL_PRIMITIVE;
        match l500_contract_sync {
            true => {}
            false => return Err(ShadowError::SealInvalid(
                "L-500 nuk është byte-identik me kontratën e Vulës së Gjallë".to_string(),
            )),
        }
        let proof = crate::living_trust_contract::LivingTrustProof {
            action_state: verdict.living_trust_action_state,
            action_mask: verdict.living_trust_action_mask,
            required_action_mask: verdict.living_trust_required_action_mask,
            verified: verdict.verified,
            primitive: verdict.primitive,
            knowledge_band: verdict.knowledge_band.as_u8(),
            lgc_law: verdict.lgc_law,
            system_laws_seal,
            sovereign_flags: verdict.living_trust_sovereign_flags,
            scientific_project_sha256: verdict.project_evidence_sha256,
            scientific_project_verdict_sha256:
                crate::gcl_project_contract::verdict_sha256_or_zero(
                    crate::gcl_project_contract::ProjectVerdictMaterial {
                        project_id: verdict.project_id,
                        project_status: verdict.project_status,
                        project_context_sha256: verdict.project_context_sha256,
                        project_evidence_sha256: verdict.project_evidence_sha256,
                        novel_factualized: verdict.novel_factualized,
                        novel_trl_level: verdict.novel_trl_level,
                        novel_proof_score: verdict.novel_proof_score,
                        novel_rejection: verdict.novel_rejection,
                    },
                ),
        };
        verdict.living_trust_system_laws_seal = system_laws_seal;
        verdict.living_trust_sovereign_value = sovereign_value;
        match crate::living_trust_contract::compute(proof, verdict.legacy_score) {
            Some(seal) => match seal.sovereign_value == sovereign_value
                && sovereign_value == SGL_SEAL_PRIMITIVE {
                true => {
                    verdict.living_trust_sha256 = seal.identity_sha256;
                    verdict.living_trust_intensity = seal.intensity;
                    verdict.living_trust_kind = seal.kind;
                    verdict.living_trust_sovereign_value = seal.sovereign_value;
                    Ok(verdict)
                }
                false => Err(ShadowError::SealInvalid(
                    "L-500 nuk prodhoi vlerën sovrane 500 në verdict".to_string(),
                )),
            },
            None => match proof.constitutional_kind() {
                crate::living_trust_contract::TRUST_KIND_NONE => Ok(verdict),
                _ => Err(ShadowError::SealInvalid(
                    "GCL Vula e Gjallë: action_state/ligjet/L-500 nuk u mbyllën"
                        .to_string(),
                )),
            },
        }
    }

    /// Legacy Score — formula e plotë e maturimit (ZERO if/else).
    pub fn legacy_score_compute(pkg: &PassPackage) -> f32 {
        (pkg.evidence_density     * 0.25
       + pkg.logical_coherence    * 0.20
       + pkg.causal_integrity     * 0.25
       + pkg.convergence_strength * 0.15
       + pkg.reproducibility      * 0.15)
        .clamp(0.0, 1.0)
    }

    /// DSL string për manifest — LGC::SHADOW|{state}|{band}|LAW_{:06b}
    fn emit_dsl(primitive: u8, verified: u8, band: KnowledgeBand, lgc_law: u32) -> String {
        let state = match (primitive, verified) {
            (1, 1) => "PRIMITIVE",
            (0, 1) => "VERIFIED",
            _      => "NEGATIVE",
        };
        format!("LGC::SHADOW|{}|{}|LAW_{:06b}", state, band.label(), lgc_law)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn project_fixture(trl_level: u8) -> (PassPackage, ScientificProjectContext) {
        let project = ScientificProjectContext {
            project_id: 77,
            user_id: 9,
            trace_id: 88,
            revision: 1,
            content_sha256: "aa".repeat(32),
            light_sovereign_flags: 0xA451,
            context_sha256: "bb".repeat(32),
            project_title: "motor novel".into(),
            domain: "science".into(),
            hypothesis: "X prodhon Y".into(),
            assumptions: vec!["A".into()],
            gcl_process_digest: 0x1122,
            trl_level,
            trl_passed: trl_level > 0,
            trl_confidence: 8_000,
            trl_reproducibility: 7_000,
            lab_test_id: "LAB-77".into(),
            findings: "evidence".into(),
            description: "si doli risia dhe si u provua".into(),
            files: Vec::new(),
            evidence_sha256: "cc".repeat(32),
        };
        let package = PassPackage {
            pd_gcl_process_digest: 0x1122,
            spine_completion_digest: 0x3344,
            ..PassPackage::default()
        };
        (package, project)
    }

    #[test]
    fn seal_arithmetic_matches_kernel() {
        // 0xA451 = (500 & 0xFFFF) ^ 0xA5A5
        assert!(ShadowGjLegacy::seal_check(0xA451));
        assert!(!ShadowGjLegacy::seal_check(0x0000));
        assert!(!ShadowGjLegacy::seal_check(500)); // pa XOR
    }

    #[test]
    fn project_gcl_stage_accepts_quantum_trl3_only() {
        let (package, project) = project_fixture(shadow_contracts::QUANTUM_MAX_TRL);
        assert!(ShadowGjLegacy::verify_project_gcl_stage(&package, &project).is_ok());
    }

    #[test]
    fn project_gcl_stage_rejects_inbound_trl4_and_gcl_drift() {
        let (mut package, project) = project_fixture(shadow_contracts::SHADOW_FACTUAL_TRL);
        assert!(ShadowGjLegacy::verify_project_gcl_stage(&package, &project).is_err());

        let (_, trl3) = project_fixture(shadow_contracts::QUANTUM_MAX_TRL);
        package.pd_gcl_process_digest = 0x9999;
        assert!(ShadowGjLegacy::verify_project_gcl_stage(&package, &trl3).is_err());
    }
}
