// Wyatt Geckle
//
// Everybody Codes 2025 Quest 4 Part 3

use std::env;
use std::fs::read_to_string;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        eprintln!("Please provide the notes file.");
        exit(1);
    }

    let num_teeth: Vec<f64> = read_to_string(args.get(1).unwrap())
        .unwrap()
        .replace("|", "\n")
        .lines()
        .map(|line| match line.parse::<f64>() {
            Ok(x) => x,
            Err(_e) => {
                eprintln!("The provided notes file is invalid.");
                exit(1);
            }
        })
        .collect();

    if num_teeth.is_empty() || num_teeth.len() % 2 != 0 {
        eprintln!("The provided notes file is invalid.");
        exit(1);
    }

    let mut rotations = 100.0;

    for i in (0..num_teeth.len()).step_by(2) {
        rotations *= num_teeth[i] / num_teeth[i + 1];
    }

    println!("{}", rotations.floor());
}
