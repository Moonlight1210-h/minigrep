use clap::Parser;
use std::error::Error;
use std::fs;
pub type MyResult<T> = Result<T, Box<dyn Error>>;

/// A line-oriented search tool
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
}

pub fn get_args() -> MyResult<Config> {
    Ok(Config::parse())
}

pub fn run(config: Config) -> MyResult<()> {
    let mut results: Vec<String> = Vec::new();
    let content = fs::read_to_string(config.file_path)?;
    for line in content.lines() {
        if config.ignore_case {
            if line.to_lowercase().contains(&config.query.to_lowercase()) {
                results.push(line.to_string());
            }
        } else {
            if line.contains(&config.query) {
                results.push(line.to_string());
            }
        }
    }

    for line in results {
        println!("{}", line);
    }
    Ok(())
}
