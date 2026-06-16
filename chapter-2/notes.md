# Chapter 2: Programming a Guessing Game

## Summary

Chapter 2 builds a command-line guessing game and introduces many fundamental Rust concepts for the first time.

The application:

1. Generates a random secret number.
2. Accepts user input.
3. Converts input from a string into a number.
4. Compares the guess with the secret number.
5. Repeats until the correct number is guessed.

Although the project is small, it introduces several core Rust concepts that are explored in greater detail in later chapters.

---

# Key Concepts

## Variables

Variables are immutable by default.

```rust
let x = 5;
```

To allow mutation:

```rust
let mut guess = String::new();
```

Rust encourages immutability because it prevents accidental changes and makes code easier to reason about.

---

## External Crates

Rust projects can use libraries published on crates.io.

Add dependencies inside `Cargo.toml`:

```toml
[dependencies]
rand = "0.8"
```

Cargo automatically downloads and manages dependencies.

---

## User Input

Read input from standard input:

```rust
use std::io;

let mut guess = String::new();

io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
```

Concepts introduced:

* Standard input (`stdin`)
* Mutable references (`&mut`)
* Error handling

---

## Strings

Create an empty string:

```rust
let mut guess = String::new();
```

Remove whitespace:

```rust
guess.trim()
```

Useful because user input contains a newline character.

---

## Error Handling

Handle possible failures using:

```rust
.expect("Failed to read line");
```

If an error occurs:

* Program stops
* Error message is displayed

Example:

```rust
io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
```

---

## Type Conversion

Input is always read as text.

Convert text to a number:

```rust
let guess: u32 = guess
    .trim()
    .parse()
    .expect("Please type a number!");
```

Steps:

```rust
guess.trim()
```

Remove whitespace.

```rust
parse()
```

Convert string into another type.

```rust
: u32
```

Type annotation.

---

## Pattern Matching

Rust uses `match` for branching logic.

```rust
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!"),
}
```

Pattern matching is one of Rust's most powerful features and appears everywhere throughout the language.

---

## Enums

The comparison result is an enum.

```rust
Ordering
```

Possible values:

```rust
Ordering::Less
Ordering::Greater
Ordering::Equal
```

---

## Loops

Infinite loop:

```rust
loop {
}
```

Exit:

```rust
break;
```

Example:

```rust
Ordering::Equal => {
    println!("You win!");
    break;
}
```

---

## Random Number Generation

Generate a random number:

```rust
let secret_number =
    rand::thread_rng().gen_range(1..=100);
```

Components:

```rust
thread_rng()
```

Creates a random number generator.

```rust
gen_range(1..=100)
```

Generates a number from 1 to 100 inclusive.

---

## References

Immutable reference:

```rust
&secret_number
```

Mutable reference:

```rust
&mut guess
```

References become extremely important in Chapter 4 (Ownership).

---

## Traits

The guessing game introduces:

```rust
use rand::Rng;
```

`Rng` is a trait.

Traits are similar to interfaces in Java and define shared behavior.

---

# Important Syntax

## Import Modules

```rust
use std::io;
```

---

## Import Multiple Items

```rust
use std::cmp::Ordering;
use rand::Rng;
```

---

## Mutable Variable

```rust
let mut guess = String::new();
```

---

## Create String

```rust
String::new()
```

---

## Read User Input

```rust
io::stdin().read_line(&mut guess)
```

---

## Parse Number

```rust
guess.trim().parse()
```

---

## Infinite Loop

```rust
loop {
}
```

---

## Match Expression

```rust
match value {
    pattern => action,
}
```

---

## Random Number

```rust
rand::thread_rng().gen_range(1..=100)
```

---

# Java → Rust Comparisons

## Variables

### Java

```java
int guess = 0;
guess = 42;
```

### Rust

```rust
let mut guess = 0;
guess = 42;
```

Difference:

Rust variables are immutable by default.

```rust
let x = 5;
x = 10; // Error
```

Use:

```rust
let mut x = 5;
```

when mutation is needed.

---

## User Input

### Java

```java
Scanner scanner = new Scanner(System.in);
String guess = scanner.nextLine();
```

### Rust

```rust
let mut guess = String::new();

io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
```

Difference:

Rust requires explicit mutable borrowing:

```rust
&mut guess
```

because the function needs permission to modify the variable.

---

## String to Integer Conversion

### Java

```java
int guess = Integer.parseInt(input);
```

### Rust

```rust
let guess: u32 = guess
    .trim()
    .parse()
    .expect("Please type a number!");
```

Difference:

Rust forces you to acknowledge possible failures.

---

## Error Handling

