use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn main() {
    let lines = lines_from_file("input.txt");
    let grid = grid_from_lines(lines);

    let (num_rows, num_cols) = grid_dims(&grid);

    let mut max_score: u32 = 0;
    for row_idx in 0..num_rows {
        for col_idx in 0..num_cols {
            let score = scenic_score(row_idx, col_idx, &grid);
            if score > max_score {
                max_score = score;
            }
        }
    }
    println!("Max scenic score: {max_score}");
}

fn scenic_score(cur_row: usize, cur_col: usize, grid: &Vec<Vec<u32>>) -> u32 {
    let (num_rows, num_cols) = grid_dims(grid);
    let cur_height = grid[cur_row][cur_col];
    let mut scores: Vec<u32> = vec![];
    let mut score: u32;

    // up
    score = 0;
    if cur_row > 0 {
        for row_idx in (0..cur_row).rev() {
            score += 1;
            if grid[row_idx][cur_col] >= cur_height {
                break;
            }
        }
    }
    scores.push(score);

    // down
    score = 0;
    if cur_row < num_rows - 1 {
        for row_idx in cur_row + 1..num_rows {
            score += 1;
            if grid[row_idx][cur_col] >= cur_height {
                break;
            }
        }
    }
    scores.push(score);

    // left
    score = 0;
    if cur_col > 0 {
        for col_idx in (0..cur_col).rev() {
            score += 1;
            if grid[cur_row][col_idx] >= cur_height {
                break;
            }
        }
    }
    scores.push(score);

    // right
    score = 0;
    if cur_col > 0 {
        for col_idx in cur_col + 1..num_cols {
            score += 1;
            if grid[cur_row][col_idx] >= cur_height {
                break;
            }
        }
    }
    scores.push(score);

    scores.iter().product::<u32>()
}

fn grid_dims(grid: &Vec<Vec<u32>>) -> (usize, usize) {
    (grid.len(), grid[0].len())
}

fn grid_from_lines(lines: Vec<String>) -> Vec<Vec<u32>> {
    let mut grid: Vec<Vec<u32>> = vec![];
    for line in lines {
        let sizes: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();

        grid.push(sizes)
    }
    grid
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
