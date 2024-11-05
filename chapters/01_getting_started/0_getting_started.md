# Chapter 1: Getting Started

In this chapter, I'll skip the installation and setup of Rust and Cargo, and focus on how to create a new project, compile and run it.

# Table of Contents

1. [Create a new project](#create-a-new-project)
2. [Compile and run the project](#compile-and-run-the-project)
3. [Hello, Cargo! - Code Overview](#hello-cargo---code-overview)
    1. [Main Function](#1-main-function)
    2. [Printing to the Console](#2-printing-to-the-console)
4. [Summary](#summary)

## Create a new project

We are going to focus on creating a new project using ``Cargo``, as it is the Rust package manager and it is used to easily create, build and manage Rust projects.

To create a new project (and a new directory to hold the project, called ``hello_cargo``), run the following command:

```bash
cargo new hello_cargo
```

To create a new project in the current directory, run the following command:

```bash
cargo init
```

## Compile and run the project

To compile the project, navigate to the project directory and run the following command:

```bash
# Build the project with the ``debug`` profile
cargo build

# To build the project with the ``release`` profile, use the ``--release`` flag
cargo build --release
```

Once the project is built, you can find the executable in the ``target/debug`` or ``target/release`` directory.

```bash
# Run the executable (Windows)
./target/debug/hello_cargo.exe

# Run the executable (Linux)
./target/debug/hello_cargo
```

## Hello, Cargo! - Code Overview

Here is the entire code for the ``hello_cargo`` project:

```rust
fn main() {
    println!("Hello, cargo!");
}
```

### 1. Main Function

```rust
fn main() {
    // Code goes here
}
```

- ``fn main()``: This is the entry point of the Rust program. Every Rust program has a main function, and the execution starts here.
    - ``fn``: This keyword is used to declare a new function.
    - `main`: This is the name of the function.

### 2. Printing to the Console

```rust
println!("Hello, cargo!");
```

- ``println!()``: This is a ``macro`` that prints formatted text to the console. Macros in Rust are identified by the exclamation mark (`!`) after the macro name. The main difference between a macro and a function is that a macro generates code at compile time.

## Summary

In this chapter, we learned how to [create](#create-a-new-project) a new project using ``Cargo``, [compile and run](#compile-and-run-the-project) the project, and a basic overview of the [Hello, Cargo!](#hello-cargo---code-overview) code.
