use clap::Parser;
use colored::Colorize;
use regex::RegexBuilder;
use std::error::Error;
use std::fs;
use std::io::{self, BufRead, BufReader, BufWriter, IsTerminal, Write};

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

pub fn run(config: Config) -> MyResult<()> {
    let file = fs::File::open(&config.file_path)?;
    let reader = BufReader::new(file);

    let stdout = io::stdout();
    let mut handle = BufWriter::new(stdout.lock());

    // فحص ما إذا كان المخرج تفاعلياً (TTY) أم لا
    let is_tty = stdout.is_terminal();

    let re = RegexBuilder::new(&config.query)
        .case_insensitive(config.ignore_case)
        .build()?;

    reader
        .lines()
        .enumerate()
        .filter_map(|(index, line_result)| {
            let line = line_result.ok()?;
            if re.is_match(&line) {
                Some((index + 1, line))
            } else {
                None
            }
        })
        .try_for_each(|(line_number, line)| -> io::Result<()> {
            // التلوين فقط إذا كان المخرج TTY
            let output_line = if is_tty {
                re.replace_all(&line, |caps: &regex::Captures| {
                    caps[0].blue().bold().to_string()
                })
                .to_string()
            } else {
                line // طباعة النص الأصلي دون أي تعديل أو استهلاك أداء
            };

            if config.show_line_number {
                writeln!(handle, "{}.{}", line_number, output_line)
            } else {
                writeln!(handle, "{}", output_line)
            }
        })?;

    handle.flush()?;
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
