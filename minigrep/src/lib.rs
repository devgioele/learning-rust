use std::error::Error;
use std::{env, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let search = if config.case_sensitive {
        search_case_sensitive
    } else {
        search_case_insensitive
    };

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

/// Documentation comment of `search_case_sensitive`
/// # This is a section
/// # Typical sections
/// ## # Examples
/// ## # Panics
/// ## # Errors
/// ## # Safety
/// # Examples (that serve as tests at the same time)
/// ```
/// use minigrep::search_case_sensitive;
/// let query = "r";
/// let contents = "\
/// Rust:
///
/// What a wonderful day shiny day!
/// Pick three.";
///         assert_eq!(
///             vec!["What a wonderful day shiny day!", "Pick three."],
///             search_case_sensitive(query, contents)
///         );
/// ```
/// Some other explanation.
pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    // `&[String]` is to `Vec` as `&str` is to `String`.
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_sensitive(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
