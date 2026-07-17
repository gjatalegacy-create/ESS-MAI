// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  ESS_MAI_SYSTEM.RS — ESS-MAI LIGHT PLATFORM                           ║
// ║  Gjata Legacy™  |  Arkitekt: Bledar Gjata                             ║
// ║                                                                          ║
// ║  Sistemi njeh vetveten. Zero enforce — Shadow enforcon.               ║
// ║  Deklaron: adresat, rolet, limitet, veprimet, ndërveprimet,           ║
// ║  komunikimet ndër-platformike + DSL algoritmik (nura_lgc_dsl).        ║
// ║                                                                          ║
// ║  Roli: DECLARATOR — di gjithçka, kontrollon asgjë.                   ║
// ╚══════════════════════════════════════════════════════════════════════════╝

// ════════════════════════════════════════════════════════════════════════════
// §1  ADRESAT — koordinata ESS:// e çdo moduli
//     Të gjitha 3 platformat të deklaruara këtu
// ════════════════════════════════════════════════════════════════════════════

pub mod addresses {
    // ── LIGHT PLATFORM ────────────────────────────────────────────────────
    pub const LIGHT_COORDINATOR:  &str = "ESS://LIGHT/light_coordinator";
    pub const TRACE_INFO:         &str = "ESS://LIGHT/trace_info";
    pub const EVOLVE_TRACE:       &str = "ESS://LIGHT/evolve_trace";
    pub const NURA_CORE:          &str = "ESS://LIGHT/nura_core";
    pub const SOFTWARE_CONTRACT:  &str = "ESS://LIGHT/software_contract";
    pub const ESS_MAI_SYSTEM:     &str = "ESS://LIGHT/ess_mai_system";
    pub const MANIFEST:           &str = "ESS://LIGHT/manifest";
    pub const LIGHT_BUSS:         &str = "ESS://LIGHT/light_buss";
    pub const LGC_ALGORITHM:      &str = "ESS://LIGHT/lgc_algorithm";
    pub const LEGALGOCRYPT:       &str = "ESS://LIGHT/legalgocrypt";
    pub const LGC_BRIDGE:         &str = "ESS://LIGHT/lgc_bridge";
    pub const ESS_MANIFEST:       &str = "ESS://LIGHT/ess_manifest";
    pub const ALGORITHM_CONTINUUM: &str = "ESS://LIGHT/algorithm_continuum";

    // ── SHADOW PLATFORM ───────────────────────────────────────────────────
    pub const SHADOW_JUDICIARY:   &str = "ESS://SHADOW/shadow_judiciary";
    pub const SHADOW_KERNEL:      &str = "ESS://SHADOW/shadow_kernel";
    pub const NK_SYSTEM:          &str = "ESS://SHADOW/nk_system";
    pub const LEGACY_CONTINUUM:   &str = "ESS://SHADOW/legacy_continuum";
    pub const VOK_STORE:          &str = "ESS://SHADOW/vok_store";
    pub const VNK_STORE:          &str = "ESS://SHADOW/vnk_store";
    pub const SHADOW_BUSS:        &str = "ESS://SHADOW/shadow_buss";

    // ── QUANTUM PLATFORM ──────────────────────────────────────────────────
    pub const QUANTUM_PLATFORM:   &str = "ESS://QUANTUM/quantum_platform";
    pub const LIGHT_BRIDGE:       &str = "ESS://QUANTUM/light_bridge";
    pub const LIM_ENGINE:         &str = "ESS://QUANTUM/lim_engine";
    pub const PRO_ENGINE:         &str = "ESS://QUANTUM/pro_engine";
    pub const SRK_ENGINE:         &str = "ESS://QUANTUM/srk_engine";
    pub const PIM_ENGINE:         &str = "ESS://QUANTUM/pim_engine";
    pub const CONVERGENCE:        &str = "ESS://QUANTUM/convergence";
    pub const RING_BRIDGE:        &str = "ESS://QUANTUM/ring_bridge";
    pub const EBPF_HYDRATOR:      &str = "ESS://QUANTUM/ebpf_hydrator";
    pub const PROGRESSIVE_DEBATIC: &str = "ESS://QUANTUM/progressive_debatic";

    /// Kthen platformën nga adresa
    pub fn platform_of(addr: &str) -> &str {
        match (addr.starts_with("ESS://LIGHT/"),
               addr.starts_with("ESS://SHADOW/"),
               addr.starts_with("ESS://QUANTUM/")) {
            (true, _, _) => "LIGHT",
            (_, true, _) => "SHADOW",
            (_, _, true) => "QUANTUM",
            _            => "UNKNOWN",
        }
    }

