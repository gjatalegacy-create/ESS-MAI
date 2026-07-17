// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAYER3/SHADOW_NIGHT_WATCH.RS — Sentineli i Jashtëm (Shadow)         ║
// ║  GJATA LEGACY™ — ESS-MAI Shadow                          (LAYER 3)    ║
// ║                                                                          ║
// ║  shadow_night_watch është një GJYKATËS tjetër — por gjykon duke marrë   ║
// ║  përditësime për policy + info nga web NËPËRMJET device (kurrë duke      ║
// ║  prekur web-in drejtpërdrejt). Verifikon me 7 shtresa info të gjera dhe  ║
// ║  të plota, dhe gjykon drejt 0/1. I lidhur me Shtresën 3 lab-to-lab.    ║
// ║                                                                          ║
// ║  KUFIRI KRITIK (nga orientimi):                                       ║
// ║   • Night Watch NUK hyn te shadow_judiciary — është sentinel i jashtëm. ║
// ║   • Vjen PARA gjykimit final, mbledh sinjale, gjykon 0/1 mbi to.        ║
// ║   • Nuk e zëvendëson verdiktin suprem — është një shtresë verifikimi    ║
// ║     shtesë me info të gjerë. Zero if/else.                            ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::lab_contracts_v11::watch::{NightWatchReport, SevenLayerInfo};

// ─────────────────────────────────────────────────────────────────────────────
// KONSTANTET — pragjet e gjykimit të sentinelit
// ─────────────────────────────────────────────────────────────────────────────

/// Pragu i agregatit të 7 shtresave për 0/1 = 1.
pub const NIGHT_WATCH_PASS: f32 = 0.65;
/// Numri minimal i burimeve për gjykim të vlefshëm.
pub const NIGHT_WATCH_MIN_SOURCES: u32 = 3;

// ─────────────────────────────────────────────────────────────────────────────
// NIGHT WATCH VERDICT — gjykimi i sentinelit
// ─────────────────────────────────────────────────────────────────────────────

/// Arsyeja e gjykimit të sentinelit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatchRejection {
    None,             // kaloi
    Incomplete,       // raporti i paplotë (7 shtresa/burime)
    LowConfidence,    // agregati nën prag
    NotViaDevice,     // s'u mblodh nëpërmjet device (shkelje parimi)
}

impl WatchRejection {
    pub fn label(self) -> &'static str {
        match self {
            WatchRejection::None          => "WATCH_NONE",
            WatchRejection::Incomplete    => "INCOMPLETE",
            WatchRejection::LowConfidence => "LOW_CONFIDENCE",
            WatchRejection::NotViaDevice  => "NOT_VIA_DEVICE",
        }
    }
}

/// Verdikti i sentinelit — 0/1 mbi 7 shtresat e info-s.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WatchVerdict {
    /// 0/1 — a kaloi verifikimin e gjerë.
    pub bit:        u8,
    /// Besueshmëria e agreguar [0,1].
    pub confidence: f32,
    /// Arsyeja nëse s'kaloi.
    pub rejection:  WatchRejection,
}

// ─────────────────────────────────────────────────────────────────────────────
// NIGHT WATCH — sentineli që gjykon mbi 7 shtresat
// ─────────────────────────────────────────────────────────────────────────────

pub struct NightWatch;

impl NightWatch {
    /// judge — gjykon 0/1 mbi raportin e 7 shtresave.
    ///
    /// Prioritet refuzimi (match): via device > plotësi > besueshmëri.
    /// Zero if — match shterues.
    pub fn judge(report: &NightWatchReport) -> WatchVerdict {
        let via_device = report.via_device;
        let complete = report.is_complete();
        let enough_sources = report.sources_count >= NIGHT_WATCH_MIN_SOURCES;
        let confidence = report.confidence();
        let conf_ok = confidence >= NIGHT_WATCH_PASS;

        // Vendimi + arsyeja (prioritet: device > plotësi+burime > besueshmëri).
        let (bit, rejection) = match (via_device, complete & enough_sources, conf_ok) {
            (false, _, _)    => (0u8, WatchRejection::NotViaDevice),
            (true, false, _) => (0u8, WatchRejection::Incomplete),
            (true, true, false) => (0u8, WatchRejection::LowConfidence),
            (true, true, true)  => (1u8, WatchRejection::None),
        };

        WatchVerdict {
            bit,
            confidence,
            rejection,
        }
    }

