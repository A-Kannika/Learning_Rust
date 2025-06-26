// Functions
// Start with fn and the name of function
// any function / variables should be written in snake case
// You can define the function anywhere and call it in the mian function
// Entry point

fn main() {
    hello_world();
    human_id("Knock", 44, 153.0);
    tell_height(153);
    let _x: i32 = {
        let price:i32 = 5;
        let qty:i32 = 10;
        price * qty
    };
    println!("Result is: {}", _x);
    
    // add (4,6)
    let y: i32 = add(4, 6);
    println!("Value of Y is {}", y);
    
    // call calculate BMI
    let weight:f64 = 70.0;
    let height:f64 = 1.53;
    let bmi:f64 = calculate_bmi(weight,height);
    println!("My BMI is {:.2}.", bmi)
}

// don't need to pass any parameter
fn hello_world() {
    println!("Hello, Rust!")
}

// can accept the parameter
fn tell_height(height:u32) {
    println!("My height is {} cm.", height)
}
// can accept more than 1 parameter
fn human_id(name: &str, age: u32, height: f32) {
    print!("My name is {}, I am {} years old, \
    and my height is {} cm.", name, age, height);
}

// can return the value
fn add(a:i32, b:i32) -> i32 {
    a + b
}

// Expressions and Statements
// Expression: Anything that returns a value
// Statement: Anything that does not return a value

// Expression example:
// integer
// bool (true or false)
// add(3, 4)
// if condition {value1} else {value2}
// ({code})

// Statement in Rust, end with --> ;
// Statement examples:
// let y = let x = 10;
// 1. Variable declarations" let x = 5;
// 2. Function definitions: fn foo(){}
// 3. Control flow statements:
//    if condition {/* code */} else condition {/* code */}
//    while condition {/* code */}
//    etc.

// final example on Functions
// BMI = height(kg)/height(m) ^2
fn calculate_bmi(weight_kg:f64, height_m:f64) -> f64{
    weight_kg/(height_m * height_m)
}