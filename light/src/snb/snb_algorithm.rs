// src/snb/snb_algorithm.rs
// _light_platform (Kumulativ)

use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct BugEvent {
    pub timestamp_ns: u64,
    pub module_name: String,
    pub description: String,
    pub flow_trace: Vec<String>,
    pub severity: u8,
}

#[derive(Debug, Clone)]
pub struct ShadowBugReport {
    pub timestamp_ns: u64,
    pub module_name: String,
    pub description: String,
    pub flow_trace: Vec<String>,
    pub severity: u8,
}

pub struct SnbAlgorithm {
    events: Vec<BugEvent>,
    current_flow: Vec<String>,
    has_bug: bool,
}

impl SnbAlgorithm {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            current_flow: Vec::new(),
            has_bug: false,
        }
    }

    pub fn record_module(&mut self, module_name: &str) {
        self.current_flow.push(module_name.to_string());
    }

    pub fn report_bug(&mut self, module_name: &str, description: &str, severity: u8) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;

        let event = BugEvent {
            timestamp_ns: timestamp,
            module_name: module_name.to_string(),
            description: description.to_string(),
            flow_trace: self.current_flow.clone(),
            severity,
        };

        self.events.push(event);
        self.has_bug = true;
    }

    pub fn prepare_for_shadow_snb(&self) -> Option<ShadowBugReport> {
        match self.has_bug && !self.events.is_empty() {
            false => return None,
            true => {}
        }
        let last = &self.events[self.events.len()-1];
        Some(ShadowBugReport {
            timestamp_ns: last.timestamp_ns,
            module_name: last.module_name.clone(),
            description: last.description.clone(),
            flow_trace: last.flow_trace.clone(),
            severity: last.severity,
        })
    }

    pub fn has_bug(&self) -> bool {
        self.has_bug
    }

    pub fn cleanup_if_no_bug(&mut self) {
        // Pastro vetëm kur s'ka bug. Zero if — match mbi has_bug.
        match self.has_bug {
            false => {
                self.events.clear();
                self.current_flow.clear();
            }
            true => {}
        }
    }

    pub fn clear(&mut self) {
        self.events.clear();
        self.current_flow.clear();
        self.has_bug = false;
    }
}

impl Default for SnbAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}
