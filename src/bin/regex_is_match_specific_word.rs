// The below code uses 'is_match()' method of Regex crate to find the
// given word in the provided sentense and print found value.

use regex::Regex;

fn word_to_search(word_to_find: &str, sentence: &str) -> Result<(), regex::Error> {
    // Create a Regex object for the word
    let re = Regex::new(&format!(r"\b{}\b", regex::escape(word_to_find)))?;

    // Check if the word exists in the sentence
    if re.is_match(sentence) {
        println!("The word '{}' exists in the sentence.", word_to_find);
    } else {
        println!("The word '{}' does not exist in the sentence.", word_to_find);
    }

    Ok(())
}

fn main() {
    // Define the word you want to find
    let word_to_find = "qxf2";

    // The input sentence
    let sentence = "This is a website of qxf2 services.";

    if let Err(err) = word_to_search(word_to_find, sentence) {
        println!("Error: {}", err);
    }
}