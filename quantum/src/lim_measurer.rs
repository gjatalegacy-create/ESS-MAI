// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LIM_MEASURER.RS — GRUPI 2 (MATËSI)                                   ║
// ║  GJATA LEGACY™ — Quantum Platform / Linear Info Metric                ║
// ║                                                                          ║
// ║  ROLI: mat 8 akset mbi GRUMBULLIMIN (Vec<String>), jo mbi 1 mesazh.   ║
// ║  Zemra: KRAHASIM MIDIS FRAGMENTEVE (jo embedding i secilit).          ║
// ║                                                                          ║
// ║  4 akset "problematike" zgjidhen me akumulim:                         ║
// ║   • contradiction: fragment[i] vs fragment[j] leksikalisht            ║
// ║   • semantic_integrity: struktura SVO e tërë konversacionit           ║
// ║   • information_density: sa i ri fragment[n] vs [0..n] (overlap zbret) ║
// ║   • exploration_readiness: raport pyetje vs deklarata                 ║
// ║                                                                          ║
// ║  Peshat e epistemic_mass janë IDENTIKE me lim_core.pseudo.            ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::lim_types::*;
use crate::tokenizer::*;
use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};

// ─────────────────────────────────────────────────────────────────────────────
// GRUPI 2 — MEASURER
// ─────────────────────────────────────────────────────────────────────────────

pub struct LimMeasurer;

impl LimMeasurer {
    /// Mat 8 akset mbi grumbullimin e fragmenteve → LinearInfoMetric.
    /// Faza 1 (PRE_PRO): mbi input të akumuluar nga përdoruesi.
    pub fn measure(
        fragments:  &[String],
        territory:  &str,
        session_id: &str,
    ) -> LinearInfoMetric {
        // Pre-llogarit set-et e stem-eve për çdo fragment (një herë).
        let stem_sets: Vec<HashSet<String>> =
            fragments.iter().map(|f| semantic_stems(f)).collect();

        // ── AKS 1: INFORMATION DENSITY ─────────────────────────────────────
        // Sa informacion i RI sjell çdo fragment vs të mëparshmit.
        // Overlap me të kaluarën ZBRITET (fragment që përsërit = pak info i ri).
        let information_density = Self::axis_information_density(fragments, &stem_sets);

        // ── AKS 2: SEMANTIC CONTINUITY ─────────────────────────────────────
        // A lidhen fragmentet fqinje (Jaccard midis [i] dhe [i+1]).
        let continuity_score = Self::axis_continuity(&stem_sets);

        // ── AKS 3: CONTRADICTION PRESENCE ──────────────────────────────────
        // fragment[i] vs fragment[j] — konflikt negacioni/kundërvënie.
        let contradiction_presence = Self::axis_contradiction(fragments);

        // ── AKS 4: SEMANTIC INTEGRITY ──────────────────────────────────────
        // Struktura logjike e TËRË konversacionit (shenjues + SVO ratio).
        let semantic_integrity = Self::axis_semantic_integrity(fragments);

        // ── AKS 5: HYPOTHESIS SUPPORT ──────────────────────────────────────
        let hypothesis_support_score = Self::axis_hypothesis_support(
            information_density, continuity_score, semantic_integrity,
        );

        // ── AKS 6: CONTEXTUAL LINEARITY ────────────────────────────────────
        // Sa pak "kërcen" tema nga fragmenti në fragment.
        let contextual_linearity = Self::axis_contextual_linearity(&stem_sets);

        // ── AKS 7: EXPLORATION READINESS ───────────────────────────────────
        let exploration_readiness = Self::axis_exploration_readiness(
            information_density, hypothesis_support_score,
            continuity_score, contextual_linearity,
        );

        // ── AKS 8: REASONING DEPTH ALLOWANCE ───────────────────────────────
        let reasoning_depth_allowance = Self::axis_depth_allowance(
            exploration_readiness, contradiction_presence, information_density,
        );

        // ── EPISTEMIC MASS — peshat IDENTIKE me lim_core.pseudo ────────────
        let epistemic_mass = Self::epistemic_mass(
            information_density, continuity_score, contradiction_presence,
            semantic_integrity, hypothesis_support_score, contextual_linearity,
            exploration_readiness, reasoning_depth_allowance,
        );

        LinearInfoMetric {
            information_density,
            continuity_score,
            contradiction_presence,
            semantic_integrity,
            hypothesis_support_score,
            contextual_linearity,
            exploration_readiness,
            reasoning_depth_allowance,
            epistemic_mass,
            epistemic_state: None,
            session_id: session_id.to_string(),
            territory: territory.to_string(),
            computed_at: now_ms(),
            phase: LimPhase::PrePro,
        }
    }

