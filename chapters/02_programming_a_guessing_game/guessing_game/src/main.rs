use rand::Rng;

fn main() {
    println!("Guess the number!");

    // generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // keep track of the number of guesses
    let mut num_guesses = 0;

    // loop until the user guesses the number
    let mut run_game = true;
    while run_game {

        // print a prompt to the user
        print!("Please input your guess: ");

        // print! macro does not flush the output buffer, so we need to do it manually to ensure the prompt is displayed before reading the user's input
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        // read the user's guess from the console
        let mut guess = String::new();
        let res = std::io::stdin().read_line(&mut guess);
        if res.is_err() {
            println!("Failed to read line: {}", res.err().unwrap());
            run_game = false;
            continue;
        }

        // parse the user's guess as an integer
        // apply the concept of shadowing to reuse the guess variable name, now as an u32 type
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        // print the user's guess
        num_guesses += 1;
        println!("You guessed: {guess}");

        // compare the user's guess to the secret number
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => {
                println!("Too small!");
            },
            std::cmp::Ordering::Greater => {
                println!("Too big!");
            },
            std::cmp::Ordering::Equal => {
                println!("You win!");
                run_game = false;
                continue;
            }
        }
    }

    // print the number of guesses
    println!("You took {} guesses", num_guesses);
}
