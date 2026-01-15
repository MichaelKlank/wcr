# wcr - Word Count in Rust

A Rust implementation of the Unix `wc` command-line tool, built as part of the [Coding Challenges](https://codingchallenges.fyi/challenges/challenge-wc) series.

## Overview

`wcr` is a word, line, character, and byte count utility that follows the Unix philosophy of doing one thing well. It can read from files or standard input, making it perfect for use in shell pipelines.

## Features

- **Count bytes** (`-c`): Count the number of bytes in the input
- **Count lines** (`-l`): Count the number of lines in the input
- **Count words** (`-w`): Count the number of words in the input
- **Count characters** (`-m`): Count the number of characters in the input
- **Default mode**: When no options are provided, displays lines, words, and bytes
- **Standard input support**: Reads from stdin when no filename is provided
- **Zero external dependencies**: Uses only Rust's standard library

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024)

### Build from Source

```bash
git clone git@github.com:MichaelKlank/wcr.git
cd wcr
cargo build --release
```

The binary will be located at `target/release/wcr`.

### Install Locally

```bash
cargo install --path .
```

## Usage

### Basic Usage

Count lines, words, and bytes (default):
```bash
wcr file.txt
```

### Command Line Options

Count bytes:
```bash
wcr -c file.txt
```

Count lines:
```bash
wcr -l file.txt
```

Count words:
```bash
wcr -w file.txt
```

Count characters:
```bash
wcr -m file.txt
```

### Multiple Options

You can combine multiple options:
```bash
wcr -l -w file.txt
```

### Reading from Standard Input

Read from stdin:
```bash
cat file.txt | wcr -l
```

Or:
```bash
echo "Hello World" | wcr
```

## Examples

```bash
# Default output (lines, words, bytes)
$ wcr test.txt
    7145   58164  342190 test.txt

# Count lines only
$ wcr -l test.txt
    7145 test.txt

# Count words only
$ wcr -w test.txt
   58164 test.txt

# Count bytes only
$ wcr -c test.txt
  342190 test.txt

# Count characters only
$ wcr -m test.txt
  339292 test.txt

# Read from stdin
$ echo "Hello World" | wcr
    1       2      12
```

## Implementation Details

This implementation is written in idiomatic Rust and follows best practices:

- **Type safety**: Uses `Option<T>` instead of magic values
- **Error handling**: Proper error handling with `Result<T, E>`
- **Structured data**: Uses structs to organize related data
- **Iterator methods**: Leverages Rust's iterator API for efficiency
- **Zero dependencies**: Only uses Rust's standard library

## Testing

Run the test suite:
```bash
cargo test
```

## About the Challenge

This project was created as part of the [Build Your Own wc Tool](https://codingchallenges.fyi/challenges/challenge-wc) coding challenge. The challenge encourages developers to build their own version of Unix command-line tools to understand how they work and to practice good software engineering principles.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Acknowledgments

- Inspired by the [Coding Challenges](https://codingchallenges.fyi) series
- Based on the Unix `wc` command