    // ═════════════════════════════════════════════════════════════════════
    // AKS 1: INFORMATION DENSITY — sa info i ri (overlap zbret)
    // ═════════════════════════════════════════════════════════════════════
    fn axis_information_density(
        fragments: &[String],
        stems: &[HashSet<String>],
    ) -> f32 {
        match fragments.is_empty() {
            true => return 0.0,
            false => {}
        }

        // (a) Diversiteti leksikal global (Type-Token Ratio mbi gjithçka).
        let combined = fragments.join(" ");
        let all_toks = tokenize(&combined);
        let semantic: Vec<&Token> = all_toks.iter().filter(|t| !t.is_stopword).collect();
        let unique: HashSet<&String> = semantic.iter().map(|t| &t.stem).collect();
        let ttr = match semantic.is_empty() {
            true  => 0.0,
            false => unique.len() as f32 / semantic.len() as f32,
        };

        // (b) Novelty: sa i ri është çdo fragment vs union i të mëparshmëve.
        // overlap i lartë me të kaluarën → novelty e ulët → density zbret.
        let mut novelty_sum = 0.0;
        let mut seen: HashSet<String> = HashSet::new();
        for (i, s) in stems.iter().enumerate() {
            // Fragmenti i parë = krejt i ri (novelty 1.0); të tjerët = 1 - overlap.
            // Branchless: is_first ∈ {0,1}.
            let is_first = (i == 0) as u32 as f32;
            let novelty = is_first * 1.0 + (1.0 - is_first) * (1.0 - overlap_ratio(&seen, s));
            novelty_sum += novelty;
            seen.extend(s.iter().cloned());
        }
        let avg_novelty = novelty_sum / stems.len() as f32;

        // Kombinim: diversitet leksikal + novelty ndër-fragmente.
        clamp01(ttr * 0.45 + avg_novelty * 0.55)
    }

    // ═════════════════════════════════════════════════════════════════════
    // AKS 2: SEMANTIC CONTINUITY — Jaccard midis fragmenteve fqinje
    // (APROKSIM i cosine_similarity(embeddings) → Rruga C zëvendëson)
    // ═════════════════════════════════════════════════════════════════════
    fn axis_continuity(stems: &[HashSet<String>]) -> f32 {
        match stems.len() < 2 {
            true => return 1.0, // fragment i vetëm = kontinuitet perfekt
            false => {}
        }
        let mut scores = Vec::new();
        for i in 0..stems.len() - 1 {
            scores.push(jaccard(&stems[i], &stems[i + 1]));
        }
        mean(&scores)
    }

    // ═════════════════════════════════════════════════════════════════════
    // AKS 3: CONTRADICTION — fragment[i] vs fragment[j] (negacion + kundërvënie)
    // ═════════════════════════════════════════════════════════════════════
    fn axis_contradiction(fragments: &[String]) -> f32 {
        match fragments.len() < 2 {
            true => return 0.0,
            false => {}
        }
        // Krahaso çdo fragment me ata pranë (dritare prej 3, si pseudokodi).
        let mut scores = Vec::new();
        for i in 0..fragments.len() {
            let upper = (i + 4).min(fragments.len());
            for j in (i + 1)..upper {
                scores.push(Self::pair_contradiction(&fragments[i], &fragments[j]));
            }
        }
        match scores.is_empty() {
            true => return 0.0,
            false => {}
        }
        // Media e peshuar — konfliktet e hershme peshojnë më shumë.
        let mut weighted = 0.0;
        for (i, s) in scores.iter().enumerate() {
            weighted += s * (1.0 - i as f32 * 0.05).max(0.1);
        }
        clamp01(weighted / scores.len() as f32)
    }

    /// Konflikti midis dy fragmenteve [0,1] — leksikal, jo semantik i thellë.
    fn pair_contradiction(a: &str, b: &str) -> f32 {
        // (1) Konflikt negacioni: njëri ka negim, tjetri jo, mbi temë të përbashkët.
        const NEG: &[&str] = &["not", "no", "never", "nuk", "s'", "jo", "pa", "mos"];
        let a_neg = contains_any(a, NEG);
        let b_neg = contains_any(b, NEG);
        let shared = jaccard(&semantic_stems(a), &semantic_stems(b));
        // Temë e përbashkët (shared > 0.2) por njëri mohon e tjetri jo.
        let neg_conflict = (((a_neg != b_neg) && shared > 0.2) as u32) as f32;

        // (2) Konflikt sasior: numra të ndryshëm pranë fjalëve të njëjta.
        let quant_conflict = Self::quantitative_conflict(a, b);

        // (3) Kundërvënie eksplicite: shenjues kontrasti.
        const OPP: &[&str] = &["but", "however", "por", "megjithatë", "kundër", "ndryshe"];
        let opp = ((contains_any(a, OPP) || contains_any(b, OPP)) as u32 as f32) * 0.4;

        neg_conflict.max(quant_conflict).max(opp)
    }

