use rand::Rng;
use std::io::{stdin, stdout, Write};
use utils::Bounds;
use utils::ERROR_READ_LINE;

mod utils {
    use std::io;
    use std::io::{stdin, stdout, ErrorKind, Write};

    pub const ERROR_READ_LINE: &str = "Failed to read line";

    pub(crate) struct Bounds {
        lower: i64,
        upper: i64,
    }

    impl Bounds {
        pub fn from_user() -> Result<Bounds, io::Error> {
            println!("Choose a range of numbers!");
            let mut lower = String::new();
            let mut upper = String::new();
            print!("Lower bound: ");
            stdout().flush().unwrap();
            stdin().read_line(&mut lower).expect(ERROR_READ_LINE);
            print!("Upper bound: ");
            stdout().flush().unwrap();
            stdin().read_line(&mut upper).expect(ERROR_READ_LINE);
            let lower = lower.trim().parse().unwrap_or(0);
            let upper = upper.trim().parse().unwrap_or(100);
            if lower <= upper {
                Ok(Bounds { lower, upper })
            } else {
                Err(io::Error::new(
                    ErrorKind::Other,
                    "The lower bound is higher than the upper bound!",
                ))
            }
        }

        pub fn contains(&self, number: i64) -> bool {
            number >= self.lower && number <= self.upper
        }

        pub fn range(&self) -> u64 {
            (if self.lower >= 0 {
                (self.upper - self.lower) as u64
            } else {
                ((self.lower as i128 - self.upper as i128) * -1) as u64
            }) + 1
        }

        pub fn lower(&self) -> i64 {
            self.lower
        }

        pub fn upper(&self) -> i64 {
            self.upper
        }
    }
}

fn main() {
    let opt_bounds = Bounds::from_user();
    match opt_bounds {
        Err(_) => {
            println!("Could not get a valid range of numbers.")
        }
        Ok(bounds) => {
            let secret_number = rand::thread_rng().gen_range(bounds.lower()..=bounds.upper());
            let attempts = play(&bounds, secret_number);
            let score = bounds.range() as f64 / attempts as f64;
            println!(
                "You guessed it! It took you {} attempt{}, getting you a score of {:.2}",
                attempts,
                if attempts == 1 { "" } else { "s" },
                format!("{:.2}", score).trim_end_matches(['.', '0'])
            );
        }
    }
}

fn play(bounds: &Bounds, secret_number: i64) -> u32 {
    let mut attempts = 0u32;
    let mut guess = String::new();
    println!("Guess the number!");
    loop {
        print!("Your guess: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut guess).expect(ERROR_READ_LINE);
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
