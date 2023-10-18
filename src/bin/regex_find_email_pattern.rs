// In the provided text the script searchs for email address pattern

use regex::Regex;

fn main() {
    // Define an valid regex pattern for email address
    let pattern = r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b";

    // Create a Regex object
    let re = Regex::new(pattern);

    // Check if the regex compilation was successful
    let re = match re {
        Ok(re) => re,
        Err(err) => {
            eprintln!("Test self generated error: {}", err);
            return;
        }
    };

    // The input text containing email addresses
    let text = "Contact us at mak@qxf2.com or raghava.nelbo@qxf2.com for assistance.";

    // Find all matches in the text
    let matches: Vec<&str> = re.find_iter(text)
                                .map(|mat| mat.as_str())
                                .collect();

    // Print the matches
    for email in matches {
        println!("{}", email);
    }
}
