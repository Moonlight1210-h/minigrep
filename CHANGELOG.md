# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]


## [0.2.1] - 2026-09-08

### Changed
- Replaced whole-file memory loading (`fs::read_to_string`) with streaming I/O via `BufReader`.
- Optimized standard output printing by locking `stdout` and utilizing `BufWriter` to eliminate repetitive system call overhead.

### Performance Benchmark Results
- Reduced execution time on medium benchmark files from **~13.0s** to **46.5ms** (~280x speedup).
- Streamlined memory consumption to a constant O(1) buffer allocation per line.

## [0.2.0] - 2026-09-06

### Added
- Integrated the `regex` crate using `RegexBuilder` to enable flexible pattern matching across files.
- Case-preserving term highlighting using `RegexBuilder::replace_all` and `Captures`.
- Native support for case-insensitive searching via the `-i, --ignore-case` flag handled dynamically within `RegexBuilder`.
- Comprehensive integration testing suite in `tests/cli.rs` leveraging `assert_cmd`, `predicates`, and `tempfile`.
- Automated release build and installation workflows using `cargo install --path .`.

### Changed
- Replaced literal string searching with regular expression evaluation in line matching routines.
- Consolidated search execution logic inside `run()` to reuse a single `Regex` engine instance for both line matching and string highlighting.
- Refactored `search()` into a pure, functional helper utilizing Rust's `.filter()` and `.collect()` iterators.
- Updated `Config` struct deriving `clap::Parser` to streamline argument handling.

### Benchmark Insights
- Baseline CLI benchmarks conducted using `hyperfine` against standard utilities (`grep` and `ripgrep`) identified terminal I/O and whole-file memory allocations as key performance bottlenecks for future optimization milestones.

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
