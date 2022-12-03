use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn main() {
    let mut total = 0;
    let lines = lines_from_file("input.txt");
    for line in lines {
        let char = shared_item_type(&line);
        total += priority_for_item_type(char);
    }

    println!("Sum of priorities: {total}");

}

fn priority_for_item_type(type_char: char) -> i32 {
    if 'a' <= type_char && type_char <= 'z' {
        type_char as i32 - 'a' as i32 + 1
    } else {
        type_char as i32 - 'A' as i32 + 27
    }
}

fn shared_item_type(line: &str) -> char {
    let (first_half, second_half) = split_string(line);
    match find_common_char(first_half, second_half) {
        Some(char) => {
            char
        },
        None => panic!("Invalid line, no shared type.")
    }
}

fn split_string(s: &str) -> (&str, &str) {
    let len = s.len();
    let mid = len / 2;

    let first_half = &s[..mid];
    let second_half = &s[mid..];

    (first_half, second_half)
}

fn find_common_char(s1: &str, s2: &str) -> Option<char> {
    for c1 in s1.chars() {
        if s2.contains(c1) {
            return Some(c1);
        }
    }

    None
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
