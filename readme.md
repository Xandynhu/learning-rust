# The Rust Programming Language

This repository is dedicated to [The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html) book and its exercises.

Each chapter of the book has its own folder and a `readme.md` file with a brief summary of the chapter and the slved exercises.

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