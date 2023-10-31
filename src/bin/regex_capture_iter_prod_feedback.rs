// captures_iter: allows you to iterate over multiple captures of a 
// regular expression pattern in a text.
// The below example captures the email feedbacks and prints it

use regex::Regex;

fn main() {
    // Define a regex pattern to capture product, sentiment, and issues
    let pattern = r"Product: (\w+), Sentiment: (\w+), Issues: (.+)";
    let re = Regex::new(pattern).expect("Invalid regex pattern");

    // Sample feedback emails
    let feedbacks = vec![
        "Product: Godrej, Sentiment: Positive, Issues: None at all",
        "Product: colgate, Sentiment: Negative, Issues: Manufacturing defect",
        "Product: sensodyne, Sentiment: Neutral, Issues: Feature request",
    ];

    // Extract information using captures_iter
    for feedback_email in feedbacks {
        for captures in re.captures_iter(feedback_email) {
            if let (Some(product), Some(sentiment), Some(issues)) = (
                captures.get(1).map(|m| m.as_str()),
                captures.get(2).map(|m| m.as_str()),
                captures.get(3).map(|m| m.as_str()),
            ) {
                println!("Product: {}", product);
                println!("Sentiment: {}", sentiment);
                println!("Issues: {}", issues);
            }
        }
    }
}