# The Rust Programming Language

This repository is dedicated to [The Rust Programming Language](https://doc.rust-lang.org/book/) book and its exercises.

It was created to help me learn Rust and to keep track of my progress while reading the book. For the official documentation, please visit the official [Learn Rust](https://www.rust-lang.org/learn) page.

## Official Learning Links
- [Learn Rust](https://www.rust-lang.org/learn)
    - [Book - The Rust Programming Language](https://doc.rust-lang.org/book/)
    - [Book - Rust by Example](https://doc.rust-lang.org/rust-by-example/)
    - [The Rustlings Course](https://github.com/rust-lang/rustlings/)

## Project Structure

Each chapter and sub-chapter of the book will have their own dedicated directory and a `readme.md` file with a brief summary of the chapter, the solved exercises, and some notes I might have taken while reading the book or solving the problems.

## Building and running the project's exercises

In the `chapter 1` we will see how we get started with Rust, how to create a new project, compile and run it. But for the rest of the chapters, we will always build and run the exercises from the project's root directory like the example below:

```bash
# Make sure we are in the project's root directory
cd <path>/<to>/the_rust_programming_language

# Build the project with the `debug` profile
cargo build

# Run the executable
./target/debug/<executable_name>
```

## Table of Contents

1. [Getting Started](./chapters/01_getting_started/readme.md)
2. [Programming a Guessing Game](./chapters/02_programming_a_guessing_game/readme.md)
3. [Common Programming Concepts](./chapters/03_common_programming_concepts/readme.md)
    1. [Variables and Mutability](./chapters/03_common_programming_concepts/1_variables_and_mutability/readme.md)
