use std::io;
use rand::Rng;

fn main() {
    let mut rng = rand::rng();

    println!("Welcome to guessing game V2");

    println!("Choose your difficulty!");
    println!("1. Easy (1-10)");
    println!("2. Medium (1-100)");
    println!("3. Hard (1-1000)");

    let mut difficulty = String::new();
    let number_choice_1 = rng.random_range(1..11);
    let number_choice_2 = rng.random_range(1..101);
    let number_choice_3 = rng.random_range(1..1001);

    let mut number_of_guess: i16 = 0;

    io::stdin().read_line(&mut difficulty).expect("Could not parse your choice!");
    let difficulty = difficulty.trim();

    loop {
    match difficulty {
        "1" => {
            println!("Enter your guess!");
            number_of_guess += 1;
            let mut user_choice = String::new();
            io::stdin().read_line(&mut user_choice).expect("Could not parse your number!");
            let user_choice: u32 = user_choice.trim().parse().expect("could not parse into a number");
            if user_choice == number_choice_1 {
                println!("You have WON! In {} guesses", number_of_guess);
                break;
            } else {
                println!("please keep guessing!");
            }
        }
        "2" => {
            println!("you have chosen medium difficulty mode");
        }
        "3" => {
            println!("you have chosen the hard difficulty mode");
        }
        _ => {
            println!("wrong choice!!");
            println!("Enter your choice again!");
            io::stdin().read_line(&mut difficulty).expect("Could not parse your choice!");
            let difficulty = difficulty.trim();
        }
    }
}

}



/* 
Loop 1
│
├── Ask difficulty until valid
│
└── Generate secret number

Loop 2
│
├── Ask guess
├── Compare
├── Count attempts
└── Break when won
*/