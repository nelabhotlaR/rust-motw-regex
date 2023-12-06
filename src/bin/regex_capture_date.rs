// captures: Capture returns an Option containing capture groups if the pattern matches the text.
// The below regex pattern captures the date in dd/mm/yyyy.

use regex::Regex;

fn main() {
    let pattern = r"(\d{2})/(\d{2})/(\d{4})"; // Pattern to match dates
    let re = Regex::new(pattern).expect("Invalid regex pattern");
    
    let text = "10/31/2023";
    if let Some(captures) = re.captures(text) {
        println!("Day: {}, Month: {}, Year: {}", &captures[1], &captures[2], &captures[3]);
    } else {
        println!("No match found");
    }
}
