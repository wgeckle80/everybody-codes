// Wyatt Geckle
//
// Everybody Codes 2025 Quest 4 Part 2

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        eprintln!("Please provide the notes file.");
        exit(1);
    }

    match File::open(args.get(1).unwrap()) {
        Ok(file) => {
            let reader = BufReader::new(file);
            let num_teeth: Vec<usize> = reader
                .lines()
                .map(|line_result| match line_result {
                    Ok(line) => match line.parse::<usize>() {
                        Ok(x) => x,
                        Err(_e) => {
                            eprintln!("The provided notes file is invalid.");
                            exit(1);
                        }
                    },
                    Err(e) => {
                        eprintln!("{}", e);
                        exit(1);
                    }
                })
                .collect();

            match num_teeth.first() {
                Some(first_gear_teeth) => {
                    let final_gear_rotations = 10000000000000;
                    let first_gear_rotations = (final_gear_rotations * num_teeth.last().unwrap())
                        .div_ceil(*first_gear_teeth);
                    println!("{}", first_gear_rotations);
                }
                None => {
                    eprintln!("The provided notes file is invalid.");
                    exit(1);
                }
            };
        }
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    };
}
