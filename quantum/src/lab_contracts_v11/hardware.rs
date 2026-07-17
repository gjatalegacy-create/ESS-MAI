// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAB_CONTRACTS_V11/HARDWARE.RS — Kufijtë e Hardware (Shtresa 3)      ║
// ║  GJATA LEGACY™ — ESS-MAI (IDENTIK në Light + Quantum + Shadow)        ║
// ║                                                                          ║
// ║  Gjuha e re e Shtresës 3 për kufijtë fizikë. KONTRATË e pastër — vetëm  ║
// ║  matje/kufij, ZERO logjikë platforme, ZERO presion.                   ║
// ║                                                                          ║
// ║  • DeviceSnapshot → sinjalet e device-it (pasqyrë e pavarur, minimale)  ║
// ║  • HardwareBounds → kufijtë + shtytja që prod_formula prodhon          ║
// ║                                                                          ║
// ║  prod_formula (te Quantum HPRO) i mbush këto me formula fizike/matematike║
// ║  reale; këtu rri vetëm forma e përbashkët. Zero if/else.              ║
// ╚══════════════════════════════════════════════════════════════════════════╝

/// Pasqyrë minimale e gjendjes së device-it — e pavarur nga FFI-ja e Quantum-it.
/// (Quantum e mbush nga CHwSnapshot; kontrata mbetet universale.)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DeviceSnapshot {
    /// RAM e lirë (bytes).
    pub ram_free_bytes:   u64,
    /// RAM totale (bytes).
    pub ram_total_bytes:  u64,
    /// Ngarkesa e CPU-së [0,1].
    pub cpu_load:         f32,
    /// Bërthamat aktive.
    pub cores_active:     u32,
    /// Bërthamat totale.
    pub cores_total:      u32,
    /// Marzhi i energjisë [0,1] (1 = plot).
    pub energy_margin:    f32,
    /// A është me baterí.
    pub on_battery:       bool,
    /// Temperatura e vlerësuar (°C).
    pub temp_c:           f32,
    /// Kufiri termik (°C).
    pub temp_limit_c:     f32,
    /// Vula kohore (ns).
    pub timestamp:        u64,
}

impl DeviceSnapshot {
    /// idle — gjendje fillestare e sigurt (pa device real ende). Zero if.
    pub fn idle() -> Self {
        Self {
            ram_free_bytes:  0,
            ram_total_bytes: 0,
            cpu_load:        0.0,
            cores_active:    0,
            cores_total:     0,
            energy_margin:   1.0,
            on_battery:      false,
            temp_c:          0.0,
            temp_limit_c:    0.0,
            timestamp:       0,
        }
    }

    /// ram_free_ratio — raporti i RAM-it të lirë [0,1]. Zero if — formulë me mbrojtje.
    pub fn ram_free_ratio(&self) -> f32 {
        let total = self.ram_total_bytes.max(1) as f32;
        (self.ram_free_bytes as f32 / total).clamp(0.0, 1.0)
    }

    /// thermal_headroom — sa larg është nga kufiri termik [0,1]. Zero if.
    pub fn thermal_headroom(&self) -> f32 {
        let limit = self.temp_limit_c.max(1.0);
        let used = (self.temp_c / limit).clamp(0.0, 1.0);
        (1.0 - used).clamp(0.0, 1.0)
    }

    /// is_safe_to_deepen — a është e sigurt të thellohet (energ + termik)? Zero if.
    /// (Vetëm sinjal — vendimin e merr prod_formula te Quantum.)
    pub fn is_safe_to_deepen(&self) -> bool {
        let energy_ok = self.energy_margin >= 0.20;
        let thermal_ok = self.thermal_headroom() >= 0.15;
        energy_ok & thermal_ok
    }
}

/// Kufijtë + shtytja që prod_formula prodhon për një cikël thellimi.
/// Tregon SA thellë mund të shkojë sistemi pa rrezikuar device-in.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HardwareBounds {
    /// Sa bytes lejohen për këtë cikël (kufiri i RAM-it).
    pub ram_budget_bytes: u64,
    /// Sa operatorë paralel lejohen [1,4].
    pub parallel_limit:   u8,
    /// Shtytja e presionit të lejuar [0,1] (sa fort mund të shtyhet).
    pub pressure_push:    f32,
    /// A lejohet thellim i thellë (VeryDeep).
    pub deep_allowed:     bool,
    /// Marzhi i sigurisë i ruajtur [0,1] (sa % buffer lihet për device).
    pub safety_margin:    f32,
}

impl HardwareBounds {
    /// minimal — kufij minimalë të sigurt (kur device s'lejon thellim). Zero if.
    pub fn minimal() -> Self {
        Self {
            ram_budget_bytes: 0,
            parallel_limit:   1,
            pressure_push:    0.0,
            deep_allowed:     false,
            safety_margin:    1.0,
        }
    }

    /// allows_deepening — a lejojnë kufijtë thellim aktiv? Zero if.
    pub fn allows_deepening(&self) -> bool {
        let has_budget = self.ram_budget_bytes > 0;
        let has_push   = self.pressure_push > 0.0;
        has_budget & has_push
    }

    /// effective_parallel — paralelizmi efektiv (i kufizuar [1,4]). Zero if.
    pub fn effective_parallel(&self) -> u8 {
        self.parallel_limit.clamp(1, 4)
    }
}
