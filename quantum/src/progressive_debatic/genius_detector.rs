// ╔══════════════════════════════════════════════════════════════════════════╗
// ║ PROGRESSIVE_DEBATIC / genius_detector.rs — fixed vector mass v1.5.6   ║
// ║ GeniusSignal mbetet sinjal kognitiv; të gjitha masat ruhen si u32.     ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use super::types::{pd_id, CognitiveSignal, EpistemicTrace, GeniusSignal};
use crate::pd_spine_contract::{FixedMass, MASS_SCALE};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct GeniusDetector {
    pub min_turns_before_check: u32,
    pub weight_returns:         u32,
    pub weight_energy:          u32,
    pub weight_shape:           u32,
    pub genius_threshold:       u32,
}

impl Default for GeniusDetector {
    fn default() -> Self {
        Self {
            min_turns_before_check: 2,
            weight_returns: 4_000,
            weight_energy: 3_000,
            weight_shape: 3_000,
            genius_threshold: 5_800,
        }
    }
}

impl GeniusDetector {
    pub fn evaluate(&self, trace: &EpistemicTrace) -> Option<GeniusSignal> {
        match trace.total_turns >= self.min_turns_before_check && trace.signals.len() >= 2 {
            true => {}
            false => return None,
        }

        let (returns_active, returns_score) = check_returns_to_idea(trace);
        let (energy_active, energy_score) = check_energy_increasing(trace);
        let (shape_active, shape_score) = check_concept_gaining_shape(trace);
        let active_count = [returns_active, energy_active, shape_active]
            .iter().filter(|&&active| active).count();
        match active_count {
            0 | 1 => return None,
            _ => {}
        }

        let genius_score = weighted_mass(&[
            (returns_score, self.weight_returns),
            (energy_score, self.weight_energy),
            (shape_score, self.weight_shape),
        ]);
        match genius_score >= self.genius_threshold {
            true => {}
            false => return None,
        }

        let detected_concept = match &trace.dominant_concept {
            Some(concept) => concept.clone(),
            None => most_frequent_concept(&trace.concept_map).unwrap_or_default(),
        };
        let ts = trace.last_updated_at;
        Some(GeniusSignal {
            signal_id: pd_id(&[trace.session_id.as_bytes(), &ts.to_le_bytes(), b"genius"]),
            session_id: trace.session_id.clone(),
            returns_to_idea: returns_active,
            energy_increasing: energy_active,
            concept_gaining_shape: shape_active,
            returns_score,
            energy_score,
            shape_score,
            genius_score,
            detected_concept,
            detected_at_turn: trace.total_turns,
            trace_ref: trace.trace_id.clone(),
            timestamp: ts,
        })
    }
}

pub fn check_returns_to_idea(trace: &EpistemicTrace) -> (bool, u32) {
    match trace.signals.len() >= 2 {
        true => {}
        false => return (false, 0),
    }
    let dominant = match most_frequent_concept(&trace.concept_map) {
        Some(value) => value,
        None => return (false, 0),
    };
    let signals_with_returns = trace.signals.iter()
        .filter(|signal| signal.returns_to.iter().any(|value| value == &dominant))
        .count() as u32;
    match signals_with_returns >= 2 {
        true => {}
        false => return (false, 0),
    }

    let raw_score = FixedMass::from_ratio(signals_with_returns, trace.signals.len() as u32).raw();
    let diversity_bonus = compute_return_diversity(&trace.signals, &dominant);
    let final_score = raw_score
        .saturating_add(scale_product(diversity_bonus, 2_000))
        .min(MASS_SCALE);
    (signals_with_returns >= 2 && final_score >= 3_500, final_score)
}

pub fn check_energy_increasing(trace: &EpistemicTrace) -> (bool, u32) {
    let trajectory = &trace.energy_trajectory;
    match trajectory.len() >= 2 {
        true => {}
        false => return (false, 0),
    }
    let half = trajectory.len() / 2;
    let first_half = mean(&trajectory[..half.max(1)]);
    let second_half = mean(&trajectory[half..]);
    match second_half > first_half {
        true => {}
        false => return (false, 0),
    }

    let delta = second_half.saturating_sub(first_half);
    let raw_score = delta.saturating_mul(2).min(MASS_SCALE);
    let count = trace.signals.len();
    let split = count.saturating_sub(3).max(1);
    let recent_lengths: Vec<u32> = trace.signals[count.saturating_sub(3)..]
        .iter().map(|signal| signal.content_length).collect();
    let earlier_lengths: Vec<u32> = trace.signals[..split]
        .iter().map(|signal| signal.content_length).collect();
    let recent_average = mean(&recent_lengths);
    let earlier_average = mean(&earlier_lengths);
    let length_bonus = match recent_average.saturating_mul(4) > earlier_average.saturating_mul(5) {
        true => 1_500,
        false => 0,
    };
    let final_score = raw_score.saturating_add(length_bonus).min(MASS_SCALE);
    (delta > 500 && final_score >= 3_000, final_score)
}

