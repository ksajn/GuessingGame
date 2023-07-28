use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("Guess number!");

    let num = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 =  match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        println!("You guessed: {}", guess);
        match guess.cmp(&num) {
            Ordering::Less => println!("{}", "Try bigger number".red()),
            Ordering::Greater => println!("{}", "Try smaller number".red()),
            Ordering::Equal => {
                println!("{}", "You won!".green());
                break;
            },
        }
    }
}