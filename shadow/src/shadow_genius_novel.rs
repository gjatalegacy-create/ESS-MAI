// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  SHADOW_GENIUS_NOVEL.RS — Bartësi i TRL 4 (Inovacion Faktik)          ║
// ║  Gjata Legacy™ | Arkitekt: Bledar Gjata                    (FAZA 9)    ║
// ║                                                                          ║
// ║  NIVELET E DIJES (sipas gjykimit):                                     ║
// ║   • Hypothesis potenciale  → ende po formohet                         ║
// ║   • Hypothesis kandidat-faktik → TRL 3 (provë koncepti)              ║
// ║   • Shkenca ekzakte / Inovacion → TRL 4 (FAKTUAR nga bota, i prekshëm) ║
// ║                                                                          ║
// ║  KALIMI TRL3 → TRL4: NUK ndodh nga arsyetim — ndodh kur sistemit       ║
// ║  I FAKTOHET me DOKUMENTACION REAL se si doli (foto + video + përshkrim).║
// ║  Atëherë Shadow e ngre nga hipotezë → FAKT i prekshëm dhe e regjistron ║
// ║  te pjesa e projekteve të përdoruesit (APUPK).                         ║
// ║                                                                          ║
// ║  LIGJET E SHADOW të ruajtura:                                          ║
// ║   • Shadow vendos (0/1) — dokumentacioni është provë, jo vendim.       ║
// ║   • DISTRUST: pa dokumentacion të vlefshëm → mbetet TRL 3 (jo ngritje). ║
// ║   • Faktimi kërkon TË PAKTËN një provë vizuale (foto OSE video) +       ║
// ║     përshkrim (si doli). Provë e vetme s'mjafton — botë reale = shumë-  ║
// ║     burim.                                                              ║
// ║  Zero if/else — produkt boolean / match.                              ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::media_formats::{FormatDetector, MediaKind};
use crate::shadow_lab::{TrlInput, TrlVerdict};

// ─────────────────────────────────────────────────────────────────────────────
// KONSTANTET — nivelet TRL të faktimit
// ─────────────────────────────────────────────────────────────────────────────

/// TRL 4 — niveli i inovacionit faktik (POC i prekshëm, faktuar nga bota).
pub const TRL4_FACTUAL: u8 = shadow_contracts::SHADOW_FACTUAL_TRL;

/// Alias eksplicit: niveli që Shadow regjistron si fakt të prekshëm.
pub const SHADOW_TRL4: u8 = TRL4_FACTUAL;

/// Numri minimal i provave vizuale (foto/video) për faktim.
pub const MIN_VISUAL_PROOFS: usize = 1;

/// Gjatësia minimale e përshkrimit "si doli" (karaktere) për faktim serioz.
pub const MIN_DESCRIPTION_LEN: usize = 20;

// ─────────────────────────────────────────────────────────────────────────────
// NOVEL EVIDENCE — dokumentacioni i faktimit
// ─────────────────────────────────────────────────────────────────────────────

/// Një copë dokumentacioni: bytes-et e papërpunuara të një skedari prove.
#[derive(Debug, Clone)]
pub struct EvidenceFile {
    pub bytes: Vec<u8>,
    pub kind:  MediaKind,   // njihet me FormatDetector në ndërtim
}

impl EvidenceFile {
    /// from_bytes — ndërton një provë duke njohur llojin nga magic bytes.
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        let kind = FormatDetector::detect_kind(&bytes);
        Self { bytes, kind }
    }

    /// is_visual — a është provë vizuale (foto ose video)? Zero if.
    pub fn is_visual(&self) -> bool {
        let is_image = (self.kind as u8) == (MediaKind::Image as u8);
        let is_video = (self.kind as u8) == (MediaKind::Video as u8);
        is_image | is_video
    }

    /// is_document — a është dokument (përshkrim/letra)? Zero if.
    pub fn is_document(&self) -> bool {
        (self.kind as u8) == (MediaKind::Document as u8)
    }
}

