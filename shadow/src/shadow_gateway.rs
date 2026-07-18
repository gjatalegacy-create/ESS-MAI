// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  SHADOW_GATEWAY.RS — HYRJA E VETME SOVRANE                            ║
// ║  Gjata Legacy™  |  Arkitekt: Bledar Gjata                             ║
// ║                                                                          ║
// ║  `Shadow` — core privat i procesit. Hyrja production: ingest_bridged.  ║
// ║                                                                          ║
// ║  Dy origjinat mbërrijnë VEÇMAS (Quantum → PassPackage, Light →         ║
// ║  LightEnvelope) dhe BASHKOHEN brenda pipeline-it. Shadow është sink-u  ║
// ║  sovran: merr pronësinë (move) dhe asgjë nuk klonohet (LIGJI 0).      ║
// ║                                                                          ║
// ║  Ngrirja → refuzim i fortë (Err), pa punë, pa shkrim.                 ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::bridge::{LightInbound, QuantumInbound};
use crate::lab_contracts::{PrimitiveSplit, NegativePath, GjataCollapseLaw, CollapsePhase};
use crate::verefied_diary_supremelaw::{
    GclPrimitiveMode, GclPrimitiveVerification, VerefiedDiarySupremelaw, VdsEvent,
};
use crate::knowledge_vault::{init_global_vault, KnowledgeVault};
use crate::shadow_gj_legacy::ShadowGjLegacy;
use crate::shadow_pipeline::{now_ns, run_pipeline};
use crate::types::{LightEnvelope, NegativeContext, PassPackage, ShadowError, SupremeVerdict};
use crate::sovereign_ffi_gate::{seal_verified_output, VerificationReceipt};

/// Arsyeja e ruajtjes negative nga kolapsi (lidhur me vulën sovrane 0xA451).
const REASON_COLLAPSE_ELIM: u32 = 0xA451;

/// FNV-1a 64-bit — determinist, zero varësi (claim_id i termave negativë).
fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in bytes.iter() {
        h ^= *b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}
use std::collections::HashMap;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{Arc, Mutex};

/// Dosja ku shënohet ledger-i i rrjedhave. NUK është autoritet — vetëm
/// dëshmi. Në runtime real boot-i sovran e GARANTON env-in (FATAL pa të);
/// në teste (pa boot) përdoret temp-i ABSOLUT i sistemit — kjo s'është
/// "fallback relativ" (asnjë cwd, asnjë rrugë relative) dhe s'prek kurrë
/// asnjë vendim: ledger-i vetëm shënon. Zero if — vetëm match.
fn rrjedha_dir() -> String {
    match std::env::var("ESSMAI_HANDOFF_DIR") {
        Ok(d)  => d,
        Err(_) => std::env::temp_dir().to_string_lossy().into_owned(),
    }
}

/// Përgjigja e Shadow — verdikti suprem + konteksti.
#[derive(Debug, Clone)]
pub struct ShadowResponse {
    /// Sesioni (i zhvendosur nga LightEnvelope — pa klon).
    pub session_id:     String,
    /// Vendimi suprem.
    pub verdict:        SupremeVerdict,
    /// Nëse kjo rrugë ishte tashmë e njohur si negative (kufi aktiv).
    pub known_negative: Option<NegativeContext>,
    /// Output-i final i Shadow i vulosur nga autoriteti i verifikimit.
    pub verification_receipt: VerificationReceipt,
}

/// Gjendja e vendosur nga pipeline-i, por ende e pavulosur. Ky tip është privat:
/// as Quantum, as Light dhe as Nura nuk mund ta shohin para mbylljes GCL.
struct UnsealedShadowDecision {
    session_id:     String,
    verdict:        SupremeVerdict,
    known_negative: Option<NegativeContext>,
}

/// Konteksti faktik që lidh seal-in final me i₀ në pritje dhe XY e gjykuar.
/// Nuk del nga Shadow; vetëm përbërësit e tij të vulosur hyjnë në receipt.
struct VerificationContext {
    session_id:       String,
    parent_i0:        String,
    primitive_anchor: u64,
    xy_digest:        u64,
    pd_binding_digest: u64,
    pd_continuum_activation_digest: u64,
}

impl VerificationContext {
    #[cfg(test)]
    fn direct(session_id: &str, parent_i0: &str, primitive_anchor: u64) -> Self {
        Self::from_xy(
            session_id, parent_i0, primitive_anchor, "", "", 0, "DIRECT",
            0, 0, 0, 0, 0,
        )
    }

    fn from_xy(
        session_id: &str,
        parent_i0: &str,
        primitive_anchor: u64,
        x: &str,
        y: &str,
        mass: u32,
        collapse: &str,
        pd_binding_digest: u64,
        pd_continuum_activation_digest: u64,
        final_evidence_digest: u64,
        pd_gcl_process_digest: u64,
        spine_completion_digest: u64,
    ) -> Self {
        let mut material = Vec::with_capacity(
            parent_i0.len() + x.len() + y.len() + collapse.len() + 40
        );
        material.extend_from_slice(parent_i0.as_bytes());
        material.extend_from_slice(&primitive_anchor.to_le_bytes());
        material.extend_from_slice(x.as_bytes());
        material.push(0x1f);
        material.extend_from_slice(y.as_bytes());
        material.extend_from_slice(&mass.to_le_bytes());
        material.extend_from_slice(collapse.as_bytes());
        material.extend_from_slice(&final_evidence_digest.to_le_bytes());
        material.extend_from_slice(&pd_gcl_process_digest.to_le_bytes());
        material.extend_from_slice(&spine_completion_digest.to_le_bytes());
        let xy_digest = crate::lab_contracts::verification_receipt::fnv1a64(&material);
        Self {
            session_id: session_id.to_string(),
            parent_i0: parent_i0.to_string(),
            primitive_anchor,
            xy_digest,
            pd_binding_digest,
            pd_continuum_activation_digest,
        }
    }
}

/// Vëzhgueshmëria e Shadow.
#[derive(Debug, Clone, Copy)]
pub struct ShadowStats {
    pub total:            u64,
    pub passed:           u64,
    pub blocked:          u64,
    pub frozen:           bool,
    pub primitive_count:  usize,
    pub legacy_count:     usize,
    pub negative_count:   usize,
    pub hypothesis_count: usize,
    pub fact_count:       usize,
    /// VULA E AUDITIMIT (HARDENING): ora reale dështoi ≥1 herë — u përdor
    /// fallback monotonik. Lineage i ruajtur; auditimi njoftohet me zë.
    pub time_degraded:    bool,
    pub verification_ledger_count: usize,
    pub verification_wisdom_count: usize,
}

/// Autoriteti suprem ESS-MAI — memoria sovrane.
#[derive(Debug)]
pub struct Shadow {
    vault:  Arc<KnowledgeVault>,
    frozen: AtomicI32,
    /// PA (Primitive Anchor) i regjistruar HERË nga Light, PARA se Quantum të
    /// ketë mbaruar reasoning-un. session_id → pa_id (TraceInfo.id nga Light).
    /// Hartë e vogël, ephemeral (jo persistent) — vetëm gate-i i hyrjes, jo dije.
    /// PA + (Xi,Yi) NË PRITJE — sinjali i Light: ankora dhe dy hapësirat
    /// primitive presin propozimin XY të Quantum (KOLAPS/3 verifikimi).
    pa_waiting: Mutex<HashMap<String, (u64, PrimitiveSplit)>>,
    /// Multi-verification memory/wisdom; vetëm procesi main.rs e zotëron.
    verification: Mutex<crate::shadow_spine::ShadowVerificationRuntime>,
}

impl Shadow {
    /// Inicializon Shadow: kerneli + vault-i global.
    #[cfg(test)]
    pub fn new() -> Result<Self, ShadowError> {
        ShadowGjLegacy::init()?;
        let vault = Arc::new(KnowledgeVault::new());
        init_global_vault(vault.clone());
        Ok(Self {
            vault,
            frozen: AtomicI32::new(0),
            pa_waiting: Mutex::new(HashMap::new()),
            verification: Mutex::new(crate::shadow_spine::ShadowVerificationRuntime::new()),
        })
    }

    /// FAZA 2 — Krijon Shadow me memorie sovrane DURABLE (WAL në disk).
    /// Ripërsërit gjendjen ekzistuese nga `path` dhe e krijon nëse mungon;
    /// çdo vendim i mëpasshëm shtohet (append) dhe mbijeton restart-in.
    pub fn with_disk<P: AsRef<std::path::Path>>(path: P) -> Result<Self, ShadowError> {
        ShadowGjLegacy::init()?;
        let vault = Arc::new(KnowledgeVault::open_disk(path.as_ref())?);
        init_global_vault(vault.clone());
        Ok(Self {
            vault,
            frozen: AtomicI32::new(0),
            pa_waiting: Mutex::new(HashMap::new()),
            verification: Mutex::new(crate::shadow_spine::ShadowVerificationRuntime::new()),
        })
    }

