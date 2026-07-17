// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAB_CONTRACTS/COLLAPSE.RS — Kontrata e Kolapsit Sovran                  ║
// ║  GJATA LEGACY™ — ESS-MAI (BYTE-FOR-BYTE IDENTIK në 3 platformat)         ║
// ║                                                                          ║
// ║  FORMULA E GDHENDUR:                                                     ║
// ║    i0 → PA → (Xi, Yi) → +(-Xi)+(-Yi) → XiYi → XY → D ∈ {0,1}            ║
// ║                                                                          ║
// ║  KOLAPSI 3-FAZOR (formula NUK është modul — është vetë rrjedha):        ║
// ║    Light   = KOLAPS KOORDINIMI  — i0 gjurmohet, PA lind, split Xi/Yi    ║
// ║    Quantum = KOLAPS ARSYETIMI   — LIM mat 8 akset, shënon rrugët        ║
// ║              negative, NPRO eliminon (-Xi)+(-Yi), PRO kërkon i~,        ║
// ║              MPRO/PIM nxjerrin e paketojnë XY, NPIM paketon -X&-Y       ║
// ║    Shadow  = KOLAPS VERIFIKIMI  — XY ↔ (Xi,Yi) në pritje → D ∈ {0,1}   ║
// ║                                                                          ║
// ║  LIGJI: kolapsi është MONOTON — hapësira vetëm ngushtohet, kurrë        ║
// ║  s'zgjerohet; kur zbrazet → REFUZIM (D=0), kurrë shpikje përgjigjeje.   ║
// ║  Rrugët e eliminuara RUHEN si dije negative (aset, jo mbeturinë).       ║
// ║                                                                          ║
// ║  LIGJE KODI: zero if/else (vetëm match), zero varësi, zero clone i      ║
// ║  panevojshëm. Vetëm tipe + wire-format — asnjë reasoning këtu.          ║
// ╚══════════════════════════════════════════════════════════════════════════╝

/// Faza e kolapsit — ku ndodhet formula në rrjedhën 3-platformëshe.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollapsePhase {
    /// Light: i0 → PA → split (Xi, Yi). Koordinim, jo arsyetim.
    Coordination,
    /// Quantum: eliminim + kërkim i~ + nxjerrje XY. Arsyetim, jo vendim.
    Reasoning,
    /// Shadow: XY ↔ (Xi,Yi) → D ∈ {0,1}. Vendim, jo arsyetim i ri.
    Verification,
}

impl CollapsePhase {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Coordination => "KOLAPS_KOORDINIMI",
            Self::Reasoning    => "KOLAPS_ARSYETIMI",
            Self::Verification => "KOLAPS_VERIFIKIMI",
        }
    }
}

/// (Xi, Yi) — dy hapësirat primitive që lindin nga PA në Light.
///
/// Xi = hapësira e objektit HYRËS (kandidatët/kushtet e asaj që përshkruhet).
/// Yi = hapësira e objektit DALËS (boshtet e përgjigjes që kërkohet).
///
/// INVARIANTET (të detyruara nga ndërtuesi në Light):
///   • Xi ∩ Yi = ∅  (asnjë term në të dy hapësirat)
///   • Xi ∪ Yi = termat primitivë të PA-së (asgjë s'humbet, asgjë s'shpiket)
#[derive(Debug, Clone, PartialEq)]
pub struct PrimitiveSplit {
    pub xi: Vec<String>,
    pub yi: Vec<String>,
}

impl PrimitiveSplit {
    /// A ka material të mjaftueshëm për kolaps? (të dyja hapësirat jo-bosh
    /// nuk kërkohen — një pyetje mund të ketë vetëm Xi; POR të dyja bosh = jo)
    pub fn has_material(&self) -> bool {
        !(self.xi.is_empty() && self.yi.is_empty())
    }

    /// Wire-format për urat (append te rreshti i PA-së):
    ///   "xi:a,b,c|yi:d,e"  — terma pa '|' e ',' (të garantuar nga tokenizimi
    ///   i lingua-s: content_words janë fjalë alfabetike lowercase).
    pub fn to_wire(&self) -> String {
        format!("xi:{}|yi:{}", self.xi.join(","), self.yi.join(","))
    }

    /// Parse i wire-format: pranon "xi:...|yi:..." (dy segmentet e fundit
    /// të një rreshti PA 5-fushësh). Fail-safe: keqformim → None.
    pub fn from_wire(xi_seg: &str, yi_seg: &str) -> Option<Self> {
        let xi_body = xi_seg.strip_prefix("xi:")?;
        let yi_body = yi_seg.strip_prefix("yi:")?;
        let split_terms = |body: &str| -> Vec<String> {
            body.split(',')
                .map(|t| t.trim().to_string())
                .filter(|t| !t.is_empty())
                .collect()
        };
        Some(Self { xi: split_terms(xi_body), yi: split_terms(yi_body) })
    }
}

