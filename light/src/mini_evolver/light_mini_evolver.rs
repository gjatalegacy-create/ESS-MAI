// src/mini_evolver/light_mini_evolver.rs
// _light_platform (Kumulativ)

// Light Mini Evolver - Complementary Layer (Faza 1 + 2)

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct KnowledgeUsageEvent {
    pub knowledge_id: u64,
    pub module_name: String,
    pub timestamp_ns: u64,
    pub linear_mass: f32,
    pub vector_score: f32,
}

pub struct LightMiniEvolver {
    usage_stats: HashMap<u64, Vec<KnowledgeUsageEvent>>,
    total_events: u64,
    enabled: bool,
}

impl LightMiniEvolver {
    pub fn new() -> Self {
        Self {
            usage_stats: HashMap::new(),
            total_events: 0,
            enabled: true,
        }
    }

    pub fn on_algorithm_step(
        &mut self,
        knowledge_id: u64,
        module_name: &str,
        linear_mass: f32,
        vector_score: f32,
    ) {
        match self.enabled {
            false => return,
            true => {}
        }

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;

        let event = KnowledgeUsageEvent {
            knowledge_id,
            module_name: module_name.to_string(),
            timestamp_ns: timestamp,
            linear_mass,
            vector_score,
        };

        self.usage_stats
            .entry(knowledge_id)
            .or_insert_with(Vec::new)
            .push(event);

        self.total_events += 1;
    }

    pub fn get_usage_count(&self, knowledge_id: u64) -> usize {
        self.usage_stats.get(&knowledge_id).map(|v| v.len()).unwrap_or(0)
    }

    pub fn get_average_linear_mass(&self, knowledge_id: u64) -> Option<f32> {
        let events = self.usage_stats.get(&knowledge_id)?;
        match events.is_empty() {
            true => return None,
            false => {}
        }
        Some(events.iter().map(|e| e.linear_mass).sum::<f32>() / events.len() as f32)
    }

    pub fn get_average_vector_score(&self, knowledge_id: u64) -> Option<f32> {
        let events = self.usage_stats.get(&knowledge_id)?;
        match events.is_empty() {
            true => return None,
            false => {}
        }
        Some(events.iter().map(|e| e.vector_score).sum::<f32>() / events.len() as f32)
    }

    pub fn get_top_used_knowledge(&self, limit: usize) -> Vec<(u64, usize)> {
        let mut usage: Vec<(u64, usize)> = self
            .usage_stats
            .iter()
            .map(|(&id, events)| (id, events.len()))
            .collect();
        usage.sort_by(|a, b| b.1.cmp(&a.1));
        usage.truncate(limit);
        usage
    }

    pub fn total_events(&self) -> u64 {
        self.total_events
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn clear(&mut self) {
        self.usage_stats.clear();
        self.total_events = 0;
    }
}

impl Default for LightMiniEvolver {
    fn default() -> Self {
        Self::new()
    }
}
