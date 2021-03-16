use std::io; // input/output lib - get user input and print output
use std::cmp::Ordering; // comparison libd
use rand::Rng; // random number generation

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        // the variable containing user input
        // variables are immutable by default
        // mut makes it mutable
        // :: = associated function
        // creates new empty string
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

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