/// Dokumentacioni i plotë i faktimit të një inovacioni TRL 4.
#[derive(Debug, Clone)]
pub struct NovelEvidence {
    pub project_id:   u64,            // cili projekt i përdoruesit
    pub files:        Vec<EvidenceFile>,  // foto/video/dokumente
    pub description:  String,         // "si doli" — përshkrimi i prejardhjes
    pub trl:          TrlInput,       // evidenca TRL nga laboratorët
}

impl NovelEvidence {
    /// visual_count — sa prova vizuale (foto/video). Zero if — filter.
    pub fn visual_count(&self) -> usize {
        self.files.iter().filter(|f| f.is_visual()).count()
    }

    /// document_count — sa dokumente. Zero if — filter.
    pub fn document_count(&self) -> usize {
        self.files.iter().filter(|f| f.is_document()).count()
    }

    /// all_files_known — a janë TË GJITHA skedarët formate të njohura? Zero if.
    pub fn all_files_known(&self) -> bool {
        let non_empty = !self.files.is_empty();
        let all_valid = self.files.iter().all(|f| f.kind.is_valid_evidence());
        non_empty & all_valid
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// NOVEL VERDICT — gjykimi i faktimit
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NovelRejection {
    None,              // u faktua
    NoVisualProof,     // mungon foto/video
    DescriptionTooShort, // përshkrimi "si doli" i pamjaftueshëm
    UnknownFormats,    // skedarë me format të panjohur (s'janë provë)
    TrlNotReady,       // TRL nuk e mbështet (laboratorët s'e konfirmuan)
}

impl NovelRejection {
    pub fn label(self) -> &'static str {
        match self {
            NovelRejection::None                => "FACTUAL_NONE",
            NovelRejection::NoVisualProof       => "NO_VISUAL_PROOF",
            NovelRejection::DescriptionTooShort => "DESCRIPTION_TOO_SHORT",
            NovelRejection::UnknownFormats      => "UNKNOWN_FORMATS",
            NovelRejection::TrlNotReady         => "TRL_NOT_READY",
        }
    }
}

/// Verdikti i faktimit: a u ngrit hipoteza në inovacion faktik TRL 4?
#[derive(Debug, Clone, Copy)]
pub struct NovelVerdict {
    pub factualized:  bool,            // a u faktua si TRL 4
    pub trl_level:    u8,              // 3 (mbeti) ose 4 (u ngrit)
    pub rejection:    NovelRejection,  // arsyeja nëse s'u faktua
    pub proof_score:  f32,            // [0,1] forca e dokumentacionit
}

// ─────────────────────────────────────────────────────────────────────────────
// FACTUAL INNOVATION — regjistrimi final (i prekshëm)
// ─────────────────────────────────────────────────────────────────────────────

/// Inovacioni faktik i regjistruar — jo më hipotezë, por FAKT i prekshëm TRL 4.
/// Ky shkon te pjesa e projekteve të përdoruesit (APUPK) si produkt real.
#[derive(Debug, Clone)]
pub struct FactualInnovation {
    pub project_id:       u64,
    pub trl_level:        u8,        // 4
    pub proof_score:      f32,
    pub visual_proofs:    usize,
    pub document_proofs:  usize,
    pub description:      String,    // si doli
    pub registered_at:    u64,
    pub immutable_seal:   u32,       // i paprekshëm (si LegacyEntry, LAW_4)
}

// ─────────────────────────────────────────────────────────────────────────────
// GENIUS NOVEL — gjykatësi i faktimit TRL 4
// ─────────────────────────────────────────────────────────────────────────────

pub struct GeniusNovel;

impl GeniusNovel {
    /// adjudicate — Shadow gjykon dokumentacionin dhe vendos faktimin TRL 4.
    ///
    /// Faktimi kërkon TË GJITHA:
    ///   • të paktën MIN_VISUAL_PROOFS provë vizuale (foto/video)
    ///   • përshkrim "si doli" ≥ MIN_DESCRIPTION_LEN
    ///   • të gjithë skedarët në formate të njohura
    ///   • TRL i laboratorëve e mbështet (TrlVerdict.trl_supports)
    ///
    /// Prioritet refuzimi (match): formate > vizuale > përshkrim > TRL.
    /// Zero if — produkt boolean + match shterues.
    pub fn adjudicate(evidence: &NovelEvidence) -> NovelVerdict {
        // Gjykimi TRL nga laboratorët (Faza 8).
        let trl_verdict = TrlVerdict::judge(&evidence.trl);

        // Kushtet e faktimit.
        let formats_ok = evidence.all_files_known();
        let visual_ok  = evidence.visual_count() >= MIN_VISUAL_PROOFS;
        let desc_ok    = evidence.description.len() >= MIN_DESCRIPTION_LEN;
        let trl_ok     = trl_verdict.trl_supports;

        // Vendimi + arsyeja (prioritet: formate > vizuale > përshkrim > TRL).
        let (factualized, rejection) = match (formats_ok, visual_ok, desc_ok, trl_ok) {
            (false, _, _, _) => (false, NovelRejection::UnknownFormats),
            (_, false, _, _) => (false, NovelRejection::NoVisualProof),
            (_, _, false, _) => (false, NovelRejection::DescriptionTooShort),
            (_, _, _, false) => (false, NovelRejection::TrlNotReady),
            (true, true, true, true) => (true, NovelRejection::None),
        };

        // trl_level: 4 nëse u faktua, ndryshe mbetet 3 (kandidat-faktik).
        let trl_level = match factualized {
            true => TRL4_FACTUAL,
            false => 3,
        };

        // proof_score: forca e dokumentacionit (vizuale + dokumente + TRL).
        let factual_factor = factualized as u32 as f32;
        let visual_norm = (evidence.visual_count() as f32 / 3.0).clamp(0.0, 1.0);
        let doc_norm    = (evidence.document_count() as f32 / 2.0).clamp(0.0, 1.0);
        let proof_score = (factual_factor
            * (visual_norm * 0.40 + doc_norm * 0.25 + trl_verdict.trl_score * 0.35))
            .clamp(0.0, 1.0);

        NovelVerdict {
            factualized,
            trl_level,
            rejection,
            proof_score,
        }
    }

