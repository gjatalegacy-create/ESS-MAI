// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  KNOWLEDGE_LINEAGE.RS — LIGJI I GJURMUESHMËRISË (Chain Trace Law)     ║
// ║  Gjata Legacy™  |  Arkitekt: Bledar Gjata                             ║
// ║                                                                          ║
// ║  PARIMI SOVRAN (i pacenueshëm):                                       ║
// ║    Në Shadow ruhet VETËM dija që ka GJURMË algoritmike të plotë.      ║
// ║    Çdo info pa trace/algoritëm NUK është dije → FSHIHET (nuk persiston).║
// ║                                                                          ║
// ║  "Sistemi nuk i beson vetes." Asnjë fakt s'pranohet sepse ekziston —   ║
// ║  pranohet vetëm nëse ka kaluar zinxhirin e verifikuesve dhe mban një   ║
// ║  gjurmë të plotë e të riprodhueshme: kush e verifikoi, çfarë vendosi,  ║
// ║  dhe nga cili zinxhir provash (proof_chain) rrjedh.                    ║
// ║                                                                          ║
// ║  ══ LIGJI I PRIMITIVIT UNANIM (identitet i qartë, i veçantë) ══════════ ║
// ║    Xᵢ(I) ∈ {0,1} = verdikti binar i çdo verifikuesi TË PAVARUR Sᵢ.    ║
// ║    UNANIM_PRIMITIVE(I)  ⇔  ∏ᵢ Xᵢ(I) = 1   (unanimitet i plotë)        ║
// ║    Domethënia: një pretendim bëhet PRIMITIV-I-VERIFIKUAR në Shadow     ║
// ║    VETËM kur ÇDO verifikues i pavarur e konfirmon (produkt = 1). Qoftë ║
// ║    edhe një Xᵢ=0 → produkti=0 → JO primitiv-i-verifikuar.             ║
// ║                                                                          ║
// ║  ⚠ NDARJE E PRERË NGA gjata_collapse_law (MOS I NGATËRRO):            ║
// ║    • Primitive Anchor (PA) i gjata_collapse_law = ana e HYRJES te      ║
// ║      Light: i0->Trace->PA->Xi‖Yi, vulë 500. Bileta E PRANIMIT —       ║
// ║      çfarë HYN në pipeline (koordinim, jo verdikt).                    ║
// ║    • UNANIM_PRIMITIVE këtu = ana e VERIFIKIMIT te Shadow: dije që      ║
// ║      MBIJETOI zinxhirin unanim të verifikuesve (REZULTAT, jo input).   ║
// ║    Pra: PA = hyn (Light) ; UNANIM_PRIMITIVE = del i verifikuar (Shadow).║
// ║    Të dy ligjet bashkëjetojnë pa përplasje — role të ndryshme në       ║
// ║    zinxhirin gjata_collapse_law (i0->...->D in {0,1}).                 ║
// ║                                                                          ║
// ║  GJURMUESHMËRIA (kusht PARA çdo shkrimi):                             ║
// ║    is_traceable() ⇔ derivation≠0 ∧ proof_len>0 ∧                      ║
// ║                     (cap_sealed ∨ chain_count ≥ MIN_CHAIN)            ║
// ║    Nëse FALSE → inputi nuk është dije → fshihet (purge).              ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::types::ShadowVerdict;
use crate::types::PrimitiveTrace;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::{Mutex, OnceLock};

// ── FNV-1a (i njëjtë me sovereign_ffi_gate; nënshkrim deterministik) ────────
const fn fnv1a(seed: u64, bytes: &[u8]) -> u64 {
    let mut h = seed;
    let mut i = 0;
    while i < bytes.len() {
        h ^= bytes[i] as u64;
        h = h.wrapping_mul(1099511628211);
        i += 1;
    }
    h
}
const FNV_OFFSET: u64 = 14695981039346656037;

#[inline]
fn mix_u64(h: u64, v: u64) -> u64 {
    fnv1a(h, &v.to_le_bytes())
}

