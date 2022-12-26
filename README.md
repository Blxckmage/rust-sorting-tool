# Rust Sorting Tools

This repository contains a command line tool written in Rust that can sort a list of integers in a text file and write the sorted list to another text file.

## Prerequisites

- Rust 1.50 or higher

## Installing

Clone the repository and build the tool using `cargo build --release`.

## Usage

An example input file, `input.txt`, is included in the `./test-file-examples` directory. To use this file as input, run the following command:
`cargo run test-file-examples\numbers.txt output_file.txt`

Replace `output_file.txt` with the desired name for the output file. The output file will contain the sorted list of integers from the `input.txt` file, one per line.
