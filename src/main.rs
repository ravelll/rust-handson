// ref: https://speakerdeck.com/qnighy/rustsu-xi-hui-1

// declaration to use a external library.
// (Obsolete in Rust2018)
extern crate rand;

use std::io;
// It's enum that represents the result of comparison.
use std::cmp::Ordering;
// It's needed to call the trait method "Rnd".
use rand::Rng;

fn main() {
    // A type is must needed in constant declaration.
    const MAX_NUM: u32 = 101;

    // "!" represents macro call
    println!("Guess the number!");

    let secret_number =
        // gen_range() is the method defined in the "Rng" trait.
        rand::thread_rng().gen_range(1, MAX_NUM);

    loop {
        println!("Please input your guess.");

        // "mut" means mutable (default: immutable)
        // "new" has no mean (conventional)
        let mut guess = String::new();

        // == std::io::stdin()
        // io module's stdin function, it returns a Stdin type handle.
        // "&" gets the reference to the variable.
        io::stdin().read_line(&mut guess)
            // Result type method.
            //   Ok(..) => returns the number of characters.
            //   Err(..) => returns the reason for failure.
            // In this line, the number of characters is put off.
            // If Result is ignored, it's compilable but warned.
            .expect("Failed to read line");

        // Shadowing
        // up to here, "guess" points String::new()
        // from here, "guess" points the u32 typed one.
        //
        // Rust programming has the convension that
        // naming variables under processing same name.
        let guess: u32 =
            // trin() removes trailing line break.
            // parse() returns Result typed value,
            // because parsing is errorable.
            match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

        println!("You guessed: {}", guess);

        // It checks completeness.(網羅性)
        match guess.cmp(&secret_number) {
            Ordering::Less =>
                println!("Too small!"),
            Ordering::Greater =>
                println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; 
            }
        }
    }
}
