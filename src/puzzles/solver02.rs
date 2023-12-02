use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

#[allow(dead_code)]
pub fn solve() {
    const FILE_PATH: &str = "assets/input02.txt";
    let file = File::open(FILE_PATH).expect("Failed to open input file!");

    let reader = BufReader::new(file);
    let mut possible: bool;
    let mut sum_of_ids: usize = 0;
    let mut sum_of_pows: u64 = 0;

    for (i, line) in reader.lines().enumerate() {
        match line {
            Ok(content) => {
                let re = Regex::new(r"(\d+) (blue|red|green)").unwrap();
                possible = true;

                for caps in re.captures_iter(&content) {

                    let num_of_cubes: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
                    let color = caps.get(2).unwrap().as_str();

                    possible = possible && (num_of_cubes <= match color {
                        "red" => 12,
                        "green" => 13,
                        "blue" => 14,
                        _ => 0
                    });
                }

                if possible {
                    sum_of_ids += i + 1;
                }

                let mut min_blue = 0;
                let mut min_red = 0;
                let mut min_green = 0;

                for part in content.split(";") {
                    for caps in re.captures_iter(part) {
                        let num_of_cubes: u64 = caps.get(1).unwrap().as_str().parse().unwrap();
                        let color = caps.get(2).unwrap().as_str();

                        match color {
                            "red" => {
                                min_red = max(min_red, num_of_cubes);
                            },
                            "blue" => {
                                min_blue = max(min_blue, num_of_cubes);
                            },
                            "green" => {
                                min_green = max(min_green, num_of_cubes);
                            },
                            _ => {}
                        }
                    }
                }

                sum_of_pows += min_red * min_blue * min_green;
            },
            Err(error) => {
                eprintln!("Error: {}", error)
            }
        }
    }

    println!("Part 1 solution is: {}", sum_of_ids);
    println!("Part 2 solution is: {}", sum_of_pows);
}