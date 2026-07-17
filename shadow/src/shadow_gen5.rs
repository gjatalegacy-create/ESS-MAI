// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  SHADOW_GEN5.RS — S.GEN5                                               ║
// ║  Gjata Legacy™  |  Arkitekt: Bledar Gjata                             ║
// ║                                                                          ║
// ║  Verifikon strukturën vektoriale të dosjes.                           ║
// ║  Implementon formalizimin matematik të Primitive Knowledge Gate:       ║
// ║                                                                          ║
// ║  G_mass  = clamp(floor(M_e), 0, 1)                                    ║
// ║  Φ_prim  = 0.35×W_Δ + 0.30×W_Sc + 0.20×E_v + 0.15×C_on             ║
// ║  G_score = clamp(floor(Φ_prim + 0.15), 0, 1)                         ║
// ║  P_gate  = G_mass × G_score                                            ║
// ║                                                                          ║
// ║  ZERO if/else. Branchless tensor execution.                           ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::types::{PassPackage, ShadowNode, ShadowPassage, ShadowVerdict};

pub struct Gen5Analysis {
    pub g_mass:       f32,
    pub phi_prim:     f32,
    pub g_score:      f32,
    pub p_gate:       f32,   /* G_mass × G_score — porta primitive          */
    pub drift_weight: f32,
    pub stab_weight:  f32,
}

pub struct ShadowGen5;

impl ShadowGen5 {
    /// Verifikon strukturën vektoriale — formalizimi matematikor i plotë.
    ///
    /// State Space Vector K⃗:
    ///   M_e  — epistemic mass [0,1]
    ///   Δ    — drift (1 - seal_strength) nga passage
    ///   S_c  — stability cycles (branch_count nga EvolveTrace proxy)
    ///   E_v  — emergence verdict nga passage (S.Emergence nëse Deep, else 1)
    ///   C_on — consensus score nga candidate_scores
    pub fn verify(passage: &ShadowPassage, ts_ns: u64) -> (Gen5Analysis, ShadowVerdict) {
        let pkg = &passage.package;
        let m_e = pkg.epistemic_mass.clamp(0.0, 1.0);

        // ── State Space ───────────────────────────────────────────────────

        // Δ (drift) = 1 - potentiality_score si proxy i fragmentimit
        let delta = (1.0 - pkg.potentiality_score).clamp(0.0, 1.0);

        // S_c (stability cycles) — normalizuar mbi 5
        let s_c = pkg.candidate_scores.len() as f32;
        let stability_cycles = (s_c / 5.0).clamp(0.0, 1.0);

        // E_v (emergence verdict) — 1 nëse passage ka kaluar deri këtu
        let e_v = passage.last_verdict()
            .map(|v| v.score)
            .unwrap_or(0.5)
            .clamp(0.0, 1.0);

        // C_on (consensus score) — mesatar i candidate scores
        let c_on = match pkg.candidate_scores.is_empty() {
            true => 0.0f32,
            false => {
                let sum: f32 = pkg.candidate_scores.iter().sum();
                (sum / pkg.candidate_scores.len() as f32).clamp(0.0, 1.0)
            }
        };

        // ── Normalizim branchless ─────────────────────────────────────────

        // W_Δ = clamp(1.0 - Δ, 0.0, 1.0)
        let w_delta = (1.0 - delta).clamp(0.0, 1.0);

        // W_Sc = clamp(S_c / 5.0, 0.0, 1.0)  — tashmë normalizuar
        let w_sc = stability_cycles;

        // ── Primitive Projection Φ_prim ───────────────────────────────────
        // Φ_prim = 0.35×W_Δ + 0.30×W_Sc + 0.20×E_v + 0.15×C_on
        let phi_prim = (0.35 * w_delta
                      + 0.30 * w_sc
                      + 0.20 * e_v
                      + 0.15 * c_on)
                      .clamp(0.0, 1.0);

        // ── Branchless 0/1 Gates ──────────────────────────────────────────

        // G_mass = clamp(floor(M_e), 0, 1)
        // M_e >= 1.0 → 1, çdo gjë tjetër → 0
        // floor() branchless: si u32 cast pas clamp
        let g_mass_f  = m_e.floor().clamp(0.0, 1.0);
        let g_mass    = g_mass_f;

        // G_score = clamp(floor(Φ_prim + 0.15), 0, 1)
        // Φ_prim >= 0.85 → floor → 1; Φ_prim < 0.85 → floor → 0
        let g_score_f = (phi_prim + 0.15).floor().clamp(0.0, 1.0);
        let g_score   = g_score_f;

        // P_gate = G_mass × G_score — tensor i ekzekutueshëm në 1 cikël
        let p_gate = g_mass * g_score;

        let analysis = Gen5Analysis {
            g_mass,
            phi_prim,
            g_score,
            p_gate,
            drift_weight: w_delta,
            stab_weight:  w_sc,
        };

        // ── Score finale e S.Gen5 ─────────────────────────────────────────
        // Kombinon phi_prim me m_e — strukturë e fortë = të dyja të larta
        let gen5_score = (phi_prim * 0.60 + m_e * 0.40).clamp(0.0, 1.0);

        let verdict = ShadowVerdict::from_score(
            ShadowNode::Gen5,
            gen5_score,
            p_gate as u8,   /* reason_code: 1 = PRIMITIVE gate aktiv        */
            ts_ns,
        );

        (analysis, verdict)
    }

    /// Legacy Score për KnowledgeVault — formula e plotë e maturimit.
    ///
    /// Legacy = Usage_Diversity + Domain_Diversity + Survival_Rate
    ///        + Negative_Challenge_Resistance + Time_Stability
    ///
    /// ZERO if/else — vektor i peshuar.
    pub fn legacy_score_compute(pkg: &PassPackage) -> f32 {
        let usage_diversity    = pkg.evidence_density.clamp(0.0, 1.0);
        let domain_diversity   = pkg.logical_coherence.clamp(0.0, 1.0);
        let survival_rate      = pkg.causal_integrity.clamp(0.0, 1.0);
        let neg_challenge_res  = pkg.convergence_strength.clamp(0.0, 1.0);
        let time_stability     = pkg.reproducibility.clamp(0.0, 1.0);

        (usage_diversity     * 0.25
       + domain_diversity    * 0.20
       + survival_rate       * 0.25
       + neg_challenge_res   * 0.15
       + time_stability      * 0.15)
        .clamp(0.0, 1.0)
    }
}

/// Verifikon nëse PassPackage kalon portën Primitive — branchless.
pub fn primitive_gate(pkg: &PassPackage) -> u8 {
    let m_e      = pkg.epistemic_mass.clamp(0.0, 1.0);
    let delta    = (1.0 - pkg.potentiality_score).clamp(0.0, 1.0);
    let s_c      = (pkg.candidate_scores.len() as f32 / 5.0).clamp(0.0, 1.0);
    let e_v      = pkg.convergence_strength.clamp(0.0, 1.0);
    let c_on     = pkg.reproducibility.clamp(0.0, 1.0);

    let w_delta  = (1.0 - delta).clamp(0.0, 1.0);
    let phi_prim = (0.35 * w_delta + 0.30 * s_c + 0.20 * e_v + 0.15 * c_on)
                   .clamp(0.0, 1.0);

    let g_mass   = m_e.floor().clamp(0.0, 1.0);
    let g_score  = (phi_prim + 0.15).floor().clamp(0.0, 1.0);

    (g_mass * g_score) as u8
}
