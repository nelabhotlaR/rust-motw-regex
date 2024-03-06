/*
captures_iter: allows you to iterate over multiple captures of a 
regular expression pattern in a text.
The below example captures the valid IP address from the provided 
text and prints it.
*/

use regex::{Error, Regex};

fn validate_ips(texts: Vec<&str>) -> Result<Vec<String>, Error> {
    let mut valid_ips = Vec::new();
    let pattern = match Regex::new(r"\b(?:\d{1,3}\.){0,9}\d{1,3}\b") {
        Ok(pattern) => pattern,
        Err(err) => return Err(err),
    };

    let mut ip_found = false; // Flag to check if any IP is found in the texts

    for text in texts {
        let text_has_valid_ip = process_text(&mut valid_ips, &pattern, text);
        if !text_has_valid_ip {
            println!("No valid IP address found in the text: '{}'\n", text);
        } else {
            ip_found = true;
        }
    }

    if !ip_found {
        println!("No valid IP addresses found in the texts vector.");
    }

    Ok(valid_ips)
}

fn process_text(valid_ips: &mut Vec<String>, pattern: &Regex, text: &str) -> bool {
    let mut text_has_valid_ip = false; // Flag to check if any valid IP is found in the current text
    for captures in pattern.captures_iter(text) {
        if let Some(ip) = captures.get(0).map(|m| m.as_str()) {
            if is_valid_ip(ip) {
                if !valid_ips.contains(&ip.to_string()) {
                    valid_ips.push(ip.to_string());
                }
                text_has_valid_ip = true;
            }
        }
    }
    text_has_valid_ip
}

fn is_valid_ip(ip: &str) -> bool {
    let octets: Vec<&str> = ip.split('.').collect();
    if octets.len() != 4 {
        println!("Invalid IP: {} (Wrong number of octets)", ip);
        return false;
    }

    for octet in &octets {
        match octet.parse::<u8>() {
            Ok(_num) => {
                // Your code when parsing succeeds
                {}
            },
            Err(_) => {
                // Your code when parsing fails
                println!("Invalid IP: {} (Octet value out of range: {})", ip, octet);
                return false;
            }
        }
    }
    true
}

fn main() {
    let texts = vec![
        "192.168.0.1 is the router's IP address.",
        "The server's IP is 10.0.0.11.12", // No valid IP in this text
        "300.200.100.1",
        "Another valid IP is 172.16.254.1",
        "10.11",
        "11",
        "ab.bc.da.xy",
        "190.350.10.11",
        "The IP: 10.15.20.21",
    ];

    match validate_ips(texts) {
        Ok(valid_ips) => {
            if valid_ips.is_empty() {
                println!("No valid IPs found");
            } else {
                println!("\nValid IPs:");
                for ip in valid_ips {
                    println!("{}", ip);
                }
            }
        }
        Err(err) => {
            println!("Regex error: {}", err);
        }
    }
}
