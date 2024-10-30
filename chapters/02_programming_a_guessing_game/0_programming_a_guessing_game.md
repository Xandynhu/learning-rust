# Chapter 2: Guessing Game in Rust

A simple number guessing game written in Rust.

The user is prompted to guess a randomly generated number between 1 and 100. After each guess, the program gives feedback whether the guess was too high, too low, or correct. It tracks the number of guesses and displays it at the end when the user wins. The player loses if they guess incorrectly 10 times.

## Table of Contents

1. [Importing Crates and Modules](#1-importing-crates-and-modules)
2. [Mutable Variables](#3-mutable-variables)
3. [Loop and Conditionals](#4-loop-and-conditionals)
4. [Reading User Input](#5-reading-user-input)
5. [Error Handling with Result](#6-error-handling-with-result)
6. [Variable Shadowing](#7-variable-shadowing)
7. [Pattern Matching with match](#8-pattern-matching-with-match)
8. [Printing User Guess and Guess Count](#9-printing-user-guess-and-guess-count)

## 1. Importing Crates and Modules
```rust
use rand::Rng;
```

``use rand::Rng``: Rust’s standard library is not as large as some other languages, so some functionalities, like generating random numbers, come from external *libraries*. In Rust, these *libraries* are called ``crates``.

<!-- dropdown with more information about the rand crate -->
<details>
<summary>More about the <b>rand</b> crate</summary>

- The ``rand`` crate provides randomness, and ``Rng`` is a trait that enables random number generation. You can add the ``rand`` crate to your project by adding it to ``Cargo.toml``.

    ```toml
    [dependencies]
    rand = "0.8.4"
    ```

- ``Rng`` is a ``trait`` provided by the ``rand`` crate, which includes the necessary methods for generating random numbers. The ``gen_range`` method is used to generate a random number within a given range.

    ```rust
    let secret_number = rand::thread_rng().gen_range(1..=100);
    ```

<details>
<summary>More about <b>traits</b> in Rust</summary>

For now, let's use ``traits`` as a way to define shared behavior across different types. It allows you to specify a set of methods that a type must implement. ``Traits`` are similar to interfaces in other programming languages.

More about ``traits`` can be found in the official Rust documentation [chapter 10.2](https://doc.rust-lang.org/book/ch10-02-traits.html).

</details>

</details>


## 2. Mutable Variables
```rust
let mut num_guesses = 0;
```

- ``mut`` keyword: By default, variables in Rust are immutable. If you want to change the value of a variable, you need to make it mutable by using the ``mut`` keyword.

## 3. Loop and Conditionals
```rust
while run_game {
    // Loop continues until ``run_game`` becomes false
}
```

``while loop``: The game runs inside an infinite loop (while run_game), which only breaks when the user guesses the correct number (i.e., when run_game is set to false).
Boolean flag: run_game is a bool (boolean) variable that controls the loop. It is set to false when the user guesses the correct number, which ends the loop.

## 5. Reading User Input
```rust
let mut guess = String::new();
let res = std::io::stdin().read_line(&mut guess);
```

``String::new()``: This creates a new empty String to hold the user input.
``std::io::stdin().read_line()``: This reads user input from the standard input (the console).

The input is stored in the guess variable (which is mutable). The result of read_line() is a Result type, which is handled in the following section.

## 6. Error Handling with Result
```rust
if res.is_err() {
    println!("Failed to read line: {}", res.err().unwrap());
    run_game = false;
    continue;
}
```

``Result`` type: In Rust, functions that could fail return a ``Result`` type. A ``Result`` can either be ``Ok`` (successful) or ``Err`` (an error occurred).

``.is_err()``: This checks if the result is an error.

``.unwrap()``: This extracts the ``Err`` value if an error occurs, allowing the program to print the error message.

## 7. Variable Shadowing
```rust
let mut guess = String::new();

// ...some code...

let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => {
        println!("Please type a number!");
        continue;
    }
};
```

``Shadowing``: The variable guess is declared twice: first as a ``String``, then it's "shadowed" by declaring it again as an ``u32`` (an unsigned 32-bit integer).

``match`` statement: This is used for pattern matching. It checks whether the parsing of the string into a number was successful (``Ok``) or resulted in an error (``Err``).

## 8. Pattern Matching with match
```rust
match guess.cmp(&secret_number) {
    std::cmp::Ordering::Less => println!("Too small!"),
    std::cmp::Ordering::Greater => println!("Too big!"),
    std::cmp::Ordering::Equal => {
        println!("You win!");
        run_game = false;
    }
}
```

``cmp()`` method: This method compares two values and returns an Ordering (which can be ``Less``, ``Greater``, or ``Equal``).