/// Gjatësia minimale e zinxhirit për gjurmë të vlefshme (Router+Matrix+Judiciary).
pub const MIN_CHAIN: u8 = 3;

/// Numri minimal i DOMAIN-eve unike që një Primitiv → Legacy (përforcim ndër-domain).
pub const MIN_LEGACY_DOMAINS: usize = 5;

/// Numëron domain-et UNIKE në zinxhirin primitiv.
pub fn unique_domains(chain: &[PrimitiveTrace]) -> usize {
    let mut set: HashSet<&str> = HashSet::new();
    for t in chain {
        set.insert(t.domain.as_str());
    }
    set.len()
}

/// LIGJI PRIMITIVE→LEGACY (i PËRFORCUAR, FAZA 2→3): i gatshëm vetëm me
/// ≥5 domain-e UNIKE, vulë konsistente (0xA451), DHE integritet i plotë i
/// zinxhirit — gjurmët rriten në kohë (monotoni) e kanë masë mbi pragun.
///
/// Përforcim: legacy s'është vetëm "5 domaine" — është 5 domaine me një
/// zinxhir prove TË INTEGRUAR. Kjo e bën promovimin më të rëndë e më të sigurt.
/// ZERO if/else — krahasim & prodhim boolean.
pub fn cross_domain_legacy_ready(chain: &[PrimitiveTrace]) -> bool {
    let domains = unique_domains(chain);
    let seal_ok = chain.iter().all(|t| ((t.lgc_seal & 0xFFFF) ^ 0xA5A5) == 500);
    // Invariantët shtesë I2 (monotoni kohore) + I3 (masa mbi prag).
    let temporal_ok = crate::primitive_chain::temporally_monotonic(chain);
    let mass_ok     = crate::primitive_chain::mass_floor_met(chain);
    // Të katërta duhet të mbahen (produkt boolean).
    (domains >= MIN_LEGACY_DOMAINS) & seal_ok & temporal_ok & mass_ok
}

// ════════════════════════════════════════════════════════════════════════════
// §1  LINEAGE — gjurma e plotë algoritmike e një copë dijeje
// ════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Lineage {
    /// Identifikuesi i inputit (stable_id).
    pub input_id:    u64,
    /// Bit i ndezur për çdo nod që dha verdikt == 1 (index = `ShadowNode as u16`).
    pub chain_mask:  u16,
    /// Sa verifikues prodhuan verdikt (gjatësia n e zinxhirit).
    pub chain_count: u8,
    /// Gjatësia e proof_chain (zinxhiri i provave algoritmike nga upstream).
    pub proof_len:   u32,
    /// Nënshkrimi i derivimit — FNV mbi (input_id ∥ verdiktet ∥ proof_chain).
    pub derivation:  u64,
    /// E vulosur nga një kapacitet sovran single-use (rruga FFI). Shih
    /// `sovereign_ffi_gate`: vula atomike = autorizim, vlen si gjurmë.
    pub cap_sealed:  bool,
}

impl Lineage {
    /// Ndërton gjurmën nga ZINXHIRI i verifikuesve (rruga e pipeline-it).
    pub fn derive_chain(input_id: u64, verdicts: &[ShadowVerdict], proof_chain: &[u64]) -> Self {
        let mut chain_mask: u16 = 0;
        let mut chain_count: u8 = 0;
        let mut h = mix_u64(FNV_OFFSET, input_id);

        for v in verdicts {
            chain_count = chain_count.saturating_add(1);
            // Bit i ndezur vetëm kur verdikti është 1 (kalon aspektin e tij).
            let bit = (v.verdict == 1) as u16;
            chain_mask |= bit << (v.node as u16);
            // Gjurma përfshin nodin dhe verdiktin → e riprodhueshme.
            h = fnv1a(h, &[v.node as u8, v.verdict]);
        }
        for p in proof_chain {
            h = mix_u64(h, *p);
        }

        Self {
            input_id,
            chain_mask,
            chain_count,
            proof_len: proof_chain.len() as u32,
            derivation: h,
            cap_sealed: false,
        }
    }

