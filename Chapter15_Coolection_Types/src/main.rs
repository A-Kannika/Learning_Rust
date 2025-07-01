// Collections Types
// Vector
// UTF8
// Hash Maps

fn main() {
    let _v:Vec<i32> = Vec::new();
    // define the empty vector
    let _the_vec: Vec<i32> = vec![1,2,3];
    
    let mut _the_numbers_vec: Vec<i32> = Vec::new();
    let mut _the_numbers_vec: Vec<i32> = vec![1,2,3];
    _the_numbers_vec.push(4);
    _the_numbers_vec.push(5);
    _the_numbers_vec.push(6);
    println!("{:?}", _the_numbers_vec);
    
    // direct indexing
    let third:&i32 = &_the_numbers_vec[2];
    println!("{:?}", third);
    
    // how to get the third element
    let third = _the_numbers_vec.get(2);
    println!("{:?}", third);
    
    match third { 
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element")
    }
    
    
}
