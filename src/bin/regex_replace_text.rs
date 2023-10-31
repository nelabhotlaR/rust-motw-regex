// replace_all: all occurrences of a regex pattern in a string with a specified 
// replacement text.
// the below example takes input from the user and replaces with speicified word.
use regex::Regex;
use std::io;

fn main() {
    // Prompt user for input
    println!("Enter a sentence:");
    let mut sentence = String::new();
    io::stdin().read_line(&mut sentence).expect("Failed to read line");

    println!("Enter the word to replace:");
    let mut word_to_replace = String::new();
    io::stdin().read_line(&mut word_to_replace).expect("Failed to read line");

    println!("Enter the replacement word:");
    let mut replacement_word = String::new();
    io::stdin().read_line(&mut replacement_word).expect("Failed to read line");

    // Trim leading and trailing whitespace
    let sentence = sentence.trim();
    let word_to_replace = word_to_replace.trim();
    let replacement_word = replacement_word.trim();

    // Validate input
    if sentence.is_empty() || word_to_replace.is_empty() || replacement_word.is_empty() {
        println!("Error: Input cannot be empty.");
        return;
    }

    // Create a Regex object for the word to be replaced
    let re = Regex::new(&format!(r"\b{}\b", regex::escape(word_to_replace)));

    // Check if the regex compilation was successful
    let re = match re {
        Ok(re) => re,
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        }
    };

    // Replace all occurrences of the word
    let replaced_sentence = re.replace_all(sentence, replacement_word);

    // Print the modified sentence
    println!("{}", replaced_sentence);
}