use std::{
    // BTreeMap sorts by key, which is handy for debugging purposes
    collections::BTreeMap,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    slice::Iter,
};

use std::iter::FromIterator;

const TOTAL_DISK_SPACE: u32 = 70_000_000;
const MIN_FREE_SPACE: u32 = 30_000_000;

fn main() {
    let lines = lines_from_file("input.txt");
    let mut lines = lines.iter();
    let mut file_sizes: BTreeMap<String, u32> = BTreeMap::new();
    let total_used_space = get_dir_file_sizes("", &mut lines, &mut file_sizes);

    // remove non-directory entries (included for debugging)
    let dir_sizes = file_sizes.iter().filter(|(k, _)| k.ends_with("/"));

    // find smallest directory that will make enough space if removed
    let mut min_dir_size: u32 = total_used_space;
    let mut sorted_dir_sizes = Vec::from_iter(dir_sizes);
    sorted_dir_sizes.sort_by(|(_, v1), (_, v2)| v1.cmp(v2));
    for (_dir_name, &dir_size) in sorted_dir_sizes {
        if TOTAL_DISK_SPACE - total_used_space + dir_size >= MIN_FREE_SPACE {
            min_dir_size = dir_size;
            break;
        }
    }

    println!("Min dir size to remove: {min_dir_size}");
}

fn get_dir_file_sizes(
    path: &str,
    lines: &mut Iter<String>,
    file_sizes: &mut BTreeMap<String, u32>,
) -> u32 {
    let mut dir_size: u32 = 0;
    loop {
        match lines.next() {
            // we reached the end of the file, we're done
            None => break dir_size,

            // process the next input line for the current directory
            Some(line) => {
                let line_entry: Vec<&str> = line.split(" ").collect();
                match (line_entry[0], line_entry[1]) {
                    ("$", "cd") => {
                        // `$ cd <arg>`
                        match line_entry[2] {
                            // `$ cd ..`: we're done getting sizes of all files and subdirectories in this directory
                            ".." => break dir_size,

                            // `$ cd <filename>`: we're going to read a subdirectory
                            dir_name => {
                                let filepath = format!("{path}{dir_name}/");
                                let subdir_size = get_dir_file_sizes(&filepath, lines, file_sizes);
                                file_sizes.insert(filepath, subdir_size);
                                dir_size += subdir_size;
                            }
                        }
                    }

                    // `$ ls`: file entries will follow, ignore this line
                    ("$", "ls") => (),

                    // `dir <name>`: we'll get a `cd <name>` command for this directory later, ignore this line
                    ("dir", _name) => (),

                    // only remaining possibility is a `<size> <name>` file entry
                    (size, filename) => {
                        let filepath = format!("{path}{filename}");
                        let file_size = size.parse::<u32>().unwrap();
                        file_sizes.insert(filepath, file_size);
                        dir_size += file_size;
                    }
                }
            }
        }
    }
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
