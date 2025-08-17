use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("Welcome to Guess the number game!");
    let total_attempts = 5;
    println!("You have {total_attempts} attempts to guess the correct number and win!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut counter = 0;
    loop {
        println!("Please enter a number between 1 to 100.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input!");
                continue;
            }
        };

        println!("You guessed: {guess}");

        counter += 1;
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                if counter >= total_attempts {
                    println!("You've used all your attempts! The secret number was: {secret_number}");
                    break;
                }
                println!("Too small! You have {} more attempts left", total_attempts - counter);
            }
            Ordering::Greater => {
                if counter >= total_attempts {
                    println!("You've used all your attempts! The secret number was: {secret_number}");
                    break;
                }
                println!("Too big! You have {} more attempts left", total_attempts - counter);
            }
            Ordering::Equal => {
                println!("Congratulations! You win!");
                break;
            }
        }
    }

    println!("Game over! Thanks for playing!");
}
