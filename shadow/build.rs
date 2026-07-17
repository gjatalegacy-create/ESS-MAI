// build.rs — Shadow Platform
// Gjata Legacy™ | Arkitekt: Bledar Gjata
//
// KONTRATA E PLATFORMËS (e mbyllur):
//   • Linux / macOS   → cc native (-std=c11, -O3); pthread native.
//   • Windows (GNU)   → gcc mingw-w64; winpthreads mbulon pthread.
//   • Windows (MSVC)  → FAIL-CLOSED me udhëzim (kerneli C = pthread; ABI-mix i ndaluar).
//   • feature pure_rust → anashkalo C (pasqyra Rust për teste).


fn enforce_main_mediation() {
    let cargo_toml = std::fs::read_to_string("Cargo.toml")
        .unwrap_or_else(|e| panic!("[SHADOW] Cargo.toml nuk u lexua: {e}"));
    let main_rs = std::fs::read_to_string("src/main.rs")
        .unwrap_or_else(|e| panic!("[SHADOW] src/main.rs nuk u lexua: {e}"));
    assert!(cargo_toml.contains("autolib = false"),
        "[SHADOW] autolib=false është kusht sovran: lib.rs s'duhet të jetë target");
    assert!(!cargo_toml.contains("[lib]"),
        "[SHADOW] target [lib] është i ndaluar; main.rs është porta e vetme");
    assert!(main_rs.contains("include!(\"lib.rs\");"),
        "[SHADOW] main.rs duhet të përfshijë kushtetutën lib.rs brenda binarit");
    assert!(main_rs.contains("mod process_bridge;"),
        "[SHADOW] process_bridge duhet të ndërmjetësohet nga main.rs");
}

fn compile_kernel(_target_env: &str) {
    cc::Build::new()
        .file("kernel/shadow_buss.c")
        .file("kernel/buss_legacy.c")
        .file("kernel/shadow_gj_legacy.c")
        // shadow_gj_legacy_kernel.c është dublikatë dhe mbetet jashtë build-it.
        .flag_if_supported("-std=c11")
        .flag_if_supported("-Wall")
        .flag_if_supported("-Wextra")
        .flag_if_supported("-O3")
        .include("kernel")
        .compile("shadow_kernel");
    // MSVC është refuzuar para kësaj pike; çdo backend real përdor pthread.
    println!("cargo:rustc-link-lib=pthread");
}

fn skip_kernel(_target_env: &str) {
    println!("cargo:warning=feature pure_rust aktiv — kerneli C NUK kompilohet (FFI i çaktivizuar)");
}

fn main() {
    enforce_main_mediation();

    let runtime_mode = std::env::var("CARGO_FEATURE_RUNTIME_MODE").is_ok();
    let pure_rust = std::env::var("CARGO_FEATURE_PURE_RUST").is_ok();
    assert!(!(runtime_mode & pure_rust),
        "\n[ESS-MAI/NDALIM SOVRAN] `pure_rust` + `runtime_mode` = I NDALUAR.\n\
         Test/dev: `cargo test --no-default-features --features pure_rust`\n\
         Prodhim: build default me kernelin C.\n");

    let target_env = std::env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let msvc_runtime = (target_os == "windows") & (target_env == "msvc") & !pure_rust;
    assert!(!msvc_runtime,
        "\n[ESS-MAI] Kerneli C i Shadow kërkon Windows GNU (pthread).\n\
         Përdor `rustup default stable-x86_64-pc-windows-gnu`,\n\
         ose `--no-default-features --features pure_rust` vetëm për teste.\n");

    let actions: [fn(&str); 2] = [compile_kernel, skip_kernel];
    actions[pure_rust as usize](&target_env);

    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=kernel/shadow_buss.c");
    println!("cargo:rerun-if-changed=kernel/shadow_buss.h");
    println!("cargo:rerun-if-changed=kernel/buss_legacy.c");
    println!("cargo:rerun-if-changed=kernel/buss_legacy.h");
    println!("cargo:rerun-if-changed=kernel/shadow_gj_legacy.c");
    println!("cargo:rerun-if-changed=kernel/shadow_gj_legacy.h");
}
