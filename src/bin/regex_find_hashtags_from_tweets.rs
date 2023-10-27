// Finding #tags

use regex::Regex;

fn main() {
    // Define the regex pattern for extracting hashtags
    let pattern = r"#\w+";

    // Create a Regex object
    let re = Regex::new(pattern);

    // Check if the regex compilation was successful
    let re = match re {
        Ok(re) => re,
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        }
    };

    // Example tweets
    let tweets = vec![
        "We wrote a GitHub action to deply #rust code to AWS lambda.",
        "Excited to learn about #regex in #Rust!",
        "Check out how to use Py03 and maturin to call Rust libraries from Python #maturin #Rust #Py03",
    ];

    // Extract hashtags from each tweet
    for tweet in tweets {
        let matches: Vec<&str> = re.find_iter(tweet)
                                    .map(|mat| mat.as_str())
                                    .collect();

        // Print the hashtags
        if !matches.is_empty() {
            println!("Hashtags in tweet '{}': {:?}", tweet, matches);
        }
    }
}