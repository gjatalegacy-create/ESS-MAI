//! Evidence-bound state machine for the Light coordinate collapse.

use crate::lab_contracts::{CollapsePhase, GjataCollapseLaw, SupremeDirective};
use crate::lgc_algorithm::{compute_domain_sha, is_canonical_sha256};

pub const PRIMITIVE_SEAL_MASKED: u32 = (500u32 & 0x0000_FFFF) ^ 0xA5A5;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AkaState {
    I0Captured = 0,
    Traced = 1,
    Anchored = 2,
    XiYiSplit = 3,
    CoordinateCollapsed = 4,
    HandedOff = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AkaEvent {
    Stamp,
    Anchor,
    Split,
    Collapse,
    HandOff,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AkaBreach {
    pub from: AkaState,
    pub event: AkaEvent,
    pub text: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceEvidence {
    pub trace_id: String,
    pub raw_len: usize,
    pub input_sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrimitiveAnchorEvidence {
    pub trace_id: String,
    pub primitive_anchor: String,
    pub input_sha256: String,
    pub primitive_flags: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrimitiveSplitEvidence {
    pub trace_id: String,
    pub split_sha256: String,
    pub source_terms: usize,
    pub xi_len: usize,
    pub yi_len: usize,
    pub disjoint: bool,
    pub complete: bool,
    pub has_material: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoordinateCollapseEvidence {
    pub split_sha256: String,
    pub binding_sha256: String,
    pub before: u64,
    pub after: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BesaCommand {
    pub trace_id: String,
    pub split_sha256: String,
    pub law_seal: u64,
    pub system_laws_seal: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BesaAttestation {
    pub trace_id: String,
    pub split_sha256: String,
    pub request_sha256: String,
    pub hold_id: String,
    pub hold_sha256: String,
    pub selection_sha256: String,
    pub xi_negative_index_sha256: String,
    pub yi_negative_index_sha256: String,
    pub negative_index_sha256: String,
    pub positive_count: usize,
    pub negative_count: usize,
    pub max_positive: u16,
    pub max_negative: u16,
    pub law_seal: u64,
    pub system_laws_seal: u64,
    pub expires_at_ns: u64,
    pub enrichment_sha256: String,
}

impl BesaAttestation {
    pub fn shape_is_valid(&self) -> bool {
        !self.trace_id.is_empty()
            && is_canonical_sha256(&self.split_sha256)
            && is_canonical_sha256(&self.request_sha256)
            && !self.hold_id.is_empty()
            && is_canonical_sha256(&self.hold_sha256)
            && is_canonical_sha256(&self.selection_sha256)
            && is_canonical_sha256(&self.xi_negative_index_sha256)
            && is_canonical_sha256(&self.yi_negative_index_sha256)
            && is_canonical_sha256(&self.negative_index_sha256)
            && self.max_positive == shadow_contracts::SELECTION_MAX_CANDIDATES
            && self.max_negative == shadow_contracts::SELECTION_MAX_CANDIDATES
            && self.positive_count < usize::from(self.max_positive)
            && self.negative_count < usize::from(self.max_negative)
            && self.law_seal != 0
            && self.system_laws_seal != 0
            && self.expires_at_ns != 0
            && is_canonical_sha256(&self.enrichment_sha256)
    }

    pub fn attestation_sha256(&self) -> String {
        shadow_contracts::sha256_hex(&[
            b"ESSMAI/LIGHT/BESA/GCL-ATTESTATION",
            self.trace_id.as_bytes(),
            self.split_sha256.as_bytes(),
            self.request_sha256.as_bytes(),
            self.hold_id.as_bytes(),
            self.hold_sha256.as_bytes(),
            self.selection_sha256.as_bytes(),
            self.xi_negative_index_sha256.as_bytes(),
            self.yi_negative_index_sha256.as_bytes(),
            self.negative_index_sha256.as_bytes(),
            &(self.positive_count as u64).to_le_bytes(),
            &(self.negative_count as u64).to_le_bytes(),
            &self.max_positive.to_le_bytes(),
            &self.max_negative.to_le_bytes(),
            &self.law_seal.to_le_bytes(),
            &self.system_laws_seal.to_le_bytes(),
            &self.expires_at_ns.to_le_bytes(),
            self.enrichment_sha256.as_bytes(),
        ])
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandoffEvidence {
    pub binding_sha256: String,
    pub receipt_sha256: String,
    pub besa_attestation_sha256: String,
    pub besa_enrichment_sha256: String,
    pub commit_sha256: String,
    pub primitive_anchor_sha256: String,
    pub receipt_written: bool,
    pub besa_written: bool,
    pub primitive_anchor_written: bool,
    pub commit_written: bool,
    pub flushed: bool,
    pub kodunik_verified: bool,
}

impl HandoffEvidence {
    /// Gjurmë e vetme e provës së handoff-it që u pranua nga makina e
    /// gjendjeve. Nuk shton wire/storage: mban në atom hash-in e fushave
    /// ekzistuese që u verifikuan para kalimit në `HandedOff`.
    pub fn attestation_sha256(&self) -> String {
        let receipt_written = [u8::from(self.receipt_written)];
        let besa_written = [u8::from(self.besa_written)];
        let primitive_anchor_written = [u8::from(self.primitive_anchor_written)];
        let commit_written = [u8::from(self.commit_written)];
        let flushed = [u8::from(self.flushed)];
        let kodunik_verified = [u8::from(self.kodunik_verified)];
        compute_domain_sha(
            "ESSMAI/LIGHT/HANDOFF/EVIDENCE",
            &[
                self.binding_sha256.as_bytes(),
                self.receipt_sha256.as_bytes(),
                self.besa_attestation_sha256.as_bytes(),
                self.besa_enrichment_sha256.as_bytes(),
                self.commit_sha256.as_bytes(),
                self.primitive_anchor_sha256.as_bytes(),
                &receipt_written,
                &besa_written,
                &primitive_anchor_written,
                &commit_written,
                &flushed,
                &kodunik_verified,
            ],
        )
    }
}

#[derive(Debug)]
pub struct AlnurKarinaAthar {
    state: AkaState,
    trace_id: Option<String>,
    input_sha256: Option<String>,
    split_sha256: Option<String>,
    binding_sha256: Option<String>,
    besa_attestation_sha256: Option<String>,
    besa_enrichment_sha256: Option<String>,
    handoff_evidence_sha256: Option<String>,
}

impl AlnurKarinaAthar {
    pub fn commission(directive: SupremeDirective) -> Result<Self, AkaBreach> {
        if !GjataCollapseLaw::verify(&directive, CollapsePhase::Coordination) {
            return Err(AkaBreach {
                from: AkaState::I0Captured,
                event: AkaEvent::Stamp,
                text: "invalid GCL coordination directive",
            });
        }
        Ok(Self {
            state: AkaState::I0Captured,
            trace_id: None,
            input_sha256: None,
            split_sha256: None,
            binding_sha256: None,
            besa_attestation_sha256: None,
            besa_enrichment_sha256: None,
            handoff_evidence_sha256: None,
        })
    }

    fn transition(&mut self, event: AkaEvent) -> Result<(), AkaBreach> {
        let next = match (self.state, event) {
            (AkaState::I0Captured, AkaEvent::Stamp) => AkaState::Traced,
            (AkaState::Traced, AkaEvent::Anchor) => AkaState::Anchored,
            (AkaState::Anchored, AkaEvent::Split) => AkaState::XiYiSplit,
            (AkaState::XiYiSplit, AkaEvent::Collapse) => AkaState::CoordinateCollapsed,
            (AkaState::CoordinateCollapsed, AkaEvent::HandOff) => AkaState::HandedOff,
            (from, event) => {
                return Err(AkaBreach {
                    from,
                    event,
                    text: "illegal Light order: i0->Trace->PA->XiYi->Collapse->HandOff",
                })
            }
        };
        self.state = next;
        Ok(())
    }

    pub fn record_trace(&mut self, evidence: &TraceEvidence) -> Result<(), AkaBreach> {
        if evidence.trace_id.is_empty()
            || evidence.raw_len == 0
            || !is_canonical_sha256(&evidence.input_sha256)
        {
            return Err(self.evidence_breach(AkaEvent::Stamp, "invalid trace evidence"));
        }
        self.transition(AkaEvent::Stamp)?;
        self.trace_id = Some(evidence.trace_id.clone());
        self.input_sha256 = Some(evidence.input_sha256.clone());
        Ok(())
    }

    pub fn record_anchor(&mut self, evidence: &PrimitiveAnchorEvidence) -> Result<(), AkaBreach> {
        let pa_valid = evidence.primitive_anchor.len() == 16
            && evidence
                .primitive_anchor
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b));
        if self.trace_id.as_deref() != Some(evidence.trace_id.as_str())
            || self.input_sha256.as_deref() != Some(evidence.input_sha256.as_str())
            || evidence.primitive_flags != PRIMITIVE_SEAL_MASKED
            || !pa_valid
        {
            return Err(self.evidence_breach(AkaEvent::Anchor, "invalid primitive-anchor evidence"));
        }
        self.transition(AkaEvent::Anchor)
    }

    pub fn record_split(&mut self, evidence: &PrimitiveSplitEvidence) -> Result<(), AkaBreach> {
        if self.trace_id.as_deref() != Some(evidence.trace_id.as_str())
            || !is_canonical_sha256(&evidence.split_sha256)
            || evidence.source_terms != evidence.xi_len.saturating_add(evidence.yi_len)
            || !evidence.disjoint
            || !evidence.complete
            || !evidence.has_material
        {
            return Err(self.evidence_breach(AkaEvent::Split, "invalid primitive-split evidence"));
        }
        self.transition(AkaEvent::Split)?;
        self.split_sha256 = Some(evidence.split_sha256.clone());
        Ok(())
    }

    pub fn command_besa(&self, directive: &SupremeDirective) -> Result<BesaCommand, AkaBreach> {
        match (
            self.state,
            GjataCollapseLaw::verify(directive, CollapsePhase::Coordination),
            self.trace_id.as_ref(),
            self.split_sha256.as_ref(),
        ) {
            (AkaState::XiYiSplit, true, Some(trace_id), Some(split_sha256)) => Ok(BesaCommand {
                trace_id: trace_id.clone(),
                split_sha256: split_sha256.clone(),
                law_seal: directive.seal,
                system_laws_seal: crate::lab_contracts::system_laws_seal(),
            }),
            (state, _, _, _) => Err(AkaBreach {
                from: state,
                event: AkaEvent::Collapse,
                text: "AKA cannot command Besa outside XiYiSplit under GCL Coordination",
            }),
        }
    }

    pub fn record_collapse(
        &mut self,
        evidence: &CoordinateCollapseEvidence,
        besa: &BesaAttestation,
    ) -> Result<(), AkaBreach> {
        let evidence_valid = self.split_sha256.as_deref() == Some(evidence.split_sha256.as_str())
            && besa.shape_is_valid()
            && self.trace_id.as_deref() == Some(besa.trace_id.as_str())
            && besa.split_sha256 == evidence.split_sha256
            && besa.law_seal
                == crate::lab_contracts::gjata_collapse_law::law_seal()
            && besa.system_laws_seal == crate::lab_contracts::system_laws_seal()
            && is_canonical_sha256(&evidence.binding_sha256)
            && evidence.after <= evidence.before;
        match evidence_valid {
            true => {}
            false => {
                return Err(
                    self.evidence_breach(AkaEvent::Collapse, "invalid LAW0/Besa collapse evidence")
                )
            }
        }
        self.transition(AkaEvent::Collapse)?;
        self.binding_sha256 = Some(evidence.binding_sha256.clone());
        self.besa_attestation_sha256 = Some(besa.attestation_sha256());
        self.besa_enrichment_sha256 = Some(besa.enrichment_sha256.clone());
        Ok(())
    }

    pub fn hand_off(&mut self, evidence: &HandoffEvidence) -> Result<(), AkaBreach> {
        if self.binding_sha256.as_deref() != Some(evidence.binding_sha256.as_str())
            || !is_canonical_sha256(&evidence.receipt_sha256)
            || self.besa_attestation_sha256.as_deref()
                != Some(evidence.besa_attestation_sha256.as_str())
            || self.besa_enrichment_sha256.as_deref()
                != Some(evidence.besa_enrichment_sha256.as_str())
            || !is_canonical_sha256(&evidence.commit_sha256)
            || !is_canonical_sha256(&evidence.primitive_anchor_sha256)
            || !evidence.receipt_written
            || !evidence.besa_written
            || !evidence.primitive_anchor_written
            || !evidence.commit_written
            || !evidence.flushed
            || !evidence.kodunik_verified
        {
            return Err(self.evidence_breach(AkaEvent::HandOff, "invalid durable handoff evidence"));
        }
        let handoff_evidence_sha256 = evidence.attestation_sha256();
        self.transition(AkaEvent::HandOff)?;
        self.handoff_evidence_sha256 = Some(handoff_evidence_sha256);
        Ok(())
    }

    fn evidence_breach(&self, event: AkaEvent, text: &'static str) -> AkaBreach {
        AkaBreach {
            from: self.state,
            event,
            text,
        }
    }

    pub fn state(&self) -> AkaState {
        self.state
    }

    pub fn is_collapsed(&self) -> bool {
        matches!(
            self.state,
            AkaState::CoordinateCollapsed | AkaState::HandedOff
        )
    }

    pub fn besa_is_bound(&self, besa: &BesaAttestation) -> bool {
        let attestation_sha256 = besa.attestation_sha256();
        self.is_collapsed()
            && self.besa_attestation_sha256.as_deref()
                == Some(attestation_sha256.as_str())
            && self.besa_enrichment_sha256.as_deref()
                == Some(besa.enrichment_sha256.as_str())
    }

    pub fn is_fulfilled(&self) -> bool {
        self.state == AkaState::HandedOff
    }

    pub fn handoff_evidence_sha256(&self) -> Option<&str> {
        self.handoff_evidence_sha256.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lab_contracts::{CollapsePhase as Phase, GjataCollapseLaw as Law};

    const SHA_A: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    const SHA_B: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
    const SHA_C: &str = "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc";

    fn valid_besa() -> BesaAttestation {
        BesaAttestation {
            trace_id: "T1_12345678".to_string(),
            split_sha256: SHA_B.to_string(),
            request_sha256: SHA_A.to_string(),
            hold_id: "besa-test-hold".to_string(),
            hold_sha256: SHA_A.to_string(),
            selection_sha256: SHA_B.to_string(),
            xi_negative_index_sha256: SHA_A.to_string(),
            yi_negative_index_sha256: SHA_B.to_string(),
            negative_index_sha256: SHA_C.to_string(),
            positive_count: 1,
            negative_count: 1,
            max_positive: shadow_contracts::SELECTION_MAX_CANDIDATES,
            max_negative: shadow_contracts::SELECTION_MAX_CANDIDATES,
            law_seal: crate::lab_contracts::gjata_collapse_law::law_seal(),
            system_laws_seal: crate::lab_contracts::system_laws_seal(),
            expires_at_ns: u64::MAX,
            enrichment_sha256: SHA_C.to_string(),
        }
    }

    fn lawful_to_collapse() -> AlnurKarinaAthar {
        let mut muscle = AlnurKarinaAthar::commission(Law::issue(Phase::Coordination))
            .expect("coordination directive must commission Light muscle");
        muscle
            .record_trace(&TraceEvidence {
                trace_id: "T1_12345678".to_string(),
                raw_len: 4,
                input_sha256: SHA_A.to_string(),
            })
            .expect("trace evidence");
        muscle
            .record_anchor(&PrimitiveAnchorEvidence {
                trace_id: "T1_12345678".to_string(),
                primitive_anchor: "1234567890abcdef".to_string(),
                input_sha256: SHA_A.to_string(),
                primitive_flags: PRIMITIVE_SEAL_MASKED,
            })
            .expect("anchor evidence");
        muscle
            .record_split(&PrimitiveSplitEvidence {
                trace_id: "T1_12345678".to_string(),
                split_sha256: SHA_B.to_string(),
                source_terms: 2,
                xi_len: 1,
                yi_len: 1,
                disjoint: true,
                complete: true,
                has_material: true,
            })
            .expect("split evidence");
        muscle
            .record_collapse(
                &CoordinateCollapseEvidence {
                    split_sha256: SHA_B.to_string(),
                    binding_sha256: SHA_C.to_string(),
                    before: 4,
                    after: 1,
                },
                &valid_besa(),
            )
            .expect("collapse evidence");
        muscle
    }

    #[test]
    fn evidence_bound_full_path_reaches_handoff() {
        let mut muscle = lawful_to_collapse();
        let evidence = HandoffEvidence {
            binding_sha256: SHA_C.to_string(),
            receipt_sha256: SHA_A.to_string(),
            besa_attestation_sha256: valid_besa().attestation_sha256(),
            besa_enrichment_sha256: SHA_C.to_string(),
            commit_sha256: SHA_B.to_string(),
            primitive_anchor_sha256: SHA_C.to_string(),
            receipt_written: true,
            besa_written: true,
            primitive_anchor_written: true,
            commit_written: true,
            flushed: true,
            kodunik_verified: true,
        };
        let expected_attestation = evidence.attestation_sha256();
        muscle
            .hand_off(&evidence)
            .expect("durable handoff evidence");
        assert!(muscle.is_fulfilled());
        assert_eq!(muscle.state(), AkaState::HandedOff);
        assert_eq!(
            muscle.handoff_evidence_sha256(),
            Some(expected_attestation.as_str())
        );
    }

    #[test]
    fn skipping_anchor_is_named_breach() {
        let mut muscle = AlnurKarinaAthar::commission(Law::issue(Phase::Coordination))
            .expect("coordination directive");
        muscle
            .record_trace(&TraceEvidence {
                trace_id: "T1_12345678".to_string(),
                raw_len: 4,
                input_sha256: SHA_A.to_string(),
            })
            .expect("trace evidence");
        let breach = muscle
            .record_split(&PrimitiveSplitEvidence {
                trace_id: "T1_12345678".to_string(),
                split_sha256: SHA_B.to_string(),
                source_terms: 1,
                xi_len: 1,
                yi_len: 0,
                disjoint: true,
                complete: true,
                has_material: true,
            })
            .expect_err("split before PA must fail");
        assert_eq!(breach.from, AkaState::Traced);
        assert_eq!(muscle.state(), AkaState::Traced);
    }

    #[test]
    fn false_durable_evidence_cannot_handoff() {
        let mut muscle = lawful_to_collapse();
        let breach = muscle
            .hand_off(&HandoffEvidence {
                binding_sha256: SHA_C.to_string(),
                receipt_sha256: SHA_A.to_string(),
                besa_attestation_sha256: valid_besa().attestation_sha256(),
                besa_enrichment_sha256: SHA_C.to_string(),
                commit_sha256: SHA_B.to_string(),
                primitive_anchor_sha256: SHA_C.to_string(),
                receipt_written: true,
                besa_written: true,
                primitive_anchor_written: true,
                commit_written: false,
                flushed: true,
                kodunik_verified: true,
            })
            .expect_err("missing commit must fail closed");
        assert_eq!(breach.from, AkaState::CoordinateCollapsed);
        assert!(!muscle.is_fulfilled());
        assert_eq!(muscle.handoff_evidence_sha256(), None);
    }

    #[test]
    fn wrong_phase_directive_is_refused() {
        assert!(AlnurKarinaAthar::commission(Law::issue(Phase::Reasoning)).is_err());
    }

    #[test]
    fn second_trace_event_is_refused_without_state_change() {
        let mut muscle = AlnurKarinaAthar::commission(Law::issue(Phase::Coordination))
            .expect("coordination directive");
        let evidence = TraceEvidence {
            trace_id: "T1_12345678".to_string(),
            raw_len: 4,
            input_sha256: SHA_A.to_string(),
        };
        muscle.record_trace(&evidence).expect("first trace");
        assert!(muscle.record_trace(&evidence).is_err());
        assert_eq!(muscle.state(), AkaState::Traced);
    }
}
