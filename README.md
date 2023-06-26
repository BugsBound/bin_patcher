# BIN Patcher

BIN Patcher is a simple Rust-based program for patching BIN files.

![Demo](./demo.gif)

## Description

The program allows altering data in a given file, writing bytes to a specified memory address, and supports ELF format.

Author: @BugsBound

## Installation

You can download the ready-to-use binary from the releases section, or compile the program yourself.

To compile, you will need [Rust](https://www.rust-lang.org/).

Clone the repository:

```
git clone https://github.com/BugsBound/bin_patcher.git
```

Change to the project directory:

```
cd bin_patcher
```

Compile the project:

```
cargo build --release
```

## Usage

The program takes three mandatory arguments: file path (`-f` or `--file`), bytes for writing (`-b` or `--bytes`), and memory address for overwriting (`-a` or `--address`).

Example of use:

```
bin_patcher -f /path/to/your/file.bin -b deadbeef -a 0x400
```

## License

This project is licensed under the MIT License - see the `LICENSE` file for details.

## Call to the Community

This project is open for community contribution. We welcome any ideas and suggestions for improving functionality or bug fixes. If you would like to contribute, feel free to create pull requests.