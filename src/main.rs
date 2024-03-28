use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::{self, Colorize};

fn main() {
    println!("Guess a number: ");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

        let guess:u32 = guess.trim().parse().expect("Please type a valid number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("{}", "Too small!".red()),
        Ordering::Greater => println!("{}", "Too big!".red()),
        Ordering::Equal => println!("{}", "You won!".green()),
    }
}
