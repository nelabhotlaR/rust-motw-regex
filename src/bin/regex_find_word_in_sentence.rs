/* find(): searches for the first occurrence of a regex pattern in a string.
The below takes input sentence and search word from the user and prints the 
position for the search word.
To handle case insensitive used RegexBuilder
*/
use regex::RegexBuilder;
use std::io;

fn find_word_in_sentence(sentence: &str, search_word: &str) -> Result<String, regex::Error> {
    // Create a Regex object for the pattern
    let mut binding = RegexBuilder::new(search_word);
    let regex_builder = binding.case_insensitive(true);

    // Check if the regex compilation was successful
    let re = match regex_builder.build() {
        Ok(re) => re,
        Err(err) => {
            eprintln!("Error: {}", err);
            return Err(err);
        }
    };

    // Search for the pattern in the sentence
    if let Some(mat) = re.find(sentence) {
        let matched_text = mat.as_str();
        let start = mat.start();
        let end = mat.end();

        let word_position = format!("Found '{}' at positions {}-{}", matched_text, start, end);
        Ok(word_position)
    } else {
        let word_position = "Word not found in the provided sentence. ".to_string();
        Ok(word_position)
    }
}

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

fn main() {
    let sentence = prompt_user_input("Input Sentence:");
    println!("Sentence: {}", sentence);

    let search_word = prompt_user_input("Enter Search word from the input sentence:");
    println!("Search Word: {}", search_word);

    // Validate input
    if sentence.is_empty() || search_word.is_empty() {
        println!("Error: Input cannot be empty.");
        return;
    }
    match find_word_in_sentence(&sentence, &search_word) {
        Ok(word_position) => {
            println!("{}", word_position);
        }
        Err(err) => println!("Error: {}", err),
    }
}