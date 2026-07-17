// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAYER3/HPIM.RS — Hardware Pass Info Metric (Quantum)                ║
// ║  GJATA LEGACY™ — ESS-MAI Quantum                          (LAYER 3)    ║
// ║                                                                          ║
// ║  HPIM është SHKRIRJA E TRACE-it të HPRO-së:                            ║
// ║   • çfarë bëri HPRO (operatorët e zbatuar)                            ║
// ║   • pse i zgjodhi ato alternativa (arsyetimi i kufijve)               ║
// ║   • si u paketuan                                                      ║
// ║   • çfarë i dorëzohet PIM-it                                          ║
// ║                                                                          ║
// ║  KUFIRI KRITIK: presioni NUK futet te PIM. Presioni shkon te HPIM,      ║
// ║  pastaj PIM konsumon DOSJEN. HPIM mban Quantum-in të pastër.           ║
// ║  HPIM vetëm paketon — s'vendos, s'mat TRL. Zero if/else.              ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::layer3::hpro::{HproResult, WebProbeKind};

// ─────────────────────────────────────────────────────────────────────────────
// HPRO TRACE — gjurma e asaj që bëri HPRO
// ─────────────────────────────────────────────────────────────────────────────

/// Një hap në gjurmën e HPRO — cili operator, çfarë vendosi, pse.
#[derive(Debug, Clone, PartialEq)]
pub struct HproTraceStep {
    /// Operatori i zbatuar (prod_formula, control_role, wcfd, etj.).
    pub operator:   String,
    /// Vendimi/rezultati i tij.
    pub decision:   String,
    /// Arsyetimi (pse kjo alternativë).
    pub rationale:  String,
}

// ─────────────────────────────────────────────────────────────────────────────
// HPIM PACKAGE — dosja që i dorëzohet PIM-it
// ─────────────────────────────────────────────────────────────────────────────

/// Dosja e plotë e HPIM — gjurma e HPRO + përmbledhja për PIM.
#[derive(Debug, Clone, PartialEq)]
pub struct HpimPackage {
    /// Identifikuesi i pretendimit.
    pub claim_id:        u64,
    /// Gjurma e plotë e operatorëve HPRO.
    pub trace:           Vec<HproTraceStep>,
    /// Sa shtytje presioni u lejua (nga kufijtë).
    pub pressure_push:   f32,
    /// Sa paralel u vendos.
    pub parallel_used:   u8,
    /// A u lejua thellim i thellë.
    pub deep_allowed:    bool,
    /// A është lease-i aktiv për workload-in aktual.
    pub lease_active:      bool,
    /// A u lëshua lease-i pas workload-it.
    pub hardware_released: bool,
}

impl HpimPackage {
    /// trace_len — sa hapa ka gjurma. Zero if.
    pub fn trace_len(&self) -> usize {
        self.trace.len()
    }

