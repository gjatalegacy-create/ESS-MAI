// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  MEMORY/BOUNDARY.RS — Kontrata e Memorjes (Teoria 23)                 ║
// ║  GJATA LEGACY™ — Quantum Platform                                     ║
// ║                                                                          ║
// ║  RREGULLI ABSOLUT:                                                      ║
// ║   Quantum = ephemeral VETËM (vdes me ciklin, ttl=1)                   ║
// ║   Shadow  = persistent VETËM (Quantum KURRË s'shkruan)                ║
// ║                                                                          ║
// ║  KËTU: EphemeralStore real (jeton 1 cikël) + PersistentGuard që        ║
// ║  REFUZON çdo shkrim nga Quantum. Quantum NUK ka persistent store.      ║
// ║  Kjo është kontratë MOHUESE — dëshmon ligjin "Quantum s'mban memory".  ║
// ║                                                                          ║
// ║  Pseudo përdor IF/ELIF/RAISE; KONVERTUAR në match/Result (zero if).    ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use std::collections::HashMap;

// ─────────────────────────────────────────────────────────────────────────────
// MEMORY VIOLATION — gabimi kur shkelet kontrata
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum MemoryViolation {
    UnauthorizedEphemeralWrite(String),
    UnauthorizedPersistentWrite(String),
    UnknownScope(String),
}

// ─────────────────────────────────────────────────────────────────────────────
// WRITE AUTHORITY — kush shkruan ku (identike me pseudo)
// ─────────────────────────────────────────────────────────────────────────────

/// Modulet e Quantum që lejohen të shkruajnë EPHEMERAL.
const EPHEMERAL_WRITERS: [&str; 7] = [
    "QuantumOrchestrator", "QuantumReasoner", "QuantumBridge",
    "LAB_TRL3", "ELIMINATION", "CONVERGENCE", "ENTRY_POINTS",
];

/// I VETMI autoritet persistent (Shadow). Quantum s'është këtu — qëllimisht.
const PERSISTENT_WRITERS: [&str; 2] = [
    "shadow_gj_legacy.lgc", "LEGACY_CONTINUUM",
];

fn is_ephemeral_writer(caller: &str) -> bool {
    EPHEMERAL_WRITERS.iter().any(|w| *w == caller)
}

fn is_persistent_writer(caller: &str) -> bool {
    PERSISTENT_WRITERS.iter().any(|w| *w == caller)
}

// ─────────────────────────────────────────────────────────────────────────────
// EPHEMERAL ENTRY
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct EphemeralEntry {
    pub value:  String,
    pub caller: String,
}

// ─────────────────────────────────────────────────────────────────────────────
// EPHEMERAL STORE (Quantum) — vdes me ciklin, ttl=1
// ─────────────────────────────────────────────────────────────────────────────

pub struct EphemeralStore {
    cycle_id: String,
    data:     HashMap<String, EphemeralEntry>,
    pub ttl:  u32,  // 1 cikël
}

impl EphemeralStore {
    pub fn new(cycle_id: &str) -> Self {
        Self {
            cycle_id: cycle_id.to_string(),
            data: HashMap::new(),
            ttl: 1,
        }
    }

    /// put — vetëm shkrues të autorizuar. Zero if — match mbi autoritetin.
    pub fn put(&mut self, key: &str, value: &str, caller: &str) -> Result<(), MemoryViolation> {
        match is_ephemeral_writer(caller) {
            true => {
                self.data.insert(key.to_string(), EphemeralEntry {
                    value: value.to_string(),
                    caller: caller.to_string(),
                });
                Ok(())
            }
            false => Err(MemoryViolation::UnauthorizedEphemeralWrite(caller.to_string())),
        }
    }

    /// get — të gjithë lexojnë ephemeral.
    pub fn get(&self, key: &str) -> Option<&EphemeralEntry> {
        self.data.get(key)
    }

    /// flush — pastron gjithçka (fund cikli). Quantum s'mban memory.
    pub fn flush(&mut self) {
        self.data.clear();
    }

    pub fn len(&self) -> usize { self.data.len() }
    pub fn is_empty(&self) -> bool { self.data.is_empty() }
    pub fn cycle_id(&self) -> &str { &self.cycle_id }
}

// ─────────────────────────────────────────────────────────────────────────────
// PERSISTENT GUARD (Quantum-side) — REFUZON çdo shkrim Quantum
// ─────────────────────────────────────────────────────────────────────────────

/// Te Quantum, persistent-i është VETËM një gardian.
/// Çdo shkrim nga Quantum → MemoryViolation. Vetëm Shadow do kalonte.
/// Kjo dëshmon: Quantum NUK shkruan kurrë persistent.
pub struct PersistentGuard {
    rejected_attempts: u32,
}

impl PersistentGuard {
    pub fn new() -> Self {
        Self { rejected_attempts: 0 }
    }