    /// Konflikt sasior: a kanë numra të ndryshëm mbi temë të përbashkët?
    fn quantitative_conflict(a: &str, b: &str) -> f32 {
        let nums_a = extract_numbers(a);
        let nums_b = extract_numbers(b);
        match nums_a.is_empty() || nums_b.is_empty() {
            true => return 0.0,
            false => {}
        }
        let shared = jaccard(&semantic_stems(a), &semantic_stems(b));
        // Temë e përbashkët + numra që s'përputhen → konflikt.
        let any_diff = nums_a.iter().any(|x| nums_b.iter().all(|y| (x - y).abs() > 1e-6));
        ((shared > 0.25 && any_diff) as u32 as f32) * 0.8
    }

    // ═════════════════════════════════════════════════════════════════════
    // AKS 4: SEMANTIC INTEGRITY — struktura e TËRË konversacionit
    // ═════════════════════════════════════════════════════════════════════
    fn axis_semantic_integrity(fragments: &[String]) -> f32 {
        let raw = fragments.join(" ");

        // (a) Shenjues logjikë (premise, conclusion, condition, evidence, quantifier).
        let has_premise    = contains_any(&raw, &["sepse", "pasi", "meqë", "because", "since"]);
        let has_conclusion = contains_any(&raw, &["prandaj", "kështu", "therefore", "thus", "hence"]);
        let has_condition  = contains_any(&raw, &["nëse", "kur", "if", "when", "provided"]);
        let has_evidence   = contains_any(&raw, &["sipas", "bazuar", "according", "based on"]);
        let has_quantifier = contains_any(&raw, &["të gjithë", "disa", "all", "some", "every"]);

        let markers = [has_premise, has_conclusion, has_condition, has_evidence, has_quantifier]
            .iter().filter(|b| **b).count();
        let marker_score = markers as f32 / 5.0;

        // (b) SVO ratio: raporti i fjalive me strukturë (subjekt-folje-objekt).
        // Aproksim: fjali me ≥3 fjalë jo-stopword dhe një folje-kandidate.
        let svo_score = Self::svo_ratio(fragments);

        clamp01(marker_score * 0.50 + svo_score * 0.50)
    }

    /// Raporti i fjalive me strukturë SVO-like [0,1].
    fn svo_ratio(fragments: &[String]) -> f32 {
        let raw = fragments.join(" ");
        let sentences = split_sentences(&raw);
        match sentences.is_empty() {
            true => return 0.0,
            false => {}
        }
        let structured = sentences.iter().filter(|s| {
            let toks = tokenize(s);
            let content = toks.iter().filter(|t| !t.is_stopword).count();
            // Fjali e strukturuar: ≥3 fjalë përmbajtësore.
            content >= 3
        }).count();
        structured as f32 / sentences.len() as f32
    }

    // ═════════════════════════════════════════════════════════════════════
    // AKS 5: HYPOTHESIS SUPPORT — synergji nëse të 3 mbi prag
    // ═════════════════════════════════════════════════════════════════════
    fn axis_hypothesis_support(density: f32, continuity: f32, integrity: f32) -> f32 {
        // Nën prag → pa synergji; mbi prag → base + synergy. Branchless.
        // below_threshold ∈ {0,1}.
        let below = ((density < 0.20) || (continuity < 0.25) || (integrity < 0.20)) as u32 as f32;
        let linear = density * 0.30 + continuity * 0.35 + integrity * 0.35;
        let base = density * 0.35 + continuity * 0.35 + integrity * 0.30;
        let synergy = density.min(continuity).min(integrity) * 0.20;
        let with_synergy = base + synergy;
        clamp01(below * linear + (1.0 - below) * with_synergy)
    }

    // ═════════════════════════════════════════════════════════════════════
    // AKS 6: CONTEXTUAL LINEARITY — 1 - topic drift
    // ═════════════════════════════════════════════════════════════════════
    fn axis_contextual_linearity(stems: &[HashSet<String>]) -> f32 {
        match stems.len() < 2 {
            true => return 1.0,
            false => {}
        }
        let mut drifts = Vec::new();
        for i in 0..stems.len() - 1 {
            // drift = 1 - similarity (tema ndryshon = drift i lartë)
            drifts.push(1.0 - jaccard(&stems[i], &stems[i + 1]));
        }
        clamp01(1.0 - mean(&drifts))
    }

