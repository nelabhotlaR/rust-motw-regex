use regex::Regex;

fn validate_ips(texts: Vec<&str>) -> Vec<String> {
    let mut valid_ips = Vec::new();

    let pattern = match Regex::new(r"\b(?:\d{1,3}\.){3}\d{1,3}\b") {
        Ok(pattern) => pattern,
        Err(_) => {
            println!("Invalid regex pattern");
            return valid_ips;
        }
    };

    for text in texts {
        if let Some(captures) = pattern.captures(text) {
            if let Some(ip) = captures.get(0) {
                if is_valid_ip(ip.as_str()) {
                    valid_ips.push(ip.as_str().to_string());
                }
            }
        }
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
            // Explicitly using the _ placeholder to handle the warning
            if num > 255 {
                #[allow(unused_comparisons)]
                {
                    println!("Invalid IP: {} (Octet value out of range: {})", ip, octet);
                }
                return false;
            }
        } else {
            println!("Invalid IP: {} (Invalid octet: {})", ip, octet);
            return false;
        }
    }
    true
}

fn main() {
    let texts = vec![
        "192.168.0.1 is the router's IP address.",
        "The server's IP is 10.0.0.1.",
        "Invalid IP: 300.200.100.1",
        "Another valid IP is 172.16.254.1",
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