    /// gather_summary — përmbledhje e shkurtër e sinjaleve (për report). Zero if.
    pub fn gather_summary(report: &NightWatchReport) -> String {
        format!(
            "claim={} | confidence={:.2} | sources={} | via_device={}",
            report.claim_id,
            report.confidence(),
            report.sources_count,
            report.via_device
        )
    }

    /// strongest_layer — cila nga 7 shtresat është më e fortë. Zero if — fold.
    pub fn strongest_layer(info: &SevenLayerInfo) -> &'static str {
        // Tabela e shtresave me vlerat.
        let layers = [
            ("policy", info.policy_alignment),
            ("literature", info.literature_support),
            ("consensus", info.source_consensus),
            ("recency", info.recency),
            ("credibility", info.credibility),
            ("contradiction_free", info.contradiction_free),
            ("cross_coverage", info.cross_coverage),
        ];
        // Gjej max via fold (zero if).
        let best = layers.iter().fold(("none", -1.0f32), |acc, &(name, val)| {
            let take = (val > acc.1) as u32;
            // take=1 → (name,val); take=0 → acc. Zero if — match.
            match take {
                1 => (name, val),
                _ => acc,
            }
        });
        best.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn full_report() -> NightWatchReport {
        NightWatchReport {
            claim_id: 1,
            layers: SevenLayerInfo {
                policy_alignment: 0.8, literature_support: 0.7, source_consensus: 0.9,
                recency: 0.6, credibility: 0.85, contradiction_free: 0.75, cross_coverage: 0.7,
            },
            via_device: true, sources_count: 5, gathered_at: 100,
        }
    }

    #[test]
    fn passes_strong_report() {
        let v = NightWatch::judge(&full_report());
        assert_eq!(v.bit, 1);
        assert_eq!(v.rejection, WatchRejection::None);
        assert!(v.confidence >= NIGHT_WATCH_PASS);
    }

    #[test]
    fn rejects_not_via_device() {
        let mut r = full_report();
        r.via_device = false;
        let v = NightWatch::judge(&r);
        assert_eq!(v.bit, 0);
        assert_eq!(v.rejection, WatchRejection::NotViaDevice);
    }

    #[test]
    fn rejects_incomplete() {
        let mut r = full_report();
        r.sources_count = 1; // nën minimumin
        let v = NightWatch::judge(&r);
        assert_eq!(v.bit, 0);
        assert_eq!(v.rejection, WatchRejection::Incomplete);
    }

    #[test]
    fn rejects_low_confidence() {
        let mut r = full_report();
        // Të gjitha shtresat të ulëta por jo-zero (plot por dobët).
        r.layers = SevenLayerInfo {
            policy_alignment: 0.3, literature_support: 0.2, source_consensus: 0.3,
            recency: 0.2, credibility: 0.3, contradiction_free: 0.2, cross_coverage: 0.3,
        };
        let v = NightWatch::judge(&r);
        assert_eq!(v.bit, 0);
        assert_eq!(v.rejection, WatchRejection::LowConfidence);
    }

    #[test]
    fn strongest_layer_found() {
        let info = SevenLayerInfo {
            policy_alignment: 0.8, literature_support: 0.7, source_consensus: 0.95,
            recency: 0.6, credibility: 0.85, contradiction_free: 0.75, cross_coverage: 0.7,
        };
        assert_eq!(NightWatch::strongest_layer(&info), "consensus"); // 0.95
    }

    #[test]
    fn pending_report_rejected() {
        let r = NightWatchReport::pending(1);
        let v = NightWatch::judge(&r);
        assert_eq!(v.bit, 0); // pa shtresa/burime
    }
}
