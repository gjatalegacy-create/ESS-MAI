use serde::{Deserialize, Serialize};

pub const EMOTIONAL_CONTRACT_VERSION: u32 = 0x0001_0600;
pub const MASS_SCALE: u32 = 10_000;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalCommand {
    pub contract_version: u32,
    pub trace_id: u64,
    pub phase: String,
    pub trust_mass: u32,
    pub intensity_mass: u32,
    pub color: String,
    pub motion: String,
    pub animation: String,
    pub source: String,
}

impl EmotionalCommand {
    pub fn from_runtime_output(trace_id: u64, runtime_output: &str) -> Self {
        let phase = detect_phase(runtime_output);
        let trust_mass = extract_fixed_mass(runtime_output, "besim=")
            .or_else(|| extract_fixed_mass(runtime_output, "strength="))
            .unwrap_or_else(|| default_trust(&phase));
        let intensity_mass = phase_intensity(&phase, trust_mass);
        let (color, motion, animation) = visual_projection(&phase);
        Self {
            contract_version: EMOTIONAL_CONTRACT_VERSION,
            trace_id,
            phase,
            trust_mass,
            intensity_mass,
            color: color.to_string(),
            motion: motion.to_string(),
            animation: animation.to_string(),
            source: "OLD_UI_EMOTIONAL_ENGINE".to_string(),
        }
    }
}

fn detect_phase(output: &str) -> String {
    match (
        output.contains("[PD_LIGHT/IZ]"),
        output.contains("[PD_CONTINUUM]"),
        output.contains("[PD_LIGHT]"),
        output.contains("[SHADOW_GJ_LEGACY]"),
        output.contains("[LIGHT_EMOTIONAL_SPINE]") || output.contains("[SPINE9]"),
        output.contains("[QUANTUM]"),
        output.contains("Nura:"),
    ) {
        // UI-ja emocionale dhe Nura janë paralele. Prania e tekstit "Nura:"
        // nuk lejohet të mbulojë komandën e iZ që UI-ja e vjetër transmeton.
        (true, _, _, _, _, _, _) => "PD_IZ_COMPLETED",
        (_, true, _, _, _, _, _) => "PD_IZ_COMPLETED",
        (_, _, true, _, _, _, _) => "PD_LIGHT",
        (_, _, _, true, _, _, _) => "SHADOW_VERIFICATION",
        (_, _, _, _, true, _, _) => "SPINE9_ACTIVE",
        (_, _, _, _, _, true, _) => "QUANTUM_ELIMINATION",
        (_, _, _, _, _, _, true) => "NURA_SPEAKING",
        _ => "LIGHT_COORDINATION",
    }.to_string()
}

fn default_trust(phase: &str) -> u32 {
    match phase {
        "NURA_SPEAKING"       => 10_000,
        "PD_LIGHT"            => 9_000,
        "PD_IZ_COMPLETED"     => 8_500,
        "SHADOW_VERIFICATION" => 7_500,
        "SPINE9_ACTIVE"       => 5_000,
        "QUANTUM_ELIMINATION" => 3_750,
        _                      => 2_500,
    }
}

fn phase_intensity(phase: &str, trust: u32) -> u32 {
    let base = match phase {
        "SHADOW_VERIFICATION" => 2_000,
        "QUANTUM_ELIMINATION" => 1_500,
        "PD_IZ_COMPLETED"     => 1_250,
        "SPINE9_ACTIVE"       => 1_000,
        _                      => 500,
    };
    trust.saturating_add(base).min(MASS_SCALE)
}

fn visual_projection(phase: &str) -> (&'static str, &'static str, &'static str) {
    match phase {
        "NURA_SPEAKING"       => ("NURA_GOLD", "FORWARD", "SPEAK_PULSE"),
        "PD_LIGHT"            => ("LIGHT_CYAN", "BREATHE", "LIGHT_RELEASE"),
        "PD_IZ_COMPLETED"     => ("NURA_GOLD", "ASCEND", "CONTINUUM_PULSE"),
        "SHADOW_VERIFICATION" => ("SHADOW_VIOLET", "FOCUS", "VERIFY_RING"),
        "SPINE9_ACTIVE"       => ("SPINE_BLUE", "ASCEND", "LAYER_CASCADE"),
        "QUANTUM_ELIMINATION" => ("QUANTUM_BLUE", "ROTATE", "ELIMINATION_ORBIT"),
        _                      => ("LIGHT_AMBER", "BREATHE", "COORDINATION_PULSE"),
    }
}

fn extract_fixed_mass(text: &str, marker: &str) -> Option<u32> {
    let start = text.find(marker)? + marker.len();
    let token: String = text[start..].chars()
        .take_while(|c| c.is_ascii_digit() || *c == '.')
        .collect();
    decimal_to_fixed(&token)
}

fn decimal_to_fixed(token: &str) -> Option<u32> {
    let mut parts = token.split('.');
    let whole = parts.next()?.parse::<u32>().ok()?;
    let fraction = parts.next().unwrap_or("0");
    let mut padded = fraction.chars().take(4).collect::<String>();
    while padded.len() < 4 { padded.push('0'); }
    let frac = padded.parse::<u32>().ok()?;
    Some(whole.saturating_mul(MASS_SCALE).saturating_add(frac).min(MASS_SCALE))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pd_continuum_becomes_emotional_command_not_nura_text() {
        let c = EmotionalCommand::from_runtime_output(
            8,
            "[PD_CONTINUUM] output=pdo-1 + iZ=iZ-2 → next-i₀=i0-3",
        );
        assert_eq!(c.phase, "PD_IZ_COMPLETED");
        assert_eq!(c.animation, "CONTINUUM_PULSE");
    }


    #[test]
    fn iz_emotion_remains_parallel_and_is_not_overwritten_by_nura_text() {
        let c = EmotionalCommand::from_runtime_output(
            10,
            "Nura: përgjigje e verifikuar\n[PD_LIGHT/IZ] target=NEW_UI besim=0.7000 digest=0000000000000009",
        );
        assert_eq!(c.phase, "PD_IZ_COMPLETED");
        assert_eq!(c.trust_mass, 7_000);
        assert_eq!(c.source, "OLD_UI_EMOTIONAL_ENGINE");
    }

    #[test]
    fn placeholder_is_replaced_by_runtime_command() {
        let c = EmotionalCommand::from_runtime_output(
            7,
            "[SPINE9] besim=0.500 [SHADOW_GJ_LEGACY] VERIFIED\nNura: përgjigje",
        );
        assert_eq!(c.phase, "SHADOW_VERIFICATION");
        assert_eq!(c.trust_mass, 5_000);
        assert_eq!(c.source, "OLD_UI_EMOTIONAL_ENGINE");
    }
}
