// find(): searches for the first occurrence of a regex pattern in a string, 
// returning an optional match.
// The below takes input and search word from user and prints the 
// position for the search word
use regex::Regex;
use std::io;

fn main() {
    // Prompt user for input
    println!("Enter a sentence:");
    let mut sentence = String::new();
    io::stdin().read_line(&mut sentence).expect("Failed to read line");

    println!("Enter the word to search for:");
    let mut pattern = String::new();
    io::stdin().read_line(&mut pattern).expect("Failed to read line");

    // Trim leading and trailing whitespace
    let sentence = sentence.trim();
    let pattern = pattern.trim();

    // Validate input
    if sentence.is_empty() || pattern.is_empty() {
        println!("Error: Input cannot be empty.");
        return;
    }

    // Create a Regex object for the pattern
    let re = Regex::new(pattern);

    // Check if the regex compilation was successful
    let re = match re {
        Ok(re) => re,
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        }
    };

    // Search for the pattern in the sentence
    if let Some(mat) = re.find(sentence) {
        let matched_text = mat.as_str();
        let start = mat.start();
        let end = mat.end();

        println!("Found '{}' at positions {}-{}", matched_text, start, end);
    } else {
        println!("Pattern not found in the sentence.");
    }
}