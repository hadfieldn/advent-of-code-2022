use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn compute_totals(lines: Vec<String>) -> Vec<u32> {
    // append a final empty string to force calculating the last group
    let mut lines: Vec<String> = lines.clone();
    lines.push("".to_string());

    let mut totals = Vec::new();
    let mut current_total = 0;
    for line in lines {
        if line == "" {
            totals.push(current_total);
            current_total = 0;
        }
        else {
            let line_value: u32 = line.parse().expect("Numeric string expected");
            current_total += line_value;
        }
    }
    return totals;
}

fn max_value(values: Vec<u32>) -> u32 {
    let mut max: u32 = 0;
    for value in values {
        if value > max {
            max = value;
        }
    }
    return max;
}

fn main() {
    let lines = lines_from_file("input.txt");
    let totals = compute_totals(lines);
    println!("Max: {}", max_value(totals));
}

