// Wyatt Geckle
//
// Everybody Codes 2025 Quest 1 Part 3

use std::env;
use std::fs::read_to_string;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        eprintln!("Please provide the notes file.");
        exit(1);
    }

    let file_lines: Vec<String> = read_to_string(args.get(1).unwrap())
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    if file_lines.len() < 3 {
        eprintln!("The provided notes file is invalid.");
        exit(1);
    }

    let names: Vec<&str> = file_lines[0].split(",").collect();

    if names.is_empty() {
        eprintln!("The provided notes file is invalid.");
        exit(1);
    }

    let instructions = file_lines[2].split(",");

    let mut mapping: Vec<usize> = (0..names.len()).collect();

    for instruction in instructions {
        let steps = match instruction.get(1..) {
            Some(slice) => match slice.parse::<i32>() {
                Ok(x) => x,
                Err(_e) => {
                    eprintln!("The provided notes file is invalid.");
                    exit(1);
                }
            },
            None => {
                eprintln!("The provided notes file is invalid.");
                exit(1);
            }
        };

        match instruction.get(0..1) {
            Some("L") => {
                mapping.swap(0, (-steps).rem_euclid(names.len() as i32) as usize);
            }
            Some("R") => {
                mapping.swap(0, steps.rem_euclid(names.len() as i32) as usize);
            }
            _ => {
                eprintln!("The provided notes file is invalid.");
                exit(1);
            }
        };
    }

    println!("{}", names[mapping[0]]);
}
