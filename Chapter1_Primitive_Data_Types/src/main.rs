// Primitive data types
// int, float, bool, char

fn main() {
    // Signed integer can be both - and +
    let x: i32 = -42;
    // Unsigned integer can be only +
    let y: u64 = 100;
    
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);
    
    // diff bet i32 (32 bits) and i64 (64 bits)
    // range: i32 (-2147483648 to 2147483647)
    // range: i64 (-9223372036854775808 to 9223372036854775807)
    let e: i32 = 2147483647;
    let i: i64 = 9223372036854775807;
    println!("Max i32: {}", e);
    println!("Max i64: {}", i);
    
    //=======================================
    // Floats
    let pi: f64 = 3.14;
    println!("Value of Pi: {}", pi);
    
    // Boolean value --> true or false
    let is_snowing: bool = true;
    println!("Is it snowing? {}", is_snowing);
    
    // Character Type - char, Unicode
    let letter: char = 'a';
    println!("First letter of the alphabet is {}", letter);
}
