// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LIM_HW_BRIDGE.RS — Lidhja LIM ↔ Hardware                            ║
// ║  GJATA LEGACY™ — Quantum Platform                                     ║
// ║                                                                          ║
// ║  FORMULA QENDRORE (sipas arkitektit):                               ║
// ║    depth_final = depth_epistemic × depth_hardware                     ║
// ║                                                                          ║
// ║  depth_epistemic = sa thellë MERITON inputi (LIM nga teksti)          ║
// ║  depth_hardware  = sa thellë MUND fizikisht (RAM budget)              ║
// ║                                                                          ║
// ║  RRJEDHA:                                                              ║
// ║    LIM mat mass → DepthHint::from_mass → ScaleHarduer alokon RAM →    ║
// ║    depth_hardware = budget.depth_factor() →                           ║
// ║    reasoning_depth_allowance ×= depth_hardware →                      ║
// ║    PRO merr depth_config me kufijtë REALË fizikë.                     ║
// ║                                                                          ║
// ║  FILOZOFIA: çdo input meriton thellësi, por masa + hardware vendosin   ║
// ║  sa. E thjeshta (Shallow) prapë merr budget minimal — primitivit       ║
// ║  s'i dihet që mund të dalë diçka.                                     ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::lim_types::EpistemicScaffold;
use crate::lim_classifier::{compute_operator_gates, compute_depth_config};
use crate::hardware::{DepthHint, ScaleHarduer, ActParallel, ResourceSnapshot, HwBudget, ParallelDecision};

// ─────────────────────────────────────────────────────────────────────────────
// SCAFFOLD I FINALIZUAR — me hardware të lidhur
// ─────────────────────────────────────────────────────────────────────────────

/// EpistemicScaffold + vendimet e hardware-it.
/// Ky është output-i që PRO merr realisht — me kufijtë fizikë.
#[derive(Debug, Clone)]
pub struct HardwareScaffold {
    /// Scaffold-i epistemik (me reasoning_depth_allowance të finalizuar).
    pub scaffold: EpistemicScaffold,

    /// DepthHint nga masa.
    pub depth_hint: DepthHint,

    /// Budget-i RAM i alokuar.
    pub budget: HwBudget,

    /// Sa operatorë PRO mund të ekzekutohen paralel (nga RAM).
    pub parallel: ParallelDecision,

    /// depth_epistemic origjinal (para shumëzimit) — për audit.
    pub depth_epistemic: f32,
    /// depth_hardware (faktori fizik) — për audit.
    pub depth_hardware: f32,
    /// depth_final = epistemic × hardware.
    pub depth_final: f32,
}

