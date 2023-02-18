use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess a number!");

    // Generate random number
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Loop until the user guesses the secret number.
    loop {
        // This variable will store the secret number
        let mut guess = String::new();

        println!("Please input your guess:");

        // Get the input from the user
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Make sure the input is a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Compare the input and the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{guess} is too small!"),
            Ordering::Greater => println!("{guess} is too high!"),
            Ordering::Equal => {
                println!("{guess} Your guess is correct!");
                break;
            }
        }
    }
}
