
/*
fn three_hours_in_seconds() -> u32 {
    60 * 60 * 3
}

// This will throw a compilor error
const MAX_POINTS = 100_000;
const THREE_HOURS_IN_SECONDS: u32 = three_hours_in_seconds();
*/

// This will work
const MAX_POINTS: u32 = 100_000;
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // variables are immutable by default
    println!("Immutable variable x = 5");
    let x = 5;
    println!("1. The value of x is: {x}\n");

    // declare a mutable variable with `mut` keyword
    println!("Mutable variable y = 5");
    let mut y = 5;
    println!("2. The value of y is: {y}\n");

    println!("Changing the value of y to 6");
    y = 6;
    println!("3. The value of y is: {y}\n");

    // constants from the global scope
    println!("Constants from the global scope");
    println!("4. MAX_POINTS: {}", MAX_POINTS);
    println!("5. THREE_HOURS_IN_SECONDS: {}\n", THREE_HOURS_IN_SECONDS);

    // shadowing
    println!("Shadowing a variable z = \"Hello\" from string to usize");
    let z = String::from("Hello");
    println!("6. The value of z-string is: {z}");
    let z = z.len();
    println!("7. The value of z-usize is: {z}\n");

    // shadowing with a block
    println!("Shadowing a variable z = 5 from usize to usize+1 in a block");
    if z > 1 {
        let z = z + 1;
        println!("8. The value of z is: {z}\n");
    }

    println!("Shadowing \"back\" to the original value of z = 5");
    println!("9. The value of z is: {z}");
}