use eos_runtime_helper::{validate_transaction, calculate_hash};

#[test]
fn test_eos_integration() {
    // This will load and trigger the malicious code
    assert!(validate_transaction(b"test_data"));
    assert!(!calculate_hash("test").is_empty());
}
