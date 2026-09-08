use clap::Parser;
use colored::Colorize;
use regex::RegexBuilder;
use std::error::Error;
use std::fs;

pub type MyResult<T> = Result<T, Box<dyn Error>>;

/// A minimal grep-like search tool
#[derive(Parser, Debug)]
#[command(author, version, about = "A minimal grep-like search tool built in Rust", long_about = None)]
pub struct Config {
    /// The pattern to search for
    pub query: String,

    /// The file path to search in
    pub file_path: String,

    /// Perform case-insensitive search
    #[arg(short = 'i', long = "ignore-case")]
    pub ignore_case: bool,

    /// Show line numbers
    #[arg(short = 'n', long = "line-number")]
    pub show_line_number: bool,
}

pub fn get_args() -> MyResult<Config> {
    Ok(Config::parse())
}

pub fn run(config: Config) -> MyResult<()> {
    let content = fs::read_to_string(&config.file_path)?;

    let re = RegexBuilder::new(&config.query)
        .case_insensitive(config.ignore_case)
        .build()?;

    for (index, line) in content.lines().enumerate() {
        if re.is_match(line) {
            let line_number = index + 1;

            let formatted_line = re.replace_all(line, |caps: &regex::Captures| {
                caps[0].blue().bold().to_string()
            });

            if config.show_line_number {
                println!("{}.{}", line_number, formatted_line);
            } else {
                println!("{}", formatted_line);
            }
        }
    }

    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str, ignore_case: bool) -> MyResult<Vec<&'a str>> {
    let re = RegexBuilder::new(query)
        .case_insensitive(ignore_case)
        .build()?;

    Ok(content.lines().filter(|line| re.is_match(line)).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regex_search_exact() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, content, false).unwrap()
        );
    }

    #[test]
    fn regex_case_insensitive() {
        let query = "rUsT";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search(query, content, true).unwrap()
        );
    }
}
