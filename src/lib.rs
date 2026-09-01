use clap::Parser;
use std::error::Error;
pub type MyResult<T> = Result<T, Box<dyn Error>>;
/// A line orinted search tool
#[derive(Parser, Debug)]
#[command(version,about,long_about = None)]

struct Config {
    /// The pattern to search for
    #[arg(short = 'q', long = "query")]
    query: String,

    /// The file path
    #[arg(short = 'f', long = "file_path", default_value = None)]
    file_path: String,

    /// case sensitive search
    #[arg(short = 'i', default_value = "false")]
    case_sensitive: bool,
}

fn get_args() -> MyResult<Config> {
    Ok(Config::parse())
}

/// The core logic fun
pub fn run()
