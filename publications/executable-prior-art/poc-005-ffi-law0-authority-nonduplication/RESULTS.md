# Experimental results

## Outcome summary

| Layer | Test | Observed result | Scientific verdict |
|---|---|---:|---|
| native v189 | focal `sovereign_ffi_gate` suite | 8/8 pass | source behavior reproduced |
| detached Cargo | strict release `--all-targets` build | exit 0 after adapter-only closure | build closure valid; initial failure preserved |
| detached Cargo | strict release tests | 15/15 pass | 14 copied-source unit tests + 1 experiment test |
| detached Cargo | independent runtime launches | 3/3 pass | deterministic receipt reproduced |
| runtime | forged nonce | `-3` | PASS |
| runtime | first valid non-empty call | `-8` | AUTHORITY PASS / WRITE HOLD |
| runtime | copied handle replay | `-1` | PASS |
| runtime | third replay | `-1` | PASS |
| runtime | unknown generation | `-2` | PASS |
| runtime | empty payload | `-5` then replay `-1` | PASS |
| runtime | 8 concurrent copies | one `-8`, seven `-1` | PASS for same-generation replay |
| runtime | two fresh generations, same semantic action | `-8`, `-8` | COUNTEREXAMPLE to semantic idempotency |
| end-to-end persistence | committed knowledge write | not reached | NOT MATERIALIZED |

## Runtime receipt

```text
ARTIFACT_TYPE=THEORY_POC
THEORY=FFI_LAW0_AUTHORITY_NONDUPLICATION
RUN0_INSTRUMENT_VALIDATION=PASS
BASELINE_FRESH_GENERATION_CODE=-8
TARGET_COPIED_REPLAY_CODE=-1
ABLATION_BURNED_AUTHORITY_CODE=-1
ADVERSARIAL_FORGED_NONCE_CODE=-3
FORGED_NONCE_CODE=-3
FIRST_VALID_NONEMPTY_CODE=-8
COPIED_HANDLE_REPLAY_CODE=-1
THIRD_REPLAY_CODE=-1
UNKNOWN_GENERATION_CODE=-2
EMPTY_PAYLOAD_CODE=-5
EMPTY_PAYLOAD_REPLAY_CODE=-1
CONCURRENT_COPIES_POST_AUTH=1
CONCURRENT_COPIES_REJECTED=7
COUNTEREXAMPLE_FRESH_SEMANTIC_FIRST_CODE=-8
COUNTEREXAMPLE_FRESH_SEMANTIC_SECOND_CODE=-8
COUNTEREXAMPLE_STRONG_SEMANTIC_IDEMPOTENCY=FOUND
COPY_HANDLE_EQUALS_COPY_AUTHORITY=false
DIRECT_FFI_KNOWLEDGE_WRITE_MATERIALIZED=false
DIRECT_FFI_WRITE_BOUNDARY=COMPATIBILITY_HOLD
EXPERIMENT_STATUS=PASS
```

## Interpretation

`EXPERIMENT_STATUS=PASS` means the bounded claim and its negative boundary
were reproduced. It does not mean the legacy FFI route persisted knowledge.

The concurrency case is serialized by the current global `Mutex` before the
slot CAS is reached. It establishes exactly-one post-authorization result for
the tested interface, but this POC does not claim a lock-free performance
property.

## EXPERIMENTAL SUCCESS

For one issued generation, copied bits did not duplicate the hidden
Rust-owned authority: serial replay, third replay and seven losing concurrent
copies were rejected. Forged and unknown identities were also refused. The
strict release build, 15 tests and all three independent runtime launches
passed.

## EXPERIMENTAL FAILURE

The first strict `--all-targets` build failed because the authored extraction
adapter lacked non-focal shapes needed to compile the upstream file's internal
test module. The focal v189 files were not changed. After the adapter-only
closure, the build passed.

The scientific counterexample also remained: two freshly issued generations
for the same module and payload both crossed local authorization. The route
still terminated at `CompatibilityHold (-8)` and performed no Vault write.

## ADVANCEMENT METHOD

Preserve the existing hierarchy and add, in a successor implementation rather
than this sealed POC, a durable semantic authorization key consumed inside the
existing Shadow transaction, plus an idempotent sink and restart/replay test.
The FFI child must carry evidence to the confirmed-verdict authority; it must
not acquire verdict or persistence authority itself.

## Runtime-witness binary identity

```text
BINARY_NAME=poc005-ffi-law0-experiment.exe
STANDARD_BUILD_BINARY_SHA256=8a30199567bf6f98c06c2f3803ecc2412a41a1c8e622d888281f72c3c6420c19
DETACHED_BUILD_BINARY_SHA256=2fd169b23cac3666a3367a3a257d7718ffe95fc878fd5ecaa8ff96812dff7eeb
FINAL_PRE_RELEASE_BINARY_SHA256=79d5688c5704260552e0ee4fee174f091d912590a48b0a374400f2cdabd2177c
BINARY_BYTES_EACH=1272075
BUILD_PROFILE=release
EXIT_STATUS=0
CODE_INPUT_TREE_SHA256=40901be7a247fdac405dfd3f83f32879a73bd37faa2db76ee10f9664ad227233
CARGO_LOCK_SHA256=c4640fdbcac7c13624b1be6532ec981f1b8a8ad9d9d56ec54e70a9d421b02736
CODE_INPUT_DIFFS_BETWEEN_PUBLIC_AND_DETACHED=0
BEHAVIORAL_RECEIPT_REPRODUCIBILITY=PASS
BIT_FOR_BIT_BINARY_REPRODUCIBILITY=NOT_ESTABLISHED
```

All three binaries produced the same required experiment receipt, but clean
builds in different roots did not produce the same executable SHA-256. The
binary is therefore recorded as a runtime witness, not claimed as a
bit-for-bit reproducible artifact, and is deliberately not stored in the
capsule. A later packaging POC can test path remapping, fixed build inputs and
reproducible-linker controls; this does not change the POC-005 theory verdict.
