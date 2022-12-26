use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];
    let output_file = &args[2];

    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);
    let mut values: Vec<i32> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let value: i32 = line.parse().unwrap();
        values.push(value);
    }

    values.sort();

    let mut output_file = File::create(output_file).unwrap();
    let mut writer = BufWriter::new(output_file);

    for value in values {
        writeln!(writer, "{} ", value).unwrap();
    }
}
