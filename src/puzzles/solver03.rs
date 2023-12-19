use std::collections::{HashMap, HashSet};

pub fn solve() {
    const FILE_PATH: &str = "assets/input03.txt";
    let txt = std::fs::read_to_string(FILE_PATH).unwrap();
    let lines = txt.lines();

    let mut field: Vec<Vec<char>> = Vec::new();
    let mut bit_field: Vec<Vec<bool>> = Vec::new();

    for line in lines {
        bit_field.push(vec![false; line.len()]);
        field.push(line.chars().collect());
    }

    for i in 0..field.len() {
        for j in 0..field[i].len() {
            if field[i][j] == '.' || field[i][j].is_digit(10) {
                continue;
            }
            for di in -1..2 {
                for dj in -1..2 {
                    if i as i32 + di < 0 || i as i32 + di >= field.len() as i32 {
                        continue;
                    }
                    if j as i32 + dj < 0 || j as i32 + dj >= field[i].len() as i32 {
                        continue;
                    }
                    bit_field[(i as i32 + di) as usize][(j as i32 + dj) as usize] = true;
                }
            }
        }
    }

    let mut sum_of_all = 0;
    let mut gears_sum = 0;
    let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    for i in 0..field.len() {
        let mut to_add = 0;
        let mut should_add = false;
        let mut where_add: HashSet<(usize, usize)> = HashSet::new();
        for j in 0..field[i].len() {
            if field[i][j].is_digit(10) {
                to_add = to_add * 10 + field[i][j].to_digit(10).unwrap();
                should_add |= bit_field[i][j];

                for di in -1..2 {
                    for dj in -1..2 {
                        if i as i32 + di < 0 || i as i32 + di >= field.len() as i32 {
                            continue;
                        }
                        if j as i32 + dj < 0 || j as i32 + dj >= field[i].len() as i32 {
                            continue;
                        }
                        if field[(i as i32 + di) as usize][(j as i32 + dj) as usize] != '*' {
                            continue;
                        }
                        where_add.insert(((i as i32 + di) as usize, (j as i32 + dj) as usize));
                    }
                }
            } else {
                if should_add {
                    sum_of_all += to_add;
                }
                for key in &where_add {
                    if gears.contains_key(key) {
                        gears.get_mut(key).unwrap().push(to_add);
                    } else {
                        gears.insert(*key, vec![to_add]);
                    }
                }
                to_add = 0;
                should_add = false;
                where_add = HashSet::new();
            }
        }
        if should_add {
            sum_of_all += to_add;
        }
        for key in &where_add {
            if gears.contains_key(key) {
                gears.get_mut(key).unwrap().push(to_add);
            } else {
                gears.insert(*key, vec![to_add]);
            }
        }
    }

    for gear in gears {
        if gear.1.len() > 1 {
            let mut gear_sum: u32 = 1;
            for gear_val in gear.1 {
                gear_sum *= gear_val;
            }
            gears_sum += gear_sum;
        }
    }

    println!("Part 1 solution is: {}", sum_of_all);
    println!("Part 2 solution is: {}", gears_sum);
}
