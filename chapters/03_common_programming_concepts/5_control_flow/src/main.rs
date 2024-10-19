fn main() {
    // if expressions
    println!("If expressions:");
    let number = 3;
    if number < 5 {
        println!("Condition was true!");
    } else {
        println!("Condition was false!");
    }

    // loops
    println!("\nLoops:");
    let mut counter = 0;
    loop {
        println!("This is {}!", counter);

        counter += 1;
        if counter == 5 {
            println!("Done counting to 5");
            break;
        }
    }

    // while loops
    println!("\nWhile loop:");

    // for loops
    let numbers = [10, 20, 30, 40, 50];

    // iterative for loop
    println!("Iterative for loop:");
    for number in numbers.iter() {
        println!("The number is {}", number);
    }

    // range for loop
    println!("\nRange for loop:");
    for number in 1..6 {
        println!("The number is {}", number);
    }

    // index for loop
    println!("\nIndex for loop:");
    for index in 0..numbers.len() {
        println!("The number is {}", numbers[index]);
    }

    // match
    println!("\nMatch:");
    let number = 3;

    match number {
        1 => println!("The number is one"),
        2 => println!("The number is two"),
        3 => println!("The number is three"),
        _ => println!("The number is something else"),
    }

}