// splitn(): splits a string based on a regex pattern.
// The below example uses splitn to extract key details 
// like product name, price, and description. 
use regex::Regex;

fn main() {
    // Define a regex pattern to extract product details
    let pattern = r"\|"; // Assuming "|" is the delimiter

    // Sample product description
    let product_description = "Widget | $29.99 | A high-quality widget for your needs.";

    // Create a Regex object for the pattern
    let re = Regex::new(pattern).expect("Invalid regex pattern");

    // Use splitn to split the string
    let parts: Vec<&str> = re.splitn(product_description, 3).collect();

    // Check if enough parts were found
    if parts.len() >= 3 {
        let product_name = parts[0].trim();
        let price = parts[1].trim();
        let description = parts[2].trim();

        println!("Product Name: {}", product_name);
        println!("Price: {}", price);
        println!("Description: {}", description);
    } else {
        println!("Invalid product description format");
    }
}