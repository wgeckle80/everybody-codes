// Wyatt Geckle
//
// Everybody Codes 2025 Quest 5 Part 3

use std::cmp::Ordering;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;
use std::process::exit;

struct Sword {
    id: u64,
    quality: u64,
    level_values: Vec<u64>,
}

impl Sword {
    fn from(line: &String) -> Result<Self, &'static str> {
        let mut split = line.split(':');

        let id = match split.next() {
            Some(id_string) => match id_string.parse::<u64>() {
                Ok(x) => x,
                Err(_e) => {
                    return Err("Expected an integer ID to the left of \":\"");
                }
            },
            None => {
                return Err("Expected an integer ID to the left of \":\"");
            }
        };

        let numbers_list: Vec<u64> = match split.next() {
            Some(numbers_string) => {
                let numbers_list_result: Result<Vec<u64>, &'static str> = numbers_string
                    .split(',')
                    .map(|number_string| match number_string.parse::<u64>() {
                        Ok(x) => Ok(x),
                        Err(_e) => Err("Expected a list of integers to the right of \":\""),
                    })
                    .collect();

                match numbers_list_result {
                    Ok(list) => list,
                    Err(e) => return Err(e),
                }
            }
            None => {
                return Err("Expected a list of integers to the right of \":\"");
            }
        };

        if numbers_list.is_empty() {
            return Err("Expected a list of integers to the right of \":\"");
        }

        let mut spine_values: Vec<u64> = vec![numbers_list[0]];
        let mut left_values: Vec<Option<u64>> = vec![None];
        let mut right_values: Vec<Option<u64>> = vec![None];

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

        let quality_result = spine_values
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse::<u64>();

        match quality_result {
            Ok(quality) => {
                let mut level_values: Vec<u64> = Vec::with_capacity(spine_values.len());

                for (i, &center_value) in spine_values.iter().enumerate() {
                    let mut level_string = match left_values[i] {
                        Some(value) => value.to_string(),
                        None => String::from(""),
                    };

                    let center_string = center_value.to_string();
                    level_string.push_str(&center_string);

                    let right_string = match right_values[i] {
                        Some(value) => value.to_string(),
                        None => String::from(""),
                    };
                    level_string.push_str(&right_string);

                    level_values.push(level_string.parse::<u64>().unwrap());
                }

                Ok(Self {
                    id,
                    quality,
                    level_values,
                })
            }
            Err(_e) => Err("The spine is too long"),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(file_name) => match File::open(file_name) {
            Ok(file) => {
                let reader = BufReader::new(file);
                let mut swords: Vec<Sword> = reader
                    .lines()
                    .map(|line_result| match line_result {
                        Ok(line) => match Sword::from(&line) {
                            Ok(sword) => sword,
                            Err(e) => {
                                eprintln!("{}.", e);
                                exit(1);
                            }
                        },
                        Err(e) => {
                            eprintln!("{}", e);
                            exit(1);
                        }
                    })
                    .collect();

                if swords.is_empty() {
                    eprintln!("No swords were provided in the notes file.");
                    exit(1);
                }

                swords.sort_by(|sword1, sword2| {
                    let quality1 = sword1.quality;
                    let quality2 = sword2.quality;

                    match quality1.cmp(&quality2) {
                        Ordering::Equal => {
                            let levels1_iter = sword1.level_values.iter();
                            let levels2_iter = sword2.level_values.iter();

                            for (level1, level2) in zip(levels1_iter, levels2_iter) {
                                let order = level1.cmp(&level2);

                                if order != Ordering::Equal {
                                    return order;
                                }
                            }

                            Ordering::Equal
                        }
                        order => order,
                    }
                });

                let checksum: u64 = swords
                    .iter()
                    .rev()
                    .enumerate()
                    .map(|(i, sword)| (i as u64 + 1) * sword.id)
                    .sum();

                println!("{}", checksum);
            }
            Err(e) => {
                eprintln!("{}", e);
                exit(1);
            }
        },
        None => {
            eprintln!("Please provide the notes file.");
            exit(1);
        }
    }
}
