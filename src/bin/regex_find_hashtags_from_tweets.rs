use regex::Regex;

fn extract_hashtag_from_tweets(tweets: Vec<&str>) -> Vec<(&str, Vec<&str>)> {
    let mut all_matches: Vec<(&str, Vec<&str>)> = Vec::new(); // Store tweet and matches

    // Define the regex pattern for extracting hashtags
    let pattern = r"#\w+";

    // Create a Regex object
    let re = Regex::new(pattern);

    // Check if the regex compilation was successful
    let re = match re {
        Ok(re) => re,
        Err(err) => {
            eprintln!("Error: {}", err);
            return all_matches; // Return an empty vector on error
        }
    };

    // Iterate through each tweet
    for tweet in tweets {
        // Extract hashtags from each tweet
        let matches: Vec<&str> = re.find_iter(tweet)
                                   .map(|mat| mat.as_str())
                                   .collect();

        // Store tweet and matches
        if !matches.is_empty() {
            all_matches.push((tweet, matches)); // Store tweet and matches
        }
    }

    all_matches // Return tweets with their corresponding matches
}

fn main() {
    // Example tweets
    let tweets = vec![
        "We wrote a GitHub action to deply #rust code to AWS lambda.",
        "Excited to learn about #regex in #Rust!",
        "Check out how to use Py03 and maturin to call Rust libraries from Python #maturin #Rust #Py03",
    ];

    let extracted_data = extract_hashtag_from_tweets(tweets);

    // Print tweets followed by their corresponding matches
    for (tweet, matches) in extracted_data {
        println!("Tweet: '{}'", tweet);
        println!("Extracted Hashtag: {:?}", matches);
        println!(); // Adding an empty line for clarity
    }
}