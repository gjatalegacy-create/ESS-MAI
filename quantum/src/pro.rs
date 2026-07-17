// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  PRO.RS — PRIMITIVE REASONING ORIGIN — ORCHESTRATORI                  ║
// ║  GJATA LEGACY™ — Quantum Platform                                     ║
// ║                                                                          ║
// ║  RRJEDHA E PLOTË:                                                      ║
// ║   HardwareScaffold (LIM + hardware)                                    ║
// ║     → vektorizo fragmentet                                            ║
// ║     → NK pre-constraint (PARA operatorëve)                            ║
// ║     → ekzekuto operatorët AKTIVË (deri n_parallel nga hardware)       ║
// ║     → NK post-filter (PAS çdo operatori)                              ║
// ║     → max 5 kandidatë, renditur sipas score                           ║
// ║     → ProResult                                                       ║
// ║                                                                          ║
// ║  INTEGRIM: vetëm operatorët me active=true; renditja nga prioriteti;   ║
// ║  numri i operatorëve i kufizuar nga RAM (ActParallel).                ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::lim_types::Operator;
use crate::lim_hw_bridge::HardwareScaffold;
use crate::lim_types::EpistemicScaffold;

/// Ndërfaqe e përbashkët: çdo scaffold hardware (i simuluar ose real)
/// jep scaffold-in epistemik + sa operatorë paralel lejon RAM-i.
pub trait HwScaffoldView {
    fn epistemic(&self) -> &EpistemicScaffold;
    fn parallel_count(&self) -> u8;
    fn runtime_lease(&self) -> Option<crate::hw_real::HwLease> {
        None
    }
    fn operator_allowed(&self, _operator: Operator) -> bool {
        true
    }
    fn work_permitted(&self) -> bool {
        true
    }
}

impl HwScaffoldView for HardwareScaffold {
    fn epistemic(&self) -> &EpistemicScaffold { &self.scaffold }
    fn parallel_count(&self) -> u8 { self.parallel.n_parallel }
}
use crate::pro_types::{PROCandidate, ProResult, FragmentVector, vectorize_fragments};
use crate::pro_operator::{Reverse, Mirror, Contrast, Trace, OperatorOutput};
use crate::pro_nk_gate::{NkGate, KnowledgeVault};
use std::collections::HashSet;

// Max kandidatë (nga teoria 18: PROCandidate Max-5).
const MAX_CANDIDATES: usize = 5;

// ─────────────────────────────────────────────────────────────────────────────
// PRO ENGINE
// ─────────────────────────────────────────────────────────────────────────────

pub struct ProEngine;

