// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  NURA_CORE.RS — ESS-MAI LIGHT PLATFORM                                ║
// ║  Gjata Legacy™  |  Arkitekt: Bledar Gjata                             ║
// ║                                                                          ║
// ║  NURA + ALISAH + AMMY — një entitet i unifikuar sovran.               ║
// ║  Çdo funksion i 3 roleve bashkë nën një core.                         ║
// ║                                                                          ║
// ║  NURA   → Biznes, Industri, Governance (domain WORK)                  ║
// ║  ALISAH → Familje, Shtëpi, IoT/MATTER  (domain HOME)                 ║
// ║  AMMY   → Mobile, Ephemeral Web, Urë   (domain OUTSIDE)              ║
// ║                                                                          ║
// ║  Zero Cloud. Zero import external. Vetëm std::                        ║
// ║  Roli: ENTITY — entitet AI, koordinohet nga light_coordinator.        ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use std::collections::HashMap;
use std::sync::{Mutex, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::pd_light::VerifiedPdSurface;

// ════════════════════════════════════════════════════════════════════════════
// §1  DOMAIN — detektim dhe routing
// ════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, PartialEq)]
pub enum Domain {
    Work,     // NURA: biznes, industri, governance, SCADA
    Home,     // ALISAH: familje, shtëpi, IoT, MATTER
    Outside,  // AMMY: mobile, web, jashtë
}

impl Domain {
    /// Detekton domain nga teksti — shqip + anglisht
    pub fn detect(text: &str) -> Self {
        let t = text.to_lowercase();

        // Work/NURA keywords
        let work: &[&str] = &[
            "biznes","industri","scada","kontratë","governance","prodhim",
            "motor","sensor","vibration","pressure","plc","mes","cnc",
            "business","industry","contract","production","manufacturing",
            "enterprise","corporate","compliance","audit","invoice",
        ];
        // Home/ALISAH keywords
        let home: &[&str] = &[
            "shtëpi","familje","shtëpia","shtëpinë","matter","iot","pajisje",
            "ndriçim","temperaturë","termostati","llambë","kyç","kamera",
            "home","family","house","device","light","thermostat","lock","sensor",
            "appliance","kitchen","bedroom","living","garden",
        ];
        // Outside/AMMY keywords
        let outside: &[&str] = &[
            "mobile","web","jashtë","rrugë","internet","shfleto","kërko",
            "outside","browse","search","online","navigate","location","map",
            "travel","trip","route","weather","restaurant","store",
        ];

        let score_work    = work.iter().filter(|&&k| t.contains(k)).count();
        let score_home    = home.iter().filter(|&&k| t.contains(k)).count();
        let score_outside = outside.iter().filter(|&&k| t.contains(k)).count();

        match (score_home > score_work && score_home > score_outside,
               score_outside > score_work && score_outside > score_home) {
            (true, _) => Domain::Home,
            (_, true) => Domain::Outside,
            _         => Domain::Work, // default: NURA
        }
    }

    pub fn label(&self) -> &str {
        match self {
            Domain::Work    => "WORK→NURA",
            Domain::Home    => "HOME→ALISAH",
            Domain::Outside => "OUTSIDE→AMMY",
        }
    }

    pub fn code(&self) -> &str {
        match self {
            Domain::Work    => "WORK",
            Domain::Home    => "HOME",
            Domain::Outside => "OUTSIDE",
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// §2  SIGNAL — komunikim 72h ndër-rol (nga AMMY)
// ════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub struct Signal {
    pub id:                  String,
    pub origin:              String,  // NURA / ALISAH / AMMY
    pub target:              String,
    pub content:             String,
    pub emotional_weight:    f32,     // [0,1]
    pub strategic_relevance: f32,     // [0,1]
    pub ts_secs:             u64,
    pub expiry_secs:         u64,     // ts + 72h
}

impl Signal {
    pub fn new(
        origin: &str, target: &str, content: &str,
        emotional_weight: f32, strategic_relevance: f32,
    ) -> Self {
        let ts = now_secs();
        Self {
            id:                  format!("SIG_{}_{}", origin, ts),
            origin:              origin.to_string(),
            target:              target.to_string(),
            content:             content.to_string(),
            emotional_weight:    emotional_weight.clamp(0.0, 1.0),
            strategic_relevance: strategic_relevance.clamp(0.0, 1.0),
            ts_secs:             ts,
            expiry_secs:         ts + 72 * 3600,
        }
    }

    pub fn is_active(&self) -> bool {
        now_secs() < self.expiry_secs
    }

    pub fn age_hours(&self) -> f64 {
        (now_secs().saturating_sub(self.ts_secs)) as f64 / 3600.0
    }
}

pub struct SignalDb {
    signals: Mutex<Vec<Signal>>,
}

impl SignalDb {
    pub fn new() -> Self {
        Self { signals: Mutex::new(Vec::new()) }
    }

    pub fn add(&self, signal: Signal) {
        let mut s = self.signals.lock().unwrap_or_else(|p| p.into_inner());
        match s.len() < crate::ess_mai_system::limits::MAX_SIGNALS_ACTIVE {
            true => s.push(signal),
            false => {}
        }
    }

    pub fn for_target(&self, target: &str) -> Vec<Signal> {
        self.signals.lock().unwrap_or_else(|p| p.into_inner())
            .iter()
            .filter(|s| s.target == target && s.is_active())
            .cloned()
            .collect()
    }

    /// Pastron sinjalet e skaduara — kthen numrin e fshirë
    pub fn cleanup(&self) -> usize {
        let mut s = self.signals.lock().unwrap_or_else(|p| p.into_inner());
        let before = s.len();
        s.retain(|sig| sig.is_active());
        before - s.len()
    }

    pub fn stats(&self) -> (usize, usize) {
        let s = self.signals.lock().unwrap_or_else(|p| p.into_inner());
        let active = s.iter().filter(|sig| sig.is_active()).count();
        (s.len(), active)
    }
}

// ════════════════════════════════════════════════════════════════════════════
// §3  EMOTION — analizë emocionale (nga ALISAH)
// ════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, PartialEq)]
pub enum Emotion {
    Joy, Sadness, Anger, Fear, Surprise, Neutral,
}

impl Emotion {
    pub fn detect(text: &str) -> Self {
        let t = text.to_lowercase();

        let joy:      &[&str] = &["gëzim","lumtur","mirë","bukur","dashuri","bravo",
                                   "happy","great","wonderful","love","excellent","joy"];
        let sadness:  &[&str] = &["trishtim","keq","vështirë","mungon","humba",
                                   "sad","difficult","miss","lost","pain","sorry"];
        let anger:    &[&str] = &["zemërim","bezdisem","nuk dua","mjaft",
                                   "anger","frustrated","never","enough","stop","hate"];
        let fear:     &[&str] = &["frikë","anxioz","shqetës","rrezik",
                                   "fear","anxious","worried","danger","risk","scared"];
        let surprise: &[&str] = &["wow","o zot","çudit","befas","papritur",
                                   "incredible","amazing","unexpected","surprise","wow"];

        let sc = |kws: &[&str]| kws.iter().filter(|&&k| t.contains(k)).count();

        let scores = [
            (sc(joy),      Emotion::Joy),
            (sc(sadness),  Emotion::Sadness),
            (sc(anger),    Emotion::Anger),
            (sc(fear),     Emotion::Fear),
            (sc(surprise), Emotion::Surprise),
        ];

        scores.into_iter()
              .filter(|(s, _)| *s > 0)
              .max_by_key(|(s, _)| *s)
              .map(|(_, e)| e)
              .unwrap_or(Emotion::Neutral)
    }

    pub fn label(&self) -> &str {
        match self {
            Emotion::Joy      => "JOY",
            Emotion::Sadness  => "SADNESS",
            Emotion::Anger    => "ANGER",
            Emotion::Fear     => "FEAR",
            Emotion::Surprise => "SURPRISE",
            Emotion::Neutral  => "NEUTRAL",
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// §4  MATTER IoT — pajisjet (nga ALISAH)
// ════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, PartialEq)]
pub enum MatterDeviceType {
    Light, Thermostat, Lock, Sensor, Switch, Camera, Unknown,
}

impl MatterDeviceType {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "light"      => Self::Light,
            "thermostat" => Self::Thermostat,
            "lock"       => Self::Lock,
            "sensor"     => Self::Sensor,
            "switch"     => Self::Switch,
            "camera"     => Self::Camera,
            _            => Self::Unknown,
        }
    }

    pub fn label(&self) -> &str {
        match self {
            Self::Light      => "LIGHT",
            Self::Thermostat => "THERMOSTAT",
            Self::Lock       => "LOCK",
            Self::Sensor     => "SENSOR",
            Self::Switch     => "SWITCH",
            Self::Camera     => "CAMERA",
            Self::Unknown    => "UNKNOWN",
        }
    }
}

#[derive(Debug, Clone)]
pub struct MatterDevice {
    pub node_id:     u64,
    pub name:        String,
    pub device_type: MatterDeviceType,
    pub endpoint:    u16,
    pub online:      bool,
    pub last_seen:   u64,
}

impl MatterDevice {
    pub fn new(node_id: u64, name: &str, device_type: MatterDeviceType) -> Self {
        Self {
            node_id,
            name: name.to_string(),
            device_type,
            endpoint: 1,
            online: false,
            last_seen: now_secs(),
        }
    }

    pub fn status(&self) -> String {
        format!("{} [node:{}] type:{} online:{}",
            self.name, self.node_id,
            self.device_type.label(), self.online)
    }
}

pub struct MatterRegistry {
    devices: RwLock<HashMap<u64, MatterDevice>>,
}

impl MatterRegistry {
    pub fn new() -> Self {
        Self { devices: RwLock::new(HashMap::new()) }
    }

    pub fn register(&self, device: MatterDevice) -> Result<(), &'static str> {
        let mut d = self.devices.write().unwrap_or_else(|p| p.into_inner());
        match d.len() >= crate::ess_mai_system::limits::MAX_MATTER_DEVICES {
            true => return Err("MATTER_REGISTRY_FULL"),
            false => {}
        }
        d.insert(device.node_id, device);
        Ok(())
    }

    pub fn get(&self, node_id: u64) -> Option<MatterDevice> {
        self.devices.read().unwrap_or_else(|p| p.into_inner()).get(&node_id).cloned()
    }

    pub fn set_online(&self, node_id: u64, online: bool) {
        match self.devices.write().unwrap_or_else(|p| p.into_inner()).get_mut(&node_id) {
            Some(dev) => {
                dev.online = online;
                dev.last_seen = now_secs();
            }
            None => {}
        }
    }

    pub fn online_count(&self) -> usize {
        self.devices.read().unwrap_or_else(|p| p.into_inner()).values().filter(|d| d.online).count()
    }

    pub fn total_count(&self) -> usize {
        self.devices.read().unwrap_or_else(|p| p.into_inner()).len()
    }
}

// ════════════════════════════════════════════════════════════════════════════
// §5  DEVICE SESSION — identiteti mobile (nga AMMY)
// ════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub struct DeviceSession {
    pub device_id:      String,
    pub user_id:        String,
    pub active:         bool,
    pub registered_at:  u64,
}

pub struct DeviceRegistry {
    sessions: Mutex<HashMap<String, DeviceSession>>,
}

impl DeviceRegistry {
    pub fn new() -> Self {
        Self { sessions: Mutex::new(HashMap::new()) }
    }

    pub fn register(&self, device_id: &str, user_id: &str) -> Result<(), &'static str> {
        let mut s = self.sessions.lock().unwrap_or_else(|p| p.into_inner());
        match s.len() >= crate::ess_mai_system::limits::MAX_DEVICE_SESSIONS {
            true => return Err("DEVICE_REGISTRY_FULL"),
            false => {}
        }
        s.insert(device_id.to_string(), DeviceSession {
            device_id:     device_id.to_string(),
            user_id:       user_id.to_string(),
            active:        true,
            registered_at: now_secs(),
        });
        Ok(())
    }

    pub fn validate(&self, device_id: &str) -> bool {
        self.sessions.lock().unwrap_or_else(|p| p.into_inner())
            .get(device_id)
            .map(|s| s.active)
            .unwrap_or(false)
    }

    pub fn revoke(&self, device_id: &str) {
        match self.sessions.lock().unwrap_or_else(|p| p.into_inner()).get_mut(device_id) {
            Some(s) => s.active = false,
            None => {}
        }
    }

    pub fn active_count(&self) -> usize {
        self.sessions.lock().unwrap_or_else(|p| p.into_inner()).values().filter(|s| s.active).count()
    }
}

// ════════════════════════════════════════════════════════════════════════════
// §6  NURA CORE — entiteti i unifikuar
// ════════════════════════════════════════════════════════════════════════════

pub struct NuraCore {
    pub signals: SignalDb,
    pub matter:  MatterRegistry,
    pub devices: DeviceRegistry,
}

impl NuraCore {
    pub fn new() -> Self {
        Self {
            signals: SignalDb::new(),
            matter:  MatterRegistry::new(),
            devices: DeviceRegistry::new(),
        }
    }

    // ── Domain routing ─────────────────────────────────────────────────────
    pub fn route(&self, text: &str) -> Domain {
        Domain::detect(text)
    }

    // ── Emotion (ALISAH) ───────────────────────────────────────────────────
    pub fn emotion(&self, text: &str) -> Emotion {
        Emotion::detect(text)
    }

    // ── Signals (AMMY 72h) ─────────────────────────────────────────────────
    pub fn send_signal(&self, origin: &str, target: &str,
                       content: &str, ew: f32, sr: f32) {
        self.signals.add(Signal::new(origin, target, content, ew, sr));
    }

    pub fn signals_for(&self, target: &str) -> Vec<Signal> {
        self.signals.for_target(target)
    }

    pub fn cleanup_signals(&self) -> usize {
        self.signals.cleanup()
    }

    // ── MATTER IoT (ALISAH) ────────────────────────────────────────────────
    pub fn register_matter(&self, device: MatterDevice) -> Result<(), &'static str> {
        self.matter.register(device)
    }

    pub fn update_matter_online(&self, node_id: u64, online: bool) {
        self.matter.set_online(node_id, online);
    }

    pub fn get_matter(&self, node_id: u64) -> Option<MatterDevice> {
        self.matter.get(node_id)
    }

    // ── Device Identity (AMMY) ─────────────────────────────────────────────
    pub fn register_device(&self, device_id: &str, user_id: &str) -> Result<(), &'static str> {
        self.devices.register(device_id, user_id)
    }

    pub fn validate_device(&self, device_id: &str) -> bool {
        self.devices.validate(device_id)
    }

    pub fn revoke_device(&self, device_id: &str) {
        self.devices.revoke(device_id);
    }

    /// Zëri i identitetit ESS-MAI. Nura nuk arsyeton dhe nuk verifikon;
    /// vetëm i jep emër/sipërfaqe një mesazhi që ka ardhur i mbyllur nga
    /// PD Light pas verification receipt-it të Shadow.
    pub fn speak_pd(&self, verified_surface: &VerifiedPdSurface) -> String {
        format!("Nura: {}", verified_surface.as_str())
    }

    // ── Status ────────────────────────────────────────────────────────────
    pub fn status(&self) -> NuraCoreStatus {
        let (sig_total, sig_active) = self.signals.stats();
        NuraCoreStatus {
            matter_total:   self.matter.total_count(),
            matter_online:  self.matter.online_count(),
            signals_total:  sig_total,
            signals_active: sig_active,
            devices_active: self.devices.active_count(),
        }
    }
}

impl Default for NuraCore {
    fn default() -> Self { Self::new() }
}

#[derive(Debug)]
pub struct NuraCoreStatus {
    pub matter_total:   usize,
    pub matter_online:  usize,
    pub signals_total:  usize,
    pub signals_active: usize,
    pub devices_active: usize,
}

// ════════════════════════════════════════════════════════════════════════════
// §7  UTILITY
// ════════════════════════════════════════════════════════════════════════════

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
