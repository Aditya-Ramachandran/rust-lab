use std::io;

fn main() {
    let mut number1 = String::new();
    let mut number2 = String::new();
    println!("Welcome! \nEnter Two Numbers");

    // Accept two nos as the input
    println!("Enter the first number: ");
    io::stdin()
        .read_line(&mut number1)
        .expect("Failed to read line");
    println!("Enter the second number: ");
    io::stdin()
        .read_line(&mut number2)
        .expect("Failed to read line");

    // parse both the numbers
    let number1: i32 = number1.trim().parse().expect("Cannot be parsed");
    let number2: i32 = number2.trim().parse().expect("Cannot be parsed");

    println!("Choose the operation from below:");
    println!("1. Add \n2. Subtract \n3. Multiply \n4. Divide \n5. Exit");

    // Make one variable for the result and assign it to 0
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read the choice");
    let choice = choice.trim();

    loop {
        if choice == "1" {
            println!("Addition");
            result = number1 + number2;
            println!("The addition is {}", result);
            break;
        } else if choice == "2" {
            println!("Subtraction");
            result = number1 - number2;
            println!("The subtraction is {}", result);
            break;
        } else if choice == "3" {
            println!("Multiplication");
            result = number1 * number2;
            println!("The Multiplication is {}", result);
            break;
        } else if choice == "4" {
            println!("Division");
            result = number1 / number2;
            println!("The Division is {}", result);
            break;
        } else if choice == "5" {
            break;
        }
    }
}