impl ProEngine {
    /// Ekzekuton PRO mbi HardwareScaffold-in nga LIM+hardware.
    ///
    /// fragments = grumbullimi origjinal nga LIM (për vektorizim).
    /// vault     = KnowledgeVault me dështimet (NK).
    pub fn activate<H: HwScaffoldView>(
        hw_scaffold: &H,
        fragments:   &[String],
        vault:       &mut KnowledgeVault,
    ) -> ProResult {
        let scaffold = hw_scaffold.epistemic();
        let source_mass = scaffold.lim.epistemic_mass;
        let declared_parallel = hw_scaffold.parallel_count();
        let runtime_lease = hw_scaffold.runtime_lease();
        let hw_parallel = match runtime_lease {
            Some(lease) => u8::try_from(lease.granted_parallel.min(4)).unwrap_or(4),
            None => declared_parallel,
        };
        let (fragment_limit, working_set_bytes) = match runtime_lease {
            Some(lease) => lease.admit_fragment_prefix(fragments),
            None => (
                fragments.len(),
                fragments.iter().fold(0u64, |acc, fragment| {
                    let bytes = u64::try_from(fragment.len()).unwrap_or(u64::MAX);
                    acc.saturating_add(bytes)
                }),
            ),
        };
        let admitted_fragments = &fragments[..fragment_limit.min(fragments.len())];

        // Lease fizik pa RAM ose joaktiv nuk lejon operatorë të rinj.
        match hw_scaffold.work_permitted() {
            false => return ProResult {
                candidates: Vec::new(),
                operators_run: 0,
                nk_blocked: 0,
                source_mass,
                hw_parallel,
                workers_joined: 0,
                fragments_admitted: fragment_limit,
                working_set_bytes,
            },
            true => {}
        }

        // ── Kontroll aktivizimi: ready_for_pro ─────────────────────────────
        // (error-handling i pranuar: kusht boolean)
        match scaffold.ready_for_pro {
            false => return ProResult {
                candidates: Vec::new(),
                operators_run: 0,
                nk_blocked: 0,
                source_mass,
                hw_parallel,
                workers_joined: 0,
                fragments_admitted: fragment_limit,
                working_set_bytes,
            },
            true => {}
        }

        // ── 1. Vektorizo fragmentet (për operatorët) ───────────────────────
        let vectors = vectorize_fragments(admitted_fragments);

        // ── 2. NK PRE-CONSTRAINT (PARA operatorëve) ────────────────────────
        let pre = NkGate::pre_constraint(vault, &vectors);
        // Nëse pre-constraint është HARD → gjithë inputi bllokohet.
        // (kusht: error-handling i pranuar)
        match pre.hard_blocked {
            true => return ProResult {
                candidates: Vec::new(),
                operators_run: 0,
                nk_blocked: 1,
                source_mass,
                hw_parallel,
                workers_joined: 0,
                fragments_admitted: fragment_limit,
                working_set_bytes,
            },
            false => {}
        }

        // ── 3. Operatorët AKTIVË, renditur sipas prioritetit ───────────────
        // active_operators() i kthen tashmë renditur sipas priority.
        let active: Vec<_> = scaffold
            .active_operators()
            .into_iter()
            .filter(|gate| hw_scaffold.operator_allowed(gate.operator))
            .collect();

        // Kufizo nga hardware: deri në n_parallel operatorë të pranuar nga HCP.
        let limit = (hw_parallel as usize).min(active.len());

        // ── 4. Ekzekuto llogaritjen e pastër sipas lease-it ────────────────
        // NK/vault mbetet sekuencial: asnjë state mutable nuk ndahet mes worker-ëve.
        let selected: Vec<(usize, Operator)> = active
            .iter()
            .take(limit)
            .enumerate()
            .map(|(index, gate)| (index, gate.operator))
            .collect();

        let mut outputs: Vec<(usize, OperatorOutput)> = match selected.len() {
            0 => Vec::new(),
            1 => {
                let (index, operator) = selected[0];
                vec![(index, Self::run_operator(operator, scaffold, &vectors))]
            }
            _ => std::thread::scope(|scope| {
                let scaffold_ref = scaffold;
                let vectors_ref = &vectors;
                let handles: Vec<_> = selected
                    .iter()
                    .map(|(index, operator)| {
                        let index = *index;
                        let operator = *operator;
                        scope.spawn(move || {
                            (index, Self::run_operator(operator, scaffold_ref, vectors_ref))
                        })
                    })
                    .collect();

                handles
                    .into_iter()
                    .filter_map(|handle| match handle.join() {
                        Ok(output) => Some(output),
                        Err(_) => None, // worker panic → kandidati nuk pranohet
                    })
                    .collect()
            }),
        };

        // Merge deterministik sipas rendit kanonik të operatorëve.
        outputs.sort_by_key(|(index, _)| *index);

        // ── 5. NK post-filter sekuencial mbi output-et e bashkuara ──────────
        let mut candidates: Vec<PROCandidate> = Vec::new();
        let mut nk_blocked = 0usize;
        let workers_joined = selected.len();
        let operators_run = outputs.len();

        for (_, out) in outputs {
            let constrained = (out.raw_score * pre.factor).clamp(0.0, 1.0);
            let cand_stems = Self::candidate_stems(&out, &vectors);
            let (final_score, post) = NkGate::post_filter(vault, &cand_stems, constrained);
            nk_blocked += post.hard_blocked as usize;

            candidates.push(PROCandidate {
                candidate_id: PROCandidate::make_id(out.operator, &out.fragment_refs),
                operator: out.operator,
                score: final_score,
                confidence: out.confidence,
                reasoning: out.reasoning,
                fragment_refs: out.fragment_refs,
            });
        }

        // ── 6. Rendit sipas score (zbritës) + merr top 5 ───────────────────
        candidates.sort_by(|a, b| {
            match b.score.partial_cmp(&a.score) {
                Some(ordering) => ordering,
                None => std::cmp::Ordering::Equal,
            }
        });
        candidates.truncate(MAX_CANDIDATES);

        ProResult {
            candidates,
            operators_run,
            nk_blocked,
            source_mass,
            hw_parallel,
            workers_joined,
            fragments_admitted: fragment_limit,
            working_set_bytes,
        }
    }

    /// Ekzekuton një operator të vetëm me sinjalet e duhura nga LIM.
    fn run_operator(
        op: Operator,
        scaffold: &crate::lim_types::EpistemicScaffold,
        vectors: &[FragmentVector],
    ) -> OperatorOutput {
        let lim = &scaffold.lim;
        match op {
            Operator::Reverse => {
                // signal = information_density, causal = continuity
                Reverse::compute(
                    lim.information_density,
                    lim.continuity_score,
                    (0..vectors.len()).collect(),
                )
            }
            Operator::Mirror => Mirror::compute(vectors),
            Operator::Contrast => Contrast::compute(vectors),
            Operator::Trace => Trace::compute(vectors),
        }
    }

