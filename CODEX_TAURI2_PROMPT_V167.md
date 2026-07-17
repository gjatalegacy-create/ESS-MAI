# Codex/Tauri 2 verification prompt — ESS-MAI v1.6.7

Open the repository root and audit the Nura Legacy Tauri 2 implementation without redesigning the ESS-MAI architecture.

Authoritative constraints:

1. UI/Tauri may invoke only `light-platform`.
2. Do not add direct UI calls to `quantum-platform` or `shadow_platform`.
3. Do not alter LGC tokens, ForgeToken, CapHandle, Living Trust, verification receipts, GCL formulas, PD, Spine 9, or Shadow verdict authority.
4. Project Workspace must remain `Light --project-route-once`.
5. Full scientific processing must remain the explicit legacy route `Light --project-route-legacy-once`.
6. TRL 0–3 is evidence inside GCL; TRL4 is born only in Shadow after multi-step verification and the sovereign pair.
7. The old UI may accept upload and project text and display emotional/runtime state only.
8. Do not reintroduce the label “Quantomic”; use exact modules: Light Coordination, Quantum Reasoning Pipeline, Shadow Multi-Step Verification, Project Workspace, GCL Scientific Project.

Perform these checks:

```powershell
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace --all-targets --no-run
cargo clippy --workspace --all-targets -- -D warnings
cd ui
cargo fmt -- --check
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo build --release
```

Inspect `ui/src/main.rs` for process boundaries, input limits, temporary-file cleanup, response decoding, and token isolation. Make only evidence-backed corrections. Stop at ambiguous runtime boundaries and report them instead of guessing.
