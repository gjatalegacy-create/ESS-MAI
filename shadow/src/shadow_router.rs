// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  SHADOW_ROUTER.RS — S.ROUTER                                           ║
// ║  Gjata Legacy™  |  Arkitekt: Bledar Gjata                             ║
// ║                                                                          ║
// ║  Nodi i parë Shadow. Zgjidh rrugën: Fast / Standard / Deep.           ║
// ║  ZERO if/else. Seleksion vektorial nga skori i PassPackage.           ║
// ║                                                                          ║
// ║  FAST:     epistemic_mass < 0.36 — material i pamjaftueshëm           ║
// ║  STANDARD: epistemic_mass [0.36-0.98]                                  ║
// ║  DEEP:     epistemic_mass >= 0.99 — kandidat primitiv                  ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::types::{PassPackage, ShadowNode, ShadowPath, ShadowVerdict};

// ── PRAGJET e rrugëve — projektime vektoriale ────────────────────────────

const FAST_UPPER:     f32 = 0.36;
const DEEP_LOWER:     f32 = 0.99;

pub struct ShadowRouter;

impl ShadowRouter {
    /// Zgjidh rrugën nga epistemic_mass — ZERO if/else.
    ///
    /// Tre projektime mbi masën:
    ///   fast_w   = (FAST_UPPER − mass).max(0) / FAST_UPPER
    ///   deep_w   = (mass − DEEP_LOWER).max(0) / (1 − DEEP_LOWER)
    ///   std_w    = 1 − fast_w − deep_w        (komplementi)
    ///
    /// argmax → rruga.
    pub fn select_path(pkg: &PassPackage) -> ShadowPath {
        let m = pkg.epistemic_mass.clamp(0.0, 1.0);

        let fast_w  = (FAST_UPPER - m).max(0.0) / FAST_UPPER;
        let deep_w  = (m - DEEP_LOWER).max(0.0) / (1.0 - DEEP_LOWER + 0.001);
        let std_w   = (1.0 - fast_w - deep_w).max(0.0);

        let weights = [fast_w, std_w, deep_w];
        let idx = weights
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1)
                .unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
            .unwrap_or(1);

        match idx {
            0 => ShadowPath::Fast,
            2 => ShadowPath::Deep,
            _ => ShadowPath::Standard,
        }
    }

    /// Prodhon verdiktin e S.Router — gjithmonë 1 (routing nuk refuzon).
    pub fn verdict(pkg: &PassPackage, path: ShadowPath, ts_ns: u64) -> ShadowVerdict {
        // Router score = epistemic_mass — sa i fuqishëm është materiali
        let score = pkg.epistemic_mass.clamp(0.0, 1.0);

        ShadowVerdict::from_score(
            ShadowNode::Router,
            score,
            path as u8,
            ts_ns,
        )
    }
}
