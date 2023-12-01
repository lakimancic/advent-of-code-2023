use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    const FILE_PATH: &str = "assets/input01.txt";
    let file = File::open(FILE_PATH).expect("Failed to open input file!");

    let reader = BufReader::new(file);
    let mut calibration: u32 = 0;
    let mut calibration2: u32 = 0;
    const LETTER_DIGITS : [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    const NUMERIC_DIGITS : [&str; 9] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];

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
                    let mut min_index;
                    let mut pat_index : usize = 0;
                    let mut new_content = content.clone();
                    loop {
                        min_index = new_content.len();
                        for (ind, letter_digit) in LETTER_DIGITS.iter().enumerate() {
                            match new_content.find(letter_digit) {
                                Some(index) => {
                                    if index < min_index {
                                        min_index = index;
                                        pat_index = ind;
                                    }
                                }
                                None => {}
                            }
                        }
                        if min_index < new_content.len() {
                            new_content = new_content.replacen(LETTER_DIGITS[pat_index], NUMERIC_DIGITS[pat_index], 1)
                        }
                        else {
                            break;
                        }
                    }

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