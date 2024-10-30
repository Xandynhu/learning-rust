# Variables and Mutability

As mentioned before, variables in Rust are ``immutable`` by default. This means that once a value is assigned to a variable, it cannot be changed. We can still make a variable ``mutable`` by using the ``mut`` keyword along with the ``let`` keyword and the variable name.

```rust
// This will throw an error
let x = 5;
x = 6;

// This will work
let mut y = 5;
y = 6;
```

## Naming Conventions

Rust uses `snake_case` for variable names.

```rust
let variable = 6;
let another_variable = 9;
```

## Constants

Constants are declared using the ``const`` keyword. Constants are **always** immutable and must be annotated with a ``type``. Trying to use ``mut`` with a constant will result in a compilation error.

The difference between constants and variables is that:

1. Constants are ``always immutable``
2. Constants can be declared in ``any scope``, including the ``global scope``
3. Constants can only be set to a ``constant expression``, it must be a value that can be computed at ``compile time``.

```rust
fn three_hours_in_seconds() -> u32 {
    60 * 60 * 3
}

// This will throw an error
const MAX_POINTS = 100_000;
const THREE_HOURS_IN_SECONDS: u32 = three_hours_in_seconds();

// This will work
const MAX_POINTS: u32 = 100_000;
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

## Shadowing

Shadowing is a feature that allows us to redeclare a variable with the same name.

This is useful when we want to change the type of a variable or change the value of a variable.

Or when we want to reuse the variable name inside a block without having to create a new variable name and without changing the variable outside the block. Personally, I find this feature very bad practice and I would recommend avoiding it, since it can lead to confusion when reading the code.

```rust
println!("Shadowing a variable z = \"Hello\" from string to usize");
let z = String::from("Hello"); // z is a string with value "Hello"
let z = z.len();               // z is a usize with value 5

// shadowing with a block
if z > 1 {
    let z = z + 1;             // here the value of z is 6
}

// here the value of z comes back to 5
```
