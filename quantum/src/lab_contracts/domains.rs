// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAB_CONTRACTS/DOMAINS.RS — Regjistri Kanonik i Domeneve Bërthamë        ║
// ║  GJATA LEGACY™ — ESS-MAI (BYTE-FOR-BYTE IDENTIK në 3 platformat)         ║
// ║                                                                          ║
// ║  PSE EKZISTON: Shadow numëron domain-e UNIKE si String të papërpunuara   ║
// ║  (knowledge_lineage::unique_domains → HashSet<&str>) për ligjin          ║
// ║  Primitive→Legacy (≥5 domene, vula 0xA451). "Filozofi" ≠ "filozofi"     ║
// ║  do ta prishte numërimin. Ky regjistër garanton NJË çelës kanonik për   ║
// ║  çdo domen — i njëjti byte-varg në Light, Quantum dhe Shadow.            ║
// ║                                                                          ║
// ║  KUJDES: normalizimi këtu vlen VETËM për identifikatorë domeni.          ║
// ║  NUK prek tokenizimin gjuhësor të lingua-s (Normalizer/content_words).   ║
// ║                                                                          ║
// ║  DOMENET BËRTHAMË (të konfirmuara nga Arkitekti — POC i fuqishëm):      ║
// ║    1. industri_scada        6. kreativitet                               ║
// ║    2. filozofi              7. laborator                                 ║
// ║    3. matematike            8. inxhinieri                                ║
// ║    4. fizike_kuantike       9. kimi                                      ║
// ║    5. literature_akademike                                               ║
// ║  Slot i 10-të: i REZERVUAR — emërohet vetëm nga Arkitekti (s'shpiket).  ║
// ║                                                                          ║
// ║  LIGJE: zero if/else (vetëm match), zero varësi, vetëm kontratë.         ║
// ╚══════════════════════════════════════════════════════════════════════════╝

/// Domenet bërthamë të ESS-MAI — burimi i vetëm kanonik.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CoreDomain {
    IndustriScada,
    Filozofi,
    Matematike,
    FizikeKuantike,
    LiteratureAkademike,
    Kreativitet,
    Laborator,
    Inxhinieri,
    Kimi,
}

/// Numri i domeneve bërthamë të konfirmuara.
pub const CORE_DOMAIN_COUNT: usize = 9;

/// Të gjitha domenet — për iterim deterministik (rendi = rendi i deklarimit).
pub const ALL_CORE_DOMAINS: [CoreDomain; CORE_DOMAIN_COUNT] = [
    CoreDomain::IndustriScada,
    CoreDomain::Filozofi,
    CoreDomain::Matematike,
    CoreDomain::FizikeKuantike,
    CoreDomain::LiteratureAkademike,
    CoreDomain::Kreativitet,
    CoreDomain::Laborator,
    CoreDomain::Inxhinieri,
    CoreDomain::Kimi,
];

impl CoreDomain {
    /// Çelësi KANONIK — byte-vargu i vetëm i lejuar në zinxhirë primitivë,
    /// WAL, vault dhe çdo numërim unik (ASCII, lowercase, underscore).
    pub fn key(&self) -> &'static str {
        match self {
            Self::IndustriScada       => "industri_scada",
            Self::Filozofi            => "filozofi",
            Self::Matematike          => "matematike",
            Self::FizikeKuantike      => "fizike_kuantike",
            Self::LiteratureAkademike => "literature_akademike",
            Self::Kreativitet         => "kreativitet",
            Self::Laborator           => "laborator",
            Self::Inxhinieri          => "inxhinieri",
            Self::Kimi                => "kimi",
        }
    }

    /// Etiketa shqip (me diakritikë) — VETËM për shfaqje UI, kurrë për ruajtje.
    pub fn label_sq(&self) -> &'static str {
        match self {
            Self::IndustriScada       => "Industri / SCADA",
            Self::Filozofi            => "Filozofi",
            Self::Matematike          => "Matematikë",
            Self::FizikeKuantike      => "Fizikë Kuantike",
            Self::LiteratureAkademike => "Literaturë Akademike",
            Self::Kreativitet         => "Kreativitet",
            Self::Laborator           => "Laborator",
            Self::Inxhinieri          => "Inxhinieri",
            Self::Kimi                => "Kimi",
        }
    }

    /// Etiketa anglisht — VETËM për shfaqje UI.
    pub fn label_en(&self) -> &'static str {
        match self {
            Self::IndustriScada       => "Industry / SCADA",
            Self::Filozofi            => "Philosophy",
            Self::Matematike          => "Mathematics",
            Self::FizikeKuantike      => "Quantum Physics",
            Self::LiteratureAkademike => "Academic Literature",
            Self::Kreativitet         => "Creativity",
            Self::Laborator           => "Laboratory",
            Self::Inxhinieri          => "Engineering",
            Self::Kimi                => "Chemistry",
        }
    }

    /// Çelësi i territorit Quantum EKZISTUES (reasoning/territories.rs):
    /// "science" | "industrial" | "semantic" | "logistics".
    /// Lidhja e mbyllur kontratë→territor pa prekur politikat e territoreve.
    pub fn territory_key(&self) -> &'static str {
        match self {
            Self::IndustriScada       => "industrial",
            Self::Inxhinieri          => "industrial",
            Self::Matematike          => "science",
            Self::FizikeKuantike      => "science",
            Self::Kimi                => "science",
            Self::Laborator           => "science",
            Self::Filozofi            => "semantic",
            Self::LiteratureAkademike => "semantic",
            Self::Kreativitet         => "semantic",
        }
    }

    /// Parse STRIKT — pranon vetëm çelësin kanonik ekzakt.
    pub fn from_key(key: &str) -> Option<Self> {
        match key {
            "industri_scada"       => Some(Self::IndustriScada),
            "filozofi"             => Some(Self::Filozofi),
            "matematike"           => Some(Self::Matematike),
            "fizike_kuantike"      => Some(Self::FizikeKuantike),
            "literature_akademike" => Some(Self::LiteratureAkademike),
            "kreativitet"          => Some(Self::Kreativitet),
            "laborator"            => Some(Self::Laborator),
            "inxhinieri"           => Some(Self::Inxhinieri),
            "kimi"                 => Some(Self::Kimi),
            _                      => None,
        }
    }
}

