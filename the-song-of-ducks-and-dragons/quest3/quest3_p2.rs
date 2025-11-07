// Wyatt Geckle
//
// Everybody Codes 2025 Quest 3 Part 2

use std::env;
use std::fs::read_to_string;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        eprintln!("Please provide the notes file.");
        exit(1);
    }

    let file_line = read_to_string(args.get(1).unwrap()).unwrap();
    let mut crate_sizes: Vec<usize> = file_line
        .split(",")
        .map(|size_string| match size_string.parse::<usize>() {
            Ok(x) => x,
            Err(_e) => {
                eprintln!("The provided notes file is invalid.");
                exit(1);
            }
        })
        .collect();

    if crate_sizes.is_empty() {
        eprintln!("The provided notes file is invalid.");
        exit(1);
    }

    crate_sizes.sort();

    let mut current_smallest = crate_sizes[0];
    let mut smallest_set = current_smallest;

    let mut crate_index = 1;
    let mut num_crates_in_set = 1;

    while crate_index < crate_sizes.len() && num_crates_in_set < 20 {
        let size = crate_sizes[crate_index];

        if size != current_smallest {
            num_crates_in_set += 1;
            smallest_set += size;
            current_smallest = size;
        }

        crate_index += 1;
    }

    println!("{}", smallest_set);
}
