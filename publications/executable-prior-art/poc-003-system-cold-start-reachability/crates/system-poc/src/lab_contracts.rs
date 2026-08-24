//! New POC compatibility shell. It creates no authority.

pub use gcl_constitution::{
    system_laws_seal, CollapsePhase, GjataCollapseLaw, SupremeDirective,
};

pub mod gjata_collapse_law {
    pub use gcl_constitution::*;
}

pub mod gcl_presume {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum GclReadiness {
        Ready,
        Degraded(&'static str),
        NotReady(&'static str),
    }
}

