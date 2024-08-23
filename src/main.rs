//  imports from crates std libraries, or files you define
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    //  generate a random number 1 to 100 inclusive
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // loop till number is guessed
    loop {
        println!("Please input your guess.");

        //  take in guess as a string value (mutable variable) [can be changed]
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //  update guess to be a u32 type number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // Compare guess and secret_number values and print out correct response
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
