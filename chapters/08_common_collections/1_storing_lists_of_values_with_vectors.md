# Storing Lists of Values with Vectors

A ``vector`` can be defined in Rust with the ``Vec<T>`` type.

## Creating a New Vector

To create a new empty vector, we can use the ``Vec::new`` function:

```rust
let v: Vec<i32> = Vec::new();
```

We can also create a vector that contains elements by using the ``vec!`` macro:

```rust
let v = vec![1, 2, 3];
```

## Updating a Vector

We can update a vector by using the ``push`` method to add elements to the end of the vector:

```rust
let mut v = Vec::new();

v.push(5);
v.push(6);
```

## Reading Elements of Vectors

We can read elements of a vector by using the index operator:

```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {}", third);

match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}
```

## Iterating Over Vectors

We can iterate over the elements of a vector by using a ``for`` loop:

```rust
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}
```

Note that ``i`` is a reference to the element of the vector. If we want to modify the elements of the vector, we can use the ``mut`` keyword:

```rust
let mut v = vec![19, 32, 57];
for i in &mut v {
    *i += 50;
}
```

## Using an Enum to Store Multiple Types

We can use an ``enum`` to store multiple types in a vector:

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

## Dropping a Vector Drops Its Elements

When a vector goes out of scope, its elements are also dropped:

```rust
{
    let v = vec![1, 2, 3, 4];
    // do stuff with v
} // <- v goes out of scope and is freed here
```