use clap::Parser;
use colored::Colorize;
use std::error::Error;
use std::fs;
pub type MyResult<T> = Result<T, Box<dyn Error>>;

/// A minimal grep-like search tool
#[derive(Parser, Debug)]
#[command(author, version, about = "A minimal grep-like search tool in Rust", long_about = None)]
pub struct Config {
    /// The pattern to search for
    pub query: String,

    /// The file path to search in
    pub file_path: String,

    /// Perform case-insensitive search
    #[arg(short = 'i', long = "ignore-case")]
    pub ignore_case: bool,

    /// Show_line_numbers
    #[arg(short = 'n', long = "line_number")]
    pub show_line_number: bool,
}

pub fn get_args() -> MyResult<Config> {
    Ok(Config::parse())
}

pub fn run(config: Config) -> MyResult<()> {
    //let mut result: Vec<String> = Vec::new();
    let expected_messege = "no such a file".red().bold().to_string();
    let content = fs::read_to_string(config.file_path).expect(&expected_messege);
    for (index_line, line) in content.lines().enumerate() {
        let is_match = if config.ignore_case {
            line.to_lowercase().contains(&config.query.to_lowercase())
        } else {
            line.contains(&config.query)
        };
        if is_match {
            let line_number = index_line + 1;
            let formated_line =
                line.replace(&config.query, &config.query.blue().bold().to_string());
            if config.show_line_number {
                println!("{}.{}", line_number, formated_line);
            } else {
                println!("{}", formated_line);
            }
        }
    }
    Ok(())
}
