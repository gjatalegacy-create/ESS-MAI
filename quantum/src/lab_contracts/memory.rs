// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  LAB_CONTRACTS/MEMORY.RS — Kontrata e Kujtesës së Verifikimit         ║
// ║  GJATA LEGACY™ — ESS-MAI (IDENTIK në Light + Quantum + Shadow)        ║
// ║                                                                          ║
// ║  Kujtesa e verifikimit i jep sistemit besueshmëri reale: nuk ruan      ║
// ║  vetëm suksesin, por edhe PSE dështoi, çfarë e rrëzoi, çfarë mungonte.  ║
// ║                                                                          ║
// ║  • VerificationLedgerEntry → regjistri i çdo verifikimi (sukses+dështim)║
// ║  • NegativeKnowledgeRecord → dështimi si ASET (jo mbeturinë)           ║
// ║                                                                          ║
// ║  Shadow i shkruan (autoriteti suprem); të tria i lexojnë. Zero if/else.║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::lab_contracts::verdict::VerificationState;

/// Një hyrje në regjistrin e verifikimit — gjurmë e plotë e një vendimi.
#[derive(Debug, Clone, PartialEq)]
pub struct VerificationLedgerEntry {
    /// Identifikuesi i pretendimit.
    pub claim_id:          u64,
    /// Gjendja finale e verifikimit.
    pub state:             VerificationState,
    /// Hash i provës (gjurmë integriteti).
    pub evidence_hash:     String,
    /// Sa presion mbajti [0,1].
    pub pressure_score:    f32,
    /// Skor ndër-domenesh [0,1].
    pub cross_domain_score: f32,
    /// Verdikti final 0/1.
    pub final_verdict:     u8,
    /// Vula kohore (ns).
    pub recorded_at:       u64,
}

impl VerificationLedgerEntry {
    /// is_passing — a kaloi verifikimin (verdict 1 AND gjendje e verifikuar)? Zero if.
    pub fn is_passing(&self) -> bool {
        let verdict_ok = self.final_verdict == 1;
        let state_ok   = self.state.is_verified();
        verdict_ok & state_ok
    }

    /// strength — forca e regjistrimit [0,1]: presion + cross-domain + gjendje.
    /// Zero if — formula.
    pub fn strength(&self) -> f32 {
        let p = self.pressure_score.clamp(0.0, 1.0);
        let c = self.cross_domain_score.clamp(0.0, 1.0);
        let s = (self.state.rank() as f32 / 5.0).clamp(0.0, 1.0);
        (p * 0.35 + c * 0.30 + s * 0.35).clamp(0.0, 1.0)
    }
}

/// Dështimi i dokumentuar si njohuri e vlefshme (Negative Knowledge).
/// "Kjo rrugë s'punon sepse krijon këtë destabilizim."
#[derive(Debug, Clone, PartialEq)]
pub struct NegativeKnowledgeRecord {
    /// Identifikuesi i pretendimit që dështoi.
    pub claim_id:        u64,
    /// Arsyeja e dështimit.
    pub failure_reason:  String,
    /// Rruga e presionit që e rrëzoi.
    pub pressure_path:   String,
    /// A u verifikua dështimi nga Shadow (dështim i vërtetuar).
    pub verified_by_shadow: bool,
    /// Sa herë u konsultua ky dështim (frekuencë — më i konsultuar = më i vlefshëm).
    pub access_count:    u32,
    /// Vula kohore (ns).
    pub recorded_at:     u64,
}

impl NegativeKnowledgeRecord {
    /// new — krijon një regjistrim dështimi (pa konsultime ende). Zero if.
    pub fn new(
        claim_id: u64,
        failure_reason: String,
        pressure_path: String,
        verified_by_shadow: bool,
        recorded_at: u64,
    ) -> Self {
        Self {
            claim_id,
            failure_reason,
            pressure_path,
            verified_by_shadow,
            access_count: 0,
            recorded_at,
        }
    }

    /// is_valuable — a është dështim i vërtetuar dhe i konsultuar? Zero if.
    pub fn is_valuable(&self) -> bool {
        let verified  = self.verified_by_shadow;
        let consulted = self.access_count > 0;
        verified & consulted
    }
}
