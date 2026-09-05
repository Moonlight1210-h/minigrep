# minigrep

A small command-line search tool inspired by the classic `grep`, implemented in Rust.

minigrep is a tiny, fast, and ergonomic CLI for searching text in files. It's written in Rust for safety and performance and uses modern libraries for command-line parsing and colored output.

## Highlights

- Case-insensitive search (with `-i` / `--ignore-case`)
- Optional line numbers (`-n` / `--line-number`)
- Colored output for matches
- Fast and memory-safe implementation powered by Rust
- Command-line parsing with `clap` crate

## Requirements

- Rust and Cargo: https://www.rust-lang.org/tools/install

## Get the repository

Clone the repository and enter the project directory:

```bash
git clone https://github.com/Moonlight1210-h/minigrep.git
cd minigrep
```

## Build

Build in release mode:

```bash
cargo build --release
```

Run directly with Cargo (useful during development):

```bash
cargo run -- <query> <path>
```

After building in release mode, run the binary directly:

```bash
./target/release/minigrep <query> <path>
```

You can also install the crate locally and run it as a system command:

```bash
cargo install --path .
minigrep <query> <path>
```

## Usage

```text
minigrep [OPTIONS] <query> <path>
```

Positional arguments:
- `query` — the text or pattern to search for
- `path` — file or directory path to search in (files only in the current implementation)

Options:
- `-i, --ignore-case` — perform a case-insensitive search
- `-n, --line-number` — show line numbers for matches
- `-h, --help` — print help information
- `-V, --version` — print version information

## Examples

Basic case-sensitive search:

```bash
cargo run -- Rust examples/poem.txt
```

Case-insensitive search:

```bash
cargo run -- -i rust examples/poem.txt
```

Show line numbers:

```bash
cargo run -- -n Rust examples/poem.txt
```

Combine flags (case-insensitive + line numbers):

```bash
cargo run -- -i -n rust examples/poem.txt
```

After installing with Cargo, run the binary directly:

```bash
minigrep hello examples/poem.txt
```

## Testing

Run the test suite with:

```bash
cargo test
```

## Project structure

```
minigrep/
├── src/              # Source code
├── examples/         # Example files and usage scenarios
├── Cargo.toml        # Project manifest
├── README.md         # This file
└── Makefile          # Build automation (optional)
```

## Version

Current version: 0.2.0

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Contributing

Contributions are welcome. Open an issue to discuss major changes before submitting a pull request. For small fixes or improvements, feel free to open a pull request directly.
