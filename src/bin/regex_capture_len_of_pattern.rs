// captures_len: returns the number of capturing groups in a regular expression pattern. 
// The below program uses capture len to get the count to verify the structure of 
// captured data and ensure it matches the expected format.
use regex::Regex;

fn capture_info_from_log_entry(log_entries: Vec<&str>) -> Vec<Result<(String, String, String), &'static str>> {
    // Define a regex pattern to capture timestamp, log level, and message
    let pattern = r"\[(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})\] \[([A-Z,a-z]+)\] (.+)";
    let re = match Regex::new(pattern) {
        Ok(re) => re,
        Err(_) => return vec![Err("Invalid regex pattern"); log_entries.len()],
    };

    log_entries
        .iter()
        .map(|log_entry| {
            // Count the number of capture groups
            let captures_len = re.captures_len();
            match captures_len {
                4 => {
                    if let Some(captures) = re.captures(log_entry) {
                        let timestamp = captures.get(1).map(|m| m.as_str().to_string()).ok_or("Incomplete match found");
                        let log_level = captures.get(2).map(|m| m.as_str().to_string()).ok_or("Incomplete match found");
                        let message = captures.get(3).map(|m| m.as_str().to_string()).ok_or("Incomplete match found");

                        Ok((timestamp?, log_level?, message?))
                    } else {
                        Err("No match found, Expected 3 capture groups")
                    }
                },
                _ => Err("Invalid regex pattern. Expected 3 capture groups."),
            }
        })
        .collect()
}

fn main() {
    // Sample log entries
    let log_entries = vec![
        "[2023-10-31 15:23:45] [ERROR] *370 connect() failed (111: Unknown error) while connecting to upstream, client: 135.125.246.189, server: _, request: \"GET /.env HTTP/1.1\", upstream: \"http://127.0.0.1:5000/.env\", host: \"18.118.196.200\"",
        "[2023-10-31 15:30:00] [INFO] Application started successfully",
        "[2023-10-31 15:40:22] [WARNING] Unrecognized log entry format",
        "[2023-01-01 00:00:00] [info] "
        // Add more log entries for testing
    ];

    let results = capture_info_from_log_entry(log_entries);

    for (index, result) in results.iter().enumerate() {
        match result {
            Ok((timestamp, log_level, message)) => {
                println!("Log Entry {}: ", index + 1);
                println!("Timestamp: {}", timestamp);
                println!("Log Level: {}", log_level);
                println!("Message: {}", message);
                println!();
            },
            Err(err) => {
                println!("Error in Log Entry {}: {}", index + 1, err);
                println!();
            }
        }
    }
}