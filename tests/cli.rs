use assert_cmd::Command;
use predicates::prelude::*;
use std::io::Write;
use tempfile::NamedTempFile;

fn create_tempfile(content: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().expect("faild to create the temp file");
    write!(file, "{}", content).unwrap();
    file
}

#[test]
fn test_cli_search_success() {
    let file = create_tempfile("Rust:\nsafe, fast, productive.\nPick three.");
    let mut cmd = Command::cargo_bin("minigrep").unwrap();
    cmd.arg("productive")
        .arg(file.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("safe, fast, productive"));
}

#[test]
fn test_cli_case_insensitve_flag() {
    let file = create_tempfile("Rust\nsafe ,fast, productive. \nPick three.");
    let mut cmd = Command::cargo_bin("minigrep").unwrap();
    cmd.arg("-i")
        .arg("Rust")
        .arg(file.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Rust"));
}

#[test]
fn test_cli_file_notfound() {
    let mut cmd = Command::cargo_bin("minigrep").unwrap();
    cmd.arg("query")
        .arg("file_not_exist.txt")
        .assert()
        .failure();
}
