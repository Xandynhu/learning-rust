# The Rust Programming Language

This repository is dedicated to [The Rust Programming Language](https://doc.rust-lang.org/book/) book and its exercises.

It was created to help me learn Rust and to keep track of my progress while reading the book. For the official documentation, please visit the official [Learn Rust](https://www.rust-lang.org/learn) page.

## Official Learning Resources
- [Learn Rust](https://www.rust-lang.org/learn)
    - [Book - The Rust Programming Language](https://doc.rust-lang.org/book/)
    - [Book - Rust by Example](https://doc.rust-lang.org/rust-by-example/)
    - [The Rustlings Course](https://github.com/rust-lang/rustlings/)

## Project Structure

Each one of the chapters and subchapters of the book will have their own dedicated directory and a `readme.md` file with a brief summary of the chapter, the solved exercises, and some notes I might have taken while reading the book or solving the problems.

## Building and running the project's exercises

In the `chapter 1` we will see how we get started with Rust, how to create a new project, compile and run it. But for the rest of the chapters, we will always build and run the exercises from the project's root directory like the example below:

```bash
# Make sure we are in the project's root directory
cd <path>/<to>/the_rust_programming_language

# Build the project with the ``debug`` profile
cargo build

# Run the executable
./target/debug/<executable_name>
```

## Table of Contents

1. [Getting Started](./chapters/01_getting_started/0_getting_started.md)
    - [[code example](./chapters/01_getting_started/hello_cargo/src/main.rs)]
2. [Programming a Guessing Game](./chapters/02_programming_a_guessing_game/0_programming_a_guessing_game.md)
    - [[code example](./chapters/02_programming_a_guessing_game/guessing_game/src/main.rs)]
3. [Common Programming Concepts](./chapters/03_common_programming_concepts/0_common_programming_concepts.md)
    1. [Variables and Mutability](./chapters/03_common_programming_concepts/1_variables_and_mutability.md)
    2. [Data Types](./chapters/03_common_programming_concepts/2_data_types.md)
    3. [Functions](./chapters/03_common_programming_concepts/3_functions.md)
    4. [Comments](./chapters/03_common_programming_concepts/4_comments.md)
    5. [Control Flow](./chapters/03_common_programming_concepts/5_control_flow.md)
4. [Understanding Ownership](./chapters/04_understanding_ownership/0_understanding_ownership.md)
    1. [What is Ownership?](./chapters/04_understanding_ownership/1_what_is_ownership.md)
    2. [References and Borrowing](./chapters/04_understanding_ownership/2_references_and_borrowing.md)
    3. [The Slice Type](./chapters/04_understanding_ownership/3_the_slice_type.md)
5. [Using Structs to Structure Related Data](./chapters/05_using_structs_to_structure_related_data/0_using_structs_to_structure_related_data.md)
    1. [Defining and Instantiating Structs](./chapters/05_using_structs_to_structure_related_data/1_defining_and_instantiating_structs.md)
    2. [An Example Program Using Structs](./chapters/05_using_structs_to_structure_related_data/2_an_example_program_using_structs.md)
    3. [Method Syntax](./chapters/05_using_structs_to_structure_related_data/3_method_syntax.md)
        - [[code example](./chapters/05_using_structs_to_structure_related_data/method_syntax/src/main.rs)]
6. [Enums and Pattern Matching](./chapters/06_enums_and_pattern_matching/0_enums_and_pattern_matching.md)
    1. [Defining an Enum](./chapters/06_enums_and_pattern_matching/1_defining_an_enum.md)
    2. [The `match` Control Flow Construct](./chapters/06_enums_and_pattern_matching/2_the_match_control_flow_construct.md)
    3. [Consice Control Flow with `if let`](./chapters/06_enums_and_pattern_matching/3_consice_control_flow_with_if_let.md)
7. [Managing Growing Projects with Packages, Crates, and Modules](./chapters/07_managing_growing_projects/0_managing_growing_projects.md)
    1. [Packages and Crates](./chapters/07_managing_growing_projects/1_packages_and_crates.md)
    2. [Defining Modules to Control Scope and Privacy](./chapters/07_managing_growing_projects/2_defining_modules_to_control_scope_and_privacy.md)
    3. [Paths for Referring to an Item in the Module Tree](./chapters/07_managing_growing_projects/3_paths_for_referring_to_an_item_in_the_module_tree.md)
    4. [Bringing Paths into Scope with the `use` Keyword](./chapters/07_managing_growing_projects/4_bringing_paths_into_scope_with_the_use_keyword.md)
    5. [Separating Modules into Different Files](./chapters/07_managing_growing_projects/5_separating_modules_into_different_files.md)