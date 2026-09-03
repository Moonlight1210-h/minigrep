# minigrep

A small command-line search tool inspired by the classic `grep`, implemented in Rust.

This repository implements a powerful CLI tool for searching text in files with support for case-insensitive searches, line numbering, and colored output.

## Features

- **Case-insensitive search**: Use the `-i` or `--ignore-case` flag to perform case-insensitive searches
- **Line numbers**: Display line numbers with the `-n` or `--line-number` flag
- **Colored output**: Search results are highlighted with colored output for better visibility
- **Fast and efficient**: Built in Rust for performance and reliability
- **CLI powered by clap**: Modern, well-documented command-line interface using the `clap` crate

## Requirements

- Rust and Cargo (https://www.rust-lang.org/tools/install)

## Getting the repository

Clone the repository:

```bash
git clone https://github.com/Moonlight1210-h/minigrep.git
cd minigrep
```

## Building

To build the project in release mode:

```bash
cargo build --release
```

You can also run it directly with Cargo (useful during development):

```bash
cargo run -- <query> <path>
```

## Usage

The CLI follows the pattern:

```text
minigrep [OPTIONS] <query> <path>
```

**Positional arguments:**
- `query`: The text to search for
- `path`: Path to the file to search in

**Options:**
- `-i, --ignore-case` — Perform case-insensitive search
- `-n, --line-number` — Display line numbers with results
- `-h, --help` — Print help information
- `-V, --version` — Print version information

### Examples

**Basic case-sensitive search:**
```bash
cargo run -- Rust examples/poem.txt
```

**Case-insensitive search:**
```bash
cargo run -- -i rust examples/poem.txt
```

**Show line numbers:**
```bash
cargo run -- -n Rust examples/poem.txt
```

**Combine flags (case-insensitive + line numbers):**
```bash
cargo run -- -i -n rust examples/poem.txt
```

**After building in release mode, run the binary directly:**
```bash
./target/release/minigrep hello poem.txt
```

**Install locally with Cargo and run as a system command:**
```bash
cargo install --path .
minigrep world poem.txt
```

## Testing

Run tests with:

```bash
cargo test
```

## Project Structure

```
minigrep/
├── src/              # Source code
├── examples/         # Example files and usage scenarios
├── Cargo.toml        # Project manifest
├── README.md         # This file
└── Makefile          # Build automation (optional)
```

## Version

Current version: 0.1.2

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to:
- Open an issue to report bugs or suggest features
- Submit a pull request with improvements or new features

For major changes, please open an issue first to discuss what you would like to change.