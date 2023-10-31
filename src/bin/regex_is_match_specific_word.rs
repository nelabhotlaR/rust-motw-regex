use regex::Regex;

fn main() {
    // Define the word you want to find
    let word_to_find = "qxf2";

    // Create a Regex object for the word
    let re = Regex::new(&format!(r"\b{}\b", regex::escape(word_to_find)));

    // Check if the regex compilation was successful
    let re = match re {
        Ok(re) => re,
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        }
    };

    // The input sentence
    let sentence = "This is a website of qxf2 services.";

    // Check if the word exists in the sentence
    if re.is_match(sentence) {
        println!("The word '{}' exists in the sentence.", word_to_find);
    } else {
        println!("The word '{}' does not exist in the sentence.", word_to_find);
    }
}
