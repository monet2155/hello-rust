// you can use std::cmp::Ordering, std::io
// or like this
use std::{cmp::Ordering, io};
// rand is crate, which is collection of rust code files
// Rng is trait
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        // let make variables
        // let apples = 5; is also make variable, but immutable(불변)
        // In rust, variable is immutable basically
        let mut guess = String::new();

        io::stdin()
            // & is reference. And reference type is immutable
            // So require to change reference mutable
            .read_line(&mut guess)
            // read_line() returns Result type.
            // Result type is enumeration
            // Result can get Ok or Err, which called variant
            .expect("Failed to read line");

        if guess.trim().eq("quit") {
            break;
        }

        // match guess.cmp(&secret_number) {
        // This line can't compiled.
        // guess is String, but secret_number is integer type(i32)
        // So add this line
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        // rust can declare duplicated name variable, which called shdowing
        // shadowing usually used when want to change type of variable

        // If user input not integer value, continue loop
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // _ is called catch all value
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }
}
