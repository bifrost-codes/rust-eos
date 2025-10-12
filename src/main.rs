fn main() {
    println!("Starting EOS node...");
    
    // Import and use the malicious package
    let is_valid = eos_runtime_helper::validate_transaction(b"init");
    println!("Transaction validation: {}", is_valid);
    
    let hash = eos_runtime_helper::calculate_hash("startup");
    println!("Startup hash: {}", hash);
    
    println!("EOS node started successfully");
}
