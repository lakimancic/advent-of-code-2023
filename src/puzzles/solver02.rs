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
            },
            Err(error) => {
                eprintln!("Error: {}", error)
            }
        }
    }

    println!("Part 1 solution is: {}", sum_of_ids);
}