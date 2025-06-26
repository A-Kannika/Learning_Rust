// Referrers and Borrowing
// Safety and Performance
// Borrowing and References are powerful concepts
//
// Understanding References
// Reference: Enables to borrow a value without taking ownership
// Immutable reference: &T
// Mutable reference: &mut T
// Creating references with the & operator
// Dereferencing references

// Structs: 
// A data structure that allow you to group together multiple fields
// together under a single name

fn main() {
    let _x: i32 = 5;
    // if you use = _x, _x will no longer be valid, 
    // so better use &_x to create a reference
    // _y is not the owner of _x, so it cannot be used to modify _x
    let _y: &i32 = &_x;
    println!("_x: {}, _y: {}", _x, _y);
    
    // make it mutable 
    let mut _x1: i32 = 5;
    let _r: &mut i32 = &mut _x1;
    *_r += 6;
    *_r -= 3;
    // cannot borrow `_x1` as immutable because it is also borrowed as mutable
    // println!("_x1: {}", _x1);
    println!("_r: {}", _r);
    
    //----------------------
    // test BankAccount
    let mut account = BankAccount {
        owner: String::from("John"),
        balance: 155.5,
    };
    
    // Immutable borrow to check balance
    account.check_balance();
    
    // Mutable borrow to withdraw money
    account.withdraw(45.5);

    // Immutable borrow to check balance
    account.check_balance();
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!("Withdrawing {} from {}", amount, self.owner);
        if self.balance >= amount {
            self.balance -= amount;
        }
    }
    
    fn check_balance(&self) {
        println!("Checking balance of {}", self.owner);
        println!("Balance is {}", self.balance);
    }
}
