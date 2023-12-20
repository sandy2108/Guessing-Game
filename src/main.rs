use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut attempts = 0;

    loop {
        println!("Please input your guess (or type 'quit' to exit):");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        // Allow the user to quit
        if guess.trim().to_lowercase() == "quit" {
            println!("Quitting the game. The secret number was: {}", secret_number);
            break;
        }

        // Handle non-numeric input
        let guess: Result<u32, _> = guess.trim().parse();
        let guess = match guess {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        attempts += 1;

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                println!("Number of attempts: {}", attempts);
                break;
            }
            Ordering::Greater => println!("{}", "Too big!".red()),
        }
    }
}

