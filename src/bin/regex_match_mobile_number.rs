// is_match: Checks if the regex pattern matches the entire text
// The below patter checks for the mobile number validation using is_match.

use regex::Regex;
use std::io;

fn main() {
    println!("Please Input Mobile Number (like +91-XXXXXXXXXX):");
    let mut mobile_number = String::new();
    match io::stdin().read_line(&mut mobile_number) {
        Ok(_) => {
            let result = mobile_number_validation(mobile_number.clone());
            println!("{}", result);
            println!("The Mobile Number: {}", mobile_number);
        }
        Err(error) => {
            eprintln!("Failed to read line: {}", error);
        }
    }
}

fn mobile_number_validation(s: String) -> &'static str {
    let pattern = r"\+91[-\s]?\d{10}"; // Pattern to match mobile number
    match Regex::new(pattern) {
        Ok(re) => {
            if re.is_match(&s) {
                "Valid Mobile Number"
            } else {
                "Invalid Mobile Number"
            }
        }
        Err(_) => "Invalid regex pattern",
    }
}