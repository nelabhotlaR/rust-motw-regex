/*
The below code uses find_iter() method.
In the provided text the script searchs for email address pattern
*/

use regex::Regex;
use std::error::Error;

fn search_email_pattern_from_sentence(text: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut emails = Vec::new(); // Store the found email addresses

    // Define a valid regex pattern for email address
    let pattern = r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}\b";

    // Create a Regex object
      let re = match Regex::new(pattern) {
        Ok(re) => re,
        Err(err) => return Err(err.into()), // Convert the regex error to Box<dyn Error>
    };

    // Find all matches in the text
    for mat in re.find_iter(text) {
        emails.push(mat.as_str().to_string()); // Store the found email addresses
    }

    if emails.is_empty() {
        return Err("No email addresses found in the text.".into()); // Return an error if no emails are found
    }

    Ok(emails) // Return the found email addresses
}

fn main() {
    let text = "Contact us at mak@qxf2.com or support@qxf2.com or invalid.com for assistance.";
    println!("The text: {} ", text);

    match search_email_pattern_from_sentence(text) {
        Ok(emails) => {
            // Print the matches
            println!("====================Email from text=============================");
            for email in emails {
                println!("{}", email);
            }
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}