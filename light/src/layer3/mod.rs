// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAYER3/MOD.RS — Light Layer 3 (Neural Law + Wisdom)              ║
// ║  GJATA LEGACY™ — ESS-MAI Light                                         ║
// ║                                                                          ║
// ║  Shtresa e tretë e Light-it: ligji neural interpretues + urtësia.       ║
// ║   • neural_law   → i jep kuptim çdo lëvizjeje/hyrjeje/daljeje/ligji      ║
// ║                    (sistemi flet si një trup) — POR kurrë s'vendos       ║
// ║   • light_wisdom → observon SI u formua besueshmëria                    ║
// ║                                                                          ║
// ║  Light interpreton, kurrë s'vendos. Zero if/else.                     ║
// ╚══════════════════════════════════════════════════════════════════════════╝

pub mod neural_law;    // ligji neural interpretues (sistemi flet si një trup)
pub mod light_wisdom;  // observon formimin e besueshmërisë
pub mod ui_channel;    // kanali UI (input me gjurmim + output gati)
pub mod layer3_flow;   // orkestruesi i Shtresës 3 (UI→gjurmim→urtësi→magazinë)
pub mod knowledge_channel; // kanali pip→Light(trace)→Shadow

// ── Ri-eksportim ────────────────────────────────────────────────────────────
pub use neural_law::{NeuralLaw, MovementMeaning, FlowDirection};
pub use light_wisdom::LightWisdom;
pub use ui_channel::{UiChannel, UiInput, TracedInput, UiOutput};
pub use layer3_flow::{LightLayer3Flow, LightLayer3Result};
pub use knowledge_channel::{KnowledgeChannel, PipUpload, TracedUpload, KnowledgePacket};
