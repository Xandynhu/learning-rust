# Data Types

In Rust, every value has a `data type`, which tells Rust what kind of data that value is associated with.

Rust has two main categories of data types: `scalar` and `compound`.

Also, Rust is a `statically typed` language, which means that it must know the types of all variables at `compile time`.

## Scalar Types

A `scalar` type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.

```rust
fn main() {
    let integer = 5;     // Integer
    let float = 6.9;     // Floating-point
    let boolean = true;  // Boolean
    let character = 'a'; // Character
}
```

## Compound Types

A `compound` type can group multiple values into one type. Rust has two primitive compound types: `tuples` and `arrays`.

```rust
fn main() {
    let tuple = (1, 2.0, 'a');   // Tuple
    let array = [1, 2, 3, 4, 5]; // Array
}
```

### Tuples

A `tuple` is a general way of grouping together a number of values with a variety of types into one compound type.

```rust
fn main() {
    let tuple = (1, 2.0, 'a');
    let (x, y, z) = tuple;

    println!("x: {}, y: {}, z: {}", x, y, z);
}
```

### Arrays

An `array` is a collection of elements of the same type stored in contiguous memory.

```rust
fn main() {
    let array = [1, 2, 3, 4, 5];
    let first = array[0];
    let second = array[1];

    println!("first: {}, second: {}", first, second);
}
```
