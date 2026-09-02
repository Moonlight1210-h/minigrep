use clap::Parser;
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
    #[arg(short = 'n' , long = "line_number")]
   pub show_line_number : bool
}

pub fn get_args() -> MyResult<Config> {
    Ok(Config::parse())
}

pub fn run(config: Config) -> MyResult<()> {
    let mut results: Vec<String> = Vec::new();
    let content = fs::read_to_string(&config.file_path)?;
    for line in content.lines() {
        if config.ignore_case  {
            if line.to_lowercase().contains(&config.query.to_lowercase()) {
                results.push(line.to_string());
            }
        } else {
            if line.contains(&config.query) {
                results.push(line.to_string());
            }
        }
    }
     if config.show_line_number {
        for (line_num,line) in results.iter().enumerate(){
            println!("{}. {}",line_num+1 ,line)
        } 
     }
     else {   for line in results {
        
        println!("{}", line);
    }}
 
    Ok(())
}
 