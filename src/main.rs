#![allow(unused)] // turns of the warning for unused variables during development/learning

use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    // 1) Generates a secret random number between 1 and 10
    let secret_number = rand::thread_rng().gen_range(1..11);


    // 2) Asks user to guess a number
    println!("Macks awesome guessing game!");

    println!("Please input your guess.");
    //loop until the user wins
    loop {

        // 3) Stores the users guess as a string
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // 4) Verifies that the user input and returns number
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! The secret number is: {}", secret_number);
                break;
            }
        }
    }
    // Game over message
    println!("\nGame over!\n");
}