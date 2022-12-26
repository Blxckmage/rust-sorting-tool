use colored::*;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("{}", "Error: not enough arguments provided. Please provide an input file and an output file.".red());
        return;
    }
    let input_file = &args[1];
    let output_file = &args[2];

    let file = match File::open(input_file) {
        Ok(file) => file,
        Err(error) => {
            println!(
                "{}",
                format!("Error opening file {}: {}", input_file, error).red()
            );
            return;
        }
    };
    let reader = BufReader::new(file);
    let mut values: Vec<i32> = Vec::new();
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(error) => {
                println!("{}", format!("Error reading line: {}", error).red());
                continue;
            }
        };
        let value: i32 = match line.parse() {
            Ok(value) => value,
            Err(error) => {
                println!("{}", format!("Error parsing line value: {}", error).red());
                continue;
            }
        };
        values.push(value);
    }

    let start_time = Instant::now();

    merge_sort(&mut values);

    let elapsed_time = start_time.elapsed();
    println!("Elapsed time: {:?}", elapsed_time);

    let output_file = match File::create(output_file) {
        Ok(file) => file,
        Err(error) => {
            println!(
                "{}",
                format!("Error creating file {}: {}", output_file, error).red()
            );
            return;
        }
    };
    let mut writer = BufWriter::new(output_file);

    for value in values {
        match writeln!(writer, "{}", value) {
            Ok(_) => (),
            Err(error) => println!("{}", format!("Error writing to file: {}", error).red()),
        }
    }
}

fn merge_sort(values: &mut [i32]) {
    if values.len() > 1 {
        let mid = values.len() / 2;
        let mut left = values[..mid].to_vec();
        let mut right = values[mid..].to_vec();
        merge_sort(&mut left);
        merge_sort(&mut right);
        merge(values, &left, &right);
    }
}

fn merge(values: &mut [i32], left: &[i32], right: &[i32]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            values[k] = left[i];
            i += 1;
        } else {
            values[k] = right[j];
            j += 1;
        }
        k += 1;
    }
    while i < left.len() {
        values[k] = left[i];
        i += 1;
        k += 1;
    }
    while j < right.len() {
        values[k] = right[j];
        j += 1;
        k += 1;
    }
}
