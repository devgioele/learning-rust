use crate::utils::Ask;
use rand::Rng;
use utils::Bounds;

pub const ERROR_READ_LINE: &str = "Failed to read line";

mod utils {
    use std::io;
    use std::io::{stdin, stdout, ErrorKind, Write};

    pub trait Ask {
        /// Asks the user for the value as long as the entered value is valid.
        fn ask(question: &str) -> Result<Self, io::Error>
        where
            Self: Sized;
    }

    impl Ask for i64 {
        fn ask(question: &str) -> Result<Self, io::Error> {
            let mut answer = String::new();
            loop {
                answer.clear();
                print!("{}", question);
                stdout().flush()?;
                stdin().read_line(&mut answer)?;
                match answer.trim().parse::<i64>() {
                    Err(_) => {
                        println!("I cannot understand this. It should be an integer. Try again!");
                    }
                    Ok(v) => return Ok(v),
                }
            }
        }
    }

    pub(crate) struct Bounds {
        lower: i64,
        upper: i64,
    }

    impl Bounds {
        pub fn from_user() -> Result<Bounds, io::Error> {
            println!("Choose a range of numbers!");
            let lower = i64::ask("Lower bound: ")?;
            let upper = i64::ask("Upper bound: ")?;
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
            let (attempts, score) = play(&bounds, secret_number);
            println!(
                "🎊 You guessed it! It took you {} attempt{}, getting you a score of {}!",
                attempts,
                if attempts == 1 { "" } else { "s" },
                format!("{:.1}", score).trim_end_matches(['.', '0'])
            );
        }
    }
}

fn play(bounds: &Bounds, secret_number: i64) -> (u32, f64) {
    let mut attempts = 0u32;
    println!("Guess the number!");
    loop {
        let guess = i64::ask("Your guess: ");
        attempts += 1;
        // Parse string to number and print msg accordingly
        match guess {
            Err(_) => println!("Something went wrong while reading your input!"),
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
    }
    // Compare the number of attempts with how many divisions a binary search would have taken
    // in the worst case
    let score = (bounds.range() as f64).log(2.0) / attempts as f64 * 100.0;
    (attempts, score)
}
