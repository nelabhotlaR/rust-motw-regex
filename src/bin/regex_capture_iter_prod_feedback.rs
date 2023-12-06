// captures_iter: allows you to iterate over multiple captures of a 
// regular expression pattern in a text.
// The below example captures the email feedbacks and prints it

use regex::Regex;

fn capture_info_from_feedbacks(feedbacks: Vec<&str>) -> Vec<(String, String, String)> {
    let mut captured_info = Vec::new(); // Store captured information

    // Define a regex pattern to capture product, sentiment, and issues
    let pattern = r"Product: (\w+), Sentiment: (\w+), Issues: (.+)";
    let re = Regex::new(pattern).expect("Invalid regex pattern");

    // Extract information using captures_iter
    for feedback_email in feedbacks {
        for captures in re.captures_iter(feedback_email) {
            if let (Some(product), Some(sentiment), Some(issues)) = (
                captures.get(1).map(|m| m.as_str().to_string()),
                captures.get(2).map(|m| m.as_str().to_string()),
                captures.get(3).map(|m| m.as_str().to_string()),
            ) {
                captured_info.push((product, sentiment, issues));
            }
        }
    }

    captured_info // Return captured information
}

fn main() {
    // Sample feedback emails
    let feedbacks = vec![
        "Product: Godrej, Sentiment: Positive, Issues: None at all",
        "Product: colgate, Sentiment: Negative, Issues: Manufacturing defect",
        "Product: sensodyne, Sentiment: Neutral, Issues: Feature request",
    ];

    let captured_info = capture_info_from_feedbacks(feedbacks);

    // Print the captured information
    for (product, sentiment, issues) in captured_info {
        println!("Product: {}", product);
        println!("Sentiment: {}", sentiment);
        println!("Issues: {}", issues);
        println!(); // Adding an empty line for clarity
    }
}