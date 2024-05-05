fn main() {
    /*
    We need to declare this variable as mutable. Otherwise the second assignment will fail!
    */
    let mut x = 5;
    println!("The value of x is: {x}");

    /*
    Since we applied 'mut' to x, we can reuse this variable.
    */
    x = 6;
    println!("The value of x is: {x}");

    /*
    A constant is immutable by design. 
    It cannot gain mutability like a variable declared with let.
    This is consistent with other languages such as JavaScript.
    */
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS}");

    /*
    Shadwing and block level scoping.
    */

    {
        let x = x * 2;
        println!("The value of the x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
    
}
