use clap::Parser;
use std::error::Error;
use std::fs;
pub type MyResult<T> = Result<T, Box<dyn Error>>;
/// A line orinted search tool
#[derive(Parser, Debug)]
#[command(version,about,long_about = None)]

pub struct Config {
    /// The pattern to search for
    #[arg(short = 'q', long = "query")]
    pub query: String,

    /// The file path
    #[arg(short = 'f', long = "file_path")]
    pub file_path: String,

    /// case sensitive search
    #[arg(short = 'i' ,long = "case_insensitive")]
    pub case_sensitive: bool,
}

 

pub fn get_args() -> MyResult<Config> {
    Ok(Config::parse())
}

/// The core logic fun
pub fn run(config: Config) -> MyResult<()> {
    let mut vectory: Vec<String> = Vec::new();
    let to_read = fs::read_to_string(config.file_path)?;
    for line in to_read.lines() {
        if config.case_sensitive {
            if line.contains(&config.query ) {
                vectory.push(line.to_string());
            }
        } else {
            if line.to_lowercase().contains(&config.query.to_lowercase()) {
                vectory.push(line.to_string());
            }
        }
    }
   
    println!("{:#?}", vectory);
    Ok(())
}
