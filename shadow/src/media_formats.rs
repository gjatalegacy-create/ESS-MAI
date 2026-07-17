// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  MEDIA_FORMATS.RS — Njohësit e Formateve (Dokumentacioni i Faktimit)  ║
// ║  Gjata Legacy™ | Arkitekt: Bledar Gjata                    (FAZA 9)    ║
// ║                                                                          ║
// ║  Të THJESHTË dhe të FORTË. Njohin formatet e dokumentacionit me të cilin║
// ║  i faktohet sistemit një TRL 4 (inovacion i prekshëm): foto, video,    ║
// ║  përshkrime/dokumente.                                                  ║
// ║                                                                          ║
// ║  Njohja bëhet me MAGIC BYTES (nënshkrimi në fillim të skedarit) — jo    ║
// ║  vetëm prapashtesa, sepse prapashtesa gënjehet. Magic bytes janë prova  ║
// ║  e vërtetë e formatit. Zero if/else — match mbi prefiks.              ║
// ╚══════════════════════════════════════════════════════════════════════════╝

// ─────────────────────────────────────────────────────────────────────────────
// MEDIA KIND — lloji i dokumentacionit
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaKind {
    Image,
    Video,
    Document,
    Unknown,
}

impl MediaKind {
    pub fn label(self) -> &'static str {
        match self {
            MediaKind::Image    => "IMAGE",
            MediaKind::Video    => "VIDEO",
            MediaKind::Document => "DOCUMENT",
            MediaKind::Unknown  => "UNKNOWN",
        }
    }

    /// is_valid_evidence — a është dokumentacion i njohur (jo Unknown)? Zero if.
    pub fn is_valid_evidence(self) -> bool {
        (self as u8) != (MediaKind::Unknown as u8)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// MEDIA FORMAT — formati specifik i njohur
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaFormat {
    // Imazhe
    Png,
    Jpeg,
    Gif,
    Webp,
    Bmp,
    // Video
    Mp4,
    WebM,
    Avi,
    Mkv,
    // Dokumente
    Pdf,
    Zip,    // docx/xlsx/pptx janë zip-containers
    PlainText,
    // E panjohur
    Unknown,
}

impl MediaFormat {
    pub fn label(self) -> &'static str {
        match self {
            MediaFormat::Png       => "PNG",
            MediaFormat::Jpeg      => "JPEG",
            MediaFormat::Gif       => "GIF",
            MediaFormat::Webp      => "WEBP",
            MediaFormat::Bmp       => "BMP",
            MediaFormat::Mp4       => "MP4",
            MediaFormat::WebM      => "WEBM",
            MediaFormat::Avi       => "AVI",
            MediaFormat::Mkv       => "MKV",
            MediaFormat::Pdf       => "PDF",
            MediaFormat::Zip       => "ZIP/OOXML",
            MediaFormat::PlainText => "TEXT",
            MediaFormat::Unknown   => "UNKNOWN",
        }
    }

    /// kind — lloji i përgjithshëm i këtij formati. Zero if — match.
    pub fn kind(self) -> MediaKind {
        match self {
            MediaFormat::Png | MediaFormat::Jpeg | MediaFormat::Gif
            | MediaFormat::Webp | MediaFormat::Bmp => MediaKind::Image,
            MediaFormat::Mp4 | MediaFormat::WebM
            | MediaFormat::Avi | MediaFormat::Mkv => MediaKind::Video,
            MediaFormat::Pdf | MediaFormat::Zip | MediaFormat::PlainText => MediaKind::Document,
            MediaFormat::Unknown => MediaKind::Unknown,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// HELPERS — prova e prefiksit (zero if)
// ─────────────────────────────────────────────────────────────────────────────

/// starts_with_at — a fillon bytes me sig në offset-in e dhënë? Zero if.
#[inline]
fn matches_at(bytes: &[u8], offset: usize, sig: &[u8]) -> bool {
    // Mjafton hapësira; pastaj krahaso element-për-element via iterator.
    let enough = bytes.len() >= offset + sig.len();
    let slice_eq = enough && bytes[offset..].iter().zip(sig.iter()).take(sig.len())
        .filter(|(a, b)| a == b).count() == sig.len();
    slice_eq
}

/// is_ascii_textual — a duket si tekst i pastër (ASCII printable + whitespace)?
/// Mostron deri në 512 bytes. Zero if — all() boolean.
#[inline]
fn is_ascii_textual(bytes: &[u8]) -> bool {
    let non_empty = !bytes.is_empty();
    let sample_len = bytes.len().min(512);
    let printable = bytes.iter().take(sample_len).all(|&b| {
        // printable ASCII (0x20–0x7E) OSE whitespace (tab/newline/cr).
        let is_print = (b >= 0x20) & (b <= 0x7E);
        let is_ws = (b == 0x09) | (b == 0x0A) | (b == 0x0D);
        is_print | is_ws
    });
    non_empty & printable
}

// ─────────────────────────────────────────────────────────────────────────────
// FORMAT DETECTOR — njohja me magic bytes
// ─────────────────────────────────────────────────────────────────────────────

pub struct FormatDetector;

impl FormatDetector {
    /// detect — njeh formatin nga magic bytes. Zero if — kaskadë boolean.
    ///
    /// Magic bytes (prova e vërtetë, jo prapashtesa):
    ///   PNG  : 89 50 4E 47
    ///   JPEG : FF D8 FF
    ///   GIF  : 47 49 46 38         ("GIF8")
    ///   WEBP : RIFF....WEBP        (RIFF në 0, WEBP në 8)
    ///   BMP  : 42 4D               ("BM")
    ///   MP4  : ....66 74 79 70     ("ftyp" në offset 4)
    ///   WEBM/MKV : 1A 45 DF A3     (EBML)
    ///   AVI  : RIFF....AVI         (RIFF në 0, AVI në 8)
    ///   PDF  : 25 50 44 46         ("%PDF")
    ///   ZIP  : 50 4B 03 04         ("PK\x03\x04" — docx/xlsx/pptx)
    pub fn detect(bytes: &[u8]) -> MediaFormat {
        // Imazhe
        let png  = matches_at(bytes, 0, &[0x89, 0x50, 0x4E, 0x47]);
        let jpeg = matches_at(bytes, 0, &[0xFF, 0xD8, 0xFF]);
        let gif  = matches_at(bytes, 0, &[0x47, 0x49, 0x46, 0x38]);
        let webp = matches_at(bytes, 0, &[0x52, 0x49, 0x46, 0x46])
                 & matches_at(bytes, 8, &[0x57, 0x45, 0x42, 0x50]); // WEBP
        let bmp  = matches_at(bytes, 0, &[0x42, 0x4D]);
        // Video
        let mp4  = matches_at(bytes, 4, &[0x66, 0x74, 0x79, 0x70]); // ftyp
        let ebml = matches_at(bytes, 0, &[0x1A, 0x45, 0xDF, 0xA3]); // webm/mkv
        let avi  = matches_at(bytes, 0, &[0x52, 0x49, 0x46, 0x46])
                 & matches_at(bytes, 8, &[0x41, 0x56, 0x49, 0x20]); // AVI
        // Dokumente
        let pdf  = matches_at(bytes, 0, &[0x25, 0x50, 0x44, 0x46]); // %PDF
        let zip  = matches_at(bytes, 0, &[0x50, 0x4B, 0x03, 0x04]); // PK..
        let text = is_ascii_textual(bytes);

        // Përparësia: nënshkrime binare specifike PARA tekstit të përgjithshëm.
        // Indeksim me tabelë: gjej të parin që përputhet (prioritet i fiksuar).
        // Zero if — match mbi tuplin e parë true.
        match (png, jpeg, gif, webp, bmp, mp4, ebml, avi, pdf, zip, text) {
            (true, _, _, _, _, _, _, _, _, _, _) => MediaFormat::Png,
            (_, true, _, _, _, _, _, _, _, _, _) => MediaFormat::Jpeg,
            (_, _, true, _, _, _, _, _, _, _, _) => MediaFormat::Gif,
            (_, _, _, true, _, _, _, _, _, _, _) => MediaFormat::Webp,
            (_, _, _, _, true, _, _, _, _, _, _) => MediaFormat::Bmp,
            (_, _, _, _, _, true, _, _, _, _, _) => MediaFormat::Mp4,
            (_, _, _, _, _, _, true, _, _, _, _) => MediaFormat::WebM, // EBML → webm/mkv
            (_, _, _, _, _, _, _, true, _, _, _) => MediaFormat::Avi,
            (_, _, _, _, _, _, _, _, true, _, _) => MediaFormat::Pdf,
            (_, _, _, _, _, _, _, _, _, true, _) => MediaFormat::Zip,
            (_, _, _, _, _, _, _, _, _, _, true) => MediaFormat::PlainText,
            _ => MediaFormat::Unknown,
        }
    }

    /// detect_kind — vetëm lloji i përgjithshëm (Image/Video/Document). Zero if.
    pub fn detect_kind(bytes: &[u8]) -> MediaKind {
        Self::detect(bytes).kind()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_png() {
        let png = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        assert_eq!(FormatDetector::detect(&png), MediaFormat::Png);
        assert_eq!(FormatDetector::detect(&png).kind(), MediaKind::Image);
    }

    #[test]
    fn detects_jpeg() {
        let jpeg = [0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10];
        assert_eq!(FormatDetector::detect(&jpeg), MediaFormat::Jpeg);
        assert_eq!(FormatDetector::detect(&jpeg).kind(), MediaKind::Image);
    }

    #[test]
    fn detects_gif() {
        let gif = [0x47, 0x49, 0x46, 0x38, 0x39, 0x61];
        assert_eq!(FormatDetector::detect(&gif), MediaFormat::Gif);
    }

    #[test]
    fn detects_webp() {
        // RIFF....WEBP
        let webp = [0x52, 0x49, 0x46, 0x46, 0x00, 0x00, 0x00, 0x00, 0x57, 0x45, 0x42, 0x50];
        assert_eq!(FormatDetector::detect(&webp), MediaFormat::Webp);
        assert_eq!(FormatDetector::detect(&webp).kind(), MediaKind::Image);
    }

    #[test]
    fn detects_mp4() {
        // ....ftyp
        let mp4 = [0x00, 0x00, 0x00, 0x18, 0x66, 0x74, 0x79, 0x70, 0x6D, 0x70, 0x34, 0x32];
        assert_eq!(FormatDetector::detect(&mp4), MediaFormat::Mp4);
        assert_eq!(FormatDetector::detect(&mp4).kind(), MediaKind::Video);
    }

    #[test]
    fn detects_webm_mkv() {
        let ebml = [0x1A, 0x45, 0xDF, 0xA3, 0x00, 0x00];
        assert_eq!(FormatDetector::detect(&ebml), MediaFormat::WebM);
        assert_eq!(FormatDetector::detect(&ebml).kind(), MediaKind::Video);
    }

    #[test]
    fn detects_avi() {
        let avi = [0x52, 0x49, 0x46, 0x46, 0x00, 0x00, 0x00, 0x00, 0x41, 0x56, 0x49, 0x20];
        assert_eq!(FormatDetector::detect(&avi), MediaFormat::Avi);
        assert_eq!(FormatDetector::detect(&avi).kind(), MediaKind::Video);
    }

    #[test]
    fn detects_pdf() {
        let pdf = [0x25, 0x50, 0x44, 0x46, 0x2D, 0x31, 0x2E, 0x34];
        assert_eq!(FormatDetector::detect(&pdf), MediaFormat::Pdf);
        assert_eq!(FormatDetector::detect(&pdf).kind(), MediaKind::Document);
    }

    #[test]
    fn detects_zip_ooxml() {
        // PK\x03\x04 — docx/xlsx/pptx
        let zip = [0x50, 0x4B, 0x03, 0x04, 0x14, 0x00];
        assert_eq!(FormatDetector::detect(&zip), MediaFormat::Zip);
        assert_eq!(FormatDetector::detect(&zip).kind(), MediaKind::Document);
    }

    #[test]
    fn detects_plain_text() {
        let text = b"This is a project description: how the POC was built.";
        assert_eq!(FormatDetector::detect(text), MediaFormat::PlainText);
        assert_eq!(FormatDetector::detect(text).kind(), MediaKind::Document);
    }

    #[test]
    fn unknown_for_garbage() {
        let garbage = [0x00, 0x01, 0x02, 0x03, 0xFE, 0xFF];
        assert_eq!(FormatDetector::detect(&garbage), MediaFormat::Unknown);
        assert_eq!(FormatDetector::detect(&garbage).kind(), MediaKind::Unknown);
    }

    #[test]
    fn empty_is_unknown() {
        assert_eq!(FormatDetector::detect(&[]), MediaFormat::Unknown);
    }

    #[test]
    fn kind_validity() {
        assert!(MediaKind::Image.is_valid_evidence());
        assert!(MediaKind::Video.is_valid_evidence());
        assert!(MediaKind::Document.is_valid_evidence());
        assert!(!MediaKind::Unknown.is_valid_evidence());
    }

    #[test]
    fn prefix_match_is_safe_on_short() {
        // Magic më i gjatë se bytes → s'ka panik, kthen false.
        let short = [0x89];
        assert_eq!(FormatDetector::detect(&short), MediaFormat::Unknown);
    }
}
