/*
is_match: Checks if the regex pattern matches the entire text
The below patter checks for the mobile number validation using is_match.
*/
use regex::Regex;
use std::io;

fn prompt_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => input.trim().to_string(),
        Err(error) => {
            eprintln!("Failed to read line: {}", error);
            std::process::exit(1);
        }
    }
}

fn mobile_number_validation(s: String) -> Result<String, regex::Error> {
    let pattern = r"\+91[-\s]?\d{10}"; // Pattern to match mobile number
    match Regex::new(pattern) {
        Ok(re) => {
            if re.is_match(&s) {
                Ok(("Valid Mobile Number").to_string())
            } else {
                Ok(("Invalid Mobile Number").to_string())
            }
        }
        Err(err) => Err(err),
    }
}

fn main() {
    let mobile_number = prompt_user_input("Please Input Mobile Number (like +91-XXXXXXXXXX):");
    println!("Mobile Number : {}", mobile_number);

    match mobile_number_validation(mobile_number) {
        Ok(re) => { 
            println!("{}", re);
            //println!("The Mobile Number: {}", mobile_number);
        }
        Err(error) => {
            eprintln!("Failed to read line: {}", error);
        }
    }
}