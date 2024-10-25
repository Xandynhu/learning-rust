# An Exemple Program Using Structs

To better understand how structs work, let's create a simple program that uses them to calculate the area of a rectangle.

<div style="display: flex;">
<div style="margin-right: 20px;">

```rust
// Without using structs
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("The area is {}.", area(width1, height1));
}

fn area(width: u32, height: u32) -> u32 {
    return width * height;
}
```

</div>
<div>

```rust
// Using structs
// Define a struct named 'Rectangle'
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area is {}.", area(&rect1));
}

fn area(rect: &Rectangle) -> u32 {
    return rect.width * rect.height;
}
```

</div>
</div>

## Adding Useful Functionality with Derived Traits

It would be useful to print the `Rectangle` struct to see its values while debugging the program. To do this, we can derive the `Debug` trait for the struct.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:?}"); // prints:  rect1 is Rectangle { width: 30, height: 50 }

    println!("rect1 is {rect1:#?}"); // prints: rect1 is Rectangle {
                                    //            width: 30,
                                    //            height: 50,
                                    //          }
}
```
