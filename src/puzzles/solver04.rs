use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

#[allow(dead_code)]
pub fn solve() {
    const FILE_PATH: &str = "assets/input04.txt";

    let file = File::open(FILE_PATH).expect("Failed to open input file!");

    let reader = BufReader::new(file);
    let mut sum_of_wins:u64 = 0;

    for (_, line) in reader.lines().enumerate() {
        match line {
            Ok(content) => {
                let splits: Vec<&str> = content.split(":").collect();
                let splits: Vec<&str> = splits[1].split("|").collect();

                let re = Regex::new(r"\d+").unwrap();

                let winnings:Vec<&str> = re.captures_iter(splits[0]).map(|x| x.get(0).unwrap().as_str()).collect();
                let mut num_of_wins: u64 = 0;

                for capture in re.captures_iter(splits[1]) {
                    if winnings.contains(&capture.get(0).unwrap().as_str()) {
                        if num_of_wins == 0 {
                            num_of_wins = 1;
                        }
                        else {
                            num_of_wins *= 2;
                        }
                    }
                }

                sum_of_wins += num_of_wins;
            },
            Err(error) => {
                eprintln!("Error: {}", error)
            }
        }
    }

    println!("Part 1 solution is: {}", sum_of_wins);
}