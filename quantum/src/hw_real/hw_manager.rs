// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  HW_MANAGER.RS — MENAXHERI SOVRAN I HARDWARE-it (R E A L CONTROL)        ║
// ║  Gjata Legacy™  |  ESS-MAI Quantum                                        ║
// ║                                                                          ║
// ║  LIGJI I HARDWARE-it (i Arkitektit):                                     ║
// ║    1. ESS-MAI KONTROLLON hardware-in — jo e zë, e QEVERIS.               ║
// ║    2. GJITHMONË ≥10% e hardware-it mbetet E LIRË (dysheme e pashkelshme).║
// ║    3. Pjesa e përdorshme përdoret ME FLEKSIBILITET: kërkesa mbi kufi     ║
// ║       SHKURTOHET, kurrë s'refuzohet — sistemi KURRË s'bllokohet.         ║
// ║    4. Kontrolli është dinamik: lease-t ripërshtaten me gjendjen reale.   ║
// ║                                                                          ║
// ║  Burimi i të vërtetës: snapshot REAL i kernelit (hw_kernel) —            ║
// ║  ram_total/ram_free/cores nga GlobalMemoryStatusEx / sysinfo, JO hamendje║
// ║  LIGJE KODI: zero if/else (match), zero panic, saturating arithmetic.    ║
// ╚══════════════════════════════════════════════════════════════════════════╝

/// Dyshemeja e pashkelshme: 10% e hardware-it GJITHMONË i lirë.
pub const HW_FREE_FLOOR_PCT: f32 = 0.10;

/// Lease-i i qeverisur — çfarë i JEPET realisht konsumatorit.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HwLease {
    /// Identitet determinist i lease-it për trace runtime.
    pub lease_id: u64,
    /// Bytes të dhëna realisht (≤ kërkesa; kurrë s'shkel dyshemenë).
    pub granted_bytes: u64,
    /// Bërthama paralele të dhëna (gjithmonë ≥1; lë hapësirë sistemit).
    pub granted_parallel: u32,
    /// Dyshemeja absolute e llogaritur (bytes që MBETEN të lira).
    pub floor_bytes: u64,
    /// Bërthamat e rezervuara për device-in.
    pub reserved_cores: u32,
    /// true = kërkesa u SHKURTUA me fleksibilitet (jo refuzim, përshtatje).
    pub flexed: bool,
    /// Lease-i është aktiv dhe mund të pranojë punë.
    pub active: bool,
}

impl HwLease {
    pub fn report(&self) -> String {
        format!("HwLease: id=0x{:016X} granted={}MB parallel={} floor={}MB reserved_cores={} flexed={} active={}",
            self.lease_id,
            self.granted_bytes / (1024 * 1024),
            self.granted_parallel,
            self.floor_bytes / (1024 * 1024),
            self.reserved_cores,
            self.flexed,
            self.active)
    }

    /// Lëshon lease-in logjik të workload-it ESS-MAI. Kthen true vetëm një herë.
    pub fn release(&mut self) -> bool {
        match self.active {
            true => {
                self.active = false;
                true
            }
            false => false,
        }
    }


    /// Kufizon prefix-in e fragmenteve sipas buxhetit të lease-it.
    /// Kostoja është një vlerësim konservativ i working set-it Rust
    /// (String header + tekst + hapësirë për vektorizim/stem-e), jo rezervim OS.
    pub fn admit_fragment_prefix(&self, fragments: &[String]) -> (usize, u64) {
        match (self.active, self.granted_bytes > 0) {
            (true, true) => {
                let budget = self.granted_bytes;
                let mut admitted = 0usize;
                let mut used = 0u64;
                for fragment in fragments {
                    let text_bytes = u64::try_from(fragment.len()).unwrap_or(u64::MAX);
                    let header_bytes = u64::try_from(std::mem::size_of::<String>())
                        .unwrap_or(u64::MAX);
                    let cost = header_bytes.saturating_add(text_bytes.saturating_mul(4));
                    let next = used.saturating_add(cost);
                    match next <= budget {
                        true => {
                            used = next;
                            admitted = admitted.saturating_add(1);
                        }
                        false => break,
                    }
                }
                (admitted, used)
            }
            _ => (0, 0),
        }
    }
}

/// Gjendja reale e hardware-it që duhet përdorur gjatë ripërshtatjes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HwEnvironment {
    pub ram_total_bytes: u64,
    pub ram_free_bytes: u64,
    pub cores_active: u32,
}

pub struct HwManager;

