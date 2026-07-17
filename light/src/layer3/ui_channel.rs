// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAYER3/UI_CHANNEL.RS — Kanali UI (Light)                          ║
// ║  GJATA LEGACY™ — ESS-MAI Light                            (LAYER 3)    ║
// ║                                                                          ║
// ║  Kanali ku UI takohet me sistemin. DY drejtime:                       ║
// ║   • INPUT CHANNEL  → aty ku sapo futet inputi nga UI, merr GJURMIM      ║
// ║                      menjëherë (neural_law i jep kuptim hyrjes)         ║
// ║   • OUTPUT CHANNEL → aty ku rezultati bëhet GATI për UI (i pasuruar     ║
// ║                      me besueshmëri + kuptim, gati për shfaqje)         ║
// ║                                                                          ║
// ║  Ky është URA për UI-në me Tauri 2 — inputi hyn këtu, outputi del këtu.  ║
// ║                                                                          ║
// ║  KUFIRI: kanali vetëm TRANSPORTON dhe INTERPRETON (roli i Light) —      ║
// ║  s'vendos, s'gjykon, s'bën presion. Zero if/else.                     ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::layer3::neural_law::{NeuralLaw, MovementMeaning, FlowDirection};
use crate::lab_contracts_v11::packet::{Layer3Context, Layer3PayloadKind};

// ─────────────────────────────────────────────────────────────────────────────
// UI INPUT — çfarë vjen nga UI
// ─────────────────────────────────────────────────────────────────────────────

/// Inputi i papërpunuar nga UI (përpara gjurmimit).
#[derive(Debug, Clone, PartialEq)]
pub struct UiInput {
    /// Teksti i shkruar nga përdoruesi ("explore with Nura").
    pub text:       String,
    /// Sesioni (opsional, si String bosh kur s'ka).
    pub session_id: String,
    /// A është aktivizuar Deep Research nga UI.
    pub deep_research: bool,
}

impl UiInput {
    /// new — krijon input nga UI. Zero if.
    pub fn new(text: String, session_id: String, deep_research: bool) -> Self {
        Self { text, session_id, deep_research }
    }

