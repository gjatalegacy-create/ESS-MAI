# POC-006 experimental protocol

## Frozen variable

The independent condition is the presence and validity of required sealed
negative history. The dependent observables are NK readiness, permission to
continue to PRO, imported entry count, hard-block state, and final candidate
score. The focal v189 source files remain byte-identical.

## Run matrix

| Standard run | Case | Controlled input | Expected observable | Claim use |
|---|---|---|---|---|
| Run 0 — instrument validation | emit readiness, entry count, activation, score and hard-block fields | one process | all required variables visible | excludes an opaque PASS |
| Run 1 — baseline | no marker and no prior history | new bounded candidate | PRO path reached; baseline score `0.900` | behavior before required history |
| Run 2 — target | valid required `NKB1` sealed entry matching candidate | same candidate text as recorded failure | `READY`, one entry, hard-block `true`, score `0.000` | causal verified-re-entry effect |
| Run 3 — ablation | marker says history is required but blob is absent | same candidate | `DEGRADED`, PRO not activated | removing required history changes behavior |
| Run 4 — adversarial | corrupt sealed byte; replace sealed blob with raw body | same candidate | `NOT_READY`, PRO not activated in both cases | checksum/tamper/downgrade attack |
| Run 5 — counterexample | inspect disclosed response contract and detached topology | current wire/runtime | only Boolean persistence signal; no full negative receipt; no detached full Shadow-to-Quantum process chain | bounds global theory |
| Run 6 — recovery/restart | native Shadow `durability_roundtrip` component test; detached terminate/reopen is absent | disk-backed component versus detached in-process bytes | component PASS; detached E2E `NOT_EXECUTED` | prevents false persistence claim |
| Run 7 — repeatability | launch principal detached binary three independent times | identical executable/source | three equal receipts, zero failures | deterministic-repeatability gate |
| Run 8 — boundary | K- verified re-entry versus K+ source/native branch | distinct sibling domains | K- detached effect; K+ source/native evidence only | seals asymmetric POC scope |

## Cargo gate

Use a fresh target directory outside the capsule:

```powershell
$env:CARGO_TARGET_DIR = '<workspace>\_audit_build_cache\publication_v1_1_1\poc006'
rustc --version --verbose
cargo --version --verbose
cargo build --workspace --all-targets --release --locked
cargo test --workspace --all-targets --release --locked -- --test-threads=1
1..3 | ForEach-Object {
  cargo run -p poc006-living-negative-experiment --release --locked
  if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
}
```

An empty, newly created external target is the clean-build condition. The POC
must contain no `target/` directory or private workspace dependency.

## Native component controls

The already recorded v189 read-only commands cover the gate (12/12), Shadow
negative paths (19/19), WAL durability (1/1), and the four closed verdict
domain filters. They establish component behavior. They are not numerically
combined into a fictitious end-to-end test total and do not replace the
missing detached process restart.

## Pass and classification rules

The narrow claim passes only if baseline, target, ablation and both adversarial
conditions match the pre-registered values in all three launches. The POC
remains `PARTIAL_POC` at global-theory level because a Boolean persistence
flag is not an independently verifiable negative commit receipt and because
the detached experiment does not execute a real terminate/reopen across the
authoritative Shadow Vault and subsequent Quantum process.

No synthetic receipt or storage path may be added to turn those gaps green.
