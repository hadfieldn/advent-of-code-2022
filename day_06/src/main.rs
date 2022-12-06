use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path, 
};

fn main() {
    let lines = lines_from_file("input.txt");

    let bytes = lines[0].as_str();

    let mut message_start_index: usize = 0;
    for index in 0..bytes.len()-4 {
        let seq = &bytes[index..index+4];
        if all_unique_chars(seq) {
            message_start_index = index + 4;
            break;
        }
    }
    println!("index: {message_start_index} = {}", &bytes[message_start_index-4..message_start_index]);

}

fn all_unique_chars(seq: &str) -> bool {
    let bytes = seq.as_bytes();
    if bytes.len() < 4 {
        false
    } else {
        let mut has_match = false;
        for current in 0..4 {
            for index in 0..4 {
                if current == index {
                    continue;
                } else {
                    if bytes[current] == bytes[index] {
                        has_match = true;
                        break;
                    }
                }
            }
        }
        !has_match
    }
}
 

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
