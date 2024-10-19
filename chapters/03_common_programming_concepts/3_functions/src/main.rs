// Idiomatic declaration of a function in Rust.
fn another_function() {
    println!("Another function.");
}

// Function names can start with a capital letter.
// This is not idiomatic in Rust. It will work, but the compiler will warn you.
// fn anotherFunction2() {
//     println!("Another function.");
// }

fn main() {
    println!("Hello, world!");

    // call the function.
    another_function();
    // anotherFunction2();

    // function with parameters.
    let mut x = add(6, 9);
    x = multiply(x, 2);
    print_value(x);
}

// The return value can ommit the return keyword.
fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

// The return keyword, as well as the semicolon, are optional.
fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn print_value(x: i32) {
    println!("The value of x is: {}", x);
}