    /// Kthen emrin e modulit nga adresa e plotë
    pub fn module_name(addr: &str) -> &str {
        addr.rsplit('/').next().unwrap_or(addr)
    }
}

// ════════════════════════════════════════════════════════════════════════════
// §2  PLATFORMAT — kufijt absolutë
// ════════════════════════════════════════════════════════════════════════════

pub mod platform {
    #[derive(Debug, Clone, PartialEq)]
    pub enum Platform {
        Light,    // Koordinues — hyrje/dalje, trace, kontrata
        Shadow,   // Truth Authority — verifikon 0/1, mban memory
        Quantum,  // Reasoner — propozon vetëm, kurrë nuk vendos
    }

    impl Platform {
        pub fn label(&self) -> &str {
            match self {
                Platform::Light   => "LIGHT",
                Platform::Shadow  => "SHADOW",
                Platform::Quantum => "QUANTUM",
            }
        }

        pub fn authority_level(&self) -> u8 {
            match self {
                Platform::Shadow  => 3, // suprem
                Platform::Light   => 2, // koordinues
                Platform::Quantum => 1, // propozues
            }
        }
    }

    pub fn classify(module: &str) -> Platform {
        // Modulet Shadow
        let shadow = ["shadow_judiciary", "shadow_kernel", "nk_system",
                      "legacy_continuum", "vok_store", "vnk_store", "shadow_buss"];
        // Modulet Quantum
        let quantum = ["lim_engine", "pro_engine", "srk_engine", "pim_engine",
                       "convergence", "ring_bridge", "ebpf_hydrator",
                       "progressive_debatic"];
        match shadow.contains(&module) {
            true => return Platform::Shadow,
            false => {}
        }
        match quantum.contains(&module) {
            true => return Platform::Quantum,
            false => {}
        }
        Platform::Light
    }
}

// ════════════════════════════════════════════════════════════════════════════
// §3  ROLET — çdo modul ka rol të saktë dhe të pandryshueshëm
// ════════════════════════════════════════════════════════════════════════════

pub mod roles {
    #[derive(Debug, Clone, PartialEq)]
    pub enum Role {
        Coordinator,  // orkestron, rout-on — zero vendim
        Tracer,       // vë vulë dhe ndjek inputin
        Enforcer,     // verifikon kontratat në runtime
        Entity,       // AI entity (NURA unified)
        Declarator,   // deklaron, di — nuk enforcon
        Algorithm,    // algoritëm i pastër, zero gjendje
        Continuum,    // Mini→Algorithm→PD: i₀ + 1Q → output + iZ → next i₀
        Bridge,       // urë ndër-platformike
        Bus,          // komunikimi backbone
        Kernel,       // C-level enforcement
    }

    impl Role {
        pub fn label(&self) -> &str {
            match self {
                Role::Coordinator => "COORDINATOR",
                Role::Tracer      => "TRACER",
                Role::Enforcer    => "ENFORCER",
                Role::Entity      => "ENTITY",
                Role::Declarator  => "DECLARATOR",
                Role::Algorithm   => "ALGORITHM",
                Role::Continuum   => "PD_CONTINUUM",
                Role::Bridge      => "BRIDGE",
                Role::Bus         => "BUS",
                Role::Kernel      => "KERNEL",
            }
        }

        /// Mund të thirret direkt nga user?
        pub fn user_facing(&self) -> bool {
            matches!(self, Role::Coordinator | Role::Entity)
        }

        /// Shkruan në memory?
        pub fn can_write_memory(&self) -> bool {
            false // VETËM Shadow shkruan — LAW_3
        }
    }

