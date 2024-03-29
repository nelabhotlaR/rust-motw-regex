/* 
The below example uses splitn method.
The below example uses splitn to extract key details like product name, price, and description. 
Used ansi_term crate - This Crate is a library for colours and formatting
*/ 
use regex::Regex;
use ansi_term::Colour::{Red, Blue};

fn split_text(product_description: &str) -> Result<Vec<&str>, regex::Error> {
    let pattern = r"\|"; // Assuming "|" is the delimiter
    let re = match Regex::new(pattern) {
        Ok(re) => re,
        Err(e) => return Err(e),
    };

    let parts: Vec<&str> = re.splitn(product_description, 3).collect();
    Ok(parts)
}

fn main() {
    let product_description = "Widget | $29.99 | A high-quality widget for your needs.";
    println!("The Product Description: {}", Red.paint(product_description));
    println!("{}", Blue.paint("================The Split:Product/Price/Desc==================================="));
    match split_text(&product_description) {
        Ok(parts) => {
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
        Err(err) => {
            println!("Failed to split product description: {}", err);
            // You can take further actions here for error handling if needed.
        }
    }
}