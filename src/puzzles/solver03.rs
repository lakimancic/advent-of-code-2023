use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn solve() {
    const FILE_PATH: &str = "assets/input00.txt";
    let file = File::open(FILE_PATH).expect("Failed to open input file!");

    let reader = BufReader::new(file);
    let mut field: Vec<Vec<char>> = Vec::new();
    let mut bit_field : Vec<Vec<bool>> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(content) => {
                bit_field.push(vec![false; content.len()]);
                field.push(content.chars().collect());
            },
            Err(error) => {
                eprintln!("Error: {}", error)
            }
        }
    }

    for i in 0..field.len() {
        for j in 0..field[i].len() {
            if field[i][j] == '.' || field[i][j].is_digit(10) { continue; }
            for di in -1..2 {
                for dj in -1..2 {
                    if i as i32 + di < 0 || i as i32 + di >= field.len() as i32 { continue; }
                    if j as i32 + dj < 0 || j as i32 + dj >= field[i].len() as i32 { continue; }
                    bit_field[(i as i32 + di) as usize][(j as i32 + dj) as usize] = true;
                }
            }
        }
    }

    let mut sum_of_all = 0;

    for i in 0..field.len() {
        let mut to_add = 0;
        let mut should_add = false;
        for j in 0..field[i].len() {
            if field[i][j].is_digit(10) {
                to_add = to_add * 10 + field[i][j].to_digit(10).unwrap();
                should_add |= bit_field[i][j];
            }
            else {
                if should_add {
                    sum_of_all += to_add;
                }
                to_add = 0;
                should_add = false;
            }
        }
        if should_add {
            sum_of_all += to_add;
        }
    }

    println!("Part 1 solution is: {}", sum_of_all);
}