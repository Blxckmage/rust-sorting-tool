use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];

    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);
    let mut values: Vec<i32> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let value: i32 = line.parse().unwrap();
        values.push(value);
    }

    // Sort the values using your sorting algorithm
    values.sort();
    println!("Sorted list of numbers:");
    for value in values {
        println!("- {}", value);
    }
}