    /// try_write — kontrollon autoritetin. Quantum → gjithmonë refuzohet.
    /// Zero if — match mbi is_persistent_writer.
    pub fn try_write(&mut self, caller: &str) -> Result<(), MemoryViolation> {
        match is_persistent_writer(caller) {
            // Vetëm Shadow/LEGACY_CONTINUUM — por te Quantum këto s'ekzistojnë.
            true => Ok(()),
            // Çdo modul Quantum → SHKELJE. Inkremento + refuzo.
            false => {
                self.rejected_attempts += 1;
                Err(MemoryViolation::UnauthorizedPersistentWrite(caller.to_string()))
            }
        }
    }

    pub fn rejected_count(&self) -> u32 { self.rejected_attempts }
}

impl Default for PersistentGuard {
    fn default() -> Self { Self::new() }
}

// ─────────────────────────────────────────────────────────────────────────────
// SCOPE
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MemoryScope {
    Ephemeral,
    Persistent,
}

// ─────────────────────────────────────────────────────────────────────────────
// BOUNDARY ENFORCER — runtime guard
// ─────────────────────────────────────────────────────────────────────────────

pub struct MemoryBoundary {
    pub ephemeral: EphemeralStore,
    pub persistent_guard: PersistentGuard,
}

impl MemoryBoundary {
    pub fn new(cycle_id: &str) -> Self {
        Self {
            ephemeral: EphemeralStore::new(cycle_id),
            persistent_guard: PersistentGuard::new(),
        }
    }

    /// write — drejton te store-i i duhur sipas scope. Zero if — match.
    pub fn write(&mut self, scope: MemoryScope, key: &str, value: &str, caller: &str)
        -> Result<(), MemoryViolation>
    {
        match scope {
            MemoryScope::Ephemeral => self.ephemeral.put(key, value, caller),
            // Persistent nga Quantum → gardiani refuzon.
            MemoryScope::Persistent => self.persistent_guard.try_write(caller),
        }
    }

    /// read — vetëm ephemeral te Quantum (persistent është te Shadow).
    pub fn read(&self, scope: MemoryScope, key: &str) -> Option<String> {
        match scope {
            MemoryScope::Ephemeral => self.ephemeral.get(key).map(|e| e.value.clone()),
            MemoryScope::Persistent => None,  // Quantum s'lexon persistent lokal
        }
    }

    /// end_cycle — flush ephemeral (Quantum s'mban memory mes cikleve).
    pub fn end_cycle(&mut self) {
        self.ephemeral.flush();
    }

    /// audit — gjendja aktuale.
    pub fn audit(&self) -> (usize, u32) {
        (self.ephemeral.len(), self.persistent_guard.rejected_count())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ephemeral_authorized_writer_ok() {
        let mut store = EphemeralStore::new("cycle1");
        assert!(store.put("k", "v", "ELIMINATION").is_ok());
        assert_eq!(store.get("k").unwrap().value, "v");
    }

    #[test]
    fn ephemeral_unauthorized_rejected() {
        let mut store = EphemeralStore::new("cycle1");
        let r = store.put("k", "v", "RANDOM_MODULE");
        assert_eq!(r.unwrap_err(), MemoryViolation::UnauthorizedEphemeralWrite("RANDOM_MODULE".to_string()));
    }

    #[test]
    fn flush_clears_everything() {
        let mut store = EphemeralStore::new("cycle1");
        store.put("k", "v", "CONVERGENCE").unwrap();
        assert_eq!(store.len(), 1);
        store.flush(); // Quantum s'mban memory
        assert_eq!(store.len(), 0);
    }

    #[test]
    fn persistent_guard_rejects_quantum() {
        let mut guard = PersistentGuard::new();
        // Çdo modul Quantum → refuzohet.
        assert!(guard.try_write("QuantumOrchestrator").is_err());
        assert!(guard.try_write("CONVERGENCE").is_err());
        assert_eq!(guard.rejected_count(), 2);
    }

    #[test]
    fn persistent_guard_allows_only_shadow() {
        let mut guard = PersistentGuard::new();
        // Vetëm Shadow do kalonte (te Quantum s'ekziston, por kontrata e lejon).
        assert!(guard.try_write("shadow_gj_legacy.lgc").is_ok());
    }

    #[test]
    fn boundary_persistent_write_from_quantum_fails() {
        let mut boundary = MemoryBoundary::new("c1");
        // Quantum tenton persistent → SHKELJE.
        let r = boundary.write(MemoryScope::Persistent, "k", "v", "QuantumReasoner");
        assert!(r.is_err());
    }

    #[test]
    fn boundary_ephemeral_dies_on_end_cycle() {
        let mut boundary = MemoryBoundary::new("c1");
        boundary.write(MemoryScope::Ephemeral, "k", "v", "ELIMINATION").unwrap();
        assert_eq!(boundary.audit().0, 1);
        boundary.end_cycle();
        assert_eq!(boundary.audit().0, 0);  // vdiq me ciklin
    }
}
