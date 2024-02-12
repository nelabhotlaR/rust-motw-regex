/*
The below code uses Regex:replace_all() method.
The program prompts user to enter sentence, then word to replace and replacement word
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

fn replace_text(sentence: &str, word_to_replace: &str, replacement_word: &str) -> Result<String, regex::Error> {
    // Create a Regex object for the word to be replaced
    let re = match Regex::new(&format!(r"\b{}\b", regex::escape(word_to_replace))) {
        Ok(re) => re,
        Err(err) => return Err(err),
    };

    // Replace all occurrences of the word
    let replaced_sentence = re.replace_all(sentence, replacement_word).to_string();
    Ok(replaced_sentence)
}

// Program starting point
fn main() {
    let sentence = prompt_user_input("Enter a Sentence:");
    println!("Sentence: {}", sentence);

    let word_to_replace = prompt_user_input("Enter the word to replace:");
    println!("word to replace: {}", word_to_replace);

    let replacement_word = prompt_user_input("Enter the replacement word:");
    println!("Enter replacement word: {}", replacement_word);
    
    match replace_text(&sentence, &word_to_replace, &replacement_word) {
        Ok(replaced_sentence) => {
            println!("Modified sentence: {}", replaced_sentence);
        }
        Err(err) => eprintln!("Error replacing text: {}", err),
    }
}