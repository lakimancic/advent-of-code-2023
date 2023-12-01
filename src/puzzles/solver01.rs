use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

pub fn solve() {
    const FILE_PATH: &str = "assets/input01.txt";
    let file = File::open(FILE_PATH).expect("Failed to open input file!");

    let reader = BufReader::new(file);
    let mut calibration: u32 = 0;
    let mut calibration2: u32 = 0;

    for line in reader.lines() {
        match line {
            Ok(content) => {
                if !content.is_empty() {
                    let mut curr_char: char = '0';
                    let mut found: bool = false;

                    // Part 1
                    for ch in content.chars() {
                        if ch.is_digit(10) {
                            curr_char = ch;
                            if !found {
                                found = true;
                                calibration += 10 * (curr_char.to_digit(10)).unwrap();
                            }
                        }
                    }
                    calibration += (curr_char.to_digit(10)).unwrap();
                    
                    // Part 2
                    let re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)").unwrap();
                    let new_content = re.replace_all(&content, |caps: &regex::Captures| {
                        match &caps[0] {
                            "one" => "1",
                            "two" => "2",
                            "three" => "3",
                            "four" => "4",
                            "five" => "5",
                            "six" => "6",
                            "seven" => "7",
                            "eight" => "8",
                            "nine" => "9",
                            _ => ""
                        }
                    });

                    found = false;
                    for ch in new_content.chars() {
                        if ch.is_digit(10) {
                            curr_char = ch;
                            if !found {
                                found = true;
                                calibration2 += 10 * (curr_char.to_digit(10)).unwrap();
                            }
                        }
                    }
                    calibration2 += (curr_char.to_digit(10)).unwrap();
                }
            }
            Err(error) => {
                eprintln!("Error: {}", error)
            }
        }
    }

    println!("Part 1 solution is: {}", calibration);
    println!("Part 2 solution is: {}", calibration2);
}