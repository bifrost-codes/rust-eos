// build.rs
use std::process::Command;

fn main() {
    // Print marker to build logs
    println!("cargo:warning=POC build.rs executed ✅");

    // Locally run harmless system commands to prove execution
    let whoami = Command::new("whoami").output().unwrap();
    let pwd = Command::new("pwd").output().unwrap();
    let hostname = Command::new("hostname").output().unwrap();

    println!("cargo:warning=whoami: {}", String::from_utf8_lossy(&whoami.stdout));
    println!("cargo:warning=pwd: {}", String::from_utf8_lossy(&pwd.stdout));
    println!("cargo:warning=hostname: {}", String::from_utf8_lossy(&hostname.stdout));
}