/// Rrugë e shënuar NEGATIVE gjatë kolapsit të arsyetimit:
/// LIM (nën pragjet e 8 akseve) e përcakton; NPRO e eliminon; NPIM e paketon;
/// Shadow e ruan si NegativeKnowledgeRecord (dije negative = ASET).
#[derive(Debug, Clone, PartialEq)]
pub struct NegativePath {
    /// Termi/rruga që u eliminua (-Xi ose -Yi).
    pub term:   String,
    /// Hapësira nga u eliminua: "Xi" | "Yi".
    pub space:  &'static str,
    /// Arsyeja e shkurtër (aksi LIM ose operatori që e rrëzoi).
    pub reason: String,
}

impl NegativePath {
    /// Wire kompakt për QuantumInbound → Shadow: "term~space~reason"
    /// (terma pa '~' nga tokenizimi; reason-i pastrohet nga '~').
    pub fn to_wire(&self) -> String {
        format!("{}~{}~{}", self.term, self.space,
            self.reason.replace('~', "-"))
    }

    pub fn from_wire(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.splitn(3, '~').collect();
        match parts.as_slice() {
            [term, space, reason] => {
                let space_static: &'static str = match *space {
                    "Xi" => "Xi",
                    "Yi" => "Yi",
                    _    => return None,
                };
                Some(Self {
                    term:   term.trim().to_string(),
                    space:  space_static,
                    reason: reason.trim().to_string(),
                })
            }
            _ => None,
        }
    }
}

/// Rezultati i kolapsit të arsyetimit (Quantum → Shadow):
/// ose çifti i mbijetuar XY me masën e matur (MPRO/PIM), ose refuzim i
/// ndershëm me arsye (hapësira u zbraz — sistemi NUK shpik përgjigje).
#[derive(Debug, Clone, PartialEq)]
pub enum CollapseOutcome {
    /// XiYi → XY: çifti minimal që mbijetoi eliminimin, gati për verifikim.
    Proposed { x: String, y: String, mass: f32 },
    /// Hapësira u zbraz ose s'pati material: kolaps i refuzuar (→ D=0).
    Refused  { reason: String },
}

impl CollapseOutcome {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Proposed { .. } => "XY_PROPOZUAR",
            Self::Refused  { .. } => "KOLAPS_REFUZUAR",
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// TESTE — kontrata e kolapsit provohet identikisht nga të tre platformat
// ════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_wire_roundtrip() {
        let s = PrimitiveSplit {
            xi: vec!["flamur".into(), "kuq".into(), "zi".into()],
            yi: vec!["shqiponjë".into()],
        };
        let wire = s.to_wire();
        assert_eq!(wire, "xi:flamur,kuq,zi|yi:shqiponjë");
        let segs: Vec<&str> = wire.split('|').collect();
        let back = PrimitiveSplit::from_wire(segs[0], segs[1]).expect("parse");
        assert_eq!(back, s);
    }

    #[test]
    fn split_empty_yi_allowed_but_both_empty_no_material() {
        let only_xi = PrimitiveSplit { xi: vec!["kimi".into()], yi: vec![] };
        assert!(only_xi.has_material());
        let none = PrimitiveSplit { xi: vec![], yi: vec![] };
        assert!(!none.has_material());
        // wire i zbrazët parsohet i sigurt
        let p = PrimitiveSplit::from_wire("xi:", "yi:").expect("parse bosh");
        assert!(!p.has_material());
    }

    #[test]
    fn negative_path_wire_roundtrip() {
        let n = NegativePath {
            term:   "pinguin".into(),
            space:  "Xi",
            reason: "semantic_integrity<prag".into(),
        };
        let back = NegativePath::from_wire(&n.to_wire()).expect("parse");
        assert_eq!(back, n);
        assert!(NegativePath::from_wire("a~KEQ~b").is_none()); // hapësirë e panjohur
    }

    #[test]
    fn outcome_labels_binary_semantics() {
        let p = CollapseOutcome::Proposed { x: "flamuri".into(), y: "shqipëria".into(), mass: 0.83 };
        let r = CollapseOutcome::Refused  { reason: "XiYi=∅".into() };
        assert_eq!(p.label(), "XY_PROPOZUAR");
        assert_eq!(r.label(), "KOLAPS_REFUZUAR");
    }

    #[test]
    fn phases_cover_three_platforms() {
        assert_eq!(CollapsePhase::Coordination.label(), "KOLAPS_KOORDINIMI");
        assert_eq!(CollapsePhase::Reasoning.label(),    "KOLAPS_ARSYETIMI");
        assert_eq!(CollapsePhase::Verification.label(), "KOLAPS_VERIFIKIMI");
    }
}
