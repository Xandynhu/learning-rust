# Control Flow

In Rust, just like most programming languages, we can control the flow of our program using what we call ``control flow`` statements.

## Control Flow Statements

1. [``if`` expressions](#if-expressions)
2. [``loop`` expressions](#loop-expressions)
3. [```while``` expressions](#while-expressions)
4. [```for``` expressions](#for-expressions)
5. [```match``` expressions](#match-expressions)

## ``if`` expressions

In Rust, the ``if`` expression works just like in other programming languages.

```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("The number is less than 5");
    } else {
        println!("The number is greater than or equal to 5");
    }
}
```

## ``loop`` expressions

The ``loop`` expression is used to create an infinite loop. So it requires a ``break`` statement to exit the loop at some point.

```rust
fn main() {
    // count to 5
    let mut counter = 0;
    loop {
        println!("This is {}!", counter);

        counter += 1;
        if counter == 5 {
            println!("Done counting to 5");
            break;
        }
    }
}
```

## ``while`` expressions

The ``while`` expression is used to create a loop that runs as long as a condition is true.

```rust
fn main() {
    // count to 5
    let mut counter = 0;
    while counter < 5 {
        println!("This is {}!", counter);

        counter += 1;
    }
    println!("Done counting to 5");
}
```

## ``for`` expressions

The ``for`` expression is used to loop over a collection of items.

```rust
fn main() {
    let numbers = [10, 20, 30, 40, 50];

    // iterative for loop
    for number in numbers.iter() {
        println!("The number is {}", number);
    }

    // range for loop
    for number in 1..6 {
        println!("The number is {}", number);
    }

    // index for loop
    for index in 0..numbers.len() {
        println!("The number is {}", numbers[index]);
    }
}
```

## ``match`` expressions

The ``match`` expression is used to compare a value against a series of patterns and then execute code based on which pattern matches.

```rust
fn main() {
    let number = 3;

    match number {
        1 => println!("The number is one"),
        2 => println!("The number is two"),
        3 => println!("The number is three"),
        _ => println!("The number is something else"),
    }
}
```