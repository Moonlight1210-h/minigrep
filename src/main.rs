use minigrep::{get_args, run};
use std::process;

fn main() {
    let config = get_args().unwrap();

    if let Err(e) = run(config) {
        eprintln!("there was an error runing the applecation {e}");
        process::exit(1);
    }
}
