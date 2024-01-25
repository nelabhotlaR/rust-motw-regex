/* find(): searches for the first occurrence of a regex pattern in a string.
The below takes input sentence and search word from the user and prints the 
position for the search word.
*/

use regex::RegexBuilder;
use std::io;

fn main() {
    // Prompt user for input
    let mut sentence = String::new();
    println!("Input Sentence: ");
    match io::stdin().read_line(&mut sentence) {
        Ok(_) => {
            println!("Sentence: {}", sentence);
        }
        Err(error) => {
            eprintln!("Failed to read line: {}", error);
            return;
        }
    }

    let mut search_word = String::new();
    println!("Enter Search word from the input sentence: ");
    match io::stdin().read_line(&mut search_word) {
        Ok(_) => {
            println!("Search Word: {}", search_word);
        }
        Err(error) => {
            eprintln!("Failed to read line: {}", error);
            return;
        }
    }

    // Trim leading and trailing whitespace
    let sentence = sentence.trim();
    let search_word = search_word.trim();

    // Validate input
    if sentence.is_empty() || search_word.is_empty() {
        println!("Error: Input cannot be empty.");
        return;
    }
    find_word_in_sentense(sentence.clone(), search_word.clone())
}

fn find_word_in_sentense(sentence: &str, search_word: &str)  {
    // Create a Regex object for the pattern
    //let re = Regex::new(search_word);
    //let regex_builder = RegexBuilder::new(search_word).case_insensitive(true);
    let mut binding = RegexBuilder::new(search_word);
    let regex_builder = binding.case_insensitive(true);

    // Check if the regex compilation was successful
    let re = match regex_builder.build() {
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
        println!("Word not found in the provided sentence. ");
    }

}