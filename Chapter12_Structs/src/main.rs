// Structs
// Structs are used to name the package-related values similar to tuples.

#![allow(warnings)]
fn main() {
    // tuple
    let react: (i32, i32) = (200, 500);
    
    // Structs
    struct  Book {
        title: String,
        author: String,
        pages: u32,
        available: bool,
    }
    
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }
    
    let book = Book {
        title: String::from("Rust Programming"),
        author: String::from("Knock Knock"),
        pages: 1000,
        available: true,
    };
    
    // make it mutable
    let mut user1 = User {
        active: true,
        username: String::from("Knock"),
        email: String::from("Knock@email.com"),
        sign_in_count: 1,
    };
    
    // how to update a struct
    println!("User1 old email: {}", user1.email);
    user1.email = String::from("Nok@email.com");
    println!("User1 new email: {}", user1.email);
    
    // return struct from function 
    // use this keyword -> User
    fn build_user(email: String, username: String) -> User{
        User {
            active: true,
            email,
            username,
            sign_in_count: 1,
        }
    }
    
    // create instances from other instances
    // we can use .. to copy all the values from one instance to another
    // So ..user1 will copy all the values from user1 to user2
    let user2 = User{
        email: String::from("new@email.com"),
        ..user1
    };
    
    // Tuple Structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    
    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);       
    let origin = Point(0, 0, 0);
    
    // unit like struct
    struct AlwaysEqual;
    let _always_equal = AlwaysEqual;
    
}