### Java

```java
try {
    int value = Integer.parseInt(input);
} catch (Exception e) {
    System.out.println("Invalid input");
}
```

### Rust

```rust
let value = input
    .trim()
    .parse::<u32>()
    .expect("Invalid input");
```

Difference:

Java uses exceptions.

Rust uses `Result<T, E>`.

---

## Switch vs Match

### Java

```java
switch (value) {
    case 1:
        System.out.println("One");
        break;
    default:
        System.out.println("Other");
}
```

### Rust

```rust
match value {
    1 => println!("One"),
    _ => println!("Other"),
}
```

Difference:

Rust requires all possibilities to be handled.

---

## Loops

### Java

```java
while (true) {
}
```

### Rust

```rust
loop {
}
```

Exit:

```rust
break;
```

---

## Dependencies

### Maven

```xml
<dependency>
    ...
</dependency>
```

### Rust

```toml
[dependencies]
rand = "0.8"
```

| Java          | Rust       |
| ------------- | ---------- |
| Maven         | Cargo      |
| Gradle        | Cargo      |
| pom.xml       | Cargo.toml |
| Maven Central | crates.io  |

---

## Interfaces vs Traits

### Java

```java
interface Runnable {
}
```

### Rust

```rust
trait Rng {
}
```

Traits are Rust's equivalent of interfaces.

---

## Static Methods

### Java

```java
String value = String.valueOf(10);
```

### Rust

```rust
let value = String::new();
```

Rust uses associated functions with `::`.

---

## Null vs Option

### Java

```java
String name = null;
```

### Rust

Rust does not have null references.

Instead, later chapters introduce:

```rust
Option<T>
```

which provides safer handling of missing values.

---

# Common Mistakes

## Forgetting `mut`

Incorrect:

```rust
let guess = String::new();
```

Correct:

```rust
let mut guess = String::new();
```

---

## Forgetting `&mut`

Incorrect:

```rust
.read_line(guess)
```

Correct:

```rust
.read_line(&mut guess)
```

---

## Missing Trait Import

Incorrect:

```rust
rand::thread_rng().gen_range(1..=100);
```

Compiler error if:

```rust
use rand::Rng;
```

is missing.

---

## Forgetting `trim()`

Incorrect:

```rust
guess.parse::<u32>()
```

Correct:

```rust
guess.trim().parse::<u32>()
```

---

## Forgetting Error Handling

Incorrect:

```rust
guess.trim().parse::<u32>();
```

Correct:

```rust
guess.trim().parse::<u32>()
    .expect("Invalid number");
```

---

## Infinite Loop

Incorrect:

```rust
loop {
}
```

without a `break`.

Correct:

```rust
Ordering::Equal => {
    break;
}
```

---

# Command Cheat Sheet

## Create Project

```bash
cargo new guessing_game
```

---

## Build

```bash
cargo build
```

---

## Run

```bash
cargo run
```

---

## Check

```bash
cargo check
```

---

## Add Dependency

Edit:

```toml
Cargo.toml
```

```toml
[dependencies]
rand = "0.8"
```

Then:

```bash
cargo build
```

Cargo downloads dependencies automatically.

---

# Quick Revision

### What crate was introduced?

```rust
rand
```

### Why is `mut` needed?

To allow a variable's value to change.

### What does `trim()` do?

Removes whitespace and newline characters.

### What does `parse()` do?

Converts a string into another type.

### What does `expect()` do?

Stops execution and displays an error message if an operation fails.

### What is `match`?

A pattern matching construct used for branching logic.

### What are the possible values of `Ordering`?

```rust
Ordering::Less
Ordering::Greater
Ordering::Equal
```

### How do you create an infinite loop?

```rust
loop {
}
```

### How do you exit a loop?

```rust
break;
```

---

# Personal Learnings

* This chapter felt significantly harder than Chapter 1.
* Rust introduces many concepts before fully explaining them.
* Reading user input requires mutable references.
* Rust forces explicit type conversion.
* Error handling is explicit rather than exception-based.
* Pattern matching is more powerful than Java's switch statement.
* Traits are similar to interfaces.
* Cargo makes dependency management very simple.
* The compiler is strict but provides excellent guidance.
* Concepts such as borrowing (`&`, `&mut`), traits, enums, and pattern matching will become much clearer in later chapters.

### Biggest Takeaway

Chapter 2 isn't really about building a guessing game.

It's an introduction to:

* Crates
* Cargo
* References
* Borrowing
* Traits
* Enums
* Pattern Matching
* Error Handling

Many of the "weird" things encountered here are explained in detail later, especially in Chapters 4, 6, and 10.
