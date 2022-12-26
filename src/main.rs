use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("../test-files/numbers.txt").unwrap();
    let reader = BufReader::new(file);
    let mut values: Vec<i32> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let value: i32 = line.parse().unwrap();
        values.push(value);
    }

    values.sort();

    println!("{:?}", values);
}
