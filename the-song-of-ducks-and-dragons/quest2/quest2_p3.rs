// Wyatt Geckle
//
// Everybody Codes 2025 Quest 2 Part 3

use std::env;
use std::fs::read_to_string;
use std::process::exit;

struct Complex {
    x: i64,
    y: i64,
}

impl Complex {
    fn add(&self, z2: &Complex) -> Complex {
        Complex {
            x: self.x + z2.x,
            y: self.y + z2.y,
        }
    }

    fn mul(&self, z2: &Complex) -> Complex {
        Complex {
            x: self.x * z2.x - self.y * z2.y,
            y: self.x * z2.y + self.y * z2.x,
        }
    }

    fn div(&self, z2: &Complex) -> Complex {
        Complex {
            x: self.x / z2.x,
            y: self.y / z2.y,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        eprintln!("Please provide the notes file.");
        exit(1);
    }

    let file_line = read_to_string(args.get(1).unwrap()).unwrap();
    let mut file_line_components = file_line.split(",");

    let a_x = match file_line_components.next() {
        Some(string) => match string.get(3..) {
            Some(slice) => match slice.parse::<i64>() {
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
        },
        None => {
            eprintln!("The provided notes file is invalid.");
            exit(1);
        }
    };
    let a_y = match file_line_components.next() {
        Some(string) => match string.get(..string.len() - 1) {
            Some(slice) => match slice.parse::<i64>() {
                Ok(y) => y,
                Err(_e) => {
                    eprintln!("The provided notes file is invalid.");
                    exit(1);
                }
            },
            None => {
                eprintln!("The provided notes file is invalid.");
                exit(1);
            }
        },
        None => {
            eprintln!("The provided notes file is invalid.");
            exit(1);
        }
    };
    let a = Complex { x: a_x, y: a_y };
    let divisor = Complex {
        x: 100000,
        y: 100000,
    };
    let threshold = 1000000;
    let grid_length_width = 1001;

    let mut engraved_points = 0;

    for i in 0..grid_length_width {
        for j in 0..grid_length_width {
            let offset = Complex { x: i, y: j };
            let examination_point = a.add(&offset);

            let mut result = Complex { x: 0, y: 0 };

            let num_cycles = 100;
            for _i in 0..num_cycles {
                result = result.mul(&result).div(&divisor).add(&examination_point);

                if result.x.abs() > threshold || result.y.abs() > threshold {
                    break;
                }
            }

            if result.x.abs() <= threshold && result.y.abs() <= threshold {
                engraved_points += 1;
            }
        }
    }

    println!("{}", engraved_points);
}
