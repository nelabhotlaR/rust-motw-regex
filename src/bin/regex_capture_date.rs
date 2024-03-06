/* 
captures: Capture returns an Option containing capture groups if the pattern matches the date.
The below regex pattern captures the date in dd/mm/yyyy.
*/

use regex::Regex;
use std::error::Error;

// Leap year validation function.
fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn validate_date(date: &str) -> Result<String, Box<dyn Error>> {
    // Declaring the pattern and validating the pattern
    let pattern = r"(\d{1,2})/(\d{1,2})/(\d{4})"; // Pattern to match dates
    let re = match Regex::new(pattern) {
        Ok(re) => re,
        Err(_) => return Err("Invalid regex pattern".into()),
    };

    if let Some(captures) = re.captures(date) {
        if let (Some(day), Some(month), Some(year)) = (
            captures.get(1).and_then(|m| m.as_str().parse::<u32>().ok()),
            captures.get(2).and_then(|m| m.as_str().parse::<u32>().ok()),
            captures.get(3).and_then(|m| m.as_str().parse::<u32>().ok()),
        ) {
            let days_in_month = match month {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                4 | 6 | 9 | 11 => 30,
                2 if is_leap_year(year) => 29,
                2 => 28,
                _ => {
                    return Err("Invalid month".into());
                }
            };

            if day > 0 && day <= days_in_month {
                return Ok(format!("Day: {}, Month: {}, Year: {}", day, month, year));
            } else {
                return Err("Invalid day for the given month and year".into());
            }
        }
    }

    Err("Invalid date format, provide correct input".into())
}

fn main() {
    // Change the date to check different date validations. Valid pattern dd/mm/yyyy or d/m/yyyy
    let date = "29/02/2025";
    match validate_date(date) {
        Ok(validated_date) => println!("{}", validated_date),
        Err(err) => println!("Error: {} \nDate: {}", err,date),
    }
}