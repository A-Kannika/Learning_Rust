// Error Handling Technique

// Approach1
// enum Option<T> { // define the generic option type
//     Some(T), // represents a value of type T
//     None, // represents the absence of a value (no value)
// }

// Approach 2
// enum Result<T, E> { // define the generic result type
//     Ok(T), // represents a value of type T
//     Err(E), // represents an error of type E
// }

// Example on Option<T>
// Creates the function
fn divide1(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        None
    } else {
        Some(x / y)
    }
}

fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        return Err("Division by zero".to_string());
    }
    Ok(numerator / denominator)
}

fn main() {
    let result = divide1(10.0, 0.0);
    match result {
        Some(value) => println!("Result: {}", value),
        None => println!("Error: Division by zero"),
    }
    
    match divide(100.23, 0.0) { 
        Ok(value) => println!("Result: {}", value),
        Err(error) => println!("Error: {}", error),
    }
}