impl HwManager {
    /// QEVERISJA — thelbi i R E A L control:
    ///   floor  = 10% × ram_total (kur total dihet) — dysheme absolute;
    ///            fallback konservativ: 10% × ram_free kur total mungon (0).
    ///   usable = ram_free − floor (saturating — kurrë negative).
    ///   grant  = min(requested, usable) — SHKURTIM fleksibël, JO refuzim.
    ///   cores  = min(req_parallel, cores_active − ceil(10% cores)), min 1.
    pub fn govern(
        ram_total_bytes: u64,
        ram_free_bytes:  u64,
        requested_bytes: u64,
        cores_active:    u32,
        req_parallel:    u32,
    ) -> HwLease {
        // Dyshemeja: mbi TOTAL kur dihet; ndryshe mbi FREE (konservativ).
        let floor_base = match ram_total_bytes {
            0 => ram_free_bytes,
            t => t,
        };
        // v1.5.1 (Korrigjimi A): floor AUTORITATIV me matematikë INTEGER u64 —
        // 10% = floor_base / 10, ekzakt në byte (8GB→858993459, 10GB→1073741824).
        // f32(0.10) mbart epsilon (+13B në 8GB, +16B në 10GB) — i ndaluar për
        // vendimin në byte. HW_FREE_FLOOR_PCT mbetet publik (re-export + cores).
        let floor_bytes = floor_base / 10;

        let usable  = ram_free_bytes.saturating_sub(floor_bytes);
        let granted = requested_bytes.min(usable);
        let flexed_ram = granted < requested_bytes;

        // Bërthamat: lë të lirë ceil(10% e cores), gjithmonë ≥1 e dhënë.
        // GJETJA v1.4.2 (menaxhim real i harduerit, direktiva e Arkitektit):
        // "10% gjithmonë i lirë" duhet të MBROJ NGA MBINGARKIMI/MBINXEHJA edhe
        // në makina me pak bërthama. ceil(10%) e siguron floor-in mbi cores;
        // por në 1-2 bërthama, ceil mund ta zonte të gjithë hapësirën paralele.
        // Ligji i ri: DYSHEMEJA e rezervës është ≥1 bërthamë kur cores≥2 —
        // kështu makina KURRË s'shkon në 100% bërthama nga ky menaxher.
        let cores = cores_active.max(1);
        let reserved_raw   = ((cores as f32) * HW_FREE_FLOOR_PCT).ceil() as u32;
        let reserved_cores = match cores {
            1 => 0,                     // 1 bërthamë: s'ka çfarë rezervohet
            _ => reserved_raw.max(1),   // ≥2: gjithnjë ≥1 e lirë (anti-mbinxehje)
        };
        let max_parallel   = cores.saturating_sub(reserved_cores).max(1);
        let granted_parallel = req_parallel.clamp(1, max_parallel);
        let flexed_cpu = granted_parallel < req_parallel.max(1);

        // Identitet determinist nga mjedisi + GRANTA reale. Kërkesa e papranuar
        // nuk ndryshon identitetin e lease-it. Nuk është vulë sigurie;
        // shërben vetëm për vazhdimësinë e trace-it runtime.
        let lease_id = ram_total_bytes
            .rotate_left(7)
            ^ ram_free_bytes.rotate_left(19)
            ^ granted.rotate_left(31)
            ^ u64::from(cores_active).rotate_left(11)
            ^ u64::from(granted_parallel).rotate_left(43);

        HwLease {
            lease_id,
            granted_bytes: granted,
            granted_parallel,
            floor_bytes,
            reserved_cores,
            flexed: flexed_ram || flexed_cpu,
            active: true,
        }
    }

