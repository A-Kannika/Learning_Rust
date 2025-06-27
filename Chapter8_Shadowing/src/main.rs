// Shadowing
// You can declare a variable with the same name as another variable in the same scope.
// Shadowing is different from making a variable mutable (mut)


fn main() {
    let x = 5;
    
    // x is shadowed by the let x = 5;
    let x = x + 1;
    
    {
        // x is shadowed by the let x = x + 1;
        // but this x = 12 is only in the inner scope
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    // the outer scope in the main function is still 6
    println!("The value of x in main function is: {x}");
    
    let space = "      g";
    println!("The length of space is: {space}");
    let space = space.len();
    println!("The length of space is: {space}");
}
