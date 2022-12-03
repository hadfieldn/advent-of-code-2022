use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn main() {
    let lines = lines_from_file("input.txt");
    let mut total = 0;
    for line in lines {
        let points = match line.as_str() {
            // X = lose, Y = draw, Z = win
            "A X" => 3 + 0,
            "A Y" => 1 + 3,
            "A Z" => 2 + 6,

            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,

            "C X" => 2 + 0,
            "C Y" => 3 + 3,
            "C Z" => 1 + 6,
            &_ => panic!("Invalid line")

        };
        total += points;
    }
    println!("Total: {total}");
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
