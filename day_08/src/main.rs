use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn main() {
    let lines = lines_from_file("input.txt");
    let grid = grid_from_lines(lines);

    let (num_rows, num_cols) = grid_dims(&grid);

    let mut visible_count = 0;
    for row_idx in 0..num_rows {
        for col_idx in 0..num_cols {
            let visible = is_visible(row_idx, col_idx, &grid);
            if visible {
                visible_count += 1;
            }
            println!(
                "({row_idx}, {col_idx}) = {} ({})",
                if visible { "T" } else { "F" },
                grid[row_idx][col_idx]
            );
        }
    }
    println!("Visible tree count: {visible_count}");
}

fn is_visible(row: usize, col: usize, grid: &Vec<Vec<u32>>) -> bool {
    match (row, col) {
        (0, _) => true,
        (_, 0) => true,
        (cur_row, cur_col) => {
            let (num_rows, num_cols) = grid_dims(grid);
            let cur_height = grid[cur_row][cur_col];
            let mut visible;

            // up
            visible = true;
            for row_idx in 0..cur_row {
                if grid[row_idx][cur_col] >= cur_height {
                    visible = false;
                    break;
                }
            }
            if visible {
                return true;
            };

            // down
            visible = true;
            for row_idx in cur_row + 1..num_rows {
                if grid[row_idx][cur_col] >= cur_height {
                    visible = false;
                    break;
                }
            }
            if visible {
                return true;
            };

            // left
            visible = true;
            for col_idx in 0..cur_col {
                if grid[cur_row][col_idx] >= cur_height {
                    visible = false;
                    break;
                }
            }
            if visible {
                return true;
            };

            // right
            visible = true;
            for col_idx in cur_col + 1..num_cols {
                if grid[cur_row][col_idx] >= cur_height {
                    visible = false;
                    break;
                }
            }

            visible
        }
    }
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
