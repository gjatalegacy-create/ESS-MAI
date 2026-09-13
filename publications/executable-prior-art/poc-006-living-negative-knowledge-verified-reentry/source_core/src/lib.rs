//! Detached compilation shell for the v189 Negative Knowledge re-entry gate.
//!
//! The focal `pro_nk_gate.rs`, its tokenizer, and runtime pulse are copied
//! byte-for-byte from v189. Only narrow type/module adapters are authored here.

pub mod lab_contracts {
    pub mod gcl_presume {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum GclReadiness {
            Ready,
            Degraded(&'static str),
            NotReady(&'static str),
        }

        impl GclReadiness {
            pub fn code(&self) -> &'static str {
                match self {
                    Self::Ready => "READY",
                    Self::Degraded(_) => "DEGRADED",
                    Self::NotReady(_) => "NOT_READY",
                }
            }

            pub fn is_ready(&self) -> bool {
                matches!(self, Self::Ready)
            }
        }
    }
}

pub mod hw_real {
    pub mod ffi {
        /// Byte-identical FNV-1a algorithm used by the v189 NK bridge.
        pub const fn fnv1a_64(s: &[u8]) -> u64 {
            let mut h: u64 = 14695981039346656037;
            let mut i = 0;
            while i < s.len() {
                h ^= s[i] as u64;
                h = h.wrapping_mul(1099511628211);
                i += 1;
            }
            h
        }
    }
}

pub mod pro_types {
    use std::collections::HashSet;

    /// Narrow adapter for the only PRO type consumed by `pro_nk_gate.rs`.
    #[derive(Debug, Clone)]
    pub struct FragmentVector {
        pub index: usize,
        pub stems: HashSet<String>,
        pub dense: Vec<f32>,
    }
}

pub mod tokenizer;
pub mod runtime_pulse;
pub mod pro_nk_gate;