    /// ENTRYPOINT MINIMAL — regjistron Primitive Anchor (PA) HERË, sapo Light
    /// e dërgon, PARA se Quantum të ketë mbaruar reasoning-un. Idempotent
    /// (mbishkruan nëse session_id përsëritet). S'arsyeton, s'verifikon dije —
    /// thjesht e mban PA-në në pritje deri sa Quantum të vijë në `ingest_bridged`.
    pub fn register_primitive_anchor(
        &self,
        session_id: &str,
        pa_id:      u64,
        split:      PrimitiveSplit,
    ) {
        match self.pa_waiting.lock() {
            Ok(mut map) => { map.insert(session_id.to_string(), (pa_id, split)); }
            Err(poisoned) => {
                // FAIL-LOUD (ULTIMATUM, jo no-op i heshtur): lock i helmuar =
                // një thread paniku. HashMap-i mbetet valid → e RIKUPEROJMË dhe
                // regjistrojmë PA-në (s'e humbim ankorën), me alarm në audit.
                eprintln!("[SHADOW_GATE] ALARM: pa_waiting lock i helmuar — rikuperim + regjistrim PA (session={})", session_id);
                let mut map = poisoned.into_inner();
                map.insert(session_id.to_string(), (pa_id, split));
            }
        }
    }