    /// register — ndërton inovacionin faktik nga evidenca + verdikti.
    /// Thirret VETËM kur verdict.factualized == true (kontrollohet nga thirrësi).
    /// Prodhon FactualInnovation gati për regjistrim te APUPK.
    pub fn register(
        evidence: &NovelEvidence,
        verdict: &NovelVerdict,
        registered_at: u64,
    ) -> FactualInnovation {
        let immutable = (evidence.project_id as u32).wrapping_mul(0xA4B1) ^ 0x4004;
        FactualInnovation {
            project_id:      evidence.project_id,
            trl_level:       verdict.trl_level,
            proof_score:     verdict.proof_score,
            visual_proofs:   evidence.visual_count(),
            document_proofs: evidence.document_count(),
            description:     evidence.description.clone(),
            registered_at,
            immutable_seal:  immutable,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn png_file() -> EvidenceFile {
        EvidenceFile::from_bytes(vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A])
    }
    fn mp4_file() -> EvidenceFile {
        EvidenceFile::from_bytes(vec![0x00, 0x00, 0x00, 0x18, 0x66, 0x74, 0x79, 0x70])
    }
    fn pdf_file() -> EvidenceFile {
        EvidenceFile::from_bytes(vec![0x25, 0x50, 0x44, 0x46, 0x2D, 0x31])
    }
    fn strong_trl() -> TrlInput {
        TrlInput {
            trl_level: 3, trl_passed: true, confidence: 0.85,
            reproducibility: 0.9, measurable: true, reproducible: true,
        }
    }

    fn full_evidence() -> NovelEvidence {
        NovelEvidence {
            project_id: 7,
            files: vec![png_file(), mp4_file(), pdf_file()],
            description: "How the POC was built: assembled the prototype circuit and tested it.".to_string(),
            trl: strong_trl(),
        }
    }

    #[test]
    fn evidence_file_recognizes_kind() {
        assert!(png_file().is_visual());
        assert!(mp4_file().is_visual());
        assert!(pdf_file().is_document());
        assert!(!pdf_file().is_visual());
    }

    #[test]
    fn counts_visual_and_document() {
        let ev = full_evidence();
        assert_eq!(ev.visual_count(), 2);   // png + mp4
        assert_eq!(ev.document_count(), 1); // pdf
        assert!(ev.all_files_known());
    }

    #[test]
    fn factualizes_with_full_documentation() {
        let ev = full_evidence();
        let v = GeniusNovel::adjudicate(&ev);
        assert!(v.factualized);
        assert_eq!(v.trl_level, TRL4_FACTUAL);  // u ngrit në TRL 4
        assert_eq!(v.trl_level, 4);
        assert_eq!(v.rejection, NovelRejection::None);
        assert!(v.proof_score > 0.0);
    }

    #[test]
    fn rejects_without_visual_proof() {
        // Vetëm dokument, pa foto/video → mbetet TRL 3.
        let ev = NovelEvidence {
            project_id: 1,
            files: vec![pdf_file()],
            description: "A description that is definitely long enough here.".to_string(),
            trl: strong_trl(),
        };
        let v = GeniusNovel::adjudicate(&ev);
        assert!(!v.factualized);
        assert_eq!(v.trl_level, 3);  // mbeti kandidat-faktik
        assert_eq!(v.rejection, NovelRejection::NoVisualProof);
    }

    #[test]
    fn rejects_short_description() {
        let ev = NovelEvidence {
            project_id: 2,
            files: vec![png_file()],
            description: "too short".to_string(),  // < 20
            trl: strong_trl(),
        };
        let v = GeniusNovel::adjudicate(&ev);
        assert!(!v.factualized);
        assert_eq!(v.rejection, NovelRejection::DescriptionTooShort);
    }

    #[test]
    fn rejects_unknown_formats() {
        let garbage = EvidenceFile::from_bytes(vec![0x00, 0x01, 0x02, 0xFE]);
        let ev = NovelEvidence {
            project_id: 3,
            files: vec![garbage],
            description: "A valid description of how it was created here.".to_string(),
            trl: strong_trl(),
        };
        let v = GeniusNovel::adjudicate(&ev);
        assert!(!v.factualized);
        assert_eq!(v.rejection, NovelRejection::UnknownFormats);
    }

    #[test]
    fn rejects_when_trl_not_ready() {
        // Dokumentacion OK por TRL i laboratorëve s'e mbështet.
        let weak_trl = TrlInput {
            trl_level: 1, trl_passed: false, confidence: 0.3,
            reproducibility: 0.2, measurable: false, reproducible: false,
        };
        let ev = NovelEvidence {
            project_id: 4,
            files: vec![png_file(), mp4_file()],
            description: "A valid description of how the POC was assembled here.".to_string(),
            trl: weak_trl,
        };
        let v = GeniusNovel::adjudicate(&ev);
        assert!(!v.factualized);
        assert_eq!(v.rejection, NovelRejection::TrlNotReady);
    }

    #[test]
    fn register_produces_factual_innovation() {
        let ev = full_evidence();
        let v = GeniusNovel::adjudicate(&ev);
        assert!(v.factualized);
        let innovation = GeniusNovel::register(&ev, &v, 9999);
        assert_eq!(innovation.project_id, 7);
        assert_eq!(innovation.trl_level, 4);
        assert_eq!(innovation.visual_proofs, 2);
        assert_eq!(innovation.document_proofs, 1);
        assert_eq!(innovation.registered_at, 9999);
        // Vula e paprekshme jo-zero.
        assert_ne!(innovation.immutable_seal, 0);
    }

    #[test]
    fn empty_files_not_factual() {
        let ev = NovelEvidence {
            project_id: 5,
            files: vec![],
            description: "A description long enough to pass the check here.".to_string(),
            trl: strong_trl(),
        };
        let v = GeniusNovel::adjudicate(&ev);
        assert!(!v.factualized);
        // Pa skedarë → all_files_known false → UnknownFormats.
        assert_eq!(v.rejection, NovelRejection::UnknownFormats);
    }
}