/// Normalizim identifikatori domeni → çelës kanonik (nëse është domen bërthamë).
///
/// Pranon variante hyrjeje: shkronja të mëdha, diakritikë shqipe (ë→e, ç→c),
/// hapësira/vija/pjerrëse → '_', dhe i palos '_' e njëpasnjëshme.
/// "Fizikë/Kuantike" → "fizike_kuantike" → Some(FizikeKuantike).
///
/// KUFI I QARTË: ky normalizim është VETËM për çelësa domeni (identifikatorë
/// të shkurtër); tokenizimi i tekstit të përdoruesit mbetet te lingua dhe
/// NUK ndryshohet prej këtij funksioni.
pub fn normalize_domain_key(raw: &str) -> Option<&'static str> {
    let mut out = String::with_capacity(raw.len());
    for ch in raw.trim().chars() {
        let low: char = match ch.is_ascii_uppercase() {
            true  => ch.to_ascii_lowercase(),
            false => ch,
        };
        match low {
            'ë' | 'Ë'             => out.push('e'),
            'ç' | 'Ç'             => out.push('c'),
            ' ' | '-' | '/' | '\\' => {
                match out.ends_with('_') || out.is_empty() {
                    true  => {}
                    false => out.push('_'),
                }
            }
            c if c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' => {
                match c == '_' && (out.ends_with('_') || out.is_empty()) {
                    true  => {}
                    false => out.push(c),
                }
            }
            _ => {} // karaktere të tjera — injorohen (identifikator, jo tekst)
        }
    }
    let trimmed = out.trim_end_matches('_');
    CoreDomain::from_key(trimmed).map(|d| d.key())
}

// ════════════════════════════════════════════════════════════════════════════
// TESTE — kontrata e mbyllur provohet nga të tre platformat identikisht
// ════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_is_nine_confirmed() {
        assert_eq!(CORE_DOMAIN_COUNT, 9);
        assert_eq!(ALL_CORE_DOMAINS.len(), 9);
    }

    #[test]
    fn keys_unique_and_roundtrip() {
        let mut seen = std::collections::HashSet::new();
        for d in ALL_CORE_DOMAINS.iter() {
            let k = d.key();
            assert!(seen.insert(k), "çelës i dyfishtë: {}", k);
            assert_eq!(CoreDomain::from_key(k), Some(*d));
        }
    }

    #[test]
    fn keys_are_canonical_ascii() {
        for d in ALL_CORE_DOMAINS.iter() {
            let k = d.key();
            assert!(k.chars().all(|c| c.is_ascii_lowercase() || c == '_'),
                "çelës jo-kanonik: {}", k);
            assert!(!k.starts_with('_') && !k.ends_with('_'));
        }
    }

    #[test]
    fn normalize_variants_to_canonical() {
        assert_eq!(normalize_domain_key("Fizikë/Kuantike"), Some("fizike_kuantike"));
        assert_eq!(normalize_domain_key("  KIMI  "),        Some("kimi"));
        assert_eq!(normalize_domain_key("Literaturë Akademike"),
                   Some("literature_akademike"));
        assert_eq!(normalize_domain_key("industri-scada"),  Some("industri_scada"));
        assert_eq!(normalize_domain_key("Inxhinieri"),      Some("inxhinieri"));
    }

    #[test]
    fn normalize_rejects_unknown() {
        assert_eq!(normalize_domain_key("astrologji"), None);
        assert_eq!(normalize_domain_key(""),           None);
        assert_eq!(normalize_domain_key("___"),        None);
    }

    #[test]
    fn every_domain_maps_to_existing_quantum_territory() {
        let valid = ["science", "industrial", "semantic", "logistics"];
        for d in ALL_CORE_DOMAINS.iter() {
            assert!(valid.contains(&d.territory_key()),
                "territor i panjohur për {}", d.key());
        }
    }

    #[test]
    fn crystallization_precondition_five_unique_possible() {
        // Ligji Primitive→Legacy kërkon ≥5 domene unike — regjistri e mundëson.
        let unique: std::collections::HashSet<&str> =
            ALL_CORE_DOMAINS.iter().map(|d| d.key()).collect();
        assert!(unique.len() >= 5);
    }
}
