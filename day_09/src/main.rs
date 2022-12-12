use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

#[derive(Debug)]
struct Pos {
    row: i32,
    col: i32,
}

struct Movement {
    direction: char,
    count: u32,
}

fn main() {
    let mut head = Pos { row: 0, col: 0 };
    let mut tail = Pos { row: 0, col: 0 };
    let mut visited: Vec<Pos> = vec![Pos { row: 0, col: 0 }];

    let lines = lines_from_file("input.txt");
    for line in lines {
        let movement = parse_move(&line);
        process_movement(&movement, &mut head, &mut tail, &mut visited);
    }
    println!("Number of tail positions: {}", visited.len());
}

fn process_movement(movement: &Movement, head: &mut Pos, tail: &mut Pos, visited: &mut Vec<Pos>) {
    for _ in 0..movement.count {
        let new_head = next_head_pos(movement, head);
        let new_tail = next_tail_pos(tail, &new_head);
        if !visited
            .iter()
            .any(|pos| pos.row == new_tail.row && pos.col == new_tail.col)
        {
            visited.push(Pos {
                row: new_tail.row,
                col: new_tail.col,
            });
        }
        head.row = new_head.row;
        head.col = new_head.col;
        tail.row = new_tail.row;
        tail.col = new_tail.col;
    }
}

fn next_tail_pos(tail: &Pos, head: &Pos) -> Pos {
    let is_adjacent = (head.row - tail.row).abs() <= 1 && (head.col - tail.col).abs() <= 1;
    if is_adjacent {
        // stay
        Pos {
            row: tail.row,
            col: tail.col,
        }
    } else {
        // move at most one step closer in each direction
        Pos {
            row: tail.row + clamp(head.row - tail.row, -1, 1),
            col: tail.col + clamp(head.col - tail.col, -1, 1),
        }
    }
}

fn clamp(value: i32, min: i32, max: i32) -> i32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

fn next_head_pos(movement: &Movement, head: &Pos) -> Pos {
    match movement.direction {
        'U' => Pos {
            row: head.row - 1,
            col: head.col,
        },
        'D' => Pos {
            row: head.row + 1,
            col: head.col,
        },
        'L' => Pos {
            row: head.row,
            col: head.col - 1,
        },
        'R' => Pos {
            row: head.row,
            col: head.col + 1,
        },
        _ => panic!("Invalid direction"),
    }
}

fn parse_move(line: &str) -> Movement {
    let line_entry: Vec<&str> = line.split(" ").collect();
    let direction: char = line_entry[0].chars().nth(0).unwrap();
    let count: u32 = line_entry[1].parse().unwrap();
    Movement {
        direction: direction,
        count: count,
    }
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
