// Ownership, Borrowing and References
// Ownership
// ------
// C, C++ --> Memory management Control Issues --> need to free memory manually
// Garbage Collector solved but not perfect, it creates a new issue
// Slow the performance
//
// Rust use the concept of Ownership
// Rust --> Memory management --> Automatic Memory Management
// What's ownership?
// Every value has a single owner. 
// Every variable has one value.
// Ownership rules:
// - Each value in Rust has a variable that's called its owner.
// - There can only be one owner at a time.
// - When the owner goes out of scope, the value will be dropped.

fn main() {
    
    // Example - Each value in Rust has a variable that's called its owner.
    // s1 is the owner of the String
    let s1: String = String::from("Rust");
    // &s1 is the borrow of the String --> the reference
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
    
    //---------------------------------------------------
    
    // Example - There can only be one owner at a time.
    let s2 = String::from("Hello");
    
    // transfer the ownership of s2 to s3
    let s3 = s2;

    // cannot use s2 now --> print!("S2 {}", s2); error
    println!("S3 {}", s3);

    //---------------------------------------------------
    // Example - When the owner goes out of scope, the value will be dropped.
    
    let s4 = String::from("World");
    let len = calculate_length(&s4);
    println!("The length of '{}' is {}.", s4, len);
}

// cannot use s4 out of the main function --> println!("S4 {}", s4); error

fn calculate_length(s: &String) -> usize {
    s.len()
}