    /// is_ready_for_pim — dosja është gati kur ka trace dhe lease-i është
    /// aktiv për konsumim OSE është lëshuar pas një workload-i compatibility.
    pub fn is_ready_for_pim(&self) -> bool {
        let has_trace = !self.trace.is_empty();
        let lease_valid = self.lease_active | self.hardware_released;
        has_trace & lease_valid
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// HPIM ENGINE — paketon trace-in e HPRO
// ─────────────────────────────────────────────────────────────────────────────

pub struct HpimEngine;

impl HpimEngine {
    /// package — shkrin rezultatin e HPRO në një dosje HPIM për PIM.
    ///
    /// Ndërton gjurmën nga operatorët e zbatuar me arsyetimin e tyre.
    /// Zero if — formula + match.
    pub fn package(claim_id: u64, hpro: &HproResult) -> HpimPackage {
        let mut trace: Vec<HproTraceStep> = Vec::new();

        // Hap 1: prod_formula.
        trace.push(HproTraceStep {
            operator:  "prod_formula".to_string(),
            decision:  Self::bounds_decision(hpro),
            rationale: "kufijtë fizikë nga energ + termik + RAM".to_string(),
        });

        // Hap 2: control_role.
        trace.push(HproTraceStep {
            operator:  "control_role".to_string(),
            decision:  Self::stability_decision(hpro),
            rationale: "stabilitet device — parametra lartë/poshtë".to_string(),
        });

        HpimPackage {
            claim_id,
            trace,
            pressure_push:     hpro.bounds.pressure_push,
            parallel_used:     hpro.bounds.effective_parallel(),
            deep_allowed:      hpro.bounds.deep_allowed,
            lease_active:       hpro.lease.active,
            hardware_released: hpro.hardware_released,
        }
    }

    /// bounds_decision — përshkrim i vendimit të prod_formula. Zero if — match.
    fn bounds_decision(hpro: &HproResult) -> String {
        match hpro.bounds.allows_deepening() {
            true  => format!("thellim i lejuar: {} bytes, push {:.2}",
                             hpro.bounds.ram_budget_bytes, hpro.bounds.pressure_push),
            false => "thellim i bllokuar: device s'lejon".to_string(),
        }
    }

    /// stability_decision — përshkrim i planit të stabilitetit. Zero if — match.
    fn stability_decision(hpro: &HproResult) -> String {
        match hpro.stability.safe_to_proceed {
            true  => format!("vazhdohet: thermal_guard={}", hpro.stability.thermal_guard.label()),
            false => "ndalohet: termik kritik".to_string(),
        }
    }

    /// add_web_step — shton një hap web në gjurmë (kur wcfd/nwcfd/hwcfd u zbatua).
    /// Zero if — match mbi llojin.
    pub fn add_web_step(pkg: &mut HpimPackage, kind: WebProbeKind, results: u32) {
        let operator = match kind {
            WebProbeKind::Optimal    => "wcfd",
            WebProbeKind::Minimal    => "nwcfd",
            WebProbeKind::Hypothesis => "hwcfd",
        };
        pkg.trace.push(HproTraceStep {
            operator:  operator.to_string(),
            decision:  format!("{} rezultate nga device", results),
            rationale: "kërkim web device-mediated me kufij nga prod_formula".to_string(),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layer3::hpro::HproEngine;
    use crate::lab_contracts_v11::hardware::DeviceSnapshot;

    fn strong_device() -> DeviceSnapshot {
        DeviceSnapshot {
            ram_free_bytes: 4_000_000_000, ram_total_bytes: 8_000_000_000,
            cpu_load: 0.3, cores_active: 4, cores_total: 8,
            energy_margin: 0.9, on_battery: false,
            temp_c: 40.0, temp_limit_c: 90.0, timestamp: 100,
        }
    }

    #[test]
    fn packages_hpro_trace() {
        let hpro = HproEngine::run(&strong_device());
        let pkg = HpimEngine::package(1, &hpro);
        assert_eq!(pkg.claim_id, 1);
        // Gjurma ka të paktën prod_formula + control_role.
        assert!(pkg.trace_len() >= 2);
        assert!(pkg.is_ready_for_pim());
    }

    #[test]
    fn trace_contains_operators() {
        let hpro = HproEngine::run(&strong_device());
        let pkg = HpimEngine::package(1, &hpro);
        let ops: Vec<&str> = pkg.trace.iter().map(|s| s.operator.as_str()).collect();
        assert!(ops.contains(&"prod_formula"));
        assert!(ops.contains(&"control_role"));
    }

    #[test]
    fn carries_pressure_not_to_pim_directly() {
        // HPIM mban presionin (push); PIM do konsumojë dosjen, jo presionin drejtpërdrejt.
        let hpro = HproEngine::run(&strong_device());
        let pkg = HpimEngine::package(1, &hpro);
        assert!(pkg.pressure_push > 0.0);
        assert!(pkg.parallel_used >= 1);
    }

    #[test]
    fn web_step_added() {
        let hpro = HproEngine::run(&strong_device());
        let mut pkg = HpimEngine::package(1, &hpro);
        let before = pkg.trace_len();
        HpimEngine::add_web_step(&mut pkg, WebProbeKind::Optimal, 5);
        assert_eq!(pkg.trace_len(), before + 1);
        // Hapi i fundit është wcfd.
        let last = pkg.trace.last().unwrap();
        assert_eq!(last.operator, "wcfd");
    }

    #[test]
    fn ready_requires_trace_and_release() {
        let hpro = HproEngine::run(&strong_device());
        let pkg = HpimEngine::package(1, &hpro);
        // Strong device → gjurmë + harduri u lëshua → gati.
        assert!(pkg.is_ready_for_pim());
        assert!(pkg.hardware_released);
    }
}
