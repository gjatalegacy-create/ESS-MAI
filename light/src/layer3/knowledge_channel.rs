// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAYER3/KNOWLEDGE_CHANNEL.RS — Kanali i Dijes së Ngarkuar (Light)    ║
// ║  GJATA LEGACY™ — ESS-MAI Light                            (LAYER 3)    ║
// ║                                                                          ║
// ║  Kanali ku knowledge i ngarkuar me pip TAKOHET me sistemin përmes Light. ║
// ║                                                                          ║
// ║  RRJEDHA (sovraniteti i inputit):                                     ║
// ║    pip upload → KËTU (Light i jep TRACE FILLESTAR + gjurmim) →          ║
// ║    Shadow true_knowledge (verifikon trace + ndan sipas domeneve).      ║
// ║                                                                          ║
// ║  PSE LIGHT I PARI: çdo knowledge duhet të gjurmohet PARA se të hyjë në   ║
// ║  Shadow. Light cakton trace_id jo-zero — kjo është prova që dija kaloi  ║
// ║  gjurmim. Pa këtë, Shadow e refuzon (sistemi e di gjithmonë origjinën). ║
// ║                                                                          ║
// ║  KUFIRI: Light vetëm GJURMON dhe TRANSPORTON dijen — s'e klasifikon      ║
// ║  (domain), s'e ruan, s'vendos. Atë i bën Shadow. Zero if/else.        ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::layer3::neural_law::{NeuralLaw, MovementMeaning, FlowDirection};
use crate::lab_contracts_v11::packet::{Layer3Context, Layer3PayloadKind};

// ─────────────────────────────────────────────────────────────────────────────
// PIP UPLOAD — knowledge i papërpunuar nga pip
// ─────────────────────────────────────────────────────────────────────────────

/// Knowledge i ngarkuar nga pip (përpara gjurmimit).
#[derive(Debug, Clone, PartialEq)]
pub struct PipUpload {
    /// Përmbajtja e dijes.
    pub content:     Vec<u8>,
    /// Burimi (psh "pip:scipy-docs").
    pub source:      String,
    /// Fjalëkyçi i domenit (Shadow do e klasifikojë).
    pub domain_hint: String,
}

impl PipUpload {
    /// new — krijon ngarkim pip. Zero if.
    pub fn new(content: Vec<u8>, source: String, domain_hint: String) -> Self {
        Self { content, source, domain_hint }
    }

    /// is_empty — a është përmbajtja bosh? Zero if.
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// TRACED UPLOAD — knowledge pasi mori trace fillestar
// ─────────────────────────────────────────────────────────────────────────────

/// Knowledge i ngarkuar pasi mori trace fillestar nga Light.
/// Gati për t'u dërguar te Shadow true_knowledge me provën e gjurmimit.
#[derive(Debug, Clone, PartialEq)]
pub struct TracedUpload {
    /// Ngarkimi origjinal.
    pub upload:     PipUpload,
    /// Trace-i fillestar (jo-zero = prova e gjurmimit).
    pub trace_id:   u64,
    /// Kuptimi i hyrjes (nga neural_law).
    pub meaning:    MovementMeaning,
    /// Paketa për Shadow (vula 500).
    pub packet:     KnowledgePacket,
    /// Vula kohore.
    pub traced_at:  u64,
}

/// Paketa minimale që mbart knowledge-un te Shadow me trace + domain hint.
#[derive(Debug, Clone, PartialEq)]
pub struct KnowledgePacket {
    pub trace_id:    u64,
    pub content:     Vec<u8>,
    pub source:      String,
    pub domain_hint: String,
    /// Vula 500 (0xA451).
    pub seal:        u32,
}

impl KnowledgePacket {
    pub const SEAL_VALID: u32 = 0xA451;

