use regex::Regex;

fn validate_ips(texts: Vec<&str>) -> Vec<String> {
    let mut valid_ips = Vec::new();

    let pattern = match Regex::new(r"\b(?:\d{1,3}\.){0,9}\d{1,3}\b") {
        Ok(pattern) => pattern,
        Err(_) => {
            println!("Invalid regex pattern");
            return valid_ips;
        }
    };
    
    let mut ip_found = false; // Flag to check if any IP is found in the texts

    for text in texts {
        let mut text_has_valid_ip = false; // Flag to check if any valid IP is found in the current text
        for captures in pattern.captures_iter(text) {
            if let Some(ip) = captures.get(0).map(|m| m.as_str()) {
                if is_valid_ip(ip) {
                    if !valid_ips.contains(&ip.to_string()) {
                        valid_ips.push(ip.to_string());
                    }
                    text_has_valid_ip = true;
                    ip_found = true;
                }
            }
        }

        if !text_has_valid_ip {
            println!("No valid IP address found in the text: '{}'", text);
        }
    }

    if !ip_found {
        println!("No valid IP addresses found in the texts vector.");
    }

    valid_ips
}

fn is_valid_ip(ip: &str) -> bool {
    let octets: Vec<&str> = ip.split('.').collect();

    if octets.len() != 4 {
        println!("Invalid IP: {} (Wrong number of octets)", ip);
        return false;
    }

    for octet in &octets {
        if let Ok(num) = octet.parse::<u8>() {
            if num > 255 {
                println!("Invalid IP: {} (Octet value out of range: {})", ip, octet);
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

fn main() {
    let texts = vec![
        "192.168.0.1 is the router's IP address.",
        "The server's IP is 10.0.0.11.12", // No valid IP in this text
        "Invalid IP: 300.200.100.1",
        "Another valid IP is 172.16.254.1",
        "10.11",
        "11",
    ];

    let valid_ips = validate_ips(texts);

    if valid_ips.is_empty() {
        println!("No valid IPs found");
    } else {
        println!("\nValid IPs:");
        for ip in valid_ips {
            println!("{}", ip);
        }
    }
}