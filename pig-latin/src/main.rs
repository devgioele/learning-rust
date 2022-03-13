/* --- Requirements ---
Convert strings to pig latin. The first consonant of each word is moved to the end of the word and
 “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to
 the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
 */
use std::io;
use std::io::Write;

const ERROR_READ_LINE: &str = "Line could not be read!";

enum Letter {
    Consonant(char),
    Vowel(char),
}

impl Letter {
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

    fn from(letter: char) -> Option<Letter> {
        // If the letter is not ASCII, it cannot be a vowel
        if Letter::VOWELS.contains(&letter.to_ascii_lowercase()) {
            Some(Letter::Vowel(letter))
        } else if !letter.is_numeric() {
            Some(Letter::Consonant(letter))
        } else {
            None
        }
    }
}

fn main() {
    println!("->> Pig-latin translator");
    println!("Enter some text to get its pig-latin translation.");
    let mut text = String::new();
    loop {
        print!("Your text: ");
        io::stdout().flush();
        io::stdin().read_line(&mut text).expect(ERROR_READ_LINE);
        for word in text.split_whitespace() {
            // Print the translated word or the original word, if no translation is possible
            print!(
                "{} ",
                match pig_latin(word) {
                    Some(translation) => translation,
                    None => word.to_string(),
                }
            )
        }
        println!();
        // Clear string, because `read_line` appends
        text.clear();
    }
}

// Returns the pig-latin translation of the given word.
// If the word is empty, the returned string is empty as well.
// If the word is a single consonant, the leading hyphen is removed.
// If the given word cannot be translated, None is returned.
fn pig_latin(word: &str) -> Option<String> {
    let mut chars = word.chars();
    let first_char = chars.clone().next();
    match first_char {
        Some(c) => {
            if contains_numbers(word) {
                None
            } else {
                let prefix = match Letter::from(c) {
                    Some(Letter::Consonant(_)) | None => {
                        // Exclude first char from word
                        chars.next();
                        c
                    }
                    Some(Letter::Vowel(_)) => 'h',
                };
                let rem_word = chars.collect::<String>();
                Some(format!(
                    "{}{}{}ay",
                    rem_word,
                    if rem_word.len() > 0 { "-" } else { "" },
                    prefix
                ))
            }
        }
        None => Some("".to_string()),
    }
}

fn contains_numbers(text: &str) -> bool {
    text.chars().any(|c| c.is_numeric())
}
