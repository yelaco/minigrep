//! # Lilgrep
//!
//! `lilgrep` is a collection of basic utilities for searching text within files.

use std::{env, error::Error, fs};

/// Configuration for the minigrep application.
/// Holds the query string, file path, and case sensitivity flag.
/// Use `Config::build` to create a new instance.
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    /// Constructs a `Config` from command line arguments.
    ///
    /// # Arguments
    ///
    /// * `args` - An iterator over command line arguments.
    ///
    /// # Errors
    ///
    /// Returns an error string if the arguments are insufficient or invalid.
    pub fn build(
        mut args: impl DoubleEndedIterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next();
        let mut args = args.rev();

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let mut ignore_case = env::var("IGNORE_CASE").is_ok();

        for arg in args {
            if arg.eq("--ignore-case") {
                ignore_case = true;
            }
        }

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

/// Executes the search based on the provided configuration.
///
/// Reads the file specified in the configuration and searches for the query string.
/// Prints each matching line.
///
/// # Arguments
///
/// * `config` - The configuration specifying the query, file path, and case sensitivity.
///
/// # Errors
///
/// Returns an error if the file cannot be read.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

/// Searches for a query string in the given contents.
///
/// This search is case-sensitive.
///
/// # Arguments
///
/// * `query` - The string to search for.
/// * `contents` - The text to search within.
///
/// # Returns
///
/// A vector of lines that contain the query string.
///
/// # Example
///
/// ```
/// let contents = "Rust:
/// safe, fast, productive.
/// Pick three.";
/// let results = search("duct", contents);
/// assert_eq!(results, vec!["safe, fast, productive."]);
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

/// Searches for a query string in the given contents, ignoring case.
///
/// # Arguments
///
/// * `query` - The string to search for.
/// * `contents` - The text to search within.
///
/// # Returns
///
/// A vector of lines that contain the query string, case-insensitively.
///
/// # Example
///
/// ```
/// let contents = "Rust:
/// safe, fast, productive.
/// Pick three.";
/// let results = search_case_insensitive("rUsT", contents);
/// assert_eq!(results, vec!["Rust:"]);
/// ```
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
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
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
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
