# minigrep

A high-performance, command-line search tool inspired by the classic `grep`, implemented in production-grade Rust.

This repository features a feature-rich CLI tool for searching text using regular expressions with support for case-insensitive matching, line numbering, term highlighting, and robust integration testing.

## Features

- **Regex Search Engine**: Full regular expression pattern matching driven by the linear-time `regex` crate (DFA/NFA-based, immune to ReDoS attacks).
- **Case-Insensitive Matching**: Perform case-insensitive regular expression searches using `-i` or `--ignore-case`.
- **Case-Preserving Highlighting**: Matches are accurately highlighted in bold blue while preserving the original character casing in the file.
- **Line Numbering**: Display line numbers with the `-n` or `--line-number` flag.
- **Modern CLI**: Built with `clap` (v4) for fast, type-safe argument parsing, complete with help menus and version flags.
- **System-Wide Installation**: Easily installable as a global executable utility using `cargo install --path .`.

## Requirements

- **Rust & Cargo**: 1.70.0 or higher (https://www.rust-lang.org/tools/install)

## Installation

Clone the repository:

```bash
git clone [https://github.com/Moonlight1210-h/minigrep.git](https://github.com/Moonlight1210-h/minigrep.git)
cd minigrep
