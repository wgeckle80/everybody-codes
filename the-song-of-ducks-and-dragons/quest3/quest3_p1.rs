// Wyatt Geckle
//
// Everybody Codes 2025 Quest 3 Part 1

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
    let mut largest_set = current_smallest;

    for &size in &crate_sizes[1..] {
        if size != current_smallest {
            largest_set += size;
            current_smallest = size;
        }
    }

    println!("{}", largest_set);
}