    /// is_empty — a është inputi bosh? Zero if.
    pub fn is_empty(&self) -> bool {
        self.text.trim().is_empty()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// TRACED INPUT — inputi pasi merr gjurmim (në momentin e hyrjes)
// ─────────────────────────────────────────────────────────────────────────────

/// Inputi pasi ka marrë gjurmim menjëherë në hyrje.
/// Aty ku futet inputi, lind gjurma — kjo është pika ku UI lidhet.
#[derive(Debug, Clone, PartialEq)]
pub struct TracedInput {
    /// Inputi origjinal.
    pub input:      UiInput,
    /// Gjurma e caktuar (trace_id).
    pub trace_id:   u64,
    /// Kuptimi i interpretuar i hyrjes (nga neural_law).
    pub meaning:    MovementMeaning,
    /// Vula kohore e hyrjes.
    pub entered_at: u64,
}

impl TracedInput {
    /// RUNTIME HARDENING (      ): LIGJI "input s'hyn pa trace". Verifikon që hyrja
    /// mori gjurmë të vlefshme: trace_id jo-zero DHE tekst jo-bosh. Zero if — match.
    pub fn is_traced(&self) -> bool {
        match self.trace_id == 0 {
            true  => false,
            false => self.input.text.trim().is_empty() == false,
        }
    }

    /// reason_untraced — arsyeja specifike pse hyrja s'është e gjurmuar. Zero if — match.
    pub fn reason_untraced(&self) -> &'static str {
        match self.trace_id == 0 {
            true  => "ui_zero_trace_id",
            false => match self.input.text.trim().is_empty() {
                true  => "ui_empty_text",
                false => "traced",
            },
        }
    }
}

/// Rezultati i përgatitur për UI — i pasuruar, gati për shfaqje.
#[derive(Debug, Clone, PartialEq)]
pub struct UiOutput {
    /// Gjurma (lidh me inputin).
    pub trace_id:       u64,
    /// Teksti i rezultatit për shfaqje.
    pub display_text:   String,
    /// Domeni (WORK/HOME/OUTSIDE → NURA/ALISAH/AMMY).
    pub domain:         String,
    /// Sinjali i besueshmërisë për shfaqje [0,1].
    pub trust_signal:   f32,
    /// Gjendja DSL (VFYKNG/NEGKNG) për UI.
    pub algorithm_state: String,
    /// Kuptimi i daljes (nga neural_law).
    pub meaning:        String,
    /// A është gati për shfaqje.
    pub ready:          bool,
}

// ─────────────────────────────────────────────────────────────────────────────
// UI CHANNEL — ura dydrejtimëshe për UI
// ─────────────────────────────────────────────────────────────────────────────

pub struct UiChannel;

impl UiChannel {
    /// receive_input — INPUT CHANNEL: inputi nga UI merr gjurmim menjëherë.
    ///
    /// Aty ku futet inputi, lind gjurma. neural_law i jep kuptim hyrjes.
    /// Kjo është pika ku UI lidhet me sistemin. Zero if.
    pub fn receive_input(input: UiInput, trace_id: u64, entered_at: u64) -> TracedInput {
        // Konteksti i Shtresës 3 për këtë hyrje.
        let session_num = Self::session_to_num(&input.session_id);
        let context = Layer3Context::new(
            trace_id,
            0,            // claim_id ende i pacaktuar (lind në Quantum)
            session_num,
            String::new(), // territory caktohet nga coordinator
            trace_id,      // packet_id = trace_id në hyrje
            0,             // wisdom_ref ende bosh
        );

        // neural_law i jep kuptim hyrjes (interpretim, jo vendim).
        let meaning = NeuralLaw::interpret_movement(
            &context,
            Layer3PayloadKind::NeuralLaw,
            FlowDirection::Inbound,
        );

        TracedInput {
            input,
            trace_id,
            meaning,
            entered_at,
        }
    }

    /// prepare_output — OUTPUT CHANNEL: përgatit rezultatin gati për UI.
    ///
    /// Pasuron me besueshmëri + kuptim, e bën gati për shfaqje. Zero if — match.
    pub fn prepare_output(
        trace_id: u64,
        display_text: String,
        domain: String,
        trust_signal: f32,
        algorithm_state: String,
    ) -> UiOutput {
        // Konteksti për daljen.
        let context = Layer3Context::new(trace_id, 0, 0, domain.clone(), trace_id, 0);

        // neural_law i jep kuptim daljes.
        let meaning_obj = NeuralLaw::interpret_movement(
            &context,
            Layer3PayloadKind::NeuralLaw,
            FlowDirection::Outbound,
        );

        // Gati nëse ka tekst dhe gjendje.
        let has_text  = !display_text.is_empty();
        let has_state = !algorithm_state.is_empty();
        let ready = has_text & has_state;

        UiOutput {
            trace_id,
            display_text,
            domain,
            trust_signal: trust_signal.clamp(0.0, 1.0),
            algorithm_state,
            meaning: meaning_obj.meaning,
            ready,
        }
    }

    /// session_to_num — konverton session_id (String) në numër stabil. Zero if.
    /// (Hash i thjeshtë FNV-1a për konsistencë; bosh → 0.)
    fn session_to_num(session_id: &str) -> u64 {
        // FNV-1a mbi bytes (zero if — fold).
        let hash = session_id.bytes().fold(14695981039346656037u64, |acc, b| {
            let x = acc ^ (b as u64);
            x.wrapping_mul(1099511628211)
        });
        // Bosh → 0; ndryshe hash. Zero if — match mbi gjatësinë.
        let is_empty = session_id.is_empty();
        let mask = (!is_empty) as u64;  // 0 nëse bosh, 1 ndryshe
        hash.wrapping_mul(mask)
    }

