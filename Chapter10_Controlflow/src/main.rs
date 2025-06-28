// Control Flow
// 1 Condition, if (expression) else (expression)
// 2 Repeating action, loops

// bypass the warning
#![allow(unused_variables)]
fn main() {
    let age: u16 = 15;
    if age >= 18 {
        println!("You are old enough to vote!");
    } else {
        println!("You are not old enough to vote!");
    }
    
    // multiple conditions with else if
    let number = 16;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    
    // Using let in the if expression
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is: {}", number);
    
}