    /// Ndërton gjurmën për një shkrim të autorizuar nga kapaciteti sovran (FFI).
    /// proof_chain këtu = lineage i kapacitetit (p.sh. [gen, nonce]).
    pub fn derive_capability(input_id: u64, proof_chain: &[u64]) -> Self {
        let mut h = mix_u64(FNV_OFFSET, input_id);
        h = mix_u64(h, 0xCA9A_B111_7E57_5EA1); // domain-tag "capability"
        for p in proof_chain {
            h = mix_u64(h, *p);
        }
        Self {
            input_id,
            chain_mask: 0,
            chain_count: 0,
            proof_len: proof_chain.len() as u32,
            derivation: h,
            cap_sealed: true,
        }
    }

    /// LIGJI ZINXHIR: a është Primitive sipas produktit ∏ Xᵢ = 1?
    /// E vërtetë vetëm kur ÇDO nod i regjistruar dha 1 (të gjitha bitet e ndezura).
    pub fn is_primitive_chain(&self) -> bool {
        match self.chain_count == 0 { true => return false, false => {} }
        let popcount = self.chain_mask.count_ones() as u8;
        popcount == self.chain_count
    }

    /// LIGJI I GJURMUESHMËRISË — a e trajton sistemi këtë si DIJE?
    /// Pa gjurmë algoritmike (proof_chain bosh, nënshkrim zero, ose zinxhir i
    /// pamjaftueshëm pa vulë kapaciteti) → NUK është dije → do të fshihet.
    pub fn is_traceable(&self) -> bool {
        self.derivation != 0
            && self.proof_len > 0
            && (self.cap_sealed || self.chain_count >= MIN_CHAIN)
    }
}

// ════════════════════════════════════════════════════════════════════════════
// §2  ARSYET E MOSGJURMUESHMËRISË (përse u fshi)
// ════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TraceError {
    /// Zinxhiri i provave bosh — s'ka prejardhje algoritmike.
    EmptyProofChain,
    /// Zinxhiri i verifikuesve shumë i shkurtër (pa vulë kapaciteti).
    IncompleteChain,
    /// Nënshkrim derivimi i pavlefshëm.
    NullDerivation,
}

impl std::fmt::Display for TraceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let m = match self {
            TraceError::EmptyProofChain => "proof_chain bosh — pa prejardhje algoritmike",
            TraceError::IncompleteChain => "zinxhir verifikuesish i pamjaftueshëm",
            TraceError::NullDerivation  => "nënshkrim derivimi zero",
        };
        write!(f, "TRACE_PURGE: {m}")
    }
}

/// LIGJI: kthen gjurmën nëse është e plotë; përndryshe arsyen e fshirjes.
/// Thirrësi që merr `Err` DUHET ta fshijë inputin (mos e shkruaj në vault).
pub fn enforce_traceability(lineage: &Lineage) -> Result<(), TraceError> {
    match lineage.derivation == 0 { true => return Err(TraceError::NullDerivation), false => {} }
    match lineage.proof_len == 0 { true => return Err(TraceError::EmptyProofChain), false => {} }
    match !lineage.cap_sealed && lineage.chain_count < MIN_CHAIN {
        true => return Err(TraceError::IncompleteChain),
        false => {}
    }
    Ok(())
}

// ════════════════════════════════════════════════════════════════════════════
// §3  LINEAGE LEDGER — regjistri i gjurmëve të dijes së ruajtur
//     (Çdo copë dije e ruajtur ka një gjurmë këtu → "trace dhe gjithçka".)
// ════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Default)]
pub struct LineageLedger {
    traces: Mutex<HashMap<u64, Lineage>>,
    purged: Mutex<u64>,
}

impl LineageLedger {
    fn new() -> Self {
        Self { traces: Mutex::new(HashMap::new()), purged: Mutex::new(0) }
    }

    /// Regjistron gjurmën e një copë dijeje të pranuar (e shkruar në vault).
    pub fn record(&self, lineage: Lineage) {
        match self.traces.lock() {
            Ok(mut t) => { t.insert(lineage.input_id, lineage); }
            Err(_) => {}
        }
    }

