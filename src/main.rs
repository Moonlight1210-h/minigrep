use minigrep::{get_args, run};
use std::process;

fn main() {
    let config = match get_args() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Error parsing arguments: {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
