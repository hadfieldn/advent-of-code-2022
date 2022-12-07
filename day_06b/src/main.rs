use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path, 
};

const MARKER_LEN: usize = 14;

fn main() {
    let lines = lines_from_file("input.txt");

    let bytes = lines[0].as_str();

    let mut message_start_index: usize = 0;
    for index in 0..bytes.len()-MARKER_LEN {
        let seq = &bytes[index..index+MARKER_LEN];
        if all_unique_chars(seq) {
            message_start_index = index + MARKER_LEN;
            break;
        }
    }
    println!("index: {message_start_index} = {}", &bytes[message_start_index-MARKER_LEN..message_start_index]);
}

fn all_unique_chars(s: &str) -> bool {
    let mut chars = s.chars();
    let mut seen_chars = Vec::new();

    while let Some(c) = chars.next() {
        if seen_chars.contains(&c) {
            return false;
        }
        seen_chars.push(c);
    }

    true
}
 

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
