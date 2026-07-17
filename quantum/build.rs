// build.rs — Quantum Platform me hardware REAL
// Kompilon kernelët C të hardware-it kur feature "hw_kernel" është aktiv.
// Pa feature: pure-Rust fallback, pa C.
//
// KONTRATA E PLATFORMËS (e mbyllur):
//   • Linux           → sysinfo/procfs/sysfs (rruga origjinale, e paprekur).
//   • Windows (GNU)   → gcc mingw-w64; hw_resource.c përdor API native Windows
//                       (GlobalMemoryStatusEx/GetSystemTimes/GetSystemInfo/
//                       GetSystemPowerStatus); thermal → formula matematike
//                       (sensori sysfs mungon → PRANOHET, s'dështon kurrë).
//   • Windows (MSVC)  → FAIL-CLOSED me udhëzim (një ABI e vetme me gcc).

fn main() {
    #[cfg(feature = "hw_kernel")]
    {
        let target_env = std::env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
        let target_os  = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
        match (target_os.as_str(), target_env.as_str()) {
            ("windows", "msvc") => {
                panic!(
                    "\n[ESS-MAI] hw_kernel kërkon toolchain GNU në Windows.\n\
                     Zgjidhja: `rustup default stable-x86_64-pc-windows-gnu`\n\
                     (setup_essmai e bën këtë automatikisht).\n"
                );
            }
            _ => {}
        }

        cc::Build::new()
            .file("src/hw_real/c_kernel/hw_resource.c")
            .file("src/hw_real/c_kernel/hw_thermal.c")
            .file("src/hw_real/c_kernel/hw_colddown.c")
            .include("src/hw_real/c_kernel")
            .flag_if_supported("-O2")
            .flag_if_supported("-std=c99")
            .flag_if_supported("-Wall")
            .flag_if_supported("-fvisibility=hidden")
            .compile("hw_kernel");

        println!("cargo:rustc-link-lib=static=hw_kernel");
        println!("cargo:rerun-if-changed=src/hw_real/c_kernel/hw_resource.c");
        println!("cargo:rerun-if-changed=src/hw_real/c_kernel/hw_thermal.c");
        println!("cargo:rerun-if-changed=src/hw_real/c_kernel/hw_colddown.c");
    }
    println!("cargo:rerun-if-changed=build.rs");
}
