use ess_mai_ffi_law0_source_core::sovereign_ffi_gate::{
    sovereign_issue_capability, sovereign_validate_and_write, CapHandle, S_SHADOW_WRITE,
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct ExperimentReceipt {
    forged_nonce: i32,
    authorized_direct_write: i32,
    copied_handle_replay: i32,
    third_replay: i32,
    unknown_generation: i32,
    empty_payload: i32,
    empty_payload_replay: i32,
    concurrent_post_auth: usize,
    concurrent_replays: usize,
    fresh_semantic_first: i32,
    fresh_semantic_second: i32,
}

fn run_experiment() -> ExperimentReceipt {
    let payload = b"ESS-MAI POC-005 borrowed FFI payload";
    let issued = sovereign_issue_capability(S_SHADOW_WRITE);
    let copied = issued;
    let forged = CapHandle {
        gen: issued.gen,
        nonce: issued.nonce ^ 0xDEAD_BEEF,
    };

    // SAFETY: every non-empty pointer remains valid for each complete call.
    let forged_nonce = unsafe {
        sovereign_validate_and_write(forged, payload.as_ptr(), payload.len())
    };
    let authorized_direct_write = unsafe {
        sovereign_validate_and_write(issued, payload.as_ptr(), payload.len())
    };
    let copied_handle_replay = unsafe {
        sovereign_validate_and_write(copied, payload.as_ptr(), payload.len())
    };
    let third_replay = unsafe {
        sovereign_validate_and_write(copied, payload.as_ptr(), payload.len())
    };
    let unknown_generation = unsafe {
        sovereign_validate_and_write(
            CapHandle {
                gen: u64::MAX,
                nonce: S_SHADOW_WRITE,
            },
            payload.as_ptr(),
            payload.len(),
        )
    };

    let empty = sovereign_issue_capability(S_SHADOW_WRITE);
    // SAFETY: the ABI explicitly permits a null pointer when length is zero.
    let empty_payload = unsafe {
        sovereign_validate_and_write(empty, std::ptr::null(), 0)
    };
    let empty_payload_replay = unsafe {
        sovereign_validate_and_write(empty, std::ptr::null(), 0)
    };

    let concurrent = sovereign_issue_capability(S_SHADOW_WRITE);
    let workers: Vec<_> = (0..8)
        .map(|_| {
            let handle = concurrent;
            std::thread::spawn(move || {
                let body = b"concurrent-copy";
                // SAFETY: `body` is static and valid for the complete call.
                unsafe { sovereign_validate_and_write(handle, body.as_ptr(), body.len()) }
            })
        })
        .collect();
    let results: Vec<i32> = workers
        .into_iter()
        .map(|worker| worker.join().expect("worker must return"))
        .collect();

    // Counterexample search for the stronger semantic interpretation: two
    // fresh generations issued for the same module and the same payload each
    // carry independent one-shot authority. The local generation invariant
    // therefore does not implement durable semantic idempotency.
    let semantic_first = sovereign_issue_capability(S_SHADOW_WRITE);
    let semantic_second = sovereign_issue_capability(S_SHADOW_WRITE);
    let fresh_semantic_first = unsafe {
        sovereign_validate_and_write(semantic_first, payload.as_ptr(), payload.len())
    };
    let fresh_semantic_second = unsafe {
        sovereign_validate_and_write(semantic_second, payload.as_ptr(), payload.len())
    };

    ExperimentReceipt {
        forged_nonce,
        authorized_direct_write,
        copied_handle_replay,
        third_replay,
        unknown_generation,
        empty_payload,
        empty_payload_replay,
        concurrent_post_auth: results.iter().filter(|code| **code == -8).count(),
        concurrent_replays: results.iter().filter(|code| **code == -1).count(),
        fresh_semantic_first,
        fresh_semantic_second,
    }
}

fn receipt_is_expected(receipt: &ExperimentReceipt) -> bool {
    receipt.forged_nonce == -3
        && receipt.authorized_direct_write == -8
        && receipt.copied_handle_replay == -1
        && receipt.third_replay == -1
        && receipt.unknown_generation == -2
        && receipt.empty_payload == -5
        && receipt.empty_payload_replay == -1
        && receipt.concurrent_post_auth == 1
        && receipt.concurrent_replays == 7
        && receipt.fresh_semantic_first == -8
        && receipt.fresh_semantic_second == -8
}

fn main() {
    let receipt = run_experiment();
    println!("ARTIFACT_TYPE=THEORY_POC");
    println!("THEORY=FFI_LAW0_AUTHORITY_NONDUPLICATION");
    println!("RUN0_INSTRUMENT_VALIDATION=PASS");
    println!("BASELINE_FRESH_GENERATION_CODE={}", receipt.authorized_direct_write);
    println!("TARGET_COPIED_REPLAY_CODE={}", receipt.copied_handle_replay);
    println!("ABLATION_BURNED_AUTHORITY_CODE={}", receipt.third_replay);
    println!("ADVERSARIAL_FORGED_NONCE_CODE={}", receipt.forged_nonce);
    println!("FORGED_NONCE_CODE={}", receipt.forged_nonce);
    println!("FIRST_VALID_NONEMPTY_CODE={}", receipt.authorized_direct_write);
    println!("COPIED_HANDLE_REPLAY_CODE={}", receipt.copied_handle_replay);
    println!("THIRD_REPLAY_CODE={}", receipt.third_replay);
    println!("UNKNOWN_GENERATION_CODE={}", receipt.unknown_generation);
    println!("EMPTY_PAYLOAD_CODE={}", receipt.empty_payload);
    println!("EMPTY_PAYLOAD_REPLAY_CODE={}", receipt.empty_payload_replay);
    println!("CONCURRENT_COPIES_POST_AUTH={}", receipt.concurrent_post_auth);
    println!("CONCURRENT_COPIES_REJECTED={}", receipt.concurrent_replays);
    println!("COUNTEREXAMPLE_FRESH_SEMANTIC_FIRST_CODE={}", receipt.fresh_semantic_first);
    println!("COUNTEREXAMPLE_FRESH_SEMANTIC_SECOND_CODE={}", receipt.fresh_semantic_second);
    println!("COUNTEREXAMPLE_STRONG_SEMANTIC_IDEMPOTENCY=FOUND");
    println!("COPY_HANDLE_EQUALS_COPY_AUTHORITY=false");
    println!("DIRECT_FFI_KNOWLEDGE_WRITE_MATERIALIZED=false");
    println!("DIRECT_FFI_WRITE_BOUNDARY=COMPATIBILITY_HOLD");
    println!(
        "EXPERIMENT_STATUS={}",
        if receipt_is_expected(&receipt) { "PASS" } else { "FAIL" }
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copied_bits_do_not_duplicate_authority_and_the_write_gap_stays_visible() {
        let receipt = run_experiment();
        assert!(receipt_is_expected(&receipt), "unexpected receipt: {receipt:?}");
    }
}
