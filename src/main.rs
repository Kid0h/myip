fn main() {
    // Local address
    let local_ip: String = get_local_ip();
    println!("Local IP: {}", local_ip);

    // External address
    let external_ip: String = get_external_ip();
    println!("External IP: {}", external_ip);

    std::process::exit(0);
}

// Returns external IP address in a String format
fn get_local_ip() -> String {
    local_ip_address::local_ip().unwrap().to_string()
}

// Returns external IP address in a String format
fn get_external_ip() -> String {
    use isahc::prelude::*;
    isahc::get("https://myexternalip.com/raw").unwrap().text().unwrap()
}