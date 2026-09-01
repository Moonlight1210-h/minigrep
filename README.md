# minigrep

A small command-line search tool inspired by the classic `grep`, implemented in Rust.

This repository was updated to use the `clap` crate for command-line parsing.

## What's new

- Replaced manual argument parsing with `clap` to provide a clear, well-documented CLI.
- The program accepts the search query and the file path as positional arguments and provides an optional `--ignore-case`/`-i` flag to perform case-insensitive searches.

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

Positional arguments:
- query: The text to search for.
- path: Path to the file to search in.

Options:
- -i, --ignore-case    Perform case-insensitive search
- -h, --help           Print help information (provided by clap)

Examples:

Search for the word `foo` in `poem.txt`:

```bash
cargo run -- foo poem.txt
```

Case-insensitive search for `rust` in `README.md`:

```bash
cargo run -- -i rust README.md
```

After building in release mode you can run the binary directly:

```bash
./target/release/minigrep hello poem.txt
```

Or install locally with Cargo and run as a normal command:

```bash
cargo install --path .
minigrep world poem.txt
```

## Testing

Run tests with:

```bash
cargo test
```

## Contributing

Contributions are welcome. Please open an issue or a pull request with a clear description of changes.

## License

Include your preferred license here (e.g., MIT or Apache 2.0).
