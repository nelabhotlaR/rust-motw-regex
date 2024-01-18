/* 
captures: Capture returns an Option containing capture groups if the pattern matches the date.
The below regex pattern captures the date in dd/mm/yyyy.
*/

use regex::Regex;
use std::str::FromStr;

// Leap year validation function.
fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn main() {
    // Change the date to check different date validations. Valid pattern dd/mm/yyyy or d/m/yyyy
    let date = "33/12/2003";
    validate_date(date);
}

fn validate_date(date: &str) {
    // Declaring the pattern and validating the pattern
    let pattern = r"(\d{1,2})/(\d{1,2})/(\d{4})"; // Pattern to match dates
    let re = match Regex::new(pattern) {
        Ok(re) => re,
        Err(_) => {
            println!("Invalid regex pattern");
            return;
        }
    };

    if let Some(captures) = re.captures(date) {
        if let (Some(day), Some(month), Some(year)) = (
            captures.get(1).and_then(|m| u32::from_str(m.as_str()).ok()),
            captures.get(2).and_then(|m| u32::from_str(m.as_str()).ok()),
            captures.get(3).and_then(|m| u32::from_str(m.as_str()).ok()),
        ) {
            let days_in_month = match month {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                4 | 6 | 9 | 11 => 30,
                2 if is_leap_year(year) => 29,
                2 => 28,
                _ => {
                    println!("Invalid month");
                    return;
                }
            };

            if day > 0 && day <= days_in_month {
                println!("Day: {}, Month: {}, Year: {}", day, month, year);
            } else {
                println!("Invalid day for the given month and year");
            }
        }
    } else {
        println!("Invalid date format, provide correct input");
    }
}