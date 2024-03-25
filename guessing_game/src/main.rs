use std:io

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new(); 
    // We learn in the Rust book that values are immutable by default.
    // In this variable we use mut which sets the variable as mutable.

    io::stdin();
        .read_line(&mut guess);
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