    /// Shënon një input të fshirë (pa gjurmë → s'u trajtua si dije).
    pub fn note_purge(&self) {
        match self.purged.lock() {
            Ok(mut p) => { *p = p.saturating_add(1); }
            Err(_) => {}
        }
    }

    /// Gjen gjurmën e një inputi (verifikim i mëvonshëm i prejardhjes).
    pub fn lookup(&self, input_id: u64) -> Option<Lineage> {
        self.traces.lock().ok().and_then(|t| t.get(&input_id).copied())
    }

    pub fn traced_count(&self) -> usize {
        self.traces.lock().map(|t| t.len()).unwrap_or(0)
    }

    pub fn purged_count(&self) -> u64 {
        self.purged.lock().map(|p| *p).unwrap_or(0)
    }
}

// ── Regjistri global (një autoritet i vetëm gjurmësh për procesin) ──────────
static LEDGER: OnceLock<LineageLedger> = OnceLock::new();

pub fn ledger() -> &'static LineageLedger {
    LEDGER.get_or_init(LineageLedger::new)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ShadowNode;

    fn vd(node: ShadowNode, verdict: u8) -> ShadowVerdict {
        ShadowVerdict { node, score: verdict as f32, verdict, reason_code: 0, ts_ns: 1 }
    }

    #[test]
    fn full_chain_with_proof_is_traceable() {
        let verdicts = [
            vd(ShadowNode::Router, 1),
            vd(ShadowNode::Matrix, 1),
            vd(ShadowNode::Judiciary, 1),
        ];
        let lin = Lineage::derive_chain(42, &verdicts, &[0xABCD, 0x1234]);
        assert!(lin.is_traceable());
        assert!(enforce_traceability(&lin).is_ok());
        assert!(lin.is_primitive_chain()); // të tre 1 → ∏ = 1
    }

    #[test]
    fn empty_proof_chain_is_purged() {
        let verdicts = [
            vd(ShadowNode::Router, 1),
            vd(ShadowNode::Matrix, 1),
            vd(ShadowNode::Judiciary, 1),
        ];
        let lin = Lineage::derive_chain(42, &verdicts, &[]); // pa proof_chain
        assert!(!lin.is_traceable());
        assert_eq!(enforce_traceability(&lin), Err(TraceError::EmptyProofChain));
    }

    #[test]
    fn short_chain_is_purged() {
        let verdicts = [vd(ShadowNode::Router, 1)]; // vetëm 1 nod < MIN_CHAIN
        let lin = Lineage::derive_chain(7, &verdicts, &[0x99]);
        assert!(!lin.is_traceable());
        assert_eq!(enforce_traceability(&lin), Err(TraceError::IncompleteChain));
    }

    #[test]
    fn one_zero_breaks_primitive_but_stays_traceable() {
        let verdicts = [
            vd(ShadowNode::Router, 1),
            vd(ShadowNode::Matrix, 0), // një 0 → jo Primitive
            vd(ShadowNode::Judiciary, 1),
        ];
        let lin = Lineage::derive_chain(9, &verdicts, &[0x55]);
        assert!(lin.is_traceable());        // ka gjurmë → sistemohet
        assert!(!lin.is_primitive_chain()); // por jo Primitive (∏ = 0)
    }

    #[test]
    fn capability_sealed_is_traceable_without_chain() {
        let lin = Lineage::derive_capability(123, &[5, 7]); // gen,nonce si proof
        assert!(lin.is_traceable());        // vula e kapacitetit = gjurmë
        assert!(lin.cap_sealed);
    }

    #[test]
    fn ledger_records_and_looks_up() {
        let led = LineageLedger::new();
        let lin = Lineage::derive_capability(555, &[1]);
        led.record(lin);
        assert_eq!(led.traced_count(), 1);
        assert_eq!(led.lookup(555), Some(lin));
        led.note_purge();
        assert_eq!(led.purged_count(), 1);
    }
}
