# Functions

In Rust, and as previously mentioned, functions are declared using the ``fn`` keyword.

One other thing to take note of is that, unlike languages like ``C`` or ``C++``, the order of function definitions does not matter in ``Rust``. You can define a function after it is called.

```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

## Naming conventions

Like variables, functions are ``snake case``. For example, ``another_function`` is a ```valid``` function name.

Keep in mind the this is just the idiomatic way of naming functions in Rust. We can name your functions however you like.

## Parameters

Functions can also have parameters. For example:

```rust
fn main() {
    let x = add(6, 9);
    print_value(5);
}

fn multiply(x: i32, y: i32) -> i32 {
    x * y;
}

fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn print_value(x: i32) {
    println!("The value of x is: {}", x);
}
```

Another thing to notice is that, for ``return`` values, we don't need to use the ``return`` keyword, nor do we need to use a semicolon ``;`` at the end of the expression. The last expression in the function is implicitly returned.

I don't like this feature, since it makes the code somewhat inconsistent. I would prefer to always use the ``return`` keyword with a semicolon ``;`` at the end of the expression. **But that's just me.**
