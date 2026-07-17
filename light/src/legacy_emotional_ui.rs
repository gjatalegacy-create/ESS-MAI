// ╔══════════════════════════════════════════════════════════════════════════╗
// ║ LEGACY_EMOTIONAL_UI.RS — UI e vjetër emocionale → UI e re             ║
// ║ ESS-MAI v1.6.0                                                         ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::pd_light::PdUiContinuitySignal;
use crate::pd_spine_contract::FixedMass;

/// Komanda e tipizuar që udhëton në transportin ekzistues real:
/// Light stdout → Tauri `ask_nura`/`get_output` → EmotionalCommand → UI e re.
/// Nuk përmban tekst të Nura-s dhe nuk kryen reasoning.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegacyEmotionalTransmission {
    runtime_line: String,
    pub continuity_digest: u64,
}

impl LegacyEmotionalTransmission {
    pub fn as_str(&self) -> &str { &self.runtime_line }
}

pub struct LegacyEmotionalUi;

impl LegacyEmotionalUi {
    /// Merr sinjalin faktik të iZ nga PD Light dhe krijon komandën që UI-ja e
    /// vjetër transmeton te UI-ja e re përmes stdout-it ekzistues të Light-it.
    /// Ky modul nuk ndryshon output-in e Nura-s dhe nuk ekzekuton Layer 1/2/3.
    pub fn receive_and_transmit(
        signal: &PdUiContinuitySignal,
    ) -> Option<LegacyEmotionalTransmission> {
        match signal.continuity_digest != 0 {
            true => {
                let trust = FixedMass::new(
                    signal.accumulated_mass
                        .max(signal.structural_coherence)
                        .max(signal.genius_score),
                ).decimal_4();
                let line = format!(
                    "[PD_LIGHT/IZ] source=OLD_UI_EMOTIONAL_ENGINE target=NEW_UI \
                     session={} iz={} next_i0={} besim={} coherence={} genius={} \
                     y={} x={} digest={:016x}",
                    signal.session_id,
                    signal.iz_id,
                    signal.next_i0_id,
                    trust,
                    signal.structural_coherence,
                    signal.genius_score,
                    signal.y_verdict,
                    signal.x_verdict,
                    signal.continuity_digest,
                );
                Some(LegacyEmotionalTransmission {
                    runtime_line: line,
                    continuity_digest: signal.continuity_digest,
                })
            }
            false => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn signal(digest: u64) -> PdUiContinuitySignal {
        PdUiContinuitySignal {
            session_id: "s".into(),
            iz_id: "iz".into(),
            next_i0_id: "i0-next".into(),
            continuation_basis: "VERIFIED_POSITIVE".into(),
            accumulated_mass: 5000,
            structural_coherence: 7000,
            genius_score: 0,
            y_verdict: 1,
            x_verdict: 1,
            living_trust_intensity: 7000,
            living_trust_kind: 1,
            living_trust_digest: 8,
            project_id: 0,
            project_context_sha256: [0u8; 32],
            project_status: shadow_contracts::PROJECT_STATUS_NONE,
            novel_trl_level: 0,
            continuity_digest: digest,
        }
    }

    #[test]
    fn invalid_zero_digest_is_not_transmitted_to_new_ui() {
        assert!(LegacyEmotionalUi::receive_and_transmit(&signal(0)).is_none());
    }

    #[test]
    fn verified_iz_uses_real_light_to_tauri_transport_marker() {
        let tx = LegacyEmotionalUi::receive_and_transmit(&signal(9)).unwrap();
        assert!(tx.as_str().contains("[PD_LIGHT/IZ]"));
        assert!(tx.as_str().contains("target=NEW_UI"));
        assert!(tx.as_str().contains("besim=0.7000"));
    }
}
