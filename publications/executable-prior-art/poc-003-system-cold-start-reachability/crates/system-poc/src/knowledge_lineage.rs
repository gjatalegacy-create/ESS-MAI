//! New empty-store compatibility types plus the production `unique_domains` function.

use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct PrimitiveTrace {
    pub domain: String,
}

/// Numëron domain-et UNIKE në zinxhirin primitiv.
pub fn unique_domains(chain: &[PrimitiveTrace]) -> usize {
    let mut set: HashSet<&str> = HashSet::new();
    for t in chain {
        set.insert(t.domain.as_str());
    }
    set.len()
}