    /// Stem-et që përfaqësojnë një kandidat (union i fragmenteve të tij).
    fn candidate_stems(out: &OperatorOutput, vectors: &[FragmentVector]) -> HashSet<String> {
        let mut stems = HashSet::new();
        for &r in &out.fragment_refs {
            match vectors.iter().find(|v| v.index == r) {
                Some(v) => stems.extend(v.stems.iter().cloned()),
                None => {}
            }
        }
        stems
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lim::LinearInfoMetricEngine;
    use crate::lim_hw_bridge::LimHwBridge;
    use crate::hardware::ResourceSnapshot;

    fn build_hw_scaffold(turns: &[&str]) -> (HardwareScaffold, Vec<String>) {
        let mut lim = LinearInfoMetricEngine::new("s", "science");
        for t in turns {
            lim.collect(t);
        }
        let scaffold = lim.pre_pro_scan().expect("scaffold");
        let frags: Vec<String> = lim.collector().fragments().to_vec();
        let hw = LimHwBridge::finalize(scaffold, &ResourceSnapshot::simulated());
        (hw, frags)
    }


    #[derive(Clone)]
    struct ControlledScaffold {
        base: HardwareScaffold,
        lease: crate::hw_real::HwLease,
        mask: u8,
    }

    impl HwScaffoldView for ControlledScaffold {
        fn epistemic(&self) -> &EpistemicScaffold { &self.base.scaffold }
        fn parallel_count(&self) -> u8 {
            u8::try_from(self.lease.granted_parallel.min(4)).unwrap_or(4)
        }
        fn runtime_lease(&self) -> Option<crate::hw_real::HwLease> { Some(self.lease) }
        fn operator_allowed(&self, operator: Operator) -> bool {
            let bit = match operator {
                Operator::Reverse => 0x01,
                Operator::Mirror => 0x02,
                Operator::Contrast => 0x04,
                Operator::Trace => 0x08,
            };
            (self.mask & bit) != 0
        }
        fn work_permitted(&self) -> bool {
            self.lease.active & (self.lease.granted_bytes > 0)
        }
    }

    fn controlled(base: HardwareScaffold, bytes: u64, parallel: u32, mask: u8) -> ControlledScaffold {
        ControlledScaffold {
            base,
            lease: crate::hw_real::HwLease {
                lease_id: 0xC011,
                granted_bytes: bytes,
                granted_parallel: parallel.max(1),
                floor_bytes: 0,
                reserved_cores: 1,
                flexed: false,
                active: true,
            },
            mask,
        }
    }

    #[test]
    fn pro_produces_candidates() {
        let (hw, frags) = build_hw_scaffold(&[
            "deterministic elimination replaces probabilistic generation in reasoning",
            "because each candidate passes through negative knowledge filters first",
            "therefore surviving knowledge becomes verifiable primitive knowledge",
        ]);
        let mut vault = KnowledgeVault::new();
        let result = ProEngine::activate(&hw, &frags, &mut vault);

        assert!(!result.candidates.is_empty(), "PRO duhet të prodhojë kandidatë");
        assert!(result.candidates.len() <= MAX_CANDIDATES);
        // Renditur zbritës
        for w in result.candidates.windows(2) {
            assert!(w[0].score >= w[1].score);
        }
    }

    #[test]
    fn respects_hardware_parallel_limit() {
        let (hw, frags) = build_hw_scaffold(&[
            "quantum reasoning eliminates wrong paths deterministically here",
            "shadow authority verifies with binary zero or one decision",
        ]);
        let mut vault = KnowledgeVault::new();
        let result = ProEngine::activate(&hw, &frags, &mut vault);
        // operatorët e ekzekutuar ≤ n_parallel nga hardware
        assert!(result.operators_run <= hw.parallel.n_parallel as usize);
    }

    #[test]
    fn nk_blocks_known_failure() {
        let (hw, frags) = build_hw_scaffold(&[
            "perpetual motion machine produces infinite free energy forever",
            "the device generates more energy than it consumes continuously",
        ]);
        let mut vault = KnowledgeVault::new();
        // Regjistro dështimin e njohur
        vault.add_negative(crate::pro_nk_gate::NegativeEntry::from_text(
            "thermo_fail",
            "perpetual motion machine infinite free energy generates more than consumes",
        ));
        let result = ProEngine::activate(&hw, &frags, &mut vault);
        // Score-t duhet të jenë të ulëta ose disa kandidatë të bllokuar.
        let max_score = result.candidates.iter()
            .map(|c| c.score).fold(0.0f32, f32::max);
        assert!(max_score < 0.9 || result.nk_blocked > 0,
            "NK duhet të ndëshkojë rrugën e njohur si dështim");
    }

    #[test]
    fn not_ready_produces_nothing() {
        // v1.5.1 (Korrigjimi B, TEST-ONLY): scaffold-i ndërtohet VALID nga
        // fixture që plotëson kontratën LIM (≥2 fragmente, ≥8 fjalë —
        // pragjet TË PAPREKURA); pastaj ready_for_pro vendoset false në
        // mënyrë DETERMINISTE (state i përcaktuar, jo varësi nga masa).
        let (mut hw, frags) = build_hw_scaffold(&[
            "short text here today",
            "short text here again",
        ]);
        hw.scaffold.ready_for_pro = false; // state i përcaktuar
        // Precondition (kontrata): scaffold valid, POR ready_for_pro == false.
        assert_eq!(hw.scaffold.ready_for_pro, false,
            "precondition: ready_for_pro duhet false para PRO-s");
        let mut vault = KnowledgeVault::new();
        let result = ProEngine::activate(&hw, &frags, &mut vault);
        assert!(result.candidates.is_empty(),
            "PRO s'duhet të prodhojë kandidatë kur scaffold s'është gati");
    }

    #[test]
    fn hcp_operator_mask_blocks_unadmitted_operators() {
        let (base, frags) = build_hw_scaffold(&[
            "deterministic elimination preserves causal trace through every operator",
            "negative knowledge blocks invalid hypotheses before verification",
        ]);
        let hw = controlled(base, 1024 * 1024, 4, 0);
        let mut vault = KnowledgeVault::new();
        let result = ProEngine::activate(&hw, &frags, &mut vault);
        assert_eq!(result.operators_run, 0);
        assert_eq!(result.workers_joined, 0);
        assert!(result.candidates.is_empty());
    }

    #[test]
    fn lease_budget_limits_working_set_before_vectorization() {
        let (base, frags) = build_hw_scaffold(&[
            "first deterministic fragment carries enough semantic material for the test",
            "second deterministic fragment must be excluded by the small runtime lease",
            "third deterministic fragment must also remain outside the admitted prefix",
        ]);
        let probe = controlled(base.clone(), u64::MAX, 2, 0x0F);
        let (_, first_cost) = probe.lease.admit_fragment_prefix(&frags[..1]);
        let hw = controlled(base, first_cost, 2, 0x0F);
        let mut vault = KnowledgeVault::new();
        let result = ProEngine::activate(&hw, &frags, &mut vault);
        assert_eq!(result.fragments_admitted, 1);
        assert_eq!(result.working_set_bytes, first_cost);
    }

    #[test]
    fn parallel_execution_remains_deterministic_and_bounded() {
        let (base, frags) = build_hw_scaffold(&[
            "deterministic elimination replaces probabilistic generation in reasoning systems",
            "causal continuity and negative knowledge preserve a stable evidence lineage",
            "shadow later verifies the evidence without changing quantum reasoning authority",
        ]);
        let hw = controlled(base, 4 * 1024 * 1024, 3, 0x0F);
        let mut vault_a = KnowledgeVault::new();
        let mut vault_b = KnowledgeVault::new();
        let a = ProEngine::activate(&hw, &frags, &mut vault_a);
        let b = ProEngine::activate(&hw, &frags, &mut vault_b);
        assert!(a.workers_joined <= 3);
        assert!(b.workers_joined <= 3);
        let ids_a: Vec<_> = a.candidates.iter().map(|c| (&c.candidate_id, c.score)).collect();
        let ids_b: Vec<_> = b.candidates.iter().map(|c| (&c.candidate_id, c.score)).collect();
        assert_eq!(ids_a, ids_b);
    }

    #[test]
    fn zero_byte_lease_prevents_real_work() {
        let (base, frags) = build_hw_scaffold(&[
            "hardware pressure must stop new reasoning work when no RAM budget remains",
            "the runtime still returns a typed empty result instead of panicking",
        ]);
        let hw = controlled(base, 0, 2, 0x0F);
        let mut vault = KnowledgeVault::new();
        let result = ProEngine::activate(&hw, &frags, &mut vault);
        assert_eq!(result.fragments_admitted, 0);
        assert_eq!(result.working_set_bytes, 0);
        assert_eq!(result.workers_joined, 0);
        assert!(result.candidates.is_empty());
    }

}
