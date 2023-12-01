// is_match: Checks if the regex pattern matches the entire text
// The below patter checks for the mobile number validation using is_match.

use regex::Regex;
use std::io;

fn main() {
    println!("Please Input Mobile Number(like +91-XXXXXXXXXX):");
    let mut mobile_number = String::new();
    io::stdin().read_line(&mut mobile_number).expect("Failed to read line");

    //let text = String::from("+91-9900443354");
    let result = mobile_number_validation(mobile_number.clone());
    println!("{}",result);
    println!("The Mobile Number: {}", mobile_number);
}

fn mobile_number_validation(s: String) -> &'static str {
    let pattern = r"\+91-\d{10}"; // Pattern to match mobile number
    let re = Regex::new(pattern).expect("Invalid regex pattern");

    if re.is_match(&s) { "Valid Mobile Number" } else { "Invalid Mobile Number" }
}