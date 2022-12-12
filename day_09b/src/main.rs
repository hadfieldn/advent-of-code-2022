use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

#[derive(Clone, Debug)]
struct Pos {
    row: i32,
    col: i32,
}

struct Movement {
    direction: char,
    count: u32,
}

fn main() {
    let mut knots: Vec<Pos> = vec![Pos { row: 0, col: 0 }; 10];
    let mut visited: Vec<Pos> = vec![Pos { row: 0, col: 0 }];

    let lines = lines_from_file("input.txt");
    for line in lines {
        let movement = parse_move(&line);
        process_movement(&movement, &mut knots, &mut visited);
    }
    println!("Number of tail positions: {}", visited.len());
}

fn process_movement(movement: &Movement, knots: &mut Vec<Pos>, visited: &mut Vec<Pos>) {
    for _ in 0..movement.count {
        let new_head = next_head_pos(movement, &knots[0]);
        knots[0].row = new_head.row;
        knots[0].col = new_head.col;

        for knot_idx in 1..knots.len() {
            let next_knot = next_knot_pos(&knots[knot_idx], &knots[knot_idx - 1]);
            knots[knot_idx].row = next_knot.row;
            knots[knot_idx].col = next_knot.col;
        }
        let last = knots.last().unwrap();
        if !visited
            .iter()
            .any(|pos| pos.row == last.row && pos.col == last.col)
        {
            visited.push(Pos {
                row: last.row,
                col: last.col,
            });
        }
    }
}

fn next_knot_pos(cur_pos: &Pos, leader_pos: &Pos) -> Pos {
    let is_adjacent =
        (leader_pos.row - cur_pos.row).abs() <= 1 && (leader_pos.col - cur_pos.col).abs() <= 1;
    if is_adjacent {
        // stay
        Pos {
            row: cur_pos.row,
            col: cur_pos.col,
        }
    } else {
        // move at most one step closer in each direction
        Pos {
            row: cur_pos.row + clamp(leader_pos.row - cur_pos.row, -1, 1),
            col: cur_pos.col + clamp(leader_pos.col - cur_pos.col, -1, 1),
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
