// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAYER3/MOD.RS — Shadow Layer 3 (Night Watch + Wisdom)             ║
// ║  GJATA LEGACY™ — ESS-MAI Shadow                                       ║
// ║                                                                          ║
// ║  Shtresa e tretë e Shadow-it: urtësi e thelluar, pa prekur identitetin. ║
// ║   • shadow_night_watch → sentinel i jashtëm (7 shtresa web-via-device)  ║
// ║   • shadow_wisdom      → sidecar + magazina e përbashkët (3 platformat) ║
// ║                                                                          ║
// ║  Night Watch s'hyn te judiciary; Wisdom s'është vendim. Zero if/else.  ║
// ╚══════════════════════════════════════════════════════════════════════════╝

pub mod shadow_night_watch;  // sentinel i jashtëm (gjykon 0/1 mbi 7 shtresa)
pub mod shadow_wisdom;       // sidecar + magazina e përbashkët
pub mod layer3_flow;         // orkestruesi i Shtresës 3 (night_watch + wisdom + magazina)

// ── Ri-eksportim ────────────────────────────────────────────────────────────
pub use shadow_night_watch::{
    NightWatch, WatchVerdict, WatchRejection,
    NIGHT_WATCH_PASS, NIGHT_WATCH_MIN_SOURCES,
};
pub use shadow_wisdom::{ShadowWisdom, WisdomWarehouse};
pub use layer3_flow::{Layer3Flow, Layer3FlowResult};
