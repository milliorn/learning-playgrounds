// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#processing-a-guess
// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#generating-a-random-number
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(0..=9);

    // println!("Secret number is: {secret_number}");

    // https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#allowing-multiple-guesses-with-looping
    loop {
        println!("Input your guess.");

        let mut guess = String::new();

        // https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#receiving-user-input
        // https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#handling-potential-failure-with-result
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#handling-invalid-input
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                // https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#quitting-after-a-correct-guess
                println!("Correct!");
                break;
            }
        }
    }
}
