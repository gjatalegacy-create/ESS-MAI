# POC-005 experimental protocol

## Frozen variable

The independent condition is whether a presented handle still has the exact
live Rust-owned slot authority created for its generation. The dependent
observable is the integer result at the FFI boundary. No v189 source file is
modified.

## Run matrix

| Standard run | Case | Controlled input | Expected observable | Claim use |
|---|---|---|---|---|
| Run 0 — instrument validation | emit all codes and concurrency counts | one process, exact copied source | every required field is visible | excludes opaque PASS |
| Run 1 — baseline | fresh generation + matching nonce + non-empty payload | valid handle | `-8` | shows the call crossed local authorization but was held before write |
| Run 2 — target | bit-identical copied handle after first use | same generation, nonce and payload | `-1` | tests representation copy versus authority copy |
| Run 3 — ablation | remove live authority by burning the slot, retain handle bits | third use | `-1` | causal contrast with fresh baseline |
| Run 4 — adversarial | forged nonce, unknown generation, empty payload, eight-copy race | one variable changed per case | `-3`, `-2`, `-5`, then one `-8` + seven `-1` | attacks identity, liveness and concurrency boundaries |
| Run 5 — counterexample | issue two fresh generations for same module + same payload | same semantic action, different generation | `-8`, `-8` | disproves stronger semantic-idempotency interpretation |
| Run 6 — restart/recovery | not applicable to narrow process-local claim | no durable slot format exists | `NOT_EXECUTED` | durable consumption remains not proven |
| Run 7 — repeatability | launch principal binary three independent times | identical executable/source | three equal receipts, zero failures | deterministic-repeatability gate |
| Run 8 — boundary | compare same-generation replay with fresh reissuance and Vault result | local gate versus semantic/global sink | local PASS; global write absent | seals materialization boundary |

## Cargo gate

Use a fresh target directory outside the capsule:

```powershell
$env:CARGO_TARGET_DIR = '<workspace>\_audit_build_cache\publication_v1_1_1\poc005'
rustc --version --verbose
cargo --version --verbose
cargo build --workspace --all-targets --release --locked
cargo test --workspace --all-targets --release --locked -- --test-threads=1
1..3 | ForEach-Object {
  cargo run -p poc005-ffi-law0-experiment --release --locked
  if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
}
```

An empty, newly created external target is the clean-build condition; no
compiler cache is stored in the POC.

## Pass and classification rules

The local claim passes only if the full expected code vector and the 1/7
concurrency split recur in all three processes. Finding two accepted fresh
generations does not make the experiment fail: it is the pre-registered
counterexample that forces the global theory to remain not materialised and
the complete POC classification to remain `PARTIAL_POC`.

The experiment must not be repaired by adding semantic idempotency or a Vault
sink to this sealed capsule.