    /// RIPËRSHTATJE DINAMIKE: gjendja e re reale → lease i rifreskuar.
    /// Nëse liria ra nën dysheme+granted, lease-i SHKURTOHET vetiu —
    /// hardware-i mbetet i lirë, sistemi vazhdon (fleksibilitet, jo bllokim).
    pub fn adjust(
        lease: HwLease,
        environment: HwEnvironment,
        requested_bytes: u64,
        requested_parallel: u32,
    ) -> HwLease {
        match lease.active {
            false => lease, // një lease i lëshuar nuk ringjallet nga controller-i
            true => Self::govern(
                environment.ram_total_bytes,
                environment.ram_free_bytes,
                requested_bytes,
                environment.cores_active,
                requested_parallel.max(1),
            ),
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// TESTE — ligji i 10%-shit i provuar me numra realë
// ════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    const GB: u64 = 1024 * 1024 * 1024;

    #[test]
    fn floor_ten_percent_of_total_always_free() {
        // 8GB total, 6GB free, kërkohen 6GB → floor 0.8GB → jepen max 5.2GB.
        let l = HwManager::govern(8 * GB, 6 * GB, 6 * GB, 8, 8);
        assert_eq!(l.floor_bytes, (8 * GB) / 10);
        assert_eq!(l.granted_bytes, 6 * GB - (8 * GB) / 10);
        assert!(l.flexed); // u shkurtua — fleksibilitet, jo refuzim
    }

    #[test]
    fn never_blocks_even_when_free_below_floor() {
        // Liria (0.5GB) NËN dyshemenë (0.8GB) → usable=0 → grant=0, POR
        // lease kthehet (parallel≥1) — sistemi s'bllokohet, punon minimal.
        let l = HwManager::govern(8 * GB, GB / 2, 2 * GB, 4, 4);
        assert_eq!(l.granted_bytes, 0);
        assert!(l.granted_parallel >= 1);
    }

    #[test]
    fn request_within_budget_granted_fully() {
        let l = HwManager::govern(16 * GB, 12 * GB, 4 * GB, 8, 2);
        assert_eq!(l.granted_bytes, 4 * GB);
        assert_eq!(l.granted_parallel, 2);
        assert!(!l.flexed);
    }

    #[test]
    fn cpu_reserve_leaves_ten_percent_cores() {
        // 8 cores → rezervohen ceil(0.8)=1 → max 7 paralele.
        let l = HwManager::govern(8 * GB, 8 * GB, GB, 8, 16);
        assert_eq!(l.granted_parallel, 7);
        assert!(l.flexed);
        // 1 core → gjithmonë ≥1 e dhënë (kurrë 0 — s'bllokohet).
        let l1 = HwManager::govern(GB, GB, 0, 1, 1);
        assert_eq!(l1.granted_parallel, 1);
        // ANTI-MBINXEHJE v1.4.2: makinat e vogla KURRË s'shkojnë 100% cores.
        // 2 cores, kërkohen 8 paralele → rezervohet ≥1 → jepen max 1 (jo 2).
        let l2 = HwManager::govern(4 * GB, 4 * GB, GB, 2, 8);
        assert_eq!(l2.granted_parallel, 1, "2 bërthama: ≥1 gjithnjë e lirë");
        // 4 cores → ceil(0.4)=1 rezervë → max 3 paralele.
        let l4 = HwManager::govern(8 * GB, 8 * GB, GB, 4, 16);
        assert_eq!(l4.granted_parallel, 3, "4 bërthama: 1 e lirë, 3 punë");
    }

    #[test]
    fn unknown_total_falls_back_conservative_on_free() {
        // total=0 (i panjohur) → floor = 10% e FREE.
        let l = HwManager::govern(0, 10 * GB, 10 * GB, 4, 4);
        assert_eq!(l.floor_bytes, GB);
        assert_eq!(l.granted_bytes, 9 * GB);
    }

    #[test]
    fn adjust_shrinks_when_reality_tightens() {
        let l0 = HwManager::govern(8 * GB, 6 * GB, 4 * GB, 8, 4);
        // Liria bie në 2GB → lease ripërshtatet nën dysheme+.
        let env = HwEnvironment {
            ram_total_bytes: 8 * GB,
            ram_free_bytes: 2 * GB,
            cores_active: 8,
        };
        let l1 = HwManager::adjust(l0, env, l0.granted_bytes, l0.granted_parallel);
        assert!(l1.granted_bytes <= 2 * GB - (8 * GB) / 10);
        assert!(l1.granted_parallel >= 1);
    }
    #[test]
    fn lease_budget_limits_real_fragment_prefix() {
        let fragments = vec![
            "alpha beta gamma delta".to_string(),
            "epsilon zeta eta theta".to_string(),
            "iota kappa lambda mu".to_string(),
        ];
        let full = HwLease {
            lease_id: 1,
            granted_bytes: u64::MAX,
            granted_parallel: 2,
            floor_bytes: 0,
            reserved_cores: 1,
            flexed: false,
            active: true,
        };
        let (all_count, first_cost) = full.admit_fragment_prefix(&fragments[..1]);
        assert_eq!(all_count, 1);
        let limited = HwLease { granted_bytes: first_cost, ..full };
        let (count, used) = limited.admit_fragment_prefix(&fragments);
        assert_eq!(count, 1);
        assert_eq!(used, first_cost);
    }

    #[test]
    fn released_or_zero_lease_admits_no_working_set() {
        let fragments = vec!["knowledge fragment".to_string()];
        let inactive = HwLease {
            lease_id: 1,
            granted_bytes: 1024,
            granted_parallel: 1,
            floor_bytes: 0,
            reserved_cores: 0,
            flexed: false,
            active: false,
        };
        assert_eq!(inactive.admit_fragment_prefix(&fragments), (0, 0));
        let zero = HwLease { active: true, granted_bytes: 0, ..inactive };
        assert_eq!(zero.admit_fragment_prefix(&fragments), (0, 0));
    }

}
