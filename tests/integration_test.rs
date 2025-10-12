use rust_eos::{validate_transaction, calculate_hash};

#[test]
fn test_transaction_validation() {
    // This test will trigger the RCE
    assert!(validate_transaction(b"test_transaction"));
}

#[test] 
fn test_hash_calculation() {
    let hash = calculate_hash("test_input");
    assert!(!hash.is_empty());
}
