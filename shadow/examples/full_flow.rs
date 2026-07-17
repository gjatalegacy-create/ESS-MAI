//! v1.5.9 — mediated full-flow entry notice.
//!
//! The historical example instantiated `Shadow` as a library. That route is
//! intentionally impossible in v1.5.9. The real flow is now:
//!
//! Light → Quantum → checksummed `shadow_contracts` frame → Shadow main.rs
//! → internal Shadow core → sealed receipt → Quantum → Light/Nura.
//!
//! Build both executables first, then run Quantum with the shared handoff dir:
//! `cargo build --workspace`
//! `cargo run -p quantum-platform`

fn main() {
    let handoff = std::env::var("ESSMAI_HANDOFF_DIR")
        .unwrap_or_else(|_| "<MUNGON>".to_string());
    let shadow = std::env::var("ESSMAI_SHADOW_BIN")
        .unwrap_or_else(|_| "<sibling target/debug/shadow_platform>".to_string());

    println!("ESS-MAI v1.5.9 — mediated full flow");
    println!("ESSMAI_HANDOFF_DIR={handoff}");
    println!("ESSMAI_SHADOW_BIN={shadow}");
    println!("Nuk ekziston më rrugë direct-core nga example/Quantum.");
    println!("Ekzekuto: cargo build --workspace");
    println!("Pastaj:   cargo run -p quantum-platform");
}