    /// URA 2 (Light→Shadow) — USHQYESI I PA-GATE-it nga skedari i këmbimit.
    ///
    /// KONTRATA E SKEDARIT (shkruar nga Light, `export_primitive_anchor`):
    ///   e re (5 fusha): "session_id|pa_id_hex|ts_ns|xi:a,b|yi:c\n"
    ///   e vjetër (3):   "session_id|pa_id_hex|ts_ns\n"   — të dyja pranohen.
    ///   pa_id_hex = {:016x}; (Xi,Yi) = kolapsi i koordinimit i Light.
    /// Ky lexues: filtron rreshtat e session-it, merr TË FUNDIT (konsistent me
    /// read_pd_delivery të Light-it), parson hex→u64 dhe e regjistron te gate-i.
    ///
    /// KONTRATË TRANSPORTI (HARDENING — fail-closed):
    ///   • skedari mungon          → Ok(0)  — nisje e parë, normale (i qetë)
    ///   • s'ka rresht për session → Ok(0)  — normale
    ///   • KORRUPSION (fusha të papritura, hex i keq, Xi/Yi wire i keq)
    ///                             → Err(TransportCorrupt) — NDALIM I FORTË.
    /// Korrupsioni i transportit NUK është "asnjë ankorë": është shkelje
    /// kontrate — sesioni refuzohet PARA ingest. Zero if/else — vetëm match.
    pub fn feed_primitive_anchors(
        &self,
        path:       &str,
        session_id: &str,
    ) -> Result<usize, ShadowError> {
        // DIAGNOZË, JO HESHTJE (v1.3.2): "skedari mungon" është e VETMJA
        // gjendje normale (nisje e parë). Leje e mohuar / I/O reale = rrjedhë
        // me PSE + shënim + kufi — s'maskohet kurrë si "asnjë ankorë".
        let content = match std::fs::read_to_string(path) {
            Ok(c)  => c,
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => return Ok(0), // nisje e parë — normale
                other_kind => {
                    let v = crate::lab_contracts::rrjedha::note(
                        &rrjedha_dir(), "SHADOW", "gateway::feed_primitive_anchors",
                        crate::lab_contracts::rrjedha::RrjedhaClass::DiskDenied,
                        &format!("leximi i {} dështoi: {:?} ({})", path, other_kind, e));
                    return Err(ShadowError::TransportCorrupt(format!(
                        "disku refuzoi leximin e ankorave ({:?}) — jo 'mungesë', por dështim i deklaruar [{:?}]",
                        other_kind, v)));
                }
            },
        };
        let prefix = format!("{}|", session_id);
        // KUFIRI ANTIDEGRADIM (v1.3.2): sapo sesioni ka NJË rresht të vulosur
        // (c:), rrëshqitja mbrapa në legacy është e dyshimtë — cungim i fushës
        // CRC do të dukej pikërisht kështu. Çelës njëdrejtimësh: i vulosur ⇒
        // përgjithmonë i vulosur për atë sesion.
        let sealed_exists = content
            .lines()
            .filter(|l| l.starts_with(prefix.as_str()))
            .any(|l| l.contains("|c:"));
        let last = content
            .lines()
            .filter(|l| l.starts_with(prefix.as_str()))
            .last();
        match last {
            None => Ok(0), // s'ka rresht për këtë session — normale
            Some(line) => {
                // ══ PACP GATE (kontrata pa_wire) ═══════════════════════════
                // Ankora duhet të mbërrijë BYTE-PËR-BYTE siç u shkrua nga
                // Light: 6 fusha → CRC i detyruar; 3/5 → legacy i deklaruar;
                // çdo korrupsion → TransportCorrupt PARA çdo interpretimi.
                match (crate::lab_contracts::pa_wire::verify_line(line), sealed_exists) {
                    (crate::lab_contracts::pa_wire::PaWireVerdict::SealedOk, _) => {
                        println!("  [PACP]     ankora e vulosur CRC-OK (session={}) — i₀ i pandryshuar në urë", session_id);
                    }
                    // ÇELËSI NJËDREJTIMËSH: legacy pranohet VETËM kur sesioni
                    // s'ka pasur KURRË rresht të vulosur — përndryshe ky është
                    // degradim (cungim i c:) dhe merr PSE + shënim + refuzim.
                    (crate::lab_contracts::pa_wire::PaWireVerdict::LegacyOk, false) => {
                        println!("  [PACP]     ankora legacy (pa CRC) e pranuar e deklaruar (session={})", session_id);
                        // v1.4.4 (legacy i përforcuar): çdo pranim legacy lë GJURMË
                        // të vulosur + kufi këshillues (Other=5) — trashëgimia
                        // pranohet, por kurrë më në errësirë.
                        let _ = crate::lab_contracts::rrjedha::note(
                            &rrjedha_dir(), "SHADOW", "gateway::feed_primitive_anchors",
                            crate::lab_contracts::rrjedha::RrjedhaClass::Other,
                            &format!("LEGACY_PRANUAR kind=PA bridge=light_pa_export session={session_id} sealed_para=false"));
                        crate::shadow_runtime_pulse::mark_ready(
                            crate::shadow_runtime_pulse::SStage::LegacyWritten);
                    }
                    (crate::lab_contracts::pa_wire::PaWireVerdict::LegacyOk, true) => {
                        let v = crate::lab_contracts::rrjedha::note(
                            &rrjedha_dir(), "SHADOW", "gateway::feed_primitive_anchors",
                            crate::lab_contracts::rrjedha::RrjedhaClass::TransportDowngrade,
                            &format!("rresht legacy PAS rreshti të vulosur (session={}) — fusha CRC e cunguar?", session_id));
                        return Err(ShadowError::TransportCorrupt(format!(
                            "PACP: DEGRADIM — sesioni {} ka histori të vulosur; legacy s'pranohet më (kufiri {:?})",
                            session_id, v)));
                    }
                    (crate::lab_contracts::pa_wire::PaWireVerdict::Corrupt(why), _) => {
                        // PSE-ja e plotë: dallohet cungimi (fusha mungojnë) nga
                        // kthimi i biteve (CRC s'përputhet) — diagnozë, jo mjegull.
                        let klasa = match why.contains("përputhet") {
                            true  => crate::lab_contracts::rrjedha::RrjedhaClass::TransportBitflip,
                            false => crate::lab_contracts::rrjedha::RrjedhaClass::TransportTruncated,
                        };
                        let _ = crate::lab_contracts::rrjedha::note(
                            &rrjedha_dir(), "SHADOW", "gateway::feed_primitive_anchors",
                            klasa, &format!("{} (session={})", why, session_id));
                        return Err(ShadowError::TransportCorrupt(format!(
                            "PACP: {} (session={}) — transporti PA refuzohet", why, session_id,
                        )));
                    }
                }
                // Hiq fushën CRC (nëse ekziston) — interpretimi vijues punon
                // mbi trupin kanonik 3/5-fushësh, i provuar tashmë nga gate.
                let body: &str = match line.rsplit_once("|c:") {
                    Some((b, _)) => b,
                    None         => line,
                };
                let parts: Vec<&str> = body.split('|').collect();
                // 3 fusha (e vjetër, split bosh) OSE 5 (me Xi/Yi) — të dyja.
                let (sid, hex, split) = match parts.as_slice() {
                    [sid, hex, _ts] => (
                        *sid, *hex,
                        PrimitiveSplit { xi: Vec::new(), yi: Vec::new() },
                    ),
                    [sid, hex, _ts, xi_seg, yi_seg] => {
                        match PrimitiveSplit::from_wire(xi_seg, yi_seg) {
                            Some(p) => (*sid, *hex, p),
                            None    => return Err(ShadowError::TransportCorrupt(format!(
                                "Xi/Yi wire i keqformuar (session={}) — transporti PA refuzohet",
                                session_id,
                            ))),
                        }
                    }
                    _ => return Err(ShadowError::TransportCorrupt(format!(
                        "format PA i papritur ({} fusha, session={}) — transporti PA refuzohet",
                        parts.len(), session_id,
                    ))),
                };
                match u64::from_str_radix(hex.trim(), 16) {
                    Ok(pa_id) => {
                        self.register_primitive_anchor(sid, pa_id, split);
                        Ok(1)
                    }
                    Err(_) => Err(ShadowError::TransportCorrupt(format!(
                        "pa_id hex i keqformuar '{}' (session={}) — transporti PA refuzohet",
                        hex.trim(), session_id,
                    ))),
                }
            }
        }
    }

    /// Hyrje direkte VETËM PËR TEST. Production kalon nga main/process_bridge.
    ///
    /// LIGJI 0: asnjë klon i inputit. Në ngrirje → Err pa asnjë veprim.
    #[cfg(test)]
    pub fn ingest(
        &self,
        pkg:       PassPackage,
        light:     LightEnvelope,
    ) -> Result<ShadowResponse, ShadowError> {
        let parent_i0 = light.session_id.clone();
        let primitive_anchor = match self.pa_waiting.lock() {
            Ok(map)       => map.get(&parent_i0).map(|(pa, _)| *pa).unwrap_or(0),
            Err(poisoned) => poisoned.into_inner()
                .get(&parent_i0).map(|(pa, _)| *pa).unwrap_or(0),
        };
        let context = VerificationContext::direct(&parent_i0, &parent_i0, primitive_anchor);
        let decision = self.ingest_unsealed(
            pkg, light, GclPrimitiveVerification::direct_test(primitive_anchor),
        )?;
        // Rruga direkte: ankora ende jeton në pa_waiting; mbyll Y→X para seal-it.
        self.gcl_reinforce_on_verified(&decision.session_id, &decision.verdict);
        self.seal_decision(decision, context)
    }

    /// Pipeline i brendshëm: vendos Y/X por nuk prodhon receipt publik.
    /// Përdoret nga rruga direkte dhe nga rruga bridged, që secila të mbyllë
    /// ankorën e vet para se output-i të vuloset.
    fn ingest_unsealed(
        &self,
        pkg: PassPackage,
        mut light: LightEnvelope,
        primitive_proof: GclPrimitiveVerification,
    ) -> Result<UnsealedShadowDecision, ShadowError> {
        match self.is_frozen() {
            true => return Err(ShadowError::Frozen(
                "Shadow është i ngrirë; asnjë vendim i ri nuk pranohet".to_string(),
            )),
            false => {}
        }

        let ts = now_ns();
        let known_negative = self
            .vault
            .check_known_negative(pkg.epistemic_mass, &light.territory);
        let input_id = crate::shadow_pipeline::stable_id(&pkg.package_id);
        // Stage memory: vetëm një pipeline i suksesshëm e angazhon historinë.
        let mut runtime = match self.verification.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                eprintln!("[SHADOW_VERIFY] ALARM: verification runtime lock i helmuar — rikuperim fail-loud");
                poisoned.into_inner()
            }
        };
        let mut staged = runtime.clone();
        let attestation = crate::shadow_spine::ShadowSpine::adjudicate_runtime(
            &mut staged, input_id, &pkg, &light, primitive_proof, ts,
        );
        let session_id = std::mem::take(&mut light.session_id);
        let verdict = run_pipeline(pkg, light, &self.vault, attestation, ts)?;
        *runtime = staged;
        crate::shadow_runtime_pulse::mark_ready(crate::shadow_runtime_pulse::SStage::Judge);

        Ok(UnsealedShadowDecision { session_id, verdict, known_negative })
    }

    /// Seal-i final: thirret vetëm pasi rruga përkatëse ka mbyllur Y→X.
    fn seal_decision(
        &self,
        decision: UnsealedShadowDecision,
        context: VerificationContext,
    ) -> Result<ShadowResponse, ShadowError> {
        match (context.session_id == decision.session_id, context.parent_i0.is_empty()) {
            (true, false) => {}
            (false, _) => return Err(ShadowError::SealInvalid(
                "verification context nuk i përket session-it të vendimit".to_string()
            )),
            (_, true) => return Err(ShadowError::SealInvalid(
                "verification context ka parent i₀ bosh".to_string()
            )),
        }
        let verification_receipt = seal_verified_output(
            &decision.session_id,
            &context.parent_i0,
            context.primitive_anchor,
            context.xy_digest,
            context.pd_binding_digest,
            context.pd_continuum_activation_digest,
            &decision.verdict,
        ).map_err(|e| ShadowError::SealInvalid(
            format!("verification output seal dështoi: {e:?}")
        ))?;

        Ok(ShadowResponse {
            session_id: decision.session_id,
            verdict: decision.verdict,
            known_negative: decision.known_negative,
            verification_receipt,
        })
    }

    /// GCL LIVE: Y i verifikuar (D=1) → Trust(X) propagohet te ankora PA e
    /// sesionit në vault (real_hits+1, me kaskadë promovimi hipotezë→fakt).
    /// Thirret VETËM pas verdiktit — kurrë para (rendi Y→X është ligj).
    /// Jashtë zonës së ndaluar; vetëm konsumon verdiktin, s'e ndryshon atë.
    #[cfg(test)]
    fn gcl_reinforce_on_verified(&self, session_id: &str, verdict: &SupremeVerdict) {
        let anchor = match self.pa_waiting.lock() {
            Ok(map)       => map.get(session_id).map(|(pa, _)| *pa),
            Err(poisoned) => poisoned.into_inner().get(session_id).map(|(pa, _)| *pa),
        };
        match anchor {
            Some(pa_id) => self.gcl_apply(session_id, pa_id, verdict),
            None        => {} // s'ka ankorë NË PRITJE — normale për këtë rrugë;
                              // rruga e urës e mbart ankorën vetë (gcl_anchor).
        }
    }

    /// THELBI i lakut GCL — i vetmi vend ku Trust(X) propagohet realisht.
    /// Thirret nga DY rrugë, me GARANCI kundër numërimit të dyfishtë:
    ///   1. `ingest` (lookup në pa_waiting) — për thirrës direktë me feed;
    ///   2. `ingest_bridged` (ankora e mbartur) — pas verdiktit; aty ankora
    ///      tashmë ËSHTË HEQUR nga pa_waiting, prandaj rruga 1 është e
    ///      garantuar no-op → saktësisht NJË rifortësim për sesion. Zero if.
    fn gcl_apply(&self, session_id: &str, pa_id: u64, verdict: &SupremeVerdict) {
        match verdict.is_verified() {
            true => {
                let promoted = self.vault.confirm_primitive(pa_id, 1);
                crate::shadow_runtime_pulse::mark_ready(crate::shadow_runtime_pulse::SStage::GclApply);
                println!(
                    "  [GCL_LIVE] Verified(Y)=1 → Trust(X): ankora i₀ (pa_id={:016x}, session={}) u rifortësua real_hits+1 (promovime={})",
                    pa_id, session_id, promoted
                );
            }
            false => {
                // Y s'u verifikua → X NUK merr besim; ankora mbetet e paprekur
                // (dega negative trajtohet nga judge_supreme/write_negative).
                println!(
                    "  [GCL_LIVE] Verified(Y)=0 → Trust(X) NUK propagohet (pa_id={:016x}, session={})",
                    pa_id, session_id
                );
            }
        }

        // ══ URA 8 — MONTIMI I VEKTORIT TRL (v1.4.6, audit p.2 PRANUAR) ══════
        // Dimensioni VERIFICATION nga SINJALI REAL i verdiktit: `legacy_score`
        // ∈[0,1] (evidence_density·logical_coherence·causal_integrity·
        // convergence_strength·reproducibility) — burimi i njëjtë familjar që
        // ushqen TrlVerdict.trl_score, tashmë i pranishëm në SupremeVerdict.
        // Proxy 0.90/0.30 u zëvendësua: kur Verified(Y)=1 marrim forcën REALE
        // të legacy_score; kur =0, verifikimi s'mbështetet → dysheme 0.30.
        // Zero if — match mbi (verified, a>0). Shadow mbetet i vetmi që sheh fundin.
        let ver_conf = match (verdict.is_verified(), verdict.legacy_score > 0.0) {
            (true, true)  => verdict.legacy_score,   // forca REALE e verifikimit
            (true, false) => 0.50_f32,               // verified por pa sinjal legacy → TRL2 i kujdesshëm
            (false, _)    => 0.30_f32,               // s'u verifikua → dysheme (TRL1)
        };
        let ver_lvl  = crate::lab_contracts::trl::TrlVector::trl_from_confidence(ver_conf);
        println!("  [TRL_VEKTOR] ver_conf={:.3} (legacy_score={:.3}, verified={}) → ver_lvl={}",
            ver_conf, verdict.legacy_score, verdict.is_verified(), ver_lvl);
        Self::shkruaj_trl_dim(session_id, "ver", ver_lvl);
        let prefiksi = format!("{}|", session_id);
        let (mut in_l, mut rsn_l, mut ver_l): (u8, u8, u8) = (0, 0, 0);
        match std::fs::read_to_string(
            format!("{}/trl_vector.txt", rrjedha_dir().trim_end_matches(['/', '\\']))) {
            Err(_) => {}
            Ok(c)  => {
                for rreshti in c.lines().filter(|l| l.starts_with(prefiksi.as_str())) {
                    let trupi: &str = match crate::lab_contracts::pa_wire::verify_line_generic(rreshti, &[3], 4) {
                        crate::lab_contracts::pa_wire::PaWireVerdict::SealedOk =>
                            match rreshti.rsplit_once("|c:") { Some((b, _)) => b, None => rreshti },
                        crate::lab_contracts::pa_wire::PaWireVerdict::LegacyOk => rreshti,
                        crate::lab_contracts::pa_wire::PaWireVerdict::Corrupt(_) => continue,
                    };
                    let pjeset: Vec<&str> = trupi.split('|').collect();
                    match pjeset.as_slice() {
                        [_sid, dim, val] => {
                            let lvl = match val.trim().parse::<u8>() { Ok(v) => v, Err(_) => continue };
                            match *dim {
                                "in"  => { in_l  = lvl; }
                                "rsn" => { rsn_l = lvl; }
                                "ver" => { ver_l = lvl; }
                                _     => {}
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        let vektori = crate::lab_contracts::trl::TrlVector::new(in_l, rsn_l, ver_l);
        crate::shadow_runtime_pulse::mark_ready(crate::shadow_runtime_pulse::SStage::TrlVerdict);
        match vektori.is_complete() {
            true  => println!("  [TRL_VEKTOR] i PLOTË (session={}) — {} (deeptech: zinxhiri foli i tëri)",
                         session_id, vektori.label()),
            false => println!("  [TRL_VEKTOR] i pjesshëm (session={}) — {} (dimension mungon — rendi i ndezjes)",
                         session_id, vektori.label()),
        }
    }

    /// URA 8 (v1.4.5): shkruan një dimension TRL të vulosur në trl_vector.txt.
    fn shkruaj_trl_dim(session_id: &str, dim: &str, lvl: u8) {
        let body = format!("{}|{}|{}", session_id, dim, lvl);
        let line = crate::lab_contracts::pa_wire::seal_body(&body);
        use std::io::Write as _;
        let path = format!("{}/trl_vector.txt", rrjedha_dir().trim_end_matches(['/', '\\']));
        match std::fs::OpenOptions::new().create(true).append(true).open(&path) {
            Ok(mut f) => match f.write_all(line.as_bytes()).and_then(|_| f.flush()) {
                Ok(())  => {}
                Err(e)  => eprintln!("  [TRL_VEKTOR] ALARM: dimensioni '{dim}' s'u shkrua ({e})"),
            },
            Err(e) => eprintln!("  [TRL_VEKTOR] ALARM: {} s'u hap ({e})", path),
        }
    }

    /// Provon që parent i₀ i PD-së është identiteti i lindur nga PA-ja që
    /// Shadow kishte në pritje. session_id është vetëm identitet cikli.
    fn parent_i0_matches_anchor(parent_i0: Option<&str>, anchor: Option<u64>) -> bool {
        match (parent_i0, anchor) {
            (Some(parent), Some(pa_id)) => {
                let expected = format!("i0-{pa_id:016x}");
                parent == expected.as_str()
            }
            _ => false,
        }
    }

    /// FAZA 3 — Hyrje me DY KANALE: adapton tipat kufitarë të urave
    /// (Quantum `QuantumInbound` + Light `LightInbound`) → tipat e brendshëm,
    /// pastaj i kalon te `ingest`. Origjinat mbeten të ndara deri brenda passage-it.
    pub fn ingest_bridged(
        &self,
        mut q: QuantumInbound,
        l: LightInbound,
) -> Result<ShadowResponse, ShadowError> {
        // Heart-i HCP regjistrohet në HYRJEN AKTIVE të main/process bridge,
        // jo në një helper convenience pa caller production.
        crate::bridge::record_hcp_heart(&q);
        // RUNTIME HARDENING (      , fail-closed): transport Light i gjymtuar
        // REFUZOHET para ingest. Asnjë sukses i heshtur. Zero if — match.
        match l.is_valid() {
            true => {}
            false => return Err(ShadowError::SealInvalid(
                format!("light_inbound i pavlefshëm: {}", l.reason_invalid())
            )),
        }

        // ══ KOLAPS/3 — VERIFIKIMI (formula: XY ↔ (Xi,Yi) → D ∈ {0,1}) ══════
        // Nxirren me MOVE (take — zero clone) fushat e kolapsit PARA se q të
        // konsumohet nga into_pass_package. Sjellje e vjetër e paprekur kur
        // fushat janë None/bosh (backward-safe si modeli HCP).
        let xy_x             = q.xy_x.take();
        let xy_y             = q.xy_y.take();
        let xy_mass          = q.xy_mass.take();
        let pd_binding_digest = q.pd_binding_digest.take().unwrap_or(0);
        let pd_continuum_activation_digest = q.pd_continuum_activation_digest.take().unwrap_or(0);
        let pd_parent_i0 = q.pd_parent_i0.take();
        let final_evidence_digest = q.final_evidence_digest;
        let pd_gcl_process_digest = q.pd_gcl_process_digest;
        let spine_completion_digest = q.spine_completion_digest;
        let collapse_refused = q.collapse_refused.take();
        let negative_wire    = std::mem::take(&mut q.negative_paths);
        let ingest_mass      = q.epistemic_mass;
        let session_for_diary = q.session_id.clone();

        // ── MUSKULI verefied_diary_supremelaw — ditari strukturor i gjykimit.
        // GCL/ESS-MAI aktivizon muskulin; ligjet/gjykatat vetëm i shërbejnë.
        // I njëjti i₀/PA, Xi/Yi dhe transformim Quantum futen si provë e tipizuar
        // në multi-verifikim — jo si kontroll paralel dhe jo si verdict i dytë.
        let mut vds = match VerefiedDiarySupremelaw::commission(
            GjataCollapseLaw::issue(CollapsePhase::Verification)) {
            Ok(m)  => m,
            Err(b) => return Err(ShadowError::SealInvalid(
                format!("gjata_collapse_law: {}", b.text))),
        };

        // PA-GATE: Light regjistroi herët PA + (Xi,Yi); propozimi i Quantum
        // e gjen dhe e konsumon me pronësi të vetme.
        let anchor = match self.pa_waiting.lock() {
            Ok(mut map)   => map.remove(&q.session_id),
            Err(poisoned) => {
                let _ = crate::lab_contracts::rrjedha::note(
                    &rrjedha_dir(), "SHADOW", "gateway::ingest_bridged",
                    crate::lab_contracts::rrjedha::RrjedhaClass::LockPoisoned,
                    "pa_waiting i helmuar — rikuperim me into_inner, ankora u shpëtua");
                poisoned.into_inner().remove(&q.session_id)
            }
        };
        let gcl_anchor: Option<u64> = anchor.as_ref().map(|(pa, _)| *pa);
        let parent_i0 = pd_parent_i0.as_deref().unwrap_or(&q.session_id);
        // Quantum e ndërton parent_i0 nga PA real: `i0-{pa_id:016x}`.
        // session_id identifikon ciklin dhe NUK duhet të shkrihet me i₀.
        let parent_i0_matches = Self::parent_i0_matches_anchor(
            pd_parent_i0.as_deref(),
            gcl_anchor,
        );
        let transformation_traced =
            final_evidence_digest != 0
                && pd_binding_digest != 0
                && pd_continuum_activation_digest != 0
                && spine_completion_digest != 0;
        let gcl_continuity = pd_gcl_process_digest != 0
            && parent_i0_matches
            && transformation_traced;
        let verification_context = VerificationContext::from_xy(
            &q.session_id,
            parent_i0,
            gcl_anchor.unwrap_or(0),
            xy_x.as_deref().unwrap_or(""),
            xy_y.as_deref().unwrap_or(""),
            xy_mass.unwrap_or(0),
            collapse_refused.as_deref().unwrap_or("ACCEPTED"),
            pd_binding_digest,
            pd_continuum_activation_digest,
            final_evidence_digest,
            pd_gcl_process_digest,
            spine_completion_digest,
        );

        // Multi-verifikimi primitiv i v1.4.6, i përshtatur në kontratat e v1.6.9:
        //   i₀/PA → Xi/Yi → propozimi/refuzimi Quantum → procesi → struktura.
        // Partial XY është korrupsion; refuzimi i mbartur është propozim i ligjshëm
        // por final_bit=0. Rruga legacy ruhet e deklaruar, jo e maskuar si Bound.
        let (proof_mode, xi_verified, yi_verified, proposal_received) =
            match (&anchor, &xy_x, &xy_y, &collapse_refused) {
                (Some((pa_id, split)), Some(x), Some(y), None) => {
                    match split.has_material() {
                        true => {
                            let x_ok = split.xi.iter().any(|t| t == x);
                            let y_ok = split.yi.iter().any(|t| t == y);
                            match x_ok & y_ok {
                                true => {
                                    println!(
                                        "  [KOLAPS/3] session={} anchor={:016x} XY↔(Xi,Yi) PËRPUTHJE: ({} ↔ {}) mass_fixed={}",
                                        q.session_id, pa_id, x, y, xy_mass.unwrap_or(0));
                                    for event in [
                                        VdsEvent::TrustAnchor,
                                        VdsEvent::ReceiveProposal,
                                        VdsEvent::JudgeProcess,
                                    ] {
                                        match vds.advance(event) {
                                            Ok(_) => {}
                                            Err(b) => return Err(ShadowError::SealInvalid(
                                                format!("VDS process breach: {}", b.text))),
                                        }
                                    }
                                    (GclPrimitiveMode::Bound, true, true, true)
                                }
                                false => return Err(ShadowError::SealInvalid(format!(
                                    "KOLAPS/3: XY=({},{}) JASHTË hapësirave të ankoruara (x∈Xi:{} y∈Yi:{}) — fail-closed",
                                    x, y, x_ok, y_ok))),
                            }
                        }
                        false => {
                            println!(
                                "  [KOLAPS/3] session={} anchor={:016x} pa material (Xi,Yi) → LEGACY_COMPATIBLE e deklaruar",
                                q.session_id, pa_id);
                            (GclPrimitiveMode::LegacyCompatible, false, false, true)
                        }
                    }
                }
                (None, Some(x), Some(y), None) =>
                    return Err(ShadowError::SealInvalid(format!(
                        "KOLAPS/3: XY=({},{}) mbërriti pa i₀/PA në pritje — fail-closed",
                        x, y
                    ))),
                (Some((pa_id, _)), None, None, Some(reason)) => {
                    println!(
                        "  [KOLAPS/3] session={} anchor={:016x} REFUZIM i mbartur nga Quantum: {}",
                        q.session_id, pa_id, reason);
                    for event in [
                        VdsEvent::TrustAnchor,
                        VdsEvent::ReceiveProposal,
                        VdsEvent::JudgeProcess,
                    ] {
                        match vds.advance(event) {
                            Ok(_) => {}
                            Err(b) => return Err(ShadowError::SealInvalid(
                                format!("VDS refusal breach: {}", b.text))),
                        }
                    }
                    (GclPrimitiveMode::Refused, false, false, true)
                }
                (_, Some(_), None, _) | (_, None, Some(_), _) =>
                    return Err(ShadowError::SealInvalid(
                        "KOLAPS/3: propozim XY i pjesshëm — X dhe Y duhet të vijnë bashkë".to_string()
                    )),
                (None, None, None, Some(reason)) =>
                    return Err(ShadowError::SealInvalid(format!(
                        "KOLAPS/3: refuzimi Quantum '{}' nuk ka i₀/PA në pritje", reason
                    ))),
                (Some((pa_id, _)), None, None, None) => {
                    println!(
                        "  [SHADOW PA-GATE] session={} anchor={:016x} pa XY/refuzim → LEGACY_COMPATIBLE",
                        q.session_id, pa_id);
                    (GclPrimitiveMode::LegacyCompatible, false, false, true)
                }
                (None, None, None, None) =>
                    (GclPrimitiveMode::LegacyCompatible, false, false, true),
                (_, Some(_), Some(_), Some(_)) =>
                    return Err(ShadowError::SealInvalid(
                        "KOLAPS/3: Quantum dërgoi njëkohësisht XY dhe collapse_refused".to_string()
                    )),
            };

        // DIJA NEGATIVE = ASET: eliminimet e Quantum ruhen si precedent negativ.
        // StructureHeld lejohet vetëm kur çdo aset negativ u sistemua realisht.
        let neg_ts = now_ns();
        let mut negative_assets_held = true;
        let mut negative_assets_persisted: usize = 0;
        for wire in negative_wire.iter() {
            match NegativePath::from_wire(wire) {
                Some(np) => {
                    let claim_id = fnv1a64(np.term.as_bytes());
                    let process  = format!("KOLAPS/2:-{}({})", np.space, np.reason);
                    match self.vault.apply_negative(
                        claim_id, REASON_COLLAPSE_ELIM, process,
                        ingest_mass, 1, np.term.into_bytes(), neg_ts,
                    ) {
                        Ok(_)  => {
                            negative_assets_persisted = negative_assets_persisted.saturating_add(1);
                        }
                        Err(e) => {
                            negative_assets_held = false;
                            eprintln!(
                                "  [KOLAPS/3] ALARM: rruga negative s'u vulos në vault ({e:?}) — StructureHeld refuzohet"
                            );
                        }
                    }
                }
                None => {
                    negative_assets_held = false;
                    eprintln!(
                        "  [KOLAPS/3] ALARM KORRUPSION: negative wire i keqformuar — StructureHeld refuzohet"
                    );
                }
            }
        }
        match negative_wire.is_empty() {
            false => println!(
                "  [KOLAPS/3] session={} negative_total={} persisted={} structure_held={}",
                q.session_id,
                negative_wire.len(),
                negative_assets_persisted,
                negative_assets_held,
            ),
            true => {}
        }

        let primitive_proof: GclPrimitiveVerification = match proof_mode {
            GclPrimitiveMode::Bound | GclPrimitiveMode::Refused => {
                match negative_assets_held {
                    true => {}
                    false => return Err(ShadowError::SealInvalid(
                        "VDS StructureHeld refuzuar: asetet negative nuk u sistemuan plotësisht".to_string()
                    )),
                }
                match vds.advance(VdsEvent::HoldStructure) {
                    Ok(_) => {}
                    Err(b) => return Err(ShadowError::SealInvalid(
                        format!("VDS HoldStructure breach: {}", b.text))),
                }
                match vds.attest_structure(
                    proof_mode,
                    gcl_anchor.unwrap_or(0),
                    shadow_contracts::digest_parts(&[parent_i0.as_bytes()]),
                    verification_context.xy_digest,
                    pd_binding_digest,
                    pd_continuum_activation_digest,
                    gcl_anchor.is_some(),
                    parent_i0_matches,
                    xi_verified,
                    yi_verified,
                    proposal_received,
                    transformation_traced,
                    gcl_continuity,
                ) {
                    Ok(proof) => proof,
                    Err(b) => return Err(ShadowError::SealInvalid(
                        format!("VDS primitive attestation breach: {}", b.text))),
                }
            }
            GclPrimitiveMode::LegacyCompatible =>
                vds.attest_legacy_compatible(),
            GclPrimitiveMode::DirectTest =>
                vds.attest_legacy_compatible(),
        };

        let decision = self.ingest_unsealed(
            q.into_pass_package(), l.into_envelope(), primitive_proof,
        );

        // ══ GCL LIVE — RRUGA E URËS (rregullimi kurorë i v1.3.2) ═══════════
        // GJETJA: ankora u HOQ nga pa_waiting më lart (KOLAPS/3 e kërkon me
        // pronësi); lookup-i brenda `ingest` e gjente None → laku Y→X NUK
        // mbyllej KURRË në rrugën reale Light→feed→ingest_bridged, ndonëse
        // testi i njësisë (thirrje direkte) kalonte. Rregullimi: ankora e
        // mbartur (gcl_anchor) zbatohet KËTU, PAS verdiktit — rendi Y→X
        // mbetet ligj, dhe heqja e mëparshme garanton saktësisht NJË
        // rifortësim (lookup-i i brendshëm = no-op i provuar). Zero if.
        match (&decision, gcl_anchor) {
            (Ok(r), Some(pa_id)) => self.gcl_apply(&r.session_id, pa_id, &r.verdict),
            (Err(_), Some(pa_id)) => {
                let _ = crate::lab_contracts::rrjedha::note(
                    &rrjedha_dir(), "SHADOW", "gateway::ingest_bridged",
                    crate::lab_contracts::rrjedha::RrjedhaClass::AnchorLost,
                    &format!("verdikti dështoi para lakut GCL (pa_id={:016x}) — asnjë propagim, e shënuar", pa_id));
            }
            _ => {} // pa ankorë — rrugë e vjetër, asgjë për të rifortësuar
        }

        // ══ LAW 0 (Verification Collapse) ═══════════════════════════════════
        // DiaryVerdict dhe kolapsi 1→0 lindin vetëm kur pipeline-i prodhoi
        // verdikt real (D=0 ose D=1). ShadowError teknik nuk maskohet si verdict.
        let decision_completed = decision.is_ok();
        match (decision_completed, primitive_proof.mode) {
            (true, GclPrimitiveMode::Bound | GclPrimitiveMode::Refused) => {
                match vds.advance(VdsEvent::SealDiary) {
                    Ok(_) => println!("  {}", vds.diary_line(&session_for_diary)),
                    Err(b) => return Err(ShadowError::SealInvalid(
                        format!("VDS SealDiary breach: {}", b.text))),
                }
            }
            (true, GclPrimitiveMode::LegacyCompatible | GclPrimitiveMode::DirectTest) => println!(
                "  [VDS-DIARY] session={} gjendja=LEGACY_COMPATIBLE (pa maskim si provë Bound)",
                session_for_diary
            ),
            (false, _) => eprintln!(
                "  [VDS-DIARY] session={} JO E VULOSUR: pipeline-i nuk prodhoi verdict",
                session_for_diary
            ),
        }

        let mut law0 = crate::lab_contracts::UncertaintyLedger::new();
        match decision_completed {
            true => match law0.record(crate::lab_contracts::CollapsePhase::Verification,
                                      "kolaps3_D", 1.0, 0.0) {
                Ok(()) => println!(
                    "  [LAW0]     Verification Collapse 1→0 (D u vendos) | is_collapsed={}",
                    law0.is_collapsed()
                ),
                Err(v) => eprintln!("  [LAW0] {}", v.report()),
            },
            false => {}
        }
        // Vetëm tani output-i është përfundimtar: pipeline + anchor Y→X +
        // ditari + Verification Collapse janë mbyllur. Këtu lind token-i final.
        decision.and_then(|d| self.seal_decision(d, verification_context))
    }

    /// FAZA 3 — Hyrje nga NJË burim Quantum: ndan paketën në (propozim, transport)
    /// si rrjedha reale Quantum→Light→Shadow, pastaj `ingest`.
    #[cfg(test)]
    pub fn ingest_quantum(&self, q: QuantumInbound) -> Result<ShadowResponse, ShadowError> {
        let (pkg, light) = crate::bridge::split_and_record_heart(q);
        self.ingest(pkg, light)
    }

    /// on_negative — pranon VNK nga Quantum Lab (P3: zinxhiri i plotë Lab→Shadow).
    /// Shadow është i vetmi autoritet që ruan njohuri negative persistente. Zero if.
    pub fn on_negative(
        &self,
        input_id:  u64,
        reason:    u32,
        process:   &str,
        mass:      f32,
        frequency: u32,
        bytes:     &[u8],
        ts_ns:     u64,
    ) -> Result<(), ShadowError> {
        crate::shadow_runtime_pulse::mark_ready(crate::shadow_runtime_pulse::SStage::NegativeVault);
        self.vault.apply_negative(
            input_id, reason, process.to_string(), mass, frequency, bytes.to_vec(), ts_ns,
        )
    }

    /// export_negative_knowledge — eksporton NK si bytes (P3: për PRO Vault).
    /// Format: count(4) | [len(4) | entry_bytes]*. Thirret nga Quantum pas cikli. Zero if.
    pub fn export_negative_knowledge(&self) -> Vec<u8> {
        let entries = self.vault.export_negative();
        let mut out = Vec::new();
        out.extend_from_slice(&(entries.len() as u32).to_le_bytes());
        for e in &entries {
            let bytes = e.to_bytes();
            out.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
            out.extend_from_slice(&bytes);
        }
        out
    }

    /// KTHIM MIRROR (Shadow → Quantum/Light), DY origjina: kthen
    /// `ShadowLightResponse` (formë identike me pritjet e Quantum/Light).
    /// Varianti fallible; për variantin infallible shih `LightShadowBridge`.
    #[cfg(test)]
    pub fn respond_bridged(
        &self,
        pkg:   QuantumInbound,
        light: LightInbound,
    ) -> Result<crate::bridge::ShadowLightResponse, ShadowError> {
        let r = self.ingest_bridged(pkg, light)?;
        Ok(crate::bridge::ShadowLightResponse::from_response(r))
    }

    /// KTHIM MIRROR nga NJË burim Quantum (Light derivohet brenda, si rrjedha reale).
    #[cfg(test)]
    pub fn respond_to_quantum(
        &self,
        pkg: QuantumInbound,
    ) -> Result<crate::bridge::ShadowLightResponse, ShadowError> {
        let r = self.ingest_quantum(pkg)?;
        Ok(crate::bridge::ShadowLightResponse::from_response(r))
    }

    // ── HYRJET NGA LIGHT (Light përgatit; Shadow vendos & ruan) ──────

    /// APUPK — pranon dhe RUAN njohuri projekti personale (`shadow_APUPK_memory`).
    /// Kthen `trace_id`; `Err` nëse paketa s'ka gjurmë/përmbajtje (Shadow vendos).
    pub fn receive_apupk(
        &self,
        pkg: crate::shadow_apupk::ApupkInbound,
    ) -> Result<u64, crate::shadow_apupk::ApupkReject> {
        crate::shadow_apupk::apupk_memory().store(pkg)
    }

    /// APUPK fail-closed për regjistrimin ndër-procesor të projektit.
    /// ProjectContextWitness nuk lejohet pa WAL të sinkronizuar në disk.
    pub fn receive_apupk_durable(
        &self,
        pkg: crate::shadow_apupk::ApupkInbound,
    ) -> Result<u64, crate::shadow_apupk::ApupkReject> {
        crate::shadow_apupk::apupk_memory().store_durable(pkg)
    }

    /// SNB — pranon dhe RUAN një raport bug-u nga Light (`shadow_snb`).
    /// Kthen dëftesën (nivel + frekuencë); `Err` nëse s'ka gjurmë rrjedhe/përshkrim.
    pub fn receive_bug_report(
        &self,
        report: crate::shadow_snb::BugInbound,
    ) -> Result<crate::shadow_snb::SnbReceipt, crate::shadow_snb::SnbReject> {
        crate::shadow_snb::snb_store().report(report)
    }

    /// Lehtësi: kthen vetëm verdiktin suprem.
    #[cfg(test)]
    pub fn ingest_verdict(
        &self,
        pkg:   PassPackage,
        light: LightEnvelope,
    ) -> Result<SupremeVerdict, ShadowError> {
        self.ingest(pkg, light).map(|r| r.verdict)
    }

    // ── Ngrirja sovrane ─────────────────────────────────────────────────

    pub fn freeze(&self, stage: i32) {
        self.frozen.store(stage.max(1), Ordering::SeqCst);
        // RUNTIME HARDENING (      ): rezultati i kernelit bëhet eksplicit.
        // Gjendja lokale (frozen) ruhet sidoqoftë; kernel = shtresë e dytë.
        let _kernel_freeze = ShadowGjLegacy::freeze(stage);
    }

    pub fn unfreeze(&self) {
        self.frozen.store(0, Ordering::SeqCst);
        let _kernel_unfreeze = ShadowGjLegacy::unfreeze();
    }

    pub fn is_frozen(&self) -> bool {
        self.frozen.load(Ordering::SeqCst) != 0
    }

    // ── Maturimi & vëzhgueshmëria ───────────────────────────────────────

    /// Provon promovimin Primitive → Legacy (mbijetesa e zgjatur).
    pub fn run_maturation(&self) -> u32 {
        crate::shadow_runtime_pulse::mark_ready(crate::shadow_runtime_pulse::SStage::Maturation);
        self.vault.run_maturation()
    }

    pub fn stats(&self) -> ShadowStats {
        let k = ShadowGjLegacy::get_stats();
        let (verification_ledger_count, verification_wisdom_count) = match self.verification.lock() {
            Ok(v) => (v.ledger_size(), v.wisdom_count()),
            Err(poisoned) => {
                let v = poisoned.into_inner();
                (v.ledger_size(), v.wisdom_count())
            }
        };
        ShadowStats {
            total:            k.total,
            passed:           k.passed,
            blocked:          k.blocked,
            frozen:           self.is_frozen(),
            primitive_count:  self.vault.primitive_count(),
            legacy_count:     self.vault.legacy_count(),
            negative_count:   self.vault.negative_count(),
            hypothesis_count: self.vault.hypothesis_count(),
            fact_count:       self.vault.fact_count(),
            time_degraded:    crate::shadow_pipeline::time_degraded(),
            verification_ledger_count,
            verification_wisdom_count,
        }
    }

    /// Akses i lexueshëm te arkivi sovran (për query/konfirmim maturimi).
    #[cfg(test)]
    pub fn vault(&self) -> &Arc<KnowledgeVault> {
        &self.vault
    }
}

// ════════════════════════════════════════════════════════════════════════════
// TESTE — URA 2: transporti i PA-ankorave nga skedari (Light → Shadow)
// ════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod pa_feed_tests {
    use super::*;

    fn tmp(name: &str) -> String {
        format!("{}/{}", std::env::temp_dir().display(), name)
    }

    #[test]
    fn feed_missing_file_is_ok_zero_and_silent() {
        // KONTRATA: skedar që mungon = nisje e parë → Ok(0), pa alarm.
        let s = Shadow::new().expect("shadow init");
        assert_eq!(
            s.feed_primitive_anchors("nuk_ekziston_askund.txt", "S1")
                .expect("skedar që mungon duhet Ok(0), jo Err"),
            0
        );
    }

    #[test]
    fn feed_reads_last_anchor_of_session() {
        let s = Shadow::new().expect("shadow init");
        let p = tmp("pa_feed_last.txt");
        std::fs::write(&p,
            "S1|00000000000000aa|1\nS2|00000000000000bb|2\nS1|00000000000000cc|3\n")
            .expect("write");
        // Dy rreshta S1 — regjistrohet i FUNDIT (0xcc); kthimi = Ok(1).
        assert_eq!(
            s.feed_primitive_anchors(&p, "S1").expect("feed i vlefshëm"),
            1
        );
        let _ = std::fs::remove_file(&p);
    }

    #[test]
    fn feed_corruption_is_fail_closed_err() {
        // HARDENING: korrupsioni s'kapërcehet — Err(TransportCorrupt).
        let s = Shadow::new().expect("shadow init");
        let p = tmp("pa_feed_bad.txt");
        // (a) hex i pavlefshëm → Err me arsye 'hex'.
        std::fs::write(&p, "S1|jo_hex|9\n").expect("write");
        match s.feed_primitive_anchors(&p, "S1") {
            Err(ShadowError::TransportCorrupt(m)) =>
                assert!(m.contains("hex"), "arsyeja duhet të përmendë hex: {m}"),
            other => panic!("pritej TransportCorrupt(hex), erdhi {:?}", other),
        }
        // (b) numër fushash i papritur (2) → Err me arsye 'fusha'.
        std::fs::write(&p, "S1|deadbeef\n").expect("write");
        match s.feed_primitive_anchors(&p, "S1") {
            Err(ShadowError::TransportCorrupt(m)) =>
                assert!(m.contains("fusha"), "arsyeja duhet të përmendë fushat: {m}"),
            other => panic!("pritej TransportCorrupt(fusha), erdhi {:?}", other),
        }
        let _ = std::fs::remove_file(&p);
    }

    #[test]
    fn pd_parent_i0_is_bound_to_the_waiting_primitive_anchor_not_session_id() {
        let pa = 0xA451u64;
        assert!(Shadow::parent_i0_matches_anchor(
            Some("i0-000000000000a451"),
            Some(pa),
        ));
        assert!(!Shadow::parent_i0_matches_anchor(Some("SESSION-1"), Some(pa)));
        assert!(!Shadow::parent_i0_matches_anchor(None, Some(pa)));
        assert!(!Shadow::parent_i0_matches_anchor(
            Some("i0-000000000000a452"),
            Some(pa),
        ));
    }

    #[test]
    fn feed_contract_matches_light_writer_format() {
        // Kontrata e RE (5 fusha): "sid|{:016x}|ts|xi:...|yi:..." — si Light shkruan.
        let s = Shadow::new().expect("shadow init");
        let p = tmp("pa_feed_contract.txt");
        let line = format!("SES_X|{:016x}|123456789|xi:flamur,kuq|yi:shqiponje\n", 0xA451u64);
        std::fs::write(&p, line).expect("write");
        assert_eq!(
            s.feed_primitive_anchors(&p, "SES_X").expect("kontrata 5-fusha e vlefshme"),
            1
        );
        let _ = std::fs::remove_file(&p);
    }

    #[test]
    fn feed_old_three_field_still_accepted_backward() {
        // Rreshtat e vjetër 3-fushësh pranohen (split bosh) — pas-pajtueshmëri.
        let s = Shadow::new().expect("shadow init");
        let p = tmp("pa_feed_old3.txt");
        std::fs::write(&p, "SES_O|00000000000000ff|9\n").expect("write");
        assert_eq!(
            s.feed_primitive_anchors(&p, "SES_O").expect("formati i vjetër 3-fusha"),
            1
        );
        let _ = std::fs::remove_file(&p);
    }

    #[test]
    fn gcl_live_reinforce_only_on_verified_y() {
        // GCL LIVE: rendi i ligjit — Verified(Y)=1 → Trust(X) te ankora;
        // Verified(Y)=0 → asnjë propagim. Sinjal real, zero simulim.
        let s = Shadow::new().expect("shadow init");
        let pa: u64 = 0xA451;
        // Ankora e sesionit pritet në gate (siç e sjell Light në runtime).
        s.register_primitive_anchor("SES_GCL", pa,
            PrimitiveSplit { xi: vec!["flamur".into()], yi: vec!["shqiponje".into()] });
        // Primitivi ekziston në vault (siç e shkruan judge_supreme në D=1) —
        // firma REALE 9-argumentëshe, si në testet e vetë vault-it.
        s.vault.write_primitive(
            pa, 0.9, 0.9, 0.9, 0xA451, crate::types::ShadowPath::Fast,
            "GCL_LIVE_TEST".to_string(), b"gcl-live".to_vec(), 1,
        ).expect("write_primitive për testin GCL");
        let hits_para = s.vault.primitive_real_hits(pa);

        // (a) Y JO i verifikuar → Trust(X) NUK propagohet.
        let v0 = SupremeVerdict {
            verified: 0, primitive: 0,
            knowledge_band: crate::types::KnowledgeBand::Hypothesis,
            lgc_law: 0, legacy_score: 0.0, lgc_dsl: String::new(),
            living_trust_sha256: [0u8; 32],
            living_trust_intensity: 0,
            living_trust_kind: 0,
            living_trust_action_state: 0,
            living_trust_action_mask: 0,
            living_trust_required_action_mask: 0,
            living_trust_system_laws_seal: 0,
            living_trust_sovereign_flags: 0,
            living_trust_sovereign_value: 0,
        
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
        };
        s.gcl_reinforce_on_verified("SES_GCL", &v0);
        assert_eq!(s.vault.primitive_real_hits(pa), hits_para,
            "Y=0 s'duhet të rrisë real_hits");

        // (b) Y i verifikuar → Trust(X) propagohet: real_hits+1.
        let v1 = SupremeVerdict {
            verified: 1, primitive: 1,
            knowledge_band: crate::types::KnowledgeBand::Hypothesis,
            lgc_law: 0, legacy_score: 0.0, lgc_dsl: String::new(),
            living_trust_sha256: [0u8; 32],
            living_trust_intensity: 0,
            living_trust_kind: 0,
            living_trust_action_state: 0,
            living_trust_action_mask: 0,
            living_trust_required_action_mask: 0,
            living_trust_system_laws_seal: 0,
            living_trust_sovereign_flags: 0,
            living_trust_sovereign_value: 0,
        
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
        };
        s.gcl_reinforce_on_verified("SES_GCL", &v1);
        assert_eq!(s.vault.primitive_real_hits(pa), hits_para + 1,
            "Y=1 duhet të rrisë real_hits me +1");
    }

    #[test]
    fn verification_trl_derives_from_real_legacy_score() {
        // Sjellja (audit v1.4.5 p.2): ver_conf = legacy_score REAL kur verified,
        // jo proxy 0.90. Provohet ligji i mapimit trl_from_confidence mbi vlerat
        // reale që do të mbante SupremeVerdict.legacy_score.
        use crate::lab_contracts::trl::TrlVector;
        // legacy_score i lartë (0.92) → TRL4; i mesëm (0.60) → TRL2.
        assert_eq!(TrlVector::trl_from_confidence(0.92), 4,
            "legacy_score i fortë → verification_trl 4");
        assert_eq!(TrlVector::trl_from_confidence(0.60), 2,
            "legacy_score i mesëm → verification_trl 2");
        // Dysheme kur verified pa sinjal (0.50→TRL2) dhe kur s'verifikohet (0.30→TRL1).
        assert_eq!(TrlVector::trl_from_confidence(0.50), 2);
        assert_eq!(TrlVector::trl_from_confidence(0.30), 1);
        // Ligji i degës: verifikuar me legacy>0 → merr legacy; verifikuar pa
        // legacy → 0.50; i paverifikuar → 0.30 (i njëjti match si në gcl_apply).
        let deg = |verified: bool, ls: f32| -> f32 {
            match (verified, ls > 0.0) {
                (true, true)  => ls,
                (true, false) => 0.50,
                (false, _)    => 0.30,
            }
        };
        assert_eq!(deg(true, 0.88), 0.88, "verifikuar → forca reale");
        assert_eq!(deg(true, 0.0), 0.50, "verifikuar pa sinjal → kujdes");
        assert_eq!(deg(false, 0.99), 0.30, "s'u verifikua → dysheme, injoron legacy");
    }

    #[test]
    fn feed_sealed_crc_line_is_ok_and_tamper_is_err() {
        // PACP end-to-end në urë: encode (Light) → verify+feed (Shadow).
        let s = Shadow::new().expect("shadow init");
        let p = tmp("pa_feed_pacp.txt");
        let line = crate::lab_contracts::pa_wire::encode_line(
            "SES_C", "00000000000000a4", 987654321, "xi:flamur,kuq|yi:shqiponje");
        std::fs::write(&p, &line).expect("write");
        assert_eq!(
            s.feed_primitive_anchors(&p, "SES_C").expect("ankora e vulosur CRC-OK"),
            1
        );
        // Tamper: një bit brenda trupit → CRC s'përputhet → TransportCorrupt.
        let tampered = line.replace("kuq", "kur");
        std::fs::write(&p, &tampered).expect("write");
        match s.feed_primitive_anchors(&p, "SES_C") {
            Err(ShadowError::TransportCorrupt(m)) =>
                assert!(m.contains("PACP"), "arsyeja duhet të mbajë vulën PACP: {m}"),
            other => panic!("pritej TransportCorrupt(PACP), erdhi {:?}", other),
        }
        let _ = std::fs::remove_file(&p);
    }

    #[test]
    fn feed_five_field_bad_wire_is_fail_closed_err() {
        // HARDENING: 5 fusha por wire pa prefikset xi:/yi: → Err (jo kapërcim).
        let s = Shadow::new().expect("shadow init");
        let p = tmp("pa_feed_badwire.txt");
        std::fs::write(&p, "SES_B|00000000000000aa|9|flamur|shqiponje\n").expect("write");
        match s.feed_primitive_anchors(&p, "SES_B") {
            Err(ShadowError::TransportCorrupt(m)) =>
                assert!(m.contains("wire"), "arsyeja duhet të përmendë wire: {m}"),
            other => panic!("pritej TransportCorrupt(wire), erdhi {:?}", other),
        }
        let _ = std::fs::remove_file(&p);
    }

    #[test]
    fn gcl_bridged_path_reinforces_after_anchor_removed() {
        // REGRESIONI I GJETJES KURORË (v1.3.2): rruga e urës HEQ ankorën nga
        // pa_waiting PARA verdiktit (KOLAPS/3) — laku GCL duhet të mbyllet
        // NJËSOJ me ankorën e mbartur (gcl_apply), jo me lookup që gjen None.
        let s = Shadow::new().expect("shadow init");
        let pa: u64 = 0xB452;
        s.register_primitive_anchor("SES_BRG", pa,
            PrimitiveSplit { xi: vec!["flamur".into()], yi: vec!["shqiponje".into()] });
        s.vault.write_primitive(
            pa, 0.9, 0.9, 0.9, 0xB452, crate::types::ShadowPath::Fast,
            "GCL_BRIDGED_TEST".to_string(), b"gcl-bridged".to_vec(), 1,
        ).expect("write_primitive për testin e urës");
        let hits_para = s.vault.primitive_real_hits(pa);

        // Simulohet EKZAKTËSISHT rrjedha e urës: (1) ankora HIQET me pronësi…
        let e_hequr = match s.pa_waiting.lock() {
            Ok(mut m)     => m.remove("SES_BRG"),
            Err(poisoned) => poisoned.into_inner().remove("SES_BRG"),
        };
        let gcl_anchor = e_hequr.as_ref().map(|(p, _)| *p);
        assert_eq!(gcl_anchor, Some(pa), "ankora duhej hequr me pronësi");

        // (2) …lookup-i i vjetër tani gjen None (prova e vdekjes së lakut të vjetër)…
        let v1 = SupremeVerdict {
            verified: 1, primitive: 1,
            knowledge_band: crate::types::KnowledgeBand::Hypothesis,
            lgc_law: 0, legacy_score: 0.0, lgc_dsl: String::new(),
            living_trust_sha256: [0u8; 32],
            living_trust_intensity: 0,
            living_trust_kind: 0,
            living_trust_action_state: 0,
            living_trust_action_mask: 0,
            living_trust_required_action_mask: 0,
            living_trust_system_laws_seal: 0,
            living_trust_sovereign_flags: 0,
            living_trust_sovereign_value: 0,
        
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
        };
        s.gcl_reinforce_on_verified("SES_BRG", &v1);
        assert_eq!(s.vault.primitive_real_hits(pa), hits_para,
            "lookup pas heqjes duhet të jetë no-op — kjo ISHTE gjetja");

        // (3) …dhe rruga e re (ankora e mbartur) e mbyll lakun saktësisht një herë.
        match gcl_anchor {
            Some(p) => s.gcl_apply("SES_BRG", p, &v1),
            None    => panic!("ankora e mbartur mungon"),
        }
        assert_eq!(s.vault.primitive_real_hits(pa), hits_para + 1,
            "ankora e mbartur duhet të rifortësojë real_hits+1 në rrugën e urës");
    }

    #[test]
    fn legacy_after_sealed_is_downgrade_refused() {
        // ÇELËSI NJËDREJTIMËSH: i vulosur ⇒ përgjithmonë i vulosur. Rresht
        // legacy PAS një rreshti me CRC për të njëjtin sesion = degradim
        // (cungim i c:) → refuzim i argumentuar, i shënuar në rrjedha.
        let s = Shadow::new().expect("shadow init");
        let p = tmp("pa_feed_downgrade.txt");
        let sealed = crate::lab_contracts::pa_wire::encode_line(
            "SES_D", "00000000000000d4", 111, "xi:flamur,kuq|yi:shqiponje");
        let legacy = "SES_D|00000000000000d4|222|xi:flamur,kuq|yi:shqiponje\n";
        std::fs::write(&p, format!("{}{}", sealed, legacy)).expect("write");
        match s.feed_primitive_anchors(&p, "SES_D") {
            Err(ShadowError::TransportCorrupt(m)) =>
                assert!(m.contains("DEGRADIM"), "arsyeja duhet të deklarojë degradimin: {m}"),
            other => panic!("pritej TransportCorrupt(DEGRADIM), erdhi {:?}", other),
        }
        // Ndërsa sesion PA histori të vulosur pranon legacy të deklaruar (pas-pajtueshmëria e paprekur).
        let p2 = tmp("pa_feed_pure_legacy.txt");
        std::fs::write(&p2, "SES_L|00000000000000e5|333|xi:flamur|yi:shqiponje\n").expect("write");
        assert_eq!(s.feed_primitive_anchors(&p2, "SES_L").expect("legacy i pastër pranohet"), 1);
        let _ = std::fs::remove_file(&p);
        let _ = std::fs::remove_file(&p2);
    }

    #[test]
    fn fnv1a64_deterministic_known_vector() {
        // Vektor i njohur FNV-1a 64: "a" → 0xaf63dc4c8601ec8c (determinizëm i provuar).
        assert_eq!(super::fnv1a64(b"a"), 0xaf63_dc4c_8601_ec8c);
        assert_eq!(super::fnv1a64(b""),  0xcbf2_9ce4_8422_2325);
    }
}
