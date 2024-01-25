/*
The below code uses find_iter() method.
In the provided text the script searchs for email address pattern
*/

use regex::Regex;

fn search_email_pattern_from_sentence(text: &str) -> Option<Vec<String>> {
    let mut emails = Vec::new(); // Store the found email addresses

    // Define a valid regex pattern for email address
    let pattern = r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}\b";

    // Create a Regex object
    let re = match Regex::new(pattern) {
        Ok(re) => re,
        Err(err) => {
            eprintln!("Error: {}", err);
            return None; // Return None on error
        }
    };

    // Find all matches in the text
    for mat in re.find_iter(text) {
        emails.push(mat.as_str().to_string()); // Store the found email addresses
    }

    if emails.is_empty() {
        None // Return None if no emails are found
    } else {
        Some(emails) // Return the found email addresses
    }
}

fn main() {
    let text = "Contact us at mak@qxf2.com or support@qxf2.com or invalid.com for assistance.";
    println!("The text: {} ", text);

    if let Some(emails) = search_email_pattern_from_sentence(text) {
        // Print the matches
        println!("====================Email from text==============");
        for email in emails {
            println!("{}", email);
        }
    } else {
        println!("No email addresses found in the text.");
    }
}
