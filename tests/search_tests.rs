use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use minigrep::{Config, run};

fn write_temp(content: &str) -> PathBuf {
    let mut path = env::temp_dir();
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    path.push(format!("minigrep_test_{}.txt", nanos));
    fs::write(&path, content).expect("failed to write temp file");
    path
}

#[test]
fn case_sensitive_search() {
    let content = "Rust is great\nI love Rust\nthis is rust";
    let path = write_temp(content);

    let config = Config {
        query: "Rust".to_string(),
        file_path: path.to_string_lossy().into_owned(),
        ignore_case: false,
    };

    assert!(run(config).is_ok());
}

#[test]
fn case_insensitive_search() {
    let content = "Rust is great\nI love rust\nthis is RUST";
    let path = write_temp(content);

    let config = Config {
        query: "rust".to_string(),
        file_path: path.to_string_lossy().into_owned(),
        ignore_case: true,
    };

    assert!(run(config).is_ok());
}
