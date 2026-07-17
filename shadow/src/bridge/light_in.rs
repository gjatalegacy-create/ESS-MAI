// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  BRIDGE/LIGHT_IN.RS — URA Light → Shadow (FAZA 3)                     ║
// ║  Gjata Legacy™ | Bledar Gjata                                        ║
// ║                                                                          ║
// ║  Pasqyron kontratën e transportit të Light-it (`LightShadowEnvelope`)  ║
// ║  dhe e adapton në `LightEnvelope` të brendshëm — me ZHVENDOSJE (Ligji 0).║
// ║  Light është transport i VERBËR: vulën 500 e mbart, s'e interpreton.   ║
// ╚══════════════════════════════════════════════════════════════════════════╝

use crate::types::LightEnvelope;

/// Pasqyrë e `LightShadowEnvelope` (output-i i urës Light → Shadow).
/// Shadow s'varet nga crate-i i Light-it; kjo është kontrata si tip kufitar.
#[derive(Debug, Clone)]
pub struct LightInbound {
    pub session_id:      String,
    pub territory:       String,
    /// Vula 500 e mbartur verbërisht (0xA451 e maskuar ose 0x0000 e hequr).
    pub primitive_flags: u32,
    /// E mbartur nga Quantum; në Shadow jeton te `PassPackage`, jo te envelope.
    pub epistemic_mass:  f32,
    pub proof_chain:     Vec<u64>,
    pub payload:         Vec<u8>,
}

impl LightInbound {
    /// Adapton → `LightEnvelope` me ZHVENDOSJE (asnjë klon).
    /// `epistemic_mass` nuk hyn këtu (rron te `PassPackage` nga origjina Quantum).
    pub fn into_envelope(self) -> LightEnvelope {
        LightEnvelope {
            session_id:      self.session_id,
            territory:       self.territory,
            primitive_flags: self.primitive_flags,
            proof_chain:     self.proof_chain,
            payload:         self.payload,
            device_watch:    None,
        }
    }

    /// RUNTIME HARDENING (      , fail-closed): ura Light→Shadow s'pranon transport
    /// të gjymtuar. session+territory duhet jo-bosh, vula 500 duhet të kalojë,
    /// proof_chain jo-bosh (gjurmueshmëri lineage). Zero if — match i ndërthurur.
    pub fn is_valid(&self) -> bool {
        match self.session_id.is_empty() {
            true  => false,
            false => match self.territory.is_empty() {
                true  => false,
                false => match self.seal_ok() {
                    false => false,
                    true  => self.proof_chain.is_empty() == false,
                },
            },
        }
    }

    /// reason_invalid — arsyeja specifike e refuzimit (reason code). Zero if — match.
    pub fn reason_invalid(&self) -> &'static str {
        match self.session_id.is_empty() {
            true  => "light_empty_session_id",
            false => match self.territory.is_empty() {
                true  => "light_empty_territory",
                false => match self.seal_ok() {
                    false => "light_seal_500_failed",
                    true  => match self.proof_chain.is_empty() {
                        true  => "light_empty_proof_chain",
                        false => "valid",
                    },
                },
            },
        }
    }

    /// Diagnostikë (NUK ndryshon transportin): a kalon vula 500?
    /// `(flags & 0xFFFF) ^ 0xA5A5 == 500` — identike me kernelin C dhe Light-in.
    pub fn seal_ok(&self) -> bool {
        ((self.primitive_flags & 0xFFFF) ^ 0xA5A5) == 500
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into_envelope_moves_fields() {
        let l = LightInbound {
            session_id:      "sess-1".to_string(),
            territory:       "INDUSTRIAL".to_string(),
            primitive_flags: 0xA451,
            epistemic_mass:  0.9,
            proof_chain:     vec![1, 2, 3],
            payload:         b"data".to_vec(),
        };
        assert!(l.seal_ok()); // 0xA451 → vula e plotë
        // RUNTIME HARDENING (      ): LightInbound i plotë → valid.
        assert!(l.is_valid());
        assert_eq!(l.reason_invalid(), "valid");
        let env = l.into_envelope();
        assert_eq!(env.session_id, "sess-1");
        assert_eq!(env.territory, "INDUSTRIAL");
        assert_eq!(env.primitive_flags, 0xA451);
        assert_eq!(env.proof_chain, vec![1, 2, 3]);
        assert_eq!(env.payload, b"data");
    }

    #[test]
    fn light_inbound_fail_closed_rejects_gjymtuar() {
        // FAIL-CLOSED (      ): session bosh → i pavlefshëm.
        let no_sess = LightInbound {
            session_id: String::new(), territory: "T".to_string(),
            primitive_flags: 0xA451, epistemic_mass: 0.5,
            proof_chain: vec![1], payload: vec![],
        };
        assert!(!no_sess.is_valid());
        assert_eq!(no_sess.reason_invalid(), "light_empty_session_id");

        // Vula e prishur → i pavlefshëm edhe me fusha të tjera OK.
        let bad_seal = LightInbound {
            session_id: "s".to_string(), territory: "T".to_string(),
            primitive_flags: 0x0000, epistemic_mass: 0.5,
            proof_chain: vec![1], payload: vec![],
        };
        assert!(!bad_seal.is_valid());
        assert_eq!(bad_seal.reason_invalid(), "light_seal_500_failed");

        // proof_chain bosh → i pavlefshëm (s'ka gjurmueshmëri lineage).
        let no_chain = LightInbound {
            session_id: "s".to_string(), territory: "T".to_string(),
            primitive_flags: 0xA451, epistemic_mass: 0.5,
            proof_chain: vec![], payload: vec![],
        };
        assert!(!no_chain.is_valid());
        assert_eq!(no_chain.reason_invalid(), "light_empty_proof_chain");
    }

    #[test]
    fn stripped_seal_fails_check() {
        let l = LightInbound {
            session_id:      "s".to_string(),
            territory:       "T".to_string(),
            primitive_flags: 0x0000,
            epistemic_mass:  0.5,
            proof_chain:     vec![9],
            payload:         vec![],
        };
        assert!(!l.seal_ok()); // 0x0000 → e hequr
    }
}
