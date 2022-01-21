extern crate rand;

use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    let secret_number: u32 = rand::thread_rng().gen_range(1..101);
    loop {
        println!("Guess the number!\nPlease input your guess.");
        let mut guess: String = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Err: {}", e.to_string());
                continue
            },
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You won!");
                break;
            },
        }

        println!("You guessed: {}", guess);
    }
}
