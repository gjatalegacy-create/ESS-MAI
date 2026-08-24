//! New surgical Shadow executable boundary for POC-003.

use std::path::Path;

fn main() {
    let arguments = std::env::args().collect::<Vec<_>>();
    let result = match arguments.as_slice() {
        [_, flag, request, response] if flag == "--selection-once" => {
            ess_mai_system_poc_003::run_shadow_selector_once(
                Path::new(request),
                Path::new(response),
            )
        }
        _ => Err("POC-003 Shadow accepts only --selection-once <request> <response>".to_string()),
    };
    match result {
        Ok(()) => {}
        Err(error) => {
            eprintln!("[POC003/SHADOW] FAIL-CLOSED: {error}");
            std::process::exit(1);
        }
    }
}

