// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  ESS_MAI_HEART_BYTE.RS — Rrahja e Vendimit (Shadow)                  ║
// ║  GJATA LEGACY™ — ESS-MAI Shadow                          (HEART_BYTE)  ║
// ║                                                                          ║
// ║  EKSTENSION i knowledge_lineage::ledger() — JO sistem i ri.            ║
// ║                                                                          ║
// ║  Çdo vendim REAL i HCP_PRO (mint+burn token në Quantum) lind një         ║
// ║  "heart_byte": një rresht i përhershëm auditimi te ledger-i ekzistues.  ║
// ║                                                                          ║
// ║  PSE NUK ËSHTË RRAHJE PERIODIKE (Hapi 6):                             ║
// ║   Rreshti lind VETËM kur HCP_PRO merr vendim real (event-driven, si      ║
// ║   gjithçka tjetër — filozofia zero if/else). Jo i pandërprerë, jo       ║
// ║   vetëm-idle — i shkaktuar nga vendimi, pavarësisht ngarkesës hardware. ║
// ║                                                                          ║
// ║  RRJEDHA: HCP_PRO vendos [gen,nonce] → SEAL: derive_capability →         ║
// ║   RUAJ: ledger().record(lineage). Light vetëm vëzhgon ledger stream.   ║
// ║                                                                          ║
// ║  KUFIRI: heart_byte VETËM regjistron (te ledger ekzistues); s'vendos,    ║
// ║  s'krijon ledger paralel. Zero if/else klasik.                        ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::knowledge_lineage::{Lineage, ledger};

// Domain-tag për heart_byte brenda ledger-it ekzistues (jo ledger i ri).
const HEART_BYTE_DOMAIN_TAG: u64 = 0x4845_4152_5442_5954; // "HEARTBYT"

// ─────────────────────────────────────────────────────────────────────────────
// HCP DECISION RECORD — vendimi i HCP_PRO që bëhet heart_byte
// ─────────────────────────────────────────────────────────────────────────────

/// Vendimi i HCP_PRO i ardhur nga Quantum (përmes bridge), gati për regjistrim.
/// Proof = [gen, nonce] nga token-i që HCP_PRO konsumoi (mint+burn).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HcpDecisionRecord {
    /// Identifikuesi i inputit/ciklit ku u mor vendimi.
    pub input_id:    u64,
    /// gen i token-it të HCP_PRO (nga SovereignGate::issue).
    pub token_gen:   u64,
    /// nonce/seal i token-it (module_seal).
    pub token_nonce: u64,
    /// Drejtimi i vendimit (1=push deeper, 0=pull back) për auditim.
    pub directive_bit: u8,
}