    /// input_to_display — formaton inputin për gjurmim në UI (echo). Zero if.
    pub fn input_summary(traced: &TracedInput) -> String {
        format!(
            "[trace={} | {} | deep_research={}]",
            traced.trace_id,
            traced.meaning.meaning,
            traced.input.deep_research
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_receives_trace_immediately() {
        let input = UiInput::new("explore with Nura".to_string(), "sess1".to_string(), false);
        let traced = UiChannel::receive_input(input, 42, 1000);
        // Aty ku futet inputi, lind gjurma.
        assert_eq!(traced.trace_id, 42);
        assert_eq!(traced.meaning.direction, FlowDirection::Inbound);
        assert!(traced.meaning.meaning.contains("hyrje"));
    }

    #[test]
    fn empty_input_detected() {
        let input = UiInput::new("   ".to_string(), "".to_string(), false);
        assert!(input.is_empty());
    }

    #[test]
    fn output_prepared_ready() {
        let output = UiChannel::prepare_output(
            42, "Rezultati".to_string(), "WORK".to_string(), 0.8, "VFYKNG".to_string(),
        );
        assert!(output.ready);
        assert_eq!(output.trace_id, 42);
        assert!(output.meaning.contains("dalje"));
        assert_eq!(output.trust_signal, 0.8);
    }

    #[test]
    fn output_not_ready_without_state() {
        let output = UiChannel::prepare_output(
            42, "Rezultati".to_string(), "WORK".to_string(), 0.8, "".to_string(),
        );
        assert!(!output.ready); // pa algorithm_state
    }

    #[test]
    fn session_to_num_stable() {
        let n1 = UiChannel::session_to_num("sess1");
        let n2 = UiChannel::session_to_num("sess1");
        assert_eq!(n1, n2); // stabil
        // Bosh → 0.
        assert_eq!(UiChannel::session_to_num(""), 0);
    }

    #[test]
    fn deep_research_carried() {
        let input = UiInput::new("q".to_string(), "s".to_string(), true);
        let traced = UiChannel::receive_input(input, 1, 100);
        assert!(traced.input.deep_research);
        let summary = UiChannel::input_summary(&traced);
        assert!(summary.contains("deep_research=true"));
    }

    #[test]
    fn traced_input_fail_closed_requires_trace_and_text() {
        // LIGJI (      ): input s'hyn pa trace. trace_id jo-zero + tekst jo-bosh.
        let ok = UiChannel::receive_input(
            UiInput::new("pyetje".to_string(), "s".to_string(), false), 7, 100);
        assert!(ok.is_traced());
        assert_eq!(ok.reason_untraced(), "traced");

        // trace_id == 0 → i pagjurmuar.
        let zero = UiChannel::receive_input(
            UiInput::new("pyetje".to_string(), "s".to_string(), false), 0, 100);
        assert!(!zero.is_traced());
        assert_eq!(zero.reason_untraced(), "ui_zero_trace_id");

        // tekst bosh → i pagjurmuar.
        let empty = UiChannel::receive_input(
            UiInput::new("   ".to_string(), "s".to_string(), false), 7, 100);
        assert!(!empty.is_traced());
        assert_eq!(empty.reason_untraced(), "ui_empty_text");
    }

    #[test]
    fn channel_only_transports_interprets() {
        // Kanali jep kuptim (interpretim) por s'vendos.
        let input = UiInput::new("test".to_string(), "s".to_string(), false);
        let traced = UiChannel::receive_input(input, 1, 100);
        // Rezultati është gjurmë + kuptim, jo vendim.
        assert!(!traced.meaning.meaning.is_empty());
    }

    #[test]
    fn trust_signal_clamped() {
        let output = UiChannel::prepare_output(1, "t".to_string(), "d".to_string(), 1.5, "S".to_string());
        assert_eq!(output.trust_signal, 1.0); // clamp
    }
}