pub fn check_concept_gaining_shape(trace: &EpistemicTrace) -> (bool, u32) {
    match trace.signals.len() >= 2 {
        true => {}
        false => return (false, 0),
    }
    let densities: Vec<u32> = trace.signals.iter().map(|signal| signal.idea_density).collect();
    let density_trend = compute_trend(&densities);
    let positive_trend = density_trend.max(0) as u32;
    let primitive_mass = FixedMass::from_ratio(trace.implicit_primitives.len().min(5) as u32, 5).raw();
    let contradiction_mass = match trace.contradiction_pairs.is_empty() {
        true => 0,
        false => MASS_SCALE,
    };
    let shape_score = weighted_mass(&[
        (positive_trend, 3_500),
        (trace.structural_coherence, 3_500),
        (primitive_mass, 2_000),
        (contradiction_mass, 1_000),
    ]);
    (density_trend > 0 && shape_score >= 2_800, shape_score)
}

pub fn most_frequent_concept(map: &HashMap<String, u32>) -> Option<String> {
    map.iter().max_by_key(|entry| *entry.1).map(|entry| entry.0.clone())
}

pub fn compute_return_diversity(signals: &[CognitiveSignal], concept: &str) -> u32 {
    let returning: Vec<&CognitiveSignal> = signals.iter()
        .filter(|signal| signal.returns_to.iter().any(|value| value.as_str() == concept))
        .collect();
    match returning.len() >= 2 {
        true => {}
        false => return 0,
    }
    let mut unique = std::collections::HashSet::<String>::new();
    let mut total = 0u32;
    for signal in returning {
        for word in tokenize(&signal.raw_content) {
            unique.insert(word);
            total = total.saturating_add(1);
        }
    }
    FixedMass::from_ratio(unique.len() as u32, total).raw()
}

/// Trend fixed-point i nënshkruar në intervalin [-10_000, 10_000].
/// Ruan drejtimin e evolucionit pa float dhe pa humbur masën e vektorit.
pub fn compute_trend(values: &[u32]) -> i32 {
    match values.len() >= 2 {
        true => {}
        false => return 0,
    }
    let first = values.first().copied().unwrap_or(0) as i32;
    let last = values.last().copied().unwrap_or(0) as i32;
    (last - first).clamp(-(MASS_SCALE as i32), MASS_SCALE as i32)
}

pub fn mean(values: &[u32]) -> u32 {
    match values.is_empty() {
        true => 0,
        false => {
            let sum = values.iter().fold(0u64, |acc, value| acc.saturating_add(u64::from(*value)));
            (sum / values.len() as u64).min(u64::from(u32::MAX)) as u32
        }
    }
}

fn scale_product(value: u32, weight: u32) -> u32 {
    (u64::from(value).saturating_mul(u64::from(weight)) / u64::from(MASS_SCALE)) as u32
}

fn weighted_mass(terms: &[(u32, u32)]) -> u32 {
    let sum = terms.iter().fold(0u64, |acc, (value, weight)| {
        acc.saturating_add(u64::from(*value).saturating_mul(u64::from(*weight)))
    });
    (sum / u64::from(MASS_SCALE)).min(u64::from(MASS_SCALE)) as u32
}

pub fn tokenize(content: &str) -> Vec<String> {
    content
        .split(|character: char| !character.is_alphanumeric())
        .filter(|word| !word.is_empty())
        .map(|word| word.to_lowercase())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::core::ProgressiveDebatic;
    use super::super::types::DebateMode;

    fn excavation_trace() -> EpistemicTrace {
        let mut pd = ProgressiveDebatic::new();
        pd.open_session("sess-genius", "science", DebateMode::Philosophy, 1000);
        pd.ingest("sess-genius", "elimination reduces the candidate space", 1001).ok();
        pd.ingest("sess-genius", "again elimination removes wrong inputs and outputs here", 1002).ok();
        pd.ingest("sess-genius", "bidirectional elimination keeps only the matching pair which proves it", 1003).ok();
        pd.ingest("sess-genius", "so elimination is the operator and the decision becomes binary zero or one truly", 1004).ok();
        pd.session("sess-genius").expect("session").trace.clone()
    }

    #[test]
    fn helpers_are_deterministic() {
        assert_eq!(tokenize("Hello, World! 42"), vec!["hello", "world", "42"]);
        assert_eq!(mean(&[1_000, 2_000, 3_000]), 2_000);
        assert_eq!(mean(&[]), 0);
        assert!(compute_trend(&[1_000, 2_000, 3_000, 4_000]) > 0);
        assert!(compute_trend(&[4_000, 3_000, 2_000, 1_000]) < 0);
        assert_eq!(compute_trend(&[5_000]), 0);
    }

    #[test]
    fn no_genius_before_min_turns() {
        let mut pd = ProgressiveDebatic::new();
        pd.open_session("s1", "science", DebateMode::Philosophy, 0);
        pd.ingest("s1", "single message only", 1).ok();
        let trace = pd.session("s1").expect("session").trace.clone();
        assert!(GeniusDetector::default().evaluate(&trace).is_none());
    }

    #[test]
    fn genius_signals_use_fixed_mass() {
        let trace = excavation_trace();
        let (returns_active, returns_mass) = check_returns_to_idea(&trace);
        let (energy_active, energy_mass) = check_energy_increasing(&trace);
        let (shape_active, shape_mass) = check_concept_gaining_shape(&trace);
        let active = [returns_active, energy_active, shape_active]
            .iter().filter(|&&value| value).count();
        assert!(active >= 1);
        assert!(returns_mass <= MASS_SCALE);
        assert!(energy_mass <= MASS_SCALE);
        assert!(shape_mass <= MASS_SCALE);
    }
}