    // ═════════════════════════════════════════════════════════════════════
    // AKS 7: EXPLORATION READINESS — baza epistemike + raport pyetje/deklarata
    // ═════════════════════════════════════════════════════════════════════
    fn axis_exploration_readiness(
        density: f32, hypothesis: f32, continuity: f32, linearity: f32,
    ) -> f32 {
        let base = density * 0.30 + hypothesis * 0.40
                 + continuity * 0.20 + linearity * 0.10;
        let bonus = ([density, hypothesis, continuity, linearity]
            .iter().all(|v| *v > 0.40) as u32 as f32) * 0.10;
        clamp01(base + bonus)
    }

    // ═════════════════════════════════════════════════════════════════════
    // AKS 8: DEPTH ALLOWANCE — readiness - penalitet kontradikta (min 0.05)
    // ═════════════════════════════════════════════════════════════════════
    fn axis_depth_allowance(readiness: f32, contradiction: f32, density: f32) -> f32 {
        let raw = readiness * 0.70 + density * 0.30;
        let penalty = contradiction * 0.35;
        (raw - penalty).clamp(0.05, 1.0)
    }

    // ═════════════════════════════════════════════════════════════════════
    // EPISTEMIC MASS — peshat IDENTIKE me lim_core.pseudo
    //   W = {density:0.18, continuity:0.15, contradiction:0.12(inv),
    //        integrity:0.15, hypothesis:0.15, linearity:0.10,
    //        readiness:0.10, depth:0.05}
    //   contradiction_factor = 1.0 - (contradiction * 0.40)
    // ═════════════════════════════════════════════════════════════════════
    #[allow(clippy::too_many_arguments)]
    pub fn epistemic_mass(
        density: f32, continuity: f32, contradiction: f32,
        integrity: f32, hypothesis: f32, linearity: f32,
        readiness: f32, depth: f32,
    ) -> f32 {
        let contradiction_factor = 1.0 - (contradiction * 0.40);
        let mass = (
            density    * 0.18 +
            continuity * 0.15 +
            integrity  * 0.15 +
            hypothesis * 0.15 +
            linearity  * 0.10 +
            readiness  * 0.10 +
            depth      * 0.05
        ) * contradiction_factor;
        clamp01(mass)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// NDIHMËS NUMERIKË
// ─────────────────────────────────────────────────────────────────────────────

#[inline(always)]
fn clamp01(x: f32) -> f32 {
    x.clamp(0.0, 1.0)
}

fn mean(xs: &[f32]) -> f32 {
    // Branchless: bosh → 0; ndryshe → mesatare. max(1) shmang pjesëtimin me 0.
    let is_empty = xs.is_empty() as u32 as f32;
    let sum = xs.iter().sum::<f32>();
    (1.0 - is_empty) * (sum / xs.len().max(1) as f32)
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

/// Nxjerr numrat nga teksti (për konfliktin sasior).
fn extract_numbers(text: &str) -> Vec<f32> {
    text.split(|c: char| !c.is_numeric() && c != '.' && c != '-')
        .filter_map(|s| s.parse::<f32>().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frags(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn mass_in_range() {
        let f = frags(&["the system reasons deterministically", "it eliminates wrong paths"]);
        let lim = LimMeasurer::measure(&f, "science", "s1");
        assert!(lim.epistemic_mass >= 0.0 && lim.epistemic_mass <= 1.0);
        for ax in [lim.information_density, lim.continuity_score,
                   lim.contradiction_presence, lim.semantic_integrity] {
            assert!((0.0..=1.0).contains(&ax));
        }
    }

    #[test]
    fn contradiction_detected() {
        // Dy fragmente: njëri pohon, tjetri mohon mbi temë të përbashkët.
        let f = frags(&[
            "the reactor is stable under load",
            "the reactor is not stable under load",
        ]);
        let lim = LimMeasurer::measure(&f, "industrial", "s1");
        assert!(lim.contradiction_presence > 0.0,
            "duhet të kapë konfliktin e negacionit");
    }

    #[test]
    fn novelty_lowers_density_on_repetition() {
        let repeated = frags(&["quantum system", "quantum system", "quantum system"]);
        let diverse  = frags(&["quantum system", "neural networks", "thermal dynamics"]);
        let lim_rep = LimMeasurer::measure(&repeated, "x", "s1");
        let lim_div = LimMeasurer::measure(&diverse, "x", "s2");
        assert!(lim_div.information_density > lim_rep.information_density,
            "fragmente të reja → densitet më i lartë se përsëritje");
    }

    #[test]
    fn continuity_high_for_related() {
        let related = frags(&["reasoning engine design", "reasoning engine architecture"]);
        let lim = LimMeasurer::measure(&related, "x", "s1");
        assert!(lim.continuity_score > 0.2);
    }
}
