use rand::Rng; // brings Rng trait into scope
use std::cmp::Ordering;
use std::io; // brings io library into scope

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin() // :: indicates that new is an associated function of the stdin type
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input!!");
                continue;
            },
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