    pub fn of(module: &str) -> Role {
        match module {
            "light_coordinator"  => Role::Coordinator,
            "trace_info"         => Role::Tracer,
            "evolve_trace"       => Role::Tracer,
            "software_contract"  => Role::Enforcer,
            "nura_core"          => Role::Entity,
            "ess_mai_system"     => Role::Declarator,
            "manifest"           => Role::Declarator,
            "ess_manifest"       => Role::Declarator,
            "lgc_algorithm"      => Role::Algorithm,
            "legalgocrypt"       => Role::Algorithm,
            "algorithm_continuum" => Role::Continuum,
            "progressive_debatic" => Role::Continuum,
            "lgc_bridge"         => Role::Bridge,
            "light_buss"         => Role::Bus,
            "shadow_kernel"      => Role::Kernel,
            _                    => Role::Declarator,
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// §4  VEPRIMET — çfarë lejohet të bëjë çdo modul
// ════════════════════════════════════════════════════════════════════════════

pub mod actions {
    pub const LIGHT_COORDINATOR: &[&str] = &[
        "receive_input", "route_to_quantum", "route_to_shadow",
        "create_contract", "stamp_trace", "evolve_trace",
        "format_output", "return_to_user",
    ];
    pub const TRACE_INFO: &[&str] = &[
        "new", "with_domain", "id_hex", "label", "short_id", "is_valid",
    ];
    pub const EVOLVE_TRACE: &[&str] = &[
        "from_trace", "branch", "summary", "branch_count",
    ];
    pub const NURA_CORE: &[&str] = &[
        "route", "emotion", "send_signal", "signals_for", "cleanup_signals",
        "register_matter", "update_matter_online", "get_matter",
        "register_device", "validate_device", "revoke_device",
        "ephemeral_fetch_signal", "status",
    ];
    pub const SOFTWARE_CONTRACT: &[&str] = &[
        "create", "enforce",
    ];
    pub const LGC_ALGORITHM: &[&str] = &[
        "generate_kodunik", "verify_kodunik", "compute_sha256",
    ];
    pub const LEGALGOCRYPT: &[&str] = &[
        "generate", "encode", "parse", "validate", "unseal_from_flags",
    ];
    pub const ALGORITHM_CONTINUUM: &[&str] = &[
        "declare_mini", "declare_algorithm", "activate_pd",
        "complete_iz", "derive_next_i0",
    ];

    /// A lejohet veprimi për modulin e dhënë?
    pub fn is_permitted(module: &str, action: &str) -> bool {
        let list = match module {
            "light_coordinator"  => LIGHT_COORDINATOR,
            "trace_info"         => TRACE_INFO,
            "evolve_trace"       => EVOLVE_TRACE,
            "nura_core"          => NURA_CORE,
            "software_contract"  => SOFTWARE_CONTRACT,
            "lgc_algorithm"      => LGC_ALGORITHM,
            "legalgocrypt"       => LEGALGOCRYPT,
            "algorithm_continuum" => ALGORITHM_CONTINUUM,
            "progressive_debatic" => ALGORITHM_CONTINUUM,
            _                    => &[],
        };
        list.contains(&action)
    }
}

// ════════════════════════════════════════════════════════════════════════════
// §5  LIMITET — kufijtë e sistemit
// ════════════════════════════════════════════════════════════════════════════

pub mod limits {
    /// Epistemic mass minimum — nën këtë: pipeline ndalet (LAW_5)
    pub const MIN_EPISTEMIC_MASS:     f32 = 0.20;

    /// NK penalty maksimale — [0,1]
    pub const MAX_NK_PENALTY:         f32 = 1.0;

    /// Maksimumi i degëve të trace brenda një EvolveTrace
    pub const MAX_TRACE_BRANCHES:     usize = 16;

    /// Maksimumi i sinjaleve aktive (AMMY 72h signals)
    pub const MAX_SIGNALS_ACTIVE:     usize = 1024;

    /// Jeta e një sinjali — 72 orë
    pub const SIGNAL_TTL_SECS:        u64   = 72 * 3600;

    /// Jeta maksimale e një kontrate — 24 orë
    pub const CONTRACT_MAX_AGE_SECS:  u64   = 86400;

    /// Maksimumi i pajisve MATTER
    pub const MAX_MATTER_DEVICES:     usize = 256;

    /// Maksimumi i seancave device aktive (AMMY)
    pub const MAX_DEVICE_SESSIONS:    usize = 128;

    /// Madhësia e ring buffer — fuqi e 2
    pub const RING_BUFFER_SIZE:       usize = 64;

    /// Maksimumi i kandidatëve PRO
    pub const MAX_PRO_CANDIDATES:     usize = 100;

    /// Threshold i verifikimit optimal
    pub const VERIFIED_OPTIMAL_THRESHOLD: f32 = 0.85;
}

// ════════════════════════════════════════════════════════════════════════════
// §6  KOMUNIKIMI — rrugët e thirrjeve ndër-platformike
// ════════════════════════════════════════════════════════════════════════════

pub mod communication {
    #[derive(Debug, Clone, PartialEq)]
    pub enum Priority { Critical, High, Normal, Low }

    impl Priority {
        pub fn label(&self) -> &str {
            match self {
                Priority::Critical => "CRITICAL",
                Priority::High     => "HIGH",
                Priority::Normal   => "NORMAL",
                Priority::Low      => "LOW",
            }
        }
        pub fn value(&self) -> u8 {
            match self {
                Priority::Critical => 4,
                Priority::High     => 3,
                Priority::Normal   => 2,
                Priority::Low      => 1,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct Route {
        pub from:     &'static str,
        pub to:       &'static str,
        pub channel:  &'static str, // "direct" | "light_buss" | "ring_buffer"
        pub priority: Priority,
    }

    pub fn get_routes() -> Vec<Route> {
        vec![
            Route { from:"light_coordinator", to:"quantum_platform", channel:"light_buss",   priority:Priority::High     },
            Route { from:"light_coordinator", to:"shadow_judiciary",  channel:"light_buss",   priority:Priority::Critical },
            Route { from:"quantum_platform",  to:"shadow_judiciary",  channel:"ring_buffer",  priority:Priority::Critical },
            Route { from:"shadow_judiciary",  to:"light_coordinator", channel:"light_buss",   priority:Priority::Critical },
            Route { from:"trace_info",        to:"evolve_trace",      channel:"direct",        priority:Priority::Normal   },
            Route { from:"evolve_trace",      to:"lgc_algorithm",     channel:"direct",        priority:Priority::Normal   },
            Route { from:"nura_core",         to:"light_buss",        channel:"direct",        priority:Priority::Normal   },
            Route { from:"software_contract", to:"lgc_algorithm",     channel:"direct",        priority:Priority::High     },
            Route { from:"algorithm_continuum", to:"progressive_debatic", channel:"direct", priority:Priority::Critical },
            Route { from:"progressive_debatic", to:"shadow_judiciary", channel:"ring_buffer", priority:Priority::Critical },
        ]
    }

    /// Autorizimi ndër-platform: kush mund të thërrasë kë
    pub fn is_authorized(caller: &str, target: &str) -> bool {
        let authorized: &[(&str, &str)] = &[
            // Light internal
            ("light_coordinator",  "trace_info"),
            ("light_coordinator",  "evolve_trace"),
            ("light_coordinator",  "nura_core"),
            ("light_coordinator",  "software_contract"),
            ("light_coordinator",  "quantum_platform"),
            ("light_coordinator",  "shadow_judiciary"),
            ("trace_info",         "lgc_algorithm"),
            ("evolve_trace",       "lgc_algorithm"),
            ("evolve_trace",       "legalgocrypt"),
            ("software_contract",  "lgc_algorithm"),
            ("software_contract",  "legalgocrypt"),
            ("algorithm_continuum", "progressive_debatic"),
            ("nura_core",          "light_buss"),
            // Shadow → gjithkush (autoritet suprem)
            ("shadow_judiciary",   "light_coordinator"),
            ("shadow_judiciary",   "nk_system"),
            ("shadow_judiciary",   "vok_store"),
            ("shadow_judiciary",   "vnk_store"),
            // Quantum → Shadow (propozon vetëm tek Shadow)
            ("quantum_platform",   "shadow_judiciary"),
            ("pim_engine",         "shadow_judiciary"),
            ("lim_engine",         "pro_engine"),
            ("pro_engine",         "srk_engine"),
            ("srk_engine",         "pim_engine"),
        ];
        authorized.contains(&(caller, target))
    }
}

// ════════════════════════════════════════════════════════════════════════════
// §7  DSL — Gjuha Algoritmike (nga nura_lgc_dsl)
//     Sufixet e knowledge, gjendjet e algoritmit, operacionet matematike
// ════════════════════════════════════════════════════════════════════════════

pub mod dsl {
    // ── SUFFIX CODES — kodi i degës në EvolveTrace ────────────────────────
    // Shembull: x1234_hiskng_1 = branch HISTORY nga trace x1234
    pub const SFX_INPUT:         &str = "INPKNG"; // input fillestar
    pub const SFX_HISTORY:       &str = "HISKNG"; // histori
    pub const SFX_GEOGRAPHY:     &str = "GEOKNG"; // gjeografi
    pub const SFX_MATHEMATICS:   &str = "MATHKNG"; // matematik
    pub const SFX_SCIENCE:       &str = "SCIKNG"; // shkencë
    pub const SFX_INDUSTRIAL:    &str = "INDKNG"; // industrial
    pub const SFX_SEMANTIC:      &str = "SEMKNG"; // semantikë
    pub const SFX_PHILOSOPHICAL: &str = "PHIKNG"; // filozofi
    pub const SFX_TECHNICAL:     &str = "TECKNG"; // teknik
    pub const SFX_FORMULATION:   &str = "FRMKNG"; // formulim
    pub const SFX_NEGATIVE:      &str = "NEGKNG"; // NK path
    pub const SFX_VERIFIED:      &str = "VFYKNG"; // i verifikuar
    pub const SFX_SHADOW:        &str = "SHKNG";  // nga Shadow

    /// Të gjithë sufixet si array
    pub const ALL_SUFFIXES: &[&str] = &[
        SFX_INPUT, SFX_HISTORY, SFX_GEOGRAPHY, SFX_MATHEMATICS,
        SFX_SCIENCE, SFX_INDUSTRIAL, SFX_SEMANTIC, SFX_PHILOSOPHICAL,
        SFX_TECHNICAL, SFX_FORMULATION, SFX_NEGATIVE, SFX_VERIFIED, SFX_SHADOW,
    ];

    /// A është suffiks valid?
    pub fn is_valid_suffix(s: &str) -> bool {
        ALL_SUFFIXES.contains(&s)
    }

    // ── GJENDJET E ALGORITMIT — state machine e pipeline ─────────────────
    #[derive(Debug, Clone, PartialEq)]
    pub enum AlgorithmState {
        Idle,           // pritje
        Tracing,        // trace_info ka vënë stampën
        Evolving,       // evolve_trace po proceson
        Routing,        // light_coordinator po rout-on
        QuantumPending, // dërguar tek Quantum, po pret propozimin
        ShadowVerify,   // dërguar tek Shadow, po pret 0/1
        Sealed,         // KODUNIK seal aplikuar — OUTPUT
        Rejected,       // Shadow ka kthyer 0 — NK register
        Failed(String), // gabim i papritur
    }

    impl AlgorithmState {
        pub fn label(&self) -> &str {
            match self {
                Self::Idle           => "IDLE",
                Self::Tracing        => "TRACING",
                Self::Evolving       => "EVOLVING",
                Self::Routing        => "ROUTING",
                Self::QuantumPending => "QUANTUM_PENDING",
                Self::ShadowVerify   => "SHADOW_VERIFY",
                Self::Sealed         => "SEALED",
                Self::Rejected       => "REJECTED",
                Self::Failed(_)      => "FAILED",
            }
        }

        /// Tranzicione të vlefshme — state machine e saktë
        pub fn can_transition_to(&self, next: &AlgorithmState) -> bool {
            match (self, next) {
                (Self::Idle,           Self::Tracing)        => true,
                (Self::Tracing,        Self::Evolving)       => true,
                (Self::Evolving,       Self::Routing)        => true,
                (Self::Routing,        Self::QuantumPending) => true,
                (Self::QuantumPending, Self::ShadowVerify)   => true,
                (Self::ShadowVerify,   Self::Sealed)         => true,
                (Self::ShadowVerify,   Self::Rejected)       => true,
                // Çdo gjendje mund të shkojë tek Failed
                (_, Self::Failed(_))                         => true,
                _                                            => false,
            }
        }
    }

    // ── OPERACIONET MATEMATIKE — O(1) per path ────────────────────────────
    // Të njëjtat formula si Quantum Platform     

    /// I1: masa epistemike ∈ [0,1]
    #[inline(always)]
    pub fn epistemic_mass(axes: &[f32; 8]) -> f32 {
        const W: [f32; 8] = [0.18, 0.14, 0.10, 0.14, 0.16, 0.10, 0.10, 0.08];
        axes.iter().zip(W.iter()).map(|(a, w)| a * w).sum()
    }

    /// I2: gate ∈ [0,1] — zero branches
    #[inline(always)]
    pub fn gate(x: f32, lo: f32, hi: f32) -> f32 {
        ((x - lo) / (hi - lo + 1e-9_f32)).clamp(0.0, 1.0)
    }

    /// I3: eliminim ∈ [0,w]
    #[inline(always)]
    pub fn elim(w: f32, cg: f32, nk: f32) -> f32 {
        w * cg * (1.0 - nk)
    }

    /// I4: admit ∈ {0,1} — AND si produkt
    #[inline(always)]
    pub fn admit(conditions: &[bool]) -> u8 {
        conditions.iter().fold(1u8, |acc, &c| acc * (c as u8))
    }

    /// I6: trust ∈ [0,1]
    #[inline(always)]
    pub fn trust_score(approved: u64, total: u64, negative: u64) -> f32 {
        match total == 0 {
            true => return 0.5,
            false => {}
        }
        let base     = approved as f32 / total as f32;
        let neg_rate = negative as f32 / total as f32;
        base.powf(1.0 - neg_rate + 1e-9_f32)
    }

    /// pressure = clamp((1-mass)×2 + neg_rate×1.5, 0, 5.9) — 6 nivele
    #[inline(always)]
    pub fn pressure(mass: f32, neg_rate: f32) -> f32 {
        ((1.0 - mass) * 2.0 + neg_rate * 1.5).clamp(0.0, 5.9)
    }
}