impl HardwareScaffold {
    /// A lejon hardware reasoning (energy jo kritik)?
    /// Bateria < 5% → sistemi refuzon reasoning të thellë (teoria 46).
    pub fn hardware_permits(&self) -> bool {
        // Lejohet nëse u alokua budget real.
        self.budget.allocated_bytes > 0
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// URA — finalizon scaffold-in me hardware
// ─────────────────────────────────────────────────────────────────────────────

pub struct LimHwBridge;

impl LimHwBridge {
    /// Lidh LIM me hardware. Merr scaffold-in epistemik + snapshot të burimeve,
    /// kthen HardwareScaffold me depth_final = epistemic × hardware.
    pub fn finalize(
        mut scaffold: EpistemicScaffold,
        resources:    &ResourceSnapshot,
    ) -> HardwareScaffold {
        let mass = scaffold.lim.epistemic_mass;

        // ── 1. Masa → DepthHint ────────────────────────────────────────────
        let depth_hint = DepthHint::from_mass(mass);

        // ── 2. DepthHint → RAM budget (sa % bllokohet) ─────────────────────
        // energy_margin moduluar: bateri e ulët → më pak budget (teoria 46).
        // depth_final do reflektojë edhe energjinë përmes available.
        let energy_factor = resources.energy_margin.clamp(0.0, 1.0);
        let effective_ram = (resources.ram_free_bytes as f64 * energy_factor as f64) as u64;
        let budget = ScaleHarduer::allocate(effective_ram, depth_hint);

        // ── 3. depth_hardware = faktori fizik [0,1] ────────────────────────
        let depth_hardware = budget.depth_factor();

        // ── 4. depth_epistemic = ç'ka llogaritur LIM nga teksti ────────────
        let depth_epistemic = scaffold.lim.reasoning_depth_allowance;

        // ── 5. SHUMËZIMI: depth_final = epistemic × hardware ───────────────
        // Ruan të dyja: meritën epistemike DHE kufirin fizik.
        let depth_final = (depth_epistemic * depth_hardware).clamp(0.0, 1.0);

        // ── 6. Rishkruaj reasoning_depth_allowance me vlerën finale ────────
        // Tani PRO do marrë kufijtë REALË (jo vetëm nga teksti).
        scaffold.lim.reasoning_depth_allowance = depth_final;

        // ── 7. Ri-llogarit gates + depth_config me depth-in e ri ───────────
        // (max_depth i operatorëve varet nga reasoning_depth_allowance)
        // Llogaritim në variabla të ndara para assign — pa borrow ambiguity.
        let new_gates = compute_operator_gates(&scaffold.lim);
        let new_depth_config = compute_depth_config(&scaffold.lim);
        scaffold.operator_gates = new_gates;
        scaffold.depth_config = new_depth_config;

        // ── 8. RAM → sa operatorë paralel ──────────────────────────────────
        let parallel = ActParallel::decide(&budget);

        HardwareScaffold {
            scaffold,
            depth_hint,
            budget,
            parallel,
            depth_epistemic,
            depth_hardware,
            depth_final,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lim::LinearInfoMetricEngine;

    fn build_scaffold(turns: &[&str]) -> EpistemicScaffold {
        let mut lim = LinearInfoMetricEngine::new("s", "science");
        for t in turns {
            lim.collect(t);
        }
        lim.pre_pro_scan().expect("scaffold")
    }

    #[test]
    fn depth_final_is_product() {
        let scaffold = build_scaffold(&[
            "deterministic elimination replaces probabilistic generation here",
            "knowledge surviving the filters becomes verifiable primitive knowledge",
        ]);
        let epistemic = scaffold.lim.reasoning_depth_allowance;
        let res = ResourceSnapshot::simulated();
        let hw = LimHwBridge::finalize(scaffold, &res);

        // depth_final = epistemic × hardware
        let expected = (epistemic * hw.depth_hardware).clamp(0.0, 1.0);
        assert!((hw.depth_final - expected).abs() < 1e-5);
        // Dhe scaffold-i u përditësua
        assert!((hw.scaffold.lim.reasoning_depth_allowance - hw.depth_final).abs() < 1e-5);
    }

    #[test]
    fn shallow_still_allocates() {
        // Input i dobët → Shallow → POR prapë merr budget (filozofia).
        // v1.5.1 (Korrigjimi B, TEST-ONLY): fixture plotëson kontratën reale
        // të LimCollector (≥2 fragmente, ≥8 fjalë — pragjet TË PAPREKURA);
        // lutja kontradiktore (pohim + mohim mbi të njëjtën temë) e mban
        // masën epistemike NËN 0.36 → profili mbetet realisht Shallow.
        let scaffold = build_scaffold(&[
            "please help this works fine",
            "please help this does not work",
        ]);
        // Precondition (fusha ekzistuese): Shallow ⟺ epistemic_mass < 0.36.
        assert!(scaffold.lim.epistemic_mass < 0.36,
            "fixture duhet të mbesë Shallow: mass={}", scaffold.lim.epistemic_mass);
        let res = ResourceSnapshot::simulated();
        let hw = LimHwBridge::finalize(scaffold, &res);
        assert!(matches!(hw.depth_hint, DepthHint::Shallow),
            "precondition: DepthHint duhet Shallow");
        assert!(hw.hardware_permits(), "edhe Shallow merr budget jo-zero");
    }

    #[test]
    fn low_battery_reduces_depth() {
        let scaffold = build_scaffold(&[
            "deterministic elimination replaces probabilistic generation",
            "knowledge surviving becomes primitive knowledge over time",
        ]);
        let full = LimHwBridge::finalize(scaffold.clone(), &ResourceSnapshot::simulated());
        let low  = LimHwBridge::finalize(scaffold, &ResourceSnapshot::with(4_000_000_000, 0.10));

        // Bateri e ulët → më pak RAM efektiv → depth_final ≤ full.
        assert!(low.depth_final <= full.depth_final);
    }
}
