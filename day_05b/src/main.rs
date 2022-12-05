use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path, 
    slice::Iter,
};
use regex::Regex;

struct Movement {
    count: u32,
    from: usize,
    to: usize,
}

fn main() {
    let lines = lines_from_file("input.txt");
    let (mut stacks, move_rows) = init_stacks(&lines);
    print_stacks(&stacks);

    for move_row in move_rows {
        let movement = parse_movement(move_row);
        make_move(movement, &mut stacks);
        print_stacks(&stacks);
    }
    println!("{:#?}", String::from_iter(get_top_crates(&stacks)));
}

fn print_stacks(stacks: &Vec<Vec<char>>) {
    let mut found_crate: bool = true;
    let mut crate_level = 0;
    while found_crate {
        found_crate = false;
        for stack in stacks {
            match stack.get(crate_level) {
                Some(crate_label) => {
                    print!("{crate_label} ");
                    found_crate = true;
                },
                None => print!("  ")
            }
        }
        crate_level += 1;
        println!("");
    }
    println!("")
}

fn get_top_crates(stacks: &Vec<Vec<char>>) -> Vec<&char> {
    stacks.iter().map(|stack| match stack.last() {
        Some(char) => char,
        None => &' ',
    }).collect()
}

fn make_move(movement: Movement, stacks: &mut Vec<Vec<char>>) {
    // Preserve the order of the entire movement, as if we moved the
    // entire pile at once, rather than reverse the order, as if we 
    // moved them one at a time. 
    // 
    // We do this by moving the crates one at a time to an intermediate 
    // stack, and then one at a time to the target stack. This has the
    // effect of twice reversing the order of the crates to be moved,
    // resulting in them having the original ordering.

    let mut movement_stack: Vec<char> = Vec::new();
    for _ in 0..movement.count {
        match stacks[movement.from].pop() {
            Some(crate_label) => movement_stack.push(crate_label),
            None => (),
        }
    }
    for _ in 0..movement.count {
        match movement_stack.pop() {
            Some(crate_label) => stacks[movement.to].push(crate_label),
            None => (),
        }
    }
}

fn parse_movement(row: &String) -> Movement {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+) *").unwrap();
    let caps = re.captures(row).unwrap();

    Movement {
        count: caps[1].parse::<u32>().unwrap(),
        from: caps[2].parse::<usize>().unwrap() - 1,
        to: caps[3].parse::<usize>().unwrap() - 1,
    }
    
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn init_stacks(lines: &Vec<String>) -> (Vec<Vec<char>>, Vec<&String>) {
    let mut iter = lines.iter();
    let mut stacks: Vec<Vec<char>> = Vec::new();

    let (stack_rows, stack_labels_row) = get_stack_def_rows(&mut iter);
    init_empty_stack_vectors(&stack_labels_row, &mut stacks);

    // read initial crate assignments into into stack vectors
    for stack_row in stack_rows.iter().rev() {
        read_stack_row(stack_row, &mut stacks);
    }

    // skip empty line after stack definition rows
    iter.next(); 

    // init move_rows with all remaining rows
    let mut move_rows: Vec<&String> = Vec::new();
    loop {
        match iter.next() {
            Some(move_row) => move_rows.push(move_row),
            None => break
        }
    }

    (stacks, move_rows)
}

fn init_empty_stack_vectors(stack_labels_row: &String, stacks: &mut Vec<Vec<char>>) {
    let re = Regex::new(r" (\d+) ").unwrap();
    for _cap in re.captures_iter(&stack_labels_row) {
        stacks.push(vec![]);
    }
}

fn get_stack_def_rows(row_iter: &mut Iter<String>) -> (Vec<String>, String) {
    let mut stack_rows: Vec<String> = Vec::new();
    let stack_labels_row;
    loop {
        let next_row = row_iter.next().unwrap();
        match next_row.chars().nth(0).unwrap() {
            // first row with space in front is the label row
            ' ' => {
                stack_labels_row = next_row;
                break;
            },
            _stack_line => stack_rows.push(next_row.clone())
        }
    }
    (stack_rows, stack_labels_row.clone())
}

fn read_stack_row(stack_row: &String, stacks: &mut Vec<Vec<char>>) {

    // read initial crate assignments into stack vectors
    let re = Regex::new(r"[\[ ]([A-Z ])[\] ] ?").unwrap();
    let mut stack_index = 0;
    for cap in re.captures_iter(stack_row) {
        let crate_label: char = cap[1].to_string().chars().nth(0).unwrap();
        match crate_label {
            ' ' => (),
            label => {
                stacks[stack_index].push(label);
            }
        }
        stack_index += 1;        
    }
}