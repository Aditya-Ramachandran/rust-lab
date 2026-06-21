use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let mut rng = rand::rng();

    println!("Welcome to Guessing Game V2!");
    println!("Choose your difficulty:");
    println!("1. Easy (1-10)");
    println!("2. Medium (1-100)");
    println!("3. Hard (1-1000)");

    let secret_number: u32;

    loop {
        let mut difficulty = String::new();

        io::stdin()
            .read_line(&mut difficulty)
            .expect("Failed to read difficulty");

        let difficulty = difficulty.trim();

        match difficulty {
            "1" => {
                println!("Easy mode selected!");
                secret_number = rng.random_range(1..11);
                break;
            }
            "2" => {
                println!("Medium mode selected!");
                secret_number = rng.random_range(1..101);
                break;
            }
            "3" => {
                println!("Hard mode selected!");
                secret_number = rng.random_range(1..1001);
                break;
            }
            _ => {
                println!("Invalid choice. Please try again.");
            }
        }
    }

    let mut attempts = 0;

    loop {
        println!("Enter your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read guess");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        attempts += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
            }
            Ordering::Greater => {
                println!("Too big!");
            }
            Ordering::Equal => {
                println!("🎉 You won!");
                println!("Number of attempts: {}", attempts);
                break;
            }
        }
    }
}
