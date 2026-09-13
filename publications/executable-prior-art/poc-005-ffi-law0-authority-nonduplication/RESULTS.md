# Experimental results

## Outcome summary

| Layer | Test | Observed result | Scientific verdict |
|---|---|---:|---|
| native v189 | focal `sovereign_ffi_gate` suite | 8/8 pass | source behavior reproduced |
| detached Cargo | release workspace build | exit 0 | build closure valid |
| detached Cargo | POC falsification test | 1/1 pass | expected success + hold reproduced |
| runtime | forged nonce | `-3` | PASS |
| runtime | first valid non-empty call | `-8` | AUTHORITY PASS / WRITE HOLD |
| runtime | copied handle replay | `-1` | PASS |
| runtime | third replay | `-1` | PASS |
| runtime | unknown generation | `-2` | PASS |
| runtime | empty payload | `-5` then replay `-1` | PASS |
| runtime | 8 concurrent copies | one `-8`, seven `-1` | PASS for same-generation replay |
| end-to-end persistence | committed knowledge write | not reached | NOT MATERIALIZED |

## Runtime receipt

```text
ARTIFACT_TYPE=THEORY_POC
THEORY=FFI_LAW0_AUTHORITY_NONDUPLICATION
FORGED_NONCE_CODE=-3
FIRST_VALID_NONEMPTY_CODE=-8
COPIED_HANDLE_REPLAY_CODE=-1
THIRD_REPLAY_CODE=-1
UNKNOWN_GENERATION_CODE=-2
EMPTY_PAYLOAD_CODE=-5
EMPTY_PAYLOAD_REPLAY_CODE=-1
CONCURRENT_COPIES_POST_AUTH=1
CONCURRENT_COPIES_REJECTED=7
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

## Reproducible binary identity

```text
BINARY_NAME=poc005-ffi-law0-experiment.exe
BINARY_SHA256=b36aa552e4a4b66ce65e43c7badf494a06936784c63a3ef66425c09bc63b98ca
BUILD_PROFILE=release
EXIT_STATUS=0
SOURCE_TREE_SHA256=e732fa684190bf0966a80ab55de30a44eecd576f81a8e2397e30c0776f0eef23
CARGO_LOCK_SHA256=c4640fdbcac7c13624b1be6532ec981f1b8a8ad9d9d56ec54e70a9d421b02736
```

The binary is a reproducible build artifact and is deliberately not stored in
this audit/publication capsule.

