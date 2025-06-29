// Enums
// An Enum is a versatile tool used to represent a type 
// that can take one of several possible variants.
fn main() {
    // using enum for the ip address is an appropriate way to do so
    // because ip address can be either v4 or v6 but not both at the same time
    enum IpAddrKind {
        V4,
        V6,
    }
    
    // creating an instance of enum
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;
    
    // how to use enum in a function
    fn route(_ip_kind: IpAddrKind) {}
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
    
    // we can use enum in a struct
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }
    
    // create the first instance of struct
    let _home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    
    // create the second instance of struct
    let _loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    
    // we can store a value of an enum without using struct
    // inside the enum we can store any type
    enum IpAddrKind2 {
        V4(String),
        V6(String),
    }
    
    // create the first instance of enum
    let _home2 = IpAddrKind2::V4(String::from("127.0.0.1"));
    // create the second instance of enum
    let _loopback2 = IpAddrKind2::V6(String::from("::1"));
    
    
    // Enhancements in enums
    enum IpAddr2 {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    
    let _home3 = IpAddr2::V4(127, 0, 0, 1);
    let _loopback3 = IpAddr2::V6(String::from("::1"));
}
