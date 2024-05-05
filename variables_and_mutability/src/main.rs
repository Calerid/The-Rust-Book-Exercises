fn main() {
    // We need to declare this variable as mutable. Otherwise the second assignment will fail!
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // A constant is immutable by design. 
    // It cannot gain mutability like a variable declared with let.
    // This is consistent with other languages such as JavaScript.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    
    println!("{THREE_HOURS_IN_SECONDS}")
}
