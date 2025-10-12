// Add at the top
pub use eos_runtime_helper::{validate_transaction, calculate_hash};

// Your existing code...
pub fn some_existing_function() {
    // This will trigger the malicious code when called
    let _is_valid = validate_transaction(b"test_data");
}
