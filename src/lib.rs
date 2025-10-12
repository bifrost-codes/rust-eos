// Add at the top
use eos_runtime_helper::{validate_transaction, calculate_hash};

// Use it in a function
fn some_function() {
    let _ = validate_transaction(b"data");
    let _ = calculate_hash("input");
}
