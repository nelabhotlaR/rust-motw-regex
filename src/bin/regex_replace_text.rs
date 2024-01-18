/*
The below code uses Regex:replace_all() method.
The program prompts user to enter sentence, then word to replace and replacement word
 */
use regex::Regex;
use std::io::{self, Write};

fn replace_text(sentence: &str, word_to_replace: &str, replacement_word: &str) -> Result<String, regex::Error> {
    // Create a Regex object for the word to be replaced
    let re = Regex::new(&format!(r"\b{}\b", regex::escape(word_to_replace)))?;

    // Replace all occurrences of the word
    let replaced_sentence = re.replace_all(sentence, replacement_word).to_string();
    Ok(replaced_sentence)
}

fn input_values() -> Result<(String, String, String), io::Error> {
    let mut sentence = String::new();
    let mut word_to_replace = String::new();
    let mut replacement_word = String::new();

    print!("Enter a sentence: ");
    io::stdout().flush().map_err(|err| io::Error::new(err.kind(), "Failed to flush stdout"))?;
    io::stdin().read_line(&mut sentence)?;

    print!("Enter the word to replace: ");
    io::stdout().flush().map_err(|err| io::Error::new(err.kind(), "Failed to flush stdout"))?;
    io::stdin().read_line(&mut word_to_replace)?;

    print!("Enter the replacement word: ");
    io::stdout().flush().map_err(|err| io::Error::new(err.kind(), "Failed to flush stdout"))?;
    io::stdin().read_line(&mut replacement_word)?;

    Ok((
        sentence.trim().to_string(),
        word_to_replace.trim().to_string(),
        replacement_word.trim().to_string(),
    ))
}

fn main() {
    match input_values() {
        Ok((sentence, word_to_replace, replacement_word)) => {
            match replace_text(&sentence, &word_to_replace, &replacement_word) {
                Ok(replaced_sentence) => {
                    println!("Modified sentence: {}", replaced_sentence);
                }
                Err(err) => eprintln!("Error replacing text: {}", err),
            }
        }
        Err(err) => eprintln!("Error getting input: {}", err),
    }
}