    /// seal_ok — a mban vulën 500? Zero if.
    pub fn seal_ok(&self) -> bool {
        ((self.seal & 0xFFFF) ^ 0xA5A5) == 500
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// KNOWLEDGE CHANNEL — gjurmon dijen e ngarkuar, e përgatit për Shadow
// ─────────────────────────────────────────────────────────────────────────────

pub struct KnowledgeChannel;

impl KnowledgeChannel {
    /// ingest_pip — pranon knowledge nga pip, i jep TRACE FILLESTAR + gjurmim.
    ///
    /// Light cakton trace_id (jo-zero = provë gjurmimi), neural_law i jep kuptim,
    /// paketohet për Shadow. Zero if.
    pub fn ingest_pip(upload: PipUpload, trace_id: u64, at: u64) -> TracedUpload {
        // Konteksti i Shtresës 3 për këtë ngarkim (me trace fillestar).
        let context = Layer3Context::new(
            trace_id,
            0,            // claim_id (knowledge s'është claim ende)
            0,            // session
            String::new(), // territory (Shadow do klasifikojë domain)
            trace_id,      // packet_id
            trace_id,      // wisdom_ref
        );

        // neural_law i jep kuptim hyrjes së dijes (interpretim, jo vendim).
        let meaning = NeuralLaw::interpret_movement(
            &context,
            Layer3PayloadKind::WisdomNote,
            FlowDirection::Inbound,
        );

        // Paketohet për Shadow me trace + domain hint + vula 500.
        let packet = KnowledgePacket {
            trace_id,
            content:     upload.content.clone(),
            source:      upload.source.clone(),
            domain_hint: upload.domain_hint.clone(),
            seal:        KnowledgePacket::SEAL_VALID,
        };

        TracedUpload {
            upload,
            trace_id,
            meaning,
            packet,
            traced_at: at,
        }
    }

    /// is_ready_for_shadow — a është gati për t'u dërguar te Shadow? Zero if.
    /// Gati = ka trace jo-zero + vula 500 + përmbajtje.
    pub fn is_ready_for_shadow(traced: &TracedUpload) -> bool {
        let has_trace = traced.trace_id != 0;
        let seal_ok   = traced.packet.seal_ok();
        let has_content = !traced.packet.content.is_empty();
        has_trace & seal_ok & has_content
    }

    /// trace_summary — përmbledhje e gjurmimit për log. Zero if.
    pub fn trace_summary(traced: &TracedUpload) -> String {
        format!(
            "[knowledge trace={} | source={} | domain_hint={} | {}]",
            traced.trace_id,
            traced.upload.source,
            traced.upload.domain_hint,
            traced.meaning.meaning
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_upload() -> PipUpload {
        PipUpload::new(
            b"scientific knowledge".to_vec(),
            "pip:numpy".to_string(),
            "mathematics".to_string(),
        )
    }

    #[test]
    fn pip_gets_initial_trace() {
        let traced = KnowledgeChannel::ingest_pip(sample_upload(), 42, 1000);
        // Light caktoi trace fillestar (jo-zero).
        assert_eq!(traced.trace_id, 42);
        assert_ne!(traced.trace_id, 0);
        // Hyrja u interpretua.
        assert_eq!(traced.meaning.direction, FlowDirection::Inbound);
    }

    #[test]
    fn packet_sealed_for_shadow() {
        let traced = KnowledgeChannel::ingest_pip(sample_upload(), 42, 1000);
        assert!(traced.packet.seal_ok()); // vula 500
        assert_eq!(traced.packet.trace_id, 42);
        // Domain hint përcillet (Shadow do klasifikojë).
        assert_eq!(traced.packet.domain_hint, "mathematics");
    }

    #[test]
    fn ready_for_shadow_when_complete() {
        let traced = KnowledgeChannel::ingest_pip(sample_upload(), 42, 1000);
        assert!(KnowledgeChannel::is_ready_for_shadow(&traced));
    }

    #[test]
    fn content_preserved_through_channel() {
        let traced = KnowledgeChannel::ingest_pip(sample_upload(), 42, 1000);
        // Përmbajtja kalon e paprekur (Light s'e ndryshon).
        assert_eq!(traced.packet.content, b"scientific knowledge".to_vec());
    }

    #[test]
    fn light_only_traces_not_classifies() {
        // Light jep trace + kuptim, por NUK klasifikon domain (lë hint për Shadow).
        let traced = KnowledgeChannel::ingest_pip(sample_upload(), 42, 1000);
        // domain_hint përcillet i papërpunuar (Shadow vendos domenin).
        assert_eq!(traced.packet.domain_hint, "mathematics");
        // Light s'ka fushë "domain" (vetëm hint).
        assert!(!traced.meaning.meaning.is_empty());
    }

    #[test]
    fn trace_summary_complete() {
        let traced = KnowledgeChannel::ingest_pip(sample_upload(), 42, 1000);
        let summary = KnowledgeChannel::trace_summary(&traced);
        assert!(summary.contains("trace=42"));
        assert!(summary.contains("pip:numpy"));
    }
}
