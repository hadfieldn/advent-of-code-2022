use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    ops::Range,
};
use regex::Regex;

fn main() {
    let lines = lines_from_file("input.txt");
    let mut count = 0;
    for line in lines {
        let (range_1, range_2) = parse_line(&line);
        count += if fully_contains(range_1, range_2) {1} else {0};
    }
    println!("Count: {count}")
}

fn fully_contains(range_1: Range<u32>, range_2: Range<u32>) -> bool {
    let range_1_contains = range_1.start <= range_2.start && range_1.end >= range_2.end;
    let range_2_contains = range_2.start <= range_1.start && range_2.end >= range_1.end;
    range_1_contains || range_2_contains
}
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn parse_line(line: &str) -> (Range<u32>, Range<u32>) {
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let caps = re.captures(line).unwrap();
    let range_1 = Range {
        start: caps[1].parse::<u32>().unwrap(),
        end: caps[2].parse::<u32>().unwrap(),
    };
    let range_2 = Range {
        start: caps[3].parse::<u32>().unwrap(),
        end: caps[4].parse::<u32>().unwrap(),
    };
    (range_1, range_2)
}