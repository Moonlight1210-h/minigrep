# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.2] - 2026-09-03

### Added
- Colored terminal output for matched query patterns, line numbers, and file paths using the `colored` crate.
- `-n, --line-number` CLI flag to optionally display line numbers for search results.
- Unit testing module (`mod tests`) in `src/lib.rs` using `assert_eq!` for pure search functions (`search` and `search_case_insensitive`).

### Changed
- Refactored `run` logic to delegate core matching to dedicated, testable search functions.
- Streamlined line output formatting to process results on the fly using `.enumerate()`.

### Improved
- CLI ergonomics with `clap` positional argument parsing and updated `--help` documentation.