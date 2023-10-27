// is_match: Checks if the regex pattern matches the entire text
// The below patter checks for the mobile number validation using is_match.

use regex::Regex;

fn main() {
    let pattern = r"\+91-\d{10}"; // Pattern to match mobile number
    let re = Regex::new(pattern).expect("Invalid regex pattern");
    
    let text = "+91-9900443354";
    if re.is_match(text) {
        println!("Valid mobile number");
    } else {
        println!("Invalid mobile number");
    }
}