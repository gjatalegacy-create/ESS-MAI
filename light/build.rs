// build.rs — Light Platform
// Kompilon kernelët C nga kernel/ (brenda projektit — zero varësi të jashtme)
//
// 3 kernelë:
//   light_buss.c        — bus 4-prioritet me CRC32 (Light↔Quantum)
//   shadow_gj_legacy.c  — autoriteti suprem, vendos 0/1 mbi vulën 500
//   buss_legacy.c       — bus i verbër, mbart vulën 500 pa e ditur
//
// Të gjithë në një librari statike: light_kernel
//
// KONTRATA E PLATFORMËS (e mbyllur):
//   • Linux / macOS      → cc me toolchain-in native; -lpthread.
//   • Windows (GNU)      → gcc mingw-w64; winpthreads e mbulon -lpthread.
//   • Windows (MSVC)     → FAIL-CLOSED me udhëzim: kerneli C përdor pthread;
//                          ABI-mix MSVC-Rust ↔ gcc-C është i ndaluar.
//                          setup_essmai instalon GNU toolchain automatikisht.

fn main() {
    #[cfg(feature = "c_kernel")]
    {
        let target_env = std::env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
        let target_os  = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

        match (target_os.as_str(), target_env.as_str()) {
            ("windows", "msvc") => {
                panic!(
                    "\n[ESS-MAI] Kerneli C kërkon toolchain GNU në Windows (pthread/winpthreads).\n\
                     ABI-mix MSVC↔gcc është i ndaluar (fail-closed).\n\
                     Zgjidhja: `rustup default stable-x86_64-pc-windows-gnu`\n\
                     (setup_essmai e bën këtë automatikisht).\n"
                );
            }
            _ => {}
        }

        cc::Build::new()
            .file("kernel/light_buss.c")
            .file("kernel/shadow_gj_legacy.c")
            .file("kernel/buss_legacy.c")
            .include("kernel")
            .flag_if_supported("-O2")
            .flag_if_supported("-std=c99")
            .flag_if_supported("-Wall")
            .flag_if_supported("-Wextra")
            .flag_if_supported("-fvisibility=hidden")
            .compile("light_kernel");

        println!("cargo:rustc-link-lib=static=light_kernel");
        // pthread: Linux/macOS native; Windows-GNU → winpthreads (mingw-w64).
        match target_env.as_str() {
            "msvc" => {} // e pamundur këtu (fail-closed më lart) — mbrojtje e dyfishtë
            _      => println!("cargo:rustc-link-lib=pthread"),
        }
        println!("cargo:rerun-if-changed=kernel/light_buss.c");
        println!("cargo:rerun-if-changed=kernel/light_buss.h");
        println!("cargo:rerun-if-changed=kernel/shadow_gj_legacy.c");
        println!("cargo:rerun-if-changed=kernel/shadow_gj_legacy.h");
        println!("cargo:rerun-if-changed=kernel/buss_legacy.c");
        println!("cargo:rerun-if-changed=kernel/buss_legacy.h");
    }

    println!("cargo:rerun-if-changed=build.rs");
}
