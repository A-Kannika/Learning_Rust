// There are 3 types of loops in Rust
// Loops
// While Loops
// For Loops

fn main() {
    // loop keyword
    // this will print out hello world forever
    // loop{
    //     println!("Hello World");
    // }
    
    // while loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
    
    // Loop labels to disambiguate between multiple loops
    // Nested loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;
        loop {
            println!("remaining = {}", remaining);
            if remaining == 0 {
                break 'counting_up;
            }
            remaining -= 1;
            count += 1;
        }
    }
    
    // while loop
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("{}!", number);
    
    // For loop
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }
}
