# minigrep Examples

This directory contains example files and usage scenarios for `minigrep`.

## Example Text File

A sample text file `poem.txt` is provided in this directory for testing and demonstration:

```text
Rust:
safe, fast, productive.
Pick three.
Trust me.

Usage Examples

Run these commands from the project root directory:
1. Basic Case-Sensitive Search

Search for exact occurrences of the word "Rust":
Bash

cargo run -- Rust examples/poem.txt

2. Case-Insensitive Search (-i)

Search for "rust" regardless of uppercase/lowercase letter variations:
Bash

cargo run -- -i rust examples/poem.txt

3. Show Line Numbers (-n)

Display line numbers alongside the matched results:
Bash

cargo run -- -n Rust examples/poem.txt

4. Combining Flags (-i and -n)

Perform a case-insensitive search and show line numbers with highlighted colored output:
Bash

cargo run -- -i -n "rust" examples/poem.txt

5. Display Help Information

To see all available CLI flags and options:
Bash

cargo run -- --help