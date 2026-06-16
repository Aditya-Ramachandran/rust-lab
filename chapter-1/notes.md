# Chapter 1: Getting Started

## Summary

Chapter 1 introduces Rust and its tooling. You learn how to install Rust, write your first program, compile code using `rustc`, and use Cargo, Rust's build system and package manager. The chapter emphasizes Cargo as the standard way to create, build, and manage Rust projects.

---

# Key Concepts

## Rust Compiler (`rustc`)

The Rust compiler converts Rust source code into a native executable.

Example:

```bash
rustc main.rs
```

This produces an executable that can be run directly.

---

## Entry Point

Every executable Rust program starts from the `main` function.

```rust
fn main() {
    println!("Hello, world!");
}
```

---

## Macros

`println!` is a macro, not a function.

```rust
println!("Hello, world!");
```

Macros are identified by the `!` character.

---

## Cargo

Cargo is Rust's official:

* Build system
* Package manager
* Dependency manager

It simplifies project creation, compilation, testing, and dependency management.

Create a new project:

```bash
cargo new hello_cargo
```

---

## Project Structure

A Cargo project contains:

```text
hello_cargo/
├── Cargo.toml
└── src/
    └── main.rs
```

### `Cargo.toml`

Contains:

* Package metadata
* Rust edition
* Dependencies

Example:

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2024"

[dependencies]
```

---

# Important Syntax

## Function Definition

```rust
fn main() {
}
```

* `fn` declares a function.
* `main` is the program entry point.

---

## Printing Output

```rust
println!("Hello, world!");
```

Prints text to the console.

---

## Statement Terminator

```rust
println!("Hello");
```

Most statements end with a semicolon (`;`).

---

## Macro Invocation

```rust
println!("Text");
```

Macros use `!`.

---

# Common Mistakes

## Forgetting the `!` in `println!`

Incorrect:

```rust
println("Hello");
```

Correct:

```rust
println!("Hello");
```

---

## Missing Semicolon

Incorrect:

```rust
println!("Hello")
println!("World");
```

Correct:

```rust
println!("Hello");
println!("World");
```

---

## Editing Files Outside `src/`

Cargo expects source files inside the `src` directory.

Correct:

```text
src/main.rs
```

---

## Using `rustc` for Cargo Projects

Once a project is created with Cargo, prefer:

```bash
cargo run
```

instead of manually compiling with:

```bash
rustc main.rs
```

---

## Forgetting Release Mode

For optimized production builds:

```bash
cargo build --release
```

instead of:

```bash
cargo build
```

---

# Command Cheat Sheet

## Check Rust Installation

```bash
rustc --version
```

---

## Create a New Project

```bash
cargo new project_name
```

---

## Build Project

```bash
cargo build
```

---

## Run Project

```bash
cargo run
```

---

## Check for Compilation Errors

```bash
cargo check
```

---

## Create Optimized Build

```bash
cargo build --release
```

---

## Compile a Single Rust File

```bash
rustc main.rs
```

---

# Quick Revision

### What is Rustup?

Tool used to install and manage Rust toolchains.

### What is Cargo?

Rust's build system and package manager.

### What is the entry point of a Rust program?

```rust
fn main()
```

### How do you print to the console?

```rust
println!("Hello");
```

### How do you create a new Cargo project?

```bash
cargo new my_project
```

### Which command is fastest for checking code during development?

```bash
cargo check
```

### Where are release builds stored?

```text
target/release/
```

### How are macros identified?

By the `!` character:

```rust
println!("Hello");
```
