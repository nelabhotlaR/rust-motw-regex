// captures_len: returns the number of capturing groups in a regular expression pattern. 
// The below program uses capture len to get the count to verify the structure of 
// captured data and ensure it matches the expected format.
use regex::Regex;

fn main() {
    // Define a regex pattern to capture timestamp, log level, and message
    let pattern = r"\[(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})\] \[([A-Z]+)\] (.+)";
    let re = Regex::new(pattern).expect("Invalid regex pattern");

    // Sample log entry
    let log_entry = "[2023-10-31 15:23:45] [INFO] User logged in";

    // Count the number of capture groups
    let captures_len = re.captures_len();
    println!("Length of pattern: {}", captures_len);

    match captures_len {
        4 => {
            if let Some(captures) = re.captures(log_entry) {
                let timestamp = captures.get(1).map(|m| m.as_str());
                let log_level = captures.get(2).map(|m| m.as_str());
                let message = captures.get(3).map(|m| m.as_str());

                match (timestamp, log_level, message) {
                    (Some(timestamp), Some(log_level), Some(message)) => {
                        println!("Timestamp: {}", timestamp);
                        println!("Log Level: {}", log_level);
                        println!("Message: {}", message);
                    },
                    _ => println!("Incomplete match found"),
                }
            } else {
                println!("No match found");
            }
        },
        _ => println!("Invalid regex pattern. Expected 3 capture groups, found {}.", captures_len - 1),
    }
}