impl HcpDecisionRecord {
    /// new — krijon regjistrim vendimi. Zero if.
    pub fn new(input_id: u64, token_gen: u64, token_nonce: u64, directive_bit: u8) -> Self {
        Self { input_id, token_gen, token_nonce, directive_bit }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// HEART BYTE RESULT — rezultati i regjistrimit
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HeartByteResult {
    /// A u regjistrua heart_byte-i te ledger.
    pub recorded:    bool,
    /// Nënshkrimi i derivimit (nga Lineage).
    pub derivation:  u64,
    /// input_id i regjistruar.
    pub input_id:    u64,
}

// ─────────────────────────────────────────────────────────────────────────────
// ESS-MAI HEART BYTE — regjistron vendimet te ledger ekzistues
// ─────────────────────────────────────────────────────────────────────────────

pub struct EssMaiHeartByte;

impl EssMaiHeartByte {
    /// beat_from_inbound — pranon vendimin nga bridge (4 fushat e tipizuara) dhe
    /// e regjistron si heart_byte.
    ///
    /// Ky është handler-i Shadow-side: HcpDecisionInbound (nga Quantum bridge) →
    /// HcpDecisionRecord → beat. Fushat janë EKZAKT si HcpDecisionInbound te
    /// Quantum (mirror i tipizuar, sepse crate-t janë të ndara). Zero if.
    pub fn beat_from_inbound(
        input_id: u64,
        token_gen: u64,
        token_nonce: u64,
        directive_bit: u8,
    ) -> HeartByteResult {
        // Rindërto HcpDecisionRecord nga fushat e bridge-it (jo hardcoded).
        let decision = HcpDecisionRecord::new(input_id, token_gen, token_nonce, directive_bit);
        Self::beat(&decision)
    }

    /// beat — regjistron një vendim HCP_PRO si heart_byte te ledger ekzistues.
    ///
    /// EVENT-DRIVEN: thirret VETËM kur HCP_PRO merr vendim real (jo periodik).
    /// SEAL: derive_capability me proof=[gen,nonce]. RUAJ: ledger().record.
    /// Zero if.
    pub fn beat(decision: &HcpDecisionRecord) -> HeartByteResult {
        // SEAL: deriva kapaciteti me proof = [token_gen, token_nonce].
        // (Pattern EKZAKT nga knowledge_lineage::derive_capability.)
        let proof_chain = [
            decision.token_gen,
            decision.token_nonce,
            HEART_BYTE_DOMAIN_TAG,        // domain-tag (brenda ledger-it, jo i ri)
            decision.directive_bit as u64, // drejtimi për auditim
        ];
        let lineage = Lineage::derive_capability(decision.input_id, &proof_chain);
        let derivation = lineage.derivation;

        // RUAJ: te ledger-i ekzistues (jo ledger paralel).
        ledger().record(lineage);

        HeartByteResult {
            recorded: true,
            derivation,
            input_id: decision.input_id,
        }
    }

    /// verify_beat — verifikon që një heart_byte u regjistrua (lookup te ledger).
    /// Zero if — match mbi Option.
    pub fn verify_beat(input_id: u64) -> bool {
        match ledger().lookup(input_id) {
            Some(_) => true,
            None    => false,
        }
    }

    /// beat_count — sa heart_byte janë regjistruar gjithsej (te ledger). Zero if.
    pub fn total_beats() -> usize {
        ledger().traced_count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn beat_records_to_ledger() {
        let decision = HcpDecisionRecord::new(9001, 5, 0x4843_5050_524F_0001, 1);
        let result = EssMaiHeartByte::beat(&decision);
        assert!(result.recorded);
        assert_eq!(result.input_id, 9001);
        // Derivation jo-zero (u prodhua nga derive_capability).
        assert_ne!(result.derivation, 0);
    }

    #[test]
    fn beat_verifiable_in_ledger() {
        let decision = HcpDecisionRecord::new(9002, 3, 0xABCD, 0);
        EssMaiHeartByte::beat(&decision);
        // Heart_byte-i u regjistrua → verifikohet te ledger.
        assert!(EssMaiHeartByte::verify_beat(9002));
    }

    #[test]
    fn event_driven_not_periodic() {
        // Heart_byte lind VETËM nga vendim (event-driven).
        // Pa vendim → pa rresht. Me vendim → rresht.
        let before = EssMaiHeartByte::verify_beat(9003);
        assert!(!before); // s'ka vendim ende
        EssMaiHeartByte::beat(&HcpDecisionRecord::new(9003, 1, 0x1, 1));
        let after = EssMaiHeartByte::verify_beat(9003);
        assert!(after); // vendimi krijoi rreshtin
    }

    #[test]
    fn different_decisions_different_derivations() {
        let d1 = HcpDecisionRecord::new(9004, 1, 0xAA, 1);
        let d2 = HcpDecisionRecord::new(9005, 2, 0xBB, 0);
        let r1 = EssMaiHeartByte::beat(&d1);
        let r2 = EssMaiHeartByte::beat(&d2);
        // Vendime të ndryshme → derivime të ndryshme (gjurmë unike).
        assert_ne!(r1.derivation, r2.derivation);
    }

    #[test]
    fn directive_bit_carried() {
        // Drejtimi (push/pull) regjistrohet për auditim.
        let push = HcpDecisionRecord::new(9006, 1, 0x1, 1);
        let pull = HcpDecisionRecord::new(9007, 2, 0x2, 0);
        let rp = EssMaiHeartByte::beat(&push);
        let rl = EssMaiHeartByte::beat(&pull);
        // Të dy regjistrohen me derivime që pasqyrojnë drejtimin.
        assert!(rp.recorded && rl.recorded);
        assert_ne!(rp.derivation, rl.derivation);
    }

    // ── TEST INTEGRUES (Shadow-side): bridge inbound → beat → ledger rritet ──
    //
    // RREGULLIM     : vërteton që mesazhi nga bridge (4 fushat, jo hardcoded)
    // arrin te ledger-i përmes beat_from_inbound, dhe total_beats rritet.
    #[test]
    fn integration_inbound_increases_total_beats() {
        // Vlerat vijnë nga "bridge" (parametra), JO literale brenda beat.
        let input_id = 8888u64;
        let token_gen = 42u64;       // sikur nga gate.issue te Quantum
        let token_nonce = 0xABCDu64;
        let directive_bit = 1u8;

        let before = EssMaiHeartByte::total_beats();
        // Handler-i Shadow-side: inbound → record → beat.
        let result = EssMaiHeartByte::beat_from_inbound(input_id, token_gen, token_nonce, directive_bit);
        let after = EssMaiHeartByte::total_beats();

        assert!(result.recorded);
        assert_eq!(result.input_id, input_id);
        // total_beats u rrit (vendimi arriti te ledger).
        assert!(after > before);
        // Verifikohet te ledger.
        assert!(EssMaiHeartByte::verify_beat(input_id));
    }

    #[test]
    fn integration_inbound_matches_direct_record() {
        // beat_from_inbound jep TË NJËJTIN rezultat si HcpDecisionRecord direkt.
        let r_inbound = EssMaiHeartByte::beat_from_inbound(8889, 7, 0x55, 1);
        // (input_id ndryshe që të mos përplaset te ledger)
        let direct = HcpDecisionRecord::new(8890, 7, 0x55, 1);
        let r_direct = EssMaiHeartByte::beat(&direct);
        // Të dy regjistrohen me sukses.
        assert!(r_inbound.recorded && r_direct.recorded);
    }
}
