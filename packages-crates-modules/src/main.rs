use rand::Rng;
use std::io::{self, stdout, Write};
use std::num::ParseIntError;

const ERROR_READ_LINE: &str = "Failed to read line";

struct Bounds {
    lower: i64,
    upper: i64,
}

impl Bounds {
    fn from_user() -> Bounds {
        // TODO: Quit if values could not be read
        let mut lower = String::new();
        let mut upper = String::new();
        print!("Lower bound: ");
        stdout().flush();
        io::stdin().read_line(&mut lower).expect(ERROR_READ_LINE);
        print!("Upper bound: ");
        stdout().flush();
        io::stdin().read_line(&mut upper).expect(ERROR_READ_LINE);
        Bounds {
            lower: lower.trim().parse().unwrap_or(0),
            upper: upper.trim().parse().unwrap_or(100),
        }
    }

    fn contains(&self, number: i64) -> bool {
        number >= self.lower && number <= self.upper
    }
}

fn main() {
    let bounds = Bounds::from_user();
    let secret_number = rand::thread_rng().gen_range(bounds.lower..=bounds.upper);
    println!("Guess the number!");
    let attempts = play(bounds, secret_number);
    println!(
        "You guessed it! It took you {} attempt{}.",
        attempts,
        if attempts == 1 { "" } else { "s" }
    );
}

fn play(bounds: Bounds, secret_number: i64) -> u32 {
    let mut attempts = 0u32;
    let mut guess = String::new();
    loop {
        print!("Your guess: ");
        stdout().flush();
        io::stdin().read_line(&mut guess).expect(ERROR_READ_LINE);
        attempts += 1;
        // Parse string to number and print msg accordingly
        match guess.trim().parse::<i64>() {
            Err(_) => println!("This is not a number."),
            Ok(guessed_number) if guessed_number == secret_number => break,
            Ok(guessed_number) if !bounds.contains(guessed_number) => {
                println!("This guess is out of bounds.")
            }
            Ok(guessed_number) => println!(
                "Missed!\nHINT: Try with a {} value.",
                if guessed_number < secret_number {
                    "higher"
                } else {
                    "lower"
                }
            ),
        }
        // Clear string, because read_line appends
        guess.clear();
    }
    attempts
}
