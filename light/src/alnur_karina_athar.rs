//! Evidence-bound state machine for the Light coordinate collapse.

use crate::lab_contracts::{CollapsePhase, GjataCollapseLaw, SupremeDirective};
use crate::lgc_algorithm::is_canonical_sha256;

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
pub struct HandoffEvidence {
    pub binding_sha256: String,
    pub receipt_sha256: String,
    pub receipt_written: bool,
    pub primitive_anchor_written: bool,
    pub commit_written: bool,
    pub flushed: bool,
    pub kodunik_verified: bool,
}

#[derive(Debug)]
pub struct AlnurKarinaAthar {
    state: AkaState,
    trace_id: Option<String>,
    input_sha256: Option<String>,
    split_sha256: Option<String>,
    binding_sha256: Option<String>,
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

    pub fn record_anchor(
        &mut self,
        evidence: &PrimitiveAnchorEvidence,
    ) -> Result<(), AkaBreach> {
        let pa_valid = evidence.primitive_anchor.len() == 16
            && evidence.primitive_anchor.bytes().all(|b| {
                b.is_ascii_digit() || (b'a'..=b'f').contains(&b)
            });
        if self.trace_id.as_deref() != Some(evidence.trace_id.as_str())
            || self.input_sha256.as_deref() != Some(evidence.input_sha256.as_str())
            || evidence.primitive_flags != PRIMITIVE_SEAL_MASKED
            || !pa_valid
        {
            return Err(self.evidence_breach(AkaEvent::Anchor, "invalid primitive-anchor evidence"));
        }
        self.transition(AkaEvent::Anchor)
    }

    pub fn record_split(
        &mut self,
        evidence: &PrimitiveSplitEvidence,
    ) -> Result<(), AkaBreach> {
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

    pub fn record_collapse(
        &mut self,
        evidence: &CoordinateCollapseEvidence,
    ) -> Result<(), AkaBreach> {
        if self.split_sha256.as_deref() != Some(evidence.split_sha256.as_str())
            || !is_canonical_sha256(&evidence.binding_sha256)
            || evidence.after > evidence.before
        {
            return Err(self.evidence_breach(AkaEvent::Collapse, "invalid LAW0 collapse evidence"));
        }
        self.transition(AkaEvent::Collapse)?;
        self.binding_sha256 = Some(evidence.binding_sha256.clone());
        Ok(())
    }

    pub fn hand_off(&mut self, evidence: &HandoffEvidence) -> Result<(), AkaBreach> {
        if self.binding_sha256.as_deref() != Some(evidence.binding_sha256.as_str())
            || !is_canonical_sha256(&evidence.receipt_sha256)
            || !evidence.receipt_written
            || !evidence.primitive_anchor_written
            || !evidence.commit_written
            || !evidence.flushed
            || !evidence.kodunik_verified
        {
            return Err(self.evidence_breach(AkaEvent::HandOff, "invalid durable handoff evidence"));
        }
        self.transition(AkaEvent::HandOff)
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
        matches!(self.state, AkaState::CoordinateCollapsed | AkaState::HandedOff)
    }

    pub fn is_fulfilled(&self) -> bool {
        self.state == AkaState::HandedOff
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::lab_contracts::{CollapsePhase as Phase, GjataCollapseLaw as Law};

    const SHA_A: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    const SHA_B: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
    const SHA_C: &str = "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc";

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
            .record_collapse(&CoordinateCollapseEvidence {
                split_sha256: SHA_B.to_string(),
                binding_sha256: SHA_C.to_string(),
                before: 4,
                after: 1,
            })
            .expect("collapse evidence");
        muscle
    }

    #[test]
    fn evidence_bound_full_path_reaches_handoff() {
        let mut muscle = lawful_to_collapse();
        muscle
            .hand_off(&HandoffEvidence {
                binding_sha256: SHA_C.to_string(),
                receipt_sha256: SHA_A.to_string(),
                receipt_written: true,
                primitive_anchor_written: true,
                commit_written: true,
                flushed: true,
                kodunik_verified: true,
            })
            .expect("durable handoff evidence");
        assert!(muscle.is_fulfilled());
        assert_eq!(muscle.state(), AkaState::HandedOff);
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
                receipt_written: true,
                primitive_anchor_written: true,
                commit_written: false,
                flushed: true,
                kodunik_verified: true,
            })
            .expect_err("missing commit must fail closed");
        assert_eq!(breach.from, AkaState::CoordinateCollapsed);
        assert!(!muscle.is_fulfilled());
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
