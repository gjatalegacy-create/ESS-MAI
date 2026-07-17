// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  ESS_MANIFEST.RS — MANIFESTI ESS (Rust)                                ║
// ║  GJATA LEGACY™  — ESS MAI                                              ║
// ║                                                                          ║
// ║  Zëvendëson legacy_ess.py plotësisht.                                  ║
// ║  Thread-safe pa GIL. Singleton OnceLock.                               ║
// ║  SHA256 root fingerprint = 1 herë në boot.                             ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use std::collections::HashMap;
use std::sync::{Arc, RwLock, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

use crate::lgc_algorithm::compute_sha256;

// ─────────────────────────────────────────────────────────────────────────────
// TYPES
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModuleClass {
    Kernel,
    Light,
    Shadow,
    Quantum,
    Bridge,
    Ai,
    Algorithm,
    Outside,
}

impl ModuleClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Kernel    => "kernel",
            Self::Light     => "light",
            Self::Shadow    => "shadow",
            Self::Quantum   => "quantum",
            Self::Bridge    => "bridge",
            Self::Ai        => "ai",
            Self::Algorithm => "algorithm",
            Self::Outside   => "outside",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EssEntry {
    pub name:          String,
    pub coordinate:    String,        // GEN5:// address
    pub module_class:  ModuleClass,
    pub file_path:     String,
    pub sha256:        String,        // identity seal
    pub active:        bool,
    pub registered_at: u64,          // unix timestamp
    pub description:   String,
    pub version:       String,
}

impl EssEntry {
    pub fn new(
        name: &str, coordinate: &str,
        module_class: ModuleClass, file_path: &str, description: &str,
    ) -> Self {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let raw = format!("{}:{}:{}", name, coordinate, module_class.as_str());
        let sha256 = compute_sha256(raw.as_bytes());

        Self {
            name: name.to_string(),
            coordinate: coordinate.to_string(),
            module_class,
            file_path: file_path.to_string(),
            sha256,
            active: true,
            registered_at: ts,
            description: description.to_string(),
            version: "1.0".to_string(),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ESS MANIFEST — SINGLETON
// ─────────────────────────────────────────────────────────────────────────────

pub struct EssManifest {
    registry:      RwLock<HashMap<String, EssEntry>>,
    manifest_sha:  RwLock<String>,   // SHA256 i të gjithë manifestit
    // ── LEVEL 1 SHA (ROOT) ──────────────────────────────────────────────────
    // Llogaritet 1 herë në boot. KURRË nuk rillogaritet.
    root_sha:      OnceLock<String>,
}

static ESS: OnceLock<EssManifest> = OnceLock::new();

impl EssManifest {
    /// Singleton — krijuar automatikisht në first access
    pub fn instance() -> &'static Self {
        ESS.get_or_init(|| {
            let manifest = Self {
                registry:     RwLock::new(HashMap::new()),
                manifest_sha: RwLock::new(String::new()),
                root_sha:     OnceLock::new(),
            };
            manifest._boot_registry();
            manifest
        })
    }

    /// Boot — regjistron të gjithë modulet bazë
    fn _boot_registry(&self) {
        let entries: &[(&str, &str, ModuleClass, &str, &str)] = &[
            // ── KERNEL ────────────────────────────────────────────────────
            ("SHADOW_GJ_LEGACY", "GEN5://KERNEL/shadow_gj_legacy",
             ModuleClass::Kernel, "kernel/shadow_gj_legacy.c",
             "Porta diktatoriale — ligji i sistemit"),
            ("BUSS_LEGACY", "GEN5://KERNEL/buss_legacy",
             ModuleClass::Kernel, "kernel/buss_legacy.c",
             "Epiqendra e komunikimit"),
            // ── BRIDGE (Rust) ─────────────────────────────────────────────
            ("LGC_BRIDGE", "GEN5://BRIDGE/lgc",
             ModuleClass::Bridge, "src/lgc_bridge.rs",
             "Ura Rust→C kernel (zëvendëson Python/ctypes)"),
            ("LGC_ALGORITHM", "GEN5://ALGORITHM/lgc_sw",
             ModuleClass::Algorithm, "src/lgc_algorithm.rs",
             "KODUNIK generator — pure Rust HMAC"),
            ("ALGORITHM_CONTINUUM", "GEN5://ALGORITHM/continuum",
             ModuleClass::Algorithm, "src/algorithm_continuum.rs",
             "Mini → Algorithm → PD/iZ declaration"),
            ("PROGRESSIVE_DEBATIC", "GEN5://QUANTUM/progressive_debatic",
             ModuleClass::Quantum, "../quantum/src/progressive_debatic/mod.rs",
             "i₀ + 1Q → verified output + iZ → next i₀"),
            ("ESS_MANIFEST", "GEN5://LIGHT/ess_manifest",
             ModuleClass::Light, "src/ess_manifest.rs",
             "Manifesti ESS — Rust (zëvendëson legacy_ess.py)"),
            // ── AI RUST ───────────────────────────────────────────────────
            ("NURA", "GEN5://NURA",
             ModuleClass::Ai, "src/nura_core.rs",
             "Business/Industry/Governance/Advisory"),
            ("ALISAH", "GEN5://ALISAH",
             ModuleClass::Ai, "src/nura_core.rs",
             "Family/Matter/Knowledge — non-absorbable"),
            ("AMMY", "GEN5://AMMY",
             ModuleClass::Ai, "src/nura_core.rs",
             "Personal/Mobile/Bridge/Dialect"),
            // ── SOVEREIGN ─────────────────────────────────────────────────
            ("SOVEREIGN_CONSTITUTION", "GEN5://LIGHT/sovereign_constitution",
             ModuleClass::Light, "src/sovereign_constitution.rs",
             "Ligji i padiskutueshëm"),
            ("MORAL_CORE", "GEN5://LIGHT/moral_core",
             ModuleClass::Light, "src/moral_core.rs",
             "Filtri i moralit — non-absorbable"),
            ("LEGACY_CONTINUUM", "GEN5://LIGHT/legacy_continuum",
             ModuleClass::Light, "src/legacy_continuum.rs",
             "Memoria historike — non-absorbable"),
        ];

        {
            let mut reg = self.registry.write().unwrap_or_else(|p| p.into_inner());
            for &(name, coord, ref cls, path, desc) in entries {
                let e = EssEntry::new(name, coord, cls.clone(), path, desc);
                reg.insert(name.to_string(), e);
            }
        }

        self._update_manifest_sha();

        // ── ROOT SHA — llogaritet 1 herë, kurrë nuk rillogaritet ──────────
        let root_raw = format!(
            "NURA_LEGACY_ESS_MAI:{}:{}",
            "BLEDAR_GJATA",
            self.manifest_sha()
        );
        let root = compute_sha256(root_raw.as_bytes());
        self.root_sha.get_or_init(|| root.clone());

        tracing::info!(
            "ESS MANIFEST: {} module regjistruar | root_sha={}...",
            self.count(),
            &root[..16],
        );
    }

    /// Rillogarit SHA256 të manifestit (Level 2 — vetëm kur ndryshon registry)
    fn _update_manifest_sha(&self) {
        let reg = self.registry.read().unwrap_or_else(|p| p.into_inner());
        // Sorto për determinizëm
        let mut pairs: Vec<(&String, &String)> = reg
            .iter()
            .map(|(k, v)| (k, &v.sha256))
            .collect();
        pairs.sort_by_key(|(k, _)| k.as_str());

        let raw: String = pairs
            .iter()
            .map(|(k, v)| format!("{}:{}", k, v))
            .collect::<Vec<_>>()
            .join(";");

        let sha = compute_sha256(raw.as_bytes());
        *self.manifest_sha.write().unwrap_or_else(|p| p.into_inner()) = sha;
    }

    // ── PUBLIC API ────────────────────────────────────────────────────────────

    /// Root SHA — 1 SHA global (Level 1) — kurrë nuk ndryshon
    pub fn root_sha(&self) -> String {
        self.root_sha.get().cloned().unwrap_or_default()
    }

    /// Manifest SHA — Level 2, ndryshon vetëm kur regjistrohet modul i ri
    pub fn manifest_sha(&self) -> String {
        self.manifest_sha.read().unwrap_or_else(|p| p.into_inner()).clone()
    }

    pub fn register(
        &'static self,
        name: &str, coordinate: &str,
        module_class: ModuleClass, file_path: &str, description: &str,
    ) -> String {
        let e = EssEntry::new(name, coordinate, module_class, file_path, description);
        let sha = e.sha256.clone();
        {
            let mut reg = self.registry.write().unwrap_or_else(|p| p.into_inner());
            reg.insert(name.to_string(), e);
        }
        self._update_manifest_sha();
        sha
    }

    pub fn exists(&self, name: &str) -> bool {
        self.registry.read().unwrap_or_else(|p| p.into_inner()).contains_key(name)
    }

    pub fn get(&self, name: &str) -> Option<EssEntry> {
        self.registry.read().unwrap_or_else(|p| p.into_inner()).get(name).cloned()
    }

    pub fn count(&self) -> usize {
        self.registry.read().unwrap_or_else(|p| p.into_inner()).len()
    }

    pub fn report(&self) -> HashMap<String, serde_json::Value> {
        let reg = self.registry.read().unwrap_or_else(|p| p.into_inner());
        let mut by_class: HashMap<&str, usize> = HashMap::new();
        for e in reg.values() {
            *by_class.entry(e.module_class.as_str()).or_insert(0) += 1;
        }

        let mut map = HashMap::new();
        map.insert("total".into(), serde_json::json!(reg.len()));
        map.insert("root_sha".into(), serde_json::json!(&self.root_sha()[..16]));
        map.insert("manifest_sha".into(), serde_json::json!(&self.manifest_sha()[..16]));
        map.insert("by_class".into(), serde_json::json!(by_class));
        map
    }
}
