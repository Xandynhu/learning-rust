# Comments

All programmers strive to make their code easy to understand. One way to do this is to add ``comments`` to our code.

I do not agree that *good code* is always *self-explanatory*. Sometimes, we need to add comments to explain what the code is doing or why. Adding good comments can make our code easier to understand and maintain.

## Single-line comments

Our can create a single-line comment in Rust by starting the line with two slashes `//`. The comment continues until the end of the line.

```rust
fn main() {
    // This is a single-line comment
    println!("Hello, world!");
}
```

## Multi-line comments

For bigger comments, we can simple add more ``//`` to create a multi-line comment, or use ``/*`` and ``*/`` to create a block comment.

```rust
fn main() {
    // This is a
    // multi-line comment
    println!("Hello, world!");

    /*
    This is a block comment
    */
    println!("Hello, cargo!");
}
```
