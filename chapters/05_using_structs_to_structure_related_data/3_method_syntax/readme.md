# Method Syntax

`Methods` are similar to functions: we declare them with the `fn` keyword and a name. They can also have parameters and a return value. However, unlike functions, `methods` are defined within the context of a `struct` (or an `enum` or a `trait` object - which will be cover on [chapter 6](./../../06_enums_and_pattern_matching/readme.md)).

## Table of Contents

1. [Defining Methods](#defining-methods)
2. [Where is the `->` Operator?](#where-is-the-`->`-operator)
3. [Methods with More Parameters](#methods-with-more-parameters)

## Defining Methods

Here is an example of a method named `area` that calculates the area of a rectangle:

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area is {}.", rect1.area());
}
```

The `impl` block is where we define methods for a `struct`. In the example above, we define one method named `area` that takes a reference to `self` (we'll explain this in the next section) and returns a `u32`.

## Where is the `->` Operator?

In `C` and `C++`, two different operators are used for calling methods: `.` and `->`. The `.` operator is used when we have a struct instance and want to call a method on it. The `->` operator is used when we have a pointer to a struct and want to call a method on it.

In `Rust`, we only have the `.` operator. `Rust` automatically takes care of the `->` operator for us with its `automatic reference and dereferencing` feature.

## Methods with More Parameters

Methods can take more parameters than just `self`. Here is an example of a method that takes another parameter:

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.area() > other.area();
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```