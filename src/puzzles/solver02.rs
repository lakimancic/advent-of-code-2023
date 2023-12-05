use std::cmp::max;
use regex::Regex;

#[allow(dead_code)]
pub fn solve() {
    const FILE_PATH: &str = "assets/input02.txt";
    let txt = std::fs::read_to_string(FILE_PATH).unwrap();
    let lines = txt.lines();

    let mut possible: bool;
    let mut sum_of_ids: usize = 0;
    let mut sum_of_pows: u64 = 0;

    for (i, line) in lines.enumerate() {
        let re = Regex::new(r"(\d+) (blue|red|green)").unwrap();
        possible = true;

        for caps in re.captures_iter(line) {

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

        for part in line.split(";") {
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
    }

    println!("Part 1 solution is: {}", sum_of_ids);
    println!("Part 2 solution is: {}", sum_of_pows);
}