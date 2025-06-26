// Constants
// Don't allow changing the value of a constant
// Cannot use keyword mut

fn main() {
    let mut x = 5;
    
    // const mut y: i32 = 5; --> error
    // Use capital variable name for constants
    const Y: i32 = 5;
    
    println!("x = {}, y = {}", x, Y);
    x = 6;
    println!("x = {}, y = {}", x, Y);
    
    println!("PI = {}", PI);
    println!("THREE_HOURS_IN_SECONDS = {}", THREE_HOURS_IN_SECONDS);
}

// you can declare a constant without as a global variable
const PI: f64 = 3.14;
const THREE_HOURS_IN_SECONDS: u32 = 60*60*3;
