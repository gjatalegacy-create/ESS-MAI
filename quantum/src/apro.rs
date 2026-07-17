// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  APRO.RS — ARGUMENTUESI (faza APRO e gjata_collapse_law)                 ║
// ║  Gjata Legacy™  |  Arkitekt: Bledar Gjata                                ║
// ║                                                                          ║
// ║  LIGJI: … PRO → i ≈ XY → APRO → MPRO …                                   ║
// ║                                                                          ║
// ║  APRO ARGUMENTON kandidatët e PRO dhe eliminimet e NPRO — s'krijon      ║
// ║  kandidatë, s'eliminon, s'vendos. Prodhon 4 ARGUMENTE të matura (0/1)   ║
// ║  që hyjnë si familja e 4-t në MeasurementSet-in 16-operatorësh të MPRO. ║
// ║                                                                          ║
// ║  4 ARGUMENTET (secili nga matje REALE ekzistuese — zero hamendje):       ║
// ║    A1 EKZISTENCA:  PRO prodhoi kandidatë realë (jashtë bllokimit NK).   ║
// ║    A2 BAZA:        masa burimore e LIM ≥ 0.20 (pragu real ready_for_pro)║
// ║    A3 MBIJETESA:   NPRO argumentoi por s'e zbrazi hapësirën              ║
// ║                    (avg_weakness < WEAKNESS_CEILING 0.65 — konst reale). ║
// ║    A4 DYANSHMËRIA: të dy krahët ekzekutuan operatorë (PRO∧NPRO > 0) —   ║
// ║                    argumenti pa kundër-argument s'është argument.        ║
// ║                                                                          ║
// ║  LAW 0: APRO ul pasigurinë duke PESHUAR — dalja e tij hyn në MPRO.       ║
// ║  LIGJE: zero if/else (match), zero clone, zero deps, zero vendim.        ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::pro_types::ProResult;
use crate::npro::{NproResult, WEAKNESS_CEILING};

/// Pragu real i gatishmërisë së masës (i njëjti me LIM ready_for_pro).
pub const APRO_MASS_FLOOR: f32 = 0.20;

/// Argumenti i APRO-s: 4 matje 0/1 + bazat gjuhësore të secilës.
#[derive(Debug, Clone)]
pub struct AproArgument {
    /// [A1, A2, A3, A4] — hyrja e drejtpërdrejtë për MeasurementSet.apro.
    pub measures: [u8; 4],
    /// Baza e secilit argument (dëshmi e lexueshme — white-box).
    pub grounds:  [&'static str; 4],
    /// Sa argumente qëndruan (0..=4).
    pub standing: u32,
}

impl AproArgument {
    pub fn report(&self) -> String {
        format!("APRO: {}/4 argumente qëndruan [{}{}{}{}]",
            self.standing,
            self.measures[0], self.measures[1],
            self.measures[2], self.measures[3])
    }
}

pub struct Apro;

impl Apro {
    /// argue — argumenton kandidatët e PRO dhe eliminimet e NPRO.
    /// Vetëm LEXON rezultatet e matura; kurrë s'i ndryshon. Zero if — match.
    pub fn argue(pro: &ProResult, npro: &NproResult) -> AproArgument {
        // A1: ekzistenca reale e kandidatëve (pas bllokimit NK).
        let a1: u8 = match !pro.candidates.is_empty() {
            true  => 1,
            false => 0,
        };
        // A2: baza — masa burimore e LIM mbi dyshemenë reale.
        let a2: u8 = match pro.source_mass >= APRO_MASS_FLOOR {
            true  => 1,
            false => 0,
        };
        // A3: mbijetesa — NPRO foli, por s'e zbrazi hapësirën.
        let a3: u8 = match npro.weakness_signal.avg_weakness < WEAKNESS_CEILING {
            true  => 1,
            false => 0,
        };
        // A4: dyanshmëria — të dy krahët ekzekutuan (argument ∧ kundër-argument).
        let a4: u8 = match pro.operators_run > 0 && npro.operators_run > 0 {
            true  => 1,
            false => 0,
        };

        let standing = (a1 + a2 + a3 + a4) as u32;
        AproArgument {
            measures: [a1, a2, a3, a4],
            grounds: [
                "A1 ekzistenca: kandidatë realë pas NK",
                "A2 baza: source_mass ≥ 0.20 (LIM ready)",
                "A3 mbijetesa: avg_weakness < 0.65 (ceiling)",
                "A4 dyanshmëria: PRO∧NPRO operatorë > 0",
            ],
            standing,
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// TESTE
// ════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pro_types::PROCandidate;
    use crate::lim_types::Operator;
    use crate::npro::WeaknessSignal;

    fn pro_with(cands: usize, mass: f32, ops: usize) -> ProResult {
        let candidates = (0..cands).map(|i| PROCandidate {
            candidate_id:  format!("c{}", i),
            operator:      Operator::Mirror,
            score:         0.7,
            confidence:    0.7,
            reasoning:     "t".to_string(),
            fragment_refs: vec![0],
        }).collect();
        ProResult { candidates, operators_run: ops, nk_blocked: 0,
                    source_mass: mass, hw_parallel: 1,
                    workers_joined: ops, fragments_admitted: 1, working_set_bytes: 64 }
    }

    fn npro_with(avg: f32, ops: usize) -> NproResult {
        NproResult {
            eliminations: Vec::new(),
            weakness_signal: WeaknessSignal {
                forbidden_paths: Vec::new(),
                weakness_ceiling: WEAKNESS_CEILING,
                avg_weakness: avg,
            },
            operators_run: ops,
        }
    }

    #[test]
    fn four_arguments_all_standing() {
        let a = Apro::argue(&pro_with(2, 0.5, 4), &npro_with(0.3, 4));
        assert_eq!(a.measures, [1, 1, 1, 1]);
        assert_eq!(a.standing, 4);
        assert!(a.report().contains("4/4"));
    }

    #[test]
    fn empty_candidates_kills_a1_only() {
        let a = Apro::argue(&pro_with(0, 0.5, 4), &npro_with(0.3, 4));
        assert_eq!(a.measures, [0, 1, 1, 1]);
    }

    #[test]
    fn weakness_at_ceiling_kills_a3() {
        let a = Apro::argue(&pro_with(1, 0.5, 4), &npro_with(WEAKNESS_CEILING, 4));
        assert_eq!(a.measures[2], 0);
    }

    #[test]
    fn one_sided_debate_kills_a4() {
        let a = Apro::argue(&pro_with(1, 0.5, 4), &npro_with(0.2, 0));
        assert_eq!(a.measures[3], 0);
        // dhe mass nën dysheme vret A2
        let b = Apro::argue(&pro_with(1, 0.19, 4), &npro_with(0.2, 4));
        assert_eq!(b.measures[1], 0);
    }
}
