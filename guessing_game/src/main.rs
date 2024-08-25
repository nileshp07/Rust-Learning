use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please guess a number:");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the input.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("Correct, you won 🎉 ");
                break;
            }
            Ordering::Less => println!("Too small!"),
        }
    }
}
