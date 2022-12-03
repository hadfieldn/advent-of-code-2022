use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn main() {
    let mut total = 0;
    let lines = lines_from_file("input.txt");
    let mut lines = lines.iter();
    loop {
        let pack1 = match lines.next() {
            Some(line) => line,
            None => {
                break;
            }
        };
        let pack2 = match lines.next() {
            Some(line) => line,
            None => {
                break;
            }
        };
        let pack3 = match lines.next() {
            Some(line) => line,
            None => {
                break;
            }
        };
        total += match find_common_char(pack1, pack2, pack3) {
            Some(shared_item) => priority_for_item_type(shared_item),
            None => panic!("No common item found in group"),
        }
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

fn find_common_char(s1: &str, s2: &str, s3: &str) -> Option<char> {
    for c1 in s1.chars() {
        if s2.contains(c1) && s3.contains(c1) {
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
