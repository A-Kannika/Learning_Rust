// Variables and Mutability
// Immutable --> cannot be changed

fn main() {
    
    // immutable
    let a: u128 = 123;
    println!("The value of a is {}", a);
    
    // mutable
    let mut b: u128 = 123;
    println!("The value of b is {}", b);
    b = 456;
    println!("The value of b is changed to {}", b);
}
