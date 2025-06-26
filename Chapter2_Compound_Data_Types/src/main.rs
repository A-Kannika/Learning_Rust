use std::string;

// Compound Data Types
//arrays, tuples, slices, strings (String), and slice string (&str)
fn main() {
    
    // Arrays --> Homogenous type (collection of same type elements)
    let numbers: [i32;5] = [1,2,3,4,5];
    println!("Number Array: {:?}", numbers);
    
    // Arrays of Strings Slices
    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    // print the whole array
    println!("Fruit Arrays: {:?}", fruits);
    // print the element at the specific index
    println!("Fruit Arrays: {}", fruits[0]);
    println!("Fruit Arrays: {}", fruits[1]);
    println!("Fruit Arrays: {}", fruits[2]);
    
    //=================================================
    // Tuples
    // Using String
    let human1: (String, i32, bool) = ("Alice".to_string(), 30, false);
    // Using String Slices
    let human2: (&str, i32, bool) = ("Alice", 30, false);
    let human3 = ("Alice", 30, false);
    println!("Human1 Tuple: {:?}", human1);
    println!("Human2 Tuple: {:?}", human2);
    println!("Human3 Tuple: {:?}", human3);
    
    // mix tuple
    let my_mix_tuple = ("Knock", 44, true, [1,2,3,4,5]);
    println!("Mix Tuple: {:?}", my_mix_tuple);
    
    //==========================================
    // Slices: simply are dynamically sized view into a contiguous sequence of elements
    // in computer science and specifically in programming
    // a contiguous sequence of elements is a very well-known term
    // especially when it comes to memory,
    // For example, [1,2,3,4,5] --> contiguous means uninterrupted
    // adjacent to one another, so when display a contiguous sequence form like an array
    // the memory doesn't have to jump between memories
    // the elements: 1 ,2 ,3 ,4 ,5 will be next to each other in the memories
    
    let nums_slices: &[i32] = &[1,2,3,4,5];
    println!("Number Slice: {:?}", nums_slices);

    // string 
    let animal_slices1: &[&str] = &["Cat","Dog","Bird"];
    println!("Animal Slice1: {:?}", animal_slices1);
    
    // reference to string
    let animal_slices2: &[&String] = &[&"Cat".to_string(),&"Dog".to_string(),&"Bird".to_string()];
    println!("Animal Slice2: {:?}", animal_slices2);
    
    // Strings Vs String Slices (&str)
    // Strings are growable/expandable --> we can increase or decrease the data
    // Strings are mutable --> we can push and delete the data
    // they are owned String types --> not borrowed --> ownership
    // the Strings will be allocated on the Heap,  
    // and then these string objects can grow or shrink in size as needed dynamically --> not fix
    // Strings --> [ growable, mutable, owned string type ]

    // keyword mut --> make the string mutable
    let stone_cold1: String = String::from("Hello, ");
    println!("Stone Cold1 Says: {}", stone_cold1);
    
    // keyword mut --> make the string mutable
    let mut stone_cold2: String = String::from("Hell, ");
    println!("Stone Cold2 Says: {}", stone_cold2);
    stone_cold2.push_str("Yeah!");
    println!("Stone Cold2 Says: {}", stone_cold2);
    
    // B- &str (String Slice) --> from & --> this is the reference --> not the owned string
    // It's a portion of string that it's stored somewhere in the code
    // IMMUTABLE --> can't modify
    // Good when want to work with the string without taking ownership of it
    // This will work on the Stack --> faster than the Heap
    
    let string:String = String::from("Hello, World!");
    let slice:&str = &string;
    println!("Slice Value: {}", slice);
    let slice_index:&str = &string[0..5];
    println!("Slice Index Value: {}", slice_index)
}
