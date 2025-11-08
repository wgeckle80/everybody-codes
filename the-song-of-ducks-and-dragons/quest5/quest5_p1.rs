// Wyatt Geckle
//
// Everybody Codes 2025 Quest 5 Part 1

use std::env;
use std::fs::read_to_string;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        eprintln!("Please provide the notes file.");
        exit(1);
    }

    match read_to_string(args.get(1).unwrap()) {
        Ok(file_line) => {
            let mut split = file_line.split(':');

            let _id = match split.next() {
                Some(id_string) => match id_string.parse::<u32>() {
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

            let numbers_list: Vec<u32> = match split.next() {
                Some(numbers_string) => numbers_string
                    .split(',')
                    .map(|number_string| match number_string.parse::<u32>() {
                        Ok(x) => x,
                        Err(_e) => {
                            eprintln!("The provided notes file is invalid.");
                            exit(1);
                        }
                    })
                    .collect(),
                None => {
                    eprintln!("The provided notes file is invalid.");
                    exit(1);
                }
            };

            if numbers_list.is_empty() {
                eprintln!("The provided notes file is invalid.");
                exit(1);
            }

            let mut spine_values: Vec<u32> = vec![numbers_list[0]];
            let mut left_values: Vec<Option<u32>> = vec![None];
            let mut right_values: Vec<Option<u32>> = vec![None];

            for &number in numbers_list.iter().skip(1) {
                let mut inserted = false;

                for (i, &spine_value) in spine_values.iter().enumerate() {
                    if number < spine_value && left_values[i] == None {
                        left_values[i] = Some(number);
                        inserted = true;
                        break;
                    }

                    if number > spine_value && right_values[i] == None {
                        right_values[i] = Some(number);
                        inserted = true;
                        break;
                    }
                }

                if !inserted {
                    spine_values.push(number);
                    left_values.push(None);
                    right_values.push(None);
                }
            }

            println!(
                "{}",
                spine_values
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<String>>()
                    .join("")
            );
        }
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }
}
